use chrono::DateTime;
use chrono::NaiveDateTime;
use chrono::Utc;

use crate::data::*;
use crate::tl_extensions::*;
use crate::ConvertInstance;
use crate::{Error, Error::ConversionError};

impl ConvertInstance for Vec<UpcomingMatch> {
    fn convert<'a>(d: &'a tl::VDom<'a>) -> Result<Vec<UpcomingMatch>, Error> {
        let mut result = Vec::<UpcomingMatch>::new();
        let match_containers = d.query_selector("div.upcomingMatch").unwrap();
        for c in match_containers {
            let h = c.to_rich(d);
            result.push(UpcomingMatch {
                id: parse_id(h)?,
                stars: parse_stars(d, c)?,
                team1: parse_team(h, "team1"),
                team2: parse_team(h, "team2"),
                event: parse_event(h)?,
                format: MatchFormat::Bo1,
                date: parse_date(h)?,
            })
        }
        Ok(result)
    }
}

/// Returns a Team contained in the NodeHandle. Use tag `"team1"` or `"team2"`
/// to search for either.
fn parse_team(h: RichNode, team_id: &str) -> Option<Team> {
    Some(Team {
        id: h.get_attr(team_id).unwrap_or(None)?,
        name: h.find(team_id).find("matchTeamName").inner_text()?,
        logo: "".to_string(),
        alt_logo: None,
    })
}

/// Parses the match date time.
fn parse_date(h: RichNode) -> Result<DateTime<Utc>, Error> {
    let time: i64 = h
        .get_attr::<i64>("data-zonedgrouping-entry-unix")?
        .ok_or(ConversionError("time is not set in div"))?;
    Ok(DateTime::<Utc>::from_utc(
        NaiveDateTime::from_timestamp(time / 1000, 0),
        Utc,
    ))
}

/// Parses the match ID from the given root node
fn parse_id(h: RichNode) -> Result<u32, Error> {
    let href = h
        .find("match")
        .get_attr_str("href")
        .ok_or(Error::ParseError)?;
    match href.split('/').nth(2).ok_or(Error::ParseError)?.parse() {
        Ok(x) => Ok(x),
        Err(_) => Err(ConversionError("match ID isn't a valid number")),
    }
}

/// Parses the event name
fn parse_event(h: RichNode) -> Result<String, Error> {
    let m = h.find("match").find("matchEvent").find("matchEventName");
    match m.n {
        Some(_) => m.inner_text().ok_or(Error::ParseError),
        None => {
            // If teams are unknown, need to match for different classes.
            let m = h.find("match").find("matchInfoEmpty").find("line-clamp-3");
            match m.n {
                Some(_) => m.inner_text().ok_or(Error::ParseError),
                None => Err(ConversionError("no event description found")),
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
        None => Err(ConversionError("no stars attribute in div.upcomingMatch")),
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
        let result: Vec<UpcomingMatch> = Vec::convert(&dom).unwrap();
        assert_eq!(result.len(), 2);
        // test existance of teams
        assert!(result[1].team1.is_none() && result[1].team2.is_none());
        // test team data
        assert_eq!(
            *result[0].team1.as_ref().unwrap(),
            Team::new(6667, "FaZe", "", None),
        );
        assert_eq!(
            *result[0].team2.as_ref().unwrap(),
            Team::new(5973, "Liquid", "", None),
        );
        // test match ID
        assert_eq!(result[0].id, 2353980);
        assert_eq!(result[1].id, 2353979);
    }
}
