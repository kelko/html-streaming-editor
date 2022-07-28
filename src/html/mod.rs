use std::collections::{HashMap, HashSet};
use std::ops::Index;
use tl::{NodeHandle, Parser, VDom};

#[cfg(test)]
mod tests;

pub(crate) struct HtmlNodeIndex {
    pub(crate) children: HashSet<NodeHandle>,
    pub(crate) descendents: HashSet<NodeHandle>,
    pub(crate) siblings: HashSet<NodeHandle>,
}

pub(crate) struct HtmlIndex<'a> {
    inner: HashMap<NodeHandle, HtmlNodeIndex>,
    pub(crate) dom: &'a VDom<'a>,
    pub(crate) all: HashSet<NodeHandle>,
}

impl<'a> HtmlIndex<'a> {
    pub fn load(dom: &'a VDom<'a>) -> Self {
        let mut index = HashMap::new();
        let all = Self::fill(&mut index, &dom);

        HtmlIndex {
            inner: index,
            dom,
            all,
        }
    }

    fn fill(
        index: &mut HashMap<NodeHandle, HtmlNodeIndex>,
        dom: &'a VDom<'a>,
    ) -> HashSet<NodeHandle> {
        let parser = dom.parser();
        let mut descendents = HashSet::new();
        let mut children: HashSet<NodeHandle> = HashSet::new();

        for child in dom.children().to_vec().iter().rev() {
            let node_descendents = Self::fill_recursive(index, child, parser, &children);
            descendents.extend(node_descendents);
            children.insert(child.clone());
        }
        descendents.extend(&children);

        return descendents;
    }

    fn fill_recursive(
        index: &mut HashMap<NodeHandle, HtmlNodeIndex>,
        node_handle: &NodeHandle,
        parser: &'a Parser<'a>,
        sibling: &HashSet<NodeHandle>,
    ) -> HashSet<NodeHandle> {
        let mut descendents = HashSet::new();

        if let Some(node) = node_handle.get(parser) {
            let mut children: HashSet<NodeHandle> = HashSet::new();
            if let Some(tl_children) = node.children() {
                for child in tl_children.top().to_vec().iter().rev() {
                    let node_descendents = Self::fill_recursive(index, &child, parser, &children);
                    descendents.extend(node_descendents);
                    children.insert(child.clone());
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
                },
            );
        }

        return descendents;
    }

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
