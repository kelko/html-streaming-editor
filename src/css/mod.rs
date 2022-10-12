use std::fmt::Debug;
use std::ops::Index;

use crate::html::{HtmlContent, HtmlQueryable};
use log::trace;

#[cfg(test)]
mod tests;

/// CSS [pseudo classes](https://developer.mozilla.org/en-US/docs/Web/CSS/Pseudo-classes) selector
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CssPseudoClass {
    /// CSS [:first-child](https://developer.mozilla.org/en-US/docs/Web/CSS/:first-child) selector
    FirstChild,
    /// CSS [:nth-child()](https://developer.mozilla.org/en-US/docs/Web/CSS/:nth-child) selector
    NthChild(usize),
    /// CSS [:first-of-type](https://developer.mozilla.org/en-US/docs/Web/CSS/:first-of-type) selector
    FirstOfType,
    /// CSS [:nth-of-type()](https://developer.mozilla.org/en-US/docs/Web/CSS/:nth-of-type) selector
    NthOfType(usize),
    /// CSS [:last-child](https://developer.mozilla.org/en-US/docs/Web/CSS/:last-child) selector
    LastChild,
    /// CSS [:nth-last-child()](https://developer.mozilla.org/en-US/docs/Web/CSS/:nth-last-child) selector
    NthLastChild(usize),
    /// CSS [:last-of-type](https://developer.mozilla.org/en-US/docs/Web/CSS/:last-of-type) selector
    LastOfType,
    /// CSS [:nth-last-of-type()](https://developer.mozilla.org/en-US/docs/Web/CSS/:nth-last-of-type) selector
    NthLastOfType(usize),
    //more candidates:
    //Not(??)
    //Is(==)
    //Where(==)
}

/// Operator for CSS [attribute](https://developer.mozilla.org/en-US/docs/Web/CSS/Attribute_selectors) selector
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CssAttributeComparison {
    /// CSS \[attr\] attribute selector
    Exist,
    /// CSS \[attr\^="value"] attribute selector
    Starts,
    /// CSS \[attr\$="value"] attribute selector
    Ends,
    /// CSS \[attr\*="value"] attribute selector
    CharacterContains,
    /// CSS \[attr~="value"] attribute selector
    TermContains,
    /// CSS \[attr="value"] attribute selector
    EqualsExact,
    /// CSS \[attr|="value"] attribute selector
    EqualsTillHyphen,
}

#[derive(Debug, Clone, PartialEq, Eq)]
/// CSS [attribute](https://developer.mozilla.org/en-US/docs/Web/CSS/Attribute_selectors) selector
pub struct CssAttributeSelector<'a> {
    /// the attribute name to match against
    pub(crate) attribute: &'a str,
    /// operator to use for matching
    pub(crate) operator: CssAttributeComparison,
    /// value the attribute has to match
    pub(crate) value: Option<&'a str>,
}

impl<'a> CssAttributeSelector<'a> {
    pub(crate) fn matches(&self, attribute: impl Into<String>) -> bool {
        let given_value = attribute.into();

        if self.operator == CssAttributeComparison::Exist {
            // has to short-circuit as Exist does not have a self.value
            return true;
        }

        let expected_value = self.value.expect(
            "If operator is not Exist a value must be given or the parser works incorrectly",
        );

        match self.operator {
            CssAttributeComparison::Exist => unreachable!(),
            CssAttributeComparison::Starts => given_value.starts_with(expected_value),
            CssAttributeComparison::Ends => given_value.ends_with(expected_value),
            CssAttributeComparison::CharacterContains => given_value.contains(expected_value),
            CssAttributeComparison::TermContains => {
                given_value.split_whitespace().any(|x| x == expected_value)
            }
            CssAttributeComparison::EqualsExact => given_value.eq(expected_value),
            CssAttributeComparison::EqualsTillHyphen => {
                Self::equals_till_hyphen(expected_value, given_value)
            }
        }
    }

    fn equals_till_hyphen(expected_value: &str, given_value: String) -> bool {
        match given_value.find('-') {
            None => given_value.eq(expected_value),
            Some(position) => {
                let slice = &given_value[0..position];
                slice.eq(expected_value)
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
/// model for [CSS selectors](https://developer.mozilla.org/en-US/docs/Web/CSS/CSS_Selectors)
pub struct CssSelector<'a> {
    /// selector on element name
    pub(crate) element: Option<&'a str>,
    /// selector on element id
    pub(crate) id: Option<&'a str>,
    /// selector(s) on elements classes
    pub(crate) classes: Vec<&'a str>,
    /// selector(s) on elements pseudo-classes
    pub(crate) pseudo_classes: Vec<CssPseudoClass>,
    /// selector(s) on elements attributes
    pub(crate) attributes: Vec<CssAttributeSelector<'a>>,
}

impl<'a> CssSelector<'a> {
    #[cfg(test)]
    pub(crate) fn for_element(element: &'a str) -> Self {
        CssSelector {
            element: Some(element),
            id: None,
            classes: vec![],
            pseudo_classes: vec![],
            attributes: vec![],
        }
    }

    #[cfg(test)]
    pub(crate) fn for_id(id: &'a str) -> Self {
        CssSelector {
            element: None,
            id: Some(id),
            classes: vec![],
            pseudo_classes: vec![],
            attributes: vec![],
        }
    }

    #[cfg(test)]
    pub(crate) fn for_class(class: &'a str) -> Self {
        CssSelector {
            element: None,
            id: None,
            classes: vec![class],
            pseudo_classes: vec![],
            attributes: vec![],
        }
    }

    #[cfg(test)]
    pub(crate) fn for_classes(classes: Vec<&'a str>) -> Self {
        CssSelector {
            element: None,
            id: None,
            classes,
            pseudo_classes: vec![],
            attributes: vec![],
        }
    }

    #[cfg(test)]
    pub(crate) fn for_attribute(attribute: CssAttributeSelector<'a>) -> Self {
        CssSelector {
            element: None,
            id: None,
            classes: vec![],
            pseudo_classes: vec![],
            attributes: vec![attribute],
        }
    }

    pub(crate) fn query(
        &self,
        nodes: &Vec<rctree::Node<HtmlContent>>,
    ) -> Vec<rctree::Node<HtmlContent>> {
        let mut findings = vec![];

        for node in nodes {
            if node.matches_selector(self) {
                findings.push(rctree::Node::clone(node));
            }
        }

        findings
    }
}

/// Combining different "steps" of the CSS selector path
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CssSelectorCombinator {
    /// used internally only, to indicate the beginning of the path
    Start,
    /// CSS descendent combinator "a b"
    Descendent,
    /// CSS direct child combinator "a > b"
    DirectChild,
    /// CSS general sibling combinator "a ~ b"
    GeneralSibling,
    /// CSS adjacent sibling combinator "a + b"
    AdjacentSibling,
}

/// Individual "step" of the CSS Selector path
///
/// # examples
///
/// `a > b` would be two steps:
/// - start of a
/// - direct-child of b
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CssSelectorStep<'a> {
    pub selector: CssSelector<'a>,
    pub combinator: CssSelectorCombinator,
}

impl<'a> CssSelectorStep<'a> {
    pub fn start(selector: CssSelector<'a>) -> Self {
        CssSelectorStep {
            selector,
            combinator: CssSelectorCombinator::Start,
        }
    }

    pub fn direct_child(selector: CssSelector<'a>) -> Self {
        CssSelectorStep {
            selector,
            combinator: CssSelectorCombinator::DirectChild,
        }
    }

    pub fn descendent(selector: CssSelector<'a>) -> Self {
        CssSelectorStep {
            selector,
            combinator: CssSelectorCombinator::Descendent,
        }
    }

    pub fn general_sibling(selector: CssSelector<'a>) -> Self {
        CssSelectorStep {
            selector,
            combinator: CssSelectorCombinator::GeneralSibling,
        }
    }

    pub fn adjacent_sibling(selector: CssSelector<'a>) -> Self {
        CssSelectorStep {
            selector,
            combinator: CssSelectorCombinator::AdjacentSibling,
        }
    }
}

/// a whole "path" of CSS selectors
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CssSelectorPath<'a>(Vec<CssSelectorStep<'a>>);

impl<'a> CssSelectorPath<'a> {
    #[cfg(test)]
    pub fn single(step: CssSelector<'a>) -> Self {
        CssSelectorPath(vec![CssSelectorStep::start(step)])
    }

    pub fn new(start: CssSelector<'a>, rest: Vec<CssSelectorStep<'a>>) -> Self {
        let mut list = vec![CssSelectorStep::start(start)];
        list.extend_from_slice(&rest);
        CssSelectorPath(list)
    }

    pub(crate) fn query(
        &self,
        start: &[rctree::Node<HtmlContent>],
    ) -> Vec<rctree::Node<HtmlContent>> {
        let mut findings = start.iter().map(rctree::Node::clone).collect::<Vec<_>>();

        for step in &self.0 {
            let candidates = Self::resolve_combinator(&step.combinator, findings);
            findings = step.selector.query(&candidates);
        }

        findings
    }

    fn resolve_combinator(
        combinator: &CssSelectorCombinator,
        source: Vec<rctree::Node<HtmlContent>>,
    ) -> Vec<rctree::Node<HtmlContent>> {
        match combinator {
            CssSelectorCombinator::Start => source
                .iter()
                .flat_map(|s| s.descendants())
                .collect::<Vec<_>>(),
            CssSelectorCombinator::Descendent => source
                .iter()
                .flat_map(|s| s.descendants().filter(move |d| s != d))
                .collect::<Vec<_>>(),
            CssSelectorCombinator::DirectChild => {
                source.iter().flat_map(|s| s.children()).collect::<Vec<_>>()
            }
            CssSelectorCombinator::GeneralSibling => source
                .iter()
                .flat_map(|s| s.following_siblings().filter(move |d| s != d))
                .collect::<Vec<_>>(),
            CssSelectorCombinator::AdjacentSibling => source
                .iter()
                .flat_map(Self::find_tag_sibling)
                .collect::<Vec<_>>(),
        }
    }

    fn find_tag_sibling(start: &rctree::Node<HtmlContent>) -> Option<rctree::Node<HtmlContent>> {
        let mut candidate = start.next_sibling();
        while let Some(node) = candidate {
            if node.borrow().is_tag() {
                return Some(node);
            } else {
                candidate = node.next_sibling();
            }
        }

        None
    }
}

impl<'a> Index<usize> for CssSelectorPath<'a> {
    type Output = CssSelectorStep<'a>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

/// the list of all (comma-seperated) CSS paths
#[derive(Debug, PartialEq, Clone, Eq)]
pub struct CssSelectorList<'a>(Vec<CssSelectorPath<'a>>);

impl<'a> CssSelectorList<'a> {
    pub fn new(content: Vec<CssSelectorPath<'a>>) -> Self {
        CssSelectorList(content)
    }

    pub(crate) fn query(
        &self,
        start: &[rctree::Node<HtmlContent>],
    ) -> Vec<rctree::Node<HtmlContent>> {
        trace!("Querying using Selector {:#?}", &self.0);

        self.0
            .iter()
            .flat_map(|p| p.query(start))
            .collect::<Vec<_>>()
    }
}
