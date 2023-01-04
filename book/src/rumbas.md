# Rumbas (Rust + Numbas)

## What is Rumbas ?

- A system to create online (randomized) (math) exercises, build on top of [Numbas](https://www.numbas.org.uk/).
- Text-based: yaml (and html and tex) specifications are converted to html files (e.g. scorm package)
  - So git can be used for version control 🎉
- Consistent naming of fields in the yaml (not the case in the json of `Numbas`)
- Written in Rust
- Preferred way to use it is docker
- Currently in beta, not all features are implemented yet.
- Do you want to use it? Read the [Tutorial](tutorial.html).

## How does rumbas work?

- `rumbas` is entirely file based
- The content of exams and questions are specified in [YAML](https://en.wikipedia.org/wiki/YAML)
- The `rumbas` program converts these descriptions in YAML to a [Numbas](https://www.numbas.org.uk/) exam
- The `rumbas` program uses `Numbas` to convert these `Numbas` exams to an html exam.
