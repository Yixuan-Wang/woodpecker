use std::hash::Hash;

use serde::{Deserialize, Deserializer, Serialize};
use serde_with::{serde_as, TimestampSeconds};
use time::OffsetDateTime;

// use crate::common::{MergeResource, ParseResource, ParseResourceError, Resource};

pub mod reply;

#[derive(Debug, Deserialize)]
pub struct RawHoleID(#[serde(deserialize_with = "lossy_deserialize_usize")] pub usize);

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize)]
pub struct HoleID(pub usize);

impl From<RawHoleID> for HoleID {
    fn from(raw: RawHoleID) -> Self {
        Self(raw.0)
    }
}

impl From<usize> for HoleID {
    fn from(id: usize) -> Self {
        Self(id)
    }
}

impl From<HoleID> for usize {
    fn from(id: HoleID) -> Self {
        id.0
    }
}

impl From<HoleID> for String {
    fn from(id: HoleID) -> Self {
        id.0.to_string()
    }
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Hash)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum HoleKind {
    Text,
    Image { url: String },
    Audio { url: String },
}

#[serde_as]
#[derive(Debug, Deserialize)]
pub struct RawHole {
    #[serde(rename = "pid")]
    pub id: RawHoleID,
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
}

#[derive(Debug, Deserialize, Eq, Serialize)]
pub struct Hole {
    pub id: HoleID,
    pub text: String,
    pub kind: HoleKind,
    #[serde(with = "crate::util::iso8601")]
    pub timestamp: OffsetDateTime,
    pub reply: usize,
    pub likenum: usize,
    pub tag: Option<String>,
}

impl From<RawHole> for Hole {
    fn from(raw: RawHole) -> Self {
        let RawHole { id, text, kind, timestamp, reply, likenum, tag } = raw;
        Self { id: id.into(), text, kind, timestamp, reply, likenum, tag }
    }
}

impl PartialEq for Hole {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl PartialOrd for Hole {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.id.partial_cmp(&other.id)
    }
}

impl Ord for Hole {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.id.cmp(&other.id)
    }
}

impl Hash for Hole {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state)
    }
}

#[serde_as]
#[derive(Debug, Deserialize)]
pub struct RawHolePage {
    pub code: i32,
    #[serde(default)]
    pub count: Option<i32>,
    pub data: Vec<RawHole>,
    #[serde(default)]
    #[serde_as(as = "Option<TimestampSeconds<i64>>")]
    pub timestamp: Option<OffsetDateTime>,
}

// derive_set!{ hole, Hole, HoleEntry, RawHolePage, HoleSet, HoleList }

#[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize)]
pub struct HoleEntry {
    pub entry: Hole,
    #[serde(with = "crate::util::iso8601")]
    pub snapshot: OffsetDateTime,
}

impl IntoIterator for RawHolePage {
    type Item = HoleEntry;
    type IntoIter = <Vec<Self::Item> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        let RawHolePage {
            data, timestamp, ..
        } = self;
        let snapshot = timestamp.unwrap_or_else(|| OffsetDateTime::now_utc().replace_nanosecond(0).unwrap());
        data
            .into_iter()
            .map(|hole| HoleEntry { entry: hole.into(), snapshot })
            .collect::<Vec<_>>()
            .into_iter()
    }
}

fn lossy_deserialize_usize<'de, D>(d: D) -> Result<usize, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(d)?;
    Ok(s.parse::<usize>().unwrap_or(0))
}
