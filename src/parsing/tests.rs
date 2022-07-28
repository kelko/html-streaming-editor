use crate::{CssSelector, CssSelectorList, CssSelectorPath};

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
fn parse_identifier_simple_doublequotes() {
    let parsed = super::grammar::enclosed_identifier("\"a\"");
    assert_eq!(parsed, Ok("a"))
}

#[test]
fn parse_identifier_simple_singlequotes() {
    let parsed = super::grammar::enclosed_identifier("'a'");
    assert_eq!(parsed, Ok("a"))
}

#[test]
fn parse_identifier_simple_questionsmarks() {
    let parsed = super::grammar::enclosed_identifier("?a?");
    assert_eq!(parsed, Ok("a"))
}

#[test]
fn parse_identifier_whitespaced_questionsmarks() {
    let parsed = super::grammar::enclosed_identifier("? a ?");
    assert_eq!(parsed, Ok("a"))
}

#[test]
fn parse_identifier_whitespaced_doublequotes() {
    let parsed = super::grammar::enclosed_identifier("\" a \"");
    assert_eq!(parsed, Ok("a"))
}

#[test]
fn parse_identifier_whitespaced_singlequotes() {
    let parsed = super::grammar::enclosed_identifier("' a '");
    assert_eq!(parsed, Ok("a"))
}

#[test]
fn parse_identifier_underscore() {
    let parsed = super::grammar::enclosed_identifier("\"a_b\"");
    assert_eq!(parsed, Ok("a_b"))
}

#[test]
fn parse_identifier_dash() {
    let parsed = super::grammar::enclosed_identifier("\"a-b\"");
    assert_eq!(parsed, Ok("a-b"))
}

#[test]
fn parse_selector_simple_doublequotes() {
    let parsed = super::grammar::selector("\"a\"");
    assert_eq!(
        parsed,
        Ok(CssSelectorList::new(vec![CssSelectorPath::single(
            CssSelector::for_element("a")
        )]))
    )
}
/*
#[test]
fn parse_selector_simple_singlequotes() {
    let parsed = super::grammar::selector("'a'");
    assert_eq!(parsed, Ok("a"))
}

#[test]
fn parse_selector_whitespaced_doublequotes() {
    let parsed = super::grammar::selector("\" a \"");
    assert_eq!(parsed, Ok("a"))
}

#[test]
fn parse_selector_whitespaced_singlequotes() {
    let parsed = super::grammar::selector("' a '");
    assert_eq!(parsed, Ok("a"))
}

#[test]
fn parse_selector_whitespaced_questionsmarks() {
    let parsed = super::grammar::selector("? a ?");
    assert_eq!(parsed, Ok("a"))
}

#[test]
fn parse_selector_doublequoted_cant_have_doublequotes() {
    let parsed = super::grammar::selector("\"a\"b\"");
    assert!(parsed.is_err())
}

#[test]
fn parse_selector_chained_singlequotes() {
    let parsed = super::grammar::selector("'a.b c > d[data-test=\"foo\"]'");
    assert_eq!(parsed, Ok("a.b c > d[data-test=\"foo\"]"))
}

#[test]
fn parse_selector_chained_questionsmarks() {
    let parsed = super::grammar::selector("?a.b c > d[data-test=\"foo\"]?");
    assert_eq!(parsed, Ok("a.b c > d[data-test=\"foo\"]"))
}

#[test]
fn parse_single_only() {
    let parsed = super::grammar::only_command("(ONLY 'a')");
    assert_eq!(parsed, Ok(super::Command::Only(String::from("a"))));
}

#[test]
fn parse_single_filter() {
    let parsed = super::grammar::filter_command("(FILTER 'a')");
    assert_eq!(parsed, Ok(super::Command::Filter(String::from("a"))));
}

#[test]
fn parse_two_grammar() {
    let parsed = super::grammar::pipeline("(ONLY 'a') | (FILTER 'b')");
    assert_eq!(
        parsed,
        Ok(Pipeline::new(vec![
            super::Command::Only(String::from("a")),
            super::Command::Filter(String::from("b")),
        ]))
    );
}
*/
