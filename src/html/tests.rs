use crate::html::{HtmlContent, HtmlDocument, HtmlRenderable, HtmlTag};
use std::collections::BTreeMap;
use tl::HTMLVersion;

fn build_comment() -> rctree::Node<HtmlContent> {
    rctree::Node::<HtmlContent>::new(HtmlContent::Comment(String::from("Some Comment")))
}

fn build_text() -> rctree::Node<HtmlContent> {
    build_text_with_content("Some Text")
}

fn build_text_with_content(text: impl Into<String>) -> rctree::Node<HtmlContent> {
    rctree::Node::<HtmlContent>::new(HtmlContent::Text(text.into()))
}

fn build_tag() -> rctree::Node<HtmlContent> {
    rctree::Node::<HtmlContent>::new(HtmlContent::Tag(HtmlTag::of_name("div")))
}

fn build_tag_with_attr() -> rctree::Node<HtmlContent> {
    rctree::Node::<HtmlContent>::new(HtmlContent::Tag(HtmlTag {
        name: String::from("div"),
        attributes: BTreeMap::<String, String>::from([
            (String::from("class"), String::from("foo")),
            (String::from("data-bar"), String::from("value")),
        ]),
    }))
}

fn build_tag_with_comment() -> rctree::Node<HtmlContent> {
    let unit_of_test = build_tag();
    unit_of_test.append(build_comment());

    unit_of_test
}

fn build_tag_with_text() -> rctree::Node<HtmlContent> {
    let unit_of_test = build_tag();
    unit_of_test.append(build_text());

    unit_of_test
}

fn build_tag_with_complex_content() -> rctree::Node<HtmlContent> {
    let unit_of_test = build_tag();
    unit_of_test.append(build_text());
    unit_of_test.append(build_comment());

    let child_tag = build_tag_with_attr();
    child_tag.append(build_text_with_content("Other Text"));
    unit_of_test.append(child_tag);

    unit_of_test.append(build_text_with_content("Third Text"));

    unit_of_test
}

fn build_document() -> rctree::Node<HtmlContent> {
    let unit_of_tests = rctree::Node::<HtmlContent>::new(HtmlContent::Document(HtmlDocument {
        doctype: Some(HTMLVersion::HTML5),
    }));

    let html = rctree::Node::<HtmlContent>::new(HtmlContent::Tag(HtmlTag::of_name("html")));
    html.append(rctree::Node::<HtmlContent>::new(HtmlContent::Tag(
        HtmlTag::of_name("head"),
    )));

    let body = rctree::Node::<HtmlContent>::new(HtmlContent::Tag(HtmlTag::of_name("body")));
    body.append(build_text());

    html.append(body);
    unit_of_tests.append(html);

    unit_of_tests
}

#[test]
fn outer_html_of_comment_has_correct_syntax() {
    let unit_of_test = build_comment();
    let rendered_outer_html = unit_of_test.outer_html();

    assert_eq!(rendered_outer_html, String::from("<!-- Some Comment -->"));
}

#[test]
fn outer_html_of_text_is_string() {
    let unit_of_test = build_text();
    let rendered_outer_html = unit_of_test.outer_html();

    assert_eq!(rendered_outer_html, String::from("Some Text"));
}

#[test]
fn outer_html_of_childless_tag_is_tag_pair() {
    let unit_of_test = build_tag();
    let rendered_outer_html = unit_of_test.outer_html();

    assert_eq!(rendered_outer_html, String::from("<div></div>"));
}

#[test]
fn outer_html_of_tag_contains_attributes() {
    let unit_of_test = build_tag_with_attr();
    let rendered_outer_html = unit_of_test.outer_html();

    assert_eq!(
        rendered_outer_html,
        String::from(r#"<div class="foo" data-bar="value"></div>"#)
    );
}

#[test]
fn outer_html_of_tag_with_comment_has_comment() {
    let unit_of_test = build_tag_with_comment();
    let rendered_outer_html = unit_of_test.outer_html();

    assert_eq!(
        rendered_outer_html,
        String::from("<div><!-- Some Comment --></div>")
    );
}

#[test]
fn outer_html_of_tag_with_text_has_string() {
    let unit_of_test = build_tag_with_text();
    let rendered_outer_html = unit_of_test.outer_html();

    assert_eq!(rendered_outer_html, String::from("<div>Some Text</div>"));
}

#[test]
fn outer_html_of_complex_node_has_all() {
    let unit_of_test = build_tag_with_complex_content();
    let rendered_outer_html = unit_of_test.outer_html();

    assert_eq!(
        rendered_outer_html,
        String::from(
            r#"<div>Some Text<!-- Some Comment --><div class="foo" data-bar="value">Other Text</div>Third Text</div>"#
        )
    );
}

#[test]
fn outer_html_of_document_has_all() {
    let unit_of_test = build_document();
    let rendered = unit_of_test.outer_html();

    assert_eq!(
        rendered,
        "<!DOCTYPE html>\n<html><head></head><body>Some Text</body></html>"
    )
}

#[test]
fn inner_html_of_comment_is_empty() {
    let unit_of_test = build_comment();
    let rendered_inner_html = unit_of_test.inner_html();

    assert_eq!(rendered_inner_html, String::new());
}

#[test]
fn inner_html_of_text_is_string() {
    let unit_of_test = build_text();
    let rendered_inner_html = unit_of_test.inner_html();

    assert_eq!(rendered_inner_html, String::from("Some Text"));
}

#[test]
fn inner_html_of_childless_tag_is_empty() {
    let unit_of_test = build_tag();
    let rendered_inner_html = unit_of_test.inner_html();

    assert_eq!(rendered_inner_html, String::new());
}

#[test]
fn inner_html_of_tag_containing_attributes_is_empty() {
    let unit_of_test = build_tag_with_attr();
    let rendered_inner_html = unit_of_test.inner_html();

    assert_eq!(rendered_inner_html, String::new());
}

#[test]
fn inner_html_of_tag_with_comment_has_comment() {
    let unit_of_test = build_tag_with_comment();
    let rendered_inner_html = unit_of_test.inner_html();

    assert_eq!(rendered_inner_html, String::from("<!-- Some Comment -->"));
}

#[test]
fn inner_html_of_tag_with_text_has_string() {
    let unit_of_test = build_tag_with_text();
    let rendered_inner_html = unit_of_test.inner_html();

    assert_eq!(rendered_inner_html, String::from("Some Text"));
}

#[test]
fn inner_html_of_complex_node_has_all() {
    let unit_of_test = build_tag_with_complex_content();
    let rendered_inner_html = unit_of_test.inner_html();

    assert_eq!(
        rendered_inner_html,
        String::from(
            r#"Some Text<!-- Some Comment --><div class="foo" data-bar="value">Other Text</div>Third Text"#
        )
    );
}

#[test]
fn inner_html_of_document_has_all() {
    let unit_of_test = build_document();
    let rendered = unit_of_test.inner_html();

    assert_eq!(
        rendered,
        "<!DOCTYPE html>\n<html><head></head><body>Some Text</body></html>"
    )
}

#[test]
fn text_content_of_comment_is_empty() {
    let unit_of_test = build_comment();
    let rendered_text_content = unit_of_test.text_content();

    assert_eq!(rendered_text_content, String::new());
}

#[test]
fn text_content_of_text_is_string() {
    let unit_of_test = build_text();
    let rendered_text_content = unit_of_test.text_content();

    assert_eq!(rendered_text_content, String::from("Some Text"));
}

#[test]
fn text_content_of_childless_tag_is_empty() {
    let unit_of_test = build_tag();
    let rendered_text_content = unit_of_test.text_content();

    assert_eq!(rendered_text_content, String::new());
}

#[test]
fn text_content_of_tag_containing_attributes_is_empty() {
    let unit_of_test = build_tag_with_attr();
    let rendered_text_content = unit_of_test.text_content();

    assert_eq!(rendered_text_content, String::new());
}

#[test]
fn text_content_of_tag_with_comment_is_empty() {
    let unit_of_test = build_tag_with_comment();
    let rendered_text_content = unit_of_test.text_content();

    assert_eq!(rendered_text_content, String::new());
}

#[test]
fn text_content_of_tag_with_text_is_string() {
    let unit_of_test = build_tag_with_text();
    let rendered_text_content = unit_of_test.text_content();

    assert_eq!(rendered_text_content, String::from("Some Text"));
}

#[test]
fn text_content_of_complex_node_is_set() {
    let unit_of_test = build_tag_with_complex_content();
    let rendered_text_content = unit_of_test.text_content();

    assert_eq!(
        rendered_text_content,
        String::from("Some Text Other Text Third Text")
    );
}

#[test]
fn text_content_of_document_has_all() {
    let unit_of_test = build_document();
    let rendered = unit_of_test.text_content();

    assert_eq!(rendered, "Some Text");
}

#[test]
fn convert_single_vdom_works() {
    let dom = tl::parse(
        "<html><head></head><!-- nothing here --><body class=\"simple\" data-test=\"Ala ma kota\">Hello World</body></html>",
        tl::ParserOptions::default(),
    )
        .unwrap();
    let converted = HtmlContent::import(dom).unwrap();

    let expected = rctree::Node::<HtmlContent>::new(HtmlContent::Tag(HtmlTag::of_name("html")));
    expected.append(rctree::Node::<HtmlContent>::new(HtmlContent::Tag(
        HtmlTag::of_name("head"),
    )));
    expected.append(rctree::Node::<HtmlContent>::new(HtmlContent::Comment(
        String::from("nothing here"),
    )));

    let body = rctree::Node::<HtmlContent>::new(HtmlContent::Tag(HtmlTag {
        name: String::from("body"),
        attributes: BTreeMap::<String, String>::from([
            (String::from("class"), String::from("simple")),
            (String::from("data-test"), String::from("Ala ma kota")),
        ]),
    }));
    body.append(rctree::Node::<HtmlContent>::new(HtmlContent::Text(
        String::from("Hello World"),
    )));
    expected.append(body);

    assert_eq!(converted.outer_html(), expected.outer_html());
}

#[test]
fn convert_empty_comments_works() {
    let dom = tl::parse(
        "<body>Hello <!-- -->World</body>",
        tl::ParserOptions::default(),
    )
    .unwrap();

    let converted = HtmlContent::import(dom).unwrap();

    let body = rctree::Node::<HtmlContent>::new(HtmlContent::Tag(HtmlTag::of_name("body")));
    body.append(rctree::Node::<HtmlContent>::new(HtmlContent::Text(
        String::from("Hello "),
    )));
    body.append(rctree::Node::<HtmlContent>::new(HtmlContent::Comment(
        String::new(),
    )));
    body.append(rctree::Node::<HtmlContent>::new(HtmlContent::Text(
        String::from("World"),
    )));

    assert_eq!(converted.outer_html(), body.outer_html());
}

#[test]
fn convert_vdom_keeps_doctype5_if_present() {
    let dom = tl::parse(
        "<!DOCTYPE html>\n<html>Hello World</html>",
        tl::ParserOptions::default(),
    )
    .unwrap();

    let converted = HtmlContent::import(dom).unwrap();
    assert_eq!(
        converted.outer_html(),
        "<!DOCTYPE html>\n<html>Hello World</html>"
    );
}

#[test]
#[ignore] // won't work as long as underlying tl lib does not handle 4.01 DOCTYPE correctly
fn convert_vdom_keeps_doctype4_if_present() {
    let dom = tl::parse(
        r#"<!DOCTYPE HTML PUBLIC "-//W3C//DTD HTML 4.01//EN" "http://www.w3.org/TR/html4/strict.dtd">
<html>Hello World</html>"#,
        tl::ParserOptions::default(),
    )
    .unwrap();

    let converted = HtmlContent::import(dom).unwrap();
    assert_eq!(
        converted.outer_html(),
        r#"<!DOCTYPE HTML PUBLIC "-//W3C//DTD HTML 4.01//EN" "http://www.w3.org/TR/html4/strict.dtd">
<html>Hello World</html>"#
    );
}

#[test]
fn convert_vdom_no_doctype_if_none_present() {
    let dom = tl::parse("<html>Hello World</html>", tl::ParserOptions::default()).unwrap();

    let converted = HtmlContent::import(dom).unwrap();
    assert_eq!(converted.outer_html(), "<html>Hello World</html>");
}

#[test]
fn convert_vdom_html_tag_builds_document() {
    let dom = tl::parse("<html>Hello World</html>", tl::ParserOptions::default()).unwrap();

    let converted = HtmlContent::import(dom).unwrap();

    let content = converted.borrow();
    assert!(matches!(*content, HtmlContent::Document(_)));
}

#[test]
fn convert_vdom_other_tag_builds_tag() {
    let dom = tl::parse("<body>Hello World</body>", tl::ParserOptions::default()).unwrap();

    let converted = HtmlContent::import(dom).unwrap();
    let content = converted.borrow();
    assert!(matches!(*content, HtmlContent::Tag(_)));
}
