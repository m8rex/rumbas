# Folded strings

Folded strings are written after a greater than sign (<kbd>\></kbd>). Newlines are not preserved, they are replaced by a space. Blank lines can be used to insert a newline.

```yaml
>
    This is a folded string.
    It can contain newlines.

    It can contain colons (:) followed by a space or a hashtag (#) after a space.

    It can contain single quotes (') without having to escape them.
    It can contain double quotes (") without having to escape them.

    It can contain backslashes (\) without having to escape them.
```

means

```text
This is a folded string. It can contain newlines.
It can contain colons (:) followed by a space or a hashtag (#) after a space.
It can contain single quotes (') without having to escape them. It can contain double quotes (") without having to escape them.
It can contain backslashes (\) without having to escape them.
```

As you can see, newline are replaced by a space. Blank lines are replaced by a newline.