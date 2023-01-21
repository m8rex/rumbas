## Step 8: Using randomisation

Now we will create a question that uses randomisation. We will copy the `first_question_with_variables.yaml` file to `first_question_with_randomisation.yaml` and change the value of variable `a` to `random(1..10)`. This will choose a random integer between 1 and 10 as value for `a`.

```yaml
{{#include ../../../../examples/first_question/questions/first_question_with_randomisation.yaml}}
```

```admonish question title="Task"
Copy the `first_question_with_variables.yaml` file to `first_question_with_randomisation.yaml` and change the definition of the variables `a` as indicated in the file above.
```

Now we will create a new exam that uses this question. We will copy the `first_exam_with_variables.yaml` file to `first_exam_with_randomisation.yaml` and add the new question to the `questions` field.

```yaml
{{#include ../../../../examples/first_question/exams/first_exam_with_randomisation.yaml}}
```

```admonish question title="Task"
Copy the `first_exam_with_variables.yaml` file to `first_exam_with_randomisation.yaml` and add the question as indicated in the file above.
```

We can then compile all exams with the following command:
    
```bash
rumbas compile exams
```

And open the new exam in the browser by browsing to `http://localhost:8000/en/exams/first_exam_with_randomisation/`.

```admonish question title="Task"
Try answering the third question and see if it works.
```

```admonish question title="Task"
Also try the following things for the third question:
- Reload the page and use the `Reveal answers` button to see the correct answer. You will see that the calculations in the `marks` and `advice` fields are calculated.
- Try the `Try another question like this one` button.
  - For the third question, this will generate a new question with a new value for `a`.
  - You will see that after trying another question and using the 'Reveal answers' button, the advice is changed to use the new value of `a`.
```

You can also find the exam in the [online demo](https://m8rex.github.io/rumbas/examples/first_question/en/exams/first_exam_with_randomisation/).

```admonish success title="Congratulations"
This concludes our very first example of rumbas. In the next example we will look at the default value files to reduce the size of our question and exam files.
```
