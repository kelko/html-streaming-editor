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
fn overwrite_first_p_content() -> Result<(), StreamingEditorError> {
    let command = "ONLY{#first-para} | SET-TEXT-CONTENT{'Some new, boring text'}";

    let mut input = Box::new(HTML_INPUT.as_bytes());
    let mut output = Vec::new();
    let hse = HtmlStreamingEditor::new(&mut input, &mut output);

    let _ = hse.run(command)?;
    let result_string = String::from_utf8(output).unwrap();

    assert_eq!(
        result_string,
        String::from(r#"<p id="first-para">Some new, boring text</p>"#)
    );

    Ok(())
}

#[test]
fn overwrite_third_p_content() -> Result<(), StreamingEditorError> {
    let command = "ONLY{#third-para} | SET-TEXT-CONTENT{'Simple Text'}";

    let mut input = Box::new(HTML_INPUT.as_bytes());
    let mut output = Vec::new();
    let hse = HtmlStreamingEditor::new(&mut input, &mut output);

    let _ = hse.run(command)?;
    let result_string = String::from_utf8(output).unwrap();

    assert_eq!(
        result_string,
        String::from(r#"<p id="third-para">Simple Text</p>"#)
    );

    Ok(())
}

#[test]
fn set_escape_needing_content() -> Result<(), StreamingEditorError> {
    let command =
        "ONLY{#first-para} | SET-TEXT-CONTENT{'Some is > others < & you never know which'}";

    let mut input = Box::new(HTML_INPUT.as_bytes());
    let mut output = Vec::new();
    let hse = HtmlStreamingEditor::new(&mut input, &mut output);

    let _ = hse.run(command)?;
    let result_string = String::from_utf8(output).unwrap();

    assert_eq!(
        result_string,
        String::from(
            r#"<p id="first-para">Some is &gt; others &lt; &amp; you never know which</p>"#
        )
    );

    Ok(())
}
