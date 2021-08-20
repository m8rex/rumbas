# Contributing

## Conventions

- Don't use numbas types in rumbas types
  - Exceptions are:
    - String based types like `Primitive`, `JMEString`, `ContentAreaString`...

## Code style

### Data files (question and exam folder)

- Order in file:
  - If there is a question part structure, we place this on the top
  - For each type
    - Struct / Enum
    - RumbasCheck implementation (if done manually)
    - ToNumbas implementation
    - ToRumbas implementation
    - Other trait implementations:
      - JsonSchema
      - builtin traits
    - Other impls on structure

### Support files
