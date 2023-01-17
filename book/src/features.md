# Features

Rumbas is a library for creating and running assessments. It is designed to be used in a variety of ways, and can be used in a variety of contexts. This chapter describes the features of Rumbas, and how they can be used.

Rumbas tries to support all features that Numbas supports, and adds some new features. The new features are:
- Templating for questions and exams. This allows you to create the format of a question or exam once, and then use it in multiple questions or exams with different values.
- Multi language support. This allows you to create questions and exams in multiple languages while sharing the language independent parts.
  - Locale folders allow you to specify different values for different languages and to reuse these values in different questions or exams.
- Explicity: Rumbas does not use any default values within the program. All values must be explicitly set, and the program will not guess what you want. This makes it easier to understand what is going on, and to debug problems.
  - Default value files can (and should) be used to specify default values for your questions and exams.
