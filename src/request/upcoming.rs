use crate::request::*;

/// Request builder for upcoming matches
#[derive(Default)]
pub struct UpcomingRB {
    top_tier: bool,
    events: Vec<u32>,
    event_filter: EventTypeFilter,
}

/// Use this to build requests for upcoming matches.
///
/// # Example
///
/// Here is an example using all available builder methods to create a request.
///
/// ```rust
/// let req = hltv::upcoming()
///     .events(vec![6343, 6335])
///     .event_type(EventTypeFilter::Online)
///     .build();
///
/// // because top_tier is a predefined filter, it's doesn't work together
/// // with the other methods.
/// let req = hltv::upcoming()
///     .top_tier()
///     .build();
/// ```
pub fn upcoming() -> UpcomingRB {
    UpcomingRB::default()
}

/// Here you can find all builder methods to specify which upcoming matches you want to
/// fetch.
impl UpcomingRB {
    #[must_use]
    pub fn top_tier(mut self) -> Self {
        self.top_tier = true;
        self
    }
    #[must_use]
    pub fn events(mut self, event_ids: Vec<u32>) -> Self {
        self.events = event_ids;
        self
    }
    #[must_use]
    pub fn event_type(mut self, event_filter: EventTypeFilter) -> Self {
        self.event_filter = event_filter;
        self
    }
    #[must_use]
    pub fn build(self) -> Request<Vec<UpcomingMatch>> {
        let query = self.query();
        Request {
            url: format!("{}{}", HLTV_ROOT, query),
            _m: PhantomData,
        }
    }
    fn query(self) -> String {
        if self.top_tier {
            return "matches?predefinedFilter=top_tier".to_string();
        }
        let mut result = String::from("matches?");
        result += &format!("eventType={}", self.event_filter);
        for &ev in self.events.iter() {
            result += &format!("&event={}", ev);
        }
        result
    }
}
