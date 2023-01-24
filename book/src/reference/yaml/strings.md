# Strings

Strings can be written in many ways in yaml, each with their specific advantages and disadvantages.

| type | When to use? |
|--------|------------------|
| [Unquoted strings](unquoted_strings.md) | Short text without hashtags (with a space in front) or colons (with a space behind). |
| [Single quoted strings](single_quoted_strings.md) | Short text where unquoted strings can't be used. |
| [Double quoted strings](double_quoted_strings.md) | Almost never. When you need to be able to write escaped sequences like `\n`. |
| [Folded strings](folded_strings.md) | When you have a longer piece of text that you would like to write on multiple lines. |
| [Literal strings](literal_strings.md) | When you absolutely need to have newlines in the resulting string. |
