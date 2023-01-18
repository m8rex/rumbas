# Arrays

Arrays are an ordered list of values. They can be written inline or as a block.

## Block arrays

Block arrays are written after a dash (-) and a space. Each value is written on a new line.

```yaml
- value1
- value2
```

The values can be any yaml value, even other arrays or hashes.

```yaml
hash_key: 
  - value1
  - value2
other_key:
  - key: value
    other: value
list_of_lists:
   - - value1
     - value2
   - - value3
     - value4
```

## Inline arrays

Inline arrays can be used to write a list of values on a single line. They are written between square brackets ([ and ]), with values separated by commas.

```yaml
[ value1, value2 ]
```

These values can be any value, except for block hashes or block arrays. 

This is mostly used to represent an empty array `[]` or an array of a small amount of strings.