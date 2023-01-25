# Using Templates

Another very important feature of rumbas, is its templating feature. This feature allows you to create a template for a question or exam. This template can then be used to create multiple questions or exams. This is very useful when you have a lot of questions or exams that are very similar.

## Create a new rumbas project

```admonish question title="Task"
- Create a new folder `using_templates` for the rumbas project of this tutorial.
- Create a `.rumbasrc.yaml` file (with the right content) in this folder.
- Create a `questions` folder in this folder.
- Create an `exams` folder in this folder.
- Copy the `defaults` folder from the previous tutorial.
```

## Add a question and an exam

We will add an exam with the following content:

```yaml
{{#include ../../../examples/using_templates/exams/exam.yaml}}
```

And also add a question with the following content:

```yaml
{{#include ../../../examples/using_templates/questions/question.yaml}}
```

```admonish question title="Task"
Add the `exam.yaml` and `question.yaml` files to the `exams` and `questions` folders respectively.
```


```admonish question title="Task"
Run `rumbas check exams` to check if the setup is correct.
```

## Using a template

### Create a template

We might want to create a lot of questions that are very similar to the question we just created. In some cases, we might want to let the variable a go up to 100 or be limited between 1 and 5.

To do this, we copy the contents of `question.yaml` to `question_template.yaml` and make the change the value of the variable `a` to `template:a`. This yields the following content for `question_template.yaml`:

```yaml
{{#include ../../../examples/using_templates/questions/question_template.yaml}}
```

There is no meaning behind the `_template` suffix of the file name, we just use it for the example.

```admonish question title="Task"
- Copy the contents of `question.yaml` to `question_template.yaml`.
- Change the value of the variable `a`.
```

Now that we have added a value of the form `template:<template-key>` the `question_template.yaml` file is a template. We can now use this template to create multiple questions.

### Loading the template in an exam

The easiest way to use this templated question, is by loading it in an exam and giving values for each template key.

We will load the template two times in the exam. The first time we will set the value of `a` to 3 so it will always yield the same question. The second time, we set it to `random(10..100)` to let `a` be a random value between 10 and 100.

We will change our `exam.yaml` file to the following:

```yaml
{{#include ../../../examples/using_templates/exams/exam_with_template.yaml}}
```

```admonish question title="Task"
update the `exam.yaml` file to the content above.
```

```admonish question title="Task"
Recompile all exams with `rumbas compile exams`.
```

```admonish question title="Task"
Try the exam, you should see three questions that work slightly different in their generation of `a`.
```

### What if we forget to set a template key?

What would happen when we don't set all template keys? Let's try it out.

We will change our `exam.yaml` file to the following:

```yaml
{{#include ../../../examples/using_templates/exams/exam_with_template_missing_keys}}
```

```admonish question title="Task"
Change the `exam.yaml` file to the content above.
```

```admonish question title="Task"
Check all exams with `rumbas check exams`.
```

This will yield an error message like the following:

```text
[2023-01-25][09:48:30][rumbas_support::input][ERROR] Found 1 missing template keys:
[2023-01-25][09:48:30][rumbas_support::input][ERROR] 1  a at question_groups.0.questions.3.question_template.yaml.variables.a
```

Which says that we forgot to set the value of `a` for the fourth question (index 3 when you count from 0).

```admonish question title="Task"
Set a value for the template key `a` for the fourth question.
```

```admonish question title="Task"
Check all exams with `rumbas check exams`. This should now pass.
```

```admonish question title="Task"
Recompile all exams with `rumbas compile exams` and try out the fourth question in the exam.
```

### Using default values for template keys

It is possible to defined default values for template keys. This allows you to override the value but also have a default value that is used when the value is not set.

We will demonstrate this by allowing the marks for the question to be set by a template key, but also have a default value of 5 marks.

We will change our `question_template.yaml` file to the following:

```yaml
{{#include ../../../examples/using_templates/questions/question_template_default.yaml}}
```

```admonish question title="Task"
Update the `question_template.yaml` file to the content above.
```

This allows us to optionally specify the value of the template key `marks` in the exam. If we don't specify it, it will use the default value of 5.

We will change our `exam.yaml` file to the following:

```yaml
{{#include ../../../examples/using_templates/exams/exam_with_template_default}}
```

```admonish question title="Task"
Update the `exam.yaml` file to the content above.
```

```admonish question title="Task"
Recompile all exams with `rumbas compile exams`.
```

```admonish question title="Task"
Try the exam, you should see that the third question assigns 10 marks.
```


### Create a question that uses a template

If you want to reuse a version of a question that uses a template, you can create a question that uses a template.

We will create a question that uses the template we created in the previous section.

We will create a `question_using_template.yaml` file with the following content:

```yaml
{{#include ../../../examples/using_templates/questions/question_using_template.yaml}}
```

We can now include this question in an exam.

```admonish question title="Task"
Add this question to the `exam.yaml` file.
```

```admonish question title="Task"
Recompile all exams with `rumbas compile exams`.
```

```admonish question title="Task"
Try the exam, you should see that the added question uses a value of 11, 55 or 78 for `a`.
```
