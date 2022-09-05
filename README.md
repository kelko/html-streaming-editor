HTML Streaming Editor
=======================

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

- "map"/"replace": For each (sub) node matching the inner selector the sub-pipeline is processed, and the pipelines result is used instead of the element

  > COMMAND{ SELECTOR ↤ COMMAND{ SELECTOR } | COMMAND { SELECTOR } }
 
The `SELECTOR` is a [CSS selector](https://developer.mozilla.org/en-US/docs/Web/CSS/CSS_Selectors).

Commands
-------------

Planned commands:

- `ONLY`: remove everything not matching the CSS selector (alias: `SELECT`)
- `WITHOUT`: remove everything matching the CSS selector (alias: `FILTER`)
- Some attribute & text-content manipulation

Currently supported:

- `ONLY`
- `WITHOUT`

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
hse -i index.html 'ONLY{header .content}'
```

```shell
# fetches the `<main>` or element with CSS class `main`, but without any `<script>` defined inside
hse -i index.html 'ONLY{main, .main} | WITHOUT{script}'
```
