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
fn parse_team(d: &tl::VDom, h: tl::NodeHandle, team_id: &str) -> Option<Team> {
    let tag = h.get(d.parser()).unwrap().as_tag().unwrap();
    let id: u32 = tag.get_attr(team_id).unwrap_or(None)?;
    let team_node = d.select_first(h, team_id)?;
    let name_node = d.select_first(team_node, "matchTeamName")?;
    name_node.inner_text(d.parser()).map(|n| Team {
        id,
        name: n.to_string(),
    })
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
}
