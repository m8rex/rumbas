# Yaml files

Each yaml file start with three dashes: `---` and is then followed by either a hash or an array.

```yaml
---
key1: value1
key2: value2
```

```yaml
---
- value1
- value2
```

## Comments

Comments can be written after a hashtag (#) The hashtag needs a space in front of it (in unquoted strings). They can be written on a separate line or after a value.

```yaml
# This is a comment before the opening dashes
---
key1: value1 # This is a comment
# This is a comment before a key
key2: value2 # This is another comment
```