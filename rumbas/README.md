# Rumbas (Rust + Numbas)

## What is Rumbas ?

- A system to create online exercises, build on top of numbas.
- Text-based: json (and html) specifications are converted to html files (e.g. scorm package)
  - So git can be used for version control 🎉 
- Consistent naming of fields in the json (not the case in numbas)

## Folder structure
A rumbas project should have to following folder structure:
  - A folder named `default` that contains the default specifications
  - A folder named `questions` that contains the questions
  - A folder named `exams` that contains the exams
  - A folder named `themes` the contains the themes
  - A folder named `custom_part_types` that contains `custom_part_types`

Rumbas does not specify default values by itself:
  - Users can (and should) use the `default` folder to specify their own default versions for different settings
  - By doing this, their question and exam specifications become much more concise.
  - Users can also create multiple versions of the default settings (e.g. a default setting for practice exams and a default setting for real exams)

The html input can be specified in two ways:
  - inline in the json,
  - in a separate html file (Recommended for larger jsons)
      - The value of the json field should then equal `file:<filename>`
This is also possible for the description of functions

Functions can be specified by just a filename:
  - Language is taken from the extension (`js` or `jme`)
  - file should start with comments of form `# param <name> <type>`
  - next should be the real definition

## TODO
- [ ] Basic exam settings (`name`, `duration`, `percentPass`, `showQuestionGroupNames`, `showStudentName`)
  - [ ] support in json
  - [ ] support in default
- [ ] Navigation exam settings
  - [ ] support in json
  - [ ] support in default
- [ ] Timing exam settings
  - [ ] support in json
  - [ ] support in default
- [ ] Feedback exam settings
  - [ ] support in json
  - [ ] support in default
  - [ ] support for `file:<filename>`
- [ ] Support for `question_groups` in exams
  - [ ] `name` and `pickingStrategy`
  - [ ] `questions`
- [ ] Support for basic info in questions (`name`, `statement`, `advice`)
- [ ] Support for `parts` in questions
  - [ ] JME
  - [ ] NumberEntry
  - [ ] Matrix
  - [ ] PatternMatch
  - [ ] OneNTwo (rename)
  - [ ] MNTwo (rename)
  - [ ] MNX (rename)
  - [ ] GapFill
  - [ ] Information
  - [ ] Extension
- [ ] Support for resources and extensions in questions
  - [ ] Resources and extensions are added to exam if they are used in the questions
- [ ] Support for variables in questions
  - [ ] Specified in json
  - [ ] Ungrouped
  - [ ] variable groups?
  - [ ] variables test
- [ ] Support for functions in questions
  - [ ] Specified in json
  - [ ] Specified as file
- [ ] Support for `custom_part_types` in questions (sharable, defined in folder `custom_part_types`)
  - [ ] Support for `custom_part_types` in questions (sharable, defined in folder `custom_part_types`)
  - [ ] `custom_part_types` are added to exam if they are used in the questions
- [ ] Theme support
- [ ] rulesets and preamble
  - [ ] at Questions
  - [ ] at Exams
- [ ] functions, variables at Exams
- [ ] contributors and metadata
  - [ ] at Questions
  - [ ] at Exams
