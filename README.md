HTML Streaming Editor
=======================

![GitHub](https://img.shields.io/github/license/kelko/html-streaming-editor)
![GitHub Workflow Status](https://img.shields.io/github/workflow/status/kelko/html-streaming-editor/Rust)
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

Commands
-------------

Currently supported:

- `ONLY`: remove everything not matching the CSS selector (alias: `SELECT`)
- `WITHOUT`: remove everything matching the CSS selector (alias: `FILTER`)
- `FOR-EACH`: run a sub-pipeline on all sub-elements matching a CSS selector but return the previously selected elements (alias: `FOR`)
- `CLEAR-ATTR`: removes a given attribute from the previously selected elements  
- `CLEAR-CONTENT`: clears all children from the previously selected elements
- `SET-ATTR`: Sets a given attribute to a specified value
- `SET-TEXT-CONTENT`: removes previous children and replaces it with exactly one given text child
- `ADD-TEXT-CONTENT`: appends a new text child
- `ADD-COMMENT`: appends a new comment child
- `ADD-ELEMENT`: appends a new tag/element child
- `REPLACE`: replace all elements matching a CSS selector with new elements (alias: `MAP`)
- `CREATE-ELEMENT`: creates a new, empty element, mainly in combination with `ADD-ELEMENT` or `REPLACE` (alias: `NEW`)
- `READ-FROM`: reads a DOM from a different file, mainly in combination with `ADD-ELEMENT` or `REPLACE` (alias: `SOURCE`) 

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
hse -i index.html 'REPLACE{.placeholder ↤ READ-FROM{"other.html"} | ONLY{div.content} }'

# add a new <meta name="version" value=""> element to <head> with git version info 
hse -i index.html "FOR{head ↦ ADD-ELEMENT{ CREATE-ELEMENT{meta} | SET-ATTR{name ↤ 'version'} | SET-ATTR{content ↤ '`git describe --tags`'}  } }"

# add a new comment to <body> with git version info
hse -i index.html "FOR{body ↦ ADD-COMMENT{'`git describe --tags`'}}"
```
