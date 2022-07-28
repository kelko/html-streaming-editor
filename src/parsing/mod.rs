#[cfg(test)]
mod tests;

use crate::{
    Command, CssAttributeComparison, CssAttributeSelector, CssPseudoClass, CssSelector,
    CssSelectorList, CssSelectorPath, CssSelectorStep, Pipeline,
};
use peg::parser;

fn build_css_path<'a>(
    first: CssSelectorStep<'a>,
    rest: Option<Vec<CssSelectorStep<'a>>>,
) -> Vec<CssSelectorStep<'a>> {
    let mut result = vec![first];

    if let Some(mut rest_content) = rest {
        result.append(&mut rest_content);
    }

    return result;
}

parser! {
  pub grammar grammar() for str {
        rule whitespace()
            = quiet!{[' ' | '\n' | '\t']+}
        rule number() -> usize
            = n:$(['0'..='9']+) { n.parse().unwrap() }
        pub(crate) rule identifier() -> &'input str
            = i:$(['a'..='z' | 'A'..='Z' | '0'..='9' | '-' | '_' ]+) { i }
        rule css_attribute() -> CssAttributeSelector<'input>
            = "[" a:(identifier()) "]" { CssAttributeSelector::<'input> { attribute: a, operator: CssAttributeComparison::Exist, value: None } }
            / "[" a:(identifier()) "=" v:(string_value()) "]" { CssAttributeSelector::<'input> { attribute: a, operator: CssAttributeComparison::EqualsExact, value: Some(v) } }
            / "[" a:(identifier()) "|=" v:(string_value()) "]" { CssAttributeSelector::<'input> { attribute: a, operator: CssAttributeComparison::EqualsTillHyphen, value: Some(v) } }
            / "[" a:(identifier()) "^=" v:(string_value()) "]" { CssAttributeSelector::<'input> { attribute: a, operator: CssAttributeComparison::Starts, value: Some(v) } }
            / "[" a:(identifier()) "$=" v:(string_value()) "]" { CssAttributeSelector::<'input> { attribute: a, operator: CssAttributeComparison::Ends, value: Some(v) } }
            / "[" a:(identifier()) "*=" v:(string_value()) "]" { CssAttributeSelector::<'input> { attribute: a, operator: CssAttributeComparison::CharacterContains, value: Some(v) } }
            / "[" a:(identifier()) "~=" v:(string_value()) "]" { CssAttributeSelector::<'input> { attribute: a, operator: CssAttributeComparison::TermContains, value: Some(v) } }
        rule css_attributes() -> Vec<CssAttributeSelector<'input>>
            = a:(css_attribute() ++ "") { a }
        rule css_class() -> &'input str
            = "." c:(identifier()) { c }
        rule css_classes() -> Vec<&'input str>
            = c:(css_class() ++ "") { c }
        rule css_pseudo_class() -> CssPseudoClass
            = ":first-child" { CssPseudoClass::FirstChild }
            / ":first-of-type" { CssPseudoClass::FirstOfType }
            / ":nth-child(" i:(number()) ")" { CssPseudoClass::NthChild(i) }
            / ":nth-of-type(" i:(number()) ")" { CssPseudoClass::NthOfType(i) }
        rule css_pseudo_classes() -> Vec<CssPseudoClass>
            = p:(css_pseudo_class() ++ "") { p }
        rule css_id() -> &'input str
            = "#" i:(identifier()) { i }
        pub(crate) rule css_selector() -> CssSelector<'input>
            = e:(identifier())i:(css_id())?c:(css_classes()?)p:(css_pseudo_classes())?a:(css_attributes()?) { CssSelector{element:Some(e), id: i, classes: c.unwrap_or(vec![]), pseudo_classes: p.unwrap_or(vec![]), attributes: a.unwrap_or(vec![])} }
            / i:(css_id())c:(css_classes())?p:(css_pseudo_classes())?a:(css_attributes()?) { CssSelector{element:None, id: Some(i), classes: c.unwrap_or(vec![]), pseudo_classes: p.unwrap_or(vec![]), attributes: a.unwrap_or(vec![])} }
            / c:(css_classes())p:(css_pseudo_classes())?a:(css_attributes()?) { CssSelector{element:None, id: None, classes: c, pseudo_classes: p.unwrap_or(vec![]), attributes: a.unwrap_or(vec![])} }
            / p:(css_pseudo_classes())a:(css_attributes())? { CssSelector{element:None, id: None, classes: vec![], pseudo_classes: p, attributes: a.unwrap_or(vec![])} }
            / a:(css_attributes()) { CssSelector{element:None, id: None, classes: vec![], pseudo_classes: vec![], attributes: a} }
        rule css_selector_step() -> Vec<CssSelectorStep<'input>>
            = " "? ">" " "? s:(css_selector()) l:(css_selector_step())? { build_css_path(CssSelectorStep::direct_child(s), l) }
            / " "? "+" " "? s:(css_selector()) l:(css_selector_step())? { build_css_path(CssSelectorStep::sibling(s), l) }
            / " " s:(css_selector()) l:(css_selector_step())? { build_css_path(CssSelectorStep::descendent(s), l) }
        pub(crate) rule css_selector_path() -> CssSelectorPath<'input>
            = whitespace()? f:(css_selector()) l:(css_selector_step())? whitespace()?{ CssSelectorPath::new(f, l.unwrap_or(vec![]))  }
        pub(crate) rule css_selector_list() -> CssSelectorList<'input>
            = v:(css_selector_path() ++ ",") { CssSelectorList::new(v) }

        pub(crate) rule string_value() -> &'input str
            = "\"" whitespace()? s:$([^'"']+) "\"" { s.trim() }
            / "'" whitespace()? s:$([^'\'']+) "'" { s.trim() }
            / "?" whitespace()? s:$([^'?']+) "?" { s.trim() }
        pub(crate) rule enclosed_identifier() -> &'input str
            = "\"" whitespace()? s:(identifier()) whitespace()? "\"" { s }
            / "'" whitespace()? s:(identifier())  whitespace()? "'" { s }
            / "?" whitespace()? s:(identifier())  whitespace()? "?" { s }
        pub(crate) rule selector() -> CssSelectorList<'input>
            = "\"" c:(css_selector_list()) "\"" { c }
            / "'" c:(css_selector_list()) "'" { c }
            / "?" c:(css_selector_list()) "?" { c }
        pub(crate) rule only_command() -> Command<'input>
            = "(ONLY " oc:selector() whitespace()? ")" { Command::Only(oc) }
        pub(crate) rule filter_command() -> Command<'input>
            = "(FILTER " oc:selector() whitespace()? ")" { Command::Filter(oc) }
        rule command() -> Command<'input>
            = only_command()
            / filter_command()
        pub rule pipeline() -> Pipeline<'input>
            = p:(command() ** " | ") { Pipeline::new(p) }
  }
}
