/*!
This module defines how a match page is parsed. Note that unlike "upcoming" or "result" type matches,
a match page can have many different conditions and edge cases. The crate gives no guarantees about
completeness. If you think an edge case is not correctly parsed, feel free to create an issue on github
and attach a sample of html that is not correctly recognized.
*/

use crate::data::*;
use crate::tl_extensions::*;
use crate::ConvertInstance;
use crate::{Error, Error::ConversionError};

impl ConvertInstance for MatchPage {
    fn convert<'a>(d: &'a tl::VDom<'a>) -> Result<MatchPage, Error> {
        let match_containers = d.query_selector("div.upcomingMatch").unwrap();
        
        Err(Error::ParseError)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Tests if a finished bo3 match is correctly parsed.
    #[test]
    pub fn concluded_bo3() {
        let input = include_str!("../testdata/matchPages/finished_bo3.html");
        let dom = tl::parse(input, tl::ParserOptions::default()).unwrap();
        let result = MatchPage::convert(&dom).unwrap();
    }
}
