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
fn parse_single_extract_element() {
    let parsed = super::grammar::command("EXTRACT-ELEMENT{a}");
    assert_eq!(
        parsed,
        Ok(Command::ExtractElement(CssSelectorList::new(vec![
            CssSelectorPath::single(CssSelector::for_element("a"))
        ])))
    );
}

#[test]
fn parse_single_extract_element_alias_only() {
    let parsed = super::grammar::command("ONLY{a}");
    assert_eq!(
        parsed,
        Ok(Command::ExtractElement(CssSelectorList::new(vec![
            CssSelectorPath::single(CssSelector::for_element("a"))
        ])))
    );
}

#[test]
fn parse_single_remove_element() {
    let parsed = super::grammar::command("REMOVE-ELEMENT{a}");
    assert_eq!(
        parsed,
        Ok(Command::RemoveElement(CssSelectorList::new(vec![
            CssSelectorPath::single(CssSelector::for_element("a"))
        ])))
    );
}

#[test]
fn parse_single_remove_element_alias_without() {
    let parsed = super::grammar::command("WITHOUT{a}");
    assert_eq!(
        parsed,
        Ok(Command::RemoveElement(CssSelectorList::new(vec![
            CssSelectorPath::single(CssSelector::for_element("a"))
        ])))
    );
}

#[test]
fn parse_two_grammar() {
    let parsed = super::grammar::pipeline("EXTRACT-ELEMENT{a} | REMOVE-ELEMENT{b}");
    assert_eq!(
        parsed,
        Ok(Pipeline::new(vec![
            Command::ExtractElement(CssSelectorList::new(vec![CssSelectorPath::single(
                CssSelector::for_element("a")
            )])),
            Command::RemoveElement(CssSelectorList::new(vec![CssSelectorPath::single(
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
    let parsed = super::grammar::command("SET-ATTR{data-test <= 'some text'}");
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
    let parsed = super::grammar::command("SET-TEXT-CONTENT{ <= 'some text'}");
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
    let parsed = super::grammar::command("ADD-TEXT-CONTENT{ <= 'some text'}");
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
    let parsed = super::grammar::command("ADD-COMMENT{ <= 'some text'}");
    assert_eq!(
        parsed,
        Ok(Command::AddComment(ValueSource::StringValue(String::from(
            "some text"
        ))))
    );
}

#[test]
fn parse_single_for_using_set_attr() {
    let parsed = super::grammar::command("FOR{li ↦ SET-ATTR{data-test ↤ 'some text'}}");
    assert_eq!(
        parsed,
        Ok(Command::ForEach(
            CssSelectorList::new(vec![CssSelectorPath::single(CssSelector::for_element(
                "li"
            ))]),
            Pipeline::new(vec![Command::SetAttribute(
                String::from("data-test"),
                ValueSource::StringValue(String::from("some text"))
            )]),
        ))
    );
}

#[test]
fn parse_single_for_each_using_set_attr() {
    let parsed = super::grammar::command("FOR-EACH{li ↦ SET-ATTR{data-test ↤ 'some text'}}");
    assert_eq!(
        parsed,
        Ok(Command::ForEach(
            CssSelectorList::new(vec![CssSelectorPath::single(CssSelector::for_element(
                "li"
            ))]),
            Pipeline::new(vec![Command::SetAttribute(
                String::from("data-test"),
                ValueSource::StringValue(String::from("some text"))
            )]),
        ))
    );
}

#[test]
fn parse_single_for_each_with_ascii_arrow_using_set_attr() {
    let parsed = super::grammar::command("FOR-EACH{li => SET-ATTR{data-test ↤ 'some text'}}");
    assert_eq!(
        parsed,
        Ok(Command::ForEach(
            CssSelectorList::new(vec![CssSelectorPath::single(CssSelector::for_element(
                "li"
            ))]),
            Pipeline::new(vec![Command::SetAttribute(
                String::from("data-test"),
                ValueSource::StringValue(String::from("some text"))
            )]),
        ))
    );
}

#[test]
fn parse_single_add_element_using_new_alias() {
    let parsed = super::grammar::command("ADD-ELEMENT{NEW{div}}");
    assert_eq!(
        parsed,
        Ok(Command::AddElement(Pipeline::new(vec![
            Command::CreateElement(String::from("div"))
        ])))
    );
}

#[test]
fn parse_single_add_element_using_create() {
    let parsed = super::grammar::command("ADD-ELEMENT{CREATE-ELEMENT{div}}");
    assert_eq!(
        parsed,
        Ok(Command::AddElement(Pipeline::new(vec![
            Command::CreateElement(String::from("div"))
        ])))
    );
}

#[test]
fn parse_single_add_element_using_from_file() {
    let parsed = super::grammar::command("ADD-ELEMENT{FROM-FILE{'tests/source.html'}}");
    assert_eq!(
        parsed,
        Ok(Command::AddElement(Pipeline::new(vec![Command::FromFile(
            String::from("tests/source.html")
        )])))
    );
}

#[test]
fn parse_single_add_element_using_source() {
    let parsed = super::grammar::command("ADD-ELEMENT{SOURCE{'tests/source.html'}}");
    assert_eq!(
        parsed,
        Ok(Command::AddElement(Pipeline::new(vec![Command::FromFile(
            String::from("tests/source.html")
        )])))
    );
}

#[test]
fn parse_single_add_element_with_arrow_using_create() {
    let parsed = super::grammar::command("ADD-ELEMENT{ ↤ CREATE-ELEMENT{div}}");
    assert_eq!(
        parsed,
        Ok(Command::AddElement(Pipeline::new(vec![
            Command::CreateElement(String::from("div"))
        ])))
    );
}

#[test]
fn parse_single_add_element_with_ascii_arrow_using_create() {
    let parsed = super::grammar::command("ADD-ELEMENT{ <= CREATE-ELEMENT{div}}");
    assert_eq!(
        parsed,
        Ok(Command::AddElement(Pipeline::new(vec![
            Command::CreateElement(String::from("div"))
        ])))
    );
}

//noinspection DuplicatedCode
#[test]
fn parse_single_replace_using_create() {
    let parsed = super::grammar::command("REPLACE{.replace-me ↤ CREATE-ELEMENT{p} }");
    assert_eq!(
        parsed,
        Ok(Command::Replace(
            CssSelectorList::new(vec![CssSelectorPath::single(CssSelector::for_class(
                "replace-me"
            ))]),
            Pipeline::new(vec![Command::CreateElement(String::from("p"))])
        )),
    );
}

//noinspection DuplicatedCode
#[test]
fn parse_single_replace_with_ascii_arrow_using_create() {
    let parsed = super::grammar::command("REPLACE{.replace-me <= CREATE-ELEMENT{p} }");
    assert_eq!(
        parsed,
        Ok(Command::Replace(
            CssSelectorList::new(vec![CssSelectorPath::single(CssSelector::for_class(
                "replace-me"
            ))]),
            Pipeline::new(vec![Command::CreateElement(String::from("p"))])
        )),
    );
}

//noinspection DuplicatedCode
#[test]
fn parse_single_replace_using_from_replaced() {
    let parsed = super::grammar::command("REPLACE{.replace-me ↤ FROM-REPLACED{p} }");
    assert_eq!(
        parsed,
        Ok(Command::Replace(
            CssSelectorList::new(vec![CssSelectorPath::single(CssSelector::for_class(
                "replace-me"
            ))]),
            Pipeline::new(vec![Command::FromReplaced(CssSelectorList::new(vec![
                CssSelectorPath::single(CssSelector::for_element("p"))
            ]))])
        )),
    );
}

//noinspection DuplicatedCode
#[test]
fn parse_single_replace_using_from_replaced_alias_keep() {
    let parsed = super::grammar::command("REPLACE{.replace-me ↤ KEEP{p} }");
    assert_eq!(
        parsed,
        Ok(Command::Replace(
            CssSelectorList::new(vec![CssSelectorPath::single(CssSelector::for_class(
                "replace-me"
            ))]),
            Pipeline::new(vec![Command::FromReplaced(CssSelectorList::new(vec![
                CssSelectorPath::single(CssSelector::for_element("p"))
            ]))])
        )),
    );
}
