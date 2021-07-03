# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- Support for `stats` extension (you will need to add a default value to `defauls/question.yaml` for the `stats` extension)
- Support for `simplify_no_leading_minus`, `simplify_fractions`, `simplify_trigonometric`, `cancel_terms`, `cancel_factors`, `collect_like_fractions`, `order_canonical`, `use_times_dot` and `expand_brackets` in answer simplification.
- `value_generators` support in JME
- Support for diagnostic exams
- `builtin_constants` and `custom_constants` in question description

### Changed

- Numbas version (v6.0)

### Removed

- `checking_type` option in `number_entry` questionparts
- `name` and `strings` options in `must_match_pattern` option of `jme` questionparts

## [0.3.0] - 2020-12-17

### Changed

- Numbas version

## [0.2.3] - 2020-11-20

### Fixed

- Creation of the `_output` folder and locale folders within

## [0.2.2] - 2020-09-05

### Added

- Support for `questionpart.gapfill.gap.choose_one` default file
- Support for `jsx_graph` extension
- Support for `match_answers` part type

### Changed

- Type of marks in multiple choice answer (from float to primitive)
- Type of parameters in function (from HashMap to Vec to preserve order)

## [0.2.1] - 2020-08-18

### Added

- Support for `choose_multiple` part type

## [0.2.0] - 2020-08-11

### Changed

- Use yaml instead of json as file format
- Templating is much more robust now

## [0.1.1] - 2020-07-08

### Added

- Cli options: scorm and zip

## [0.1.0] - 2020-07-08

### Added

- First release
