use crate::{
    CssAttributeComparison, CssAttributeSelector, CssSelector, CssSelectorList, CssSelectorPath,
    CssSelectorStep,
};

#[test]
fn parse_selector_onlyelement() {
    let parsed = crate::parsing::grammar::css_selector("a");
    assert_eq!(parsed, Ok(CssSelector::for_element("a")))
}

#[test]
fn parse_selector_onlyid() {
    let parsed = crate::parsing::grammar::css_selector("#a");
    assert_eq!(parsed, Ok(CssSelector::for_id("a")))
}

#[test]
fn parse_selector_onlyclass() {
    let parsed = crate::parsing::grammar::css_selector(".a");
    assert_eq!(parsed, Ok(CssSelector::for_class("a")))
}

#[test]
fn parse_selector_attribute_exists() {
    let parsed = crate::parsing::grammar::css_selector("[a]");
    assert_eq!(
        parsed,
        Ok(CssSelector::for_attribute(CssAttributeSelector {
            attribute: "a",
            operator: CssAttributeComparison::Exist,
            value: None
        }))
    )
}

#[test]
fn parse_selector_attribute_equals() {
    let parsed = crate::parsing::grammar::css_selector("[a=\"b\"]");
    assert_eq!(
        parsed,
        Ok(CssSelector::for_attribute(CssAttributeSelector {
            attribute: "a",
            operator: CssAttributeComparison::EqualsExact,
            value: Some("b")
        }))
    )
}

#[test]
fn parse_selector_mix() {
    let parsed = crate::parsing::grammar::css_selector("a#blubb.foo.bar[a=\"b\"]");
    assert_eq!(
        parsed,
        Ok(CssSelector {
            element: Some("a"),
            id: Some("blubb"),
            classes: vec!["foo", "bar"],
            pseudo_classes: vec![],
            attributes: vec![CssAttributeSelector {
                attribute: "a",
                operator: CssAttributeComparison::EqualsExact,
                value: Some("b")
            }]
        })
    )
}

#[test]
fn parse_path_single() {
    let parsed = crate::parsing::grammar::css_selector_path("a");
    assert_eq!(
        parsed,
        Ok(CssSelectorPath::single(CssSelector::for_element("a")))
    )
}

#[test]
fn parse_path_two_descendant() {
    let parsed = crate::parsing::grammar::css_selector_path("a b");
    assert_eq!(
        parsed,
        Ok(CssSelectorPath::new(
            CssSelector::for_element("a"),
            vec![CssSelectorStep::descendent(CssSelector::for_element("b"))]
        ))
    )
}

#[test]
fn parse_path_two_direct_child() {
    let parsed = crate::parsing::grammar::css_selector_path("a > b");
    assert_eq!(
        parsed,
        Ok(CssSelectorPath::new(
            CssSelector::for_element("a"),
            vec![CssSelectorStep::direct_child(CssSelector::for_element("b"))]
        ))
    )
}

#[test]
fn parse_path_two_sibling() {
    let parsed = crate::parsing::grammar::css_selector_path("a + b");
    assert_eq!(
        parsed,
        Ok(CssSelectorPath::new(
            CssSelector::for_element("a"),
            vec![CssSelectorStep::sibling(CssSelector::for_element("b"))]
        ))
    )
}

#[test]
fn parse_path_two_spaceless() {
    let parsed = crate::parsing::grammar::css_selector_path("a+b");
    assert_eq!(
        parsed,
        Ok(CssSelectorPath::new(
            CssSelector::for_element("a"),
            vec![CssSelectorStep::sibling(CssSelector::for_element("b"))]
        ))
    )
}

#[test]
fn parse_list_single() {
    let parsed = crate::parsing::grammar::css_selector_list("a");
    assert_eq!(
        parsed,
        Ok(CssSelectorList::new(vec![CssSelectorPath::single(
            CssSelector::for_element("a")
        )]))
    )
}

#[test]
fn parse_list_two() {
    let parsed = crate::parsing::grammar::css_selector_list("a,.b");
    assert_eq!(
        parsed,
        Ok(CssSelectorList::new(vec![
            CssSelectorPath::single(CssSelector::for_element("a")),
            CssSelectorPath::single(CssSelector::for_class("b")),
        ]))
    )
}

#[test]
fn parse_list_complex() {
    let parsed = crate::parsing::grammar::css_selector_list(
        "img[src],main#content .single aside, .row > .col",
    );
    assert_eq!(
        parsed,
        Ok(CssSelectorList::new(vec![
            CssSelectorPath::single(CssSelector {
                element: Some("img"),
                id: None,
                classes: vec![],
                pseudo_classes: vec![],
                attributes: vec![CssAttributeSelector {
                    attribute: "src",
                    operator: CssAttributeComparison::Exist,
                    value: None
                }]
            }),
            CssSelectorPath::new(
                CssSelector {
                    element: Some("main"),
                    id: Some("content"),
                    classes: vec![],
                    pseudo_classes: vec![],
                    attributes: vec![]
                },
                vec![
                    CssSelectorStep::descendent(CssSelector::for_class("single")),
                    CssSelectorStep::descendent(CssSelector::for_element("aside")),
                ]
            ),
            CssSelectorPath::new(
                CssSelector::for_class("row"),
                vec![CssSelectorStep::direct_child(CssSelector::for_class("col"))]
            ),
        ]))
    )
}
