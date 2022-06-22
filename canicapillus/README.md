<div align="center"><h1 align="center">canicapillus</h1><p><em>Yungipicus canicapillus</em></p>
<p><a title="Dr. Raju Kasambe, CC BY-SA 4.0 &lt;https://creativecommons.org/licenses/by-sa/4.0&gt;, via Wikimedia Commons" href="https://commons.wikimedia.org/wiki/File:Grey-capped_Pygmy_Woodpecker_Dendrocopos_canicapillus_IMG_0716_(1).jpg"><img width="512" alt="Grey-capped Pygmy Woodpecker Dendrocopos canicapillus IMG 0716 (1)" src="https://upload.wikimedia.org/wikipedia/commons/thumb/1/19/Grey-capped_Pygmy_Woodpecker_Dendrocopos_canicapillus_IMG_0716_%281%29.jpg/512px-Grey-capped_Pygmy_Woodpecker_Dendrocopos_canicapillus_IMG_0716_%281%29.jpg"></a></p>
</div>

A flexible and extensible async client implemenation of `woodpecker`.

# User Token

To use the default implemenation, specify `WOODPECKER_USER_TOKEN` in your environment variable or `.env` file.

# Examples

## Concurrency

The following example queries the keyword "test" and queries 3 pages of search results concurrently, not necessarily parallel. 

Please notice that the default implementation contains no counter anti-scraping measures. 

```rust
let mut fetcher = fetcher::Fetcher::default();
let fetch = FetchSearch {
    keyword: String::from("test"),
};
let result = fetcher
    .fetch(&fetch)
    .swarm(Some(Swarm::Concurrent {
        count: 3,
        page_size: 50,
    }))
    .execute()
    .await
    .unwrap();
dbg!(result.len());
```
