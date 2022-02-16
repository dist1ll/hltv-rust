use chrono::{DateTime, NaiveDateTime, Utc};
use hltv::data::*;
use pretty_assertions::assert_eq;
use std::error::Error;
use std::time::Duration;

async fn wait() {
    tokio::time::sleep(Duration::from_millis(1500)).await;
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
            stats: vec![],
        }
    );

    Ok(())
}
