# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/en/1.0.0/)
and this project adheres to [Semantic
Versioning](http://semver.org/spec/v2.0.0.html).

## [0.1.1] - 2018-22-29

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
