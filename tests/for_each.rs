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
fn add_attr_to_li() -> Result<(), StreamingEditorError> {
    let command = r#"ONLY{ul} | FOR-EACH{li ↦ SET-ATTR{data-test ↤ "x"}}"#;

    let mut input = Box::new(HTML_INPUT.as_bytes());
    let mut output = Vec::new();
    let hse = HtmlStreamingEditor::new(&mut input, &mut output);

    let _ = hse.run(command)?;
    let result_string = String::from_utf8(output).unwrap();

    assert_eq!(
        result_string,
        String::from(
            r#"<ul id="list">
            <li data-test="x" id="item-1">1</li>
            <li data-test="x" id="item-2">2</li>
            <li data-test="x" id="item-3">3</li>
        </ul>"#
        )
    );

    Ok(())
}
