# Rumbas (Rust + Numbas)

## What is Rumbas ?

- A system to create online (randomized) (math) exercises, build on top of [Numbas](https://www.numbas.org.uk/).
  - It is useful to https://docs.numbas.org.uk/en/latest/jme-reference.html
- All features of Numbas, are available in rumbas.
  - Randomised questions
    - You can use the Numbas [JME](https://docs.numbas.org.uk/en/latest/jme-reference.html) language to generate random numbers, do calculations, etc.
  - Diagnostic mode: the exam reacts to the student's performance to choose appropriate questions.
  - Rich interaction: easy to include graphics, videos and interactive diagrams (GeoGebra and JSXGraph) through extensions.
  - Many answer types: numbers, mathematical expressions, multiple choice, matrix ...
  - Automatic and immediate marking
  - In your language: translated in 19 languages and counting.
  - LTI Integration: [The Numbas LTI tool](https://docs.numbas.org.uk/lti/en/latest/) makes it easy to add Numbas assessments to virtual learning environments such as Blackboard, Canvas, Moodle and Brightspace. 
    - The result of a rumbas compilation is a Numbas exam, so all information you find about using numbas exams, can be used for rumbas exams.
  - Extensions: a specific subset of all Numbas extensions is supported
- More features:
  - Multi language: easily make the same exam available in different languages
  - Templating: when you create different questions/exams that are almost identical, you can extract these identical parts to a template to reduce redundancy.
  - Version control systems (like git) can be used 🎉:
    - Text-based: yaml (and html and tex) specifications are converted to an web-based exam.
- Currently in beta, not all features are implemented yet.
- Do you want to use it? Read the [Tutorial](tutorial.html).

## How does rumbas work?

- The content of _exams_ and _questions_ are specified in a *subset* of YAML. Please consult our [reference](./reference/yaml.html) for the specifics.
- The `rumbas` program converts these descriptions in YAML subset to a [Numbas](https://www.numbas.org.uk/) exam
- The `rumbas` program uses the `Numbas` codebase to convert these `Numbas` exams to an html exam.
