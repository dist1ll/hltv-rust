use std::error::Error;

use hltv::EventTypeFilter;
use hltv::data::Map;

/// Testing if upcoming matches are correctly parsed.
#[tokio::test]
async fn upcoming_matches() -> Result<(), Box<dyn Error>> {
    let req = hltv::upcoming()
        .events(vec![6343, 6335])
        .event_type(EventTypeFilter::Online)
        .build();
    println!("{:?}", req);
    let matches = req.fetch().await?;
    println!("{:?}", matches);
    Ok(())
}

/// Testing if upcoming matches are correctly parsed.
#[tokio::test]
async fn results() -> Result<(), Box<dyn Error>> {
    let req = hltv::results()
        .map(Map::Inferno)
        .team(4608)
        .year(2016)
        .event_type(EventTypeFilter::Lan)
        .build();
    println!("{:?}", req);
    let matches = req.fetch().await?;
    println!("{:?}", matches);
    Ok(())
}
