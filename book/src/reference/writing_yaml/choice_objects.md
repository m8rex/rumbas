# Choice object

Some of the datatypes in the reference can take multiple forms. 

This following datatype is either a `TranslationStruct` or a `FileString`. See the `One of the following items:` text.
```admonish example title="Reference"
{{#include ../../datatypes/Translation.md}}
```


To represent this datatype in yaml, you would either write a `TranslationStruct` or a `FileString`.

A `TranslationStruct` is specified as follows:

```admonish example title="Reference"
{{#include ../../datatypes/TranslationStruct.md}}
```

Which could be represented in yaml as follows:

```yaml
# TranslationStruct
content: Some content
placeholders: {}
```

The yaml above is thus also a valid yaml for a `Translation`.

A `FileString` is specified as follows:

```admonish example title="Reference"
{{#include ../../datatypes/FileString.md}}
```

Which could be represented in yaml as follows:

```yaml
# FileString
file:path/to/file
```

or 

```yaml
# FileString
any string value
```

The yaml above is thus also a valid yaml for a `Translation`.