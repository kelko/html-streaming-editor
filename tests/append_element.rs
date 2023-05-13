use html_streaming_editor::*;

const HTML_INPUT: &str = r#"<html>
    <head></head>
    <body>
        <h1>Title</h1>
        <p id="first-para">Some first text</p>
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
fn append_simple_div_to_first_p_content() -> Result<(), StreamingEditorError> {
    let command = "EXTRACT-ELEMENT{#first-para} | APPEND-ELEMENT{ CREATE-ELEMENT{div} }";

    let mut input = Box::new(HTML_INPUT.as_bytes());
    let hse = HtmlStreamingEditor::new(&mut input);

    let result = hse.run(command)?;
    let result_string = result
        .iter()
        .map(|n| n.outer_html())
        .collect::<Vec<_>>()
        .join("");

    assert_eq!(
        result_string,
        String::from(r#"<p id="first-para">Some first text<div></div></p>"#)
    );

    Ok(())
}

#[test]
fn append_two_divs_to_first_p_content() -> Result<(), StreamingEditorError> {
    let command = "EXTRACT-ELEMENT{#first-para} | APPEND-ELEMENT{ CREATE-ELEMENT{div} } | APPEND-ELEMENT{ CREATE-ELEMENT{div} }";

    let mut input = Box::new(HTML_INPUT.as_bytes());
    let hse = HtmlStreamingEditor::new(&mut input);

    let result = hse.run(command)?;
    let result_string = result
        .iter()
        .map(|n| n.outer_html())
        .collect::<Vec<_>>()
        .join("");

    assert_eq!(
        result_string,
        String::from(r#"<p id="first-para">Some first text<div></div><div></div></p>"#)
    );

    Ok(())
}

#[test]
fn append_div_with_attr_to_first_p_content() -> Result<(), StreamingEditorError> {
    let command =
        "EXTRACT-ELEMENT{#first-para} | APPEND-ELEMENT{ CREATE-ELEMENT{div} | SET-ATTR{id ↤ 'new'} }";

    let mut input = Box::new(HTML_INPUT.as_bytes());
    let hse = HtmlStreamingEditor::new(&mut input);

    let result = hse.run(command)?;
    let result_string = result
        .iter()
        .map(|n| n.outer_html())
        .collect::<Vec<_>>()
        .join("");

    assert_eq!(
        result_string,
        String::from(r#"<p id="first-para">Some first text<div id="new"></div></p>"#)
    );

    Ok(())
}

#[test]
fn copy_title_to_meta_tag() -> Result<(), StreamingEditorError> {
    let command = "FOR-EACH{head ↦ APPEND-ELEMENT{ ↤ CREATE-ELEMENT{meta} | SET-ATTR{name ↤ 'title' } } | FOR-EACH{meta[name='title'] ↦ SET-ATTR{content ↤ QUERY-PARENT{title} | GET-TEXT-CONTENT } } }";

    let mut input = Box::new(
        r#"<html>
    <head>
        <title>This is the title</title>
    </head>
    <body>
        <h1>Title</h1>
    </body>
</html>"#
            .as_bytes(),
    );
    let hse = HtmlStreamingEditor::new(&mut input);

    let result = hse.run(command)?;
    let result_string = result
        .iter()
        .map(|n| n.outer_html())
        .collect::<Vec<_>>()
        .join("");

    assert_eq!(
        result_string,
        String::from(
            r#"<html>
    <head>
        <title>This is the title</title>
    <meta content="This is the title" name="title"></head>
    <body>
        <h1>Title</h1>
    </body>
</html>"#
        )
    );

    Ok(())
}
