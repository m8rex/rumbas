# Basic objects

The [Exam](/datatypes/complete_exam.html), [Question](/datatypes/complete_question.html) and [Custom Part Type](/datatypes/complete_custom_part_type.html) reference shows tables that explain the dataformat for rumbas. In its most basic form, it looks like the following reference for the `QuestionPartMatrixRangedDimension` type which is used to specify the possible sizes for a matrix. You don't need to (and can't) understand what this type is used for.

```admonish example title="Reference"
{{#include ../../datatypes/QuestionPartMatrixRangedDimension.md}}
```

The tables lists three fields that has to be present in the yaml file. The first column is the name of the field, the second column is the datatype of the field and the third column is a description of the field.

To represent this datatype in yaml, you would write the following:

```yaml
default: 2
min: 1
max: 4 # could also be set to none to indicate no maximum
```

The order of the keys has no meaning, so we could just as well write:

```yaml
max: 4
min: 1
default: 2
```