pub mod api;
pub mod common;
pub mod fetcher;
pub mod prebuilt;
use std::collections::BTreeSet;

pub(crate) use major::hole::{self, HoleEntry, reply::ReplyEntry};

pub type HoleSet = BTreeSet<HoleEntry>;
pub type ReplySet = BTreeSet<ReplyEntry>;

#[cfg(test)]
mod tests {
    use std::{fs::File, io::Write};

    use crate::{
        fetcher,
        hole::{HoleID},
        HoleSet, ReplySet,
        prebuilt::*,
    };

    #[tokio::test]
    #[allow(unused_must_use)] 
    async fn fetch_reply() {
        let mut fetcher = fetcher::Fetcher::default();
        let fetch = FetchReply {
            hole_id: HoleID(3761702),
        };
        let result = fetcher
                .fetch(&fetch)
                .execute()
                .await;

        match result {
            Ok(res) => { 
                let string = serde_json::to_string(&res).unwrap();
                let back = serde_json::from_str::<ReplySet>(&string).unwrap();
                assert_eq!(res, back);
                dbg!(back);
             },
            Err(err) => { dbg!(err); }
        };
    }

    #[tokio::test]
    async fn fetch_hole() {
        let mut fetcher = fetcher::Fetcher::default();
        let fetch = FetchFeed;
        let result = fetcher
            .fetch(&fetch)
            .execute()
            .await
            .unwrap();
        let string = serde_json::to_string(&result).unwrap();
        let back = serde_json::from_str::<HoleSet>(&string).unwrap();
        assert_eq!(result, back);
        let mut file = File::create("holes.test.json").unwrap();
        file.write_all(string.as_bytes()).unwrap();
        dbg!(string);
        // dbg!(result);
    }

    #[tokio::test]
    async fn fetch_single_hole() {
        let mut fetcher = fetcher::Fetcher::default();
        let fetch = FetchSingle { id: HoleID(3558999) };
        let result = fetcher
            .fetch(&fetch)
            .execute()
            .await
            .unwrap();
        let string = serde_json::to_string(&result).unwrap();
        let back = serde_json::from_str::<HoleSet>(&string).unwrap();
        assert_eq!(result, back);
        dbg!(string);
        // dbg!(result);
    }
}
