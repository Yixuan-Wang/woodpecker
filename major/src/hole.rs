use std::{
    collections::HashSet,
    hash::Hash,
    ops::{BitOr, BitOrAssign},
};

use async_trait::async_trait;
use bitflags::bitflags;
use serde::{Deserialize, Deserializer};
use serde_with::{serde_as, TimestampSeconds};
use time::OffsetDateTime;

use crate::common::{MergeResource, ParseResource, ParseResourceError, Resource};

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord, Deserialize)]
pub struct HoleID(#[serde(deserialize_with = "lossy_deserialize_usize")] usize);

impl From<HoleID> for usize {
    fn from(id: HoleID) -> Self {
        id.0
    }
}

#[derive(Debug, Deserialize, PartialEq, Eq, Hash)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum HoleKind {
    Text,
    Image { url: String },
    Audio { url: String },
}

#[serde_as]
#[derive(Debug, Deserialize, Eq)]
pub struct Hole {
    #[serde(rename = "pid")]
    pub id: HoleID,
    pub text: String,
    #[serde(rename = "type", flatten)]
    pub kind: HoleKind,
    #[serde_as(as = "TimestampSeconds<String>")]
    pub timestamp: OffsetDateTime,
    #[serde(deserialize_with = "lossy_deserialize_usize")]
    pub reply: usize,
    #[serde(deserialize_with = "lossy_deserialize_usize")]
    pub likenum: usize,
    pub tag: Option<String>,
    /* #[serde(deserialize_with = "lossy_deserialize_usize")]
    pub hot: usize, */
    /* {
        "pid": "3655935",
        "hidden": "0",
        "text": "3655875\n预告一下明天的内容：周三周三",
        "type": "text",
        "timestamp": "1653983358",
        "reply": "0",
        "likenum": "1",
        "extra": "0",
        "url": "",
        "hot": "1653983358",
        "tag": null
    } */
}

impl PartialEq for Hole {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Hash for Hole {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state)
    }
}

bitflags! {
    #[derive(Default)]
    pub struct HoleFlag: u8 {
        const REPLY  = 0b00000001;
        const LIKE   = 0b00000010;
        const RECORD = 0b00000100;
    }
}

#[derive(Debug, Eq)]
pub struct HoleEntry {
    pub hole: Hole,
    pub record: OffsetDateTime,
    pub flag: HoleFlag,
}

impl Hash for HoleEntry {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.hole.id.hash(state);
        if self.flag.contains(HoleFlag::REPLY) {
            self.hole.reply.hash(state)
        }
        if self.flag.contains(HoleFlag::LIKE) {
            self.hole.likenum.hash(state)
        }
        if self.flag.contains(HoleFlag::RECORD) {
            self.record.hash(state)
        }
    }
}

impl PartialEq for HoleEntry {
    fn eq(&self, other: &Self) -> bool {
        if self.hole.id != other.hole.id
            || (self.flag.contains(HoleFlag::REPLY) && self.hole.reply != other.hole.reply)
            || (self.flag.contains(HoleFlag::LIKE) && self.hole.likenum != other.hole.likenum)
        {
            false
        } else {
            !(self.flag.contains(HoleFlag::RECORD) && self.record != other.record)
        }
    }
}

#[serde_as]
#[derive(Debug, Deserialize)]
pub struct HolePage {
    pub code: i32,
    #[serde(default)]
    pub count: Option<i32>,
    pub data: Vec<Hole>,
    #[serde(default)]
    #[serde_as(as = "Option<TimestampSeconds<i64>>")]
    pub timestamp: Option<OffsetDateTime>,
}

#[derive(Debug, Default, PartialEq, Eq)]
pub struct HoleSet {
    set: HashSet<HoleEntry>,
    flag: HoleFlag,
}

impl HoleSet {
    pub fn len(&self) -> usize {
        self.set.len()
    }

    pub fn is_empty(&self) -> bool {
        self.set.is_empty()
    }
}

impl Resource for HoleSet {
    type Specifier = HoleFlag;

    fn blank(flag: Self::Specifier) -> Self {
        HoleSet {
            set: HashSet::default(),
            flag,
        }
    }
}

#[async_trait]
impl ParseResource<HoleSet> for HoleSet {
    async fn parse(
        response: reqwest::Response,
        flag: HoleFlag,
    ) -> Result<HoleSet, ParseResourceError> {
        let page: HolePage = response.json().await?;
        Ok((page, flag).into())
    }
}

impl MergeResource<HoleSet> for HoleSet {
    fn merge(
        lhs: HoleSet,
        rhs: HoleSet,
        _flag: <HoleSet as Resource>::Specifier,
    ) -> Result<HoleSet, crate::common::MergeResourceError> {
        Ok(lhs | rhs)
    }
}

impl From<HolePage> for HoleSet {
    fn from(hole_page: HolePage) -> Self {
        (hole_page, HoleFlag::default()).into()
    }
}

impl From<(HolePage, HoleFlag)> for HoleSet {
    fn from((hole_page, flag): (HolePage, HoleFlag)) -> Self {
        let HolePage {
            data, timestamp, ..
        } = hole_page;
        let record = timestamp.unwrap_or_else(OffsetDateTime::now_utc);
        let set = data
            .into_iter()
            .map(|hole| HoleEntry { hole, record, flag })
            .collect();
        HoleSet { set, flag }
    }
}

impl IntoIterator for HoleSet {
    type Item = HoleEntry;
    type IntoIter = std::collections::hash_set::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.set.into_iter()
    }
}

impl BitOr for HoleSet {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        assert_eq!(
            self.flag, rhs.flag,
            "`HashSet`s with {:?} and {:?} cannot be intersected",
            self.flag, rhs.flag
        );
        let HoleSet { mut set, flag } = self;
        let HoleSet { set: set_rhs, .. } = rhs;
        set.extend(set_rhs);
        HoleSet { set, flag }
    }
}

impl BitOrAssign for HoleSet {
    fn bitor_assign(&mut self, rhs: Self) {
        assert_eq!(
            self.flag, rhs.flag,
            "`HashSet`s with {:?} and {:?} cannot be intersected",
            self.flag, rhs.flag
        );
        let HoleSet { set: set_rhs, .. } = rhs;
        self.set.extend(set_rhs);
    }
}

fn lossy_deserialize_usize<'de, D>(d: D) -> Result<usize, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(d)?;
    Ok(s.parse::<usize>().unwrap_or(0))
}
