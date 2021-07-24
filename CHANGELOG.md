# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.3.1] - 2021-07-24

### Added

- Support for `allow_printing` in navigation
- Support for `stats`, `geogebra` and `eukleides` extensions (you will need to add a default value to `defauls/question.yaml` for the these extensions)
- Support for `simplify_no_leading_minus`, `simplify_fractions`, `simplify_trigonometric`, `cancel_terms`, `cancel_factors`, `collect_like_fractions`, `order_canonical`, `use_times_dot` and `expand_brackets` in answer simplification.
- `value_generators` support in JME
- Support for diagnostic exams
- `builtin_constants` and `custom_constants` in question description
- `diagnostic_topic_names` in question description
- `resources` in question description
- Support for `NumbasLike` answers field for ChooseOne part
- Support for resources
- `rumbas init`: command to initialize a rumbas project
- `rumbas import`: command to import numbas .exam files

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
- Renamed and added some AnswerStyles. `plain` is renamed to `english-plain` etc
- `allow_regenerate` field in navigation is changed to `can_regenerate` (old name is still allowed)

- `allow_steps` field in navigation is changed to `show_steps` (old name is still allowed)

- `show_frontpage` field in navigation is changed to `show_title_page` (old name is still allowed)
- `reverse` field in navigation is changed to `can_move_to_previous` (old name is still allowed)

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
