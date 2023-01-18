# Anchors and aliases

Anchors and aliases can be used to reference a value multiple times. This is useful when you want to use the same value multiple times, but don't want to write it multiple times.

Anchors are written after an ampersand (&) and a space. They are then followed by a key. The key can be any string, but it is recommended to use a unique key.

```yaml
first_time: &anchor_key value
second_time: *anchor_key
```

which is identical to

```yaml
first_time: value
second_time: value
```

It is also possible to use anchors for whole hashes and arrays.

```yaml
first_time: &anchor_key
  key1: value1
  key2: value2
second_time: *anchor_key
```

which is identical to

```yaml
first_time:
  key1: value1
  key2: value2
second_time:
  key1: value1
  key2: value2
```

It is obviously also possible to reference an anchor multiple times.

```yaml
first_time: &anchor_key value
second_time: *anchor_key
third_time: *anchor_key
```

which is identical to

```yaml
first_time: value
second_time: value
third_time: value
```
```