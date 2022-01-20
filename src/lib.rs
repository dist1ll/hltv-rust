#![allow(warnings)]

use chrono::{DateTime, Utc};

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

/// This is the map score of a match. Examples are: 1-0, 2-1, 1-2, 3-0, etc.
pub struct MatchResult {
    pub winner: WhichTeam,
    /// Number of maps won by first team.
    pub team1_maps: u64,
    /// Number of maps won by first team.
    pub team2_maps: u64,
}
