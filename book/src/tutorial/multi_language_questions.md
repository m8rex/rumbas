# Multi language questions

One of the most important features of rumbas is that it is easy to create version in different languages of the same question.

## Create a new rumbas project

```admonish question title="Task"
- Create a new folder `multi_language_questions` for this tutorial.
- Create a `.rumbasrc.yaml` file (with the right content) in this folder.
- Create a `questions` folder in this folder.
- Create an `exams` folder in this folder.
- Create a `defaults` folder in this folder.
```


```admonish question title="Task"
Copy the questions, exams and default from your `using_defaults` rumbas project to this project.
```

## Translating the statement

Currently our question with variables is defined in yaml as follows:

```yaml
{{#include ../../../examples/multi-language-questions/questions/questionpart_defaults/first_question_with_variables.yaml}}
```

Currently this question is only available in English. We want to make this question available in Dutch as well.

Let's take a look again at the structure of a questions.

```admonish question title="Task"
Take a good look at the (datatype of the) `statement` and `advice` field.
```

```admonish example title="Reference"
{{#include ../datatypes/Question.md}}
```

We see that the `statement` and `advice` fields have the type `Translation`. 

```admonish example title="Reference"
{{#include ../datatypes/Translation.md}}
```

For now we will focus on the first option, the `TranslationStruct`.

```admonish example title="Reference"
{{#include ../datatypes/TranslationStruct.md}}
```

```yaml
---
content: # the content of form TranslationContent
placeholders: {} # empty for now
```

Let's now have a look at `TranslationContent`.

```admonish example title="Reference"
{{#include ../datatypes/TranslationContent.md}}
```

The first option is the most important for now. It says that we can specify different versions of the content in different languages by using a hash.


```yaml
---
content:
  en: english content
  nl: dutch content
placeholders: {} # empty for now
```

```admonish question title="Task"
Update the `statement` and `advice` fields of the `first_question_with_variables.yaml` question to make it available in Dutch as shown below.
```

```yaml
{{#include ../../../examples/multi_language_questions/questions/first_question_with_variables_translated.yaml}}
```

```admonish question title="Task"
Recompile all exams
```

```admonish danger
Take a good look at the output of the compilation. You should see that all successfull compilation happened 'with locale en'. 
```

```admonish question
Any idea why this is the case?
```

```admonish info
Each exams specifies for which locales it needs to be compiled.
```

```admonish question title="Task"
Take a look at the default values to see which locales are set.
```

```yaml
{{#include ../../../examples/multi_language_questions/defaults/locales.yaml}}
```

```admonish question
Do you see why we used `en` in the `content` field of the question?
```

```admonish question title="Task"
Add the nl locale to the `locales.yaml` file.
```

```yaml
{{#include ../../../examples/multi_language_questions/exams/changed/defaults/locales.yaml}}
```

```admonish question title="Task"
Recompile all exams. You should see that the exams are now compiled for both locales.
```

You can access the dutch exams at `http://localhost:8000/nl/exams/first_question_with_variables/`

You can also find the [dutch exam](https://m8rex.github.io/rumbas/examples/multi_language_questions/nl/exams/first_question_with_variables_translated/) and [english exam](https://m8rex.github.io/rumbas/examples/multi_language_questions/en/exams/first_question_with_variables_translated/) in the [online demo in the online demo.
