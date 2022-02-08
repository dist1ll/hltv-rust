/*!
This module defines how a match page is parsed. Note that unlike "upcoming" or "result" type matches,
a match page can have many different conditions and edge cases. The crate gives no guarantees about
completeness. If you think an edge case is not correctly parsed, feel free to create an issue on github
and attach a sample of html that is not correctly recognized.
*/

use tl::NodeHandle;

use crate::data::*;
use crate::tl_extensions::*;
use crate::ConvertInstance;
use crate::{Error, Error::ConversionError};

impl ConvertInstance for MatchPage {
    fn convert<'a>(d: &'a tl::VDom<'a>) -> Result<MatchPage, Error> {
        let root = get_root(d)?.to_rich(d);
        Err(Error::ParseError)
    }
}

fn get_root(d: &tl::VDom) -> Result<NodeHandle, Error> {
    d.query_selector("div.match-page")
        .unwrap()
        .next()
        .ok_or(ConversionError("no root node match-page found"))
}

/// Returns the team information, given the appropriate root match-page element.
fn get_team(root: RichNode, class: &str) -> Option<Team> {
    let t = root.find(class);
    Some(Team {
        id: t
            .child(0)?
            .get_attr_str("href")?
            .split('/')
            .nth(2)?
            .parse()
            .ok()?,
        name: t.find("teamName").inner_text()?,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn xyz() {
        let input = include_str!("../testdata/matchPages/finished_bo3.html");
        let dom = tl::parse(input, tl::ParserOptions::default()).unwrap();
        let root = get_root(&dom).unwrap().to_rich(&dom);
        println!("{:?}", get_team(root, "team1-gradient"));
    }
    /// Tests if a finished bo3 match is correctly parsed.
    #[test]
    pub fn concluded_bo3() {
        let input = include_str!("../testdata/matchPages/finished_bo3.html");
        let dom = tl::parse(input, tl::ParserOptions::default()).unwrap();
        let result = MatchPage::convert(&dom).unwrap();
    }
}
