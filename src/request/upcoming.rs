use crate::request::*;

#[derive(Default)]
pub struct UpcomingRB {
    top_tier: bool,
    events: Vec<u32>,
    event_filter: EventTypeFilter,
}

impl From<UpcomingRB> for String {
    fn from(d: UpcomingRB) -> Self {
        if d.top_tier {
            return "matches?predefinedFilter=top_tier".to_string();
        }
        let mut result = String::from("matches?");
        result += &format!("&eventType={}", d.event_filter);
        for &ev in d.events.iter() {
            result += &format!("&event={}", ev);
        }
        result
    }
}
/// Creates a request builder for upcoming matches.
pub fn upcoming() -> RequestBuilder<UpcomingMatch, UpcomingRB> {
    RequestBuilder {
        data: UpcomingRB::default(),
        _p: PhantomData,
    }
}

impl RequestBuilder<UpcomingMatch, UpcomingRB> {
    #[must_use]
    pub fn top_tier(mut self) -> Self {
        self.data.top_tier = true;
        self
    }
    #[must_use]
    pub fn events(mut self, event_ids: Vec<u32>) -> Self {
        self.data.events = event_ids;
        self
    }
    #[must_use]
    pub fn event_type(mut self, event_filter: EventTypeFilter) -> Self {
        self.data.event_filter = event_filter;
        self
    }
}
