use crate::html::HtmlTag;
use crate::{load_html_file, CommandError, CssSelectorList, HtmlContent};
use log::trace;

#[derive(Debug, Clone, PartialEq, Eq)]
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
        input: &[rctree::Node<HtmlContent>],
    ) -> Result<Vec<rctree::Node<HtmlContent>>, CommandError> {
        match self {
            ElementCreatingCommand::CreateElement(element_name) => {
                Self::create_element(element_name)
            }
            ElementCreatingCommand::FromFile(file_path) => Self::load_file(file_path),
            ElementCreatingCommand::FromReplaced(selector) => Self::query_replaced(input, selector),
        }
    }

    fn create_element(name: &str) -> Result<Vec<rctree::Node<HtmlContent>>, CommandError> {
        trace!("Running CREATE-ELEMENT command using name: {:#?}", name);

        Ok(vec![rctree::Node::new(HtmlContent::Tag(HtmlTag::of_name(
            <&str>::clone(&name),
        )))])
    }

    fn load_file(file_path: &str) -> Result<Vec<rctree::Node<HtmlContent>>, CommandError> {
        trace!("Running LOAD-FILE command using file: {:#?}", file_path);

        let root_element = load_html_file(file_path)?;
        Ok(vec![root_element.make_deep_copy()])
    }

    fn query_replaced(
        input: &[rctree::Node<HtmlContent>],
        selector: &CssSelectorList<'a>,
    ) -> Result<Vec<rctree::Node<HtmlContent>>, CommandError> {
        trace!("Running QUERY-REPLACED command");
        Ok(selector
            .query(input)
            .iter()
            .map(|e| rctree::Node::clone(e).make_deep_copy())
            .collect::<Vec<_>>())
    }
}

#[cfg(test)]
mod tests {
    use crate::element_creating::ElementCreatingCommand;
    use crate::html::HtmlTag;
    use crate::{
        load_inline_html, CssSelector, CssSelectorList, CssSelectorPath, HtmlContent,
        HtmlRenderable,
    };
    use std::collections::BTreeMap;

    #[test]
    fn create_element_builds_new_element_on_empty_input() {
        let command = ElementCreatingCommand::CreateElement("div");

        let mut result = command.execute(&vec![]).unwrap();

        assert_eq!(result.len(), 1);

        let first_result = result.pop().unwrap();
        let first_result = first_result.borrow();
        assert_eq!(*first_result, HtmlContent::Tag(HtmlTag::of_name("div")));
    }

    #[test]
    fn create_element_builds_new_element_ignoring_input() {
        let command = ElementCreatingCommand::CreateElement("div");

        let root = rctree::Node::new(HtmlContent::Tag(HtmlTag::of_name("html")));

        let mut result = command.execute(&vec![root]).unwrap();

        assert_eq!(result.len(), 1);

        let first_result = result.pop().unwrap();
        let first_result = first_result.borrow();
        assert_eq!(*first_result, HtmlContent::Tag(HtmlTag::of_name("div")));
    }

    #[test]
    fn load_file_read_file_content() {
        let command = ElementCreatingCommand::FromFile("tests/source.html");
        let mut result = command.execute(&vec![]).unwrap();

        assert_eq!(result.len(), 1);

        let first_result = result.pop().unwrap();
        assert_eq!(
            first_result.outer_html(),
            r#"<html lang="en">
<head>
    <meta charset="UTF-8">
    <title>LOAD-FILE Source</title>
</head>
<body>
    <div>Some other stuff</div>
    <ul id="first">
        <li>1</li>
        <li>2</li>
        <li>3</li>
    </ul>
    <ul id="second">
        <li>a</li>
        <li><!-- Some Comment -->b</li>
        <li><em class="intense">c</em></li>
    </ul>
    <!-- not taken into account -->
</body>
</html>"#
        );
    }

    #[test]
    fn query_replaced_returns_matching_descendent_of_input() {
        let command = ElementCreatingCommand::FromReplaced(CssSelectorList::new(vec![
            CssSelectorPath::single(CssSelector::for_class("test-source")),
        ]));
        let root = load_inline_html(
            r#"<div id="replaced"><p class="first"></p><aside class="test-source"></aside></div>"#,
        );

        let mut result = command.execute(&vec![root]).unwrap();

        assert_eq!(result.len(), 1);

        let first_result = result.pop().unwrap();
        let first_result = first_result.borrow();
        assert_eq!(
            *first_result,
            HtmlContent::Tag(HtmlTag {
                name: String::from("aside"),
                attributes: BTreeMap::<String, String>::from([(
                    String::from("class"),
                    String::from("test-source")
                )])
            })
        );
    }

    #[test]
    fn query_replaced_returns_all_matching_descendents_of_input() {
        let command = ElementCreatingCommand::FromReplaced(CssSelectorList::new(vec![
            CssSelectorPath::single(CssSelector::for_class("test-source")),
        ]));
        let root = load_inline_html(
            r#"<div id="replaced">
    <p class="first">
        <em class="test-source">Content 1</em>
    </p>
    <aside class="test-source">Content 2</aside>
    <div>
        <div></div>
        <div><img src="" class="test-source"></div>
        <div></div>
    </div>
</div>"#,
        );

        let result = command.execute(&vec![root]).unwrap();
        let result = result.iter().map(|n| n.outer_html()).collect::<Vec<_>>();

        assert_eq!(result.len(), 3);
        assert!(result.contains(&String::from(r#"<em class="test-source">Content 1</em>"#)));
        assert!(result.contains(&String::from(
            r#"<aside class="test-source">Content 2</aside>"#
        )));
        assert!(result.contains(&String::from(r#"<img class="test-source" src="">"#)));
    }

    #[test]
    fn query_replaced_returns_empty_on_no_match() {
        let command = ElementCreatingCommand::FromReplaced(CssSelectorList::new(vec![
            CssSelectorPath::single(CssSelector::for_class("test-source")),
        ]));
        let root =
            load_inline_html(r#"<div id="replaced"><p class="first"></p><aside></aside></div>"#);

        let result = command.execute(&vec![root]).unwrap();

        assert_eq!(result.len(), 0);
    }

    #[test]
    fn query_replaced_returns_empty_on_empty_input() {
        let command = ElementCreatingCommand::FromReplaced(CssSelectorList::new(vec![
            CssSelectorPath::single(CssSelector::for_class("test-source")),
        ]));

        let result = command.execute(&vec![]).unwrap();

        assert_eq!(result.len(), 0);
    }
}
