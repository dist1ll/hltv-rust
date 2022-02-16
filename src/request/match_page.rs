use std::marker::PhantomData;

use crate::data::MatchPage;
use crate::Request;

/// Builds a [`Request`] object to fetch the page of a given match. The ID of a match
/// is the number you can find in the HLTV URL after `hltv.com/matches/{id}/...`
pub fn get_match(id: u32) -> Request<MatchPage> {
    Request{
        // Interesting: currently you can put any string after the last slash. It doesn't
        // need to contain the team or event names. 
        url: format!("https://www.hltv.org/matches/{}/xyz", id),
        _m: PhantomData
    }
}
