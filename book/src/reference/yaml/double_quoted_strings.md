# Double quoted strings

Double quoted strings are written between double quotes.

```yaml
"This is a double quoted string"
```

means

```text
This is a double quoted string
```

Advantages:
- Can contain colons (:) followed by a space or a hashtag (#) after a space.
- Can contain single quotes (') without having to escape them.
- Newlines can be added by placing an escaped character (\) at the end of the line.

Disadvantages:
- Backslashes (\) are interpreted as escape characters. This means that you can't write a string containing a backslash without escaping it.
  - Special characters like newlines and tabs are interpreted as escape characters.
  - Makes it hard to type a string containing jme commands. Because you will need to escape each backslash.

```yaml
"It's a double quoted string with a jme simplify command \\simplify{2x+3}"
```

means

```text
It's a double quoted string  with a jme simplify command \simplify{2x+3}
```