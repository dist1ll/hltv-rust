use crate::data::*;
use crate::Request;
use std::marker::PhantomData;

pub mod results;
pub mod upcoming;
pub mod match_page;

const HLTV_ROOT: &str = "https://www.hltv.org/";

/// An event/match filter for building Requests.
#[derive(Default)]
pub enum EventTypeFilter {
    #[default]
    All,
    Lan,
    Online,
}

impl std::fmt::Display for EventTypeFilter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EventTypeFilter::All => write!(f, "All"),
            EventTypeFilter::Lan => write!(f, "LAN"),
            EventTypeFilter::Online => write!(f, "Online"),
        }
    }
}
