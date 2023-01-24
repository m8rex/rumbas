# Hashes

Hashes map keys to values. They are written as a list of key-value pairs, separated by colons (<kbd>:</kbd>). The keys may only contain letters, numbers, and underscores.

```yaml
key1: value1
key2: value2
```

The values can be any yaml value, even other hashes.

```yaml
key1: value1
level1: # the value of this key is a hash
  key2: value2
  level2: # the value of this key is a hash
    key3: value3
```

The order of the keys in a hash has no meaning. The following two hashes are identical.

```yaml
some_key: value1
another_key: value2
```

```yaml
another_key: value2
some_key: value1
```

An empty hash can be written as `{}`.

```yaml
empty_subhash: {}
```