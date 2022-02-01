# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.5.1] - 2022-02-01

### Added
- Support for `||` as `or` in jme
- A Translation text without translation can now be written as plain text instead of needing a map with `content` and `placeholders` keys.

### Changed
- Feedback in multiple choices are content area's instead of EmbracedJMEStrings.
- `check` and `compile` command do now accept multiple paths, which will all be checked / compiled.
- Some debug prints are changed from INFO level to DEBUG level (default file logs).

## [0.5.0] - 2022-01-30

### Added

- `answer_display` field to `jme` part type.
- `sqlite` extension
- `text` extension
- `--no-minification` flag for `compile` command to suppress the `js` and `css` minification of Numbas. Before this addition, minification was not performed, now it is enabled by default.

### Changed

- Renamed `custom_marking_algorithm` to `custom_marking_algorithm_notes`
  - The type is changed from a string, to a map
- Validation of JME fields
  - Previously they were parsed as strings
  - Now they are parsed as JME expressions / ContentAreas
- Better error message when there is something wrong in the yaml of a question_part
- Moved `use_times_dot` field from `JMEAnswerSimplification` to `JMEAnswerDisplay` and renamed it to `always_show_multiplication_sign`.
- Renamed fields of `JMEAnswerSimplification`:
  - The new names should be much more clear
  - The old names are still valid
- Dockerfile: updated numbas version (so minify_css is supported)
- Syntax for `TranslatableString` is changed.
  - Instead of a map that contains `content`, some locale fields and some placeholders that are surrounded with brackets (`{<placeholder>}`)
  - It now is a map with fields `content` and `placeholders`
    - `content` is either a map (with locales mapping to strings) or just a string (not locale dependant)
    - `placeholders` is a map that maps placeholders (strings) to`TranslatableString`s
- Improved Input support
  - Enums that have multiple variants with a type that is an Input struct, work
    - Previously the first variant was always used
    - Now parsing an Input struct fails when all fields are None
- removed `has_to_select_option` from `choose_one`.
- `display` in `choose_one` parts is now an object:
  - `type` is either `dropdown` or `radio`
  - if it is `radio` is also has a `columns` field
  - previously this was flattened and the `columns` field was set at the higher level
- `display` in `match_answers` parts is now an object:
  - `type` is either `check` or `radio`
  - if it is `check` is also has a `marking_method` field
  - previously this was flattened and the `columns` field was set at the higher level
- Added `minimal_achievable_marks` and `maximal_achievable_marks` to `match_answers` part types.
- Added `minimal_achievable_marks`, `maximal_achievable_marks` and `marking_method` to `choose_multiple` part types.

## [0.4.0] - 2021-08-04

### Fixed

- Multiple choice marking

### Added

- Support for `allow_printing` in navigation
- Support for `geogebra` and `eukleides` extensions (you will need to add a default value to `defauls/question.yaml` for the these extensions)
- Support for diagnostic exams
- `builtin_constants` and `custom_constants` in question description
- `diagnostic_topic_names` in question description
- `resources` in question description
- Support for `NumbasLike` answers field for ChooseOne part (this means that a type has to be specified)
- Support for `matrix` part type
- Support for resources
- `rumbas init`: command to initialize a rumbas project
- `rumbas import`: command to import numbas .exam files
- `wrong_nb_answers_warning_type` field to all multiple choice part types

### Changed

- Better messages when the yaml is invalid. The specific part that is invalid, is shown.
- Numbas version (v6.0)
- `should_select_at_least` in `QuestionPartChooseOne` is changed to a boolean `has_to_select_option`
- The command to compile a question or an exam is now `rumbas compile <path>` instead of just `rumbas <path>`.
- Change template folder names to `question_templates` and `exam_templates` (instead of `template_questions` and `template_exams`)
- Parsing of answer simplification types also handles the `!` operation.
- `checking_type` and `checking_accuracy` fields in jme part are moved to a new field `answer_check` which is an object.
  - This object has a `type` (previously this was `checking_type`)
  - If the type is `relative_difference` or `absolute_difference` there is a second field `max_difference` (previously this was `checking_accuracy`)
  - If the type is `decimal_places` or `significant_figures` there is a second field `amount` (previously this was `checking_accuracy`)
- `answers` field in multiple choice parts in renamed to `answer_data` (old name is still allowed)
  - Added `type` field to `answer_data`
- Renamed and added some AnswerStyles. `plain` is renamed to `english-plain` etc
- `allow_regenerate` field in navigation is changed to `can_regenerate` (old name is still allowed)
- `allow_steps` field in navigation is changed to `show_steps` (old name is still allowed)
- `show_frontpage` field in navigation is changed to `show_title_page` (old name is still allowed)
- `reverse` field in navigation is changed to `can_move_to_previous` (old name is still allowed)
- `prevent_leaving` field in navigation is changed to `confirm_when_leaving` (old name is still allowed)
- Added `type` field to `input_widget` field of custom part types

### Removed

- `checking_type` option in `number_entry` questionparts
- `name` and `strings` options in `must_match_pattern`, `max_length` and `min_length` option of `jme` questionparts
- `name` field in `must_have` and `may_not_have` option of `jme` questionparts

## [0.3.1] - 2021-07-24

### Added

- Support for `stats` extension (you will need to add a default value to `defauls/question.yaml` for the `stats` extension)
- Support for `simplify_no_leading_minus`, `simplify_fractions`, `simplify_trigonometric`, `cancel_terms`, `cancel_factors`, `collect_like_fractions`, `order_canonical`, `use_times_dot` and `expand_brackets` in answer simplification.
- `value_generators` support in JME

### Changed

- Numbas version

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
