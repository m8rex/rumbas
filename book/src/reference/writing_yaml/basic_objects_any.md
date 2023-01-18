# Object with an [any] field

Some object have a field name `[any]` in their reference. This means that any other key-value pair can be added to the object.

{{#include ../../datatypes/TemplateFile.md}}

This can be represented in yaml by adding any key-value pair to the object.

```yaml
template: path_to_template
any_key: any_value
another_key: another_value
```