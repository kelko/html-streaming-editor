HTML Streaming Editor
=======================

[![License: MIT](https://img.shields.io/github/license/kelko/html-streaming-editor)](https://github.com/kelko/html-streaming-editor/blob/main/LICENSE)
[![GitHub Workflow Status](https://img.shields.io/github/workflow/status/kelko/html-streaming-editor/Rust)](https://github.com/kelko/html-streaming-editor/actions/workflows/main.yml)
[![docs.rs](https://img.shields.io/docsrs/html-streaming-editor?style=flat-square])](https://docs.rs/html-streaming-editor/)
[![Crates.io](https://img.shields.io/crates/v/html-streaming-editor)](https://crates.io/crates/html-streaming-editor)


Run (simple) manipulations on HTML files, like extracting parts.
Use CSS selectors to define which parts of the HTML to operator on,
use different commands in pipes to perform the desired operations.

Syntax
----------

The basic syntax is:
> COMMAND{ SELECTOR } | COMMAND{ SELECTOR }

Some `COMMAND` use sub-pipelines. There are two kind of `COMMANDS` with this:
- "iterate"/"forEach": For each (sub) node matching the inner selector the sub-pipeline is processed, but the elements themselves are not changed

  > COMMAND{ SELECTOR ↦ COMMAND{ SELECTOR } | COMMAND { SELECTOR } }
  > 
  > COMMAND{ SELECTOR => COMMAND{ SELECTOR } | COMMAND { SELECTOR } }

- "map"/"replace": For each (sub) node matching the inner selector the sub-pipeline is processed, and the pipelines result is used instead of the element

  > COMMAND{ SELECTOR ↤ COMMAND{ SELECTOR } | COMMAND { SELECTOR } }
  > 
  > COMMAND{ SELECTOR <= COMMAND{ SELECTOR } | COMMAND { SELECTOR } }
 
The `SELECTOR` is a [CSS selector](https://developer.mozilla.org/en-US/docs/Web/CSS/CSS_Selectors).

Pipeline Types
-----------------

There are three types of pipelines:

- element processing pipeline: The default. You have some input HTML which you run through the pipeline
- element creating sub-pipeline: special sub-pipeline wherever a commands adds one or more elements into the HTML tree (or into a different place of said tree)
- string value creating sub-pipeline: special sub-pipeline wherever a commands set a string value (text content, comment, attribute value)


Commands
-------------

Currently supported element processing commands:

- `EXTRACT-ELEMENT`: remove everything not matching the CSS selector (alias: `ONLY`)
- `REMOVE-ELEMENT`: remove everything matching the CSS selector (alias: `WITHOUT`)
- `FOR-EACH`: run a sub-pipeline on all sub-elements matching a CSS selector but return the previously selected elements (alias: `WITH`)
- `CLEAR-ATTR`: removes a given attribute from the previously selected elements  
- `CLEAR-CONTENT`: clears all children from the previously selected elements
- `SET-ATTR`: Sets a given attribute to a specified value
- `SET-TEXT-CONTENT`: removes previous children and replaces it with exactly one given text child
- `ADD-TEXT-CONTENT`: appends a new text child
- `ADD-COMMENT`: appends a new comment child
- `ADD-ELEMENT`: appends a new tag/element child
- `REPLACE`: replace all elements matching a CSS selector with new elements (alias: `MAP`)

Currently supported element creating commands:

- `CREATE-ELEMENT`: creates a new, empty element, mainly in combination with `ADD-ELEMENT` or `REPLACE` (alias: `NEW`)
- `LOAD-FILE`: reads a DOM from a different file, mainly in combination with `ADD-ELEMENT` or `REPLACE` (alias: `SOURCE`)
- `QUERY-REPLACED`: returns children matching the CSS selector of those elements meant to be replaced, only combination with or `REPLACE` (alias: `KEEP`)

Currently supported string-value creating commands:

- `USE-ELEMENT`: returns the currently selected element for a sub-pipeline, mainly in combination with "string value producing pipelines" (alias: `THIS`)
- `USE-PARENT`: returns the parent of the currently selected element for a sub-pipeline, mainly in combination with "string value producing pipelines" (alias: `PARENT`)
- `QUERY-ELEMENT`: runs a query on the currently selected element for a sub-pipeline, without detaching target element from HTML tree unlike `EXTRACT-ELEMENT`
- `QUERY-PARENT`: runs a query on the parent of the currently selected element for a sub-pipeline, without detaching target element from HTML tree unlike `EXTRACT-ELEMENT`
- `QUERY-ROOT`: runs a query on the root of the currently selected element for a sub-pipeline, without detaching target element from HTML tree unlike `EXTRACT-ELEMENT`
- `GET-ATTR`: returns the value of an attribute of the currently selected element for a string-value producing pipelines
- `GET-TEXT-CONTENT`: returns the text content of the currently selected element for a string-value producing pipelines
- `REGEX-REPLACE`: runs a RegEx-based value replacements on the current string value of the pipeline
- `TO-LOWER`: all-lower the current string value of the pipeline
- `TO-UPPER`: all-caps the current string value of the pipeline
- `ADD-PREFIX`: add a given string to the beginning of the string value of the pipeline
- `ADD-SUFFIX`: add a given string to the end of the string value of the pipeline


Binary
-------

The binary is called `hse` and supports following options:

```
USAGE:
    hse [OPTIONS] <COMMANDS>

ARGS:
    <COMMANDS>    Single string with the command pipeline to perform

OPTIONS:
    -h, --help               Print help information
    -i, --input <input>      File name of the Input. `-` for stdin (default)
    -o, --output <output>    File name of the Output. `-` for stdout (default)
    -V, --version            Print version information
```

Example
--------

```shell
# fetches all elements with CSS class "content" inside a <main> element
hse -i index.html 'ONLY{main .content}'

# fetches the `<main>` or element with CSS class `main`, but without any `<script>` defined inside
hse -i index.html 'ONLY{main, .main} | WITHOUT{script}'

# replaces all elements with `placeholder` class with the <div class="content"> from a second HTML file 
hse -i index.html 'REPLACE{.placeholder ↤ SOURCE{"other.html"} | ONLY{div.content} }'

# add a new <meta name="version" value=""> element to <head> with git version info 
hse -i index.html "WITH{head ↦ ADD-ELEMENT{ NEW{meta} | SET-ATTR{name ↤ 'version'} | SET-ATTR{content ↤ '`git describe --tags`'}  } }"

# add a new comment to <body> with git version info
hse -i index.html "WITH{body ↦ ADD-COMMENT{'`git describe --tags`'}}"

# add an RDF <meta name="dc:title"> with same content as <title>
hse -i input.html "WITH{head ↦ ADD-ELEMENT{ NEW{meta} | SET-ATTR{name ↤ 'dc:title' } } | WITH{meta[name='dc:title'] ↦ SET-ATTR{content ↤ QUERY-PARENT{title} | GET-TEXT-CONTENT } } }"

# replace non-word characters with an underscore in an attribute
hse -i index.html "EXTRACT-ELEMENT{#target} | SET-ATTR{data-test ↤ USE-ELEMENT | GET-ATTR{data-test} | REGEX-REPLACE{'\\W' ↤ '_'} }";"
```
