use crate::element_creating::{ElementCreatingCommand, ElementCreatingPipeline};
use crate::html::HtmlRenderable;
use crate::string_creating::{ElementSelectingCommand, ValueExtractingCommand};
use crate::{
    element_processing::{command::ElementProcessingCommand, pipeline::ElementProcessingPipeline},
    load_inline_html, CssSelector, CssSelectorList, CssSelectorPath, CssSelectorStep,
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
fn extract_command() {
    let command =
        ElementProcessingCommand::ExtractElement(CssSelectorList::new(vec![CssSelectorPath::new(
            CssSelector::for_element("h1"),
            vec![CssSelectorStep::adjacent_sibling(CssSelector::for_element(
                "p",
            ))],
        )]));

    let root = load_inline_html(TEST_HTML_DOCUMENT);

    let mut result = command.execute(&vec![rctree::Node::clone(&root)]).unwrap();

    assert_eq!(result.len(), 1);
    let first_result = result.pop().unwrap();
    assert_eq!(
        first_result.outer_html(),
        String::from(r#"<p id="first-para">Some <em class="fancy">first</em> text</p>"#)
    );
}

#[test]
fn remove_command() {
    let command =
        ElementProcessingCommand::RemoveElement(CssSelectorList::new(vec![CssSelectorPath::new(
            CssSelector::for_element("h1"),
            vec![CssSelectorStep::adjacent_sibling(CssSelector::for_element(
                "p",
            ))],
        )]));

    let root = load_inline_html(TEST_HTML_DOCUMENT);
    let mut result = command.execute(&vec![rctree::Node::clone(&root)]).unwrap();

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
fn clear_attribute() {
    let command = ElementProcessingCommand::ClearAttribute("data-test");

    let root = load_inline_html(r#"<div data-test="foo" class="bar">Some Content</div>"#);
    let mut result = command.execute(&vec![rctree::Node::clone(&root)]).unwrap();

    assert_eq!(result.len(), 1);
    let first_result = result.pop().unwrap();
    assert_eq!(
        first_result.outer_html(),
        String::from(r#"<div class="bar">Some Content</div>"#)
    );
}

#[test]
fn clear_content() {
    let command = ElementProcessingCommand::ClearContent;

    let root = load_inline_html(r#"<div data-test="foo" class="bar">Some Content</div>"#);

    let mut result = command.execute(&vec![rctree::Node::clone(&root)]).unwrap();

    assert_eq!(result.len(), 1);
    let first_result = result.pop().unwrap();
    assert_eq!(
        first_result.outer_html(),
        String::from(r#"<div class="bar" data-test="foo"></div>"#)
    );
}

#[test]
fn set_attribute_from_string_over_existing_attr() {
    let command =
        ElementProcessingCommand::SetAttribute("data-test", ValueSource::StringValue("some text"));

    let root = load_inline_html(r#"<div data-test="foo" class="bar">Some Content</div>"#);

    let mut result = command.execute(&vec![rctree::Node::clone(&root)]).unwrap();

    assert_eq!(result.len(), 1);
    let first_result = result.pop().unwrap();
    assert_eq!(
        first_result.outer_html(),
        String::from(r#"<div class="bar" data-test="some text">Some Content</div>"#)
    );
}

#[test]
fn set_attribute_from_string_as_new_attr() {
    let command =
        ElementProcessingCommand::SetAttribute("data-fubar", ValueSource::StringValue("some text"));

    let root = load_inline_html(r#"<div data-test="foo" class="bar">Some Content</div>"#);

    let mut result = command.execute(&vec![rctree::Node::clone(&root)]).unwrap();

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
fn set_attribute_from_other_attr_as_new_attr() {
    let command = ElementProcessingCommand::SetAttribute(
        "data-fubar",
        ValueSource::SubPipeline(StringValueCreatingPipeline::new(
            ElementSelectingCommand::UseElement,
            ValueExtractingCommand::GetAttribute("data-test"),
        )),
    );

    let root = load_inline_html(r#"<div data-test="foo" class="bar">Some Content</div>"#);

    let mut result = command.execute(&vec![rctree::Node::clone(&root)]).unwrap();

    assert_eq!(result.len(), 1);
    let first_result = result.pop().unwrap();
    assert_eq!(
        first_result.outer_html(),
        String::from(r#"<div class="bar" data-fubar="foo" data-test="foo">Some Content</div>"#)
    );
}

#[test]
fn set_text_content_from_string_for_tag() {
    let command =
        ElementProcessingCommand::SetTextContent(ValueSource::StringValue("Other Content"));

    let root = load_inline_html(r#"<div data-test="foo" class="bar">Some Content</div>"#);

    let mut result = command.execute(&vec![rctree::Node::clone(&root)]).unwrap();

    assert_eq!(result.len(), 1);
    let first_result = result.pop().unwrap();
    assert_eq!(
        first_result.outer_html(),
        String::from(r#"<div class="bar" data-test="foo">Other Content</div>"#)
    );
}

#[test]
fn set_text_content_from_string_for_empty_tag() {
    let command =
        ElementProcessingCommand::SetTextContent(ValueSource::StringValue("Other Content"));

    let root = load_inline_html(r#"<div data-test="foo" class="bar"></div>"#);

    let mut result = command.execute(&vec![rctree::Node::clone(&root)]).unwrap();

    assert_eq!(result.len(), 1);
    let first_result = result.pop().unwrap();
    assert_eq!(
        first_result.outer_html(),
        String::from(r#"<div class="bar" data-test="foo">Other Content</div>"#)
    );
}

#[test]
fn set_text_content_from_attr_for_empty_tag() {
    let command = ElementProcessingCommand::SetTextContent(ValueSource::SubPipeline(
        StringValueCreatingPipeline::new(
            ElementSelectingCommand::UseElement,
            ValueExtractingCommand::GetAttribute("data-test"),
        ),
    ));

    let root = load_inline_html(r#"<div data-test="foo" class="bar"></div>"#);

    let mut result = command.execute(&vec![rctree::Node::clone(&root)]).unwrap();

    assert_eq!(result.len(), 1);
    let first_result = result.pop().unwrap();
    assert_eq!(
        first_result.outer_html(),
        String::from(r#"<div class="bar" data-test="foo">foo</div>"#)
    );
}

#[test]
fn set_text_content_from_string_for_tag_with_multiple_children() {
    let command =
        ElementProcessingCommand::SetTextContent(ValueSource::StringValue("Other Content"));

    let root = load_inline_html(
        r#"<div data-test="foo" class="bar">Some <em>special</em> Content. <!-- rightly so --></div>"#,
    );

    let mut result = command.execute(&vec![rctree::Node::clone(&root)]).unwrap();

    assert_eq!(result.len(), 1);
    let first_result = result.pop().unwrap();
    assert_eq!(
        first_result.outer_html(),
        String::from(r#"<div class="bar" data-test="foo">Other Content</div>"#)
    );
}

#[test]
fn append_text_content_from_string_for_tag() {
    let command =
        ElementProcessingCommand::AppendTextContent(ValueSource::StringValue("Other Content"));

    let root = load_inline_html(r#"<div data-test="foo" class="bar">Some Content</div>"#);

    let mut result = command.execute(&vec![rctree::Node::clone(&root)]).unwrap();

    assert_eq!(result.len(), 1);
    let first_result = result.pop().unwrap();
    assert_eq!(
        first_result.outer_html(),
        String::from(r#"<div class="bar" data-test="foo">Some ContentOther Content</div>"#)
    );
}

#[test]
fn append_text_content_from_string_for_empty_tag() {
    let command = ElementProcessingCommand::AppendTextContent(ValueSource::SubPipeline(
        StringValueCreatingPipeline::new(
            ElementSelectingCommand::UseElement,
            ValueExtractingCommand::GetAttribute("data-test"),
        ),
    ));

    let root = load_inline_html(r#"<div data-test="foo" class="bar"></div>"#);

    let mut result = command.execute(&vec![rctree::Node::clone(&root)]).unwrap();

    assert_eq!(result.len(), 1);
    let first_result = result.pop().unwrap();
    assert_eq!(
        first_result.outer_html(),
        String::from(r#"<div class="bar" data-test="foo">foo</div>"#)
    );
}

#[test]
fn append_text_content_from_attr_for_empty_tag() {
    let command =
        ElementProcessingCommand::AppendTextContent(ValueSource::StringValue("Other Content"));

    let root = load_inline_html(r#"<div data-test="foo" class="bar"></div>"#);

    let mut result = command.execute(&vec![rctree::Node::clone(&root)]).unwrap();

    assert_eq!(result.len(), 1);
    let first_result = result.pop().unwrap();
    assert_eq!(
        first_result.outer_html(),
        String::from(r#"<div class="bar" data-test="foo">Other Content</div>"#)
    );
}

#[test]
fn append_text_content_from_string_for_tag_with_multiple_children() {
    let command =
        ElementProcessingCommand::AppendTextContent(ValueSource::StringValue("Other Content"));

    let root = load_inline_html(
        r#"<div data-test="foo" class="bar">Some <em>special</em> Content. <!-- rightly so --></div>"#,
    );

    let mut result = command.execute(&vec![rctree::Node::clone(&root)]).unwrap();

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
fn append_comment_from_string_for_tag() {
    let command =
        ElementProcessingCommand::AppendComment(ValueSource::StringValue("Other Content"));

    let root = load_inline_html(r#"<div data-test="foo" class="bar">Some Content</div>"#);

    let mut result = command.execute(&vec![rctree::Node::clone(&root)]).unwrap();

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
fn append_comment_from_string_for_empty_tag() {
    let command =
        ElementProcessingCommand::AppendComment(ValueSource::StringValue("Other Content"));

    let root = load_inline_html(r#"<div data-test="foo" class="bar"></div>"#);

    let mut result = command.execute(&vec![rctree::Node::clone(&root)]).unwrap();

    assert_eq!(result.len(), 1);
    let first_result = result.pop().unwrap();
    assert_eq!(
        first_result.outer_html(),
        String::from(r#"<div class="bar" data-test="foo"><!-- Other Content --></div>"#)
    );
}

#[test]
fn append_comment_from_attr_for_empty_tag() {
    let command = ElementProcessingCommand::AppendComment(ValueSource::SubPipeline(
        StringValueCreatingPipeline::new(
            ElementSelectingCommand::UseElement,
            ValueExtractingCommand::GetAttribute("data-test"),
        ),
    ));

    let root = load_inline_html(r#"<div data-test="foo" class="bar"></div>"#);

    let mut result = command.execute(&vec![rctree::Node::clone(&root)]).unwrap();

    assert_eq!(result.len(), 1);
    let first_result = result.pop().unwrap();
    assert_eq!(
        first_result.outer_html(),
        String::from(r#"<div class="bar" data-test="foo"><!-- foo --></div>"#)
    );
}

#[test]
fn append_comment_from_string_for_tag_with_multiple_children() {
    let command =
        ElementProcessingCommand::AppendComment(ValueSource::StringValue("Other Content"));

    let root = load_inline_html(
        r#"<div data-test="foo" class="bar"><!-- rightly so -->Some <em>special</em> Content.</div>"#,
    );

    let mut result = command.execute(&vec![rctree::Node::clone(&root)]).unwrap();

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
fn append_element_from_create_for_tag() {
    let command = ElementProcessingCommand::AppendElement(ElementCreatingPipeline::new(
        ElementCreatingCommand::CreateElement("div"),
        None,
    ));

    let root = load_inline_html(r#"<div data-test="foo" class="bar">Some Content</div>"#);

    let mut result = command.execute(&vec![rctree::Node::clone(&root)]).unwrap();

    assert_eq!(result.len(), 1);
    let first_result = result.pop().unwrap();
    assert_eq!(
        first_result.outer_html(),
        String::from(r#"<div class="bar" data-test="foo">Some Content<div></div></div>"#)
    );
}

#[test]
fn for_each_on_ul() {
    let command = ElementProcessingCommand::ForEach(
        CssSelectorList::new(vec![CssSelectorPath::single(CssSelector::for_element(
            "li",
        ))]),
        ElementProcessingPipeline::new(vec![ElementProcessingCommand::SetAttribute(
            "data-test",
            ValueSource::StringValue("x"),
        )]),
    );

    let root = load_inline_html(r#"<ul><li>1</li><li>2</li></ul>"#);

    let mut result = command.execute(&vec![rctree::Node::clone(&root)]).unwrap();

    assert_eq!(result.len(), 1);
    let first_result = result.pop().unwrap();
    assert_eq!(
        first_result.outer_html(),
        String::from(r#"<ul><li data-test="x">1</li><li data-test="x">2</li></ul>"#)
    );
}

//noinspection DuplicatedCode
#[test]
fn replace_element_from_create() {
    let command = ElementProcessingCommand::ReplaceElement(
        CssSelectorList::new(vec![CssSelectorPath::single(CssSelector::for_class(
            "replace-me",
        ))]),
        ElementCreatingPipeline::new(ElementCreatingCommand::CreateElement("p"), None),
    );

    let root = load_inline_html(
        r#"<body><div class="replace-me">Some Content</div><div class="stay">This will be kept</div></body>"#,
    );

    let mut result = command.execute(&vec![rctree::Node::clone(&root)]).unwrap();

    assert_eq!(result.len(), 1);
    let first_result = result.pop().unwrap();
    assert_eq!(
        first_result.outer_html(),
        String::from(r#"<body><p></p><div class="stay">This will be kept</div></body>"#)
    );
}

//noinspection DuplicatedCode
#[test]
fn replace_element_using_load_file() {
    let command = ElementProcessingCommand::ReplaceElement(
        CssSelectorList::new(vec![CssSelectorPath::single(CssSelector::for_class(
            "replace-me",
        ))]),
        ElementCreatingPipeline::new(
            ElementCreatingCommand::FromFile("tests/single_div.html"),
            None,
        ),
    );

    let root = load_inline_html(
        r#"<body><div class="replace-me">Some Content</div><div class="stay">This will be kept</div></body>"#,
    );

    let mut result = command.execute(&vec![rctree::Node::clone(&root)]).unwrap();

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
fn replace_element_using_query_replaced() {
    let command = ElementProcessingCommand::ReplaceElement(
        CssSelectorList::new(vec![CssSelectorPath::single(CssSelector::for_class(
            "replace-me",
        ))]),
        ElementCreatingPipeline::new(
            ElementCreatingCommand::FromReplaced(CssSelectorList::new(vec![
                CssSelectorPath::single(CssSelector::for_element("p")),
            ])),
            None,
        ),
    );

    let root = load_inline_html(
        r#"<body><div class="replace-me">Some <aside>mixed <p>Content</p> with multiple </aside><p>levels</p></div><div class="stay">This will be kept</div></body>"#,
    );

    let mut result = command.execute(&vec![rctree::Node::clone(&root)]).unwrap();

    assert_eq!(result.len(), 1);
    let first_result = result.pop().unwrap();
    assert_eq!(
        first_result.outer_html(),
        String::from(
            r#"<body><p>Content</p><p>levels</p><div class="stay">This will be kept</div></body>"#
        )
    );
}

#[test]
fn prepend_text_content_from_string_for_tag() {
    let command =
        ElementProcessingCommand::PrependTextContent(ValueSource::StringValue("Other Content"));

    let root = load_inline_html(r#"<div data-test="foo" class="bar">Some Content</div>"#);

    let mut result = command.execute(&vec![rctree::Node::clone(&root)]).unwrap();

    assert_eq!(result.len(), 1);
    let first_result = result.pop().unwrap();
    assert_eq!(
        first_result.outer_html(),
        String::from(r#"<div class="bar" data-test="foo">Other ContentSome Content</div>"#)
    );
}

#[test]
fn prepend_text_content_from_string_for_empty_tag() {
    let command = ElementProcessingCommand::PrependTextContent(ValueSource::SubPipeline(
        StringValueCreatingPipeline::new(
            ElementSelectingCommand::UseElement,
            ValueExtractingCommand::GetAttribute("data-test"),
        ),
    ));

    let root = load_inline_html(r#"<div data-test="foo" class="bar"></div>"#);

    let mut result = command.execute(&vec![rctree::Node::clone(&root)]).unwrap();

    assert_eq!(result.len(), 1);
    let first_result = result.pop().unwrap();
    assert_eq!(
        first_result.outer_html(),
        String::from(r#"<div class="bar" data-test="foo">foo</div>"#)
    );
}

#[test]
fn prepend_text_content_from_attr_for_empty_tag() {
    let command =
        ElementProcessingCommand::PrependTextContent(ValueSource::StringValue("Other Content"));

    let root = load_inline_html(r#"<div data-test="foo" class="bar"></div>"#);

    let mut result = command.execute(&vec![rctree::Node::clone(&root)]).unwrap();

    assert_eq!(result.len(), 1);
    let first_result = result.pop().unwrap();
    assert_eq!(
        first_result.outer_html(),
        String::from(r#"<div class="bar" data-test="foo">Other Content</div>"#)
    );
}

#[test]
fn prepend_text_content_from_string_for_tag_with_multiple_children() {
    let command =
        ElementProcessingCommand::PrependTextContent(ValueSource::StringValue("Other Content"));

    let root = load_inline_html(
        r#"<div data-test="foo" class="bar">Some <em>special</em> Content. <!-- rightly so --></div>"#,
    );

    let mut result = command.execute(&vec![rctree::Node::clone(&root)]).unwrap();

    assert_eq!(result.len(), 1);
    let first_result = result.pop().unwrap();
    assert_eq!(
        first_result.outer_html(),
        String::from(
            r#"<div class="bar" data-test="foo">Other ContentSome <em>special</em> Content. <!-- rightly so --></div>"#
        )
    );
}

#[test]
fn prepend_comment_from_string_for_tag() {
    let command =
        ElementProcessingCommand::PrependComment(ValueSource::StringValue("Other Content"));

    let root = load_inline_html(r#"<div data-test="foo" class="bar">Some Content</div>"#);

    let mut result = command.execute(&vec![rctree::Node::clone(&root)]).unwrap();

    assert_eq!(result.len(), 1);
    let first_result = result.pop().unwrap();
    assert_eq!(
        first_result.outer_html(),
        String::from(
            r#"<div class="bar" data-test="foo"><!-- Other Content -->Some Content</div>"#
        )
    );
}

#[test]
fn prepend_comment_from_string_for_empty_tag() {
    let command =
        ElementProcessingCommand::PrependComment(ValueSource::StringValue("Other Content"));

    let root = load_inline_html(r#"<div data-test="foo" class="bar"></div>"#);

    let mut result = command.execute(&vec![rctree::Node::clone(&root)]).unwrap();

    assert_eq!(result.len(), 1);
    let first_result = result.pop().unwrap();
    assert_eq!(
        first_result.outer_html(),
        String::from(r#"<div class="bar" data-test="foo"><!-- Other Content --></div>"#)
    );
}

#[test]
fn prepend_comment_from_attr_for_empty_tag() {
    let command = ElementProcessingCommand::PrependComment(ValueSource::SubPipeline(
        StringValueCreatingPipeline::new(
            ElementSelectingCommand::UseElement,
            ValueExtractingCommand::GetAttribute("data-test"),
        ),
    ));

    let root = load_inline_html(r#"<div data-test="foo" class="bar"></div>"#);

    let mut result = command.execute(&vec![rctree::Node::clone(&root)]).unwrap();

    assert_eq!(result.len(), 1);
    let first_result = result.pop().unwrap();
    assert_eq!(
        first_result.outer_html(),
        String::from(r#"<div class="bar" data-test="foo"><!-- foo --></div>"#)
    );
}

#[test]
fn prepend_comment_from_string_for_tag_with_multiple_children() {
    let command =
        ElementProcessingCommand::PrependComment(ValueSource::StringValue("Other Content"));

    let root = load_inline_html(
        r#"<div data-test="foo" class="bar"><!-- rightly so -->Some <em>special</em> Content.</div>"#,
    );

    let mut result = command.execute(&vec![rctree::Node::clone(&root)]).unwrap();

    assert_eq!(result.len(), 1);
    let first_result = result.pop().unwrap();
    assert_eq!(
        first_result.outer_html(),
        String::from(
            r#"<div class="bar" data-test="foo"><!-- Other Content --><!-- rightly so -->Some <em>special</em> Content.</div>"#
        )
    );
}

#[test]
fn prepend_element_from_create_for_tag() {
    let command = ElementProcessingCommand::PrependElement(ElementCreatingPipeline::new(
        ElementCreatingCommand::CreateElement("div"),
        None,
    ));

    let root = load_inline_html(r#"<div data-test="foo" class="bar">Some Content</div>"#);

    let mut result = command.execute(&vec![rctree::Node::clone(&root)]).unwrap();

    assert_eq!(result.len(), 1);
    let first_result = result.pop().unwrap();
    assert_eq!(
        first_result.outer_html(),
        String::from(r#"<div class="bar" data-test="foo"><div></div>Some Content</div>"#)
    );
}
