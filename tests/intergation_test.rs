use hltv::EventTypeFilter;

/// Testing if upcoming matches are correctly parsed.
#[test]
fn upcoming_matches() {
    let req = hltv::upcoming()
        .events(vec![6343, 6335])
        .event_type(EventTypeFilter::Online)
        .build();
    println!("{:?}", req);
}
