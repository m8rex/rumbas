# Choice object

Some of the datatypes in the reference can take multiple forms. 

{{#include ../../datatypes/Translation.md}}

This datatype is either a `TranslationStruct` or a `FileString`.

To represent this datatype in yaml, you would either write a `TranslationStruct` or a `FileString`.

{{#include ../../datatypes/TranslationStruct.md}}

```yaml
# TranslationStruct
content: Some content
placeholders: {}
```

{{#include ../../datatypes/FileString.md}}


```yaml
# FileString
file:path/to/file
```

or 

```yaml
# FileString
any string value
```
