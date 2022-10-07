use crate::html::HtmlTag;
use crate::{load_html_file, CommandError, CssSelectorList, HtmlContent};
use log::trace;

#[derive(Debug, PartialEq, Clone)]
pub enum ElementCreatingCommand<'a> {
    /// creates an HTML element of given type
    /// Returns the created element as result.
    CreateElement(&'a str),
    /// reads a different file into memory
    /// Returns the content of that file as result.
    FromFile(&'a str),
    /// Starting at the element being replaced run a sub-query
    /// Returns all sub-elements that match the given CSS selector.
    FromReplaced(CssSelectorList<'a>),
}

impl<'a> ElementCreatingCommand<'a> {
    /// perform the action defined by the command on the set of nodes
    /// and return the calculated results.
    /// For some command the output can be equal to the input,
    /// others change the result-set
    pub(crate) fn execute(
        &self,
        input: &Vec<rctree::Node<HtmlContent>>,
    ) -> Result<Vec<rctree::Node<HtmlContent>>, CommandError> {
        match self {
            ElementCreatingCommand::CreateElement(element_name) => {
                Self::create_element(element_name)
            }
            ElementCreatingCommand::FromFile(file_path) => Self::from_file(file_path),
            ElementCreatingCommand::FromReplaced(selector) => Self::from_replaced(input, selector),
        }
    }

    fn create_element(name: &str) -> Result<Vec<rctree::Node<HtmlContent>>, CommandError> {
        trace!("Running CREATE-ELEMENT command using name: {:#?}", name);

        Ok(vec![rctree::Node::new(HtmlContent::Tag(HtmlTag::of_name(
            name.clone(),
        )))])
    }

    fn from_file(file_path: &str) -> Result<Vec<rctree::Node<HtmlContent>>, CommandError> {
        trace!("Running FROM-FILE command using file: {:#?}", file_path);

        let mut root_element = load_html_file(file_path)?;
        Ok(vec![root_element.make_deep_copy()])
    }

    fn from_replaced(
        input: &Vec<rctree::Node<HtmlContent>>,
        selector: &CssSelectorList<'a>,
    ) -> Result<Vec<rctree::Node<HtmlContent>>, CommandError> {
        trace!("Running FROM-REPLACED command");
        Ok(selector
            .query(input)
            .iter()
            .map(|e| rctree::Node::clone(e).make_deep_copy())
            .collect::<Vec<_>>())
    }
}
