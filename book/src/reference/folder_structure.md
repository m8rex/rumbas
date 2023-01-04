# Folder structure

A rumbas project should have to following folder structure:

- A folder named `default` that contains the default specifications
- A folder named `questions` that contains the questions
- A folder named `exams` that contains the exams
- A folder named `themes` the contains the themes
- A folder named `custom_part_types` that contains `custom_part_types`
- A folder named `resources` that contains the resources that are used in exams.

## Default folders

TODO: list the files that can be set

It is important to note that rumbas does not specify any default value by itself. Numbas does however have a quite extensive range of options that can be set. Setting all these options for every question and exam would be a real hassle and the files would not be readable.

To prevent this problem, the `default` folders were created.

Folders named `default` can be specified:

- In the root of the rumbas project
- In any (sub)folder in the `exams` folder
- In any (sub)folder in the `questions` folder

When the description of a `question`/`exam` is read:

- All default folders in ancestor folders are examined for default values:
  - e.g. if a question is positioned in `questions/M0/algebra/H1/nul_in_N_and_Z.yaml` the following default folders will be checked:
    - `questions/M0/algebra/H1/default`
    - `questions/M0/algebra/default`
    - `questions/M0/default/`
    - `questions/default`
    - `default`
- The default folders are examined in order: first the 'closer' ones.
  - e.g. The folders will be examined in the order shown above

## Locale folders

Each subfolder of the `exams` and `questions` folder can have a `locale` folder. This are folders with a name of the form `locale-<locale-key>`. They allow easily translating things that are used many times.

