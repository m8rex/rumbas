# Hashes

Hashes map keys to values. They are written as a list of key-value pairs, separated by colons. The keys may only contain letters, numbers, and underscores.

```yaml
key1: value1
key2: value2
```

The values can be any yaml value, even other hashes.

```yaml
key1: value1
level1:
  key2: value2
  level2:
    key3: value3
```

An empty hash can be written as `{}`.

```yaml
empty_subhash: {}
```