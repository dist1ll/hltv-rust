/*!
Extensions to make the [`tl`] crate more ergonomic. 

This module contains functionality that makes it easier to traverse 
DOM elements and NodeHandles, as well as extract information (without
the overhead of the NodeHandle -> Node abstraction).
*/

use std::borrow::Cow;
use tl::*;

pub trait HTMLTagExtension {
    fn get_attr(&self, attr: &str) -> Option<String>;
}

impl<'a> HTMLTagExtension for HTMLTag<'a> {
    fn get_attr(&self, attr: &str) -> Option<String> {
        let result = self.attributes().get(attr).flatten()?;
        Some(result.as_utf8_str().to_string())
    }
}

pub trait NodeHandleExtension {
    fn inner_text<'b, 'p : 'b>(&'b self, parser: &'p tl::Parser<'b>) -> Option<Cow<'b, str>>;
}

impl<'a> NodeHandleExtension for NodeHandle {
    fn inner_text<'b, 'p : 'b>(&'b self, parser: &'p tl::Parser<'b>) -> Option<Cow<'b, str>> {
        let node = self.get(parser)?;
        Some(node.inner_text(parser))
    }
}

pub trait VDomExtension<'a> {
    fn select_nodes(&'a self, h: NodeHandle, class: &str) -> Vec<NodeHandle>;
}

impl<'a> VDomExtension<'a> for VDom<'a> {
    /// Return all nodes in the subtree of `h` that have the given
    /// class string.
    fn select_nodes(&'a self, h: NodeHandle, class: &str) -> Vec<NodeHandle> {
        let mut result = Vec::<NodeHandle>::new();
        dfs(h, self, class, &mut result);
        result
    }
}

/// Populate a vector with children of the given NodeHandle that
/// match the class string. Traverses the subtree via depth-first search.
fn dfs<'a>(h: NodeHandle, dom: &'a VDom<'a>, class: &str, result: &mut Vec<NodeHandle>) {
    // break condition
    let tag = h.get(dom.parser()).unwrap().as_tag();
    if tag.is_none() {
        return;
    }

    if tag.unwrap().attributes().is_class_member(class) {
        result.push(h);
    }

    // return None when no children
    let children = h.get(dom.parser()).unwrap().children();
    if children.is_none() {
        return;
    }

    // otherwise, iterate over all children
    for c in children.unwrap() {
        dfs(c, dom, class, result);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn dfs_test() {
        let input = include_str!("./testdata/dfs.html");
        let dom = tl::parse(input, tl::ParserOptions::default()).unwrap();
        let nodes = dom.select_nodes(dom.children()[0], "abc");
        assert_eq!(nodes.len(), 2);
        assert_eq!(nodes[0].get(dom.parser()).unwrap().inner_text(dom.parser()), "dist1ll");
        
        let nodes = dom.select_nodes(dom.children()[1], "abc");
        assert_eq!(nodes.len(), 1);
    }
}
