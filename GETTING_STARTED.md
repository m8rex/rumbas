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
The questions folder contains json files of the following form:
```json
{
  "variables": {
    "m": "random(2,3)",
    "b": "random(1..5)",
    "a": "random(5,6,8)"
  },
  "statement": {
    "content": "file:expression_no_calculator.html",
    "{expr}": "\\( (\\var{a} \\cdot \\var{m} + 1) \\cdot (-\\var{b}) + \\left(\\frac{\\var{m} \\cdot \\var{a}}{\\var{m} }+(-\\var{2*b})\\right) \\cdot \\var{m} \\)"
  },
  "parts": [
    {
      "type": "number_entry",
      "answer": "{(a*m+1)*(-b) + (a - 2*b)*m}",
      "marks": 1,
      "allow_fractions": false
    }
  ],
  "advice": {
    "content": "{sol}{postamble}",
    "{sol}": "\\begin{array}<br/>&amp; (\\var{a} \\cdot \\var{m} + 1) \\cdot (-\\var{b}) + \\left(\\frac{\\var{m} \\cdot \\var{a}}{\\var{m} }+(-\\var{2*b})\\right) \\cdot \\var{m} \\\\<br/> &amp; = (\\var{a*m} + 1) \\cdot (-\\var{b}) + (\\var{a} - \\var{2*b}) \\cdot \\var{m} \\\\<br/> &amp; = (\\var{a*m+1}) \\cdot (-\\var{b}) + (\\var{a - 2*b}) \\cdot \\var{m} \\\\<br/> &amp; = \\var{(a*m+1)*(-b)} + \\var{(a - 2*b)*m} \\\\<br/> &amp; = \\var{(a*m+1)*(-b) + (a - 2*b)*m}\\\\<br/>\\end{array}",
    "{postamble}": "file:postamble.html",
    "{chapter}": "file:M0/algebra/H2.html"
  }
}
```

This specifies:
- That there are three variables:
  - The variable m is a random element of the set {2,3}
  - The variable b is a random element of the set {1,2,3,4,5}
  - The variable a is a random element of the set {5,6,8}
- That the content of the statement can be found in the file `questions/expression_no_calculator.html`
  - When you work with translations (for example the locale 'nl'), he will first check for the file `questions/locale-nl/expression_no_calculator.html`
- That this content contains a placeholder `{expr}` which will be set with the given expression
  - Note that backslashes need to be escaped in JSON
  - Rumbas supports a shorter way to write `\\var{}` (`µ{}`) and `\\simplify{}` (`§{}`)
  - You can also specify this expression in a .tex file and load it with `file:<filepath_in_questions_folder>`
- That this question has one part:
  - Of type `number_entry`
  - With a certain `answer` and amount of `marks` that a correct answer gives
  - in which typing fractions is not allowed
- That the advice consists of a solution and a postamble
  - This postamble specifies a `{chapter}` placeholder which is also set

IMPORTANT: There are much more settings that can (and should be set) for questions. See the section about the default folder.

#### Default folders

It is important to note that rumbas does not specify any default value by itself. Numbas does however have a quite extensive range of options that can be set. Setting all these options for every question and exam would be a real hassle and the files would not be readable.

To prevent this problem, the `default` folders were created.
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
- The default folders are examined in order: first the 'closer' ones. 
	- e.g. The folders will be examined in the order shown above
	
### Translations
- Translations are mandatory in the sense that you need the specify which locales you need in an exam file:
  - This can obviously also be just one locale
  - You need to give each locale a name and specify which locale numbas should use for its text (limited set of locales)
- Every string can be:
  - Just a string:
    - There is no translation
    - The same content is used for every locale
  - A string of the form `file:<locale_file_path_in_question/exams_folder>`
    - The content is loaded from a file 
      - Separate version for each locale can be specified in the `locale-<locale_name>` folders
      - The locale folders can be found in the same folder as the specified file
      - In those folders, rumbas looks for a file with the same name
    - Example: `file:M0/example.html`
      - For the `nl` version: `M0/locale-nl/example.html` is loaded if it exists, otherwise `M0/example.html`
      - For the `en` version: `M0/locale-en/example.html` is loaded if it exists, otherwise `M0/example.html`
  - An object with:
    - A field `content` of which the value can be any of the three types specifiied here for strings
    - And/or a field named `<locale>` to specify the content for a specific locale
    - Fields of the forms `{<placeholder_name>}` with as value the string value of the placeholder (specified in one of the three ways explained here)

### Templating
- Templates need to be specified in the `template_questions` and `template_exams` folder
- The template files are specified in JSON format but some fields have a string value of the form `template:<parameter_name>`
  - This specifies that the value of this field is a parameter and how that parameter is called
- A question that uses a template is specified in the `questions` folder
  - In JSON format
  - Contains the `template` field which specifies which template to use (relative to the `template_exams`/`template_questions` folder
  - Contains a field for every parameter in the template
- A special templated exam is the file `templated_exams/question_preview.json` which is used as wrapper when a question is converted to a numbas exam by itself.

## Running rumbas
- Run `docker run --rm -it -v <absolute_path_to_rumbas-examples_repo>:/rumbas m8rex/rumbas <exam_or_question_file>`
	- e.g.`docker run --rm -it -v C:\Users\jesse\Documents\rumbas-examples:/rumbas m8rex/rumbas exams/M0/algebra/begintest.json`
- Go to `<absolute_path_to_rumbas-examples_repo>/_output` to find the generated html. 
	- Click on the `index.html` file to open the exam in the browser

## Next steps
- Look at all questions in the `M0` folder
  - Look at the default files and which fields they specify
  - See how content is loaded from files
  - How content is translated
  - How templates are used