use crate::data::Map;
use crate::request::*;

#[derive(Default)]
pub struct ResultRB {
    stars: u32,
    from: String,
    to: String,
    events: Vec<u32>,
    players: Vec<u32>,
    teams: Vec<u32>,
    maps: Vec<Map>,
    match_filter: EventTypeFilter,
}

impl From<ResultRB> for String {
    fn from(d: ResultRB) -> Self {
        let mut result = String::from("results?");
        result += &format!("stars={}", d.stars);
        result += &format!("&matchType={}", d.match_filter);
        if !d.from.is_empty() && !d.to.is_empty() {
            result += &format!("&startDate={}&endDate={}", d.from, d.to);
        } 
        for &ev in d.events.iter() {
            result += &format!("&event={}", ev);
        }
        for &pl in d.players.iter() {
            result += &format!("&player={}", pl);
        }
        for &team in d.teams.iter() {
            result += &format!("&team={}", team);
        }
        for map in d.maps.iter() {
            result += &format!("&map={}", map);
        }
        result
    }
}

/// Use this to build requests for match results. To find out which builder methods
/// exist, refer to the docs of [`RequestBuilder`].
///
/// # Example
///
/// Here is an example using all available builder methods to create a request.
///
/// ```rust
/// let req = hltv::results()
///     .from(2016, 2, 20)
///     .to(2017, 12, 20)
///     .map(Map::Inferno)
///     .event(2683)
///     .team(4608)
///     .player(7998)
///     .event_type(EventTypeFilter::Lan)
///     .build();
///
/// // You can also specify multiple maps/teams/players instead
/// let req = hltv::results()
///     .from(2016, 2, 20)
///     .to(2017, 12, 20)
///     .maps(vec![Map::Inferno, Map::Cobblestone])
///     .events(vec![2683, 3016])
///     .teams(vec![4608, 5995])
///     .players(vec![7998, 7167])
///     .event_type(EventTypeFilter::Lan)
///     .build();

/// ```
pub fn results() -> RequestBuilder<Vec<MatchResult>, ResultRB> {
    RequestBuilder {
        data: ResultRB::default(),
        _p: PhantomData,
    }
}

/// Here you can find all builder methods to specify which match results you want to
/// fetch. 
impl RequestBuilder<Vec<MatchResult>, ResultRB> {
    #[must_use]
    pub fn stars(mut self, stars: u32) -> Self {
        self.data.stars = stars;
        self
    }
    /// Get results from a particular year.
    #[must_use]
    pub fn year(mut self, year: u32) -> Self {
        self.data.from = format!("{}-01-01", year);
        self.data.to = format!("{}-12-31", year);
        self
    }
    /// Specify start date of results. Sould be used with .to()
    #[must_use]
    pub fn from(mut self, year: u32, month: u32, day: u32) -> Self {
        self.data.from = format!("{}-{}-{}", year, month, day);
        self
    }
    /// Specify end date of results. Needs to be used with .to()
    #[must_use]
    pub fn to(mut self, year: u32, month: u32, day: u32) -> Self {
        self.data.to = format!("{}-{}-{}", year, month, day);
        self
    }
    /// Only select results with the given event ID.
    #[must_use]
    pub fn event(mut self, event_id: u32) -> Self {
        self.data.events = vec![event_id];
        self
    }

    /// Only select results with the given event IDs.
    #[must_use]
    pub fn events(mut self, event_ids: Vec<u32>) -> Self {
        self.data.events = event_ids;
        self
    }
    /// Only select results where the given player ID participated.
    #[must_use]
    pub fn player(mut self, player_id: u32) -> Self {
        self.data.players = vec![player_id];
        self
    }

    /// Only select results where the given player IDs participated.
    #[must_use]
    pub fn players(mut self, player_ids: Vec<u32>) -> Self {
        self.data.players = player_ids;
        self
    }
    /// Only select results where the given team ID participated.
    #[must_use]
    pub fn team(mut self, team_id: u32) -> Self {
        self.data.teams = vec![team_id];
        self
    }
    /// Only select results where the given team IDs participated.
    #[must_use]
    pub fn teams(mut self, team_ids: Vec<u32>) -> Self {
        self.data.teams = team_ids;
        self
    }

    /// Only select results where the given map was played.
    #[must_use]
    pub fn map(mut self, map: Map) -> Self {
        self.data.maps = vec![map];
        self
    }
    /// Only select results where the given maps were played.
    #[must_use]
    pub fn maps(mut self, maps: Vec<Map>) -> Self {
        self.data.maps = maps;
        self
    }
    #[must_use]
    pub fn event_type(mut self, event_filter: EventTypeFilter) -> Self {
        self.data.match_filter = event_filter;
        self
    }
}
