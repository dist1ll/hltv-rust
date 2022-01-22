/*!
Methods to convert HTML to hltv data types.

This module contains all implemenations for converting HTML documents (type [`tl::VDom`])
to the types contained in the `data` module. Correct conversion is dependent on the HTML
layout of the HLTV webpage, and therefore its stability depends on HLTV not changing their
site.

If the conversion breaks at any point in the future, feel free to create an issue on the
GitHub repository or submit a pull request.
*/

use crate::data::*;
use crate::Error;
use crate::Convert;

impl<'a> TryFrom<tl::VDom<'a>> for Match {
    type Error = crate::Error;
    fn try_from(_dom: tl::VDom<'a>) -> Result<Self, Self::Error> {
        Err(Error::ConversionError)
    }
}

impl<'a> Convert<'a> for Player {
    fn convert(_d: tl::VDom<'a>) -> Result<Vec<Player>, crate::Error> {
        Err(Error::ConversionError)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    pub fn player() {
        let input = include_str!("./testdata/player.html");
        let dom = tl::parse(input, tl::ParserOptions::default()).unwrap();
        let mut qs = dom.query_selector("td.player").unwrap();
        qs.next().unwrap().get(dom.parser());
        for x in qs {
            let node = x.get(dom.parser()).unwrap();
            assert_eq!("{:?}", node.inner_text(dom.parser()));
        }
    }
}

