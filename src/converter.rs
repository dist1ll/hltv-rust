/*!
Methods to convert HTML to hltv data types.

This module contains all implemenations for converting HTML documents (type [`tl::VDom`])
to the types contained in the `data` module. Correct conversion is dependent on the HTML
layout of the HLTV webpage, and therefore its stability depends on HLTV not changing their
site.

If the conversion breaks at any point in the future, feel free to create an issue on the
GitHub repository or submit a pull request.
*/

use std::os::windows::prelude::OpenOptionsExt;

use crate::data::*;
use crate::ConvertCollection;
use crate::Error;

impl<'a> ConvertCollection<'a> for Player {
    fn convert(d: tl::VDom<'a>) -> Result<Vec<Player>, Error> {
        let mut result = Vec::<Player>::new();
        // query selector for match page
        for_matchpage(&d, &mut result)?;
        // query selector for team page
        for_teampage(&d, &mut result);

        Ok(result)
    }
}

/// Parses the DOM according to the schema found on the match page.
fn for_matchpage(d: &tl::VDom, r: &mut Vec<Player>) -> Result<(), Error> {
    let selector = d.query_selector("td.player").unwrap();
    for x in selector {
        let node = x.get(d.parser()).unwrap();
        let id = node
            .children()
            .and_then(|mut i| i.next())
            .and_then(|h| h.get(d.parser()))
            .and_then(|n| n.as_tag())
            .and_then(|t| t.attributes().get("data-player-id")).flatten();

        if let Some(bytes) = &id {
            let s = bytes.as_utf8_str();
            let rez: u32 = s.parse().unwrap();
            println!("{:?}", rez);
        } else {
            return Err(Error::ConversionError);
        }
        let s = node.inner_text(d.parser()).to_string();
        let p = Player { id: 0, nickname: s };
        r.push(p);
    }
    Ok(())
}

/// Parses the DOM according to the schema found on the team page.
fn for_teampage(d: &tl::VDom, r: &mut Vec<Player>) {
    let selector = d.query_selector("div.playerFlagName").unwrap();
    for x in selector {
        let node = x.get(d.parser()).unwrap();
        let s = node.inner_text(d.parser()).to_string();
        let p = Player { id: 0, nickname: s };
        r.push(p);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Tests if the converter parses player info from a match page correctly.
    #[test]
    pub fn player_match() {
        let input = include_str!("./testdata/player.html");
        let dom = tl::parse(input, tl::ParserOptions::default()).unwrap();
        let result = Player::convert(dom).unwrap();
        assert_eq!(result[0].nickname, "dist1ll");
        assert_eq!(result[1].nickname, "3rr0r");
    }

    /// Tests if the converter parses player info from a team page correctly.
    #[test]
    pub fn player_team() {
        let input = include_str!("./testdata/player_team.html");
        let dom = tl::parse(input, tl::ParserOptions::default()).unwrap();
        let result = Player::convert(dom).unwrap();
        assert_eq!(result.len(), 2);
        assert_eq!(result[0].nickname, "dist1ll");
        assert_eq!(result[1].nickname, "3rr0r");
    }
}
