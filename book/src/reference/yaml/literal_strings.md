# Literal strings

Literal strings are written after a pipe (<kbd>|</kbd>). Newlines are preserved.

```yaml
|
    This is a literal string.
    It can contain newlines.
    It can contain colons (:) followed by a space or a hashtag (#) after a space.

    It can contain single quotes (') without having to escape them.
    It can contain double quotes (") without having to escape them.
    It can contain backslashes (\) without having to escape them.
```

means

```text
This is a literal string.
It can contain newlines.
It can contain colons (:) followed by a space or a hashtag (#) after a space.

It can contain single quotes (') without having to escape them.
It can contain double quotes (") without having to escape them.
It can contain backslashes (\) without having to escape them.
```

As you can see, newlines are preserved. The text is literally as typed, except for the leading spaces that are removed.