use crate::HtmlIndex;
use std::collections::HashSet;

#[test]
fn fill_simplest_html() {
    let dom = tl::parse(
        "<html><head></head><body></body></html>",
        tl::ParserOptions::default(),
    )
    .unwrap();
    let index = HtmlIndex::load(dom);

    assert_eq!(index.inner.len(), 3);
    assert_eq!(
        index
            .dom
            .borrow()
            .children()
            .iter()
            .filter_map(|n| index.relations_of(n))
            .collect::<Vec<_>>()
            .len(),
        1
    );
    assert_eq!(
        index
            .relations_of(index.dom.borrow().children().first().unwrap())
            .unwrap()
            .children
            .len(),
        2
    );
}

#[test]
fn fill_medium_html() {
    let dom = tl::parse(
        "<html><head></head><body id=\"element-under-test\"><header><h1>Hallo</h1></header><main><p>Ups <em>I'm sorry</em></p><img src=\"\"></main><footer></footer><nav><ul><li>1</li><li>2</li></ul></nav></body></html>",
        tl::ParserOptions::default(),
    )
        .unwrap();
    let index = HtmlIndex::load(dom);

    let body_handle = index
        .dom
        .borrow()
        .get_element_by_id("element-under-test")
        .unwrap();
    let body_index = index.relations_of(&body_handle).unwrap();

    assert_eq!(index.inner.len(), 19);
    assert_eq!(body_index.children.len(), 4);
    assert_eq!(body_index.descendents.len(), 16);
    assert_eq!(
        body_index.children,
        //order is not preserved
        HashSet::from_iter(
            body_handle
                .get(index.dom.borrow().parser())
                .unwrap()
                .children()
                .unwrap()
                .top()
                .iter()
                .map(|n| n.clone())
        )
    );
}

#[test]
fn fill_medium_html_siblings_of_main() {
    let dom = tl::parse(
        "<html><head></head><body><header><h1>Hallo</h1></header><main id=\"element-under-test\"><p>Ups <em>I'm sorry</em></p><img src=\"\"></main><footer id=\"sibling\"></footer><nav><ul><li>1</li><li>2</li></ul></nav></body></html>",
        tl::ParserOptions::default(),
    )
        .unwrap();
    let index = HtmlIndex::load(dom);

    let main_handle = index
        .dom
        .borrow()
        .get_element_by_id("element-under-test")
        .unwrap();
    let main_index = index.relations_of(&main_handle).unwrap();

    let footer_handle = index.dom.borrow().get_element_by_id("sibling").unwrap();

    assert_eq!(main_index.siblings.len(), 2);
    assert_eq!(main_index.direct_sibling, Some(footer_handle));
}

#[test]
fn fill_medium_html_siblings_of_header() {
    let dom = tl::parse(
        "<html><head></head><body><header id=\"element-under-test\"><h1>Hallo</h1></header><main id=\"sibling\"><p>Ups <em>I'm sorry</em></p><img src=\"\"></main><footer></footer><nav><ul><li>1</li><li>2</li></ul></nav></body></html>",
        tl::ParserOptions::default(),
    )
        .unwrap();
    let index = HtmlIndex::load(dom);

    let header_handle = index
        .dom
        .borrow()
        .get_element_by_id("element-under-test")
        .unwrap();
    let header_index = index.relations_of(&header_handle).unwrap();

    let main_handle = index.dom.borrow().get_element_by_id("sibling").unwrap();

    assert_eq!(header_index.siblings.len(), 3);
    assert_eq!(header_index.direct_sibling, Some(main_handle));
}
