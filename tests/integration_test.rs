use std::error::Error;
use std::time::Duration;
use hltv::request::EventTypeFilter;
use hltv::data::Map;

async fn wait() {
    tokio::time::sleep(Duration::from_millis(1500)).await;
}

/// Testing if specific matches are parsed without throwing errors
#[tokio::test]
async fn get_match() -> Result<(), Box<dyn Error>> {
    wait().await;
    let req = hltv::get_match(2346065);
    let res = req.fetch().await?;
    println!("{:?}", res);
    Ok(())
}

/// Testing if upcoming matches are correctly parsed.
#[tokio::test]
async fn upcoming_matches() -> Result<(), Box<dyn Error>> {
    wait().await;
    let req = hltv::upcoming()
        .top_tier()
        .build();
    req.fetch().await?;
    Ok(())
}

/// Testing if upcoming matches are correctly parsed.
#[tokio::test]
async fn results() -> Result<(), Box<dyn Error>> {
    wait().await;
    let req = hltv::results()
        .from(2016, 2, 20)
        .to(2017, 5, 20)
        .map(Map::Inferno)
        .team(4608)
        .player(7998)
        .event_type(EventTypeFilter::Lan)
        .build();
    req.fetch().await?;
    Ok(())
}
