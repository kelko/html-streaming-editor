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
    let command = "EXTRACT-ELEMENT{#first-para} | SET-TEXT-CONTENT{'Some new, boring text'}";

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
        String::from(r#"<p id="first-para">Some new, boring text</p>"#)
    );

    Ok(())
}

#[test]
fn overwrite_third_p_content() -> Result<(), StreamingEditorError> {
    let command = "EXTRACT-ELEMENT{#third-para} | SET-TEXT-CONTENT{'Simple Text'}";

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
        String::from(r#"<p id="third-para">Simple Text</p>"#)
    );

    Ok(())
}

#[test]
fn set_escape_needing_content() -> Result<(), StreamingEditorError> {
    let command =
        "EXTRACT-ELEMENT{#first-para} | SET-TEXT-CONTENT{'Some is > others < & you never know which'}";

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
            r#"<p id="first-para">Some is &gt; others &lt; &amp; you never know which</p>"#
        )
    );

    Ok(())
}

#[test]
fn set_ul_id_as_text_to_first_para() -> Result<(), StreamingEditorError> {
    let command = "FOR-EACH{#first-para ↦ SET-TEXT-CONTENT{ QUERY-PARENT{ul} | GET-ATTR{id} } }";

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
        <p id="first-para">list</p>
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
fn set_second_para_content_as_text_to_first_para() -> Result<(), StreamingEditorError> {
    let command = "FOR-EACH{#first-para ↦ SET-TEXT-CONTENT{ QUERY-PARENT{#second-para} | GET-TEXT-CONTENT } }";

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
        <p id="first-para">Some more text, even with an </p>
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
fn set_text_content_to_value_of_text_content() -> Result<(), StreamingEditorError> {
    let command =
        r#"EXTRACT-ELEMENT{#first-para} | SET-TEXT-CONTENT{ USE-ELEMENT | GET-TEXT-CONTENT }"#;

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
        String::from(r#"<p id="first-para">Some first text</p>"#)
    );

    Ok(())
}

#[test]
fn set_text_content_to_adjusted_value_of_text_content() -> Result<(), StreamingEditorError> {
    let command = r"EXTRACT-ELEMENT{#first-para} | SET-TEXT-CONTENT{USE-ELEMENT | GET-TEXT-CONTENT | REGEX-REPLACE{ '\s' ↤ '_'}}";

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
        String::from(r#"<p id="first-para">Some_first_text</p>"#)
    );

    Ok(())
}
