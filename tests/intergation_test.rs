use hltv;

/// Testing if upcoming matches are correctly parsed.
#[test]
fn upcoming_matches() {
    let req = hltv::upcoming().events(vec![125, 5454]).build();
}
