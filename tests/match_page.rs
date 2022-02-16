use chrono::{DateTime, NaiveDateTime, Utc};
use hltv::data::*;
use pretty_assertions::assert_eq;
use std::error::Error;
use std::time::Duration;

async fn wait() {
    tokio::time::sleep(Duration::from_millis(1500)).await;
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
            team1: Some(Team {
                id: 6665,
                name: "Astralis".to_string()
            }),
            team2: Some(Team {
                id: 9565,
                name: "Vitality".to_string()
            }),
            event: Event {
                id: 5206,
                name: "BLAST Premier Global Final 2020".to_string()
            },
            date: DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(1611415800, 0), Utc),
            format: MatchFormat::Bo3,
            maps: vec![
                MapScore {
                    map: Map::Dust2,
                    team1: 16,
                    team2: 14,
                },
                MapScore {
                    map: Map::Overpass,
                    team1: 10,
                    team2: 16,
                },
                MapScore {
                    map: Map::Inferno,
                    team1: 16,
                    team2: 5,
                }
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
