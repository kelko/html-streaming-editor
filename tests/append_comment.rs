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
fn append_to_first_p_content() -> Result<(), StreamingEditorError> {
    let command = "EXTRACT-ELEMENT{#first-para} | APPEND-COMMENT{'followed by a comment'}";

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
        String::from(r#"<p id="first-para">Some first text<!-- followed by a comment --></p>"#)
    );

    Ok(())
}

#[test]
fn append_to_ul() -> Result<(), StreamingEditorError> {
    let command = "EXTRACT-ELEMENT{ul} | APPEND-COMMENT{'Foo'}";

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
        String::from(
            r#"<ul id="list">
            <li id="item-1">1</li>
            <li id="item-2">2</li>
            <li id="item-3">3</li>
        <!-- Foo --></ul>"#
        )
    );

    Ok(())
}

#[test]
fn append_double_dash_will_be_escaped() -> Result<(), StreamingEditorError> {
    let command =
        "EXTRACT-ELEMENT{#first-para} | APPEND-COMMENT{'Actually -- is illegal in comments'}";

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
        String::from(
            r#"<p id="first-para">Some first text<!-- Actually \x2D\x2D is illegal in comments --></p>"#
        )
    );

    Ok(())
}

#[test]
fn append_ul_id_as_comment_to_first_para() -> Result<(), StreamingEditorError> {
    let command = "FOR-EACH{#first-para ↦ APPEND-COMMENT{ QUERY-PARENT{ul} | GET-ATTR{id} } }";

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
        String::from(
            r#"<html>
    <head></head>
    <body>
        <h1>Title</h1>
        <p id="first-para">Some first text<!-- list --></p>
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

    Ok(())
}
