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
fn replace_ul_with_created_div() -> Result<(), StreamingEditorError> {
    let command = "REPLACE-ELEMENT{ul ↤ CREATE-ELEMENT{div} | SET-TEXT-CONTENT{'this was an UL'} | SET-ATTR{id ↤ 'new'}}";

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
        <p id="first-para">Some first text</p>
        <p id="second-para">Some more text, even with an <img src=""></p>
        <p id="third-para">Third text of <abbr>HTML</abbr>, but no <abbr>CSS</abbr></p>
        <div id="new">this was an UL</div>
    </body>
</html>"#
        )
    );

    Ok(())
}

#[test]
fn replace_ul_with_sourced_html() -> Result<(), StreamingEditorError> {
    let command = "REPLACE-ELEMENT{ul ↤ LOAD-FILE{'tests/source.html'} | EXTRACT-ELEMENT{ul}}";

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
        <p id="first-para">Some first text</p>
        <p id="second-para">Some more text, even with an <img src=""></p>
        <p id="third-para">Third text of <abbr>HTML</abbr>, but no <abbr>CSS</abbr></p>
        <ul id="first">
        <li>1</li>
        <li>2</li>
        <li>3</li>
    </ul><ul id="second">
        <li>a</li>
        <li><!-- Some Comment -->b</li>
        <li><em class="intense">c</em></li>
    </ul>
    </body>
</html>"#
        )
    );

    Ok(())
}

#[test]
fn replace_third_para_with_child_abbr() -> Result<(), StreamingEditorError> {
    let command = "REPLACE-ELEMENT{#third-para ↤ QUERY-REPLACED{abbr}}";

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
        <p id="first-para">Some first text</p>
        <p id="second-para">Some more text, even with an <img src=""></p>
        <abbr>HTML</abbr><abbr>CSS</abbr>
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
