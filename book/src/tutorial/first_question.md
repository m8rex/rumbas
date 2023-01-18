# Your first question

We will start your journey with rumbas by creating a simple question in the rumbas system. This question will be a simple calculation question. 

To start, we need to open a folder in which we will create our 'rumbas project'. 

## Step 1: Questions folder

Create the folder 'questions' within the folder of your rumbas project.

## Step 2: Create a question

Create a file named `first_question.yaml` within the folder 'questions'. This file will contain the yaml code for the question. To know which fields we can set for a question, you can look at the [Reference for question](./datatypes/complete_question.md) page. For now we will show the fields that can be set for a question:

{{#include ../datatypes/QuestionFileType.md}}

For now we will just look at the 'normal' question type.

{{#include ../datatypes/Question.md}}

We can set a value for each of these fields.

```yaml
{{#include ../../../examples/first_question/questions/first_question.yaml}}
```

## Step 3: Try compiling the first question

Now we can compile the question. To do this, we need to open a terminal in the folder of your rumbas project. Then we can run the following command:

```bash
rumbas check questions/first_question.yaml
```

This command will now show an error. This is because our rumbas repository is missing a crucial file. This file is the `rumbasrc.yaml` file. This file contains the settings for the rumbas repository. Create the file and add the following content:

```yaml
{{#include ../../../examples/first_question/.rumbasrc.yaml}}
```

This file is used to check whether you are using the right rumbas version for the rumbas project. If you are using the wrong version, you will get an error. 

## Step 4: Try compiling the first question again

Now we can try compiling the question again. This time we will get a different error. This error is because we are missing a crucial file in our rumbas project. This file is the `exams/questions_preview.yaml` file. This file is used to create a preview exam for a question. Because this file makes use a feature of rumbas that we will explain later (templating), we will create a specific first exam and compile that exam.

Create the exams folder and and an `exams/first_exam.yaml` file.

To know which fields we can set for a question, you can look at the [Reference for exam](./datatypes/complete_exam.md) page. For now we will show the fields that can be set for a question:

{{#include ../datatypes/ExamFileType.md}}

For now we will just look at the 'normal' exam type. This is the exam type that we will use for our first exam.

{{#include ../datatypes/NormalExam.md}}

We can set a value for each of these fields.

```yaml
{{#include ../../../examples/first_question/exams/first_exam.yaml}}
``` 

## Step 5: Compiling our first exam

Now we can compile the exam. To do this, we need to open a terminal in the folder of your rumbas project. Then we can run the following command:

```bash
rumbas check exams/first_exam.yaml
```

It should result in `All 1 checks passed!`. This means that the exam is valid and can be compiled.

```bash
rumbas compile exams/first_exam.yaml
```

This command will create a folder in `_output/en/exams/first_exam` with the compiled exam. To view this in the browser you should run a local webserver that hosts the `_output` folder.

Some options are:
- Live server extension in visual studio code (this is installed in our google codespaces setup)
- The [Web Server for Chrome](https://chrome.google.com/webstore/detail/web-server-for-chrome/ofhbbkphhbklhfoeikjpcbhemlocgigb) extension 
- Python: execute `python -m http.server` in the `_output` folder
- ...

## Step 6: Try the exam

After starting your local webserver, you should be able to browse to the exam. You can find the url in the terminal. It should be something like `http://localhost:8000/en/exams/first_exam/`. Try answering the question and see if it works.

You can also find the exam in the [online demo](https://m8rex.github.io/rumbas/examples/first_question/en/exams/first_exam/).