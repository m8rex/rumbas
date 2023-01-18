## Step 4: Try compiling the first question again

Now we can try compiling the question again. This time we will get a different error. This error is because we are missing a crucial file in our rumbas project. This file is the `exams/questions_preview.yaml` file. This file is used to create a preview exam for a question. Because this file makes use a feature of rumbas that we will explain later (templating), we will create a specific first exam and compile that exam.

Create the exams folder and and an `exams/first_exam.yaml` file.

To know which fields we can set for a question, you can look at the [Reference for exam](./datatypes/complete_exam.md) page. For now we will show the fields that can be set for a question:

{{#include ../../datatypes/ExamFileType.md}}

For now we will just look at the 'normal' exam type. This is the exam type that we will use for our first exam.

{{#include ../../datatypes/NormalExam.md}}

We can set a value for each of these fields.

```yaml
{{#include ../../../../examples/first_question/exams/first_exam.yaml}}
``` 
