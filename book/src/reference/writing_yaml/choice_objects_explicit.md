# Choice object with explicit tag

Sometimes is it needed to explicitly specify the type of a choice object. This is done by adding a specific key-value pair to the object.

{{#include ../../datatypes/ExamFileType.md}}

This specification means that we should write the yaml as one of the three following options and add the key-value pair `type: <tag-value>` to the object.


```yaml
type: template # set the internal tag
# TemplateFile
template: path_to_template 
any_key: any_value
```
