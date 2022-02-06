/*!
Methods to convert HTML to HLTV data types.

This module contains all implemenations for converting HTML documents (type [`tl::VDom`])
to the types contained in the `data` module. Correct conversion is dependent on the HTML
layout of the HLTV webpage, and therefore its stability depends on HLTV not changing their
site.

If the conversion breaks at any point in the future, feel free to create an issue on the
GitHub repository or submit a pull request.
*/

pub mod player;
pub mod matches;
pub mod results;
