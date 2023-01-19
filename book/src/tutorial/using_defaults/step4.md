# Step 4: Default question files

## Default question files

We can do the same thing for question files. For example, we can move the all question fields (except question_groups) to a default file.

```admonish question title="Task"
Move all fields of the questions that are equal to the `question.yaml` file in the `defaults` folder. For example:
```

```yaml
{{#include ../../../../examples/using_defaults/defaults/question.yaml}}
```

We also add the variables field in here and set it to an empty hash by default. This is because we will be able to override the variables field in the question files that do use variables.

```admonish question title="Task"
Remove the fields from the question files.
```

```admonish question title="Task"
Try compiling the exams again. You should see that the compilation works and that the questions are still the same within the exams.
```

Our question files are now a lot smaller. Our question with variables is now:

```yaml
{{#include ../../../../examples/using_defaults/questions/first_question_with_variables.yaml}}
```

### Default question part files

There is still some duplication of settings within the number_entry question parts. We can move the settings of the number_entry question parts to a default file.

```admonish question title="Task"
Move all fields of the number_entry question parts that are equal to the `questionpart.number_entry.yaml` file in the `defaults` folder. For example:
```


```yaml
{{#include ../../../../examples/using_defaults/questions/questionpart_defaults/defaults/questionpart.number_entry.yaml}}
```

```admonish info
For each questionpart type you can specify such a default file.
```

Our question files are now even smaller. Our question with variables is now:

```yaml
{{#include ../../../../examples/using_defaults/questions/questionpart_defaults/first_question_with_variables.yaml}}
```