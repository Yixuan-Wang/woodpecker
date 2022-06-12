use std::{hash::Hash, collections::HashSet, ops::{BitOr, BitOrAssign}};

use async_trait::async_trait;
use serde::{Deserialize, Deserializer};
use serde_with::{serde_as, TimestampSeconds};
use time::OffsetDateTime;

use crate::{hole::{HoleID, lossy_deserialize_usize}, common::{Resource, ParseResource, ParseResourceError, MergeResource}};

use crate::derive_set;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord, Deserialize)]
pub struct ReplyID(#[serde(deserialize_with = "lossy_deserialize_usize")] pub usize);

impl From<usize> for ReplyID {
    fn from(id: usize) -> Self {
        Self(id)
    }
}

impl From<ReplyID> for usize {
    fn from(id: ReplyID) -> Self {
        id.0
    }
}

impl From<ReplyID> for String {
    fn from(id: ReplyID) -> Self {
        id.0.to_string()
    }
}

#[serde_as]
#[derive(Debug, Deserialize, Eq)]
pub struct Reply {
    #[serde(rename = "cid")]
    pub id: ReplyID,
    #[serde(rename = "pid")]
    pub hole: HoleID,
    pub name: String,
    pub text: String,
    #[serde(rename = "islz", deserialize_with = "number_to_bool")]
    pub owner: bool,
    #[serde_as(as = "TimestampSeconds<String>")]
    pub timestamp: OffsetDateTime,
    pub tag: Option<String>,
}

impl PartialEq for Reply {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Hash for Reply {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state)
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct ReplyFlag(bool);

#[derive(Debug, Deserialize, PartialEq, Eq)]
pub struct ReplyPage {
    pub code: usize,
    pub data: Vec<Reply>,
    #[serde(deserialize_with = "number_to_bool")]
    pub attention: bool,
}

derive_set!{reply, Reply, ReplyEntry, ReplyFlag, ReplyPage, ReplySet}

impl Hash for ReplyEntry {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.reply.id.hash(state);
        if self.flag == ReplyFlag(true) {
            self.record.hash(state)
        }
    }
}

impl PartialEq for ReplyEntry {
    fn eq(&self, other: &Self) -> bool {
        return !(
            self.reply.id != other.reply.id
            || (self.flag == ReplyFlag(true) && self.record != other.record)
        ) 
    }
}

/* #[derive(Debug, Eq)]
pub struct ReplyEntry {
    pub reply: Reply,
    pub record: OffsetDateTime,
    pub flag: ReplyFlag,
}

impl Hash for ReplyEntry {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.reply.id.hash(state);
        if self.flag == ReplyFlag(true) {
            self.record.hash(state)
        }
    }
}

impl PartialEq for ReplyEntry {
    fn eq(&self, other: &Self) -> bool {
        return !(
            self.reply.id != other.reply.id
            || (self.flag == ReplyFlag(true) && self.record != other.record)
        ) 
    }
}

#[derive(Debug, Default, PartialEq, Eq)]
pub struct ReplySet {
    set: HashSet<ReplyEntry>,
    flag: ReplyFlag,
}

impl ReplySet {
    pub fn len(&self) -> usize {
        self.set.len()
    }

    pub fn is_empty(&self) -> bool {
        self.set.is_empty()
    }
}

impl Resource for ReplySet {
    type Specifier = ReplyFlag;

    fn blank(flag: Self::Specifier) -> Self {
        ReplySet {
            set: HashSet::default(),
            flag,
        }
    }
}

#[async_trait]
impl ParseResource<ReplySet> for ReplySet {
    async fn parse(
        response: reqwest::Response,
        flag: ReplyFlag,
    ) -> Result<ReplySet, ParseResourceError> {
        let page: ReplyPage = response.json().await?;
        Ok((page, flag).into())
    }
}

impl MergeResource<ReplySet> for ReplySet {
    fn merge(
        lhs: ReplySet,
        rhs: ReplySet,
        _flag: <ReplySet as Resource>::Specifier,
    ) -> Result<ReplySet, crate::common::MergeResourceError> {
        Ok(lhs | rhs)
    }
}

impl From<ReplyPage> for ReplySet {
    fn from(reply_page: ReplyPage) -> Self {
        (reply_page, ReplyFlag::default()).into()
    }
}



impl IntoIterator for ReplySet {
    type Item = ReplyEntry;
    type IntoIter = std::collections::hash_set::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.set.into_iter()
    }
}

impl BitOr for ReplySet {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        assert_eq!(
            self.flag, rhs.flag,
            "`HashSet`s with {:?} and {:?} cannot be intersected",
            self.flag, rhs.flag
        );
        let ReplySet { mut set, flag } = self;
        let ReplySet { set: set_rhs, .. } = rhs;
        set.extend(set_rhs);
        ReplySet { set, flag }
    }
}

impl BitOrAssign for ReplySet {
    fn bitor_assign(&mut self, rhs: Self) {
        assert_eq!(
            self.flag, rhs.flag,
            "`HashSet`s with {:?} and {:?} cannot be intersected",
            self.flag, rhs.flag
        );
        let ReplySet { set: set_rhs, .. } = rhs;
        self.set.extend(set_rhs);
    }
} */

impl From<(ReplyPage, ReplyFlag)> for ReplySet {
    fn from((reply_page, flag): (ReplyPage, ReplyFlag)) -> Self {
        let ReplyPage {
            data, ..
        } = reply_page;
        let record = OffsetDateTime::now_utc();
        let set = data
            .into_iter()
            .map(|reply| ReplyEntry { reply, record, flag })
            .collect();
        ReplySet { set, flag }
    }
}

/* fn number_str_to_bool<'de, D>(d: D) -> Result<bool, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(d)?;
    Ok(s.parse::<usize>().unwrap_or(0) == 1)
} */

fn number_to_bool<'de, D>(d: D) -> Result<bool, D::Error>
where
    D: Deserializer<'de>,
{
    let s = usize::deserialize(d)?;
    Ok(s == 1)
}

/*

"cid":"16355926","pid":"3712013","text":"[Alice] \u4e0d\u91cd\u8981\uff01","timestamp":"1655035401","anonymous":"1","tag":null,"islz":0,"name":"Alice"}

*/
