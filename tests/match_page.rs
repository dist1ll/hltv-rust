use chrono::{DateTime, NaiveDateTime, Utc};
use hltv::data::*;
use pretty_assertions::assert_eq;
use std::error::Error;
use std::time::Duration;

async fn wait() {
    tokio::time::sleep(Duration::from_millis(1000)).await;
}

/// Convenient constructor for Performance
fn perf(id: u32, s: (u32, u32, f32, f32, f32), name: &str) -> Performance {
    Performance(
        Player {
            id,
            nickname: name.to_string(),
        },
        Stats {
            kills: s.0,
            deaths: s.1,
            adr: s.2,
            kast: s.3,
            rating: s.4,
        },
    )
}

/// Ad-hoc testing method for LIVE matches.
#[tokio::test]
async fn ad_hoc() -> Result<(), Box<dyn Error>> {
    let res = hltv::get_match(2354349).fetch().await?;
    println!("{:?}", res);
    Ok(())
}

/// Ad-hoc testing method for LIVE matches.
#[tokio::test]
async fn alt_logo() -> Result<(), Box<dyn Error>> {
    let res = hltv::get_match(2353990).fetch().await?;
    let team2 = res.team2.unwrap();
    // only BIG has an alt version logo
    assert!(res.team1.unwrap().alt_logo.is_none());
    assert!(team2.alt_logo.is_some());
    println!("BIG's alternative logo link: {:?}", team2.alt_logo.unwrap());
    Ok(())
}

/// Testing if specific matches are parsed without throwing errors
#[tokio::test]
async fn concluded_bo3() -> Result<(), Box<dyn Error>> {
    wait().await;
    // Bo3 with one 6 man Team
    let res = hltv::get_match(2346065).fetch().await?;
    assert_eq!(
        res,
        MatchPage {
            id: 2346065,
            status: MatchStatus::Finished,
            team1: Some(Team::new(6665, "Astralis", "https://img-cdn.hltv.org/teamlogo/9bgXHp-oh1oaXr7F0mTGmd.svg?ixlib=java-2.1.0&amp;s=f567161ab183001be33948b98c4b2067", None)),
            team2: Some(Team::new(9565, "Vitality", "https://img-cdn.hltv.org/teamlogo/GAlByJtDTnkgbb9p_71SUL.png?ixlib=java-2.1.0&amp;w=100&amp;s=ddc5952ae492cbefb10fbe64471486b5", None)),
            event: Event {
                id: 5206,
                name: "BLAST Premier Global Final 2020".to_string()
            },
            date: DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(1611415800, 0), Utc),
            format: MatchFormat::Bo3,
            maps: vec![
                MapScore::new(Map::Dust2, 16, 14),
                MapScore::new(Map::Overpass, 10, 16),
                MapScore::new(Map::Inferno, 16, 5),
            ],
            score: Some(MatchScore { team1: 2, team2: 1 }),
            stats: vec![
                perf(7398, (67, 53, 87.3, 71.4, 1.25), "dupreeh"),
                perf(7592, (56, 51, 79.0, 68.8, 1.13), "device"),
                perf(4954, (52, 45, 76.1, 70.1, 1.09), "Xyp9x"),
                perf(9032, (54, 53, 73.9, 70.1, 1.07), "Magisk"),
                perf(7412, (44, 55, 65.1, 70.1, 0.9), "gla1ve"),
                perf(11893, (81, 49, 98.3, 72.7, 1.47), "ZywOo"),
                perf(1225,  (41, 38, 88.2, 76.8, 1.14), "shox"),
                perf(7169,  (43, 54, 65.5, 74.0, 0.92), "RpK"),
                perf(7322,  (43, 54, 69.3, 67.5, 0.88), "apEX"),
                perf(19512, (24, 42, 59.6, 60.8, 0.76), "Nivera"),
                perf(14176, (25, 37, 47.8, 70.2, 0.75), "misutaaa"),
            ],
        }
    );

    Ok(())
}


/// Testing if specific matches are parsed without throwing errors
#[tokio::test]
async fn unknown_upcoming() -> Result<(), Box<dyn Error>> {
    wait().await;
    let upc = hltv::upcoming().build().fetch().await?;
    let res = upc.last().unwrap();
    wait().await;
    let m = hltv::get_match(res.id).fetch().await?;
    assert_eq!(m.maps, Vec::new());
    assert_eq!(m.stats, Vec::new());
    assert_eq!(m.score, None);
    assert_eq!(m.status, MatchStatus::Upcoming);
    Ok(())
}
