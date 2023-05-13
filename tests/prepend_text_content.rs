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
fn prepend_to_first_p_content() -> Result<(), StreamingEditorError> {
    let command =
        "EXTRACT-ELEMENT{#first-para} | PREPEND-TEXT-CONTENT{'... expanded by other text'}";

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
        String::from(r#"<p id="first-para">... expanded by other textSome first text</p>"#)
    );

    Ok(())
}

#[test]
fn prepend_escape_needing_content() -> Result<(), StreamingEditorError> {
    let command =
        "EXTRACT-ELEMENT{#first-para} | PREPEND-TEXT-CONTENT{' is > others < & you never know which'}";

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
            r#"<p id="first-para"> is &gt; others &lt; &amp; you never know whichSome first text</p>"#
        )
    );

    Ok(())
}

#[test]
fn prepend_ul_id_as_text_to_first_para() -> Result<(), StreamingEditorError> {
    let command = "FOR-EACH{#first-para ↦ PREPEND-TEXT-CONTENT{' and ul-id is: '} | PREPEND-TEXT-CONTENT{ QUERY-PARENT{ul} | GET-ATTR{id} } }";

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
        <p id="first-para">list and ul-id is: Some first text</p>
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
