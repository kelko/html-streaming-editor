use snafu::{Backtrace, OptionExt, Snafu};
use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::ops::Index;

use tl::{NodeHandle, Parser, VDom};

#[cfg(test)]
mod tests;

#[derive(Debug, Snafu)]
pub enum IndexError {
    #[snafu(display("Index seems out of date. NodeHandle couldn't be found in Parser"))]
    OutdatedIndex { backtrace: Backtrace },
    #[snafu(display("HTML seems broken: {operation} failed"))]
    InvalidHtml {
        operation: String,
        backtrace: Backtrace,
    },
}

/// The related nodes for a given node
#[derive(Debug)]
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
#[derive(Debug)]
pub(crate) struct HtmlIndex<'a> {
    pub(self) inner: HashMap<NodeHandle, HtmlNodeIndex>,
    pub(crate) dom: RefCell<VDom<'a>>,
}

impl<'a> HtmlIndex<'a> {
    /// build a new Index for a given DOM
    pub fn load(dom: VDom<'a>) -> Self {
        let mut index = HashMap::new();
        Self::fill(&mut index, &dom);

        HtmlIndex {
            inner: index,
            dom: RefCell::new(dom),
        }
    }

    /// start on the top level elements of the DOM
    fn fill(
        index: &mut HashMap<NodeHandle, HtmlNodeIndex>,
        dom: &'_ VDom<'a>,
    ) -> HashSet<NodeHandle> {
        let parser = dom.parser();
        let mut descendents = HashSet::new();
        let mut children: HashSet<NodeHandle> = HashSet::new();

        let mut direct_sibling: Option<NodeHandle> = None;

        for child in dom.children().to_vec().iter().rev() {
            if let Some(node_descendents) =
                Self::fill_recursive(index, child, parser, &children, direct_sibling)
            {
                descendents.extend(node_descendents);
                children.insert(child.clone());
                direct_sibling = Some(child.clone())
            }
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
        parser: &'_ Parser<'a>,
        sibling: &HashSet<NodeHandle>,
        direct_sibling: Option<NodeHandle>,
    ) -> Option<HashSet<NodeHandle>> {
        let mut descendents = HashSet::new();
        let mut childs_direct_sibling: Option<NodeHandle>;

        if let Some(node) = node_handle.get(parser) {
            if node.as_tag().is_none() {
                return None;
            }

            let mut children: HashSet<NodeHandle> = HashSet::new();
            childs_direct_sibling = None;

            if let Some(tl_children) = node.children() {
                for child in tl_children.top().to_vec().iter().rev() {
                    if let Some(node_descendents) = Self::fill_recursive(
                        index,
                        &child,
                        parser,
                        &children,
                        childs_direct_sibling,
                    ) {
                        descendents.extend(node_descendents);
                        children.insert(child.clone());
                        childs_direct_sibling = Some(child.clone());
                    }
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

        return Some(descendents);
    }

    /// get the relations for a given node
    pub(crate) fn relations_of(&self, node: &NodeHandle) -> Option<&HtmlNodeIndex> {
        self.inner.get(node)
    }

    pub(crate) fn find_parent(&self, child: &NodeHandle) -> Option<&NodeHandle> {
        self.inner
            .iter()
            .find(|(_, relations)| relations.children.contains(child))
            .map(|(parent, _)| parent)
    }

    /// convenience method to fetch the "root" elements, e.g. the elements on the very top
    /// of the DOM tree
    pub(crate) fn root_elements(&self) -> HashSet<NodeHandle> {
        HashSet::from_iter(self.dom.borrow().children().iter().cloned())
    }

    pub(crate) fn render(&self, handle: &NodeHandle) -> Result<String, IndexError> {
        let dom = self.dom.borrow();
        let parser = dom.parser();
        Ok(handle
            .get(parser)
            .context(OutdatedIndexSnafu)?
            .outer_html(parser)
            .into_owned())
    }

    pub(crate) fn remove(&self, handle: &NodeHandle) -> Result<(), IndexError> {
        if let Some(parent) = self.find_parent(handle) {
            let mut dom = self.dom.borrow_mut();
            let mut_parser = dom.parser_mut();
            let parent = parent.get_mut(mut_parser).context(OutdatedIndexSnafu)?;
            let parent = parent.as_tag_mut().context(InvalidHtmlSnafu {
                operation: String::from("treating parent as tag"),
            })?;

            let mut children = parent.children_mut();
            let children = children.top_mut();
            let index = children
                .iter()
                .position(|t| t == handle)
                .expect("HtmlIndex cache must be broken. Please file bug report");
            children.remove(index);

            //TODO: remove from index?
        }

        Ok(())
    }
}

impl<'a> Index<NodeHandle> for HtmlIndex<'a> {
    type Output = HtmlNodeIndex;

    fn index(&self, index: NodeHandle) -> &Self::Output {
        self.inner
            .get(&index)
            .expect("Asking for a NodeHandle that does not belong to the index")
    }
}
