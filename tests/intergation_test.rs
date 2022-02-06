use std::error::Error;

use hltv::EventTypeFilter;

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
