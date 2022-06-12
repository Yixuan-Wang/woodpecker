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
        hole::{HoleFlag, HoleSet, reply::{ReplyFlag, ReplySet}, HoleID},
        prebuilt::*,
    };

    #[tokio::test]
    async fn it_works() {
        let mut fetcher = fetcher::Fetcher::default();
        let fetch = FetchReply {
            hole_id: HoleID(114514),
        };
        let result = fetcher
                .fetch(&fetch)
                .flag(ReplyFlag::default())
                .execute()
                .await;
        dbg!(result);
    }
}
