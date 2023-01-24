# Single quoted strings


Single quoted strings are written between single quotes.

```yaml
'This is a single quoted string'
```

means

```text
This is a single quoted string
```

Advantages:
- Can contain colons (<kbd>:</kbd>) followed by a space or a hashtag (<kbd>#</kbd>) after a space.
- Backslashes (<kbd>\\</kbd>) are not interpreted as escape characters. This means that you can write a string containing a backslash without having to escape it.
  - Special characters like newlines and tabs are not interpreted as escape characters.
  - Makes it easy to type a string containing jme commands.

Disadvantages:
- Newlines (enters) can only be inserted by leaving a blank line.
- Can't contain single quotes (<kbd>'</kbd>). They need to be escaped with a single quote (<kbd>'</kbd>).

```yaml
'It''s a single quoted string with a jme simplify command \simplify{2x+3}'
```

means

```text
It's a single quoted string  with a jme simplify command \simplify{2x+3}
```

As you can see we need to type two single quotes to get one single quote in the string. This is because the single quote is used to mark the start and end of the string. If we want to type a single quote in the string we need to escape it with another single quote.
