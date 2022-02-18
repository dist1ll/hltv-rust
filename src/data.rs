/*!
Types for esports data from HLTV.


*/
use chrono::{DateTime, Utc};

/// Type of event. At this moment either LAN or online.
#[derive(Debug)]
pub enum EventType {
    LAN,
    Online,
}

/// Best-of-X format of a match. Since there are 7 maps in the CSGO map pool,
/// the maximum is Bo7.
#[derive(Debug, PartialEq)]
pub enum MatchFormat {
    Bo1,
    Bo3,
    Bo5,
    Bo7,
}

/// Basic player information.
#[derive(Default, Debug, PartialEq, Clone)]
pub struct Player {
    pub id: u32,
    pub nickname: String,
}

/// Basic information about a team.
#[derive(Debug, PartialEq)]
pub struct Team {
    /// HLTV-associated ID (found in the URL of team page).
    pub id: u32,
    /// Name of the team.
    pub name: String,
}

impl Team {
    pub fn new(id: u32, name: String) -> Self {
        Team { id, name }
    }
}

/// Basic information about a team.
#[derive(Debug, PartialEq)]
pub struct Event {
    /// HLTV-associated ID (found in the URL of the event page).
    pub id: u32,
    /// Name of the Event
    pub name: String,
}

/// Contains detailed information about an event. Corresponds to data found on [HLTV's event
/// page](https://www.hltv.org/events/6345/blast-premier-spring-final-2022).
pub struct EventDetails {
    /// HLTV-associated ID (found in the URL of the event page).
    pub id: u32,
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
#[derive(Debug)]
pub struct TeamPage {
    /// Team ID according to HLTV team page URL.
    pub id: u32,
    /// Name of the team.
    pub name: String,
    /// All known players of the team. Can be less than five, or even more than five (in the case
    /// of 6-man rosters).
    pub players: Vec<Player>,
    /// URL of the logo (hltv cdn).
    pub logo: String,
}

/// Contains a summary of an upcoming match ([reference](https://www.hltv.org/matches)).
#[derive(Debug)]
pub struct UpcomingMatch {
    pub id: u32,
    /// First team of the mach, according to HLTV's display order
    pub team1: Option<Team>,
    /// Second team of the mach, according to HLTV's display order
    pub team2: Option<Team>,
    /// Name of the event.
    pub event: String,
    /// Format of a match. For example, if the format is [`Bo1`][MatchFormat::Bo1],
    /// then only one map is played and the result is either a `1-0` or `0-1`.
    pub format: MatchFormat,
    /// Time when an upcoming match is supposed to start. If the match is finished,
    /// this date is the finish time (according to HLTV).
    pub date: DateTime<Utc>,
    /// Number of HLTV stars given to the match. Stars are a measure of match prestige.
    /// The exact meaning is defined
    /// [here](https://www.hltv.org/forums/threads/931435/what-are-these-stars-by-the-matches#r12178822).
    pub stars: u32,
}

/// Contains a summary of a concluded match ([reference](https://www.hltv.org/results)).
#[derive(Debug, PartialEq)]
pub struct MatchResult {
    pub id: u32,
    /// Enum which Team won
    pub winner: WhichTeam,
    /// Name of team 1. The result page doesn't contain team IDs unfortunately.
    pub team1: String,
    /// Name of team 2. The result page doesn't contain team IDs unfortunately.
    pub team2: String,
    /// Either a match score for bo3 and higher, or a map score for bo1s.
    pub score: Score,
    /// Name of the event
    pub event: String,
    /// Format of a match. For example, if the format is [`Bo1`][MatchFormat::Bo1],
    /// then only one map is played and the result is either a `1-0` or `0-1`.
    pub format: MatchFormat,
}

/// A general W-L match score
#[derive(Debug, PartialEq)]
pub struct Score {
    pub score_won: u32,
    pub score_lost: u32,
}

/// Contains detailed information about a match. Corresponds to data found on [HLTV's
/// match page](https://www.hltv.org/matches/2239492/nip-vs-virtuspro-sltv-starseries-v-finals).
#[derive(Debug, PartialEq)]
pub struct MatchPage {
    /// ID of the match
    pub id: u32,
    /// Status of the match.
    pub status: MatchStatus,
    /// First team, if known.
    pub team1: Option<Team>,
    /// Second team, if known.
    pub team2: Option<Team>,
    /// Event at which this match is played.
    pub event: Event,
    /// Time when a match is supposed to start. This is different from the timestamp
    /// found on [`MatchResult`] (which is the time the match result was published).
    pub date: DateTime<Utc>,
    /// Format of the match. The format determines how many maps a team needs to win a match.
    pub format: MatchFormat,
    /// A match score. In case of bo1, either `1-0` or `0-1`. For bo3 it's `2-0`, `2-1`
    /// and so on.
    pub score: Option<MatchScore>,
    /// A collection of map-specific scores. Up to 7 maps can be played per map. Empty
    /// if the game hasn't started yet. Contains partial results if maps have been played
    /// but the match hasn't fully concluded yet (which can be the case for bo3+).
    pub maps: Vec<MapScore>,
    /// Performance of players over all maps.
    pub stats: Vec<Performance>,
}

/// Current status of a match.
#[derive(Debug, PartialEq)]
pub enum MatchStatus {
    Upcoming,
    Finished,
    Live,
}

/// Refers to either the first or second team in a match, according to HLTV order.
#[derive(Default, Debug, PartialEq)]
pub enum WhichTeam {
    #[default]
    None,
    First,
    Second,
}

/// A match score refers to the number of won maps of both team 1 and team 2.
/// Examples are `1-0`, `2-1`, `1-3`, etc.
#[derive(Debug, PartialEq)]
pub struct MatchScore {
    pub team1: u32,
    pub team2: u32,
}

/// Represents the result of a single map. Examples are: `16-14`, `10-16`, `19-17`
#[derive(Debug, PartialEq)]
pub struct MapScore {
    pub map: Map,
    /// Number of rounds won by team 1.
    pub team1: u32,
    /// Number of rounds won by team 2.
    pub team2: u32,
}

impl MapScore {
    pub fn new(map: Map, team1: u32, team2: u32) -> Self {
        MapScore { map, team1, team2 }
    }
}

/// A tuple of a specific players map performance.
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Performance(pub Player, pub Stats);

/// Collection of performance metrics of a player.
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Stats {
    /// Total kills.
    pub kills: u32,
    /// Total deaths.
    pub deaths: u32,
    /// Average damage per round.
    pub adr: f32,
    /// Percentage of rounds with either kill, assisst, support or trade.
    pub kast: f32,
    /// HLTV 2.0 rating.
    pub rating: f32,
}

/// All CSGO maps that are listed on HLTV
#[derive(Debug, Default, PartialEq)]
pub enum Map {
    #[default]
    Unknown,
    Cache,
    Season,
    Dust2,
    Mirage,
    Inferno,
    Nuke,
    Train,
    Cobblestone,
    Overpass,
    Tuscan,
    Vertigo,
    Ancient,
}

impl From<String> for Map {
    fn from(s: String) -> Self {
        use Map::*;
        match s.as_ref() {
            "Cache" => Cache,
            "Season" => Season,
            "Dust2" => Dust2,
            "Mirage" => Mirage,
            "Inferno" => Inferno,
            "Nuke" => Nuke,
            "Train" => Train,
            "Cobblestone" => Cobblestone,
            "Overpass" => Overpass,
            "Tuscan" => Tuscan,
            "Vertigo" => Vertigo,
            "Ancient" => Ancient,
            &_ => Unknown,
        }
    }
}

impl std::fmt::Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Map::Unknown => write!(f, "n/a"),
            Map::Cache => write!(f, "de_cache"),
            Map::Season => write!(f, "de_season"),
            Map::Dust2 => write!(f, "de_dust2"),
            Map::Mirage => write!(f, "de_mirage"),
            Map::Inferno => write!(f, "de_inferno"),
            Map::Nuke => write!(f, "de_nuke"),
            Map::Train => write!(f, "de_train"),
            Map::Cobblestone => write!(f, "de_cobblestone"),
            Map::Overpass => write!(f, "de_overpass"),
            Map::Tuscan => write!(f, "de_tuscan"),
            Map::Vertigo => write!(f, "de_vertigo"),
            Map::Ancient => write!(f, "de_ancient"),
        }
    }
}
