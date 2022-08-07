use crate::{
    CssAttributeComparison, CssAttributeSelector, CssSelector, CssSelectorList, CssSelectorPath,
    CssSelectorStep, HtmlIndex,
};

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
            value: None,
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
            value: Some("b"),
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
                value: Some("b"),
            }],
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
            vec![CssSelectorStep::descendent(CssSelector::for_element("b"))],
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
            vec![CssSelectorStep::direct_child(CssSelector::for_element("b"))],
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
            ))],
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
            ))],
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
            ))],
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
            ))],
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
                    value: None,
                }],
            }),
            CssSelectorPath::new(
                CssSelector {
                    element: Some("main"),
                    id: Some("content"),
                    classes: vec![],
                    pseudo_classes: vec![],
                    attributes: vec![],
                },
                vec![
                    CssSelectorStep::descendent(CssSelector::for_class("single")),
                    CssSelectorStep::descendent(CssSelector::for_element("aside")),
                ],
            ),
            CssSelectorPath::new(
                CssSelector::for_class("row"),
                vec![CssSelectorStep::direct_child(CssSelector::for_class("col"))],
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
    let index = HtmlIndex::load(dom);

    let selector = CssSelectorList::new(vec![CssSelectorPath::single(CssSelector::for_element(
        "header",
    ))]);

    let starting_elements = index.root_elements();
    let result = selector.query(&index, &starting_elements);

    let header_handle = index
        .dom
        .borrow()
        .get_element_by_id("element-under-test")
        .unwrap();

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
    let index = HtmlIndex::load(dom);

    let selector = CssSelectorList::new(vec![CssSelectorPath::single(CssSelector::for_id(
        "element-under-test",
    ))]);

    let starting_elements = index.root_elements();
    let result = selector.query(&index, &starting_elements);

    let element_handle = index
        .dom
        .borrow()
        .get_element_by_id("element-under-test")
        .unwrap();

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
    let index = HtmlIndex::load(dom);

    let selector = CssSelectorList::new(vec![CssSelectorPath::single(CssSelector::for_class(
        "single-class",
    ))]);

    let starting_elements = index.root_elements();
    let result = selector.query(&index, &starting_elements);

    let element_handle = index
        .dom
        .borrow()
        .get_element_by_id("element-under-test")
        .unwrap();

    assert_eq!(result.len(), 1);
    assert!(result.contains(&element_handle));
}

#[test]
fn query_single_level_by_multiple_classes() {
    let dom = tl::parse(
        "<html><head></head><body><header><h1 class=\"single-class\">Hallo</h1></header><main><p>Ups <em id=\"element-under-test\" class=\"single-class other-class\">I'm sorry</em></p><img src=\"\"></main><footer></footer><nav><ul><li>1</li><li>2</li></ul></nav></body></html>",
        tl::ParserOptions::default(),
    )
        .unwrap();
    let index = HtmlIndex::load(dom);

    let selector = CssSelectorList::new(vec![CssSelectorPath::single(CssSelector::for_classes(
        vec!["single-class", "other-class"],
    ))]);

    let starting_elements = index.root_elements();
    let result = selector.query(&index, &starting_elements);

    let element_handle = index
        .dom
        .borrow()
        .get_element_by_id("element-under-test")
        .unwrap();

    assert_eq!(result.len(), 1);
    assert!(result.contains(&element_handle));
}

#[test]
fn query_single_level_by_attribute_existence() {
    let dom = tl::parse(
        "<html><head></head><body><header><h1>Hallo</h1></header><main><p>Ups <em id=\"element-under-test\" data-test=\"its a me\">I'm sorry</em></p><img src=\"\"></main><footer></footer><nav><ul><li>1</li><li>2</li></ul></nav></body></html>",
        tl::ParserOptions::default(),
    )
        .unwrap();
    let index = HtmlIndex::load(dom);

    let selector = CssSelectorList::new(vec![CssSelectorPath::single(CssSelector::for_attribute(
        CssAttributeSelector {
            attribute: "data-test",
            operator: CssAttributeComparison::Exist,
            value: None,
        },
    ))]);

    let starting_elements = index.root_elements();
    let result = selector.query(&index, &starting_elements);

    let element_handle = index
        .dom
        .borrow()
        .get_element_by_id("element-under-test")
        .unwrap();

    assert_eq!(result.len(), 1);
    assert!(result.contains(&element_handle));
}

#[test]
fn query_single_level_by_attribute_contains_term() {
    let dom = tl::parse(
        "<html><head></head><body><header><h1 data-test=\"I am not it\">Hallo</h1></header><main><p>Ups <em id=\"element-under-test\" data-test=\"its a me\">I'm sorry</em></p><img src=\"\"></main><footer></footer><nav><ul><li>1</li><li>2</li></ul></nav></body></html>",
        tl::ParserOptions::default(),
    )
        .unwrap();
    let index = HtmlIndex::load(dom);

    let selector = CssSelectorList::new(vec![CssSelectorPath::single(CssSelector::for_attribute(
        CssAttributeSelector {
            attribute: "data-test",
            operator: CssAttributeComparison::TermContains,
            value: Some("me"),
        },
    ))]);

    let starting_elements = index.root_elements();
    let result = selector.query(&index, &starting_elements);

    let element_handle = index
        .dom
        .borrow()
        .get_element_by_id("element-under-test")
        .unwrap();

    assert_eq!(result.len(), 1);
    assert!(result.contains(&element_handle));
}

#[test]
fn query_single_level_by_attribute_contains_character() {
    let dom = tl::parse(
        "<html><head></head><body><header><h1 data-test=\"I am not it\">Hallo</h1></header><main><p>Ups <em id=\"element-under-test\" data-test=\"its a me\">I'm sorry</em></p><img src=\"\"></main><footer></footer><nav><ul><li>1</li><li>2</li></ul></nav></body></html>",
        tl::ParserOptions::default(),
    )
        .unwrap();
    let index = HtmlIndex::load(dom);

    let selector = CssSelectorList::new(vec![CssSelectorPath::single(CssSelector::for_attribute(
        CssAttributeSelector {
            attribute: "data-test",
            operator: CssAttributeComparison::CharacterContains,
            value: Some("ts"),
        },
    ))]);

    let starting_elements = index.root_elements();
    let result = selector.query(&index, &starting_elements);

    let element_handle = index
        .dom
        .borrow()
        .get_element_by_id("element-under-test")
        .unwrap();

    assert_eq!(result.len(), 1);
    assert!(result.contains(&element_handle));
}

#[test]
fn query_single_level_by_attribute_starts() {
    let dom = tl::parse(
        "<html><head></head><body><header><h1 data-test=\"I am not it\">Hallo</h1></header><main><p>Ups <em id=\"element-under-test\" data-test=\"its a me\">I'm sorry</em></p><img src=\"\"></main><footer></footer><nav><ul><li>1</li><li>2</li></ul></nav></body></html>",
        tl::ParserOptions::default(),
    )
        .unwrap();
    let index = HtmlIndex::load(dom);

    let selector = CssSelectorList::new(vec![CssSelectorPath::single(CssSelector::for_attribute(
        CssAttributeSelector {
            attribute: "data-test",
            operator: CssAttributeComparison::Starts,
            value: Some("its"),
        },
    ))]);

    let starting_elements = index.root_elements();
    let result = selector.query(&index, &starting_elements);

    let element_handle = index
        .dom
        .borrow()
        .get_element_by_id("element-under-test")
        .unwrap();

    assert_eq!(result.len(), 1);
    assert!(result.contains(&element_handle));
}

#[test]
fn query_single_level_by_attribute_ends() {
    let dom = tl::parse(
        "<html><head></head><body><header><h1 data-test=\"I am not it\">Hallo</h1></header><main><p>Ups <em id=\"element-under-test\" data-test=\"its a me\">I'm sorry</em></p><img src=\"\"></main><footer></footer><nav><ul><li>1</li><li>2</li></ul></nav></body></html>",
        tl::ParserOptions::default(),
    )
        .unwrap();
    let index = HtmlIndex::load(dom);

    let selector = CssSelectorList::new(vec![CssSelectorPath::single(CssSelector::for_attribute(
        CssAttributeSelector {
            attribute: "data-test",
            operator: CssAttributeComparison::Ends,
            value: Some("me"),
        },
    ))]);

    let starting_elements = index.root_elements();
    let result = selector.query(&index, &starting_elements);

    let element_handle = index
        .dom
        .borrow()
        .get_element_by_id("element-under-test")
        .unwrap();

    assert_eq!(result.len(), 1);
    assert!(result.contains(&element_handle));
}

#[test]
fn query_single_level_by_attribute_equals_exact() {
    let dom = tl::parse(
        "<html><head></head><body><header><h1 data-test=\"its not me\">Hallo</h1></header><main><p>Ups <em id=\"element-under-test\" data-test=\"its a me\">I'm sorry</em></p><img src=\"\"></main><footer></footer><nav><ul><li>1</li><li>2</li></ul></nav></body></html>",
        tl::ParserOptions::default(),
    )
        .unwrap();
    let index = HtmlIndex::load(dom);

    let selector = CssSelectorList::new(vec![CssSelectorPath::single(CssSelector::for_attribute(
        CssAttributeSelector {
            attribute: "data-test",
            operator: CssAttributeComparison::EqualsExact,
            value: Some("its a me"),
        },
    ))]);

    let starting_elements = index.root_elements();
    let result = selector.query(&index, &starting_elements);

    let element_handle = index
        .dom
        .borrow()
        .get_element_by_id("element-under-test")
        .unwrap();

    assert_eq!(result.len(), 1);
    assert!(result.contains(&element_handle));
}

#[test]
fn query_single_level_by_attribute_equals_till_hyphen() {
    let dom = tl::parse(
        "<html><head></head><body><header><h1 data-test=\"terminology\">Hallo</h1></header><main><p>Ups <em id=\"element-under-test\" data-test=\"term-a\">I'm sorry</em></p><img src=\"\"></main><footer></footer><nav><ul><li>1</li><li>2</li></ul></nav></body></html>",
        tl::ParserOptions::default(),
    )
        .unwrap();
    let index = HtmlIndex::load(dom);

    let selector = CssSelectorList::new(vec![CssSelectorPath::single(CssSelector::for_attribute(
        CssAttributeSelector {
            attribute: "data-test",
            operator: CssAttributeComparison::EqualsTillHyphen,
            value: Some("term"),
        },
    ))]);

    let starting_elements = index.root_elements();
    let result = selector.query(&index, &starting_elements);

    let element_handle = index
        .dom
        .borrow()
        .get_element_by_id("element-under-test")
        .unwrap();

    assert_eq!(result.len(), 1);
    assert!(result.contains(&element_handle));
}

#[test]
fn query_descendents() {
    let dom = tl::parse(
        "<html><head></head><body><header><h1>Hallo</h1></header><main><p>Ups<em id=\"element-under-test\" class=\"single-class\">I'm sorry</em></p><img src=\"\"></main><footer></footer><nav><ul><li>1</li><li>2</li></ul></nav></body></html>",
        tl::ParserOptions::default(),
    )
        .unwrap();
    let index = HtmlIndex::load(dom);

    let selector = CssSelectorList::new(vec![CssSelectorPath::new(
        CssSelector::for_element("main"),
        vec![CssSelectorStep::descendent(CssSelector::for_class(
            "single-class",
        ))],
    )]);

    let starting_elements = index.root_elements();
    let result = selector.query(&index, &starting_elements);

    let element_handle = index
        .dom
        .borrow()
        .get_element_by_id("element-under-test")
        .unwrap();

    assert_eq!(result.len(), 1);
    assert!(result.contains(&element_handle));
}

#[test]
fn query_direct_child() {
    let dom = tl::parse(
        "<html><head></head><body><header><h1>Hallo</h1></header><main><p>Ups<em class=\"single-class\">I'm sorry</em></p><img id=\"element-under-test\" class=\"single-class\" src=\"\"></main><footer></footer><nav><ul><li>1</li><li>2</li></ul></nav></body></html>",
        tl::ParserOptions::default(),
    )
        .unwrap();
    let index = HtmlIndex::load(dom);

    let selector = CssSelectorList::new(vec![CssSelectorPath::new(
        CssSelector::for_element("main"),
        vec![CssSelectorStep::direct_child(CssSelector::for_class(
            "single-class",
        ))],
    )]);

    let starting_elements = index.root_elements();
    let result = selector.query(&index, &starting_elements);

    let element_handle = index
        .dom
        .borrow()
        .get_element_by_id("element-under-test")
        .unwrap();

    assert_eq!(result.len(), 1);
    assert!(result.contains(&element_handle));
}

#[test]
fn query_adjacent_sibling() {
    let dom = tl::parse(
        "<html><head></head><body><header><h1>Hallo</h1></header><main><header><h2>Title</h2></header><p id=\"element-under-test\">Hello World</p><p id=\"not-in-test\">Brave new World</p><p>Hello, there</p></main></body></html>",
        tl::ParserOptions::default(),
    )
        .unwrap();
    let index = HtmlIndex::load(dom);

    let selector = CssSelectorList::new(vec![CssSelectorPath::new(
        CssSelector::for_element("header"),
        vec![CssSelectorStep::adjacent_sibling(CssSelector::for_element(
            "p",
        ))],
    )]);

    let starting_elements = index.root_elements();
    let result = selector.query(&index, &starting_elements);

    let element_handle = index
        .dom
        .borrow()
        .get_element_by_id("element-under-test")
        .unwrap();
    let non_result_handle = index.dom.borrow().get_element_by_id("not-in-test").unwrap();

    assert_eq!(result.len(), 1);
    assert!(result.contains(&element_handle));
    assert!(!result.contains(&non_result_handle));
}

#[test]
fn query_adjacent_sibling_with_whitespaces() {
    let dom = tl::parse(
        "<html>
            <head></head>
            <body>
            <h1>Title</h1>
            <p id=\"first-para\">Some first text</p>
        <p id=\"second-para\">Some more text, even with an <img src=\"\"></p>
        <p id=\"third-para\">Third text of <abbr>HTML</abbr>, but no <abbr>CSS</abbr></p>
        <ul id=\"list\">
            <li id=\"item-1\">1</li>
            <li id=\"item-2\">2</li>
            <li id=\"item-3\">3</li>
        </ul>
    </body>
</html>",
        tl::ParserOptions::default(),
    )
    .unwrap();
    let index = HtmlIndex::load(dom);

    let selector = CssSelectorList::new(vec![CssSelectorPath::new(
        CssSelector::for_element("h1"),
        vec![CssSelectorStep::adjacent_sibling(CssSelector::for_element(
            "p",
        ))],
    )]);

    let starting_elements = index.root_elements();
    let result = selector.query(&index, &starting_elements);

    let element_handle = index.dom.borrow().get_element_by_id("first-para").unwrap();
    let non_result_handle = index.dom.borrow().get_element_by_id("second-para").unwrap();

    assert_eq!(result.len(), 1);
    assert!(result.contains(&element_handle));
    assert!(!result.contains(&non_result_handle));
}

#[test]
fn query_general_sibling() {
    let dom = tl::parse(
        "<html><head></head><body><header><h1>Hallo</h1></header><main><p id=\"not-in-test\">Prelude</p><h2>Title</h2><p>Hello World</p><p>Brave new World</p><p id=\"element-under-test\">Hello, there</p></main></body></html>",
        tl::ParserOptions::default(),
    )
        .unwrap();
    let index = HtmlIndex::load(dom);

    let selector = CssSelectorList::new(vec![CssSelectorPath::new(
        CssSelector::for_element("h2"),
        vec![CssSelectorStep::general_sibling(CssSelector::for_element(
            "p",
        ))],
    )]);

    let starting_elements = index.root_elements();
    let result = selector.query(&index, &starting_elements);

    let element_handle = index
        .dom
        .borrow()
        .get_element_by_id("element-under-test")
        .unwrap();

    let non_result_handle = index.dom.borrow().get_element_by_id("not-in-test").unwrap();

    assert_eq!(result.len(), 3);
    assert!(result.contains(&element_handle));
    assert!(!result.contains(&non_result_handle));
}

//TODO: query_or
