# Choice object with explicit tag

Sometimes it is needed to explicitly specify the type of a choice object. This is done by adding a specific key-value pair to the object.

This specification below says that we should write the yaml as one of the three given options and add the key-value pair `type: <tag-value>` to the object to specify which option we chose. This is denoted by the `Internal tag named type` text.


```admonish example title="Reference"
{{#include ../../datatypes/ExamFileType.md}}
```

If we want to choose the `template` option, we would write the following yaml for an `ExamFileType`:

```yaml
type: template # set the internal tag
# TemplateFile
template: path_to_template 
any_key: any_value
```

If we want to choose the `normal` option, we would write the following yaml for an `ExamFileType`:

```yaml
type: normal # set the internal tag
# NormalExam
name: The name of the exam
# ... more fields
```

If we want to choose the `diagnostic` option, we would write the following yaml for an `ExamFileType`:

```yaml
type: diagnostic # set the internal tag
# DiagnosticExam
name: The name of the diagnostic exam
# ... more fields
```