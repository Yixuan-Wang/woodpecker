use std::hash::Hash;

use serde::{Deserialize, Deserializer, Serialize};
use serde_with::{serde_as, TimestampSeconds};
use time::OffsetDateTime;

use super::{RawHoleID, HoleID, lossy_deserialize_usize};

#[derive(Debug, Deserialize)]
pub struct RawReplyID(#[serde(deserialize_with = "lossy_deserialize_usize")] pub usize);

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize)]
pub struct ReplyID(pub usize);

impl From<usize> for ReplyID {
    fn from(id: usize) -> Self {
        Self(id)
    }
}

impl From<RawReplyID> for ReplyID {
    fn from(raw: RawReplyID) -> Self {
        Self(raw.0)
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
#[derive(Debug, Deserialize)]
pub struct RawReply {
    #[serde(rename = "cid")]
    pub id: RawReplyID,
    #[serde(rename = "pid")]
    pub hole: RawHoleID,
    pub name: String,
    pub text: String,
    #[serde(rename = "islz", deserialize_with = "number_to_bool")]
    pub dz: bool,
    #[serde_as(as = "TimestampSeconds<String>")]
    pub timestamp: OffsetDateTime,
    pub tag: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Eq)]
pub struct Reply {
    pub id: ReplyID,
    pub hole: HoleID,
    pub name: String,
    pub text: String,
    pub dz: bool,
    pub timestamp: OffsetDateTime,
    pub tag: Option<String>,
}

impl From<RawReply> for Reply {
    fn from(raw: RawReply) -> Self {
        let RawReply { id, hole, name, text, dz, timestamp, tag } = raw;
        Self { id: id.into(), hole: hole.into(), name, text, dz, timestamp, tag }
    }
}

impl PartialEq for Reply {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl PartialOrd for Reply {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.id.partial_cmp(&other.id)
    }
}

impl Ord for Reply {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.id.cmp(&other.id)
    }
}

impl Hash for Reply {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state)
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Deserialize, Serialize)]
pub struct ReplyFlag(bool);

#[derive(Debug, Deserialize)]
pub struct RawReplyPage {
    pub code: usize,
    pub data: Vec<RawReply>,
    #[serde(deserialize_with = "number_to_bool")]
    pub attention: bool,
}

#[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize)]
pub struct ReplyEntry {
    pub entry: Reply,
    pub snapshot: OffsetDateTime,
}

impl IntoIterator for RawReplyPage {
    type Item = ReplyEntry;
    type IntoIter = <Vec<Self::Item> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        let RawReplyPage {
            data , ..
        } = self;
        let snapshot = OffsetDateTime::now_utc();
        data
            .into_iter()
            .map(|reply| ReplyEntry { entry: reply.into(), snapshot })
            .collect::<Vec<_>>()
            .into_iter()
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
