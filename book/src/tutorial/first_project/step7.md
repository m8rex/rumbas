## Step 7: Using variables

This question will always ask the same question (3 * 9). We can make this question more interesting by using variables to do any number times 9.

Let's copy our `first_question.yaml` file to `first_question_with_variables.yaml` and add variables. To do this we will:
- Add a `variable` named `a` with a `value` of `3`
- Change the `statement`, `answer` and `correct_answer` so it uses the variable `a` instead of the number 3.

```yaml
{{#include ../../../../examples/first_project/questions/first_question_with_variables.yaml}}
```

```admonish question title="Task"
Copy the `first_question.yaml` file to `first_question_with_variables.yaml` and change the parts indicated in the file above.
```

Now we will create a new exam that uses this question. We will copy the `first_exam.yaml` file to `first_exam_with_variables.yaml` and add the new question to the `questions` field.

```yaml
{{#include ../../../../examples/first_project/exams/first_exam_with_variables.yaml}}
```

```admonish question title="Task"
Copy the `first_exam.yaml` file to `first_exam_with_variables.yaml` and add the question as indicated in the file above.
```

We can then compile all exams with the following command:
    
```bash
rumbas compile exams
```

And open the new exam in the browser by browsing to `http://localhost:8000/en/exams/first_exam_with_variables/`.

```admonish question title="Task"
Try answering the second question and see if it works.
```

```admonish question title="Task"
Also try the following things for the second question:
- Reload the page and use the `Reveal answers` button to see the correct answer. You will see that the calculations in the `marks` and `advice` fields are calculated.
- Try the `Try another question like this one` button. It won't do anything yet, because we are not using randomisation yet.
```

You can also find the exam in the [online demo](https://m8rex.github.io/rumbas/examples/first_project/en/exams/first_exam_with_variables/).
