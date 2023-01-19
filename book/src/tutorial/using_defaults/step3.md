## Default exam files

For an exam we can specify a couple of different default files to specify global defaults for exams.

### Navigation

Our three exams use exactly the same navigation. We can therefore move the navigation field to a default file.

The navigation field of an exam can be specified in a file named `navigation.yaml` in the `defaults` folder.

```admonish question title="Task"
Place the value of the `navigation` field of the exam in this file. For example:
```

```yaml
{{#include ../../../../examples/using_defaults/defaults/navigation.yaml}}
```

```admonish question title="Task"
You can now remove the `navigation` field from the 3 exam files.
```

```admonish question title="Task"
Try compiling the exams again. You should see that the compilation works and that the navigation is still the same within the exams.
```

### Feedback

Our three exams use exactly the same feedback. We can therefore move the feedback field to a default file.

The feedback field of an exam can be specified in a file named `feedback.yaml` in the `defaults` folder.

```admonish question title="Task"
Place the value of the `feedback` field of the exam in this file. For example:
```

```yaml
{{#include ../../../../examples/using_defaults/exams/navigation/defaults/feedback.yaml}}
```

```admonish question title="Task"
You can now remove the `feedback` field from the 3 exam files.
```

```admonish question title="Task"
Try compiling the exams again. You should see that the compilation works and that the feedback is still the same within the exams.
```

### Timing

Our three exams use exactly the same timing. We can therefore move the timing field to a default file.

The timing field of an exam can be specified in a file named `timing.yaml` in the `defaults` folder.

```admonish question title="Task"
Place the value of the `timing` field of the exam in this file. For example:
```

```yaml
{{#include ../../../../examples/using_defaults/exams/navigation/feedback/defaults/timing.yaml}}
```

```admonish question title="Task"
You can now remove the `timing` field from the 3 exam files.
```

```admonish question title="Task"
Try compiling the exams again. You should see that the compilation works and that the timing is still the same within the exams.
```

### Locales

Our three exams use exactly the same locales. We can therefore move the locales field to a default file.

The locales field of an exam can be specified in a file named `locales.yaml` in the `defaults` folder.

```admonish question title="Task"
Place the value of the `locales` field of the exam in this file. For example:
```

```yaml
{{#include ../../../../examples/using_defaults/exams/navigation/feedback/timing/defaults/locales.yaml}}
```
```admonish question title="Task"
You can now remove the `locales` field from the 3 exam files.
```
```admonish question title="Task"
Try compiling the exams again. You should see that the compilation works and that the locales are still the same within the exams.
```

### Numbas Settings

Our three exams use exactly the same Numbas settings. We can therefore move the `numbas_settings` field to a default file.

The `numbas_settings` field of an exam can be specified in a file named `numbas_settings.yaml` in the `defaults` folder.

```admonish question title="Task"
Place the value of the `numbas_settings` field of the exam in this file. For example:
```

```yaml
{{#include ../../../../examples/using_defaults/exams/navigation/feedback/timing/locales/defaults/numbas_settings.yaml}}
```

```admonish question title="Task"
You can now remove the `numbas_settings` field from the 3 exam files.
```

```admonish question title="Task"
Try compiling the exams again. You should see that the compilation works and that the Numbas settings are still the same within the exams.
```

### Resulting exam files

Our exam files are now a lot smaller. Our exam with randomisation is now:

```yaml
{{#include ../../../../examples/using_defaults/exams/navigation/feedback/timing/locales/first_exam_with_randomisation.yaml}}
```

### Overriding defaults

These defaults are useful to reduce the size of your exam files. However, sometimes you will want to override the default values. For example, you might want to change the `show_current_marks` field of the `feedback` field for a specific exam. This can easily be done without needing to specify all fields of feedback.

```admonish question title="Task"
Override the `show_current_marks` field of the `feedback` field.
```

```yaml
{{#include ../../../../examples/using_defaults/exams/navigation/feedback/timing/locales/first_exam_with_randomisation_no_score.yaml}}
```

```admonish question title="Task"
Try compiling this exam. You should see that the compilation works and that the current total score is not shown in the exam.
```

You can also find the exam in the [online demo](https://m8rex.github.io/rumbas/examples/using_defaults/en/exams/navigation/feedback/timing/locales/first_exam_with_randomisation_no_score/).
