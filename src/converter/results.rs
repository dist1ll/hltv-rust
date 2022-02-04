use tl::queryselector::QuerySelectorIterator;

use crate::data::*;
use crate::tl_extensions::*;
use crate::ConvertCollection;
use crate::Error;

impl<'a> ConvertCollection<'a> for MatchResult {
    fn convert(d: &'a tl::VDom<'a>) -> Result<Vec<MatchResult>, Error> {
        let mut result = Vec::<MatchResult>::new();
        let match_containers = get_roots(d);
        for c in match_containers {
            let h = c.to_rich(d);
            println!("{:?}", parse_id(h));
        }
        Ok(result)
    }
}

/// Returns the an iterator over roots of interest (i.e. the containers of
/// results).
fn get_roots<'a>(d: &'a tl::VDom<'a>) -> QuerySelectorIterator<tl::VDom>{
    d.query_selector("div.result-con").unwrap()
}

/// Parses the match ID from the given root node
fn parse_id(h: RichNode) -> Result<u32, Error> {
    let href = h
        .find("a-reset")
        .get_attr_str("href")
        .ok_or(Error::ParseError)?;
    match href.split('/').nth(2).ok_or(Error::ParseError)?.parse() {
        Ok(x) => Ok(x),
        Err(_) => Err(Error::ConversionError(
            "match ID isn't a valid number".to_string(),
        )),
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    /// Tests if the converter parses player info from a match page correctly.
    #[test]
    pub fn results() {
        let input = include_str!("../testdata/results.html");
        let dom = tl::parse(input, tl::ParserOptions::default()).unwrap();
        let result = MatchResult::convert(&dom).unwrap();
    }

    #[test]
    pub fn xyz() {
        let input = include_str!("../testdata/results.html");
        let dom = tl::parse(input, tl::ParserOptions::default()).unwrap();
        let mut x = get_roots(&dom);
        println!("{:?}", x.next().unwrap());
        println!("{:?}", x.next().unwrap());
    }
}
