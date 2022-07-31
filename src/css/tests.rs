use crate::{
    CssAttributeComparison, CssAttributeSelector, CssSelector, CssSelectorList, CssSelectorPath,
    CssSelectorStep, HtmlIndex,
};
use std::collections::HashSet;

#[test]
fn parse_selector_only_element() {
    let parsed = crate::parsing::grammar::css_selector("a");
    assert_eq!(parsed, Ok(CssSelector::for_element("a")))
}

#[test]
fn parse_selector_only_id() {
    let parsed = crate::parsing::grammar::css_selector("#a");
    assert_eq!(parsed, Ok(CssSelector::for_id("a")))
}

#[test]
fn parse_selector_only_class() {
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
fn parse_single_step_path() {
    let parsed = crate::parsing::grammar::css_selector_path("a");
    assert_eq!(
        parsed,
        Ok(CssSelectorPath::single(CssSelector::for_element("a")))
    )
}

#[test]
fn parse_two_step_path_descendant() {
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
fn parse_two_step_path_direct_children() {
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
fn parse_two_step_path_general_sibling() {
    let parsed = crate::parsing::grammar::css_selector_path("a ~ b");
    assert_eq!(
        parsed,
        Ok(CssSelectorPath::new(
            CssSelector::for_element("a"),
            vec![CssSelectorStep::general_sibling(CssSelector::for_element(
                "b"
            ))]
        ))
    )
}

#[test]
fn parse_two_step_path_general_siblings_spaceless() {
    let parsed = crate::parsing::grammar::css_selector_path("a~b");
    assert_eq!(
        parsed,
        Ok(CssSelectorPath::new(
            CssSelector::for_element("a"),
            vec![CssSelectorStep::general_sibling(CssSelector::for_element(
                "b"
            ))]
        ))
    )
}

#[test]
fn parse_two_step_path_adjacent_sibling() {
    let parsed = crate::parsing::grammar::css_selector_path("a + b");
    assert_eq!(
        parsed,
        Ok(CssSelectorPath::new(
            CssSelector::for_element("a"),
            vec![CssSelectorStep::adjacent_sibling(CssSelector::for_element(
                "b"
            ))]
        ))
    )
}

#[test]
fn parse_two_step_path_adjacent_siblings_spaceless() {
    let parsed = crate::parsing::grammar::css_selector_path("a+b");
    assert_eq!(
        parsed,
        Ok(CssSelectorPath::new(
            CssSelector::for_element("a"),
            vec![CssSelectorStep::adjacent_sibling(CssSelector::for_element(
                "b"
            ))]
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

#[test]
fn query_single_level_by_element_name() {
    let dom = tl::parse(
        "<html><head></head><body><header id=\"element-under-test\"><h1>Hallo</h1></header><main><p>Ups <em>I'm sorry</em></p><img src=\"\"></main><footer></footer><nav><ul><li>1</li><li>2</li></ul></nav></body></html>",
        tl::ParserOptions::default(),
    )
        .unwrap();
    let index = HtmlIndex::load(&dom);

    let selector = CssSelectorList::new(vec![CssSelectorPath::single(CssSelector::for_element(
        "header",
    ))]);

    let starting_elements = HashSet::from_iter(dom.children().iter().cloned());
    let result = selector.query(&index, &starting_elements);

    let header_handle = dom.get_element_by_id("element-under-test").unwrap();

    assert_eq!(result.len(), 1);
    assert!(result.contains(&header_handle));
}

#[test]
fn query_single_level_by_id() {
    let dom = tl::parse(
        "<html><head></head><body><header><h1>Hallo</h1></header><main><p>Ups <em id=\"element-under-test\">I'm sorry</em></p><img src=\"\"></main><footer></footer><nav><ul><li>1</li><li>2</li></ul></nav></body></html>",
        tl::ParserOptions::default(),
    )
        .unwrap();
    let index = HtmlIndex::load(&dom);

    let selector = CssSelectorList::new(vec![CssSelectorPath::single(CssSelector::for_id(
        "element-under-test",
    ))]);

    let starting_elements = HashSet::from_iter(dom.children().iter().cloned());
    let result = selector.query(&index, &starting_elements);

    let element_handle = dom.get_element_by_id("element-under-test").unwrap();

    assert_eq!(result.len(), 1);
    assert!(result.contains(&element_handle));
}

#[test]
fn query_single_level_by_single_class() {
    let dom = tl::parse(
        "<html><head></head><body><header><h1>Hallo</h1></header><main><p>Ups <em id=\"element-under-test\" class=\"single-class\">I'm sorry</em></p><img src=\"\"></main><footer></footer><nav><ul><li>1</li><li>2</li></ul></nav></body></html>",
        tl::ParserOptions::default(),
    )
        .unwrap();
    let index = HtmlIndex::load(&dom);

    let selector = CssSelectorList::new(vec![CssSelectorPath::single(CssSelector::for_class(
        "single-class",
    ))]);

    let starting_elements = HashSet::from_iter(dom.children().iter().cloned());
    let result = selector.query(&index, &starting_elements);

    let element_handle = dom.get_element_by_id("element-under-test").unwrap();

    assert_eq!(result.len(), 1);
    assert!(result.contains(&element_handle));
}
