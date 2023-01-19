# Choice object

Some of the datatypes in the reference can take multiple forms. 

```admonish example title="Reference"
{{#include ../../datatypes/Translation.md}}
```

This datatype is either a `TranslationStruct` or a `FileString`.

To represent this datatype in yaml, you would either write a `TranslationStruct` or a `FileString`.

```admonish example title="Reference"
{{#include ../../datatypes/TranslationStruct.md}}
```

```yaml
# TranslationStruct
content: Some content
placeholders: {}
```

```admonish example title="Reference"
{{#include ../../datatypes/FileString.md}}
```


```yaml
# FileString
file:path/to/file
```

or 

```yaml
# FileString
any string value
```
