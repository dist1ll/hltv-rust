use tl::queryselector::QuerySelectorIterator;

use crate::data::*;
use crate::tl_extensions::*;
use crate::ConvertInstance;
use crate::{Error, Error::ConversionError};

impl ConvertInstance for Vec<MatchResult> {
    fn convert<'a>(d: &'a tl::VDom<'a>) -> Result<Vec<MatchResult>, Error> {
        let mut result = Vec::<MatchResult>::new();
        let match_containers = get_roots(d);
        for c in match_containers {
            let h = c.to_rich(d);
            result.push(MatchResult {
                id: parse_id(h)?,
                winner: parse_which(h)?,
                team1: parse_team(h, "team1")?,
                team2: parse_team(h, "team2")?,
                score: parse_score(h)?,
                event: parse_event(h)?,
                format: parse_format(h)?,
            })
        }
        Ok(result)
    }
}

/// Returns the an iterator over roots of interest (i.e. the containers of
/// results).
fn get_roots<'a>(d: &'a tl::VDom<'a>) -> QuerySelectorIterator<tl::VDom> {
    d.query_selector("div.result-con").unwrap()
}

fn parse_format(h: RichNode) -> Result<MatchFormat, Error> {
    match h
        .find("map-text")
        .inner_text()
        .ok_or(ConversionError("match format can't be found"))?
        .as_str()
    {
        "bo3" => Ok(MatchFormat::Bo3),
        "bo5" => Ok(MatchFormat::Bo5),
        "bo7" => Ok(MatchFormat::Bo7),
        _ => Ok(MatchFormat::Bo1),
    }
}

fn parse_score(h: RichNode) -> Result<Score, Error> {
    Ok(Score {
        score_won: h
            .find("score-won")
            .inner_parse()?
            .ok_or(ConversionError("no score-won found"))?,
        score_lost: h
            .find("score-lost")
            .inner_parse()?
            .ok_or(ConversionError("no score-lost found"))?,
    })
}

fn parse_event(h: RichNode) -> Result<String, Error> {
    h.find("event-name")
        .inner_text()
        .ok_or(ConversionError("no event name found"))
}

fn parse_team(h: RichNode, team_id: &str) -> Result<String, Error> {
    h.find(team_id)
        .find("team")
        .inner_text()
        .ok_or(ConversionError("no team name found"))
}

fn parse_which(h: RichNode) -> Result<WhichTeam, Error> {
    let res = h
        .find("team1")
        .find("team")
        .has_class("team-won")
        .ok_or(ConversionError("team format incorrect"))?;
    match res {
        true => Ok(WhichTeam::First),
        false => Ok(WhichTeam::Second),
    }
}

fn parse_id(h: RichNode) -> Result<u32, Error> {
    let href = h
        .find("a-reset")
        .get_attr_str("href")
        .ok_or(Error::ParseError)?;
    match href.split('/').nth(2).ok_or(Error::ParseError)?.parse() {
        Ok(x) => Ok(x),
        Err(_) => Err(ConversionError("match ID isn't a valid number")),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Tests if the converter parses player info from a match page correctly.
    #[test]
    pub fn results() {
        let input = include_str!("../testdata/results.html");
        let dom = tl::parse(input, tl::ParserOptions::default()).unwrap();
        let result: Vec<MatchResult> = Vec::convert(&dom).unwrap();
        assert_eq!(result.len(), 2);

        assert_eq!(
            result[0],
            MatchResult {
                id: 123456,
                winner: WhichTeam::First,
                team1: "Stars Horizon".to_string(),
                team2: "Redragon".to_string(),
                score: Score {
                    score_won: 2,
                    score_lost: 0
                },
                event: "Liga Gamers Club 2022 Serie A January Cup".to_string(),
                format: MatchFormat::Bo3,
            }
        );

        assert_eq!(
            result[1],
            MatchResult {
                id: 2354179,
                winner: WhichTeam::First,
                team1: "Dignitas".to_string(),
                team2: "HAVU".to_string(),
                score: Score {
                    score_won: 16,
                    score_lost: 10
                },
                event: "Elisa Invitational Winter 2021 Main Qualifier".to_string(),
                format: MatchFormat::Bo1,
            }
        );
    }

}
