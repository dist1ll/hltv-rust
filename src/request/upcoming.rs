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
        result += &format!("eventType={}", d.event_filter);
        for &ev in d.events.iter() {
            result += &format!("&event={}", ev);
        }
        result
    }
}
/// Use this to build requests for upcoming matches. To find out which builder methods
/// exist, refer to the docs of [`RequestBuilder`].
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
pub fn upcoming() -> RequestBuilder<Vec<UpcomingMatch>, UpcomingRB> {
    RequestBuilder {
        data: UpcomingRB::default(),
        _p: PhantomData,
    }
}

/// Here you can find all builder methods to specify which upcoming matches you want to
/// fetch. 
impl RequestBuilder<Vec<UpcomingMatch>, UpcomingRB> {
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
