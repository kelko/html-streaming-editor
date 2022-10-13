use html_streaming_editor::*;

const HTML_INPUT: &str = r#"<html>
    <head>
        <meta name="test" content="some value">
    </head>
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
fn overwrite_first_p_id() -> Result<(), StreamingEditorError> {
    let command = "EXTRACT-ELEMENT{#first-para} | SET-ATTR{id ↤ 'new-id'}";

    let mut input = Box::new(HTML_INPUT.as_bytes());
    let mut output = Vec::new();
    let hse = HtmlStreamingEditor::new(&mut input, &mut output);

    let _ = hse.run(command)?;
    let result_string = String::from_utf8(output).unwrap();

    assert_eq!(
        result_string,
        String::from(r#"<p id="new-id">Some first text</p>"#)
    );

    Ok(())
}

#[test]
fn replace_and_characters_in_first_id() -> Result<(), StreamingEditorError> {
    let command = "EXTRACT-ELEMENT{#first-para} | SET-ATTR{id ↤ USE-ELEMENT | GET-ATTR{id} | REGEX-REPLACE{'\\W' ↤ '_'} }";
    let mut input = Box::new(HTML_INPUT.as_bytes());
    let mut output = Vec::new();
    let hse = HtmlStreamingEditor::new(&mut input, &mut output);

    let _ = hse.run(command)?;
    let result_string = String::from_utf8(output).unwrap();

    assert_eq!(
        result_string,
        String::from(r#"<p id="first_para">Some first text</p>"#)
    );

    Ok(())
}

#[test]
fn uppercase_first_id() -> Result<(), StreamingEditorError> {
    let command =
        "EXTRACT-ELEMENT{#first-para} | SET-ATTR{id ↤ USE-ELEMENT | GET-ATTR{id} | TO-UPPER }";
    let mut input = Box::new(HTML_INPUT.as_bytes());
    let mut output = Vec::new();
    let hse = HtmlStreamingEditor::new(&mut input, &mut output);

    let _ = hse.run(command)?;
    let result_string = String::from_utf8(output).unwrap();

    assert_eq!(
        result_string,
        String::from(r#"<p id="FIRST-PARA">Some first text</p>"#)
    );

    Ok(())
}

#[test]
fn add_attr_to_first_p() -> Result<(), StreamingEditorError> {
    let command = r#"EXTRACT-ELEMENT{#first-para} | SET-ATTR{data-test ↤ "some value"}"#;

    let mut input = Box::new(HTML_INPUT.as_bytes());
    let mut output = Vec::new();
    let hse = HtmlStreamingEditor::new(&mut input, &mut output);

    let _ = hse.run(command)?;
    let result_string = String::from_utf8(output).unwrap();

    assert_eq!(
        result_string,
        String::from(r#"<p data-test="some value" id="first-para">Some first text</p>"#)
    );

    Ok(())
}

#[test]
fn set_attr_with_double_quotes() -> Result<(), StreamingEditorError> {
    let command = r#"EXTRACT-ELEMENT{#first-para} | SET-ATTR{data-test ↤ 'some "value"'}"#;

    let mut input = Box::new(HTML_INPUT.as_bytes());
    let mut output = Vec::new();
    let hse = HtmlStreamingEditor::new(&mut input, &mut output);

    let _ = hse.run(command)?;
    let result_string = String::from_utf8(output).unwrap();

    assert_eq!(
        result_string,
        String::from(
            r#"<p data-test="some &quot;value&quot;" id="first-para">Some first text</p>"#
        )
    );

    Ok(())
}

#[test]
fn set_attr_with_line_break() -> Result<(), StreamingEditorError> {
    let command = "EXTRACT-ELEMENT{#first-para} | SET-ATTR{data-test ↤ 'some \nvalue'}";

    let mut input = Box::new(HTML_INPUT.as_bytes());
    let mut output = Vec::new();
    let hse = HtmlStreamingEditor::new(&mut input, &mut output);

    let _ = hse.run(command)?;
    let result_string = String::from_utf8(output).unwrap();

    assert_eq!(
        result_string,
        String::from(r#"<p data-test="some \nvalue" id="first-para">Some first text</p>"#)
    );

    Ok(())
}

#[test]
fn set_attr_from_other_attr() -> Result<(), StreamingEditorError> {
    let command = "EXTRACT-ELEMENT{#first-para} | SET-ATTR{data-test ↤ USE-ELEMENT | GET-ATTR{id}}";

    let mut input = Box::new(HTML_INPUT.as_bytes());
    let mut output = Vec::new();
    let hse = HtmlStreamingEditor::new(&mut input, &mut output);

    let _ = hse.run(command)?;
    let result_string = String::from_utf8(output).unwrap();

    assert_eq!(
        result_string,
        String::from(r#"<p data-test="first-para" id="first-para">Some first text</p>"#)
    );

    Ok(())
}

#[test]
fn set_attr_from_other_attr_but_uppercase() -> Result<(), StreamingEditorError> {
    let command = "EXTRACT-ELEMENT{#first-para} | SET-ATTR{data-test ↤ USE-ELEMENT | GET-ATTR{id} | TO-UPPER}";

    let mut input = Box::new(HTML_INPUT.as_bytes());
    let mut output = Vec::new();
    let hse = HtmlStreamingEditor::new(&mut input, &mut output);

    let _ = hse.run(command)?;
    let result_string = String::from_utf8(output).unwrap();

    assert_eq!(
        result_string,
        String::from(r#"<p data-test="FIRST-PARA" id="first-para">Some first text</p>"#)
    );

    Ok(())
}

#[test]
fn set_attr_from_attr_of_sibling() -> Result<(), StreamingEditorError> {
    let command =
        "FOR-EACH{#first-para ↦ SET-ATTR{data-test ↤ QUERY-PARENT{#second-para} | GET-ATTR{id}}} | EXTRACT-ELEMENT{#first-para}";

    let mut input = Box::new(HTML_INPUT.as_bytes());
    let mut output = Vec::new();
    let hse = HtmlStreamingEditor::new(&mut input, &mut output);

    let _ = hse.run(command)?;
    let result_string = String::from_utf8(output).unwrap();

    assert_eq!(
        result_string,
        String::from(r#"<p data-test="second-para" id="first-para">Some first text</p>"#)
    );

    Ok(())
}

#[test]
fn set_attr_from_attr_of_head_meta() -> Result<(), StreamingEditorError> {
    let command = "FOR-EACH{#first-para ↦ SET-ATTR{data-test ↤ QUERY-ROOT{meta[name='test']} | GET-ATTR{content}}}";

    let mut input = Box::new(HTML_INPUT.as_bytes());
    let mut output = Vec::new();
    let hse = HtmlStreamingEditor::new(&mut input, &mut output);

    let _ = hse.run(command)?;
    let result_string = String::from_utf8(output).unwrap();

    assert_eq!(
        result_string,
        String::from(
            r#"<html>
    <head>
        <meta content="some value" name="test">
    </head>
    <body>
        <h1>Title</h1>
        <p data-test="some value" id="first-para">Some first text</p>
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

#[test]
fn set_first_word_lowercased_as_id() -> Result<(), StreamingEditorError> {
    let command = r#"EXTRACT-ELEMENT{#first-para} | SET-ATTR{id ↤ USE-ELEMENT | GET-TEXT-CONTENT | REGEX-REPLACE{"^(\w+).*" ↤ "$1"} | TO-LOWER }"#;

    let mut input = Box::new(HTML_INPUT.as_bytes());
    let mut output = Vec::new();
    let hse = HtmlStreamingEditor::new(&mut input, &mut output);

    let _ = hse.run(command)?;
    let result_string = String::from_utf8(output).unwrap();

    assert_eq!(
        result_string,
        String::from(r#"<p id="some">Some first text</p>"#)
    );

    Ok(())
}

#[test]
fn set_first_word_prefixed_as_id() -> Result<(), StreamingEditorError> {
    let command = r#"EXTRACT-ELEMENT{#first-para} | SET-ATTR{id ↤ USE-ELEMENT | GET-TEXT-CONTENT | REGEX-REPLACE{"^(\w+).*" ↤ "$1"} | ADD-PREFIX{"id-"} }"#;

    let mut input = Box::new(HTML_INPUT.as_bytes());
    let mut output = Vec::new();
    let hse = HtmlStreamingEditor::new(&mut input, &mut output);

    let _ = hse.run(command)?;
    let result_string = String::from_utf8(output).unwrap();

    assert_eq!(
        result_string,
        String::from(r#"<p id="id-Some">Some first text</p>"#)
    );

    Ok(())
}
