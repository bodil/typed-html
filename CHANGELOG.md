# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/en/1.0.0/)
and this project adheres to [Semantic
Versioning](http://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- Most types now implement `Send`. (#53)
- All tags now support the ARIA `role` attribute. (#47)
- The `text!()` macro now accepts expressions as value arguments. (#48)
- You can now create "unsafe" text nodes with the `unsafe_text!()` macro, which
  won't be quoted at all when stringifying, even if they contain HTML tags. This
  is a meaningless distinction when creating DOM nodes, however, and unsafe text
  nodes will behave like normal text nodes in this case. (#39)

### Changed

- Text in attributes are quoted less aggressively when stringified. (#26, #49)
- Attribute type conversion is now using the recently stabilised `TryFrom`
  instead of `From`, to avoid relying on panicking `From` implementations to
  detect conversion errors, though the conversions inside the macro will still
  panic if they fail. The appropriate `TryFrom` implementations have been added
  to `Class`, `Id`, `SpacedList` and `SpacedSet`, and the corresponding `From`
  implementations have been removed.

## [0.2.0] - 2019-03-16

### Added

* Support for the [Dodrio](https://github.com/fitzgen/dodrio) virtual DOM renderer: the `dodrio_macro` feature flag enables the `typed_html::dodrio` macro, which generates code to build a `dodrio::Node` directly, without going via `VNode`s. (#38)

### Fixed

* Added the missing attributes to the `<video>` tag, which was previously listed as having none. (#32)

## [0.1.1] - 2018-11-29

### Added

* `typed-html` now works on stable rustc. (#1)
* All elements and HTML marker traits now implement `IntoIterator`, so you can return them directly
  from a group. (#12)

### Fixed

* Boolean flags are now correctly rendered as empty when stringifying. (#13, #14)
* Non-self-closing tags are now correctly rendered with a closing tag in the HTML style, rather than
  in the XML style. (#15, #16)

## [0.1.0] - 2018-11-17

This is the initial release.
