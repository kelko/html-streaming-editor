use crate::html::HtmlRenderable;
use crate::{
    Command, CssSelector, CssSelectorList, CssSelectorPath, CssSelectorStep, HtmlContent, Pipeline,
};

const TEST_HTML_DOCUMENT: &str = r#"<html>
    <head></head>
    <body>
        <h1>Title</h1>
        <p id="first-para">Some <em class="fancy">first</em> text</p>
        <p id="second-para">Some more text, even with an <img src=""></p>
        <p id="third-para">Third text of <abbr>HTML</abbr>, but no <abbr>CSS</abbr></p>
        <ul id="list">
            <li id="item-1">1</li>
            <li id="item-2">2</li>
            <li id="item-3">3</li>
        </ul>
    </body>
</html>"#;

#[test]
fn run_on_single_only() {
    let pipeline = Pipeline::new(vec![Command::Only(CssSelectorList::new(vec![
        CssSelectorPath::new(
            CssSelector::for_element("h1"),
            vec![CssSelectorStep::adjacent_sibling(CssSelector::for_element(
                "p",
            ))],
        ),
    ]))]);

    let dom = tl::parse(TEST_HTML_DOCUMENT, tl::ParserOptions::default()).unwrap();
    let starting_elements = HtmlContent::import(dom).unwrap();

    let mut result = pipeline
        .run_on(vec![rctree::Node::clone(&starting_elements)])
        .unwrap();

    assert_eq!(result.len(), 1);
    let first_result = result.pop().unwrap();
    assert_eq!(
        first_result.outer_html(),
        String::from(r#"<p id="first-para">Some <em class="fancy">first</em> text</p>"#)
    );
}

#[test]
fn run_on_single_without() {
    let pipeline = Pipeline::new(vec![Command::Without(CssSelectorList::new(vec![
        CssSelectorPath::new(
            CssSelector::for_element("h1"),
            vec![CssSelectorStep::adjacent_sibling(CssSelector::for_element(
                "p",
            ))],
        ),
    ]))]);

    let dom = tl::parse(TEST_HTML_DOCUMENT, tl::ParserOptions::default()).unwrap();
    let starting_elements = HtmlContent::import(dom).unwrap();
    let mut result = pipeline.run_on(vec![starting_elements]).unwrap();

    assert_eq!(result.len(), 1);
    let first_result = result.pop().unwrap();
    assert_eq!(
        first_result.outer_html(),
        String::from(
            r#"<html>
    <head></head>
    <body>
        <h1>Title</h1>
        
        <p id="second-para">Some more text, even with an <img src=""></p>
        <p id="third-para">Third text of <abbr>HTML</abbr>, but no <abbr>CSS</abbr></p>
        <ul id="list">
            <li id="item-1">1</li>
            <li id="item-2">2</li>
            <li id="item-3">3</li>
        </ul>
    </body>
</html>"#
        )
    );
}
