# Multi language questions

One of the most important features of rumbas is that it is easy to create different language versions of the same question.

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
{{#include ../../../examples/multi_language_questions/questions/first_question_with_variables.yaml}}
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
Update the `statement`, `advice` and `prompt` fields of the `first_question_with_variables.yaml` question to make it available in Dutch as shown below.
```

```yaml
{{#include ../../../examples/multi_language_questions/questions/first_question_with_variables_translated.yaml}}
```

```admonish question title="Task"
Recompile all exams
```

```admonish danger
Take a good look at the output of the compilation. You should see that all successful compilations happened 'with locale en'. 
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
Do you see why we used the key `en` in the `content` field of the question?
```

```admonish question title="Task"
Add the `nl` locale to the `locales.yaml` file.
```

```yaml
{{#include ../../../examples/multi_language_questions/exams/changed/defaults/locales.yaml}}
```

```admonish question title="Task"
Recompile all exams. You should see that the exams are now compiled for both locales.
```

You can access the dutch exams at `http://localhost:8000/nl/exams/first_question_with_variables/`

You can also find the [dutch exam](https://m8rex.github.io/rumbas/examples/multi_language_questions/nl/exams/first_question_with_variables_translated/) and [english exam](https://m8rex.github.io/rumbas/examples/multi_language_questions/en/exams/first_question_with_variables_translated/) in the online demo.

## Placeholders

If we look at `statement` and `advice` we see that some values (mostly math expressions) are language independant.

```yaml
{{#include ../../../examples/multi_language_questions/questions/first_question_with_variables_translated.yaml}}
```

This is where placeholders come in. Placeholders can be specified by name and then be used in the `content` field by writing `{name}`.


```yaml
{{#include ../../../examples/multi_language_questions/questions/first_question_with_variables_translated_placeholders.yaml}}
```

```admonish question title="Task"
Recompile all exams. You should see no difference but there are less chances to have different formulas in the different languages.
```

```admonish info
You can also use the placeholders the other way around. This is mainly useful when almost the whole string is maths and some small parts need to be translated. This might not work for all languages due to different grammar rules etc. For example:
```


```yaml
---
content: "{command} $5x^2-10$."
placeholders:
  command: 
    content:
      nl: Ontbind in factoren
      en: Factorize
    placeholders: {}
```

## Locale Folders

It might happen that you want to use the same text in different questions or exams. This is where the `FileString` type comes in.

```admonish question title="Task"
Take a look at the second option of the `Translation` type.
``` 


```admonish example title="Reference"
{{#include ../datatypes/Translation.md}}
```

Let's take a look at the `FileString` type.

```admonish example title="Reference"
{{#include ../datatypes/FileString.md}}
```

```admonish info
A filestring is thus one of the following:
1) A string that starts with `file:` followed by a path to a file
2) Another string that is interpreted literally as text.

In the first option, it is possible to use locale folders to specify different versions of the file for different locales.
```

Using the `FileString` type in combination with locale folders gives use a powerful mmechanism to reuse text in different questions and exams.

```admonish question title="Task"
Move the `statement` content to localized files.
```

This yields following yaml code for the question

```yaml
{{#include ../../../examples/multi_language_questions/questions/first_question_with_variables_translated_placeholders_filestring.yaml}}
```

And `questions/locale-nl/how-much.html` file:

```html
{{#include ../../../examples/multi_language_questions/questions/locale-nl/how-much.html}}
```

And `questions/locale-en/how-much.html` file:

```html
{{#include ../../../examples/multi_language_questions/questions/locale-en/how-much.html}}
```

```admonish question title="Task"
Update all questions and exams so they are available in both English and Dutch. If you don't speak dutch, use any other language. It is also possible to use the locales for different styles of instruction.
```