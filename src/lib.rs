/*!
**A crate for fetching and parsing esports data from [HLTV.org](https://www.hltv.org).**

This crate allows you to fetch and parse upcoming matches, results,
event information, player performance. This crate uses blocking calls via [`attohttpc`]
and parses the HTML document with [`tl`].

## Examples

The builders in `hltv` allow you to build a generic [`Request`] object with a [`fetch`][`Request::fetch`] method.
```rust
let q = hltv::results()
              .stars(1)
              .date(d1, d2)
              .event_type(EventType::LAN)
              .build()

let result = q.fetch() // type: Result<Vec<Match>, hltv::Error>
```

## Getting more detailed information

This API mimics the way you discover information on HLTV. Summary pages (like [HLTV Matches](https://www.hltv.org/matches))
contains less information in the HTML document than the detailed match-specific page.

```rust
/// Example
```
*/
#![allow(dead_code)]
#![feature(derive_default_enum)]

use std::marker::PhantomData;

pub mod converter;
pub mod data;
// Extensions to make the [`tl`] crate more ergonomic.
mod tl_extensions;

// Export builder methods
mod request;
pub use request::upcoming::upcoming;
pub use request::results::results;
pub use request::EventTypeFilter;

/// Implements a conversion from a DOM object to a collection of its own type.
pub trait ConvertCollection
where
    Self: Sized,
{
    /// Converts a given VDOM into a vector of its own type. This is because
    /// them DOM can contain multiple instances of that type.
    fn convert<'a>(d: &'a tl::VDom<'a>) -> Result<Vec<Self>, crate::Error>;
}

/// Implements a conversion from a DOM object to a single instance of its own type.
pub trait ConvertInstance
where
    Self: Sized,
{
    /// Converts a given VDOM into a instance of its own type. If the DOM contains
    /// multiple instances, the first one is chosen.
    fn convert<'a>(d: &'a tl::VDom<'a>) -> Result<Self, crate::Error>;
}

/// A reusable request object, that fetches, parses and converts HLTV data
/// to the correct type.
#[derive(Debug)]
pub struct Request<T>
where
    T: ConvertInstance,
{
    url: String,
    _m: PhantomData<T>,
}

impl<T> Request<T>
where
    T: ConvertInstance,
{
    /// Creates a new request object with given url and conversion type.
    pub fn new(url: String) -> Request<T> {
        Request::<T> {
            url,
            _m: PhantomData,
        }
    }
    /// Fetches HTML resource, parses DOM, and converts into type T.
    /// Returns an error if the resource is not reachable.
    /// If you want to create a custom data structure that can be fetched
    /// and read from HLTV, refer to the [`converter`] module.
    pub async fn fetch(&self) -> Result<T, Box<dyn std::error::Error>> {
        let html = reqwest::get(self.url.clone()).await?.text().await?;
        let vdom = tl::parse(&html, tl::ParserOptions::default())?;
        let x = T::convert(&vdom)?;
        Ok(x)
    }
}

/// Errors that happen during request, parse or conversion of data.
#[derive(Debug)]
pub enum Error {
    /// Any non-200 status code.
    HTTPError,
    /// HTML document is invalid. Refer to `tl::parse`.
    ParseError,
    /// Parsed document can't be converted into target type.
    ConversionError(&'static str),
}

impl std::error::Error for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::HTTPError => write!(f, "error with http client or remote server"),
            Error::ParseError => write!(f, "error parsing received data"),
            Error::ConversionError(_) => write!(f, "error converting data into correct type"),
        } 
    }
}


