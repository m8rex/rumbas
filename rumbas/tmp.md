# ExamFileType
Internal tag named type.
One of the following items:
|tag-value|datatype of value|description|
|--|--|----|
|template|[TemplateFile](#TemplateFile)|An exam that uses a template.|
|normal|[NormalExam](#NormalExam)|A normal exam.|
|diagnostic|[DiagnosticExam](#DiagnosticExam)|An exam in diagnostic mode.|

# TemplateFile
|field|type|description|optional|
|--|--|----|-|
|template|String|The path to the template to use. Relative to the `exams` or `questions` folder.|false|
|[any]|YAML-value|The data to insert into the template. Maps template keys onto values.|false|

# NormalExam
|field|type|description|optional|
|--|--|----|-|
|name|[Translation](#Translation)|The name of the exam|false|
|locales|Array of [Locale](#Locale)|All locales for which the exam should be generated|false|
|navigation|[NormalNavigation](#NormalNavigation)|The navigation settings for this exam|false|
|timing|[Timing](#Timing)|The timing settings for this exam|false|
|feedback|[Feedback](#Feedback)|The feedback settings for this exam|false|
|question_groups|Array of [QuestionGroup](#QuestionGroup)|The questions groups for this exam|false|
|numbas_settings|[NumbasSettings](#NumbasSettings)|The settings to set for numbas|false|

# Translation
One of the following items:
|type|description|
|--|----|
|[TranslationStruct](#TranslationStruct)|A structured translatable string with placeholders.|
|[FileString](#FileString)|A simple filestring. This implies that it can also just be a string.|


# TranslationStruct
|field|type|description|optional|
|--|--|----|-|
|content|[TranslationContent](#TranslationContent)|The content with optional placeholders ({{placeholder-name}}).|false|
|placeholders|Map from String to [Translation](#Translation)|The values for the placeholders. It maps the placeholder-name to it's translatable value. The value for a placeholder can thus (if needed) be different for different locales.|false|

# TranslationContent
One of the following items:
|type|description|
|--|----|
|Map from String to [FileString](#FileString)|Map from locale to content. You can use this to specify different content for different locales.|
|[FileString](#FileString)|A filestring. Possibly to a file that is placed in `locale` folders and is therefore localized.|


# FileString
One of the following items:
|type|description|
|--|----|
|String|A string of the form `file:<filepath>` where `filepath` is the relative path (within the `exams` or `questions` folder) to a file containing content. This content can be localized by placing it in locale folders. e.g. `file:examples/basic-explanation.html` will search for files in folders with following form: `questions/examples/locale-<localename>/basic-explanation.html` If a file isn't found for a specific locale, `questions/examples/basic-explanation.html` will be used|
|String|A literal string.|


# Locale
|field|type|description|optional|
|--|--|----|-|
|name|String|The internal name used for the locale. It is best to use en for English, nl for dutch etc|false|
|numbas_locale|[SupportedLocale](#SupportedLocale)|The locale to use in the Numbas interface|false|

# SupportedLocale
One of the following items:
|name|description|
|--|----|
|"ar-SA"|Arabic (Saudi Arabia)|
|"de-DE"|German (Germany)|
|"en-GB"|English (United Kingdom)|
|"es-ES"|Spanish (Spain)|
|"fr-FR"|French (France)|
|"he-IL"|Hebrew (Israel)|
|"in-ID"|Indonesian (Indonesia)|
|"it-IT"|Italian (Italy)|
|"ja-JP"|Japanese (Japan)|
|"ko-KR"|Korean (Korea)|
|"nb-NO"|Norwegian (Norway)|
|"nl-NL"|Dutch (Netherlands)|
|"pl-PL"|Polish (Poland)|
|"pt-BR"|Portuguese (Brazil)|
|"sq-AL"|Albanian (Albania)|
|"sv-SE"|Swedish (Sweden)|
|"tr-TR"|Turkish (Turkey)|
|"vi-VN"|Vietnamese (Viet Nam)|
|"zg-CN"|Chinese (S)|

# NormalNavigation
Internal tag named mode.
One of the following items:
|tag-value|datatype of value|description|
|--|--|----|
|sequential|[SequentialNavigation](#SequentialNavigation)|Questions are shown in sequential order. Whether student can browse trough questions in any order, can be set.|
|menu|[MenuNavigation](#MenuNavigation)|Questions are shown in a menu and there is no real order of the questions.|

# SequentialNavigation
|field|type|description|optional|
|--|--|----|-|
|start_password|"none" or [FileString](#FileString)|Password to begin the exam none and "" are the same|false|
|can_regenerate|Boolean|Whether the student can regenerate questions Old name was `allow_regenerate`|false|
|show_steps|Boolean|If false,  then part steps will not be offered to the student, regardless of whether any have been defined in the exam’s questions Old name was `allow_steps`|false|
|show_title_page|Boolean|Whether the title page should be shown. Old name was `show_frontpage`|false|
|confirm_when_leaving|Boolean|Whether the student will be asked to confirm when leaving the exam.|false|
|show_names_of_question_groups|Boolean|Whether the names of the question groups should be shown.|false|
|allow_printing|Boolean|Whether the student is allowed to print the exam|false|
|can_move_to_previous|Boolean|Whether the student can move back to previous question (Old name was `reverse`)|false|
|browsing_enabled|Boolean|Whether the student can jump to any question.|false|
|show_results_page|[ShowResultsPage](#ShowResultsPage)|When the results page should be shown|false|
|on_leave|[LeaveAction](#LeaveAction)|Action to execute when a student changes question or tries to end the exam.|false|

# ShowResultsPage
One of the following items:
|name|description|
|--|----|
|"on_completion"|When the exam is completed.|
|"never"|Never show it.|

# LeaveAction
Internal tag named action.
One of the following items:
|tag-value|datatype of value|description|
|--|--|----|
|"none"||Don't show a warning|
|warn_if_not_attempted|[LeaveActionMessage](#LeaveActionMessage)|Warn when a question is not attempted.|
|prevent_if_not_attempted|[LeaveActionMessage](#LeaveActionMessage)|Prevent when a question is not attempted|

# LeaveActionMessage
|field|type|description|optional|
|--|--|----|-|
|message|[Translation](#Translation)|The message to show.|false|

# MenuNavigation
|field|type|description|optional|
|--|--|----|-|
|start_password|"none" or [FileString](#FileString)|Password to begin the exam none and "" are the same|false|
|can_regenerate|Boolean|Whether the student can regenerate questions Old name was `allow_regenerate`|false|
|show_steps|Boolean|If false,  then part steps will not be offered to the student, regardless of whether any have been defined in the exam’s questions Old name was `allow_steps`|false|
|show_title_page|Boolean|Whether the title page should be shown. Old name was `show_frontpage`|false|
|confirm_when_leaving|Boolean|Whether the student will be asked to confirm when leaving the exam.|false|
|show_names_of_question_groups|Boolean|Whether the names of the question groups should be shown.|false|
|allow_printing|Boolean|Whether the student is allowed to print the exam|false|

# Timing
|field|type|description|optional|
|--|--|----|-|
|duration_in_seconds|"none" or Integer|The maximal time that can be spend on the exam. If this value is `none` or 0, the student gets unlimited time.|false|
|allow_pause|Boolean|Wheher the 'pause' button is available.|false|
|on_timeout|[TimeoutAction](#TimeoutAction)|Action to do on timeout|false|
|timed_warning|[TimeoutAction](#TimeoutAction)|Action to do five minutes before timeout|false|

# TimeoutAction
Internal tag named action.
One of the following items:
|tag-value|datatype of value|description|
|--|--|----|
|"none"||Do nothing|
|warn|[TimeoutActionWarn](#TimeoutActionWarn)|Show a warning|

# TimeoutActionWarn
|field|type|description|optional|
|--|--|----|-|
|message|[Translation](#Translation)|The message to show|false|

# Feedback
|field|type|description|optional|
|--|--|----|-|
|percentage_needed_to_pass|"none" or Float|Specifies when a student passes the test. When set to "none" or 0, no percentage will be shown in frontpage.|false|
|show_name_of_student|Boolean|Whether the student's name should be shown in the exam.|false|
|show_current_marks|Boolean|Whether current marks are shown during exam or not (show_actual_mark in numbas)|false|
|show_maximum_marks|Boolean|Whether the maximal mark for a question (or the total exam) is shown (show_total_mark of numbas)|false|
|show_answer_state|Boolean|Whether answer feedback is shown (right or wrong etc)|false|
|allow_reveal_answer|Boolean|Whether the 'reveal answer' button is present|false|
|review|[Review](#Review)|The review settings|false|
|advice|[Translation](#Translation)|The advice shown at the end.|false|
|intro|[Translation](#Translation)|The introductory text|false|
|feedback_messages|Array of [FeedbackMessage](#FeedbackMessage)|Different feedback messages based on their score.|false|

# Review
|field|type|description|optional|
|--|--|----|-|
|show_score|Boolean|Whether to show score in result overview page|false|
|show_feedback|Boolean|Show feedback while reviewing|false|
|show_expected_answer|Boolean|Show expected answer while reviewing|false|
|show_advice|Boolean|Show advice while reviewing|false|

# FeedbackMessage
|field|type|description|optional|
|--|--|----|-|
|message|String|The message to show|false|
|threshold|String|The minimum score that is needed to get this feedback|false|

# QuestionGroup
|field|type|description|optional|
|--|--|----|-|
|name|[Translation](#Translation)|The name of the question group. Might be shown to students, based on the `show_names_of_question_groups` setting of the exam navigation.|false|
|picking_strategy|[PickingStrategy](#PickingStrategy)|The strategy to use to pick the questions to show|false|
|questions|Array of [QuestionPathOrTemplate](#QuestionPathOrTemplate)|The questions|false|

# PickingStrategy
Internal tag named type.
One of the following items:
|tag-value|datatype of value|description|
|--|--|----|
|"all_ordered"|||
|"all_shuffled"|||
|random_subset|[PickingStrategyRandomSubset](#PickingStrategyRandomSubset)||

# PickingStrategyRandomSubset
|field|type|description|optional|
|--|--|----|-|
|pick_questions|Integer|The amount of questions to pick|false|

# QuestionPathOrTemplate
One of the following items:
|type|description|
|--|----|
|String|The path to the question. Relative to the `questions` folder.|
|[TemplateFile](#TemplateFile)|Directly load a templated question by specifying the template values.|


# NumbasSettings
|field|type|description|optional|
|--|--|----|-|
|theme|String|The numbas theme to use|false|

# DiagnosticExam
|field|type|description|optional|
|--|--|----|-|
|name|[Translation](#Translation)|The name of the exam|false|
|locales|Array of [Locale](#Locale)|All locales for which the exam should be generated|false|
|navigation|[DiagnosticNavigation](#DiagnosticNavigation)|The navigation settings for this exam|false|
|timing|[Timing](#Timing)|The timing settings for this exam|false|
|feedback|[Feedback](#Feedback)|The feedback settings for this exam|false|
|question_groups|Array of [QuestionGroup](#QuestionGroup)|The questions groups for this exam|false|
|numbas_settings|[NumbasSettings](#NumbasSettings)|The settings to set for numbas|false|
|diagnostic|[Diagnostic](#Diagnostic)|The diagnostic data|false|

# DiagnosticNavigation
|field|type|description|optional|
|--|--|----|-|
|start_password|"none" or [FileString](#FileString)|Password to begin the exam none and "" are the same|false|
|can_regenerate|Boolean|Whether the student can regenerate questions Old name was `allow_regenerate`|false|
|show_steps|Boolean|If false,  then part steps will not be offered to the student, regardless of whether any have been defined in the exam’s questions Old name was `allow_steps`|false|
|show_title_page|Boolean|Whether the title page should be shown. Old name was `show_frontpage`|false|
|confirm_when_leaving|Boolean|Whether the student will be asked to confirm when leaving the exam.|false|
|show_names_of_question_groups|Boolean|Whether the names of the question groups should be shown.|false|
|allow_printing|Boolean|Whether the student is allowed to print the exam|false|
|on_leave|[LeaveAction](#LeaveAction)|Action to execute when a student changes question or tries to end the exam.|false|

# Diagnostic
|field|type|description|optional|
|--|--|----|-|
|script|[DiagnosticScript](#DiagnosticScript)|The script to use|false|
|objectives|Array of [LearningObjective](#LearningObjective)|The learning objectives,|false|
|topics|Array of [LearningTopic](#LearningTopic)|The learning topics|false|

# DiagnosticScript
One of the following items:
|name|description|
|--|----|
|"mastery"|The aim of the Mastery algorithm is to repeatedly test topics until the student passes them. Once all topics are passed, the exam ends. see https://docs.numbas.org.uk/en/latest/exam/diagnostic.html#mastery|
|"diagnosys"|The aim of the DIAGNOSYS algorithm is to efficiently establish which topics the student understands, and which they don’t. see https://docs.numbas.org.uk/en/latest/exam/diagnostic.html#diagnosys|
|"custom"|A custom diagnostic script. See https://docs.numbas.org.uk/en/latest/exam/diagnostic.html#writing-a-diagnostic-algorithm|

# LearningObjective
|field|type|description|optional|
|--|--|----|-|
|name|[Translation](#Translation)|The name of the learning objective|false|
|description|[Translation](#Translation)|A description of the learning objective|false|

# LearningTopic
|field|type|description|optional|
|--|--|----|-|
|name|[Translation](#Translation)|The name of the learning topic|false|
|description|[Translation](#Translation)|A description of the learning topic|false|
|objectives|Array of [Translation](#Translation)|List of names of objectives of which this topic consists|false|
|depends_on|Array of [Translation](#Translation)|List of names of topics on which this topic depends|false|


# QuestionFileType
Internal tag named type.
One of the following items:
|tag-value|datatype of value|description|
|--|--|----|
|template|[TemplateFile](#TemplateFile)|A question that uses a template|
|normal|[Question](#Question)|A normal question|

# TemplateFile
|field|type|description|optional|
|--|--|----|-|
|template|String|The path to the template to use. Relative to the `exams` or `questions` folder.|false|
|[any]|YAML-value|The data to insert into the template. Maps template keys onto values.|false|

# Question
|field|type|description|optional|
|--|--|----|-|
|statement|[Translation](#Translation)|The statement is a content area which appears at the top of the question, before any input boxes. Use the statement to set up the question and provide any information the student needs to answer it.|false|
|advice|[Translation](#Translation)|Advice is a content area which is shown when the student presses the Reveal button to reveal the question’s answers, or at the end of the exam. The advice area is normally used to present a worked solution to the question.|false|
|parts|Array of [QuestionPart](#QuestionPart)|A question consists of one or more parts. Each part can have a different type to create elaborate questions.|false|
|builtin_constants|[BuiltinConstants](#BuiltinConstants)|Specifies which constants are enabled. You might want to disable the constant e so it can be used as a variable in the questions.|false|
|custom_constants|Array of [CustomConstant](#CustomConstant)|Custom constants that are used in your question.|false|
|variables|Map from String to [VariableRepresentation](#VariableRepresentation)|The variables that are used in this question.|false|
|variables_test|[VariablesTest](#VariablesTest)|The test to which your variables should comply. Sometimes it’s hard to define randomised question variables so they’re guaranteed to produce a usable set of values. In these cases, it’s easier to state the condition you want the variables to satisfy, Variable values are generated until this condition passes.</br>While this tool allows you to pick sets of variables that would be hard to generate constructively, it’s a random process so you must be aware that there’s a chance no suitable set of values will ever be found.|false|
|functions|Map from String to [Function](#Function)|The functions that are used in this question|false|
|preamble|[Preamble](#Preamble)|Specify custom javascript and css code that should be loaded.|false|
|navigation|[QuestionNavigation](#QuestionNavigation)|Specify some navigation options for the question.|false|
|extensions|[Extensions](#Extensions)|Use this to enable the extensions that are used in the question|false|
|diagnostic_topic_names|Array of [Translation](#Translation)|The names of the topics used in diagnostic exams that this question belongs to|false|
|resources|Array of [ResourcePath](#ResourcePath)|The paths to the resources|false|
|custom_part_types|Array of [CustomPartTypeDefinitionPath](#CustomPartTypeDefinitionPath)|The custom part types used in this exam|false|
|rulesets|Map from String to [JMERulesetItem](#JMERulesetItem)|The rulesets defined in this question. A “ruleset” defines a list of named simplification rules used to manipulate mathematical expressions. https://numbas-editor.readthedocs.io/en/latest/question/reference.html#rulesets|false|

# Translation
One of the following items:
|type|description|
|--|----|
|[TranslationStruct](#TranslationStruct)|A structured translatable string with placeholders.|
|[FileString](#FileString)|A simple filestring. This implies that it can also just be a string.|


# TranslationStruct
|field|type|description|optional|
|--|--|----|-|
|content|[TranslationContent](#TranslationContent)|The content with optional placeholders ({{placeholder-name}}).|false|
|placeholders|Map from String to [Translation](#Translation)|The values for the placeholders. It maps the placeholder-name to it's translatable value. The value for a placeholder can thus (if needed) be different for different locales.|false|

# TranslationContent
One of the following items:
|type|description|
|--|----|
|Map from String to [FileString](#FileString)|Map from locale to content. You can use this to specify different content for different locales.|
|[FileString](#FileString)|A filestring. Possibly to a file that is placed in `locale` folders and is therefore localized.|


# FileString
One of the following items:
|type|description|
|--|----|
|String|A string of the form `file:<filepath>` where `filepath` is the relative path (within the `exams` or `questions` folder) to a file containing content. This content can be localized by placing it in locale folders. e.g. `file:examples/basic-explanation.html` will search for files in folders with following form: `questions/examples/locale-<localename>/basic-explanation.html` If a file isn't found for a specific locale, `questions/examples/basic-explanation.html` will be used|
|String|A literal string.|


# QuestionPart
One of the following items:
|type|description|
|--|----|
|[QuestionPartBuiltin](#QuestionPartBuiltin)|A question part using a builtin question part type|
|[QuestionPartCustom](#QuestionPartCustom)|A question part using a custom question part type|


# QuestionPartBuiltin
Internal tag named type.
One of the following items:
|tag-value|datatype of value|description|
|--|--|----|
|jme|[QuestionPartJME](#QuestionPartJME)|Mathematical expression parts require the student to enter an algebraic expression, using JME syntax.|
|gapfill|[QuestionPartGapFill](#QuestionPartGapFill)|Gap-fill parts allow you to include answer inputs inline with the prompt text, instead of at the end of the part. Each gap is a question part in itself.|
|choose_one|[QuestionPartChooseOne](#QuestionPartChooseOne)|Multiple choice part where the student must choose one of several options|
|choose_multiple|[QuestionPartChooseMultiple](#QuestionPartChooseMultiple)|Multiple choice part where the student can choose any of a list of options|
|match_answers|[QuestionPartMatchAnswersWithItems](#QuestionPartMatchAnswersWithItems)|The student is presented with a 2D grid of choices and answers. Depending on how the part is set up, they must either match up each choice with an answer, or select any number of choice-answer pairs.|
|number_entry|[QuestionPartNumberEntry](#QuestionPartNumberEntry)|Number entry parts ask the student to enter a number, which is marked if it is in a specified range|
|pattern_match|[QuestionPartPatternMatch](#QuestionPartPatternMatch)|Use a text pattern part when you want the student to enter short, non-mathematical text.|
|information|[QuestionPartInformation](#QuestionPartInformation)|An information part contains only a prompt and no answer input. It is most often used as a Step to provide a hint for a parent part.|
|extension|[QuestionPartExtension](#QuestionPartExtension)|An extension part acts as a placeholder for any interactive element added by an extension, or custom code in the question, which awards marks to the student.|
|matrix|[QuestionPartMatrix](#QuestionPartMatrix)|Matrix entry parts ask the student to enter a matrix of numbers. Marks are awarded if every cell in the student’s answer is equal to the corresponding cell in the correct answer, within the allowed margin of error.|

# QuestionPartJME
|field|type|description|optional|
|--|--|----|-|
|prompt|[Translation](#Translation)|A content area used to prompt the student for an answer.|false|
|marks|[Number](#Number)|The number of marks to award for answering the part correctly.|false|
|part_name|"none" or String|An optional custom part name, to use in part path's|false|
|show_correct_answer|Boolean|When the student reveals answers to the question, or views the question in review mode, should a correct answer be shown? You might want to turn this off if you’re doing custom marking and the part has no “correct” answer.|false|
|show_feedback_icon|Boolean|After the student submits an answer to this part, should an icon describing their score be shown? This is usually shown next to the input field, as well as in the feedback box. This option also controls whether feedback messages are shown for this part. You might want to turn this off if you’ve set up a question with a custom marking script which assigns a score based on the answers to two or more parts (or gapfills), meaning the individual parts have no independent “correct” or “incorrect” state.|false|
|custom_marking|"none" or [CustomMarking](#CustomMarking)|The marking algorithm tab allows you to customise the script used to mark the student’s answer, and test that it works correctly on answers that you provide.|false|
|steps|Array of [QuestionPart](#QuestionPart)|A (possibly empty) list of sub-parts which the student can reveal by clicking on a button. Marks awarded for steps don’t increase the total available for the part, but are given in case the student gets a lower score for the main part.|false|
|steps_penalty|[Number](#Number)|If the student reveals the Steps, reduce the total available marks by this amount. Credit for the part is scaled down accordingly. For example, if there are 6 marks available and the penalty for revealing steps is 2 marks, the total available after revealing steps is 4. An answer worth 3 marks without revealing steps is instead worth 3 * 4/6 = 2 marks after revealing steps.|false|
|adaptive_marking|"none" or [AdaptiveMarking](#AdaptiveMarking)|Adaptive marking allows you to incorporate the student’s answers to earlier parts when marking their answer to another part. You could use this to allow an “error carried forward” marking scheme, or in more free-form questions where one part has no correct answer - for example, “think of a number and find its square root”. This is achieved by replacing the values of question variables with the student’s answers to other parts. When a variable is replaced, any other variables depending on that one are recalculated using the new value. All other variables keep their original values. See for more info and a warning https://numbas-editor.readthedocs.io/en/latest/question/parts/reference.html#adaptive-marking|false|
|answer|[Translation](#Translation)|The expected answer to the part.|false|
|answer_simplification|[JMEAnswerSimplification](#JMEAnswerSimplification)|Simplification rules to apply to the correct answer, if it is displayed to the student (for example, after clicking the Reveal answers button). This shouldn’t affect marking. The simplification rules to apply to the answer|false|
|answer_display|[JMEAnswerDisplay](#JMEAnswerDisplay)|The display rules to apply to the answer|false|
|show_preview|Boolean|If ticked, a rendering of the student’s answer in mathematical notation is displayed beside the input box.|false|
|accuracy|[JMEAccuracy](#JMEAccuracy)|Defines the range of points over which the student’s answer will be compared with the correct answer, and the method used to compare them|false|
|check_variable_names|Boolean|If this is ticked, all variable names used in the student’s are checked against the variable names used in the correct answer. The first variable name which is not used in the correct answer will trigger a warning. You can use this option to prevent students incorrectly entering answers such as xy, which is interpreted as a single variable, when they mean x*y, the product of two variables.|false|
|single_letter_variables|Boolean|If this is ticked, long variable names will be interpreted as implicit multiplication of variables with single-letter names. For example, xyz will be interpreted as x * y * z. Digits, primes and single-letter underscores are still valid in variable names: a'x12y_z will be interpreted as a' * x12 * y_z. Greek letters are considered to be a single letter: pix will be interpreted as pi * x.|false|
|allow_unknown_functions|Boolean|If this is not ticked, the application of a function that is not defined in JME will be reinterpreted. If the function name can be split into several shorter names, each of which is defined in JME, it will be: for example, lnabs(x) will be interpreted as ln(abs(x)). Function names are recognised from right to left. Any remaining characters are interpreted as implicit multiplication by a variable. For example, xsin(x) will be interpreted as x * sin(x).|false|
|implicit_function_composition|Boolean|If this is ticked, the multiplication symbol (or implicit multiplication) will be interpreted as function composition when the right-hand side is a function application with one argument, and the left-hand side is the name of a function defined in JME. For example, ln * abs(x) and ln abs(x) will be interpreted as ln(abs(x)).|false|
|must_match_pattern|"none" or [JMEPatternRestriction](#JMEPatternRestriction)|The student’s answer must match the given pattern. If it does not, then a penalty is applied.|false|
|value_generators|"none" or Array of [JMEValueGenerator](#JMEValueGenerator)|Variable value generators override the default method used to pick values for variables when comparing the correct answer with the student’s answer. The expression for each variable can be written in terms of the other variables, as long as there are no circular dependencies. The values will be evaluated in order, like question variables. Each variable specified in the expected answer can be overriden The variable vRange represents the checking range defined for this part: a continuous interval between the checking range start and checking range end.|false|
|max_length|"none" or [JMELengthRestriction](#JMELengthRestriction)|*DEPRECATED* String restrictions are an unreliable method of restricting the form of a student’s answer. They are deprecated and retained only for backwards compatibility; use a pattern restriction instead.</br>Before string restrictions are applied, surplus brackets and whitespace are removed, and spaces are inserted between some operations, to minimise the possibility of the length restrictions being triggered for the wrong reasons.</br>If the student’s answer contains more than this many characters, the penalty is applied. A value of zero means no restriction is applied. The student’s answer is tidied up slightly so that things like extra or missing space characters don’t affect the calculated length. All spaces are removed, and then spaces are inserted between binary operations. For example, the answer 1+x (three characters) is marked as 1 + x (five characters).|false|
|min_length|"none" or [JMELengthRestriction](#JMELengthRestriction)|*DEPRECATED* String restrictions are an unreliable method of restricting the form of a student’s answer. They are deprecated and retained only for backwards compatibility; use a pattern restriction instead.</br>Before string restrictions are applied, surplus brackets and whitespace are removed, and spaces are inserted between some operations, to minimise the possibility of the length restrictions being triggered for the wrong reasons.</br>If the student’s answer contains fewer than this many characters, the penalty is applied. A value of zero means no restriction is applied.|false|
|must_have|"none" or [JMEStringRestriction](#JMEStringRestriction)|*DEPRECATED* String restrictions are an unreliable method of restricting the form of a student’s answer. They are deprecated and retained only for backwards compatibility; use a pattern restriction instead.</br>Before string restrictions are applied, surplus brackets and whitespace are removed, and spaces are inserted between some operations, to minimise the possibility of the length restrictions being triggered for the wrong reasons.</br>If the student’s answer doesn’t contain all of these strings, the penalty is applied.|false|
|may_not_have|"none" or [JMEStringRestriction](#JMEStringRestriction)|*DEPRECATED* String restrictions are an unreliable method of restricting the form of a student’s answer. They are deprecated and retained only for backwards compatibility; use a pattern restriction instead.</br>Before string restrictions are applied, surplus brackets and whitespace are removed, and spaces are inserted between some operations, to minimise the possibility of the length restrictions being triggered for the wrong reasons.</br>If the student’s answer contains any of these strings, the penalty is applied.|false|

# Number
One of the following items:
|type|description|
|--|----|
|Integer||
|Float||


# CustomMarking
|field|type|description|optional|
|--|--|----|-|
|algorithm_notes|Array of [JMENote](#JMENote)|This allows you to customise the script used to mark the student’s answer, and test that it works correctly on answers that you provide.|false|
|extend_base_marking_algorithm|Boolean|If this is ticked, all marking notes provided by the part’s standard marking algorithm will be available. If the same note is defined in both the standard algorithm and your custom algorithm, your version will be used.|false|

# JMENote
|field|type|description|optional|
|--|--|----|-|
|name|String||false|
|description|"none" or String||false|
|expression|[Translation](#Translation)||false|

# AdaptiveMarking
|field|type|description|optional|
|--|--|----|-|
|variable_replacements|Array of [VariableReplacement](#VariableReplacement)|The variable replacements to do|false|
|variable_replacement_strategy|[VariableReplacementStrategy](#VariableReplacementStrategy)|The circumstances under which the variable replacements are used, and adaptive marking is applied.|false|
|penalty|Integer|If adaptive marking is used, reduce the total available marks by this amount. Credit for the part is scaled down accordingly. See steps_penalty for an example.|false|

# VariableReplacement
|field|type|description|optional|
|--|--|----|-|
|variable|String|The name of the variable to replace|false|
|part_answer_to_use|String|The path to the part whose answer the variable’s value should be replaced with. Different part types produce different types of values.|false|
|must_be_answered|Boolean|If this is ticked, the student must submit an answer to the referenced part before they can submit an answer to this part.|false|

# VariableReplacementStrategy
One of the following items:
|name|description|
|--|----|
|"original_first"|The student’s answer is first marked using the original values of the question variables. If the credit given by this method is less than the maximum available, the marking is repeated using the defined variable replacements. If the credit gained with variable replacements is greater than the credit gained under the original marking, that score is used, and the student is told that their answers to previous parts have been used in the marking for this part.|
|"always_replace"|The student’s answer is only marked once, with the defined variable replacements applied.|

# JMEAnswerSimplification
|field|type|description|optional|
|--|--|----|-|
|simplify_basic|Boolean|Some basic rules: https://numbas-editor.readthedocs.io/en/latest/simplification.html?highlight=simplification#term-basic|false|
|cancel_unit_factors|Boolean|Cancel products of 1|false|
|cancel_unit_powers|Boolean|Cancel exponents of 1|false|
|cancel_unit_denominators|Boolean|Cancel fractions with denominator 1|false|
|cancel_zero_factors|Boolean|Cancel products of zero to zero|false|
|omit_zero_terms|Boolean|Omit zero terms|false|
|cancel_zero_powers|Boolean|Cancel exponents of 0|false|
|cancel_powers_with_base_zero|Boolean|Cancel any power of zero|false|
|collect_numbers|Boolean|Collect together numerical (as opposed to variable) products and sums.|false|
|constants_first|Boolean|Numbers go to the left of multiplications|false|
|collect_sqrt_products|Boolean|Collect products of square roots|false|
|collect_sqrt_divisions|Boolean|Collect fractions of square roots|false|
|cancel_sqrt_square|Boolean|Cancel square roots of squares, and squares of square roots|false|
|evaluate_powers_of_numbers|Boolean|Evaluate powers of numbers.|false|
|rewrite_to_no_leading_minus|Boolean|Rearrange expressions so they don’t start with a unary minus|false|
|simplify_fractions|Boolean|Cancel fractions to lowest form|false|
|simplify_trigonometric|Boolean|Simplify some trigonometric identities|false|
|collect_terms|Boolean|Collect together and cancel terms. Like collectNumbers, but for any kind of term.|false|
|collect_powers_of_common_factors|Boolean|Collect together powers of common factors.|false|
|collect_like_fractions|Boolean|Collect together fractions over the same denominator.|false|
|order_canonical|Boolean|Rearrange the expression into a “canonical” order, using canonical_compare.</br>Note: This rule can not be used at the same time as rewrite_to_no_leading_minus - it can lead to an infinite loop.|false|
|expand_brackets|Boolean|Expand out products of sums.|false|

# JMEAnswerDisplay
|field|type|description|optional|
|--|--|----|-|
|broken_as_fractions|Boolean|This rule doesn’t rewrite expressions, but tells the maths renderer that you’d like non-integer numbers to be displayed as fractions instead of decimals.|false|
|mixed_fractions|Boolean|Improper fractions (with numerator larger than the denominator) are displayed in mixed form, as an integer next to a proper fraction.|false|
|flat_fractions|Boolean|Fractions are displayed on a single line, with a slash between the numerator and denominator.|false|
|vector_as_row|Boolean|This rule doesn’t rewrite expressions, but tells the maths renderer that you’d like vectors to be rendered as rows instead of columns.|false|
|always_show_multiplication_sign|Boolean|The multiplication symbol is always included between multiplicands.|false|
|use_dot_as_multiplication_sign|Boolean|Use a dot for the multiplication symbol instead of a cross.|false|
|matrices_without_parentheses|Boolean|Matrices are rendered without parentheses.|false|

# JMEAccuracy
|field|type|description|optional|
|--|--|----|-|
|checking_type|[CheckingType](#CheckingType)|The rule to use to compare the student’s answer with the correct answer.|false|
|checking_range|Array of Float|The minimum and maximum value sample points can take [minimum, maximum]|false|
|points_to_check|Integer|The number of comparisons to make between the student’s answer and the correct answer.|false|
|max_failures|Float|If the comparison fails this many times or more, the student’s answer is marked as wrong.|false|

# CheckingType
Internal tag named type.
One of the following items:
|tag-value|datatype of value|description|
|--|--|----|
|relative_difference|[CheckingTypeDataFloat](#CheckingTypeDataFloat)|Fail if studentanswer / expectedanswer - 1 > amount|
|absolute_difference|[CheckingTypeDataFloat](#CheckingTypeDataFloat)|Fail if abs(studentanswer - expectedanswer) > amount|
|decimal_places|[CheckingTypeDataNatural](#CheckingTypeDataNatural)|x and y are rounded to a certain amount of decimal places, and the test fails if the rounded values are unequal|
|significant_figures|[CheckingTypeDataNatural](#CheckingTypeDataNatural)|x and y are rounded to significant figures, and the test fails if the rounded values are unequal.|

# CheckingTypeDataFloat
|field|type|description|optional|
|--|--|----|-|
|max_difference|Float|Maximum relative or absolute difference|false|

# CheckingTypeDataNatural
|field|type|description|optional|
|--|--|----|-|
|amount|Integer|Amount of decimal places or significant figures|false|

# JMEPatternRestriction
|field|type|description|optional|
|--|--|----|-|
|partial_credit|Float|If the student’s answer does not match the given pattern, their score is multiplied by this percentage.|false|
|message|[Translation](#Translation)|Warning message|false|
|pattern|String|See https://numbas-editor.readthedocs.io/en/latest/pattern-matching/examples.html#pattern-matching-examples for example patterns|false|
|name_to_compare|String|The part of the expression to mark|false|

# JMEValueGenerator
|field|type|description|optional|
|--|--|----|-|
|name|[FileString](#FileString)|The name of the variable|false|
|value|[JMEFileString](#JMEFileString)|The expression to generate the value|false|

# JMEFileString
One of the following items:
|type|description|
|--|----|
|String|A string of the form `file:<filepath>` where `filepath` is the relative path (within the `exams` or `questions` folder) to a file containing content. This content can be localized by placing it in locale folders. e.g. `file:examples/basic-explanation.html` will search for files in folders with following form: `questions/examples/locale-<localename>/basic-explanation.html` If a file isn't found for a specific locale, `questions/examples/basic-explanation.html` will be used|
|String|A literal string.|


# JMELengthRestriction
|field|type|description|optional|
|--|--|----|-|
|partial_credit|Float|The partial credit (percentage) attributed when failing the restriction|false|
|message|[Translation](#Translation)|Warning message|false|
|length|Integer|The minimum or maximum length|false|

# JMEStringRestriction
|field|type|description|optional|
|--|--|----|-|
|partial_credit|Float|The partial credit (percentage) attributed when failing the restriction|false|
|message|[Translation](#Translation)|Warning message|false|
|strings|Array of [Translation](#Translation)|The strings that are required or forbidden|false|
|show_strings|Boolean|Whether to show the strings|false|

# QuestionPartGapFill
|field|type|description|optional|
|--|--|----|-|
|prompt|[Translation](#Translation)|A content area used to prompt the student for an answer.|false|
|marks|[Number](#Number)|The number of marks to award for answering the part correctly.|false|
|part_name|"none" or String|An optional custom part name, to use in part path's|false|
|show_correct_answer|Boolean|When the student reveals answers to the question, or views the question in review mode, should a correct answer be shown? You might want to turn this off if you’re doing custom marking and the part has no “correct” answer.|false|
|show_feedback_icon|Boolean|After the student submits an answer to this part, should an icon describing their score be shown? This is usually shown next to the input field, as well as in the feedback box. This option also controls whether feedback messages are shown for this part. You might want to turn this off if you’ve set up a question with a custom marking script which assigns a score based on the answers to two or more parts (or gapfills), meaning the individual parts have no independent “correct” or “incorrect” state.|false|
|custom_marking|"none" or [CustomMarking](#CustomMarking)|The marking algorithm tab allows you to customise the script used to mark the student’s answer, and test that it works correctly on answers that you provide.|false|
|steps|Array of [QuestionPart](#QuestionPart)|A (possibly empty) list of sub-parts which the student can reveal by clicking on a button. Marks awarded for steps don’t increase the total available for the part, but are given in case the student gets a lower score for the main part.|false|
|steps_penalty|[Number](#Number)|If the student reveals the Steps, reduce the total available marks by this amount. Credit for the part is scaled down accordingly. For example, if there are 6 marks available and the penalty for revealing steps is 2 marks, the total available after revealing steps is 4. An answer worth 3 marks without revealing steps is instead worth 3 * 4/6 = 2 marks after revealing steps.|false|
|adaptive_marking|"none" or [AdaptiveMarking](#AdaptiveMarking)|Adaptive marking allows you to incorporate the student’s answers to earlier parts when marking their answer to another part. You could use this to allow an “error carried forward” marking scheme, or in more free-form questions where one part has no correct answer - for example, “think of a number and find its square root”. This is achieved by replacing the values of question variables with the student’s answers to other parts. When a variable is replaced, any other variables depending on that one are recalculated using the new value. All other variables keep their original values. See for more info and a warning https://numbas-editor.readthedocs.io/en/latest/question/parts/reference.html#adaptive-marking|false|
|sort_answers|Boolean|If ticked, then the student’s answers will be put in ascending order before the gaps are marked. The lowest answer will be submitted against the first gap, and so on. Because the order of marking might not correspond with the order in which the gaps are shown to the student, no feedback icon is shown next to the gap input boxes, only in the feedback summary for the whole part.|false|
|gaps|Array of [QuestionPart](#QuestionPart)|The question parts for the gaps|false|

# QuestionPartChooseOne
|field|type|description|optional|
|--|--|----|-|
|prompt|[Translation](#Translation)|A content area used to prompt the student for an answer.|false|
|marks|[Number](#Number)|The number of marks to award for answering the part correctly.|false|
|part_name|"none" or String|An optional custom part name, to use in part path's|false|
|show_correct_answer|Boolean|When the student reveals answers to the question, or views the question in review mode, should a correct answer be shown? You might want to turn this off if you’re doing custom marking and the part has no “correct” answer.|false|
|show_feedback_icon|Boolean|After the student submits an answer to this part, should an icon describing their score be shown? This is usually shown next to the input field, as well as in the feedback box. This option also controls whether feedback messages are shown for this part. You might want to turn this off if you’ve set up a question with a custom marking script which assigns a score based on the answers to two or more parts (or gapfills), meaning the individual parts have no independent “correct” or “incorrect” state.|false|
|custom_marking|"none" or [CustomMarking](#CustomMarking)|The marking algorithm tab allows you to customise the script used to mark the student’s answer, and test that it works correctly on answers that you provide.|false|
|steps|Array of [QuestionPart](#QuestionPart)|A (possibly empty) list of sub-parts which the student can reveal by clicking on a button. Marks awarded for steps don’t increase the total available for the part, but are given in case the student gets a lower score for the main part.|false|
|steps_penalty|[Number](#Number)|If the student reveals the Steps, reduce the total available marks by this amount. Credit for the part is scaled down accordingly. For example, if there are 6 marks available and the penalty for revealing steps is 2 marks, the total available after revealing steps is 4. An answer worth 3 marks without revealing steps is instead worth 3 * 4/6 = 2 marks after revealing steps.|false|
|adaptive_marking|"none" or [AdaptiveMarking](#AdaptiveMarking)|Adaptive marking allows you to incorporate the student’s answers to earlier parts when marking their answer to another part. You could use this to allow an “error carried forward” marking scheme, or in more free-form questions where one part has no correct answer - for example, “think of a number and find its square root”. This is achieved by replacing the values of question variables with the student’s answers to other parts. When a variable is replaced, any other variables depending on that one are recalculated using the new value. All other variables keep their original values. See for more info and a warning https://numbas-editor.readthedocs.io/en/latest/question/parts/reference.html#adaptive-marking|false|
|answer_data|[MultipleChoiceAnswerData](#MultipleChoiceAnswerData)|Specify the options, score per option and feedback per option. Old name was `answers`|false|
|shuffle_answers|Boolean|If this is ticked, the choices are displayed in random order.|false|
|show_cell_answer_state|Boolean|If ticked, choices selected by the student will be highlighted as ‘correct’ if they have a positive score, and ‘incorrect’ if they are worth zero or negative marks. If this is not ticked, the ticked choices will be given a neutral highlight regardless of their scores.|false|
|display|[ChooseOneDisplay](#ChooseOneDisplay)|How should the options be shown?|false|

# MultipleChoiceAnswerData
One of the following items:
|type|description|
|--|----|
|Array of [MultipleChoiceAnswer](#MultipleChoiceAnswer)|Specify a list of answer with it's marks and feedback|
|[MultipleChoiceAnswerDataNumbasLike](#MultipleChoiceAnswerDataNumbasLike)|Specify the answers, marks and feedback as separate lists. The first answers, matches the first mark and the first feedback etc|


# MultipleChoiceAnswer
|field|type|description|optional|
|--|--|----|-|
|statement|[Translation](#Translation)|The statement of the answer|false|
|feedback|[Translation](#Translation)|The feedback shown when this answer is chosen|false|
|marks|[Translation](#Translation)|The marks to assign when this answer is chosen|false|

# MultipleChoiceAnswerDataNumbasLike
|field|type|description|optional|
|--|--|----|-|
|answers|jme-string or Array of [Translation](#Translation)|The possible answers|false|
|marks|jme-string or Array of [Translation](#Translation)|The marks for the corresponding answers|false|
|feedback|"none" or Array of [Translation](#Translation)|The feedback for the corresponding answers.|false|

# ChooseOneDisplay
Internal tag named type.
One of the following items:
|tag-value|datatype of value|description|
|--|--|----|
|"dropdown"||“Drop down list” means that the choices are shown as a selection box; the student can click to show the choices in a vertical list.|
|radio|[ChooseOneDisplay_radio](#ChooseOneDisplay_radio)|“Radio” means that choices are shown separately, in-line with the part prompt.|

# ChooseOneDisplay_radio
|field|type|description|optional|
|--|--|----|-|
|columns|Integer|This dictates how many columns the choices are displayed in. If 0, the choices are displayed on a single line, wrapped at the edges of the screen.|false|

# QuestionPartChooseMultiple
|field|type|description|optional|
|--|--|----|-|
|prompt|[Translation](#Translation)|A content area used to prompt the student for an answer.|false|
|marks|[Number](#Number)|The number of marks to award for answering the part correctly.|false|
|part_name|"none" or String|An optional custom part name, to use in part path's|false|
|show_correct_answer|Boolean|When the student reveals answers to the question, or views the question in review mode, should a correct answer be shown? You might want to turn this off if you’re doing custom marking and the part has no “correct” answer.|false|
|show_feedback_icon|Boolean|After the student submits an answer to this part, should an icon describing their score be shown? This is usually shown next to the input field, as well as in the feedback box. This option also controls whether feedback messages are shown for this part. You might want to turn this off if you’ve set up a question with a custom marking script which assigns a score based on the answers to two or more parts (or gapfills), meaning the individual parts have no independent “correct” or “incorrect” state.|false|
|custom_marking|"none" or [CustomMarking](#CustomMarking)|The marking algorithm tab allows you to customise the script used to mark the student’s answer, and test that it works correctly on answers that you provide.|false|
|steps|Array of [QuestionPart](#QuestionPart)|A (possibly empty) list of sub-parts which the student can reveal by clicking on a button. Marks awarded for steps don’t increase the total available for the part, but are given in case the student gets a lower score for the main part.|false|
|steps_penalty|[Number](#Number)|If the student reveals the Steps, reduce the total available marks by this amount. Credit for the part is scaled down accordingly. For example, if there are 6 marks available and the penalty for revealing steps is 2 marks, the total available after revealing steps is 4. An answer worth 3 marks without revealing steps is instead worth 3 * 4/6 = 2 marks after revealing steps.|false|
|adaptive_marking|"none" or [AdaptiveMarking](#AdaptiveMarking)|Adaptive marking allows you to incorporate the student’s answers to earlier parts when marking their answer to another part. You could use this to allow an “error carried forward” marking scheme, or in more free-form questions where one part has no correct answer - for example, “think of a number and find its square root”. This is achieved by replacing the values of question variables with the student’s answers to other parts. When a variable is replaced, any other variables depending on that one are recalculated using the new value. All other variables keep their original values. See for more info and a warning https://numbas-editor.readthedocs.io/en/latest/question/parts/reference.html#adaptive-marking|false|
|answer_data|[MultipleChoiceAnswerData](#MultipleChoiceAnswerData)|Specify the options, score per option and feedback per option. Old name was `answers`|false|
|shuffle_answers|Boolean|If this is ticked, the choices are displayed in random order.|false|
|show_cell_answer_state|Boolean|If ticked, choices selected by the student will be highlighted as ‘correct’ if they have a positive score, and ‘incorrect’ if they are worth zero or negative marks. If this is not ticked, the ticked choices will be given a neutral highlight regardless of their scores.|false|
|should_select_at_least|Integer|The student must select at least this many choices. The value 0 means “no minimum”, though the student must make at least one choice to submit the part.|false|
|should_select_at_most|"none" or Integer|The student must select at most this many choices. The value 0 means “no maximum”.|false|
|columns|Integer|This dictates how many columns the choices are displayed in. If 0, the choices are displayed on a single line, wrapped at the edges of the screen.|false|
|wrong_nb_answers_warning_type|[MultipleChoiceWarningType](#MultipleChoiceWarningType)|What to do if the student picks the wrong number of responses? Either "none" (do nothing), "prevent" (don’t let the student submit), or "warn" (show a warning but let them submit)|false|
|minimal_achievable_marks|"none" or Integer|If the student would have scored less than this many marks, they are instead awarded this many. Useful in combination with negative marking.|false|
|maximal_achievable_marks|"none" or Integer|If the student would have scored more than this many marks, they are instead awarded this many. The value 0 means “no maximum mark”.|false|
|marking_method|[MultipleChoiceMarkingMethod](#MultipleChoiceMarkingMethod)|This determines how the student’s score is determined, based on their selections and the marking matrix.|false|

# MultipleChoiceWarningType
One of the following items:
|name|description|
|--|----|
|"none"|Do nothing|
|"prevent"|Prevent submission until they pick an acceptable number of answers|

# MultipleChoiceMarkingMethod
One of the following items:
|name|description|
|--|----|
|"sum_ticked_cells"|For each checkbox the student ticks, the corresponding entry in the marking matrix is added to their score. Unticked cells are ignored.</br>This marking method is suitable for situations where the student should only select choices they’re sure about. You could apply negative marks for incorrect choices.|
|"score_per_matched_cell"|For each checkbox, the student is awarded an equal proportion of the Maximum marks, if their selection for that cell matches the marking matrix. A positive value in the marking matrix signifies that the student should tick that checkbox, while a value of zero signifies that the student should not tick that box.</br>This marking method is suitable for situations where the student must separate the available choices into two sets.|
|"all_or_nothing"|the student is awarded the Maximum marks available if their selection exactly matches the marking matrix, and zero marks otherwise.</br>This marking method is suitable for situations where the student must exactly match a certain pattern, and there is no meaningful “partially correct” answer.|

# QuestionPartMatchAnswersWithItems
|field|type|description|optional|
|--|--|----|-|
|prompt|[Translation](#Translation)|A content area used to prompt the student for an answer.|false|
|marks|[Number](#Number)|The number of marks to award for answering the part correctly.|false|
|part_name|"none" or String|An optional custom part name, to use in part path's|false|
|show_correct_answer|Boolean|When the student reveals answers to the question, or views the question in review mode, should a correct answer be shown? You might want to turn this off if you’re doing custom marking and the part has no “correct” answer.|false|
|show_feedback_icon|Boolean|After the student submits an answer to this part, should an icon describing their score be shown? This is usually shown next to the input field, as well as in the feedback box. This option also controls whether feedback messages are shown for this part. You might want to turn this off if you’ve set up a question with a custom marking script which assigns a score based on the answers to two or more parts (or gapfills), meaning the individual parts have no independent “correct” or “incorrect” state.|false|
|custom_marking|"none" or [CustomMarking](#CustomMarking)|The marking algorithm tab allows you to customise the script used to mark the student’s answer, and test that it works correctly on answers that you provide.|false|
|steps|Array of [QuestionPart](#QuestionPart)|A (possibly empty) list of sub-parts which the student can reveal by clicking on a button. Marks awarded for steps don’t increase the total available for the part, but are given in case the student gets a lower score for the main part.|false|
|steps_penalty|[Number](#Number)|If the student reveals the Steps, reduce the total available marks by this amount. Credit for the part is scaled down accordingly. For example, if there are 6 marks available and the penalty for revealing steps is 2 marks, the total available after revealing steps is 4. An answer worth 3 marks without revealing steps is instead worth 3 * 4/6 = 2 marks after revealing steps.|false|
|adaptive_marking|"none" or [AdaptiveMarking](#AdaptiveMarking)|Adaptive marking allows you to incorporate the student’s answers to earlier parts when marking their answer to another part. You could use this to allow an “error carried forward” marking scheme, or in more free-form questions where one part has no correct answer - for example, “think of a number and find its square root”. This is achieved by replacing the values of question variables with the student’s answers to other parts. When a variable is replaced, any other variables depending on that one are recalculated using the new value. All other variables keep their original values. See for more info and a warning https://numbas-editor.readthedocs.io/en/latest/question/parts/reference.html#adaptive-marking|false|
|answer_data|[MultipleChoiceMatchAnswerData](#MultipleChoiceMatchAnswerData)|Specify the options, score per option and feedback per option. Old name was `answers`|false|
|shuffle_answers|Boolean|If this is ticked, the choices are displayed in random order.|false|
|shuffle_items|Boolean|If this is ticked, the items (horizontal) are displayed in random order.|false|
|show_cell_answer_state|Boolean|If ticked, choices selected by the student will be highlighted as ‘correct’ if they have a positive score, and ‘incorrect’ if they are worth zero or negative marks. If this is not ticked, the ticked choices will be given a neutral highlight regardless of their scores.|false|
|should_select_at_least|Integer|The student must select at least this many choices. The value 0 means “no minimum”, though the student must make at least one choice to submit the part.|false|
|should_select_at_most|"none" or Integer|The student must select at most this many choices. The value 0 means “no maximum”.|false|
|display|[MatchAnswerWithItemsDisplay](#MatchAnswerWithItemsDisplay)|How should the options be shown?|false|
|layout|[MatchAnswersWithChoicesLayout](#MatchAnswersWithChoicesLayout)|How should the options be shown?|false|
|wrong_nb_answers_warning_type|[MultipleChoiceWarningType](#MultipleChoiceWarningType)|What to do if the student picks the wrong number of responses? Either "none" (do nothing), "prevent" (don’t let the student submit), or "warn" (show a warning but let them submit)|false|
|minimal_achievable_marks|"none" or Integer|If the student would have scored less than this many marks, they are instead awarded this many. Useful in combination with negative marking.|false|
|maximal_achievable_marks|"none" or Integer|If the student would have scored more than this many marks, they are instead awarded this many. The value 0 means “no maximum mark”.|false|

# MultipleChoiceMatchAnswerData
Internal tag named type.
One of the following items:
|tag-value|datatype of value|description|
|--|--|----|
|item_based|[MultipleChoiceMatchAnswers](#MultipleChoiceMatchAnswers)|Specify a list of answers and a list of items with marks for different answers|
|numbas_like|[MultipleChoiceMatchAnswerDataNumbasLike](#MultipleChoiceMatchAnswerDataNumbasLike)|Specify a list of answers, choices and marks in separate lists.|

# MultipleChoiceMatchAnswers
|field|type|description|optional|
|--|--|----|-|
|answers|Array of [Translation](#Translation)|Values of the answers|false|
|items|Array of [MatchAnswersItem](#MatchAnswersItem)|Items for which the answer can be selected|false|

# MatchAnswersItem
|field|type|description|optional|
|--|--|----|-|
|statement|[Translation](#Translation)|The statement for the item|false|
|answer_marks|Array of [MatchAnswersItemMarks](#MatchAnswersItemMarks)|Map points to strings of answers ! use anchors in yaml|false|

# MatchAnswersItemMarks
|field|type|description|optional|
|--|--|----|-|
|marks|[JMEString](#JMEString)|The marks a student get's for selecting the answer|false|
|answer|[Translation](#Translation)|The answer that yields marks for the item|false|

# JMEString
|field|type|description|optional|
|--|--|----|-|
|s|String||false|
|ast|||false|

# MultipleChoiceMatchAnswerDataNumbasLike
|field|type|description|optional|
|--|--|----|-|
|answers|jme-string or Array of [Translation](#Translation)|The possible answers|false|
|choices|jme-string or Array of [Translation](#Translation)|The possible choices|false|
|marks|jme-string or Array of Array of [JMEString](#JMEString)|The marks for the corresponding answers|false|

# MatchAnswerWithItemsDisplay
Internal tag named type.
One of the following items:
|tag-value|datatype of value|description|
|--|--|----|
|"radio"||One from each row|
|check|[MatchAnswersWithChoicesDisplayCheck](#MatchAnswersWithChoicesDisplayCheck)|Any number from each row|

# MatchAnswersWithChoicesDisplayCheck
|field|type|description|optional|
|--|--|----|-|
|marking_method|[MultipleChoiceMarkingMethod](#MultipleChoiceMarkingMethod)|The marking method to use|false|

# MatchAnswersWithChoicesLayout
|field|type|description|optional|
|--|--|----|-|
|type|[MatchAnswersWithChoicesLayoutType](#MatchAnswersWithChoicesLayoutType)|Which fields should be shown|false|

# MatchAnswersWithChoicesLayoutType
One of the following items:
|name|description|
|--|----|
|"all"|All options are shown|
|"lower_triangle"|Only the lower triangle is shown|

# QuestionPartNumberEntry
|field|type|description|optional|
|--|--|----|-|
|prompt|[Translation](#Translation)|A content area used to prompt the student for an answer.|false|
|marks|[Number](#Number)|The number of marks to award for answering the part correctly.|false|
|part_name|"none" or String|An optional custom part name, to use in part path's|false|
|show_correct_answer|Boolean|When the student reveals answers to the question, or views the question in review mode, should a correct answer be shown? You might want to turn this off if you’re doing custom marking and the part has no “correct” answer.|false|
|show_feedback_icon|Boolean|After the student submits an answer to this part, should an icon describing their score be shown? This is usually shown next to the input field, as well as in the feedback box. This option also controls whether feedback messages are shown for this part. You might want to turn this off if you’ve set up a question with a custom marking script which assigns a score based on the answers to two or more parts (or gapfills), meaning the individual parts have no independent “correct” or “incorrect” state.|false|
|custom_marking|"none" or [CustomMarking](#CustomMarking)|The marking algorithm tab allows you to customise the script used to mark the student’s answer, and test that it works correctly on answers that you provide.|false|
|steps|Array of [QuestionPart](#QuestionPart)|A (possibly empty) list of sub-parts which the student can reveal by clicking on a button. Marks awarded for steps don’t increase the total available for the part, but are given in case the student gets a lower score for the main part.|false|
|steps_penalty|[Number](#Number)|If the student reveals the Steps, reduce the total available marks by this amount. Credit for the part is scaled down accordingly. For example, if there are 6 marks available and the penalty for revealing steps is 2 marks, the total available after revealing steps is 4. An answer worth 3 marks without revealing steps is instead worth 3 * 4/6 = 2 marks after revealing steps.|false|
|adaptive_marking|"none" or [AdaptiveMarking](#AdaptiveMarking)|Adaptive marking allows you to incorporate the student’s answers to earlier parts when marking their answer to another part. You could use this to allow an “error carried forward” marking scheme, or in more free-form questions where one part has no correct answer - for example, “think of a number and find its square root”. This is achieved by replacing the values of question variables with the student’s answers to other parts. When a variable is replaced, any other variables depending on that one are recalculated using the new value. All other variables keep their original values. See for more info and a warning https://numbas-editor.readthedocs.io/en/latest/question/parts/reference.html#adaptive-marking|false|
|answer|[NumberEntryAnswer](#NumberEntryAnswer)|The expected answer|false|
|display_correct_as_fraction|Boolean|If this is ticked, the correct answer to the part will be rendered as a fraction of two whole numbers instead of a decimal. For example, if the answer is 0.5, it will be displayed as 1/2 instead of 0.5.|false|
|allow_fractions|Boolean|If this is ticked, the student can enter a ratio of two whole numbers, e.g. -3/8, as their answer.|false|
|fractions_must_be_reduced|Boolean|This option only applies when “allow_fractions” is ticked. If this is ticked, the student must enter their fractional answer reduced to lowest terms. For example, consider a part whose correct answer is 5/4. If this is ticked, 10/8 will be marked as incorrect.|false|
|fractions_must_be_reduced_hint|Boolean|If this is ticked and fractions_must_be_reduced is ticked, then text explaining that the student must reduce their fraction to lowest terms is shown next to the input box.|false|
|partial_credit_if_fraction_not_reduced|[Number](#Number)|The proportion of credit to award if the student’s answer is a non-reduced fraction.|false|
|allowed_notation_styles|Array of [AnswerStyle](#AnswerStyle)|The styles of number notation that the student can use to enter their answer. There are different ways of writing numbers, based on culture and context. Tick an option to allow the student to use that style in their answer. Note that some styles conflict with each other: for example, 1.234 is a number between 1 and 2 in English, while it’s the integer 1234 in French. The student’s answer will be interpreted using the first allowed style for which it is a valid representation of a number.|false|
|display_correct_in_style|[AnswerStyle](#AnswerStyle)|The style of number notation to use when displaying the student’s answer.|false|

# NumberEntryAnswer
One of the following items:
|type|description|
|--|----|
|[JMEString](#JMEString)|The answer is accepted as correct when it equals this value|
|[NumberEntryAnswerRange](#NumberEntryAnswerRange)|The answer is accepted as correct when it is within a range|


# NumberEntryAnswerRange
|field|type|description|optional|
|--|--|----|-|
|from|[JMEString](#JMEString)|The smallest value accepted as correct.|false|
|to|[JMEString](#JMEString)|The largest value accepted as correct.|false|

# AnswerStyle
One of the following items:
|name|description|
|--|----|
|"english"|English style - commas separate thousands, dot for decimal point|
|"english-plain"|Plain English style - no thousands separator, dot for decimal point|
|"english-si"|English SI style - spaces separate thousands, dot for decimal point|
|"european"|Continental European style - dots separate thousands, comma for decimal poin|
|"european-plain"|Plain French style - no thousands separator, comma for decimal point|
|"french-si"|French SI style - spaces separate thousands, comma for decimal point|
|"indian"|Indian style - commas separate groups, dot for decimal point. The rightmost group is three digits, other groups are two digits.|
|"scientific"|Significand-exponent ("scientific") style|
|"swiss"|Swiss style - apostrophes separate thousands, dot for decimal point|

# QuestionPartPatternMatch
|field|type|description|optional|
|--|--|----|-|
|prompt|[Translation](#Translation)|A content area used to prompt the student for an answer.|false|
|marks|[Number](#Number)|The number of marks to award for answering the part correctly.|false|
|part_name|"none" or String|An optional custom part name, to use in part path's|false|
|show_correct_answer|Boolean|When the student reveals answers to the question, or views the question in review mode, should a correct answer be shown? You might want to turn this off if you’re doing custom marking and the part has no “correct” answer.|false|
|show_feedback_icon|Boolean|After the student submits an answer to this part, should an icon describing their score be shown? This is usually shown next to the input field, as well as in the feedback box. This option also controls whether feedback messages are shown for this part. You might want to turn this off if you’ve set up a question with a custom marking script which assigns a score based on the answers to two or more parts (or gapfills), meaning the individual parts have no independent “correct” or “incorrect” state.|false|
|custom_marking|"none" or [CustomMarking](#CustomMarking)|The marking algorithm tab allows you to customise the script used to mark the student’s answer, and test that it works correctly on answers that you provide.|false|
|steps|Array of [QuestionPart](#QuestionPart)|A (possibly empty) list of sub-parts which the student can reveal by clicking on a button. Marks awarded for steps don’t increase the total available for the part, but are given in case the student gets a lower score for the main part.|false|
|steps_penalty|[Number](#Number)|If the student reveals the Steps, reduce the total available marks by this amount. Credit for the part is scaled down accordingly. For example, if there are 6 marks available and the penalty for revealing steps is 2 marks, the total available after revealing steps is 4. An answer worth 3 marks without revealing steps is instead worth 3 * 4/6 = 2 marks after revealing steps.|false|
|adaptive_marking|"none" or [AdaptiveMarking](#AdaptiveMarking)|Adaptive marking allows you to incorporate the student’s answers to earlier parts when marking their answer to another part. You could use this to allow an “error carried forward” marking scheme, or in more free-form questions where one part has no correct answer - for example, “think of a number and find its square root”. This is achieved by replacing the values of question variables with the student’s answers to other parts. When a variable is replaced, any other variables depending on that one are recalculated using the new value. All other variables keep their original values. See for more info and a warning https://numbas-editor.readthedocs.io/en/latest/question/parts/reference.html#adaptive-marking|false|
|case_sensitive|Boolean|If this is ticked, the capitalisation of the student’s answer must match that of the answer pattern. If it doesn’t, partial credit will be awarded.|false|
|wrong_case_partial_credit|Float|The partial credits awarded if the students capitalisation is wrong|false|
|pattern|[Translation](#Translation)|The text or pattern the student must match.|false|
|display_answer|[Translation](#Translation)|A representative correct answer string to display to the student, in case they press the Reveal answers button.|false|
|match_mode|[PatternMatchMode](#PatternMatchMode)|The test to use to decide if the student’s answer is correct. Some examples https://numbas-editor.readthedocs.io/en/latest/question/parts/match-text-pattern.html#regular-expressions|false|

# PatternMatchMode
One of the following items:
|name|description|
|--|----|
|"regex"|The pattern is interpreted as a regular expression (https://developer.mozilla.org/en-US/docs/JavaScript/Guide/Regular_Expressions)|
|"exact"|Marks the student’s answer as correct only if it is exactly the same as the text given in Answer pattern. Space characters are removed from the start and end of the student’s answer as well as the answer pattern before comparison.|

# QuestionPartInformation
|field|type|description|optional|
|--|--|----|-|
|prompt|[Translation](#Translation)|A content area used to prompt the student for an answer.|false|
|marks|[Number](#Number)|The number of marks to award for answering the part correctly.|false|
|part_name|"none" or String|An optional custom part name, to use in part path's|false|
|show_correct_answer|Boolean|When the student reveals answers to the question, or views the question in review mode, should a correct answer be shown? You might want to turn this off if you’re doing custom marking and the part has no “correct” answer.|false|
|show_feedback_icon|Boolean|After the student submits an answer to this part, should an icon describing their score be shown? This is usually shown next to the input field, as well as in the feedback box. This option also controls whether feedback messages are shown for this part. You might want to turn this off if you’ve set up a question with a custom marking script which assigns a score based on the answers to two or more parts (or gapfills), meaning the individual parts have no independent “correct” or “incorrect” state.|false|
|custom_marking|"none" or [CustomMarking](#CustomMarking)|The marking algorithm tab allows you to customise the script used to mark the student’s answer, and test that it works correctly on answers that you provide.|false|
|steps|Array of [QuestionPart](#QuestionPart)|A (possibly empty) list of sub-parts which the student can reveal by clicking on a button. Marks awarded for steps don’t increase the total available for the part, but are given in case the student gets a lower score for the main part.|false|
|steps_penalty|[Number](#Number)|If the student reveals the Steps, reduce the total available marks by this amount. Credit for the part is scaled down accordingly. For example, if there are 6 marks available and the penalty for revealing steps is 2 marks, the total available after revealing steps is 4. An answer worth 3 marks without revealing steps is instead worth 3 * 4/6 = 2 marks after revealing steps.|false|
|adaptive_marking|"none" or [AdaptiveMarking](#AdaptiveMarking)|Adaptive marking allows you to incorporate the student’s answers to earlier parts when marking their answer to another part. You could use this to allow an “error carried forward” marking scheme, or in more free-form questions where one part has no correct answer - for example, “think of a number and find its square root”. This is achieved by replacing the values of question variables with the student’s answers to other parts. When a variable is replaced, any other variables depending on that one are recalculated using the new value. All other variables keep their original values. See for more info and a warning https://numbas-editor.readthedocs.io/en/latest/question/parts/reference.html#adaptive-marking|false|

# QuestionPartExtension
|field|type|description|optional|
|--|--|----|-|
|prompt|[Translation](#Translation)|A content area used to prompt the student for an answer.|false|
|marks|[Number](#Number)|The number of marks to award for answering the part correctly.|false|
|part_name|"none" or String|An optional custom part name, to use in part path's|false|
|show_correct_answer|Boolean|When the student reveals answers to the question, or views the question in review mode, should a correct answer be shown? You might want to turn this off if you’re doing custom marking and the part has no “correct” answer.|false|
|show_feedback_icon|Boolean|After the student submits an answer to this part, should an icon describing their score be shown? This is usually shown next to the input field, as well as in the feedback box. This option also controls whether feedback messages are shown for this part. You might want to turn this off if you’ve set up a question with a custom marking script which assigns a score based on the answers to two or more parts (or gapfills), meaning the individual parts have no independent “correct” or “incorrect” state.|false|
|custom_marking|"none" or [CustomMarking](#CustomMarking)|The marking algorithm tab allows you to customise the script used to mark the student’s answer, and test that it works correctly on answers that you provide.|false|
|steps|Array of [QuestionPart](#QuestionPart)|A (possibly empty) list of sub-parts which the student can reveal by clicking on a button. Marks awarded for steps don’t increase the total available for the part, but are given in case the student gets a lower score for the main part.|false|
|steps_penalty|[Number](#Number)|If the student reveals the Steps, reduce the total available marks by this amount. Credit for the part is scaled down accordingly. For example, if there are 6 marks available and the penalty for revealing steps is 2 marks, the total available after revealing steps is 4. An answer worth 3 marks without revealing steps is instead worth 3 * 4/6 = 2 marks after revealing steps.|false|
|adaptive_marking|"none" or [AdaptiveMarking](#AdaptiveMarking)|Adaptive marking allows you to incorporate the student’s answers to earlier parts when marking their answer to another part. You could use this to allow an “error carried forward” marking scheme, or in more free-form questions where one part has no correct answer - for example, “think of a number and find its square root”. This is achieved by replacing the values of question variables with the student’s answers to other parts. When a variable is replaced, any other variables depending on that one are recalculated using the new value. All other variables keep their original values. See for more info and a warning https://numbas-editor.readthedocs.io/en/latest/question/parts/reference.html#adaptive-marking|false|

# QuestionPartMatrix
|field|type|description|optional|
|--|--|----|-|
|prompt|[Translation](#Translation)|A content area used to prompt the student for an answer.|false|
|marks|[Number](#Number)|The number of marks to award for answering the part correctly.|false|
|part_name|"none" or String|An optional custom part name, to use in part path's|false|
|show_correct_answer|Boolean|When the student reveals answers to the question, or views the question in review mode, should a correct answer be shown? You might want to turn this off if you’re doing custom marking and the part has no “correct” answer.|false|
|show_feedback_icon|Boolean|After the student submits an answer to this part, should an icon describing their score be shown? This is usually shown next to the input field, as well as in the feedback box. This option also controls whether feedback messages are shown for this part. You might want to turn this off if you’ve set up a question with a custom marking script which assigns a score based on the answers to two or more parts (or gapfills), meaning the individual parts have no independent “correct” or “incorrect” state.|false|
|custom_marking|"none" or [CustomMarking](#CustomMarking)|The marking algorithm tab allows you to customise the script used to mark the student’s answer, and test that it works correctly on answers that you provide.|false|
|steps|Array of [QuestionPart](#QuestionPart)|A (possibly empty) list of sub-parts which the student can reveal by clicking on a button. Marks awarded for steps don’t increase the total available for the part, but are given in case the student gets a lower score for the main part.|false|
|steps_penalty|[Number](#Number)|If the student reveals the Steps, reduce the total available marks by this amount. Credit for the part is scaled down accordingly. For example, if there are 6 marks available and the penalty for revealing steps is 2 marks, the total available after revealing steps is 4. An answer worth 3 marks without revealing steps is instead worth 3 * 4/6 = 2 marks after revealing steps.|false|
|adaptive_marking|"none" or [AdaptiveMarking](#AdaptiveMarking)|Adaptive marking allows you to incorporate the student’s answers to earlier parts when marking their answer to another part. You could use this to allow an “error carried forward” marking scheme, or in more free-form questions where one part has no correct answer - for example, “think of a number and find its square root”. This is achieved by replacing the values of question variables with the student’s answers to other parts. When a variable is replaced, any other variables depending on that one are recalculated using the new value. All other variables keep their original values. See for more info and a warning https://numbas-editor.readthedocs.io/en/latest/question/parts/reference.html#adaptive-marking|false|
|correct_answer|[JMEString](#JMEString)|The expected answer to the part. This is a JME expression which must evaluate to a matrix.|false|
|dimensions|[QuestionPartMatrixDimensions](#QuestionPartMatrixDimensions)|The dimensions of the student's answer field|false|
|max_absolute_deviation|Float|If the absolute difference between the student’s value for a particular cell and the correct answer’s is less than this value, then it will be marked as correct.|false|
|mark_partial_by_cells|Boolean|If this is set to true, the student will be awarded marks according to the proportion of cells that are marked correctly. If this is not ticked, they will only receive the marks for the part if they get every cell right. If their answer does not have the same dimensions as the correct answer, they are always awarded zero marks.|false|
|display_correct_as_fraction|Boolean|If this is ticked, then non-integer numbers in the correct answer will be displayed as fractions instead of decimals.|false|
|allow_fractions|Boolean|If this is ticked, the student can enter a ratio of two whole numbers, e.g. -3/8, as their answer.|false|

# QuestionPartMatrixDimensions
|field|type|description|optional|
|--|--|----|-|
|rows|[QuestionPartMatrixDimension](#QuestionPartMatrixDimension)|The number of rows in the student’s answer field.|false|
|columns|[QuestionPartMatrixDimension](#QuestionPartMatrixDimension)|The number of columns in the student’s answer field.|false|

# QuestionPartMatrixDimension
One of the following items:
|name|description|
|--|----|
|"Fixed"|The dimensions are fixed|
|"Resizable"|The student can change the dimensions|

# QuestionPartMatrixRangedDimension
|field|type|description|optional|
|--|--|----|-|
|default|jme-string or Integer|The default size|false|
|min|jme-string or Integer|The minimal size|false|
|max|"none" or jme-string or Integer|The maximal size, if this is none, there is no limit|false|

# QuestionPartCustom
|field|type|description|optional|
|--|--|----|-|
|prompt|[Translation](#Translation)|A content area used to prompt the student for an answer.|false|
|marks|[Number](#Number)|The number of marks to award for answering the part correctly.|false|
|part_name|"none" or String|An optional custom part name, to use in part path's|false|
|show_correct_answer|Boolean|When the student reveals answers to the question, or views the question in review mode, should a correct answer be shown? You might want to turn this off if you’re doing custom marking and the part has no “correct” answer.|false|
|show_feedback_icon|Boolean|After the student submits an answer to this part, should an icon describing their score be shown? This is usually shown next to the input field, as well as in the feedback box. This option also controls whether feedback messages are shown for this part. You might want to turn this off if you’ve set up a question with a custom marking script which assigns a score based on the answers to two or more parts (or gapfills), meaning the individual parts have no independent “correct” or “incorrect” state.|false|
|custom_marking|"none" or [CustomMarking](#CustomMarking)|The marking algorithm tab allows you to customise the script used to mark the student’s answer, and test that it works correctly on answers that you provide.|false|
|steps|Array of [QuestionPart](#QuestionPart)|A (possibly empty) list of sub-parts which the student can reveal by clicking on a button. Marks awarded for steps don’t increase the total available for the part, but are given in case the student gets a lower score for the main part.|false|
|steps_penalty|[Number](#Number)|If the student reveals the Steps, reduce the total available marks by this amount. Credit for the part is scaled down accordingly. For example, if there are 6 marks available and the penalty for revealing steps is 2 marks, the total available after revealing steps is 4. An answer worth 3 marks without revealing steps is instead worth 3 * 4/6 = 2 marks after revealing steps.|false|
|adaptive_marking|"none" or [AdaptiveMarking](#AdaptiveMarking)|Adaptive marking allows you to incorporate the student’s answers to earlier parts when marking their answer to another part. You could use this to allow an “error carried forward” marking scheme, or in more free-form questions where one part has no correct answer - for example, “think of a number and find its square root”. This is achieved by replacing the values of question variables with the student’s answers to other parts. When a variable is replaced, any other variables depending on that one are recalculated using the new value. All other variables keep their original values. See for more info and a warning https://numbas-editor.readthedocs.io/en/latest/question/parts/reference.html#adaptive-marking|false|
|type|String|The name of the custom part name|false|
|settings|Map from String to [CustomPartInputTypeValue](#CustomPartInputTypeValue)|The settings for the CustomPartType|false|

# BuiltinConstants
|field|type|description|optional|
|--|--|----|-|
|e|Boolean|Whether the constant e is enabled|false|
|pi|Boolean|Whether the constant pi is enabled|false|
|i|Boolean|Whether the constant i is enabled-|false|

# CustomConstant
|field|type|description|optional|
|--|--|----|-|
|name|String|The name of the constant|false|
|value|[JMEString](#JMEString)|The value of the constant|false|
|tex|String|The tex code use to display the constant|false|

# VariablesTest
|field|type|description|optional|
|--|--|----|-|
|condition|[JMEString](#JMEString)|A JME expression which should evaluate to true when the set of variables generated has the properties you want. For example, if a, b and c are the coefficients of a quadratic equation and you want it to have real roots, the condition could be b^2-4*a*c>=0.|false|
|max_runs|Integer|The maximum number of times the system should regenerate the set of variables without finding a set which satisfies the condition before giving up. If the system exceeds this number in a compiled exam, the entire exam will fail, so try to avoid it!|false|

# Preamble
|field|type|description|optional|
|--|--|----|-|
|js|[FileString](#FileString)|The JavaScript to add to the outputfiles|false|
|css|[FileString](#FileString)|The CSS to add to the outputfiles|false|

# QuestionNavigation
|field|type|description|optional|
|--|--|----|-|
|can_regenerate|Boolean|Whether the student can regenerate the question Old name was `allow_regenerate`|false|
|show_title_page|Boolean|Whether the title page should be shown. Old name was `show_frontpage`|false|
|confirm_when_leaving|Boolean|Whether the student will be asked to confirm when leaving the exam.|false|

# Extensions
|field|type|description|optional|
|--|--|----|-|
|chemistry|Boolean||false|
|download_text_file|Boolean||false|
|eukleides|Boolean|The Eukleides extension provides functions to embed diagrams created using the Eukleides language. https://numbas-editor.readthedocs.io/en/latest/extensions/first-party.html#eukleides|false|
|geogebra|Boolean|The GeoGebra extension provides functions to embed GeoGebra worksheets in a question. https://numbas-editor.readthedocs.io/en/latest/extensions/first-party.html#geogebra|false|
|graphs|Boolean|This extension provides some functions for working with and drawing graphs (networks of vertices joined by edges) in Numbas. https://numbas-editor.readthedocs.io/en/latest/extensions/first-party.html#graph-theory|false|
|jsx_graph|Boolean|The JSXGraph extension provides functions to create and manipulate interactive diagrams with the JSXGraph library. https://numbas-editor.readthedocs.io/en/latest/extensions/first-party.html#eukleides|false|
|linear_algebra|Boolean||false|
|linear_codes|Boolean|This extension provides a new data type and some functions to deal with linear codes. https://numbas-editor.readthedocs.io/en/latest/extensions/first-party.html#linear-codes|false|
|optimisation|Boolean||false|
|permutations|Boolean||false|
|polynomials|Boolean|This extension provides a new data type and some functions to deal with rational polynomials. https://numbas-editor.readthedocs.io/en/latest/extensions/first-party.html#polynomials|false|
|quantities|Boolean|This extension wraps the js-quantities library to provide a “quantity with units” data type to Numbas. https://numbas-editor.readthedocs.io/en/latest/extensions/first-party.html#quantities|false|
|random_person|Boolean|The “random person” extension provides a collection of functions to generate random people, for use in word problems. https://numbas-editor.readthedocs.io/en/latest/extensions/first-party.html#random-person|false|
|stats|Boolean|The statistical functions extension provides many new functions for generating samples from random distributions, and calculating statistics. https://numbas-editor.readthedocs.io/en/latest/extensions/first-party.html#statistical-functions|false|
|sqlite|Boolean||false|
|text|Boolean||false|
|written_number|Boolean||false|

# ResourcePath
|field|type|description|optional|
|--|--|----|-|
|resource_name|String||false|
|resource_path|Filesystem path||false|

# CustomPartTypeDefinitionPath
|field|type|description|optional|
|--|--|----|-|
|file_name|String||false|
|data|[CustomPartTypeDefinition](#CustomPartTypeDefinition)||false|

# CustomPartTypeDefinition
|field|type|description|optional|
|--|--|----|-|
|type_name|[Translation](#Translation)||false|
|description|[Translation](#Translation)||false|
|settings|Array of [CustomPartTypeSetting](#CustomPartTypeSetting)||false|
|can_be_gap|Boolean||false|
|can_be_step|Boolean||false|
|marking_notes|Array of [JMENote](#JMENote)||false|
|help_url|[Translation](#Translation)||false|
|published|Boolean||false|
|extensions|[Extensions](#Extensions)||false|
|input_widget|[CustomPartInputWidget](#CustomPartInputWidget)||false|

# CustomPartTypeSetting
Internal tag named input_type.
One of the following items:
|tag-value|datatype of value|description|
|--|--|----|
|check_box|[CustomPartTypeSettingCheckBox](#CustomPartTypeSettingCheckBox)||
|code|[CustomPartTypeSettingCode](#CustomPartTypeSettingCode)||
|mathematical_expression|[CustomPartTypeSettingMathematicalExpression](#CustomPartTypeSettingMathematicalExpression)||
|string|[CustomPartTypeSettingString](#CustomPartTypeSettingString)||
|drop_down|[CustomPartTypeSettingDropDown](#CustomPartTypeSettingDropDown)||
|percentage|[CustomPartTypeSettingPercentage](#CustomPartTypeSettingPercentage)||

# CustomPartTypeSettingCheckBox
|field|type|description|optional|
|--|--|----|-|
|name|[Translation](#Translation)|A short name for this setting, used to refer to it in the part type’s answer input or marking algorithm. The name should uniquely identify the setting.|false|
|numbas_label|[Translation](#Translation)|The label shown next to the setting in the numbas question editor. Try to make it as clear as possible what the setting is for. For example, a checkbox which dictates whether an input hint is shown should be labelled something like “Hide the input hint?” rather than “Input hint visibility” - the latter doesn’t tell the question author whether ticking the checkbox will result in the input hint appearing or not.|false|
|documentation_url|"none" or [Translation](#Translation)|The address of documentation explaining this setting in further depth.|false|
|numbas_hint|[Translation](#Translation)|Use this field to give further guidance to question authors about this setting, if the label is not enough. For example, you might use this to say what data type a JME code setting should evaluate to.|false|
|default_value|Boolean|The initial value of the setting in the question editor.|false|

# CustomPartTypeSettingCode
|field|type|description|optional|
|--|--|----|-|
|name|[Translation](#Translation)|A short name for this setting, used to refer to it in the part type’s answer input or marking algorithm. The name should uniquely identify the setting.|false|
|numbas_label|[Translation](#Translation)|The label shown next to the setting in the numbas question editor. Try to make it as clear as possible what the setting is for. For example, a checkbox which dictates whether an input hint is shown should be labelled something like “Hide the input hint?” rather than “Input hint visibility” - the latter doesn’t tell the question author whether ticking the checkbox will result in the input hint appearing or not.|false|
|documentation_url|"none" or [Translation](#Translation)|The address of documentation explaining this setting in further depth.|false|
|numbas_hint|[Translation](#Translation)|Use this field to give further guidance to question authors about this setting, if the label is not enough. For example, you might use this to say what data type a JME code setting should evaluate to.|false|
|default_value|"none" or [Translation](#Translation)|The initial value of the setting in the question editor. If the setting has a sensible default value, set it here. If the value of the setting is likely to be different for each instance of this part type, set this to none.|false|
|evaluate|Boolean||false|

# CustomPartTypeSettingMathematicalExpression
|field|type|description|optional|
|--|--|----|-|
|name|[Translation](#Translation)|A short name for this setting, used to refer to it in the part type’s answer input or marking algorithm. The name should uniquely identify the setting.|false|
|numbas_label|[Translation](#Translation)|The label shown next to the setting in the numbas question editor. Try to make it as clear as possible what the setting is for. For example, a checkbox which dictates whether an input hint is shown should be labelled something like “Hide the input hint?” rather than “Input hint visibility” - the latter doesn’t tell the question author whether ticking the checkbox will result in the input hint appearing or not.|false|
|documentation_url|"none" or [Translation](#Translation)|The address of documentation explaining this setting in further depth.|false|
|numbas_hint|[Translation](#Translation)|Use this field to give further guidance to question authors about this setting, if the label is not enough. For example, you might use this to say what data type a JME code setting should evaluate to.|false|
|evaluate_enclosed_expressions|Boolean|If this is ticked, then JME expressions enclosed in curly braces will be evaluated and the results substituted back into the string.|false|
|default_value|"none" or [Translation](#Translation)|The initial value of the setting in the question editor. If the setting has a sensible default value, set it here. If the value of the setting is likely to be different for each instance of this part type, set this to none.|false|

# CustomPartTypeSettingString
|field|type|description|optional|
|--|--|----|-|
|name|[Translation](#Translation)|A short name for this setting, used to refer to it in the part type’s answer input or marking algorithm. The name should uniquely identify the setting.|false|
|numbas_label|[Translation](#Translation)|The label shown next to the setting in the numbas question editor. Try to make it as clear as possible what the setting is for. For example, a checkbox which dictates whether an input hint is shown should be labelled something like “Hide the input hint?” rather than “Input hint visibility” - the latter doesn’t tell the question author whether ticking the checkbox will result in the input hint appearing or not.|false|
|documentation_url|"none" or [Translation](#Translation)|The address of documentation explaining this setting in further depth.|false|
|numbas_hint|[Translation](#Translation)|Use this field to give further guidance to question authors about this setting, if the label is not enough. For example, you might use this to say what data type a JME code setting should evaluate to.|false|
|evaluate_enclosed_expressions|Boolean|If this is ticked, then JME expressions enclosed in curly braces will be evaluated and the results substituted back into the text when the question is run. Otherwise, the string will be untouched.|false|
|default_value|"none" or String|The initial value of the setting in the question editor. If the setting has a sensible default value, set it here. If the value of the setting is likely to be different for each instance of this part type, set this to none.|false|

# CustomPartTypeSettingDropDown
|field|type|description|optional|
|--|--|----|-|
|name|[Translation](#Translation)|A short name for this setting, used to refer to it in the part type’s answer input or marking algorithm. The name should uniquely identify the setting.|false|
|numbas_label|[Translation](#Translation)|The label shown next to the setting in the numbas question editor. Try to make it as clear as possible what the setting is for. For example, a checkbox which dictates whether an input hint is shown should be labelled something like “Hide the input hint?” rather than “Input hint visibility” - the latter doesn’t tell the question author whether ticking the checkbox will result in the input hint appearing or not.|false|
|documentation_url|"none" or [Translation](#Translation)|The address of documentation explaining this setting in further depth.|false|
|numbas_hint|[Translation](#Translation)|Use this field to give further guidance to question authors about this setting, if the label is not enough. For example, you might use this to say what data type a JME code setting should evaluate to.|false|
|default_value|"none" or [Translation](#Translation)|The initial value of the setting in the question editor. If the setting has a sensible default value, set it here. If the value of the setting is likely to be different for each instance of this part type, set this to none.|false|
|choices|Array of [CustomPartTypeSettingDropDownChoice](#CustomPartTypeSettingDropDownChoice)||false|

# CustomPartTypeSettingDropDownChoice
|field|type|description|optional|
|--|--|----|-|
|value|[Translation](#Translation)||false|
|label|[Translation](#Translation)||false|

# CustomPartTypeSettingPercentage
|field|type|description|optional|
|--|--|----|-|
|name|[Translation](#Translation)|A short name for this setting, used to refer to it in the part type’s answer input or marking algorithm. The name should uniquely identify the setting.|false|
|numbas_label|[Translation](#Translation)|The label shown next to the setting in the numbas question editor. Try to make it as clear as possible what the setting is for. For example, a checkbox which dictates whether an input hint is shown should be labelled something like “Hide the input hint?” rather than “Input hint visibility” - the latter doesn’t tell the question author whether ticking the checkbox will result in the input hint appearing or not.|false|
|documentation_url|"none" or [Translation](#Translation)|The address of documentation explaining this setting in further depth.|false|
|numbas_hint|[Translation](#Translation)|Use this field to give further guidance to question authors about this setting, if the label is not enough. For example, you might use this to say what data type a JME code setting should evaluate to.|false|
|default_value|"none" or Float|The initial value of the setting in the question editor. If the setting has a sensible default value, set it here. If the value of the setting is likely to be different for each instance of this part type, set this to none.|false|

# CustomPartInputWidget
Internal tag named type.
One of the following items:
|tag-value|datatype of value|description|
|--|--|----|
|string|[CustomPartStringInputOptions](#CustomPartStringInputOptions)|The student enters a single line of text.|
|number|[CustomPartNumberInputOptions](#CustomPartNumberInputOptions)|The student enters a number, using any of the allowed notation styles. If the student’s answer is not a valid number, they are shown a warning and can not submit the part.|
|radiogroup|[CustomPartRadioGroupInputOptions](#CustomPartRadioGroupInputOptions)|The student chooses one from a list of choices by selecting a radio button.|

# CustomPartStringInputOptions
|field|type|description|optional|
|--|--|----|-|
|hint|[CustomPartInputOptionValueTranslatableString](#CustomPartInputOptionValueTranslatableString)|A string displayed next to the input field, giving any necessary information about how to enter their answer.|false|
|correct_answer|[Translation](#Translation)|A JME expression which evaluates to the expected answer to the part.|false|
|allow_empty|[CustomPartInputOptionValueBool](#CustomPartInputOptionValueBool)|If false, the part will only be marked if their answer is non-empty.|false|

# CustomPartInputOptionValueTranslatableString
|field|type|description|optional|
|--|--|----|-|
|value|[Translation](#Translation)|The value|false|
|static|Boolean|A static field takes the same value in every instance of the part type. A dynamic field is defined by a JME expression which is evaluated when the question is run.|false|

# CustomPartInputOptionValueBool
|field|type|description|optional|
|--|--|----|-|
|value|Boolean|The value|false|
|static|Boolean|A static field takes the same value in every instance of the part type. A dynamic field is defined by a JME expression which is evaluated when the question is run.|false|

# CustomPartNumberInputOptions
|field|type|description|optional|
|--|--|----|-|
|hint|[CustomPartInputOptionValueTranslatableString](#CustomPartInputOptionValueTranslatableString)|A string displayed next to the input field, giving any necessary information about how to enter their answer.|false|
|correct_answer|[Translation](#Translation)|A JME expression which evaluates to the expected answer to the part.|false|
|allow_fractions|[CustomPartInputOptionValueBool](#CustomPartInputOptionValueBool)|Allow the student to enter their answer as a fraction?|false|
|allowed_notation_styles|[CustomPartInputOptionValueAnswerStyles](#CustomPartInputOptionValueAnswerStyles)||false|

# CustomPartInputOptionValueAnswerStyles
|field|type|description|optional|
|--|--|----|-|
|value|Array of [AnswerStyle](#AnswerStyle)|The value|false|
|static|Boolean|A static field takes the same value in every instance of the part type. A dynamic field is defined by a JME expression which is evaluated when the question is run.|false|

# CustomPartRadioGroupInputOptions
|field|type|description|optional|
|--|--|----|-|
|hint|[CustomPartInputOptionValueTranslatableString](#CustomPartInputOptionValueTranslatableString)|A string displayed next to the input field, giving any necessary information about how to enter their answer.|false|
|correct_answer|[Translation](#Translation)|A JME expression which evaluates to the expected answer to the part.|false|
|choices|[CustomPartInputOptionValueTranslatableStrings](#CustomPartInputOptionValueTranslatableStrings)|The labels for the choices to offer to the student.|false|

# CustomPartInputOptionValueTranslatableStrings
|field|type|description|optional|
|--|--|----|-|
|value|Array of [Translation](#Translation)|The value|false|
|static|Boolean|A static field takes the same value in every instance of the part type. A dynamic field is defined by a JME expression which is evaluated when the question is run.|false|


