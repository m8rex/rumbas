## Step 4: Try compiling the first question again

Now we can try compiling the question again.

```admonish question title="Task"
Recompile the question.
```

This time we will get a different error. This error is because we are missing another crucial file in our rumbas project. This file is the `exams/questions_preview.yaml` file. 

```admonish info
The `exams/questions_preview.yaml` file is used to create a preview exam for a question. 
```

Because this file makes use a feature of rumbas that we will explain later (templating), we will create an exam that uses our question and compile that exam.

```admonish question title="Task"
Create the `exams` folder in the project folder and add a `first_exam.yaml` file in this new `exams` folder.
```

To know which fields we can set for a question, you can look at the [Reference for exam](./datatypes/complete_exam.md) page. For now we will show the fields that can be set for a question.

```admonish example title="Reference"
{{#include ../../datatypes/ExamFileType.md}}
```

For now we will just look at the `normal` exam type. This is the exam type that we will use for our first exam.

```admonish example title="Reference"
{{#include ../../datatypes/NormalExam.md}}
```

We can set a value for each of these fields.

```yaml
{{#include ../../../../examples/first_question/exams/first_exam.yaml}}
``` 

```admonish question title="Task"
Place this yaml in the `first_exam.yaml` file.
```