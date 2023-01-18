
## Step 2: Create a question

```admonish question title="Task"
Create a file named `first_question.yaml` within the folder `questions`.
```

 This file will contain the yaml code for the question. To know which fields we can set for a question, you can look at the [Reference for question](./datatypes/complete_question.md) page. For now we will show the fields that can be set for a question:

{{#include ../../datatypes/QuestionFileType.md}}

For now we will just look at the 'normal' question type.

{{#include ../../datatypes/Question.md}}

We can set a value for each of these fields.

```yaml
{{#include ../../../../examples/first_question/questions/first_question.yaml}}
```

```admonish question title="Task"
Place this yaml in the `first_question.yaml`.
```