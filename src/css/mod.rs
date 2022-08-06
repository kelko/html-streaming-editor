#[cfg(test)]
mod tests;

use crate::HtmlIndex;
use std::borrow::Cow;
use std::collections::HashSet;
use std::fmt::{Debug, Formatter};
use std::ops::Index;
use tl::{Bytes, HTMLTag, NodeHandle};

/// CSS [pseudo classes](https://developer.mozilla.org/en-US/docs/Web/CSS/Pseudo-classes) selector
#[derive(Clone, Debug, PartialEq)]
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
#[derive(Clone, Debug, PartialEq)]
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
    fn matches(&self, attribute: &Bytes) -> bool {
        let given_value = attribute.as_utf8_str();

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

    fn equals_till_hyphen(expected_value: &str, given_value: Cow<str>) -> bool {
        match given_value.find('-') {
            None => given_value.eq(expected_value),
            Some(position) => {
                let slice = &given_value[0..position];
                slice.eq(expected_value)
            }
        }
    }
}

impl<'a> Clone for CssAttributeSelector<'a> {
    fn clone(&self) -> Self {
        CssAttributeSelector {
            attribute: self.attribute.clone(),
            operator: self.operator.clone(),
            value: self.value.clone(),
        }
    }
}

impl<'a> PartialEq for CssAttributeSelector<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.attribute == other.attribute
            && self.operator == other.operator
            && self.value == other.value
    }
}

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
        index: &HtmlIndex,
        nodes: HashSet<NodeHandle>,
    ) -> HashSet<NodeHandle> {
        let mut findings = HashSet::new();

        for node in nodes {
            if let Some(tag) = node.get(index.dom.parser()).unwrap().as_tag() {
                if self.matches(tag) {
                    findings.insert(node.clone());
                }
            }
        }

        return findings;
    }

    pub(crate) fn matches(&self, tag: &HTMLTag) -> bool {
        if let Some(element) = self.element {
            if element.as_bytes() != tag.name().as_bytes() {
                return false;
            }
        }

        if let Some(id) = self.id {
            if let Some(tag_id) = tag.attributes().id() {
                if id.as_bytes() != tag_id.as_bytes() {
                    return false;
                }
            } else {
                return false;
            }
        }

        for class in self.classes.iter() {
            if !tag.attributes().is_class_member(class) {
                return false;
            }
        }

        for _pseudo_class in self.pseudo_classes.iter() {
            todo!("Implement pseudo-class support")
        }

        for attribute in self.attributes.iter() {
            if let Some(attribute_value_bytes) = tag.attributes().get(attribute.attribute).flatten()
            {
                if !attribute.matches(attribute_value_bytes) {
                    return false;
                }
            } else {
                return false;
            }
        }

        true
    }
}

impl<'a> Clone for CssSelector<'a> {
    fn clone(&self) -> Self {
        CssSelector {
            element: self.element.clone(),
            id: self.id.clone(),
            classes: self.classes.clone(),
            pseudo_classes: self.pseudo_classes.clone(),
            attributes: self.attributes.clone(),
        }
    }
}

impl<'a> PartialEq for CssSelector<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.element == other.element
            && self.id == other.id
            && self.classes == other.classes
            && self.pseudo_classes == other.pseudo_classes
            && self.attributes == other.attributes
    }
}

impl<'a> Debug for CssSelector<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("booo")
    }
}

/// Combining different "steps" of the CSS selector path
#[derive(Clone, Debug, PartialEq)]
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

impl<'a> Clone for CssSelectorStep<'a> {
    fn clone(&self) -> Self {
        CssSelectorStep {
            selector: self.selector.clone(),
            combinator: self.combinator.clone(),
        }
    }
}

impl<'a> PartialEq for CssSelectorStep<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.combinator == other.combinator && self.selector == other.selector
    }
}

/// a whole "path" of CSS selectors
pub struct CssSelectorPath<'a>(Vec<CssSelectorStep<'a>>);

impl<'a> CssSelectorPath<'a> {
    pub fn single(step: CssSelector<'a>) -> Self {
        CssSelectorPath(vec![CssSelectorStep::descendent(step)])
    }

    pub fn new(start: CssSelector<'a>, rest: Vec<CssSelectorStep<'a>>) -> Self {
        let mut list = vec![CssSelectorStep::descendent(start)];
        list.extend_from_slice(&rest);
        CssSelectorPath(list)
    }

    pub fn as_vec(&self) -> Vec<CssSelectorStep<'a>> {
        return self.0.clone();
    }

    pub(crate) fn query(
        &self,
        index: &HtmlIndex,
        start: &HashSet<NodeHandle>,
    ) -> HashSet<NodeHandle> {
        let mut findings = start.clone();

        for step in self.0.iter() {
            let candidates = Self::resolve_combinator(index, &step.combinator, findings);
            findings = step.selector.query(index, candidates);
        }

        return findings;
    }

    fn resolve_combinator(
        index: &HtmlIndex,
        combinator: &CssSelectorCombinator,
        source: HashSet<NodeHandle>,
    ) -> HashSet<NodeHandle> {
        match combinator {
            CssSelectorCombinator::Start => source
                .iter()
                .flat_map(|n| index.get(n).unwrap().descendents.clone())
                .chain(source.clone())
                .collect::<HashSet<_>>(),
            CssSelectorCombinator::Descendent => source
                .iter()
                .flat_map(|n| index.get(n).unwrap().descendents.clone())
                .collect::<HashSet<_>>(),
            CssSelectorCombinator::DirectChild => source
                .iter()
                .flat_map(|n| index.get(n).unwrap().children.clone())
                .collect::<HashSet<_>>(),
            CssSelectorCombinator::GeneralSibling => source
                .iter()
                .flat_map(|n| index.get(n).unwrap().siblings.clone())
                .collect::<HashSet<_>>(),
            CssSelectorCombinator::AdjacentSibling => source
                .iter()
                .filter_map(|n| index.get(n).unwrap().direct_sibling)
                .collect::<HashSet<_>>(),
        }
    }
}

impl<'a> Index<usize> for CssSelectorPath<'a> {
    type Output = CssSelectorStep<'a>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl<'a> Clone for CssSelectorPath<'a> {
    fn clone(&self) -> Self {
        CssSelectorPath(self.0.clone())
    }
}

impl<'a> PartialEq for CssSelectorPath<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl<'a> Debug for CssSelectorPath<'a> {
    //TODO: Actually implement it
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("TODO")
    }
}

/// the list of all (comma-seperated) CSS paths
pub struct CssSelectorList<'a>(Vec<CssSelectorPath<'a>>);

impl<'a> CssSelectorList<'a> {
    pub fn new(content: Vec<CssSelectorPath<'a>>) -> Self {
        CssSelectorList(content)
    }

    pub fn as_vec(&self) -> Vec<CssSelectorPath<'a>> {
        return self.0.clone();
    }

    pub(crate) fn query(
        &self,
        index: &'a HtmlIndex<'a>,
        start: &HashSet<NodeHandle>,
    ) -> HashSet<NodeHandle> {
        self.0
            .iter()
            .flat_map(|p| p.query(index, start))
            .collect::<HashSet<_>>()
    }
}

impl<'a> PartialEq for CssSelectorList<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl<'a> Debug for CssSelectorList<'a> {
    //TODO: Actually implement it
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("TODO")
    }
}
