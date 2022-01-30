use crate::data::*;
use crate::ConvertCollection;
use crate::Error;

impl<'a> ConvertCollection<'a> for UpcomingMatch {
    fn convert(d: tl::VDom<'a>) -> Result<Vec<UpcomingMatch>, Error> {
        let result = Vec::<UpcomingMatch>::new();
        let match_containers = d.query_selector("div.upcomingMatch").unwrap();
        for c in match_containers {
            break;
        }
        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Tests if the converter parses player info from a match page correctly.
    #[test]
    pub fn matches() {
        let input = include_str!("../testdata/matches.html");
        let dom = tl::parse(input, tl::ParserOptions::default()).unwrap();
        let result = UpcomingMatch::convert(dom).unwrap();
    }
}
