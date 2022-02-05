use crate::data::*;
use crate::ConvertCollection;
use crate::{Error, Error::ConversionError};

use crate::tl_extensions::*;

impl ConvertCollection for Player {
    fn convert<'a>(d: &'a tl::VDom<'a>) -> Result<Vec<Player>, Error> {
        let mut result = Vec::<Player>::new();
        // query selector for match page
        for_teampage(d, &mut result)?;
        // query selector for team page
        for_matchpage(d, &mut result)?;

        Ok(result)
    }
}

/// Parses the DOM according to the schema found on the match page.
fn for_matchpage(d: &tl::VDom, r: &mut Vec<Player>) -> Result<(), Error> {
    let selector = d.query_selector("td.player").unwrap();
    for x in selector {
        let node = x.to_rich(d);
        let id: u32 = match node.find("flagAlign").get_attr("data-player-id")? {
            Some(x) => x,
            None => return Err(ConversionError("No ID found for player")),
        };
        let nickname = node
            .find("text-ellipsis")
            .inner_text()
            .ok_or(ConversionError("No player name found"))?;
        r.push(Player { id, nickname });
    }
    Ok(())
}

/// Parses the DOM according to the schema found on the team page.
fn for_teampage(d: &tl::VDom, r: &mut Vec<Player>) -> Result<(), Error> {
    let mut selector = d.query_selector("div.bodyshot-team-bg").unwrap();
    let parent = selector.next();
    if parent.is_none() {
        return Ok(());
    }
    for node in d.select_nodes(parent.unwrap(), "col-custom") {
        let tag = node.get(d.parser()).unwrap().as_tag().unwrap();

        let name: String = match tag.get_attr_str("title") {
            Some(x) => x,
            None => return Err(ConversionError("missing title attribute in player div")),
        };

        let id: String = match tag.get_attr_str("href") {
            Some(x) => x,
            None => return Err(ConversionError("missing href link in player div")),
        };

        let id = id.split('/').nth(2).ok_or(Error::ParseError)?;
       
        let p = Player {
            id: match id.parse() {
                Ok(id) => id,
                _ => return Err(ConversionError("incorrect ID / format of href was changed")),
            },
            nickname: name,
        };
        r.push(p);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Tests if the converter parses player info from a match page correctly.
    #[test]
    pub fn player_match() {
        let input = include_str!("../testdata/player_match.html");
        let dom = tl::parse(input, tl::ParserOptions::default()).unwrap();
        let result = Player::convert(&dom).unwrap();
        assert_eq!(result.len(), 2);
        assert_eq!(
            result[0],
            Player {
                id: 1337,
                nickname: "dist1ll".to_string()
            }
        );
        assert_eq!(
            result[1],
            Player {
                id: 1338,
                nickname: "3rr0r".to_string()
            }
        );
    }

    /// Tests if the converter parses player info from a team page correctly.
    #[test]
    pub fn player_team() {
        let input = include_str!("../testdata/player_team.html");
        let dom = tl::parse(input, tl::ParserOptions::default()).unwrap();
        let result = Player::convert(&dom).unwrap();
        assert_eq!(result.len(), 3);
        assert_eq!(
            result[0],
            Player {
                id: 123,
                nickname: "dist1ll".to_string()
            }
        );
        assert_eq!(
            result[1],
            Player {
                id: 124,
                nickname: "3rr0r".to_string()
            }
        );
        assert_eq!(
            result[2],
            Player {
                id: 125,
                nickname: "rabbit".to_string()
            }
        );
    }
}
