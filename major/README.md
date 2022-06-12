<div align="center"><h1 align="center">major</h1><p><em>Dendrocopos major</em></p>
<p><a title="Francesco Veronesi from Italy, CC BY-SA 2.0 &lt;https://creativecommons.org/licenses/by-sa/2.0&gt;, via Wikimedia Commons" href="https://commons.wikimedia.org/wiki/File:Great_Spotted_Woodpecker_-_Kisjuszallas_-_Hungary_S4E0087_(15671932980).jpg"><img width="256" alt="Great Spotted Woodpecker - Kisjuszallas - Hungary S4E0087 (15671932980)" src="https://upload.wikimedia.org/wikipedia/commons/thumb/c/c2/Great_Spotted_Woodpecker_-_Kisjuszallas_-_Hungary_S4E0087_%2815671932980%29.jpg/256px-Great_Spotted_Woodpecker_-_Kisjuszallas_-_Hungary_S4E0087_%2815671932980%29.jpg"></a></p>
</div>

Core library of `woodpecker`. Provides wrappers, data types and traits regarding treehole APIs.

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
    .flag()
    .execute()
    .await
    .unwrap();
dbg!(result.len());
```

## Merging Policy

`HoleSet`s can be merged together by `|` operand iff they are marked with the same specifier(`HoleFlag`). By default, two holes are considered equal if they have the same `id`. You can tweek this behavior by specifying another `HoleFlag` value.

The following example returns a `HoleSet` of size 150, instead of 30.

```rust
let mut fetcher = fetcher::Fetcher::default();
let fetch = FetchSearch {
    keyword: String::from("test"),
};
let mut results = HoleSet::blank(HoleFlag::RECORD);
for _ in 0..5 {
    let result = fetcher
        .fetch(&fetch)
        .swarm(Some(Swarm::Concurrent {
            count: 3,
            page_size: 10,
        }))
        .flag(HoleFlag::RECORD)
        .execute()
        .await
        .unwrap();
    dbg!(result.len());
    results |= result;
}
dbg!(results.len());
```
