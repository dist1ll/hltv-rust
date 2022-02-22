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
        let root = get_root(d)?.to_rich(d);
        Ok(TeamPage {
            id: 5,
            name: "navi".to_string(),
            ranking: get_team_rank(root)?,
            players: get_players(root)?,
            logo: get_logo(root)?,
        })
    }
}

fn get_root(d: &tl::VDom) -> Result<NodeHandle, Error> {
    d.query_selector("div.teamProfile")
        .unwrap()
        .next()
        .ok_or(ConversionError("no teamProfile node found"))
}

/// Returns the team's logo url.
fn get_logo(h: RichNode) -> Result<String, Error> {
    h.find("profile-team-logo-container")
        .find("teamlogo")
        .get_attr_str("src")
        .ok_or(ConversionError("couldn't find logo container or logo"))
}

/// Returns the hltv team ranking
fn get_team_rank(h: RichNode) -> Result<u32, Error> {
    let rank_txt = h
        .find("profile-team-stats-container")
        .find("right")
        .find_tag("a")
        .inner_text()
        .ok_or(ConversionError("couldn't find hltv ranking text container"))?;

    match rank_txt[1..rank_txt.len()].parse::<u32>() {
        Ok(number) => Ok(number),
        Err(_) => Err(ConversionError("couldn't parse hltv ranking")),
    }
}

/// Returns a collection of players in this team. Does not collect players who
/// have invalid hltv profile link, id or name.
fn get_players(h: RichNode) -> Result<Vec<Player>, Error> {
    let mut result = Vec::new();
    let n = h.find("bodyshot-team");
    for i in 0..5 {
        let child = n
            .child(i)
            .ok_or(ConversionError("not enough player html tags in DOM"))?;
        let nickname = child
            .get_attr_str("title")
            .ok_or(ConversionError("player tag has no title attr"))?;
        let id: u32 = child
            .get_attr_str("href")
            .ok_or(ConversionError("player tag has no href attr"))?
            .split('/')
            .nth(2)
            .ok_or(ConversionError("href of player link has incorrect format"))?
            .parse()
            .map_err(|_| ConversionError("href player id is not a number"))?;
        result.push(Player { id, nickname });
    }
    Ok(result)
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
