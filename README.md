# HLTV client

[Documentation](https://docs.rs/hltv/latest/hltv/) | [Crates.io](https://crates.io/crates/hltv) | [Repository](https://github.com/dist1ll/hltv-rust)

**A crate for fetching and parsing esports data from [HLTV.org](https://www.hltv.org).**


This crate allows you to fetch and parse upcoming matches, results,
event information, player performance. This crate uses async calls via `reqwest`
and parses the HTML document with `tl`. This API mimics the way you discover information on HLTV. 
Summary pages like [HLTV Matches](https://www.hltv.org/matches) contain less information 
in the HTML document than the detailed match-specific page.


Currently, the following API calls are supported:

- `crate::upcoming`
- `crate::results`
- `crate::get_match`

## Examples

The builders in `hltv` allow you to build a generic `Request` object with a `Request::fetch` method.

```rust
#[tokio::test]
async fn results() -> Result<(), Box<dyn Error>> {
    let req = hltv::results()
        .map(Map::Inferno)
        .team(4608) // Team Na'Vi
        .year(2016) 
        .event_type(EventTypeFilter::Lan)
        .build();

    let matches = req.fetch().await?; // Vec<MatchResult>
    Ok(())
}
```
## Getting more detailed information


## License

This project is dual-licensed under the MIT and Apache 2.0 license.
