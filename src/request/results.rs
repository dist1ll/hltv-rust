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

/// Creates a request builder for upcoming matches.
pub fn results() -> RequestBuilder<Vec<MatchResult>, ResultRB> {
    RequestBuilder {
        data: ResultRB::default(),
        _p: PhantomData,
    }
}

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
    /// Only select results with the given event IDs.
    #[must_use]
    pub fn events(mut self, event_ids: Vec<u32>) -> Self {
        self.data.events = event_ids;
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
