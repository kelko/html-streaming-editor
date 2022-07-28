HTML Streaming Editor
=======================

Run (simple) manipulations on HTML files, like extracting parts.
Use CSS selectors to define which parts of the HTML to operator on,
use different commands in pipes to perform the desired operations.

Syntax
----------

The basic syntax is:
> (COMMAND SELECTOR) | (COMMAND SELECTOR)

Some `COMMAND` use sub-pipelines, those would be written as follows:
> (COMMAND SELECTOR {(COMMAND SELECTOR) | (COMMAND SELECTOR)})

The `SELECTOR` is a [CSS selector](https://developer.mozilla.org/en-US/docs/Web/CSS/CSS_Selectors), wrapped in either `"`, `'` 
or `?` (as to reduce the necessity of escaping to a minimum).

Commands
-------------

Planned commands:
- ONLY: remove everything not matching the CSS selector
- FILTER: remove everything matching the CSS selector
- MAP: run the nodes matching the CSS selector through a sub-pipeline and replace them with the result of that pipeline
- Some attribute & text-content manipulation

Currently supported:
- ONLY
- FILTER is in code, but mis-behaves

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
# fetches all elements with CSS class "content" inside a <header> element
hse -i index.html '(ONLY ?header .content?)'
```
