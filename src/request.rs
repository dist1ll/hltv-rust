use std::marker::PhantomData;

use crate::ConvertCollection;
use crate::Request;
use crate::data::*;

const HLTV_ROOT: &str = "https://www.hltv.org/";

/// Generic request builder. After using the builder methods, call .build() to
/// generate a Request<T> object.
pub struct RequestBuilder<T: ConvertCollection> {
    url: String,
    _p: PhantomData<T>,
}

impl<T: ConvertCollection> RequestBuilder<T> {
    /// Creates a Request object from a builder.
    pub fn build(self) -> Request<T> {
        Request::<T>{url: self.url, _m: PhantomData}
    }

}

/// Creates a request builder for upcoming matches.
pub fn upcoming() -> RequestBuilder<UpcomingMatch> {
    RequestBuilder{url: "https://www.hltv.org/matches?".to_string(), _p: PhantomData}
}

impl RequestBuilder<UpcomingMatch> {
    #[must_use]
    pub fn top_tier(mut self) -> Self {
        self.url.push_str("&predefinedFilter=top_tier");
        self
    }
    #[must_use]
    pub fn event(mut self, id: u32) -> Self {
        self.url.push_str("&event=");
        self.url.push_str(&id.to_string());
        self
    }
    #[must_use]
    pub fn event_type(mut self, event_type: EventType) -> Self {
        self.url.push_str("&eventType=");
        self.url.push_str(event_type.into());
        self
    }
}
