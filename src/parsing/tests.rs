use crate::{Command, CssSelector, CssSelectorList, CssSelectorPath, Pipeline, ValueSource};

#[test]
fn parse_value_simple_doublequotes() {
    let parsed = super::grammar::string_value("\"a\"");
    assert_eq!(parsed, Ok("a"))
}

#[test]
fn parse_value_simple_singlequotes() {
    let parsed = super::grammar::string_value("'a'");
    assert_eq!(parsed, Ok("a"))
}

#[test]
fn parse_value_simple_questionsmarks() {
    let parsed = super::grammar::string_value("?a?");
    assert_eq!(parsed, Ok("a"))
}

#[test]
fn parse_value_loreipsum_doublequotes() {
    let parsed = super::grammar::string_value("\"Lorem ipsum dolor sit amet, consetetur sadipscing elitr, sed diam nonumy eirmod tempor invidunt ut labore et dolore magna aliquyam erat, sed diam voluptua. At vero eos et accusam et justo duo dolores et ea rebum. Stet clita kasd gubergren, no sea takimata sanctus est Lorem ipsum dolor sit amet. Lorem ipsum dolor sit amet, consetetur sadipscing elitr, sed diam nonumy eirmod tempor invidunt ut labore et dolore magna aliquyam erat, sed diam voluptua. At vero eos et accusam et justo duo dolores et ea rebum. Stet clita kasd gubergren, no sea takimata sanctus est Lorem ipsum dolor sit amet.\"");
    assert_eq!(parsed, Ok("Lorem ipsum dolor sit amet, consetetur sadipscing elitr, sed diam nonumy eirmod tempor invidunt ut labore et dolore magna aliquyam erat, sed diam voluptua. At vero eos et accusam et justo duo dolores et ea rebum. Stet clita kasd gubergren, no sea takimata sanctus est Lorem ipsum dolor sit amet. Lorem ipsum dolor sit amet, consetetur sadipscing elitr, sed diam nonumy eirmod tempor invidunt ut labore et dolore magna aliquyam erat, sed diam voluptua. At vero eos et accusam et justo duo dolores et ea rebum. Stet clita kasd gubergren, no sea takimata sanctus est Lorem ipsum dolor sit amet."))
}

#[test]
fn parse_value_loreipsum_singlequotes() {
    let parsed = super::grammar::string_value("'Lorem ipsum dolor sit amet, consetetur sadipscing elitr, sed diam nonumy eirmod tempor invidunt ut labore et dolore magna aliquyam erat, sed diam voluptua. At vero eos et accusam et justo duo dolores et ea rebum. Stet clita kasd gubergren, no sea takimata sanctus est Lorem ipsum dolor sit amet. Lorem ipsum dolor sit amet, consetetur sadipscing elitr, sed diam nonumy eirmod tempor invidunt ut labore et dolore magna aliquyam erat, sed diam voluptua. At vero eos et accusam et justo duo dolores et ea rebum. Stet clita kasd gubergren, no sea takimata sanctus est Lorem ipsum dolor sit amet.'");
    assert_eq!(parsed, Ok("Lorem ipsum dolor sit amet, consetetur sadipscing elitr, sed diam nonumy eirmod tempor invidunt ut labore et dolore magna aliquyam erat, sed diam voluptua. At vero eos et accusam et justo duo dolores et ea rebum. Stet clita kasd gubergren, no sea takimata sanctus est Lorem ipsum dolor sit amet. Lorem ipsum dolor sit amet, consetetur sadipscing elitr, sed diam nonumy eirmod tempor invidunt ut labore et dolore magna aliquyam erat, sed diam voluptua. At vero eos et accusam et justo duo dolores et ea rebum. Stet clita kasd gubergren, no sea takimata sanctus est Lorem ipsum dolor sit amet."))
}

#[test]
fn parse_value_loreipsum_questionsmarks() {
    let parsed = super::grammar::string_value("?Lorem ipsum dolor sit amet, consetetur sadipscing elitr, sed diam nonumy eirmod tempor invidunt ut labore et dolore magna aliquyam erat, sed diam voluptua. At vero eos et accusam et justo duo dolores et ea rebum. Stet clita kasd gubergren, no sea takimata sanctus est Lorem ipsum dolor sit amet. Lorem ipsum dolor sit amet, consetetur sadipscing elitr, sed diam nonumy eirmod tempor invidunt ut labore et dolore magna aliquyam erat, sed diam voluptua. At vero eos et accusam et justo duo dolores et ea rebum. Stet clita kasd gubergren, no sea takimata sanctus est Lorem ipsum dolor sit amet.?");
    assert_eq!(parsed, Ok("Lorem ipsum dolor sit amet, consetetur sadipscing elitr, sed diam nonumy eirmod tempor invidunt ut labore et dolore magna aliquyam erat, sed diam voluptua. At vero eos et accusam et justo duo dolores et ea rebum. Stet clita kasd gubergren, no sea takimata sanctus est Lorem ipsum dolor sit amet. Lorem ipsum dolor sit amet, consetetur sadipscing elitr, sed diam nonumy eirmod tempor invidunt ut labore et dolore magna aliquyam erat, sed diam voluptua. At vero eos et accusam et justo duo dolores et ea rebum. Stet clita kasd gubergren, no sea takimata sanctus est Lorem ipsum dolor sit amet."))
}

#[test]
fn parse_value_germanumlauts() {
    let parsed = super::grammar::string_value("'Hä?'");
    assert_eq!(parsed, Ok("Hä?"))
}

#[test]
fn parse_value_doublequoted_cant_have_doublequotes() {
    let parsed = super::grammar::string_value("\"a\"b\"");
    assert!(parsed.is_err())
}

#[test]
fn parse_value_singlequoted_cant_have_singlequotes() {
    let parsed = super::grammar::string_value("'a'b'");
    assert!(parsed.is_err())
}

#[test]
fn parse_value_questionmarked_cant_have_questionmarks() {
    let parsed = super::grammar::string_value("?a?b?");
    assert!(parsed.is_err())
}

#[test]
fn parse_single_only() {
    let parsed = super::grammar::command("ONLY{a}");
    assert_eq!(
        parsed,
        Ok(Command::Only(CssSelectorList::new(vec![
            CssSelectorPath::single(CssSelector::for_element("a"))
        ])))
    );
}

#[test]
fn parse_single_select_alias() {
    let parsed = super::grammar::command("SELECT{a}");
    assert_eq!(
        parsed,
        Ok(Command::Only(CssSelectorList::new(vec![
            CssSelectorPath::single(CssSelector::for_element("a"))
        ])))
    );
}

#[test]
fn parse_single_without() {
    let parsed = super::grammar::command("WITHOUT{a}");
    assert_eq!(
        parsed,
        Ok(Command::Without(CssSelectorList::new(vec![
            CssSelectorPath::single(CssSelector::for_element("a"))
        ])))
    );
}

#[test]
fn parse_single_filter_alias() {
    let parsed = super::grammar::command("FILTER{a}");
    assert_eq!(
        parsed,
        Ok(Command::Without(CssSelectorList::new(vec![
            CssSelectorPath::single(CssSelector::for_element("a"))
        ])))
    );
}

#[test]
fn parse_two_grammar() {
    let parsed = super::grammar::pipeline("ONLY{a} | WITHOUT{b}");
    assert_eq!(
        parsed,
        Ok(Pipeline::new(vec![
            Command::Only(CssSelectorList::new(vec![CssSelectorPath::single(
                CssSelector::for_element("a")
            )])),
            Command::Without(CssSelectorList::new(vec![CssSelectorPath::single(
                CssSelector::for_element("b")
            )])),
        ]))
    );
}

#[test]
fn parse_single_clear_attr() {
    let parsed = super::grammar::command("CLEAR-ATTR{a}");
    assert_eq!(parsed, Ok(Command::ClearAttribute(String::from("a"))));
}

#[test]
fn parse_single_clear_content() {
    let parsed = super::grammar::command("CLEAR-CONTENT");
    assert_eq!(parsed, Ok(Command::ClearContent));
}

#[test]
fn parse_single_set_attr_by_string() {
    let parsed = super::grammar::command("SET-ATTR{data-test ↤ 'some text'}");
    assert_eq!(
        parsed,
        Ok(Command::SetAttribute(
            String::from("data-test"),
            ValueSource::StringValue(String::from("some text"))
        ))
    );
}

#[test]
fn parse_single_set_attr_by_string_with_ascii_arrow() {
    let parsed = super::grammar::command("SET-ATTR{data-test <- 'some text'}");
    assert_eq!(
        parsed,
        Ok(Command::SetAttribute(
            String::from("data-test"),
            ValueSource::StringValue(String::from("some text"))
        ))
    );
}

#[test]
fn parse_single_set_text_content_by_string() {
    let parsed = super::grammar::command("SET-TEXT-CONTENT{'some text'}");
    assert_eq!(
        parsed,
        Ok(Command::SetTextContent(ValueSource::StringValue(
            String::from("some text")
        )))
    );
}

#[test]
fn parse_single_set_text_content_by_string_with_arrow() {
    let parsed = super::grammar::command("SET-TEXT-CONTENT{ ↤ 'some text'}");
    assert_eq!(
        parsed,
        Ok(Command::SetTextContent(ValueSource::StringValue(
            String::from("some text")
        )))
    );
}

#[test]
fn parse_single_set_text_content_by_string_with_ascii_arrow() {
    let parsed = super::grammar::command("SET-TEXT-CONTENT{ <- 'some text'}");
    assert_eq!(
        parsed,
        Ok(Command::SetTextContent(ValueSource::StringValue(
            String::from("some text")
        )))
    );
}

#[test]
fn parse_single_add_text_content_by_string() {
    let parsed = super::grammar::command("ADD-TEXT-CONTENT{'some text'}");
    assert_eq!(
        parsed,
        Ok(Command::AddTextContent(ValueSource::StringValue(
            String::from("some text")
        )))
    );
}

#[test]
fn parse_single_add_text_content_by_string_with_arrow() {
    let parsed = super::grammar::command("ADD-TEXT-CONTENT{ ↤ 'some text'}");
    assert_eq!(
        parsed,
        Ok(Command::AddTextContent(ValueSource::StringValue(
            String::from("some text")
        )))
    );
}

#[test]
fn parse_single_add_text_content_by_string_with_ascii_arrow() {
    let parsed = super::grammar::command("ADD-TEXT-CONTENT{ <- 'some text'}");
    assert_eq!(
        parsed,
        Ok(Command::AddTextContent(ValueSource::StringValue(
            String::from("some text")
        )))
    );
}

#[test]
fn parse_single_add_comment_by_string() {
    let parsed = super::grammar::command("ADD-COMMENT{'some text'}");
    assert_eq!(
        parsed,
        Ok(Command::AddComment(ValueSource::StringValue(String::from(
            "some text"
        ))))
    );
}

#[test]
fn parse_single_add_comment_by_string_with_arrow() {
    let parsed = super::grammar::command("ADD-COMMENT{ ↤ 'some text'}");
    assert_eq!(
        parsed,
        Ok(Command::AddComment(ValueSource::StringValue(String::from(
            "some text"
        ))))
    );
}

#[test]
fn parse_single_add_comment_by_string_with_ascii_arrow() {
    let parsed = super::grammar::command("ADD-COMMENT{ <- 'some text'}");
    assert_eq!(
        parsed,
        Ok(Command::AddComment(ValueSource::StringValue(String::from(
            "some text"
        ))))
    );
}
