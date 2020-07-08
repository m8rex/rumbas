# Getting Started

This guide assumes that you will use docker to run rumbas.

## Prerequisites
- Make sure [docker](https://www.docker.com/get-started) is installed
- Make sure you have the [rumbas-examples repo](https://github.com/m8rex/rumbas-examples) on your computer (by cloning it or downloading the zip (see the 'code' button))


## How does rumbas work?

### Short answer
- Rumbas is entirely file based
- The content of exams and questions are specified in [JSON](https://en.wikipedia.org/wiki/JSON)
- The Rumbas program converts these descriptions in JSON to a [Numbas](https://www.numbas.org.uk/) exam

### Folder structure
A rumbas project should have to following folder structure:
  - A folder named `default` that contains the default specifications
  - A folder named `questions` that contains the questions
  - A folder named `exams` that contains the exams
  - A folder named `themes` the contains the themes
  - A folder named `template_questions` that contains templates for questions
  - A folder named `template_exams` that contains the template for exams
  - A folder named `custom_part_types` that contains `custom_part_types` (not yet supported)
  
Browse through the folders in the [rumbas-examples repo](https://github.com/m8rex/rumbas-examples) on your computer to see how this works.

#### Exams folder
The exams folder contains json files of the following form:
```json
{
  "locales": [
    { 
      "name": "nl",
      "numbas_locale": "nl-NL"
    },
    { 
      "name": "en",
      "numbas_locale": "en-GB"
    }
  ],
  "name": {
    "nl": "Test algebra",
    "en": "Exam algebra"
  },
  "question_groups": [
    {
      "name": {
        "nl": "Deel 1",
        "en": "Part 1"
      },
      "picking_strategy": "all_ordered",
      "questions": [
          "M0/algebra/H2/calculate_a",
          "M0/algebra/H2/calculate_b"
      ]
    },
	{
      "name": {
        "nl": "Deel 2",
        "en": "Part 2"
      },
      "picking_strategy": "all_shuffled",
      "questions": [
          "M0/algebra/H2/calculate_a",
          "M0/algebra/H2/calculate_b"
      ]
    },
	{
      "name": {
        "nl": "Deel 3",
        "en": "Part 3"
      },
      "picking_strategy": "random_subset",
      "pick_questions": 1,
      "questions": [
          "M0/algebra/H2/calculate_a",
          "M0/algebra/H2/calculate_b"
      ]
    },
  ]
}
```

This specifies:
- That the exam is created in both dutch and english.
- That it is named "Test algebra" in dutch, and "Exam algebra" in english
- That it has three question groups:
	- The first question group:
		- Shows the two questions in the specified order
	- The second question group:
		- Shows the two questions in random order
	- The third question group:
		- Show only one of the two questions and chooses this at random
- It uses two questions that are specified by the files:
	- `questions/M0/algebra/H2/calculate_a.json`
	- `questions/M0/algebra/H2/calculate_b.json`

IMPORTANT: There are much more settings that can (and should be set) for exams. See the section about the default folder.

#### Question folders

#### Default folders

It is important to note that rumbas does not specify any default value by itself. Numbas does however have a quite extensive range of options that can be set. Setting all these options for every question and exam would be a real hassle and the files would not be readable.

To prevent this problem, the `default` folders are created.
Folders named `default` can be specified:
- In the root of the rumbas project
- In any folder in the `exams` folder
- In any folder in the `questions` folder

When the description of a `question`/`exam` is read:
- All default folders in ancestor folders are examined for default values:
	- e.g. if a question is positioned in `questions/M0/algebra/H1/nul_in_N_and_Z.json` the following default folders will be checked:
		- `questions/M0/algebra/H1/default`
		- `questions/M0/algebra/default`
		- `questions/M0/default/`
		- `questions/default`
		- `default`
- The default folder are examined in order: first the 'closer' ones. 
	- e.g. The folder will be examined in the order shown above
	
### Translations


## Running rumbas
- Run `docker run --rm -it -v <absolute_path_to_rumbas-examples_repo>:/rumbas m8rex/rumbas <exam_or_question_file>`
	- e.g.`docker run --rm -it -v C:\Users\jesse\Documents\rumbas-examples:/rumbas m8rex/rumbas exams/M0/algebra/begintest.json`
- Go to `<absolute_path_to_rumbas-examples_repo>/_output` to find the generated html. 
	- Click on the `index.html` file to open the exam in the browser

