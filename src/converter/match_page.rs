/*!
This module defines how a match page is parsed. Note that unlike "upcoming" or "result" type matches,
a match page can have many different conditions and edge cases. The crate gives no guarantees about
completeness. If you think an edge case is not correctly parsed, feel free to create an issue on github
and attach a sample of html that is not correctly recognized.
*/
use chrono::DateTime;
use chrono::NaiveDateTime;
use chrono::Utc;
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

/// Returns event information, like event ID or name. Requires match-page root.
fn get_event(h: RichNode) -> Result<Event, Error> {
    let event = h
        .find("timeAndEvent")
        .find("event")
        .child(0)
        .ok_or(ConversionError("no event data found"))?;

    let id: u32 = event
        .get_attr_str("href")
        .ok_or(ConversionError("event element has no href link"))?
        .split('/')
        .nth(2)
        .ok_or(ConversionError("event link has incorrect format"))?
        .parse()
        .map_err(|_| ConversionError("cant parse event ID"))?;

    Ok(Event {
        id,
        name: event
            .get_attr_str("title")
            .ok_or(ConversionError("no title attribute in event"))?,
    })
}

/// Return match starting date. Shouldn't change over time.
pub fn get_date(h: RichNode) -> Result<DateTime<Utc>, Error> {
    let timestamp: i64 = h
        .find("timeAndEvent")
        .find("time")
        .get_attr("data-unix")?
        .ok_or(ConversionError("no data-unix attribute"))?;

    Ok(DateTime::<Utc>::from_utc(
        NaiveDateTime::from_timestamp(timestamp / 1000, 0),
        Utc,
    ))
}

pub fn get_score(h: RichNode) -> Option<MatchScore> {
    let team1: u32 = h.find("team1-gradient").child(1)?.inner_parse().ok()??;
    let team2: u32 = h.find("team2-gradient").child(1)?.inner_parse().ok()??;
    // if one of the scores is high enough, it has to be a bo1
    if team1 > 8 && team1 > team2 {
        return Some(MatchScore{team1: 1, team2: 0});
    }
    if team2 > 8 && team2 > team1 {
        return Some(MatchScore{team1: 0, team2: 1});
    }
    Some(MatchScore { team1, team2 })
}

pub fn get_mapscores(h: RichNode) -> Result<Vec<MapScore>, Error> {
    let mut result = Vec::<MapScore>::new();
    for m in h.find("maps").find_all("mapholder") {
        let map = m.find("mapname").inner_text();
        let team1 = m.find("results-left").find("results-team-score").inner_text();
        let team2 = m.find("results-right").find("results-team-score").inner_text();
        if team1.is_none() || team2.is_none() || map.is_none() {
            continue;
        }
        let map = map.unwrap();
        if map.eq("TBA") || map.eq("-") {
            continue;
        }
        result.push(MapScore{
            map: map.into(),
            team1: team1.unwrap().parse().map_err(|_| ConversionError("can't convert 1st team's map score"))?,
            team2: team2.unwrap().parse().map_err(|_| ConversionError("cant convert 2nd team's map score"))?,
        })
    }
    Ok(result)
}

pub fn get_matchformat(h: RichNode) -> Result<MatchFormat, Error> {
    match h.find_all("mapholder").len() {
        1 => Ok(MatchFormat::Bo1),
        3 => Ok(MatchFormat::Bo3),
        5 => Ok(MatchFormat::Bo5),
        7 => Ok(MatchFormat::Bo7),
        _ => Err(ConversionError("can't determine match format. weird number of maps."))
    }
}

pub fn get_matchstatus(h: RichNode) -> Result<MatchStatus, Error> {
    let t = h.find("countdown").inner_text().ok_or(ConversionError("can't find countdown or match status"))?;
    match t.as_ref() {
        "Match over" => Ok(MatchStatus::Finished),
        "LIVE" => Ok(MatchStatus::Live),
        _ => Ok(MatchStatus::Upcoming),
    }
}

pub fn get_performance(h: RichNode) -> Option<[Performance;10]> {
    let all = h.find("stats-content").find_all("totalstats");
    if all.len() != 2 {
        return None;
    }
    let mut result: [Performance; 10] = Default::default();
    result[5..10].clone_from_slice(&get_performance_root(all[0])?);
    result[..5].clone_from_slice(&get_performance_root(all[1])?);
    Some(result)
}

/// Parse the match performance belonging to a specific team container totalstats table
fn get_performance_root(h: RichNode) -> Option<[Performance;5]> {
    let mut result: [Performance; 5] = Default::default();
    for i in 0u32..5 {
        let p = h.child(i + 1)?;
        println!("index: \t{:?}", i);
        result[i as usize] = get_performance_player(p)?;
        println!("{:?}", result[i as usize]);
    }
    Some(result)
}

/// Get the performance of a specific player in a tr-class
fn get_performance_player(h: RichNode) -> Option<Performance> {
    // Player
    let link = h.find("players").find("flagAlign").child(0)?.get_attr_str("href")?;
    let p = Player {
        id: link.split('/').nth(2)?.parse().ok()?,
        nickname: h.find("statsPlayerName").inner_text()?,
    };

    // Stats
    let kd = h.find("kd").inner_text()?;
    let kast = h.find("adr").inner_text()?;
    let s = Stats{
        kills: kd.split('-').next()?.parse().ok()?,
        deaths: kd.split('-').nth(1)?.parse().ok()?,
        adr: h.find("adr").inner_text()?.parse().ok()?,
        kast: kast.split('%').next()?.parse().ok()?,
        rating: h.find("rating").inner_text()?.parse().ok()?,
    };
    Some(Performance(p, s))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn xyz() {
        let input = include_str!("../testdata/matchPages/finished_bo3.html");
        let dom = tl::parse(input, tl::ParserOptions::default()).unwrap();
        let root = get_root(&dom).unwrap().to_rich(&dom);
        println!("{:?}", get_performance(root));
    }
    /// Tests if a finished bo3 match is correctly parsed.
    #[test]
    pub fn concluded_bo3() {
        let input = include_str!("../testdata/matchPages/finished_bo3.html");
        let dom = tl::parse(input, tl::ParserOptions::default()).unwrap();
        // let result = MatchPage::convert(&dom).unwrap();
    }
}
