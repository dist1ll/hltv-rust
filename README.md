# HLTV client

**A crate for fetching and parsing esports data from [HLTV.org](https://www.hltv.org).**

This crate allows you to fetch and parse upcoming matches, results,
event information, player performance. This crate uses blocking calls via 
[`attohttpc`](https://crates.io/crates/attohttpc)
and parses the HTML document with 
[`tl`](https://crates.io/crates/tl).

A collection of detailed examples and explanations can be found on 
[the official docs](https://www.docs.rs/hltv).

## Examples

The builders in `hltv` allow you to build a generic `Request` 
object with a `fetch` method.

```rust
let q = hltv::results()
              .stars(1)
              .date(d1, d2)
              .event_type(EventType::LAN)
              .build()

let result = q.fetch() // type: Result<Vec<Match>, hltv::Error>
```

## Getting more detailed information

This API mimics the way you discover information on HLTV. Summary pages 
(like [HLTV Matches](https://www.hltv.org/matches))
contains less information in the HTML document than the detailed match-specific page.

```rust
/// Example
```

## License

This project is dual-licensed under the MIT and Apache 2.0 license.
