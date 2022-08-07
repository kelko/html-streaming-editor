use crate::{
    Command, CssSelector, CssSelectorList, CssSelectorPath, CssSelectorStep, HtmlIndex, Pipeline,
};

const TEST_HTML_DOCUMENT: &str = "<html>
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
</html>";

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
    let index = HtmlIndex::load(dom);
    let starting_elements = index.root_elements();
    let result = pipeline.run_on(starting_elements, &index).unwrap();

    let element_handle = index.dom.borrow().get_element_by_id("first-para").unwrap();

    assert_eq!(result.len(), 1);
    assert!(result.contains(&element_handle));
}

#[test]
fn run_on_single_filter() {
    let pipeline = Pipeline::new(vec![Command::Filter(CssSelectorList::new(vec![
        CssSelectorPath::new(
            CssSelector::for_element("h1"),
            vec![CssSelectorStep::adjacent_sibling(CssSelector::for_element(
                "p",
            ))],
        ),
    ]))]);

    let dom = tl::parse(TEST_HTML_DOCUMENT, tl::ParserOptions::default()).unwrap();
    let index = HtmlIndex::load(dom);
    let starting_elements = index.root_elements();
    let result = pipeline.run_on(starting_elements, &index).unwrap();

    let element_handle = index.dom.borrow().get_element_by_id("first-para").unwrap();

    assert_eq!(result.len(), 1);
    assert!(!result.contains(&element_handle));
}
