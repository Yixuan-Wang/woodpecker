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

pub mod reply;
mod derive_set;

use crate::derive_set;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord, Deserialize)]
pub struct HoleID(#[serde(deserialize_with = "lossy_deserialize_usize")] pub usize);

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

derive_set!{ hole, Hole, HoleEntry, HoleFlag, HolePage, HoleSet }

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


fn lossy_deserialize_usize<'de, D>(d: D) -> Result<usize, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(d)?;
    Ok(s.parse::<usize>().unwrap_or(0))
}
