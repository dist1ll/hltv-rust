use crate::data::*;
use crate::tl_extensions::*;
use crate::ConvertCollection;
use crate::Error;

impl<'a> ConvertCollection<'a> for UpcomingMatch {
    fn convert(d: tl::VDom<'a>) -> Result<Vec<UpcomingMatch>, Error> {
        let result = Vec::<UpcomingMatch>::new();
        let match_containers = d.query_selector("div.upcomingMatch").unwrap();
        for c in match_containers {
            let stars = parse_stars(&d, c)?;
        }
        Ok(result)
    }
}

/// Returns a Team contained in the NodeHandle. Use tag `"team1"` or `"team2"`
/// to search for either.
fn parse_team(h: RichNode, team_id: &str) -> Option<Team> {
    Some(Team{
        id:  h.get_attr(team_id).unwrap_or(None)?,
        name: h.find(team_id).find("matchTeamName").inner_text()?,
    })
}

/// Parses the match ID from the given root node
fn parse_id(h: RichNode) -> Result<u32, Error> {
    let href = h.find("match").get_attr_str("href").ok_or(Error::ParseError)?;
    match href.split('/').nth(1).ok_or(Error::ParseError)?.parse() {
        Ok(x) => Ok(x),
        Err(_) => Err(Error::ParseError),
    }
}

fn parse_event(h: RichNode) -> Result<String, Error> {
    let m = h.find("matchEvent").find("matchEventName");
    match m.n {
        Some(_) => m.inner_text().ok_or(Error::ParseError),
        None => {
            // If teams are unknown, need to match for different classes.
            let m = h.find("matchInfo").find("matchInfoEmpty");
            match m.n {
                Some(_) => m.inner_text().ok_or(Error::ParseError),
                None => Err(Error::ConversionError("no event description found".to_string())),
            }
        }
    }
}

/// Returns the number of stars in an upcoming match. Returns errors if
/// the stars information is missing or the attribute cannot be parsed.
fn parse_stars(d: &tl::VDom, h: tl::NodeHandle) -> Result<u32, Error> {
    let tag = h.get(d.parser()).unwrap().as_tag().unwrap();
    match tag.get_attr("stars")? {
        Some(x) => Ok(x),
        None => Err(Error::ConversionError(
            "no stars attribute in div.upcomingMatch".to_string(),
        )),
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

    #[test]
    pub fn abc() {
        let href = "matches/123/nsaoenas".split('/').nth(1);
        println!("{:?}", href);
    }
}
