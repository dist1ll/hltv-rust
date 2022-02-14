/*!
Extensions to make the [`tl`] crate more ergonomic.

This module contains functionality that makes it easier to traverse
DOM elements and NodeHandles, as well as extract information (without
the overhead of the NodeHandle -> Node abstraction).
*/

use crate::Error;
use std::{borrow::Cow, str::FromStr};
use tl::*;

pub trait HTMLTagExtension {
    /// Finds an attribute in a given html tag. Returns `Some(x)` if attribute `x`
    /// could be found and parsed correctly. Returns a [`Error::ConversionError`]
    /// if the attribute string couldn't be parsed to `T`.
    fn get_attr<T>(&self, attr: &str) -> Result<Option<T>, Error>
    where
        T: FromStr;

    /// Finds an attribute in a given html tag. Returns a normal String.
    fn get_attr_str(&self, attr: &str) -> Option<String>;
}

impl<'a> HTMLTagExtension for HTMLTag<'a> {
    fn get_attr<T>(&self, attr: &str) -> Result<Option<T>, Error>
    where
        T: FromStr,
    {
        let s = self.get_attr_str(attr);
        if s.is_none() {
            return Ok(None);
        }
        match s.unwrap().parse::<T>() {
            Ok(t) => Ok(Some(t)),
            Err(_) => Err(Error::ParseError),
        }
    }

    fn get_attr_str(&self, attr: &str) -> Option<String> {
        let result = self.attributes().get(attr).flatten()?;
        Some(result.as_utf8_str().to_string())
    }
}

pub trait NodeHandleExtension {
    fn inner_text<'b, 'p: 'b>(&self, parser: &'p tl::Parser<'b>) -> Option<Cow<'b, str>>;
    fn to_rich<'a>(self, d: &'a VDom<'a>) -> RichNode<'a>;
}

impl NodeHandleExtension for NodeHandle {
    fn inner_text<'b, 'p: 'b>(&self, parser: &'p tl::Parser<'b>) -> Option<Cow<'b, str>> {
        let node = self.get(parser)?;
        Some(node.inner_text(parser))
    }
    fn to_rich<'a>(self, d: &'a VDom<'a>) -> RichNode<'a> {
        RichNode::<'a> { d, n: Some(self) }
    }
}

/// A node that also has a VDom reference. Can be used to chain
/// find calls
#[derive(Debug, Clone, Copy)]
pub struct RichNode<'a> {
    pub d: &'a VDom<'a>,
    pub n: Option<NodeHandle>,
}

impl<'a> RichNode<'a> {

    /// Returns the first child that fulfils the given predicate
    pub fn find_where(self, f: &dyn Fn(RichNode<'a>) -> bool) -> RichNode<'a>{
        if self.n.is_none() {
            return RichNode {d: self.d, n: None};
        }
        let n = dfs_first_where(self, f);
        RichNode{d: self.d, n}
    }

    fn compare_class(class: &'a str) -> Box<(dyn Fn(RichNode<'a>) -> bool + 'a)> {
        Box::new(move |n: RichNode| {
            let x = n.get().unwrap().as_tag();
            if x.is_none() {
                return false;
            }
            x.unwrap().attributes().is_class_member(class)
        })
    }

    /// Returns a child with given class.
    pub fn find(self, class: &'a str) -> RichNode<'a> {
        if self.n.is_none() {
            return RichNode { d: self.d, n: None };
        }
        let n = dfs_first_where(self, &Self::compare_class(class));
        RichNode { d: self.d, n }
    }
    /// Returns all children with given class
    pub fn find_all(self, class: &str) -> Vec<RichNode<'a>> {
        if self.n.is_none() {
            return Vec::<RichNode<'a>>::new();
        }
        self.d
            .select_nodes(self.n.unwrap(), class)
            .iter()
            .map(|n| n.to_rich(self.d))
            .collect()
    }

    /// Gets the index-th child, that is a tag.
    pub fn child(self, index: u32) -> Option<RichNode<'a>> {
        let mut i = 0;
        for x in self.get()?.children()?.top().iter() {
            if x.get(self.d.parser()).unwrap().as_tag().is_some() {
                if index == i {
                    return Some(x.to_rich(self.d));
                }
                i += 1;
            }
        }
        None
    }

    pub fn get_attr_str(&self, attr: &str) -> Option<String> {
        let tag = self.n.and_then(|n| n.get(self.d.parser()))?.as_tag()?;

        let result = tag.attributes().get(attr).flatten()?;
        Some(result.as_utf8_str().to_string())
    }

    pub fn get_attr<T>(&self, attr: &str) -> Result<Option<T>, Error>
    where
        T: FromStr,
    {
        let s = self.get_attr_str(attr);
        if s.is_none() {
            return Ok(None);
        }
        match s.unwrap().parse::<T>() {
            Ok(t) => Ok(Some(t)),
            Err(_) => Err(Error::ParseError),
        }
    }

    pub fn has_class(self, class: &'static str) -> Option<bool> {
        Some(self.n?
            .get(self.d.parser())?
            .as_tag()?
            .attributes()
            .is_class_member(class))
    }

    pub fn inner_parse<T>(self) -> Result<Option<T>, Error>
    where
        T: FromStr,
    {
        let t = self.inner_text();
        if t.is_none() {
            return Ok(None);
        }
        match t.unwrap().parse::<T>() {
            Ok(x) => Ok(Some(x)),
            Err(_) => Err(Error::ParseError),
        }
    }

    pub fn inner_text(self) -> Option<String> {
        self.n
            .and_then(|n| n.inner_text(self.d.parser()))
            .map(|c| c.to_string())
    }

    pub fn get(self) -> Option<&'a Node<'a>> {
        self.n.and_then(|n| n.get(self.d.parser()))
    }
}

pub trait VDomExtension<'a> {
    /// Return the first node in the subtree of `h` that has the given
    /// class string.
    fn select_first(&'a self, h: NodeHandle, class: &str) -> Option<NodeHandle>;
    /// Return all nodes in the subtree of `h` that have the given
    /// class string.
    fn select_nodes(&'a self, h: NodeHandle, class: &str) -> Vec<NodeHandle>;
}

impl<'a> VDomExtension<'a> for VDom<'a> {
    fn select_first(&'a self, h: NodeHandle, class: &str) -> Option<NodeHandle> {
        dfs_first(h, self, class)
    }
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
    for &c in children.unwrap().top().iter() {
        dfs(c, dom, class, result);
    }
}

fn dfs_first<'a>(h: NodeHandle, dom: &'a VDom<'a>, class: &str) -> Option<NodeHandle> {
    // break condition
    let tag = h.get(dom.parser()).unwrap().as_tag()?;
    if tag.attributes().is_class_member(class) {
        return Some(h);
    }
    // return None when no children
    let children = h.get(dom.parser()).unwrap().children()?;

    // otherwise, iterate over all children
    for &c in children.top().iter() {
        if let Some(x) = dfs_first(c, dom, class) {
            return Some(x);
        }
    }
    None
}

fn dfs_first_where<'a>(h: RichNode<'a>, f: &dyn Fn(RichNode<'a>)->bool) -> Option<NodeHandle> {
    // break condition
    if f(h) {
        return h.n;
    }
    // return None when no children
    let children = h.get().unwrap().children()?;

    // otherwise, iterate over all children
    for &c in children.top().iter() {
        let k = c.to_rich(h.d);
        if let Some(x) = dfs_first_where(k, f) {
            return Some(x);
        }
    }
    None
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
        assert_eq!(
            nodes[0].get(dom.parser()).unwrap().inner_text(dom.parser()),
            "dist1ll"
        );
        let node = dom.select_first(dom.children()[0], "abc").unwrap();
        assert_eq!(node.inner_text(dom.parser()).unwrap(), "dist1ll");
        let nodes = dom.select_nodes(dom.children()[2], "abc");
        assert_eq!(nodes.len(), 1);
    }

    #[test]
    pub fn rich() {
        let input = include_str!("./testdata/rich.html");
        let dom = tl::parse(input, tl::ParserOptions::default()).unwrap();
        let root = dom.children()[0].to_rich(&dom);
        let inner = root.find("b").find("x").find("y");
        assert!(inner.n.is_some());
        let inner = root.find("nsaedoi");
        assert!(inner.n.is_none());
        let inner = root.find("b").find("c").find_all("d");
        assert_eq!(inner.len(), 2);
        assert_eq!(inner[0].inner_text().unwrap(), "d1".to_string());
    }

    #[test]
    pub fn has_class() {
        let input = include_str!("./testdata/rich.html");
        let dom = tl::parse(input, tl::ParserOptions::default()).unwrap();
        let root = dom.get_element_by_id("has-class").unwrap().to_rich(&dom);
        let n = root.find("first");
        assert!(n.has_class("first").unwrap());
        assert!(n.has_class("second").unwrap());
        assert!(!n.has_class("third").unwrap());
    }

    #[test]
    pub fn child_index() {
        let input = include_str!("./testdata/tl/ext1.html");
        let dom = tl::parse(input, tl::ParserOptions::default()).unwrap();
        let root = dom.get_element_by_id("root-child").unwrap().to_rich(&dom);

        assert!(root.child(0).unwrap().get_attr_str("href").is_some());
        assert!(root.child(0).unwrap().get_attr_str("class").is_none());
        assert!(root.child(1).unwrap().get_attr_str("class").is_some());
    }
}
