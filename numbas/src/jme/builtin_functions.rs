use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum BuiltinFunctions {
    /// Synonym of e^x
    Exp,

    /// Construct a decimal value. Any string accepted by Decimal.js is accepted.
    #[serde(alias = "dec")]
    Decimal,
    /// Convert n to a rational nubmer, taking an approximation when necessary.
    Rational,
    /// Convert n to an integer, rounding to the nearest integer.
    Int,
    #[serde(alias = "len")]
    #[serde(alias = "length")]
    /// Absolute value, or modulus.
    Abs, // TODO: maybe split these?
    /// Argument of a complex number.
    Arg,
    /// Real part of a complex number.
    Re,
    /// Imaginary part of a complex number.
    Im,
    /// Complex conjugate.
    Conj,
    /// Returns true if x is an integer - that is, it is real and has no fractional part.
    IsInt,
    /// Returns true when n is exactly 0.
    IsZero,
    #[serde(alias = "sqr")]
    /// Square root of a number.
    Sqrt,
    /// nth root of x
    Root,
    /// Natural Logarithm
    Ln,
    /// Logarithm with base b, or base 10 if b is not given.
    Log,
    /// Convert radians to degrees.
    Degrees,
    /// Convert degrees to radians.
    Radians,
    /// Sign of a number, 1 if positive, -1 if negative and 0 when 0.
    #[serde(alias = "sgn")]
    Sign,
    /// Greatest of the given numbers.
    Max,
    /// Least of the given numbers.
    Min,
    /// Return the point nearest to x in the interval [a,b].
    /// Equivalent to max(a,min(x,b)).
    Clamp,
    /// Round n to d decimal places. On matrices and vectors, this rounds each element independently.
    PrecRound,
    /// Round n to f significant figures. On matrices and vectors, this rounds each element independently.
    SigRound,
    /// Returns true if b−t<=a<=b+t
    WithIntolerance,
    /// Round n to d decimal places and return a string, padding with zeros if necessary.
    /// If style is given, the number is rendered using the given notation style. See the page on Number notation for more on notation styles.
    DPFormat,
    /// Assuming n is a string representing a number, return the number of decimal places used. The string is passed through cleannumber() first.
    CountDP,
    /// Round n to d significant figures and return a string, padding with zeros if necessary.
    SigFormat,
    /// Assuming n is a string representing a number, return the number of significant figures. The string is passed through cleannumber() first.
    CountSigFigs,
    /// Returns true if str is a string representing a number given to the desired number of decimal places or significant figures.
    /// precisionType is either "dp", for decimal places, or "sigfig", for significant figures.
    /// If strict is true, then trailing zeroes must be included.
    ToGivenPrecision,
    /// Round a to the nearest multiple of b.
    ToNearest,
    /// Render the number n using the given number notation style.
    /// See the page on Number notation for more on notation styles.
    FormatNumber,
    /// Return a LaTeX string representing the given number in scientific notation, a×10^b.
    /// This function exists because scientific notation may use superscripts, which aren’t easily typeset in plain text.
    ScientificNumberLaTeX,
    /// Return an HTML element representing the given number in scientific notation, a×10^b.
    /// This function exists because scientific notation may use superscripts, which aren’t easily typeset in plain text.
    ScientificNumberHTML,
    /// Clean a string potentially representing a number. Remove space, and then try to identify a notation style, and rewrite to the plain-en style.
    /// styles is a list of notation styles. If styles is given, str will be tested against the given styles. If it matches, the string will be rewritten using the matched integer and decimal parts, with punctuation removed and the decimal point changed to a dot.
    CleanNumber,
    /// Try to match a string representing a number in any of the given styles at the start of the given string, and return both the matched text and a corresponding number value.
    MatchNumber,
    /// Parse a string representing a number written in the given style.
    /// If a list of styles is given, the first that accepts the given string is used.
    /// See the page on Number notation for more on notation styles.
    ParseNumber,
    #[serde(rename = "parsenumber_or_fraction")]
    /// Works the same as parsenumber(), but also accepts strings of the form number/number, which it interprets as fractions.
    ParseNumberOrFraction,
    /// Parse a string representing a number written in the given style, and return a decimal value.
    /// If a list of styles is given, the first that accepts the given string is used.
    /// See the page on Number notation for more on notation styles.
    ParseDecimal,
    #[serde(rename = "parsedecimal_or_fraction")]
    /// Works the same as parsedecimal(), but also accepts strings of the form number/number, which it interprets as fractions.
    ParseDecimalOrFraction,
    /// Write the given number in binary: base 2.
    ToBinary,
    /// Write the given number in octal: base 8.
    ToOctal,
    /// Write the given number in hexadecimal: base 16.
    ToHexadecimal,
    /// Write the given number in the given base. base can be any integer between 2 and 36.
    ToBase,
    /// Convert a string representing a number written in binary (base 2) to a integer value.
    FromBinary,
    /// Convert a string representing a number written in octal (base 8) to a integer value.
    FromOctal,
    /// Convert a string representing a number written in hexadecimal (base 16) to a integer value.
    FromHexadecimal,
    /// Convert a string representing a number written in the given base to a integer value. base can be any integer between 2 and 36.
    FromBase,
    /// Is n the “not a number” value, NaN?
    IsNan,

    /// Sine
    Sin,
    /// Cosine
    Cos,
    /// Tangent: tan(x) = sin(x)/cos(x)
    Tan,
    /// Cosecant: = 1 / sin(x)
    Cosec,
    /// Secant: = 1 / cos(x)
    Sec,
    /// Cotangent: = 1 / tan(x)
    Cot,
    /// Inverse of sin
    ArcSin,
    /// Inverse of cos
    ArcCos,
    /// Inverse of tan
    ArcTan,
    /// The angle in radians between the positive x-axis and the line through the origin and (x,y). This is often equivalent to arctan(y/x), except when x<0, when pi is either added or subtracted from the result.
    ATan2,
    /// Hyperbolic sine
    Sinh,
    /// Hyperbolic cosine
    Cosh,
    /// Hyperbolic tangent
    Tanh,
    /// Hyperbolic cosecant
    Cosech,
    /// Hyperbolic secant
    Sech,
    /// Hyperbolic cotangent
    Coth,
    /// Inverse of sinh
    ArcSinh,
    /// Inverse of cosh
    ArcCosh,
    /// Inverse of tanh
    ArcTanh,

    /// fact(x) is a synonym for x!.
    Fact,
    /// Factorise n. Returns the exponents of the prime factorisation of n as a list.
    Factorise,
    /// Returns the divisors of n as a list: positive integers d such that d || n.
    Divisors,
    #[serde(rename = "proper_divisors")]
    /// Returns the proper divisors of n as a list: positive integers d < n such that d || n.
    /// That is, the divisors of n, excluding n itself.
    ProperDivisors,
    /// Gamma function
    Gamma,
    /// Round up to the nearest integer. When x is complex, each component is rounded separately.
    Ceil,
    /// Round down to the nearest integer. When x is complex, each component is rounded separately.
    Floor,
    /// Round to the nearest integer. 0.5 is rounded up.
    Round,
    /// If x is positive, round down to the nearest integer; if it is negative, round up to the nearest integer.
    Trunc,
    /// Fractional part of a number. Equivalent to x-trunc(x).
    Fract,
    #[serde(rename = "rational_approximation")]
    /// Compute a rational approximation to the given number by computing terms of its continued fraction, returning the numerator and denominator separately. The approximation will be within e−accuracy of the true value; the default value for accuracy is 15.
    RationalApproximation,
    /// Modulo; remainder after integral division, i.e. a mod b.
    Mod,
    /// Count permutations
    Perm,
    /// Count combinations
    Comb,
    #[serde(alias = "gcf")]
    /// Greatest common divisor of integers a and b
    GCD,
    #[serde(rename = "gcd_without_pi_or_i")]
    /// Take out factors of π or i from a and b before computing their greatest common denominator.
    GCDWithoutPIorI,
    /// Are a and b coprime? True if their gcd() is 1, or if either of a or b is not an integer.
    CoPrime,
    /// Lowest common multiple of integers a and b. Can be used with any number of arguments; it returns the lowest common multiple of all the arguments.
    LCM,

    /// Create a vector with given components. Alternately, you can create a vector from a single list of numbers.
    Vector,
    /// Create a matrix with given rows, which should be either vectors or lists of numbers. Or, you can pass in a single list of lists of numbers.
    Matrix,
    /// Identity matrix with n rows and columns.
    Id,
    /// The number of rows in the given matrix
    NumRows,
    /// The number of columns in the given matrix
    NumColumns,
    /// Create a row vector (1×n matrix) with the given components. Alternately, you can create a row vector from a single list of numbers.
    RowVector,
    /// Dot (scalar) product. Inputs can be vectors or column matrices.
    Dot,
    /// Cross product. Inputs can be vectors or column matrices.
    Cross,
    /// Angle between vectors a and b, in radians. Returns 0 if either a or b has length 0.
    Angle,
    #[serde(rename = "is_zero")]
    /// Returns true if every component of the vector x is zero.
    IsZeroVector,
    #[serde(rename = "is_scalar_multiple")]
    /// Returns true if u is a scalar multiple of v. That is, if u = k*v for some real number k.
    /// The optional arguments rel_tol and abs_tol specify the relative and absolute tolerance of
    /// the equality check for each component; see isclose.
    IsScalarMultiple,
    /// Determinant of a matrix. Throws an error if used on anything larger than a 3×3 matrix.
    Det, // Why not for larger matrices?
    /// Matrix transpose.
    Transpose,
    /// Calculate the sum of all the cells in a matrix.
    #[serde(rename = "sum_cells")]
    SumCells,
    #[serde(alias = "combine_horizontally")]
    /// Combine two matrices horizontally: given r1 x c1 a matrix m1 and a r2 x c2 matrix m2, returns a new max(1, r2) x (c1 + c2) matrix formed by putting the two matrices side, and padding with zeros where necessary.
    Augment,
    #[serde(alias = "combine_vertically")]
    /// Combine two matrices vertically: given a r1 x c1 matrix m1 and a r2 x c2 matrix m2, returns a new (r1 + r2) x max(c1, c2) matrix formed by putting m1 above m2, and padding with zeros where necessary.
    Stack,
    #[serde(rename = "combine_diagonally")]
    /// Combine two matrices diagonally: given a r1 x c1 matrix m1 and a r2 x c2 matrix m2, returns a new (r1 + r2) x (c1 + c2) matrix whose top-left quadrant is m1 and bottom-right quadrant is m2.
    CombineDiagonally,

    /// Convert x to a string.
    /// When converting a expression value to a string, you can give a list of display options as a second argument, either as a comma-separated string or a list of strings.
    String,
    /// Mark string x as containing raw LaTeX, so when it’s included in a mathmode environment it doesn’t get wrapped in a \textrm environment.
    /// If x is a expression value, it’s rendered to LaTeX.
    /// Note that backslashes must be double up, because the backslash is an escape character in JME strings.
    LaTeX,
    /// Mark string x as safe: don’t substitute variable values into it when this expression is evaluated.
    /// Use this function to preserve curly braces in string literals.
    Safe,
    /// Substitute variable values into the string x, even if it’s marked as safe (see safe()).
    /// The optional dictionary values overrides any previously-defined values of variables.
    /// Note: The variable dependency checker can’t establish which variables will be used in the string until render is evaluated, so you may encounter errors if using render in the definition of a question variable. You can ensure a variable has been evaluated by including it in the values argument, e.g.:
    /// render("a is {}",["a": a])
    /// This function is intended for use primarily in content areas.
    Render,
    /// Capitalise the first letter of a string.
    Capitalise,
    /// Return singular if n is 1, otherwise return plural.
    Pluralise,
    /// Convert string to upper-case.
    Upper,
    /// Convert string to lower-case.
    Lower,
    /// Join a list of strings with the given delimiter.
    Join,
    /// Split a string at every occurrence of delimiter, returning a list of the resulting pieces.
    Split,
    #[serde(rename = "match_regex")]
    /// If str matches the regular expression pattern, returns a list of matched groups, otherwise returns an empty list.
    /// This function uses JavaScript regular expression syntax.
    /// flags is an optional string listing the options flags to use. If it’s not given, the default value of "u" is used.
    MatchRegex,
    #[serde(rename = "split_regex")]
    /// Split a string at every occurrence of a substring matching the given regular expression pattern, returning a list of the the remaining pieces.
    /// flags is an optional string listing the options flags to use. If it’s not given, the default value of "u" is used.
    SplitRegex,
    #[serde(rename = "replace_regex")]
    /// Replace a substring of string matching the given regular expression pattern with the string replacement.
    /// flags is an optional string listing the options flags to use. If it’s not given, the default value of "u" is used.
    /// Remember that backslashes must be doubled up inside JME strings, and curly braces are normally used to substitute in variables. You can use the safe() function to avoid this behaviour.
    /// To replace all occurrences of the pattern, add the flag "g".
    ReplaceRegex,
    /// Remove whitespace from the start and end of str.
    Trim,
    /// Write a currency amount, with the given prefix or suffix characters.
    Currency,
    #[serde(rename = "separateThousands")]
    /// Write a number, with the given separator character between every 3 digits
    /// To write a number using notation appropriate to a particular culture or context, see formatnumber().
    SeparateThousands,
    /// Get rid of the % on the end of a percentage and parse as a number, then divide by 100.
    UnPercent,
    /// Add copies of prefix to the start of str until the result is at least n characters long.
    LPad,
    /// Add copies of suffix to the end of str until the result is at least n characters long.
    RPad,
    /// For each occurrence of %s in str, replace it with the corresponding entry in the list values.
    FormatString,
    /// Get the n^th element of the sequence a, b, c, ..., aa, ab, ....
    /// Note that the numbering starts from 0.
    LetterOrdinal,
    /// Translate the given string, if it’s in the localisation file.
    /// Look at the default localisation file for strings which can be translated. This function takes a key representing a string to be translated, and returns the corresponding value from the current localisation file.
    /// arguments is a dictionary of named substitutions to make in the string.
    Translate,
    /// After converting to lower case, is str any of the strings "true", "false", "yes" or "no"?
    IsBool,

    /// Returns true if x is close to y.
    /// The arguments rel_tol and abs_tol are optional, with default values of 10−15.
    /// Equivalent to the following expression:
    IsClose,
    /// Returns true if a and b are both of the same data type, and “close enough” according to the given checking function.
    /// Vectors, matrices, and lists are considered equal only if every pair of corresponding elements in a and b is “close enough”.
    /// checkingFunction is the name of a checking function to use. These are documented in the Numbas runtime documentation.
    ResultsEqual,

    /// Evaluate expression n times, and return the results in a list.
    Repeat,
    /// Returns true if every element of list is true.
    All,
    /// Returns true if at least one element of list is true.
    Some,
    /// Evaluate expression for each item in list, range, vector or matrix d, replacing variable name with the element from d each time.
    /// You can also give a list of names if each element of d is a list of values. The Nth element of the list will be mapped to the Nth name.
    Map,
    /// Filter each item in list or range d, replacing variable name with the element from d each time, returning only the elements for which expression evaluates to true.
    Filter,
    /// Accumulate a value by iterating over a collection. This can be used as an abstraction of routines such as “sum of a list of numbers”, or “maximum value in a list”.
    /// Evaluate expression for each item in the list, range, vector or matrix d, accumulating a single value which is returned.
    /// At each iteration, the variable item_name is replaced with the corresponding value from d. The variable accumulator_name is replaced with first_value for the first iteration, and the result of expression from the previous iteration subsequently.
    FoldL,
    /// Iterate an expression on the given initial value the given number of times, returning a list containing the values produced at each step.
    /// You can also give a list of names. The Nth element of the value will be mapped to the Nth name.
    Iterate,
    /// Iterate an expression on the given initial value until the condition is satisfied, returning a list containing the values produced at each step.
    /// You can also give a list of names. The Nth element of the value will be mapped to the Nth name.
    /// max_iterations is an optional parameter specifying the maximum number of iterations that may be performed. If not given, the default value of 100 is used. This parameter prevents the function from running indefinitely, when the condition is never met.
    #[serde(rename = "iterate_until")]
    IterateUntil,
    /// Take the first n elements from list or range d, replacing variable name with the element from d each time, returning only the elements for which expression evaluates to true.
    /// This operation is lazy - once n elements satisfying the expression have been found, execution stops. You can use this to filter a few elements from a large list, where the condition might take a long time to calculate.
    Take,
    /// “Flatten” a list of lists, returning a single list containing the concatenation of all the entries in lists.
    Flatten,
    /// Evaluate expression, temporarily defining variables with the given names. Use this to cut down on repetition. You can define any number of variables - in the first calling pattern, follow a variable name with its definition. Or you can give a dictionary mapping variable names to their values. The last argument is the expression to be evaluated.
    Let,
    /// Sort a list
    Sort,
    /// Return a list giving the index that each entry in the list will occupy after sorting.
    #[serde(rename = "sort_destinations")]
    SortDestinations,
    /// Sort the given list of either list or dict values by their entries corresponding to the given key. When sorting a list of lists, the key is a number representing the index of each list to look at. When sorting a list of dictionaries, the key is a string.
    #[serde(rename = "sort_by")]
    SortBy,
    /// Group the entries in the given list of either list or dict values by their entries corresponding to the given key. The returned value is a list of lists of the form [key, group], where key is the value all elements of the list group have in common.
    /// When grouping a list of lists, the key argument is a number representing the index of each list to look at. When grouping a list of dictionaries, the key argument is a string.
    #[serde(rename = "group_by")]
    GroupBy,
    /// Reverse list
    Reverse,
    /// Find the indices at which value occurs in list.
    Indices,
    /// Return a copy of the list x with duplicates removed.
    Distinct,
    /// Convert a value to a list of its components (or rows, for a matrix).
    List,
    /// Evaluate a dictionary of variable definitions and return a dictionary containing the generated values.
    /// definitions is a dictionary mapping variable names to expression values corresponding to definitions.
    /// The definitions can refer to other variables to be evaluated, or variables already defined in the current scope. Variables named in the dictionary which have already been defined will be removed before evaluation begins.
    #[serde(rename = "make_variables")]
    MakeVariables,
    /// Each variable name in names should have a corresponding definition expression in
    /// definitions. conditions is a list of expressions which you want to evaluate to true.
    /// The definitions will be evaluated repeatedly until all the conditions are satisfied, or
    /// the number of attempts is greater than maxRuns. If maxRuns isn’t given, it defaults to
    /// 100 attempts.
    Satisfy,
    /// Add up a list of numbers
    Sum,
    /// Multiply a list of numbers together
    Prod,
    /// Cartesian product of lists. In other words, every possible combination of choices of one value from each given list.
    /// If one list and a number are given, then the n-th Cartesian power of the list is returned: the Cartesian product of n copies of the list.
    Product,
    /// Combine two (or more) lists into one - the Nth element of the output is a list containing the Nth elements of each of the input lists.
    Zip,
    /// All ordered choices of r elements from collection, without replacement.
    Combinations,
    /// All ordered choices of r elements from collection, with replacement.
    #[serde(rename = "combinations_with_replacement")]
    CombinationsWithReplacement,
    /// All choices of r elements from collection, in any order, without replacement.
    Permutations,
    /// Count the number of times each distinct element of collection appears.
    /// Returns a list of pairs [value, frequency], where value is a value from the list, and frequency is the number of times it appeared.
    Frequencies,
    /// Enumerate the elements of collection: this function returns a list containing, for each element v of collection, a new list of the form [i,v], where i is the index of the element in collection.
    Enumerate,

    /// Get the value corresponding to the given key string in the dictionary.
    /// If the key is not present in the dictionary, the default value will be returned.
    Get,
    /// Create a dictionary with the given key-value pairs. Equivalent to [ .. ], except when no key-value pairs are given: [] creates an empty list instead.
    /// You can alternately pass a list of pairs of the form [key, value], to transform a list into a dictionary.
    Dict,
    /// A list of all of the given dictionary’s keys.
    Keys,
    /// A list of the values corresponding to each of the given dictionary’s keys.
    /// If a list of keys is given, only the values corresponding to those keys are returned, in the same order.
    Values,
    /// A list of all of the [key,value] pairs in the given dictionary.
    Items,

    /// Create a set with the given elements. Either pass the elements as individual arguments, or as a list.
    Set,
    /// Union of sets a and b
    Union,
    /// Intersection of sets a and b, i.e. elements which are in both sets.
    Intersection,

    /// Pick uniformly at random from a range, list, or from the given arguments.
    Random,
    /// Pick random from a weighted list of items. Each element in the input list is a pair of the form [item, probability], where probability is a number value.
    /// Items with negative weight are ignored.
    #[serde(rename = "weighted_random")]
    WeightedRandom,
    /// Get a random shuffling of the integers [0…n−1]
    Deal,
    /// Reorder a list given a permutation. The i'th element of the output is the order[i]'th element of list.
    Reorder,
    /// Random shuffling of list or range.
    Shuffle,
    /// Shuffle several lists together - each list has the same permutation of its elements applied. The lists must all be the same length, otherwise an error is thrown.
    #[serde(rename = "shuffle_together")]
    ShuffleTogether,

    /// Return a if b is true, else return 0.
    Award,
    /// If p is true, return a, else return b. Only the returned value is evaluated.
    If,
    /// Select cases. Alternating boolean expressions with values to return, with the final argument representing the default case. Only the returned value is evaluated.
    Switch,
    /// If condition is false, then return value, otherwise don’t evaluate value and return false. This is intended for use in marking scripts, to apply marking feedback only if a condition is met.
    Assert,
    /// Try to evaluate expression. If it is successfully evaluated, return the result. Otherwise,
    /// evaluate except, with the error message available as name.
    Try,

    /// Parse string x as HTML.
    HTML,
    /// Does str represent a string of HTML containing text? Returns false for the empty string, or HTML elements with no text content.
    IsNonEmptyHTML,
    /// Create an HTML with cell contents defined by data, which should be a list of lists of data,
    /// and column headers defined by the list of strings headers.
    Table,
    /// Create an HTML img element loading the image from the given URL. Images uploaded through the resources tab are stored in the relative URL resources/images/<filename>.png, where <filename> is the name of the original file.
    Image,
    /// Apply a CSS max-width attribute to the given element. You can use this to ensure that an
    /// image is not displayed too wide. The given width is in pixels.
    #[serde(rename = "max_width")]
    MaxWidth,
    /// Apply a CSS max-height attribute to the given element. You can use this to ensure that an image is not displayed too long. The given height is in pixels.
    #[serde(rename = "max_height")]
    MaxHeight,

    /// Decode a JSON string into JME data types.
    /// JSON is decoded into numbers, strings, booleans, lists, or dictionaries. To produce other data types, such as matrices or vectors, you will have to post-process the resulting data.
    /// Warning: The JSON value null is silently converted to an empty string, because JME has no “null” data type. This may change in the future.
    #[serde(rename = "json_decode")]
    JsonDecode,
    /// Convert the given object to a JSON string.
    /// Numbers, strings, booleans, lists, and dictionaries are converted in a straightforward manner. Other data types may behave unexpectedly.
    #[serde(rename = "json_encode")]
    JsonEncode,

    /// Parse a string as a JME expression. The expression can be substituted into other expressions, such as the answer to a mathematical expression part, or the \simplify LaTeX command.
    /// Warning: Note that the argument to expression is evaluated using the same rules as any
    /// other JME expression, so for example expression("2" + "x") is equivalent to
    /// expression("2x"), not expression("2 + x"). A good way to construct a randomised
    /// sub-expression is using substitute().
    #[serde(alias = "expression")]
    Parse,
    /// Evaluate the given sub-expression.
    /// If values is given, it should be a dictionary mapping names of variables to their values.
    Eval,
    /// Returns the arguments of the top-level operation of expression, as a list of sub-expressions. If expression is a data type other than an operation or function, an empty list is returned.
    /// Binary operations only ever have two arguments. For example, 1+2+3 is parsed as (1+2)+3.
    Args,
    /// Returns the name of the data type of the top token in the expression, as a string.
    Type,
    /// Construct a name token with the given name.
    Name,
    /// Construct an operator with the given name.
    Op,
    /// Construct a function token with the given name.
    Function,
    /// Returns a sub-expression representing the application of the given operation to the list of arguments.
    Exec,
    /// Return a list of all unbound variables used in the given expression. Effectively, this is all the variables that need to be given values in order for this expression to be evaluated.
    /// Bound variables are those defined as part of operations which also assign values to those variables, such as map or let.
    FindVars,
    /// Substitute the given variable values into expression.
    /// variables is a dictionary mapping variable names to values.
    Substitute,
    /// Apply the given simplification rules to expression, until no rules apply.
    /// rules is a list of names of rules to apply, given either as a string containing a comma-separated list of names, or a list of strings.
    /// Unlike the \\simplify command in content areas, the basic rule is not turned on by default.
    /// See Substituting variables into displayed maths for a list of rules available.
    Simplify,
    /// Expand juxtapositions in variable and function names for implicit multiplication of terms or composition of functions. This is to do with strings of letters with no spaces or operator symbols between them.
    /// options is an optional dictionary of settings for the process. It can contain the following keys.
    ///     singleLetterVariables - Insist that all variable names consist of a single letter, interpreting longer strings of characters as implicit multiplication. Greek letters are considered to be one letter long.
    ///     noUnknownFunctions - When a name appears before a bracket, but it’s not the name of a defined function, interpret it as a multiplication instead. This does not apply function applications with more than one argument.
    /// implicitFunctionComposition - When several function names are juxtaposed together to form a string that is not the name of a defined function, or several function names are joined with the multiplication symbol *, interpret it as implicity composition of functions.
    /// If options is not given, all of these are turned on.
    /// Variable name annotations, subscripts and primes do not count towards the number of letters in a name.
    #[serde(rename = "expand_juxtapositions")]
    ExpandJuxtapositions,
    /// Compare expressions a and b using the “canonical” ordering. Returns -1 if a should go before b, 0 if they are considered “equal”, and 1 if a should go after b.
    /// Expressions are examined in the following order:
    ///     Names used: all variable names used in each expression are collected in a depth-first search and the resulting lists are compared lexicographically.
    ///     Data type: if a and b are of different data types, op and function go first, and then they are compared using the names of their data types.
    ///     Polynomials: terms of the form x^b or a*x^b, where a and b are numbers and x is a variable name, go before anything else.
    ///     Function name: if a and b are both function applications, they are compared using the names of the functions. If the functions are the same, the arguments are compared. Powers, or multiples of powers, go after anything else.
    ///     Number: if a and b are both numbers, the lowest number goes first. Complex numbers are compared by real part and then by imaginary part.
    ///     Elements of other data types are considered to be equal to any other value of the same data type.
    #[serde(rename = "canonical_compare")]
    CanonicalCompare,
    /// Compare expression a and b by substituting random values in for the free variables.
    /// Returns true if a and b have exactly the same free variables, and produce the same results when evaluated against the randomly chosen values.
    /// For more control over the evaluation, see resultsequal().
    #[serde(rename = "numerical_compare")]
    NumericalCompare,
    /// Set the case-sensitivity of the scope and then evaluate expression.
    /// If case_sensitive is not given, it defaults to true.
    /// Case-sensitivity affects variable and function names. The names x and X are considered equivalent when not in case-sensitive mode, but are considered to be different when in case-sensitive mode.
    #[serde(rename = "scope_case_sensitive")]
    ScopeCaseSensitive,

    /// Differentiate the given expression with respect to the given variable name
    Diff,

    /// If expr matches pattern, return a dictionary of the form ["match": boolean, "groups": dict], where "groups" is a dictionary mapping names of matches to sub-expressions.
    /// See the documentation on pattern-matching mathematical expressions.
    /// If you don’t need to use any parts of the matched expression, use matches() instead.
    Match,
    /// Return true if expr matches pattern.
    /// Use this if you’re not interested in capturing any parts of the matched expression.
    Matches,
    /// Replace occurrences of pattern in expr with the expression created by substituting the matched items into replacement.
    Replace,

    /// Attempt to infer the types of free variables in the given expression.
    /// There can be more than one valid assignment of types to the variables in an expression. For example, in the expression a+a, the variable a can be any type which has a defined addition operation.
    /// Returns the first possible assignment of types to variables, as a dictionary mapping variable names to the name of its type. If a variable name is missing from the dictionary, the algorithm can’t establish any constraint on it.
    #[serde(rename = "infer_variable_types")]
    InferVariableTypes,
    /// Attempt to infer the type of the value produced by the given expression, which may contain free variables.
    /// First, the types of any free variables are inferred. Then, definitions of an operations or functions in the function are chosen to match the types of their arguments.
    /// Returns the name of the expression’s output type as a string, or "?" if the type can’t be determined.
    #[serde(rename = "infer_type")]
    InferType,

    /// Returns a list containing the names of every variable defined in the current scope, as strings.
    DefinedVariables,
    /// Returns true if the variable with the given name has been defined in the current scope.
    IsSet,
    /// Temporarily remove the named variables, functions and rulesets from the scope, and evaluate the given expression.
    Unset,
}

impl BuiltinFunctions {
    pub fn get(s: &str) -> Option<Self> {
        serde_plain::from_str(s).ok()
    }
}
