use crate::element_creating::{ElementCreatingCommand, ElementCreatingPipeline};
use crate::element_processing::{ElementProcessingCommand, ElementProcessingPipeline};
use crate::string_creating::{
    ElementSelectingCommand, StringValueCreatingPipeline, ValueExtractingCommand,
};
use crate::{CssSelector, CssSelectorList, CssSelectorPath, ValueSource};

const EXEMPLARY_SUB_PIPELINE_DEFINITION: &str = "USE-ELEMENT | GET-ATTR{data-test}";
const EXEMPLARY_SUB_PIPELINE_MODEL: StringValueCreatingPipeline = StringValueCreatingPipeline::new(
    ElementSelectingCommand::UseElement,
    ValueExtractingCommand::GetAttribute("data-test"),
);

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
    let parsed = super::grammar::element_processing_command("EXTRACT-ELEMENT{a}");
    assert_eq!(
        parsed,
        Ok(ElementProcessingCommand::ExtractElement(
            CssSelectorList::new(vec![CssSelectorPath::single(CssSelector::for_element("a"))])
        ))
    );
}

#[test]
fn parse_single_extract_element_alias_only() {
    let parsed = super::grammar::element_processing_command("ONLY{a}");
    assert_eq!(
        parsed,
        Ok(ElementProcessingCommand::ExtractElement(
            CssSelectorList::new(vec![CssSelectorPath::single(CssSelector::for_element("a"))])
        ))
    );
}

#[test]
fn parse_single_remove_element() {
    let parsed = super::grammar::element_processing_command("REMOVE-ELEMENT{a}");
    assert_eq!(
        parsed,
        Ok(ElementProcessingCommand::RemoveElement(
            CssSelectorList::new(vec![CssSelectorPath::single(CssSelector::for_element("a"))])
        ))
    );
}

#[test]
fn parse_single_remove_element_alias_without() {
    let parsed = super::grammar::element_processing_command("WITHOUT{a}");
    assert_eq!(
        parsed,
        Ok(ElementProcessingCommand::RemoveElement(
            CssSelectorList::new(vec![CssSelectorPath::single(CssSelector::for_element("a"))])
        ))
    );
}

#[test]
fn parse_two_grammar() {
    let parsed = super::grammar::pipeline("EXTRACT-ELEMENT{a} | REMOVE-ELEMENT{b}");
    assert_eq!(
        parsed,
        Ok(ElementProcessingPipeline::new(vec![
            ElementProcessingCommand::ExtractElement(CssSelectorList::new(vec![
                CssSelectorPath::single(CssSelector::for_element("a"))
            ])),
            ElementProcessingCommand::RemoveElement(CssSelectorList::new(vec![
                CssSelectorPath::single(CssSelector::for_element("b"))
            ])),
        ]))
    );
}

#[test]
fn parse_single_clear_attr() {
    let parsed = super::grammar::element_processing_command("CLEAR-ATTR{a}");
    assert_eq!(parsed, Ok(ElementProcessingCommand::ClearAttribute("a")));
}

#[test]
fn parse_single_clear_content() {
    let parsed = super::grammar::element_processing_command("CLEAR-CONTENT");
    assert_eq!(parsed, Ok(ElementProcessingCommand::ClearContent));
}

#[test]
fn parse_single_set_attr_by_string() {
    let parsed = super::grammar::element_processing_command("SET-ATTR{data-test ↤ 'some text'}");
    assert_eq!(
        parsed,
        Ok(ElementProcessingCommand::SetAttribute(
            "data-test",
            ValueSource::StringValue("some text")
        ))
    );
}

#[test]
fn parse_single_set_attr_by_string_with_ascii_arrow() {
    let parsed = super::grammar::element_processing_command("SET-ATTR{data-test <= 'some text'}");
    assert_eq!(
        parsed,
        Ok(ElementProcessingCommand::SetAttribute(
            "data-test",
            ValueSource::StringValue("some text")
        ))
    );
}

#[test]
fn parse_single_set_attr_by_sub_pipeline() {
    let constructed_pipeline = format!(
        "SET-ATTR{{data-test ↤ {} }}",
        EXEMPLARY_SUB_PIPELINE_DEFINITION
    );
    let parsed = super::grammar::element_processing_command(&constructed_pipeline);

    assert_eq!(
        parsed,
        Ok(ElementProcessingCommand::SetAttribute(
            "data-test",
            ValueSource::SubPipeline(EXEMPLARY_SUB_PIPELINE_MODEL.clone())
        ))
    );
}

#[test]
fn parse_single_set_text_content_by_string() {
    let parsed = super::grammar::element_processing_command("SET-TEXT-CONTENT{'some text'}");
    assert_eq!(
        parsed,
        Ok(ElementProcessingCommand::SetTextContent(
            ValueSource::StringValue("some text")
        ))
    );
}

#[test]
fn parse_single_set_text_content_by_string_with_arrow() {
    let parsed = super::grammar::element_processing_command("SET-TEXT-CONTENT{ ↤ 'some text'}");
    assert_eq!(
        parsed,
        Ok(ElementProcessingCommand::SetTextContent(
            ValueSource::StringValue("some text")
        ))
    );
}

#[test]
fn parse_single_set_text_content_by_sub_pipeline() {
    let constructed_pipeline = format!(
        "SET-TEXT-CONTENT{{ {} }}",
        EXEMPLARY_SUB_PIPELINE_DEFINITION
    );
    let parsed = super::grammar::element_processing_command(&constructed_pipeline);

    assert_eq!(
        parsed,
        Ok(ElementProcessingCommand::SetTextContent(
            ValueSource::SubPipeline(EXEMPLARY_SUB_PIPELINE_MODEL.clone())
        ))
    );
}

#[test]
fn parse_single_set_text_content_by_string_with_ascii_arrow() {
    let parsed = super::grammar::element_processing_command("SET-TEXT-CONTENT{ <= 'some text'}");
    assert_eq!(
        parsed,
        Ok(ElementProcessingCommand::SetTextContent(
            ValueSource::StringValue("some text")
        ))
    );
}

#[test]
fn parse_single_add_text_content_by_string() {
    let parsed = super::grammar::element_processing_command("ADD-TEXT-CONTENT{'some text'}");
    assert_eq!(
        parsed,
        Ok(ElementProcessingCommand::AddTextContent(
            ValueSource::StringValue("some text")
        ))
    );
}

#[test]
fn parse_single_add_text_content_by_string_with_arrow() {
    let parsed = super::grammar::element_processing_command("ADD-TEXT-CONTENT{ ↤ 'some text'}");
    assert_eq!(
        parsed,
        Ok(ElementProcessingCommand::AddTextContent(
            ValueSource::StringValue("some text")
        ))
    );
}

#[test]
fn parse_single_add_text_content_by_sub_pipeline() {
    let constructed_pipeline = format!(
        "ADD-TEXT-CONTENT{{ {} }}",
        EXEMPLARY_SUB_PIPELINE_DEFINITION
    );
    let parsed = super::grammar::element_processing_command(&constructed_pipeline);

    assert_eq!(
        parsed,
        Ok(ElementProcessingCommand::AddTextContent(
            ValueSource::SubPipeline(EXEMPLARY_SUB_PIPELINE_MODEL.clone())
        ))
    );
}

#[test]
fn parse_single_add_text_content_by_string_with_ascii_arrow() {
    let parsed = super::grammar::element_processing_command("ADD-TEXT-CONTENT{ <= 'some text'}");
    assert_eq!(
        parsed,
        Ok(ElementProcessingCommand::AddTextContent(
            ValueSource::StringValue("some text")
        ))
    );
}

#[test]
fn parse_single_add_comment_by_string() {
    let parsed = super::grammar::element_processing_command("ADD-COMMENT{'some text'}");
    assert_eq!(
        parsed,
        Ok(ElementProcessingCommand::AddComment(
            ValueSource::StringValue("some text")
        ))
    );
}

#[test]
fn parse_single_add_comment_by_string_with_arrow() {
    let parsed = super::grammar::element_processing_command("ADD-COMMENT{ ↤ 'some text'}");
    assert_eq!(
        parsed,
        Ok(ElementProcessingCommand::AddComment(
            ValueSource::StringValue("some text")
        ))
    );
}

#[test]
fn parse_single_add_comment_by_string_with_ascii_arrow() {
    let parsed = super::grammar::element_processing_command("ADD-COMMENT{ <= 'some text'}");
    assert_eq!(
        parsed,
        Ok(ElementProcessingCommand::AddComment(
            ValueSource::StringValue("some text")
        ))
    );
}

//noinspection DuplicatedCode
#[test]
fn parse_single_for_each_alias_with_using_set_attr() {
    let parsed =
        super::grammar::element_processing_command("WITH{li ↦ SET-ATTR{data-test ↤ 'some text'}}");
    assert_eq!(
        parsed,
        Ok(ElementProcessingCommand::ForEach(
            CssSelectorList::new(vec![CssSelectorPath::single(CssSelector::for_element(
                "li"
            ))]),
            ElementProcessingPipeline::new(vec![ElementProcessingCommand::SetAttribute(
                "data-test",
                ValueSource::StringValue("some text")
            )]),
        ))
    );
}

//noinspection DuplicatedCode
#[test]
fn parse_single_for_each_using_set_attr() {
    let parsed = super::grammar::element_processing_command(
        "FOR-EACH{li ↦ SET-ATTR{data-test ↤ 'some text'}}",
    );
    assert_eq!(
        parsed,
        Ok(ElementProcessingCommand::ForEach(
            CssSelectorList::new(vec![CssSelectorPath::single(CssSelector::for_element(
                "li"
            ))]),
            ElementProcessingPipeline::new(vec![ElementProcessingCommand::SetAttribute(
                "data-test",
                ValueSource::StringValue("some text")
            )]),
        ))
    );
}

//noinspection DuplicatedCode
#[test]
fn parse_single_for_each_with_ascii_arrow_using_set_attr() {
    let parsed = super::grammar::element_processing_command(
        "FOR-EACH{li => SET-ATTR{data-test ↤ 'some text'}}",
    );
    assert_eq!(
        parsed,
        Ok(ElementProcessingCommand::ForEach(
            CssSelectorList::new(vec![CssSelectorPath::single(CssSelector::for_element(
                "li"
            ))]),
            ElementProcessingPipeline::new(vec![ElementProcessingCommand::SetAttribute(
                "data-test",
                ValueSource::StringValue("some text")
            )]),
        ))
    );
}

#[test]
fn parse_single_add_element_using_new_alias() {
    let parsed = super::grammar::element_processing_command("ADD-ELEMENT{NEW{div}}");
    assert_eq!(
        parsed,
        Ok(ElementProcessingCommand::AddElement(
            ElementCreatingPipeline::new(ElementCreatingCommand::CreateElement("div"), None)
        ))
    );
}

#[test]
fn parse_single_add_element_using_create() {
    let parsed = super::grammar::element_processing_command("ADD-ELEMENT{CREATE-ELEMENT{div}}");
    assert_eq!(
        parsed,
        Ok(ElementProcessingCommand::AddElement(
            ElementCreatingPipeline::new(ElementCreatingCommand::CreateElement("div"), None)
        ))
    );
}

#[test]
fn parse_single_add_element_using_from_file() {
    let parsed =
        super::grammar::element_processing_command("ADD-ELEMENT{FROM-FILE{'tests/source.html'}}");
    assert_eq!(
        parsed,
        Ok(ElementProcessingCommand::AddElement(
            ElementCreatingPipeline::new(
                ElementCreatingCommand::FromFile("tests/source.html"),
                None
            )
        ))
    );
}

#[test]
fn parse_single_add_element_using_source() {
    let parsed =
        super::grammar::element_processing_command("ADD-ELEMENT{SOURCE{'tests/source.html'}}");
    assert_eq!(
        parsed,
        Ok(ElementProcessingCommand::AddElement(
            ElementCreatingPipeline::new(
                ElementCreatingCommand::FromFile("tests/source.html"),
                None
            )
        ))
    );
}

#[test]
fn parse_single_add_element_with_arrow_using_create() {
    let parsed = super::grammar::element_processing_command("ADD-ELEMENT{ ↤ CREATE-ELEMENT{div}}");
    assert_eq!(
        parsed,
        Ok(ElementProcessingCommand::AddElement(
            ElementCreatingPipeline::new(ElementCreatingCommand::CreateElement("div"), None)
        ))
    );
}

#[test]
fn parse_single_add_element_with_ascii_arrow_using_create() {
    let parsed = super::grammar::element_processing_command("ADD-ELEMENT{ <= CREATE-ELEMENT{div}}");
    assert_eq!(
        parsed,
        Ok(ElementProcessingCommand::AddElement(
            ElementCreatingPipeline::new(ElementCreatingCommand::CreateElement("div"), None)
        ))
    );
}

//noinspection DuplicatedCode
#[test]
fn parse_single_replace_using_create() {
    let parsed =
        super::grammar::element_processing_command("REPLACE{.replace-me ↤ CREATE-ELEMENT{p} }");
    assert_eq!(
        parsed,
        Ok(ElementProcessingCommand::Replace(
            CssSelectorList::new(vec![CssSelectorPath::single(CssSelector::for_class(
                "replace-me"
            ))]),
            ElementCreatingPipeline::new(ElementCreatingCommand::CreateElement("p"), None)
        )),
    );
}

//noinspection DuplicatedCode
#[test]
fn parse_single_replace_with_ascii_arrow_using_create() {
    let parsed =
        super::grammar::element_processing_command("REPLACE{.replace-me <= CREATE-ELEMENT{p} }");
    assert_eq!(
        parsed,
        Ok(ElementProcessingCommand::Replace(
            CssSelectorList::new(vec![CssSelectorPath::single(CssSelector::for_class(
                "replace-me"
            ))]),
            ElementCreatingPipeline::new(ElementCreatingCommand::CreateElement("p"), None)
        )),
    );
}

//noinspection DuplicatedCode
#[test]
fn parse_single_replace_using_query_replaced() {
    let parsed =
        super::grammar::element_processing_command("REPLACE{.replace-me ↤ QUERY-REPLACED{p} }");
    assert_eq!(
        parsed,
        Ok(ElementProcessingCommand::Replace(
            CssSelectorList::new(vec![CssSelectorPath::single(CssSelector::for_class(
                "replace-me"
            ))]),
            ElementCreatingPipeline::new(
                ElementCreatingCommand::FromReplaced(CssSelectorList::new(vec![
                    CssSelectorPath::single(CssSelector::for_element("p"))
                ])),
                None
            )
        )),
    );
}

//noinspection DuplicatedCode
#[test]
fn parse_single_replace_using_query_replaced_alias_keep() {
    let parsed = super::grammar::element_processing_command("REPLACE{.replace-me ↤ KEEP{p} }");
    assert_eq!(
        parsed,
        Ok(ElementProcessingCommand::Replace(
            CssSelectorList::new(vec![CssSelectorPath::single(CssSelector::for_class(
                "replace-me"
            ))]),
            ElementCreatingPipeline::new(
                ElementCreatingCommand::FromReplaced(CssSelectorList::new(vec![
                    CssSelectorPath::single(CssSelector::for_element("p"))
                ])),
                None
            )
        )),
    );
}

#[test]
fn parse_string_creating_pipeline_use_element_get_attr() {
    let parsed = super::grammar::string_creating_pipeline("USE-ELEMENT | GET-ATTR{data-test}");
    assert_eq!(
        parsed,
        Ok(StringValueCreatingPipeline::new(
            ElementSelectingCommand::UseElement,
            ValueExtractingCommand::GetAttribute("data-test"),
        )),
    );
}

#[test]
fn parse_use_element() {
    let parsed = super::grammar::element_selecting_command("USE-ELEMENT");
    assert_eq!(parsed, Ok(ElementSelectingCommand::UseElement));
}

#[test]
fn parse_use_element_alias_this() {
    let parsed = super::grammar::element_selecting_command("THIS");
    assert_eq!(parsed, Ok(ElementSelectingCommand::UseElement));
}

#[test]
fn parse_use_parent() {
    let parsed = super::grammar::element_selecting_command("USE-PARENT");
    assert_eq!(parsed, Ok(ElementSelectingCommand::UseParent));
}

#[test]
fn parse_use_parent_alias_parent() {
    let parsed = super::grammar::element_selecting_command("PARENT");
    assert_eq!(parsed, Ok(ElementSelectingCommand::UseParent));
}

#[test]
fn parse_query_element() {
    let parsed = super::grammar::element_selecting_command("QUERY-ELEMENT{div}");
    assert_eq!(
        parsed,
        Ok(ElementSelectingCommand::QueryElement(CssSelectorList::new(
            vec![CssSelectorPath::single(CssSelector::for_element("div"))]
        ))),
    );
}

#[test]
fn parse_query_parent() {
    let parsed = super::grammar::element_selecting_command("QUERY-PARENT{div}");
    assert_eq!(
        parsed,
        Ok(ElementSelectingCommand::QueryParent(CssSelectorList::new(
            vec![CssSelectorPath::single(CssSelector::for_element("div"))]
        ))),
    );
}

#[test]
fn parse_query_root() {
    let parsed = super::grammar::element_selecting_command("QUERY-ROOT{div}");
    assert_eq!(
        parsed,
        Ok(ElementSelectingCommand::QueryRoot(CssSelectorList::new(
            vec![CssSelectorPath::single(CssSelector::for_element("div"))]
        ))),
    );
}

#[test]
fn parse_get_attr() {
    let parsed = super::grammar::value_extracting_command("GET-ATTR{data-test}");
    assert_eq!(
        parsed,
        Ok(ValueExtractingCommand::GetAttribute("data-test")),
    );
}

#[test]
fn parse_get_text_content() {
    let parsed = super::grammar::value_extracting_command("GET-TEXT-CONTENT");
    assert_eq!(parsed, Ok(ValueExtractingCommand::GetTextContent));
}
