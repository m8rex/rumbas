# Writing Rumbas Yaml
The source code for any rumbas exam or question is written in a subset of YAML.

This page will show you how to write YAML files for Rumbas.

## Basic objects

The reference shows tables that explain the dataformat for rumbas. In its most basic form, it looks like the following table.

{{#include ../datatypes/QuestionPartMatrixRangedDimension.md}}

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

## Object with an [any] field

Some object have a field name `[any]` in their reference. This means that any other key-value pair can be added to the object.

{{#include ../datatypes/TemplateFile.md}}

This can be represented in yaml by adding any key-value pair to the object.

```yaml
template: path_to_template
any_key: any_value
another_key: another_value
```

## Choice object

Some of the datatypes in the reference can take multiple forms. 

{{#include ../datatypes/Translation.md}}

This datatype is either a `TranslationStruct` or a `FileString`.

To represent this datatype in yaml, you would either write a `TranslationStruct` or a `FileString`.

{{#include ../datatypes/TranslationStruct.md}}

```yaml
# TranslationStruct
content: Some content
placeholders: {}
```

{{#include ../datatypes/FileString.md}}


```yaml
# FileString
file:path/to/file
```

or 

```yaml
# FileString
any string value
```



## Choice object with explicit tag

Sometimes is it needed to explicitly specify the type of a choice object. This is done by adding a specific key-value pair to the object.

{{#include ../datatypes/ExamFileType.md}}

This specification means that we should write the yaml as one of the three following options and add the key-value pair `type: <tag-value>` to the object.


```yaml
type: template # set the internal tag
# TemplateFile
template: path_to_template 
any_key: any_value
```
