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
fn add_simple_div_to_first_p_content() -> Result<(), StreamingEditorError> {
    let command = "EXTRACT-ELEMENT{#first-para} | ADD-ELEMENT{ CREATE-ELEMENT{div} }";

    let mut input = Box::new(HTML_INPUT.as_bytes());
    let mut output = Vec::new();
    let hse = HtmlStreamingEditor::new(&mut input, &mut output);

    let _ = hse.run(command)?;
    let result_string = String::from_utf8(output).unwrap();

    assert_eq!(
        result_string,
        String::from(r#"<p id="first-para">Some first text<div></div></p>"#)
    );

    Ok(())
}

#[test]
fn add_two_divs_to_first_p_content() -> Result<(), StreamingEditorError> {
    let command = "EXTRACT-ELEMENT{#first-para} | ADD-ELEMENT{ CREATE-ELEMENT{div} } | ADD-ELEMENT{ CREATE-ELEMENT{div} }";

    let mut input = Box::new(HTML_INPUT.as_bytes());
    let mut output = Vec::new();
    let hse = HtmlStreamingEditor::new(&mut input, &mut output);

    let _ = hse.run(command)?;
    let result_string = String::from_utf8(output).unwrap();

    assert_eq!(
        result_string,
        String::from(r#"<p id="first-para">Some first text<div></div><div></div></p>"#)
    );

    Ok(())
}

#[test]
fn add_div_with_attr_to_first_p_content() -> Result<(), StreamingEditorError> {
    let command =
        "EXTRACT-ELEMENT{#first-para} | ADD-ELEMENT{ CREATE-ELEMENT{div} | SET-ATTR{id ↤ 'new'} }";

    let mut input = Box::new(HTML_INPUT.as_bytes());
    let mut output = Vec::new();
    let hse = HtmlStreamingEditor::new(&mut input, &mut output);

    let _ = hse.run(command)?;
    let result_string = String::from_utf8(output).unwrap();

    assert_eq!(
        result_string,
        String::from(r#"<p id="first-para">Some first text<div id="new"></div></p>"#)
    );

    Ok(())
}
