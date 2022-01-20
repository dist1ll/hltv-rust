# HLTV client

**A crate for fetching and parsing esports data from [HLTV.org](https://www.hltv.org).**

This is WIP. This crate should allow you to fetch and parse upcoming matches, results, 
event information, player information. `hltv` uses blocking calls via `attohttpc`. 
Might extend functionality to support async variants.

```Rust
// Example
let q: Request<Match> = hltv::results()
                 .stars(1)
                 .date(d1, d2)
                 .type(EventType::LAN)
                 .build()

let result = q.fetch() // type: Result<Vec<Match>, hltv::Error>
```

## Getting more detailed information

This API mimics the way you discover information on HLTV. Summary pages like `hltv.org/matches` 
contains less information in the HTML document than the detailed match-specific page. 

```rust 
/// Example
```

## License

This project is dual-licensed under the MIT and Apache 2.0 license.
