/*!
**A crate for fetching and parsing esports data from [HLTV.org](https://www.hltv.org).**

This crate allows you to fetch and parse upcoming matches, results,
event information, player performance. This crate uses blocking calls via [`attohttpc`]
and parses the HTML document with [`tl`]. 

## Examples

The builders in `hltv` allow you to build a generic [`Request`] object with a [`fetch`][`Request::fetch`] method.
```rust
let q = hltv::results()
              .stars(1)
              .date(d1, d2)
              .event_type(EventType::LAN)
              .build()

let result = q.fetch() // type: Result<Vec<Match>, hltv::Error>
```

## Getting more detailed information

This API mimics the way you discover information on HLTV. Summary pages (like [HLTV Matches](https://www.hltv.org/matches)) 
contains less information in the HTML document than the detailed match-specific page. 

```rust 
/// Example
```

*/
use std::marker::PhantomData;
use chrono::{DateTime, Utc};
use tl::ParserOptions;

pub mod converter;

pub enum Error {
    /// Any non-200 status code.
    HTTPError,
    /// HTML document is invalid. Refer to `tl::parse`.
    ParseError,
    /// Parsed document can't be converted into target type.
    ConversionError,
}

pub struct Request<'a, T> where T: TryFrom<tl::VDom<'a>>{
    pub url: String,
    pub _m: PhantomData<&'a T>,
}

impl<'a, T> Request<'a, T> where T: TryFrom<tl::VDom<'a>> {
    /// Fetches HTML resource, parses DOM, and converts into type T.
    /// Returns an error if the resource is not reachable.
    /// If you want to create a custom data structure that can be fetched
    /// and read from HLTV, refer to the [`converter`] module.
    pub fn fetch(&self) -> Result<T, crate::Error> {
        Err(crate::Error::HTTPError)
    }
}

impl<'a> TryFrom<tl::VDom<'a>> for Match {
    type Error = crate::Error;
    fn try_from(value: tl::VDom<'a>) -> Result<Self, Self::Error> {
        Err(Error::ConversionError)
    }
}

impl<'a> TryFrom<tl::VDom<'a>> for Player {
    type Error = crate::Error;
    fn try_from(value: tl::VDom<'a>) -> Result<Self, Self::Error> {
        Err(Error::ConversionError)
    }
}

pub enum EventType {
    LAN,
    Online,
}

pub enum MatchFormat {
    Bo1,
    Bo3,
    Bo5,
    Bo7,
}


/// Basic player information.
pub struct Player {
    pub id: u64,
    pub nickname: String,
}

/// Basic information about a team.
pub struct Team {
    /// HLTV-associated ID (found in the URL of team page).
    pub id: u64,
    /// Name of the team.
    pub name: String,
}

/// Basic information about a team.
pub struct Event {
    /// HLTV-associated ID (found in the URL of the event page).
    pub id: u64,
    /// Name of the Event
    pub name: String,
}

/// Contains detailed information about an event. Corresponds to data found on HLTV's event
/// page.
pub struct EventDetails {
    /// HLTV-associated ID (found in the URL of the event page).
    pub id: u64,
    /// Name of the event.
    pub name: String,
    /// Date when the event starts.
    pub start_date: DateTime<Utc>,
    /// Date when the event finished.
    pub end_date: DateTime<Utc>,
    /// Price pool of the event. Can be a USD figure, or guaranteed spots in another tournament.
    pub price_pool: String,

}

/// Contains extensive information about a team.
pub struct TeamDetails {
    pub id: u64,
    pub name: String,
    pub players: [Player; 5],
}

/// Contains a summary of match data.
pub struct Match {
    pub id: u64,
    pub team1: Option<String>,
    pub team2: Option<String>,
    /// Name of the event.
    pub event: String,
    pub format: MatchFormat,
    pub result: Option<MatchResult>,

    /// Time when an upcoming match is supposed to start. If the match is finished,
    /// this date is the finish time (according to HLTV).
    pub date: DateTime<Utc>,
    pub stars: u64,
}

/// Contains detailed information about a match. Corresponds to data found on HLTV's
/// match page.
pub struct MatchDetails {
    pub id: u64,
    pub team1: Option<Team>,
    pub team2: Option<Team>,
    pub event: Event,
    pub format: MatchFormat,
    pub result: Option<MatchResult>,
}

/// Refers to either the first or second team in a match, according to HLTV order.
pub enum WhichTeam {
    First,
    Second,
    None,
}

/// Represents the result of a single map. Examples are: 16-14, 10-16, 19-17
pub struct MapScore {
    pub map: String,
    pub team1_rounds: u64,
    pub team2_rounds: u64,
}

pub struct Stats {
    /// Total kills.
    pub kills: u64,
    /// Total deaths.
    pub deaths: u64,
    /// Average damage per round.
    pub adr: f64,
    /// Percentage of rounds with either kill, assisst, support or trade.
    pub kast: f64,
    /// HLTV 2.0 rating.
    pub rating: f64,
}

/// This is the map score of a match. Examples are: 1-0, 2-1, 1-2, 3-0, etc.
pub struct MatchResult {
    pub winner: WhichTeam,
    /// Number of maps won by first team.
    pub team1_maps: u64,
    /// Number of maps won by first team.
    pub team2_maps: u64,
}
