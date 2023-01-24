# Anchors and aliases

Anchors and aliases can be used to reference a value multiple times. This is useful when you want to use the same value multiple times, but don't want to write it multiple times.

Anchors are written after an ampersand (<kbd>&</kbd>). They are then followed by a key. The key can be any string, but it is recommended to use a unique key.

```yaml
key1: &anchor_key value
key2: *anchor_key
```

which is identical to

```yaml
key1: value
key2: value
```

It is also possible to use anchors for whole hashes and arrays.

```yaml
hash_value: &hash
  key1: value1
  key2: value2
also_hash_value: *hash
```

which is identical to

```yaml
hash_value:
  key1: value1
  key2: value2
also_hash_value:
  key1: value1
  key2: value2
```

It is obviously also possible to reference an anchor multiple times.

```yaml
a_key: &value_to_reuse This can be some long text or any other datatype
another_key: *value_to_reuse
third_key: *value_to_reuse
```

which is identical to

```yaml
a_key: This can be some long text or any other datatype
another_key: This can be some long text or any other datatype
third_key: This can be some long text or any other datatype
```
