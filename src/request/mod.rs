use std::marker::PhantomData;

use crate::data::*;
use crate::ConvertCollection;
use crate::Request;

const HLTV_ROOT: &str = "https://www.hltv.org/";

pub mod upcoming;

/// Generic request builder. After using the builder methods, call .build() to
/// generate a Request<T> object.
pub struct RequestBuilder<T: ConvertCollection, V: Into<String>> {
    data: V,
    _p: PhantomData<T>,
}

impl<T: ConvertCollection, V: Into<String>> RequestBuilder<T, V> {
    /// Creates a Request object for upcoming matches
    pub fn build(self) -> Request<T> {
        Request::<T> {
            url: self.data.into(),
            _m: PhantomData,
        }
    }
}

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



