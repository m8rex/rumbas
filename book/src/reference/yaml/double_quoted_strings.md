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
- Can contain colons (<kbd>:</kbd>) followed by a space or a hashtag (<kbd>#</kbd>) after a space.
- Can contain single quotes (<kbd>'</kbd>) without having to escape them.
- Newlines can be added by placing an escape character (<kbd>\\</kbd>) at the end of the line.

Disadvantages:
- Can't contain double quotes (<kbd>"</kbd>). They need to be escaped with a backslash (<kbd>\\</kbd>).
- Backslashes (<kbd>\\</kbd>) are interpreted as escape characters. This means that you can't write a string containing a backslash without escaping it.
  - Special characters like newlines and tabs are interpreted as escape characters.
  - Makes it hard to type a string containing jme commands. Because you will need to escape each backslash.

```yaml
"It's a double quoted string with a jme simplify command \\simplify{2x+3}"
```

means

```text
It's a double quoted string with a jme simplify command \simplify{2x+3}
```

We need to write two backslashes to get one backslash in the string. This is because the backslash is used as the escape character. If we want to type a backslash in the string we need to escape it with another backslash.