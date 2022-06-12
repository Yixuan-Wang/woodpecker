pub mod api;
pub mod common;
pub mod fetcher;
pub mod hole;
pub mod prebuilt;

#[cfg(test)]
mod tests {
    use crate::{
        common::{Resource, Swarm},
        fetcher,
        hole::{HoleFlag, HoleSet},
        prebuilt::*,
    };

    #[tokio::test]
    async fn it_works() {
        let mut fetcher = fetcher::Fetcher::default();
        let fetch = FetchSearch {
            keyword: String::from("test"),
        };
        let mut results = HoleSet::blank(HoleFlag::default());
        for _ in 0..5 {
            let result = fetcher
                .fetch(&fetch)
                .swarm(Some(Swarm::Concurrent {
                    count: 3,
                    page_size: 10,
                }))
                .flag(HoleFlag::default())
                .execute()
                .await
                .unwrap();
            dbg!(result.len());
            results |= result;
        }
        dbg!(results.len());
    }
}
