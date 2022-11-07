# Rumbas (Rust + Numbas)

## What is Rumbas ?

- A system to create online exercises, build on top of numbas.
- Text-based: yaml (and html and tex) specifications are converted to html files (e.g. scorm package)
  - So git can be used for version control 🎉
- Consistent naming of fields in the yaml (not the case in the json of numbas)
- Written in Rust
- Preferred way to use it is docker
- Currently in beta, not all features are implemented yet.
- Do you want to use it? Read the [Getting Started](https://github.com/m8rex/rumbas/blob/master/GETTING_STARTED.md) guide.

## Folder structure

A rumbas project should have to following folder structure:

- A folder named `default` that contains the default specifications
- A folder named `questions` that contains the questions
- A folder named `exams` that contains the exams
- A folder named `themes` the contains the themes
- A folder named `question_templates` that contains templates for questions
- A folder named `exam_templates` that contains the template for exams
- A folder named `custom_part_types` that contains `custom_part_types`
- A folder named `resources` that contains the resources that are used in exams.

Rumbas does not specify default values by itself:

- Users can (and should) use the `default` folder to specify their own default versions for different settings
- By doing this, their question and exam specifications become much more concise.
- Users can also create multiple versions of the default settings (e.g. a default setting for practice exams and a default setting for real exams)

The html input can be specified in two ways:

- inline in the yaml,
- in a separate html file (Recommended for larger htmls) - The value of the yaml field should then equal `file:<path to filename in questions folder>`
  This will also be possible for the description of functions

Functions can be specified by just a filename:

- Not yet implemented
- Language is taken from the extension (`js` or `jme`)
- file should start with comments of form `# param <name> <type>`
- next should be the real definition

Templating is possible:

- Templates should be placed in the `question_templates`/`exam_templates` folder
  - These files look like ordinary question/exam files
  - Contains strings like `"template:<name>"` as value for some fields (e.g. `"template:equation"`)
- `Values files` should be placed in the `questions`/`exams` folder
  - Contains a `template` field which specifies the template. This path is relative within `question_templates`/`exam_templates`
  - Contains a field for every template variable in the template file
  - If the template file contains `"template:<name>"`, the field `name` has to be specified in the values file

## Running rumbas

### Docker

The easiest way is to use docker.

!IMPORTANT! See the [Getting Started file](https://github.com/m8rex/rumbas/blob/master/GETTING_STARTED.md#simplifying-docker-usage) to see a way to simplify your docker usage.

- Get the docker image
  - Either Pull the latest image from [github](https://github.com/orgs/m8rex/packages/container/package/rumbas): `docker pull ghcr.io/m8rex/rumbas`
  - Or Clone this repo & Build the docker image with `docker build -t rumbas .`
- Run rumbas:`docker run --rm -it -v <absolute_path_to_folder with rumbas structure>:/rumbas ghcr.io/m8rex/rumbas compile <relative path of exam in the mounted folder>`
  - Example `docker run --rm -it -v /Programming/rumbas/rumbas/examples/simple-example:/rumbas ghcr.io/m8rex/rumbas compile exams/rumbas-exam-test.yaml`
  - Other Example `docker run --rm -it -v /Programming/rumbas/rumbas/examples/simple-example:/rumbas ghcr.io/m8rex/rumbas compile questions/question1.yaml`
    - This compiles a single exercise by using the `exam_templates/question_preview.yaml` template

### Without docker

- Make sure python 3 is installed (and added to the path)
- Clone numbas from https://github.com/numbas/Numbas
- Install rumbas
  - Build it yourself (see rumbas folder)
  - Download binaries (not yet available)
- Run rumbas
  - Make sure that the `NUMBAS_FOLDER` env variable is set to the root of the cloned Numbas repo
  - IMPORTANT: Themes don't work the right way yet, you need to make sure that the themes are added to the themes folder of your local numbas clone -> use Docker to not have this problem.

## Progress

- [x] Basic exam settings (`name`, `duration`, `percentPass`, `showQuestionGroupNames`, `showStudentName`)
  - [x] support in yaml
  - [x] support in default
- [x] Navigation exam settings
  - [x] support in yaml
  - [x] support in default
- [x] Timing exam settings
  - [x] support in yaml
  - [x] support in default
- [x] Feedback exam settings
  - [x] support in yaml
  - [x] support in default
- [x] Support for `file:<filename>` for html strings
- [x] Support for `question_groups` in exams
  - [x] `name` and `pickingStrategy`
  - [x] `questions`
- [x] Support for basic info in questions (`name`, `statement`, `advice`)
- [x] Support for `parts` in questions
  - [x] JME
  - [x] NumberEntry
  - [x] Matrix
  - [x] PatternMatch
  - [x] ChooseOne
  - [x] ChooseMultiple
  - [x] MatchAnswers
  - [x] GapFill
  - [x] Information
  - [x] Extension
- [ ] Support for extensions in questions
  - [ ] Extensions are added to exam if they are used in the questions (partly done)
- [x] Support for variables in questions
  - [x] Specified in yaml
  - [x] Ungrouped
  - [x] Short representation as string or list
  - [ ] variable groups 
  - [x] variables test
- [ ] Support for functions in questions
  - [x] Specified in yaml
  - [ ] Specified as file
- [ ] Support for translations
  - [x] In name of exam
  - [x] Everywhere
  - [ ] Fix optional overwrite
  - [x] Fix substitutions (for example for common parts like functions)
- [ ] Theme support
  - [x] when using docker
  - [ ] when not using docker
- [ ] preamble
  - [x] at Questions
  - [ ] at Exams
- [x] rulesets
  - [x] at Questions
- [x] Support for `custom_part_types` in questions (sharable, defined in folder `custom_part_types`)
  - [x] Support for `custom_part_types` in questions (sharable, defined in folder `custom_part_types`)
  - [x] `custom_part_types` are added to exam if they are used in the questions
- [x] Support for resources in questions
  - [x] Resources are added to exam if they are used in the questions
- [ ] Tests
- [ ] functions, variables at Exams -> usefull?
- [ ] contributors and metadata -> usefull?
  - [ ] at Questions
  - [ ] at Exams
- [x] Templating
  - [x] Exams
  - [x] Questions
  - [x] What about default values?
- [x] Question preview
