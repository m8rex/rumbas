# Unquoted strings

Unquoted strings are the most basic form of string. They are written as a sequence of characters, without any quotes.

```yaml
This is an unquoted string
```

Advantages:
- Easy to write
- Backslashes (\\) are not interpreted as escape characters. This means that you can write a string containing a backslash without having to escape it.
  - Special characters like newlines and tabs are not interpreted as escape characters.
  - Makes it easy to type a string containing jme commands.

Disadvantages:
- Newlines (enters) can only be inserted by leaving a blank line.
- Can't contain colons (:) followed by a space or a hashtag (#) after a space.