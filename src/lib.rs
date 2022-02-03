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

use std::marker::PhantomData;

pub mod converter;

// Extensions to make the [`tl`] crate more ergonomic. 
mod tl_extensions;

// Make data available to the hltv crate.
mod data;
pub use data::*;

/// Errors that happen during request, parse or conversion of data.
#[derive(Debug)]
pub enum Error {
    /// Any non-200 status code.
    HTTPError,
    /// HTML document is invalid. Refer to `tl::parse`.
    ParseError,
    /// Parsed document can't be converted into target type.
    ConversionError(String),
}


/// Implements a conversion from a DOM object to a collection of its own type.
pub trait ConvertCollection<'a>
where
    Self: Sized,
{
    /// Converts a given VDOM into a vector of its own type. This is because
    /// them DOM can contain multiple instances of that type.
    fn convert(d: &'a tl::VDom<'a>) -> Result<Vec<Self>, crate::Error>;
}

/// Implements a conversion from a DOM object to a single instance of its own type.
pub trait ConvertInstance<'a>
where
    Self: Sized,
{
    /// Converts a given VDOM into a instance of its own type. If the DOM contains
    /// multiple instances, the first one is chosen.
    fn convert(d: tl::VDom<'a>) -> Result<Self, crate::Error>;
}

/// A reusable request object, that fetches, parses and converts HLTV data
/// to the correct type.
pub struct Request<'a, T>
where
    T: ConvertCollection<'a>,
{
    url: String,
    _m: PhantomData<&'a T>,
}

impl<'a, T> Request<'a, T>
where
    T: ConvertCollection<'a>,
{
    /// Creates a new request object with given url and conversion type.
    pub fn new(url: String) -> Request<'a, T> {
        Request::<'a, T> {
            url,
            _m: PhantomData,
        }
    }
    /// Fetches HTML resource, parses DOM, and converts into type T.
    /// Returns an error if the resource is not reachable.
    /// If you want to create a custom data structure that can be fetched
    /// and read from HLTV, refer to the [`converter`] module.
    pub fn fetch(&self) -> Result<T, crate::Error> {
        Err(crate::Error::HTTPError)
    }
}
