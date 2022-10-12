use peg::parser;

use crate::{
    element_creating::{ElementCreatingCommand, ElementCreatingPipeline},
    element_processing::{ElementProcessingCommand, ElementProcessingPipeline},
    string_creating::{
        ElementSelectingCommand, StringValueCreatingPipeline, ValueExtractingCommand,
    },
    CssAttributeComparison, CssAttributeSelector, CssPseudoClass, CssSelector, CssSelectorList,
    CssSelectorPath, CssSelectorStep, ValueSource,
};

#[cfg(test)]
mod tests;

/// utility method to "prepend" the first found CSS selector step before the following list,
/// generated by the recursive PEG rule
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
        rule assign_marker()
            = "↤"
            / "<="
        rule iterate_marker()
            = "↦"
            / "=>"
        rule number() -> usize
            = n:$(['0'..='9']+) { n.parse().unwrap() }
        pub(super) rule identifier() -> &'input str
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
            / ":last-child" { CssPseudoClass::LastChild }
            / ":last-of-type" { CssPseudoClass::LastOfType }
            / ":nth-last-child(" i:(number()) ")" { CssPseudoClass::NthLastChild(i) }
            / ":nth-last-of-type(" i:(number()) ")" { CssPseudoClass::NthLastOfType(i) }
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
            / " "? "~" " "? s:(css_selector()) l:(css_selector_step())? { build_css_path(CssSelectorStep::general_sibling(s), l) }
            / " "? "+" " "? s:(css_selector()) l:(css_selector_step())? { build_css_path(CssSelectorStep::adjacent_sibling(s), l) }
            / " " s:(css_selector()) l:(css_selector_step())? { build_css_path(CssSelectorStep::descendent(s), l) }
        pub(crate) rule css_selector_path() -> CssSelectorPath<'input>
            = whitespace()? f:(css_selector()) l:(css_selector_step())? whitespace()?{ CssSelectorPath::new(f, l.unwrap_or(vec![]))  }
        pub(crate) rule css_selector_list() -> CssSelectorList<'input>
            = v:(css_selector_path() ++ ",") { CssSelectorList::new(v) }

        pub(super) rule string_value() -> &'input str
            = "\"" s:$([^'"']+) "\"" { s }
            / "'" s:$([^'\'']+) "'" { s }
            / "?" s:$([^'?']+) "?" { s }

        rule value_source() -> ValueSource<'input>
            = v:string_value() { ValueSource::StringValue(v) }
            / p:string_creating_pipeline() { ValueSource::SubPipeline(p) }

        rule extract_element_command() -> ElementProcessingCommand<'input>
            = ("EXTRACT-ELEMENT" / "ONLY") "{" whitespace()?  oc:css_selector_list() whitespace()? "}" { ElementProcessingCommand::ExtractElement(oc) }
        rule remove_element_command() -> ElementProcessingCommand<'input>
            = ("REMOVE-ELEMENT" / "WITHOUT") "{" whitespace()? oc:css_selector_list() whitespace()? "}" { ElementProcessingCommand::RemoveElement(oc) }
        rule for_each_command() -> ElementProcessingCommand<'input>
            = ("FOR-EACH"/"WITH") "{" whitespace()? oc:css_selector_list() whitespace()? iterate_marker() whitespace()? sp:pipeline() whitespace()?  "}" { ElementProcessingCommand::ForEach(oc, sp) }
        rule replace_command() -> ElementProcessingCommand<'input>
            = ("REPLACE"/"MAP") "{" whitespace()? oc:css_selector_list() whitespace()? assign_marker() whitespace()? sp:element_subselect_or_creating_pipeline() whitespace()? "}" { ElementProcessingCommand::Replace(oc, sp)}
        rule clear_attr_command() -> ElementProcessingCommand<'input>
            = "CLEAR-ATTR{" whitespace()? a:identifier() whitespace()? "}" { ElementProcessingCommand::ClearAttribute(a) }
        rule clear_content_command() -> ElementProcessingCommand<'input>
            = "CLEAR-CONTENT" { ElementProcessingCommand::ClearContent }
        rule set_attr_command() -> ElementProcessingCommand<'input>
            = "SET-ATTR{" whitespace()? a:identifier() whitespace()? assign_marker() whitespace()? v:value_source() whitespace()? "}" { ElementProcessingCommand::SetAttribute(a, v) }
        rule set_text_content_command() -> ElementProcessingCommand<'input>
            = "SET-TEXT-CONTENT{" whitespace()? (assign_marker() whitespace()?)? v:value_source() whitespace()? "}" { ElementProcessingCommand::SetTextContent(v) }
        rule add_text_content_command() -> ElementProcessingCommand<'input>
            = "ADD-TEXT-CONTENT{" whitespace()? (assign_marker() whitespace()?)? v:value_source() whitespace()? "}" { ElementProcessingCommand::AddTextContent(v) }
        rule add_comment_command() -> ElementProcessingCommand<'input>
            = "ADD-COMMENT{" whitespace()? (assign_marker() whitespace()?)? v:value_source() whitespace()? "}" { ElementProcessingCommand::AddComment(v) }
        rule add_element_command() -> ElementProcessingCommand<'input>
            = "ADD-ELEMENT{" whitespace()? (assign_marker() whitespace()?)? sp:element_creating_pipeline() whitespace()?  "}" { ElementProcessingCommand::AddElement(sp) }
        pub(super) rule element_processing_command() -> ElementProcessingCommand<'input>
            = extract_element_command()
            / remove_element_command()
            / for_each_command()
            / clear_attr_command()
            / clear_content_command()
            / set_attr_command()
            / set_text_content_command()
            / add_text_content_command()
            / add_comment_command()
            / add_element_command()
            / replace_command()

        rule create_element_command() -> ElementCreatingCommand<'input>
            = ("CREATE-ELEMENT"/"NEW") "{" whitespace()? n:identifier() whitespace()? "}" { ElementCreatingCommand::CreateElement(n)}
        rule from_file_command() -> ElementCreatingCommand<'input>
            = ("FROM-FILE"/"SOURCE") "{" whitespace()? f:string_value() whitespace()? "}" { ElementCreatingCommand::FromFile(f) }
        rule element_creating_command() -> ElementCreatingCommand<'input>
            = create_element_command()
            / from_file_command()
        rule element_creating_pipeline() -> ElementCreatingPipeline<'input>
            = s:element_creating_command() p:element_manipulating_subpipeline()? { ElementCreatingPipeline::new(s, p) }
        rule element_manipulating_subpipeline() -> Vec<ElementProcessingCommand<'input>>
            = " | " p:(element_processing_command() ** " | ") { p }

        rule query_replaced_command() -> ElementCreatingCommand<'input>
            = ("QUERY-REPLACED"/"KEEP") "{" whitespace()? oc:css_selector_list() whitespace()? "}" { ElementCreatingCommand::FromReplaced(oc) }
        rule element_subselect_or_creating_category() -> ElementCreatingCommand<'input>
            = query_replaced_command()
            / element_creating_command()
        rule element_subselect_or_creating_pipeline() -> ElementCreatingPipeline<'input>
            = s:element_subselect_or_creating_category() p:element_manipulating_subpipeline()? { ElementCreatingPipeline::new(s, p) }

        rule use_element_command() -> ElementSelectingCommand<'input>
            = ("USE-ELEMENT"/"THIS") { ElementSelectingCommand::UseElement }
        rule use_parent_command() -> ElementSelectingCommand<'input>
            = ("USE-PARENT"/"PARENT") { ElementSelectingCommand::UseParent }
        rule query_element_command() -> ElementSelectingCommand<'input>
            = "QUERY-ELEMENT{" whitespace()? oc:css_selector_list() whitespace()? "}" { ElementSelectingCommand::QueryElement(oc) }
        rule query_parent_command() -> ElementSelectingCommand<'input>
            = "QUERY-PARENT{" whitespace()? oc:css_selector_list() whitespace()? "}" { ElementSelectingCommand::QueryParent(oc) }
        rule query_root_command() -> ElementSelectingCommand<'input>
            = "QUERY-ROOT{" whitespace()? oc:css_selector_list() whitespace()? "}" { ElementSelectingCommand::QueryRoot(oc) }
        pub(super) rule element_selecting_command() -> ElementSelectingCommand<'input>
            = use_element_command()
            / use_parent_command()
            / query_element_command()
            / query_parent_command()
            / query_root_command()
        rule get_attr_command() -> ValueExtractingCommand<'input>
            = "GET-ATTR{" whitespace()? a:identifier() whitespace()? "}" { ValueExtractingCommand::GetAttribute(a) }
        rule get_text_content_command() -> ValueExtractingCommand<'input>
            = "GET-TEXT-CONTENT" { ValueExtractingCommand::GetTextContent }
        pub(super) rule value_extracting_command() -> ValueExtractingCommand<'input>
            = get_attr_command()
            / get_text_content_command()
        pub(super) rule string_creating_pipeline() -> StringValueCreatingPipeline<'input>
            = s:element_selecting_command() " | " e:value_extracting_command() { StringValueCreatingPipeline::new(s, e) }

        pub(crate) rule pipeline() -> ElementProcessingPipeline<'input>
            = p:(element_processing_command() ** " | ") { ElementProcessingPipeline::new(p) }
  }
}
