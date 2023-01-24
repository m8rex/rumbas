# Object with an [any] field

Some objects have a field name `[any]` in their reference. This means that any other key-value pair can be added to the object.

```admonish example title="Reference"
{{#include ../../datatypes/TemplateFile.md}}
```

This can be represented in yaml by adding any key-value pair to the hash.

```yaml
template: path_to_template
# we can add as many other key-value pairs as we want (due to the [any] field)
any_key: any_value
another_key: another_value
```