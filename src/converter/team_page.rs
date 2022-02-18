/*!
This module defines how a team page is parsed.
*/
use tl::NodeHandle;

use crate::data::*;
use crate::tl_extensions::*;
use crate::ConvertInstance;
use crate::{Error, Error::ConversionError};

impl ConvertInstance for TeamPage {
    fn convert<'a>(d: &'a tl::VDom<'a>) -> Result<TeamPage, Error> {
        Err(Error::ParseError)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Tests if a top team with very complete data gets parsed correctly
    #[test]
    pub fn top_team() {
        let input = include_str!("../testdata/teamPages/navi.html");
        let dom = tl::parse(input, tl::ParserOptions::default()).unwrap();
        let result = TeamPage::convert(&dom).unwrap();
        println!("{:?}", result);
    }
}
