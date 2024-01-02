# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- Added this changelog file 

### Changed

- Updated dependencies.

## [0.8.0] - 2023-05-13

### Changed

- Now output is only opened after processing is complete, so hse can read from and write into same file

## [0.7.1] - 2022-10-19

### Added

- Support CRLF Newline

  Additionally to simple space, tab and Unix new line (\n) also support Windows new line (\r\n) wherever whitespaces are supported

## [0.7.0] - 2022-10-17

### Added
- Support for DOCTYPE

  If a `<!DOCTYPE >` tag is present in the input it will be kept and
  written into the output, unless `EXTRACT-ELEMENT` is used to return only
  parts.

  ⚠️ KNOWN ISSUE ⚠️
  If present _any_ DOCTYPE will be interpreted as HTML5. Originates in
  underlying lib.

- Support to read pipeline definition from file

  instead of passing the pipeline as parameter a file-named, prefixed by '@', can passed.
  The rest of the parameter will be interpreted as filename and opened to read the pipeline definition from there.

- Allow for more whitespaces around pipe symbol

  multi-line pipeline definitions now possible

### Changed
- Implement "Append" & "Prepend" instead of "Add" for element processing. This is a **breaking change**
    - The following commands have been removed:
        - `ADD-ELEMENT`
        - `ADD-COMMENT`
        - `ADD-TEXT-CONTENT`

    - Instead there are following, new commands:
        - `APPEND-ELEMENT`: same behaviour as previous `ADD-ELEMENT`
        - `APPEND-COMMENT`: same behaviour as previous `ADD-COMMENT `
        - `APPEND-TEXT-CONTENT`: same behaviour as previous `ADD-TEXT-CONTENT `
        - `PREPEND-ELEMENT`: similar to `APPEND-ELEMENT`, but new element is
          added as _first_ child
        - `PREPEND-COMMENT`: similar to `APPEND-COMMENT`, but new comment is
          added as _first_ child
        - `PREPEND-TEXT-CONTENT`: similar to `APPEND-TEXT-CONTENT`, but new text
          content is added as _first_ child

### Fixes
- Fix bug with SET-TEXT-CONTENT: couldn't GET-TEXT-CONTENT while SET-TEXT-CONTENT on same element

## [Earlier] - 2022-10-13

### Changed

- See Release specific changelogs on [github](https://github.com/kelko/html-streaming-editor/releases)