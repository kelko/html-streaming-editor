use crate::element_creating::{ElementCreatingCommand, ElementCreatingPipeline};
use crate::html::HtmlRenderable;
use crate::string_creating::{ElementSelectingCommand, ValueExtractingCommand};
use crate::{
    element_processing::{command::ElementProcessingCommand, pipeline::ElementProcessingPipeline},
    CssSelector, CssSelectorList, CssSelectorPath, CssSelectorStep, HtmlContent,
    StringValueCreatingPipeline, ValueSource,
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
    let pipeline = ElementProcessingPipeline::new(vec![ElementProcessingCommand::ExtractElement(
        CssSelectorList::new(vec![CssSelectorPath::new(
            CssSelector::for_element("h1"),
            vec![CssSelectorStep::adjacent_sibling(CssSelector::for_element(
                "p",
            ))],
        )]),
    )]);

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
    let pipeline = ElementProcessingPipeline::new(vec![ElementProcessingCommand::RemoveElement(
        CssSelectorList::new(vec![CssSelectorPath::new(
            CssSelector::for_element("h1"),
            vec![CssSelectorStep::adjacent_sibling(CssSelector::for_element(
                "p",
            ))],
        )]),
    )]);

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

#[test]
fn run_on_single_clear_attr() {
    let pipeline =
        ElementProcessingPipeline::new(vec![ElementProcessingCommand::ClearAttribute("data-test")]);

    let dom = tl::parse(
        r#"<div data-test="foo" class="bar">Some Content</div>"#,
        tl::ParserOptions::default(),
    )
    .unwrap();
    let starting_elements = HtmlContent::import(dom).unwrap();

    let mut result = pipeline
        .run_on(vec![rctree::Node::clone(&starting_elements)])
        .unwrap();

    assert_eq!(result.len(), 1);
    let first_result = result.pop().unwrap();
    assert_eq!(
        first_result.outer_html(),
        String::from(r#"<div class="bar">Some Content</div>"#)
    );
}

#[test]
fn run_on_single_clear_content() {
    let pipeline = ElementProcessingPipeline::new(vec![ElementProcessingCommand::ClearContent]);

    let dom = tl::parse(
        r#"<div data-test="foo" class="bar">Some Content</div>"#,
        tl::ParserOptions::default(),
    )
    .unwrap();
    let starting_elements = HtmlContent::import(dom).unwrap();

    let mut result = pipeline
        .run_on(vec![rctree::Node::clone(&starting_elements)])
        .unwrap();

    assert_eq!(result.len(), 1);
    let first_result = result.pop().unwrap();
    assert_eq!(
        first_result.outer_html(),
        String::from(r#"<div class="bar" data-test="foo"></div>"#)
    );
}

#[test]
fn run_on_single_set_attr_from_string_over_existing_attr() {
    let pipeline = ElementProcessingPipeline::new(vec![ElementProcessingCommand::SetAttribute(
        "data-test",
        ValueSource::StringValue("some text"),
    )]);

    let dom = tl::parse(
        r#"<div data-test="foo" class="bar">Some Content</div>"#,
        tl::ParserOptions::default(),
    )
    .unwrap();
    let starting_elements = HtmlContent::import(dom).unwrap();

    let mut result = pipeline
        .run_on(vec![rctree::Node::clone(&starting_elements)])
        .unwrap();

    assert_eq!(result.len(), 1);
    let first_result = result.pop().unwrap();
    assert_eq!(
        first_result.outer_html(),
        String::from(r#"<div class="bar" data-test="some text">Some Content</div>"#)
    );
}

#[test]
fn run_on_single_set_attr_from_string_as_new_attr() {
    let pipeline = ElementProcessingPipeline::new(vec![ElementProcessingCommand::SetAttribute(
        "data-fubar",
        ValueSource::StringValue("some text"),
    )]);

    let dom = tl::parse(
        r#"<div data-test="foo" class="bar">Some Content</div>"#,
        tl::ParserOptions::default(),
    )
    .unwrap();
    let starting_elements = HtmlContent::import(dom).unwrap();

    let mut result = pipeline
        .run_on(vec![rctree::Node::clone(&starting_elements)])
        .unwrap();

    assert_eq!(result.len(), 1);
    let first_result = result.pop().unwrap();
    assert_eq!(
        first_result.outer_html(),
        String::from(
            r#"<div class="bar" data-fubar="some text" data-test="foo">Some Content</div>"#
        )
    );
}

#[test]
fn run_on_single_set_attr_from_other_attr_as_new_attr() {
    let pipeline = ElementProcessingPipeline::new(vec![ElementProcessingCommand::SetAttribute(
        "data-fubar",
        ValueSource::SubPipeline(StringValueCreatingPipeline::new(
            ElementSelectingCommand::UseElement,
            ValueExtractingCommand::GetAttribute("data-test"),
        )),
    )]);

    let dom = tl::parse(
        r#"<div data-test="foo" class="bar">Some Content</div>"#,
        tl::ParserOptions::default(),
    )
    .unwrap();
    let starting_elements = HtmlContent::import(dom).unwrap();

    let mut result = pipeline
        .run_on(vec![rctree::Node::clone(&starting_elements)])
        .unwrap();

    assert_eq!(result.len(), 1);
    let first_result = result.pop().unwrap();
    assert_eq!(
        first_result.outer_html(),
        String::from(r#"<div class="bar" data-fubar="foo" data-test="foo">Some Content</div>"#)
    );
}

#[test]
fn run_on_single_set_text_content_from_string_for_tag() {
    let pipeline = ElementProcessingPipeline::new(vec![ElementProcessingCommand::SetTextContent(
        ValueSource::StringValue("Other Content"),
    )]);

    let dom = tl::parse(
        r#"<div data-test="foo" class="bar">Some Content</div>"#,
        tl::ParserOptions::default(),
    )
    .unwrap();
    let starting_elements = HtmlContent::import(dom).unwrap();

    let mut result = pipeline
        .run_on(vec![rctree::Node::clone(&starting_elements)])
        .unwrap();

    assert_eq!(result.len(), 1);
    let first_result = result.pop().unwrap();
    assert_eq!(
        first_result.outer_html(),
        String::from(r#"<div class="bar" data-test="foo">Other Content</div>"#)
    );
}

#[test]
fn run_on_single_set_text_content_from_string_for_empty_tag() {
    let pipeline = ElementProcessingPipeline::new(vec![ElementProcessingCommand::SetTextContent(
        ValueSource::StringValue("Other Content"),
    )]);

    let dom = tl::parse(
        r#"<div data-test="foo" class="bar"></div>"#,
        tl::ParserOptions::default(),
    )
    .unwrap();
    let starting_elements = HtmlContent::import(dom).unwrap();

    let mut result = pipeline
        .run_on(vec![rctree::Node::clone(&starting_elements)])
        .unwrap();

    assert_eq!(result.len(), 1);
    let first_result = result.pop().unwrap();
    assert_eq!(
        first_result.outer_html(),
        String::from(r#"<div class="bar" data-test="foo">Other Content</div>"#)
    );
}

#[test]
fn run_on_single_set_text_content_from_attr_for_empty_tag() {
    let pipeline = ElementProcessingPipeline::new(vec![ElementProcessingCommand::SetTextContent(
        ValueSource::SubPipeline(StringValueCreatingPipeline::new(
            ElementSelectingCommand::UseElement,
            ValueExtractingCommand::GetAttribute("data-test"),
        )),
    )]);

    let dom = tl::parse(
        r#"<div data-test="foo" class="bar"></div>"#,
        tl::ParserOptions::default(),
    )
    .unwrap();
    let starting_elements = HtmlContent::import(dom).unwrap();

    let mut result = pipeline
        .run_on(vec![rctree::Node::clone(&starting_elements)])
        .unwrap();

    assert_eq!(result.len(), 1);
    let first_result = result.pop().unwrap();
    assert_eq!(
        first_result.outer_html(),
        String::from(r#"<div class="bar" data-test="foo">foo</div>"#)
    );
}

#[test]
fn run_on_single_set_text_content_from_string_for_tag_with_multiple_children() {
    let pipeline = ElementProcessingPipeline::new(vec![ElementProcessingCommand::SetTextContent(
        ValueSource::StringValue("Other Content"),
    )]);

    let dom = tl::parse(
        r#"<div data-test="foo" class="bar">Some <em>special</em> Content. <!-- rightly so --></div>"#,
        tl::ParserOptions::default(),
    )
        .unwrap();
    let starting_elements = HtmlContent::import(dom).unwrap();

    let mut result = pipeline
        .run_on(vec![rctree::Node::clone(&starting_elements)])
        .unwrap();

    assert_eq!(result.len(), 1);
    let first_result = result.pop().unwrap();
    assert_eq!(
        first_result.outer_html(),
        String::from(r#"<div class="bar" data-test="foo">Other Content</div>"#)
    );
}

#[test]
fn run_on_single_add_text_content_from_string_for_tag() {
    let pipeline = ElementProcessingPipeline::new(vec![ElementProcessingCommand::AddTextContent(
        ValueSource::StringValue("Other Content"),
    )]);

    let dom = tl::parse(
        r#"<div data-test="foo" class="bar">Some Content</div>"#,
        tl::ParserOptions::default(),
    )
    .unwrap();
    let starting_elements = HtmlContent::import(dom).unwrap();

    let mut result = pipeline
        .run_on(vec![rctree::Node::clone(&starting_elements)])
        .unwrap();

    assert_eq!(result.len(), 1);
    let first_result = result.pop().unwrap();
    assert_eq!(
        first_result.outer_html(),
        String::from(r#"<div class="bar" data-test="foo">Some ContentOther Content</div>"#)
    );
}

#[test]
fn run_on_single_add_text_content_from_string_for_empty_tag() {
    let pipeline = ElementProcessingPipeline::new(vec![ElementProcessingCommand::AddTextContent(
        ValueSource::SubPipeline(StringValueCreatingPipeline::new(
            ElementSelectingCommand::UseElement,
            ValueExtractingCommand::GetAttribute("data-test"),
        )),
    )]);

    let dom = tl::parse(
        r#"<div data-test="foo" class="bar"></div>"#,
        tl::ParserOptions::default(),
    )
    .unwrap();
    let starting_elements = HtmlContent::import(dom).unwrap();

    let mut result = pipeline
        .run_on(vec![rctree::Node::clone(&starting_elements)])
        .unwrap();

    assert_eq!(result.len(), 1);
    let first_result = result.pop().unwrap();
    assert_eq!(
        first_result.outer_html(),
        String::from(r#"<div class="bar" data-test="foo">foo</div>"#)
    );
}

#[test]
fn run_on_single_add_text_content_from_attr_for_empty_tag() {
    let pipeline = ElementProcessingPipeline::new(vec![ElementProcessingCommand::AddTextContent(
        ValueSource::StringValue("Other Content"),
    )]);

    let dom = tl::parse(
        r#"<div data-test="foo" class="bar"></div>"#,
        tl::ParserOptions::default(),
    )
    .unwrap();
    let starting_elements = HtmlContent::import(dom).unwrap();

    let mut result = pipeline
        .run_on(vec![rctree::Node::clone(&starting_elements)])
        .unwrap();

    assert_eq!(result.len(), 1);
    let first_result = result.pop().unwrap();
    assert_eq!(
        first_result.outer_html(),
        String::from(r#"<div class="bar" data-test="foo">Other Content</div>"#)
    );
}

#[test]
fn run_on_single_add_text_content_from_string_for_tag_with_multiple_children() {
    let pipeline = ElementProcessingPipeline::new(vec![ElementProcessingCommand::AddTextContent(
        ValueSource::StringValue("Other Content"),
    )]);

    let dom = tl::parse(
        r#"<div data-test="foo" class="bar">Some <em>special</em> Content. <!-- rightly so --></div>"#,
        tl::ParserOptions::default(),
    )
        .unwrap();
    let starting_elements = HtmlContent::import(dom).unwrap();

    let mut result = pipeline
        .run_on(vec![rctree::Node::clone(&starting_elements)])
        .unwrap();

    assert_eq!(result.len(), 1);
    let first_result = result.pop().unwrap();
    assert_eq!(
        first_result.outer_html(),
        String::from(
            r#"<div class="bar" data-test="foo">Some <em>special</em> Content. <!-- rightly so -->Other Content</div>"#
        )
    );
}

#[test]
fn run_on_single_add_comment_from_string_for_tag() {
    let pipeline = ElementProcessingPipeline::new(vec![ElementProcessingCommand::AddComment(
        ValueSource::StringValue("Other Content"),
    )]);

    let dom = tl::parse(
        r#"<div data-test="foo" class="bar">Some Content</div>"#,
        tl::ParserOptions::default(),
    )
    .unwrap();
    let starting_elements = HtmlContent::import(dom).unwrap();

    let mut result = pipeline
        .run_on(vec![rctree::Node::clone(&starting_elements)])
        .unwrap();

    assert_eq!(result.len(), 1);
    let first_result = result.pop().unwrap();
    assert_eq!(
        first_result.outer_html(),
        String::from(
            r#"<div class="bar" data-test="foo">Some Content<!-- Other Content --></div>"#
        )
    );
}

#[test]
fn run_on_single_add_comment_from_string_for_empty_tag() {
    let pipeline = ElementProcessingPipeline::new(vec![ElementProcessingCommand::AddComment(
        ValueSource::StringValue("Other Content"),
    )]);

    let dom = tl::parse(
        r#"<div data-test="foo" class="bar"></div>"#,
        tl::ParserOptions::default(),
    )
    .unwrap();
    let starting_elements = HtmlContent::import(dom).unwrap();

    let mut result = pipeline
        .run_on(vec![rctree::Node::clone(&starting_elements)])
        .unwrap();

    assert_eq!(result.len(), 1);
    let first_result = result.pop().unwrap();
    assert_eq!(
        first_result.outer_html(),
        String::from(r#"<div class="bar" data-test="foo"><!-- Other Content --></div>"#)
    );
}

#[test]
fn run_on_single_add_comment_from_attr_for_empty_tag() {
    let pipeline = ElementProcessingPipeline::new(vec![ElementProcessingCommand::AddComment(
        ValueSource::SubPipeline(StringValueCreatingPipeline::new(
            ElementSelectingCommand::UseElement,
            ValueExtractingCommand::GetAttribute("data-test"),
        )),
    )]);

    let dom = tl::parse(
        r#"<div data-test="foo" class="bar"></div>"#,
        tl::ParserOptions::default(),
    )
    .unwrap();
    let starting_elements = HtmlContent::import(dom).unwrap();

    let mut result = pipeline
        .run_on(vec![rctree::Node::clone(&starting_elements)])
        .unwrap();

    assert_eq!(result.len(), 1);
    let first_result = result.pop().unwrap();
    assert_eq!(
        first_result.outer_html(),
        String::from(r#"<div class="bar" data-test="foo"><!-- foo --></div>"#)
    );
}

#[test]
fn run_on_single_add_comment_from_string_for_tag_with_multiple_children() {
    let pipeline = ElementProcessingPipeline::new(vec![ElementProcessingCommand::AddComment(
        ValueSource::StringValue("Other Content"),
    )]);

    let dom = tl::parse(
        r#"<div data-test="foo" class="bar"><!-- rightly so -->Some <em>special</em> Content.</div>"#,
        tl::ParserOptions::default(),
    )
        .unwrap();
    let starting_elements = HtmlContent::import(dom).unwrap();

    let mut result = pipeline
        .run_on(vec![rctree::Node::clone(&starting_elements)])
        .unwrap();

    assert_eq!(result.len(), 1);
    let first_result = result.pop().unwrap();
    assert_eq!(
        first_result.outer_html(),
        String::from(
            r#"<div class="bar" data-test="foo"><!-- rightly so -->Some <em>special</em> Content.<!-- Other Content --></div>"#
        )
    );
}

#[test]
fn run_on_single_for_each_on_ul() {
    let pipeline = ElementProcessingPipeline::new(vec![ElementProcessingCommand::ForEach(
        CssSelectorList::new(vec![CssSelectorPath::single(CssSelector::for_element(
            "li",
        ))]),
        ElementProcessingPipeline::new(vec![ElementProcessingCommand::SetAttribute(
            "data-test",
            ValueSource::StringValue("x"),
        )]),
    )]);

    let dom = tl::parse(
        r#"<ul><li>1</li><li>2</li></ul>"#,
        tl::ParserOptions::default(),
    )
    .unwrap();
    let starting_elements = HtmlContent::import(dom).unwrap();

    let mut result = pipeline
        .run_on(vec![rctree::Node::clone(&starting_elements)])
        .unwrap();

    assert_eq!(result.len(), 1);
    let first_result = result.pop().unwrap();
    assert_eq!(
        first_result.outer_html(),
        String::from(r#"<ul><li data-test="x">1</li><li data-test="x">2</li></ul>"#)
    );
}

#[test]
fn run_on_single_add_element_from_create_for_tag() {
    let pipeline = ElementProcessingPipeline::new(vec![ElementProcessingCommand::AddElement(
        ElementCreatingPipeline::new(ElementCreatingCommand::CreateElement("div"), None),
    )]);

    let dom = tl::parse(
        r#"<div data-test="foo" class="bar">Some Content</div>"#,
        tl::ParserOptions::default(),
    )
    .unwrap();
    let starting_elements = HtmlContent::import(dom).unwrap();

    let mut result = pipeline
        .run_on(vec![rctree::Node::clone(&starting_elements)])
        .unwrap();

    assert_eq!(result.len(), 1);
    let first_result = result.pop().unwrap();
    assert_eq!(
        first_result.outer_html(),
        String::from(r#"<div class="bar" data-test="foo">Some Content<div></div></div>"#)
    );
}

//noinspection DuplicatedCode
#[test]
fn run_on_single_replace_from_create() {
    let pipeline = ElementProcessingPipeline::new(vec![ElementProcessingCommand::Replace(
        CssSelectorList::new(vec![CssSelectorPath::single(CssSelector::for_class(
            "replace-me",
        ))]),
        ElementCreatingPipeline::new(ElementCreatingCommand::CreateElement("p"), None),
    )]);

    let dom = tl::parse(
        r#"<body><div class="replace-me">Some Content</div><div class="stay">This will be kept</div></body>"#,
        tl::ParserOptions::default(),
    )
        .unwrap();
    let starting_elements = HtmlContent::import(dom).unwrap();

    let mut result = pipeline
        .run_on(vec![rctree::Node::clone(&starting_elements)])
        .unwrap();

    assert_eq!(result.len(), 1);
    let first_result = result.pop().unwrap();
    assert_eq!(
        first_result.outer_html(),
        String::from(r#"<body><p></p><div class="stay">This will be kept</div></body>"#)
    );
}

//noinspection DuplicatedCode
#[test]
fn run_on_single_replace_using_from_file() {
    let pipeline = ElementProcessingPipeline::new(vec![ElementProcessingCommand::Replace(
        CssSelectorList::new(vec![CssSelectorPath::single(CssSelector::for_class(
            "replace-me",
        ))]),
        ElementCreatingPipeline::new(
            ElementCreatingCommand::FromFile("tests/single_div.html"),
            None,
        ),
    )]);

    let dom = tl::parse(
        r#"<body><div class="replace-me">Some Content</div><div class="stay">This will be kept</div></body>"#,
        tl::ParserOptions::default(),
    )
        .unwrap();
    let starting_elements = HtmlContent::import(dom).unwrap();

    let mut result = pipeline
        .run_on(vec![rctree::Node::clone(&starting_elements)])
        .unwrap();

    assert_eq!(result.len(), 1);
    let first_result = result.pop().unwrap();
    assert_eq!(
        first_result.outer_html(),
        String::from(
            r#"<body><div class="new">This is new</div><div class="stay">This will be kept</div></body>"#
        )
    );
}

//noinspection DuplicatedCode
#[test]
fn run_on_single_replace_using_from_replaced() {
    let pipeline = ElementProcessingPipeline::new(vec![ElementProcessingCommand::Replace(
        CssSelectorList::new(vec![CssSelectorPath::single(CssSelector::for_class(
            "replace-me",
        ))]),
        ElementCreatingPipeline::new(
            ElementCreatingCommand::FromReplaced(CssSelectorList::new(vec![
                CssSelectorPath::single(CssSelector::for_element("p")),
            ])),
            None,
        ),
    )]);

    let dom = tl::parse(
        r#"<body><div class="replace-me">Some <aside>mixed <p>Content</p> with multiple </aside><p>levels</p></div><div class="stay">This will be kept</div></body>"#,
        tl::ParserOptions::default(),
    )
        .unwrap();
    let starting_elements = HtmlContent::import(dom).unwrap();

    let mut result = pipeline
        .run_on(vec![rctree::Node::clone(&starting_elements)])
        .unwrap();

    assert_eq!(result.len(), 1);
    let first_result = result.pop().unwrap();
    assert_eq!(
        first_result.outer_html(),
        String::from(
            r#"<body><p>Content</p><p>levels</p><div class="stay">This will be kept</div></body>"#
        )
    );
}