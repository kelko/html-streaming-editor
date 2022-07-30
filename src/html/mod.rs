use std::collections::{HashMap, HashSet};
use std::ops::Index;
use tl::{NodeHandle, Parser, VDom};

#[cfg(test)]
mod tests;

/// The related nodes for a given node
pub(crate) struct HtmlNodeIndex {
    pub(crate) children: HashSet<NodeHandle>,
    pub(crate) descendents: HashSet<NodeHandle>,
    pub(crate) siblings: HashSet<NodeHandle>,
    pub(crate) direct_sibling: Option<NodeHandle>,
}

/// An index of the whole node relationships for easier CSS resolution
///
/// Iterates through the whole DOM and stores the related nodes for each node
/// to quickly find children, descendents and siblings
pub(crate) struct HtmlIndex<'a> {
    inner: HashMap<NodeHandle, HtmlNodeIndex>,
    pub(crate) dom: &'a VDom<'a>,
}

impl<'a> HtmlIndex<'a> {
    /// build a new Index for a given DOM
    pub fn load(dom: &'a VDom<'a>) -> Self {
        let mut index = HashMap::new();
        Self::fill(&mut index, &dom);

        HtmlIndex { inner: index, dom }
    }

    /// start on the top level elements of the DOM
    fn fill(
        index: &mut HashMap<NodeHandle, HtmlNodeIndex>,
        dom: &'a VDom<'a>,
    ) -> HashSet<NodeHandle> {
        let parser = dom.parser();
        let mut descendents = HashSet::new();
        let mut children: HashSet<NodeHandle> = HashSet::new();

        let mut direct_sibling: Option<NodeHandle> = None;

        for child in dom.children().to_vec().iter().rev() {
            let node_descendents =
                Self::fill_recursive(index, child, parser, &children, direct_sibling);
            descendents.extend(node_descendents);
            children.insert(child.clone());
            direct_sibling = Some(child.clone())
        }
        descendents.extend(&children);

        return descendents;
    }

    /// recursively find all children and descendents for a node,
    /// the list of siblings is passed in as parameter from the calling method.
    ///
    /// Notes:
    ///
    /// - The list of children is iterated in reverse to build up the siblings list for each elements
    ///   as only following elements count for ~ and + combinators.
    /// - Each child returns the set of all its descendents, those are then merged together,
    ///   including the list of children, to make up the set of the parent node
    ///
    fn fill_recursive(
        index: &mut HashMap<NodeHandle, HtmlNodeIndex>,
        node_handle: &NodeHandle,
        parser: &'a Parser<'a>,
        sibling: &HashSet<NodeHandle>,
        direct_sibling: Option<NodeHandle>,
    ) -> HashSet<NodeHandle> {
        let mut descendents = HashSet::new();
        let mut childs_direct_sibling: Option<NodeHandle>;

        if let Some(node) = node_handle.get(parser) {
            let mut children: HashSet<NodeHandle> = HashSet::new();
            childs_direct_sibling = None;

            if let Some(tl_children) = node.children() {
                for child in tl_children.top().to_vec().iter().rev() {
                    let node_descendents = Self::fill_recursive(
                        index,
                        &child,
                        parser,
                        &children,
                        childs_direct_sibling,
                    );
                    descendents.extend(node_descendents);
                    children.insert(child.clone());
                    childs_direct_sibling = Some(child.clone());
                }

                descendents.extend(&children);
            } else {
                children = HashSet::new();
            }

            index.insert(
                node_handle.clone(),
                HtmlNodeIndex {
                    children,
                    descendents: descendents.clone(),
                    siblings: sibling.clone(),
                    direct_sibling,
                },
            );
        }

        return descendents;
    }

    /// get the relations for a given node
    pub(crate) fn get(&self, node: &NodeHandle) -> Option<&HtmlNodeIndex> {
        self.inner.get(node)
    }
}

impl<'a> Index<NodeHandle> for HtmlIndex<'a> {
    type Output = HtmlNodeIndex;

    fn index(&self, index: NodeHandle) -> &Self::Output {
        self.inner.get(&index).unwrap()
    }
}
