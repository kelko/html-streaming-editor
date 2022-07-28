HTML Streaming Editor
=======================

Run (simple) manipulations on HTML files, like extracting parts.
Use CSS selectors to define which parts of the HTML to operator on,
use different commands in pipes to perform the desired operations.

Planned commands:
- ONLY: remove everything not matching the CSS selector
- FILTER: remove everything matching the CSS selector
- MAP: run the nodes matching the CSS selector through a sub-pipeline and replace them with the result of that pipeline
- Some attribute & text-content manipulation

Currently supported:
- ONLY
- FILTER is in code, but mis-behaves
