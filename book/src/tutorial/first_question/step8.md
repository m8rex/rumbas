## Step 8: Using randomisation

Copy the `first_question_with_variables.yaml` file to `first_question_with_randomisation.yaml` and change the value of variable `a` to `random(1..10)`. This will choose a random integer between 1 and 10 as value for `a`.


```yaml
{{#include ../../../../examples/first_question/questions/first_question_with_randomisation.yaml}}
```

Copy the `first_exam_with_variables.yaml` file to `first_exam_with_randomisation.yaml` and add the new question to the `questions` field.

```yaml
{{#include ../../../../examples/first_question/exams/first_exam_with_randomisation.yaml}}
```

We can then compile all exams with the following command:
    
```bash
rumbas compile exams
```

And open the new exam in the browser by browsing to `http://localhost:8000/en/exams/first_exam_with_randomisation/`.


You should also try the following things for all questions:
- Reload the page and use the 'Reveal answers' button to see the correct answer. You will see that the calculations in the `marks` and `advice` fields are calculated.
- Try the 'Try another question like this one' button:
  - For the third question, this will generate a new question with a new value for `a`.
  - You will see that after trying another question and using the 'Reveal answers' button, the advice is changed to use the new value of `a`.

You can also find the exam in the [online demo](https://m8rex.github.io/rumbas/examples/first_question/en/exams/first_exam_with_randomisation/).

This concludes our very first example of rumbas. In the next example we will look at the default value files to reduce the size of our question and exam files.
