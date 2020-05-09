# relbl

A mass file-renaming tool that fits all renaming needs


## Renaming Files
Have a bunch of generated (or human-made) files that need to be systematically renamed? `relbl` is the tool you are looking for. With `relbl`, you can use the power of regular expressions to search for similarly-named files and rename them according to a given pattern. Want to add `-work` to the end of all of you `.txt` files? Simply use `relbl '(.*)\.txt' '${1}-work.txt'`. Maybe you want to switch all your pictures from *MM-DD-YYYY* to *YYYY-MM-DD*. `relbl 'IMG_(\d{2})-(\d{2})-(\d{4}).jpg' 'IMG_${3}-${1}-${2}.jpg'` should do the trick!


## Table of Contents

* [Using `relbl`](#using-relbl)
* [Install](#install-relbl)
* [Replacement Strings](#replacement-string)
* [Regex Syntax](#regex-syntax)
  * [Matching One Character](#matching-one-character)
  	* [Character Classes](#character-classes)
  * [Composites](#composites)
  * [Repetitions](#repetitions)
  * [Grouping and Flags](#grouping-and-flags)
  * [Escape Sequences](#escape-sequences)
  * [Perl Character Classes](#perl-character-classes-unicode-friendly)
  * [ASCII Character Classes](#ascii-character-classes)


## Using `relbl`

The input for `relbl` is:

```
relbl <query> <replace> [--target-dir <dir> | -t <dir>]
```

* `<query>`: The regex that will be used to match files; see below for valid regex
* `<replace>`: The pattern that will be used to rename the files
* `--target-dir <dir> | -t <dir>`: The directory that contains the files to be renamed; if no target directory is given, the current directory is used

## Install `relbl`

TODO #1


## Replacement String
The replacement string is a regular string, except for the following replacement rule.

All instances of `$name` in the replacement string is replaced with the corresponding capture group name.

`name` may be an integer corresponding to the index of the capture group (counted by order of opening parenthesis where `0` is the entire match) or it can be a name (consisting of letters, digits or underscores) corresponding to a named capture group.

If `name` isn't a valid capture group (whether the name doesn't exist or isn't a valid index), then it is replaced with the empty string.

The longest possible name is used. e.g., `$1a` looks up the capture group named `1a` and not the capture group at index `1`. To exert more precise control over the name, use braces, e.g., `${1}a`.

To write a literal `$` use `$$`.


## Regex Syntax

*The following is a modified version of the documentation from the [regex crate](https://docs.rs/regex/1.3.7/regex/index.html) that `relbl` uses.*

The syntax supported is as follows below.

### Matching one character

<pre class="rust">
.             any character except new line (includes new line with s flag)
\d            digit (\p{Nd})
\D            not digit
\pN           One-letter name Unicode character class
\p{Greek}     Unicode character class (general category or script)
\PN           Negated one-letter name Unicode character class
\P{Greek}     negated Unicode character class (general category or script)
</pre>

#### Character classes

<pre class="rust">
[xyz]         A character class matching either x, y or z (union).
[^xyz]        A character class matching any character except x, y and z.
[a-z]         A character class matching any character in range a-z.
[[:alpha:]]   ASCII character class ([A-Za-z])
[[:^alpha:]]  Negated ASCII character class ([^A-Za-z])
[x[^xyz]]     Nested/grouping character class (matching any character except y and z)
[a-y&&xyz]    Intersection (matching x or y)
[0-9&&[^4]]   Subtraction using intersection and negation (matching 0-9 except 4)
[0-9--4]      Direct subtraction (matching 0-9 except 4)
[a-g~~b-h]    Symmetric difference (matching `a` and `h` only)
[\[\]]        Escaping in character classes (matching [ or ])
</pre>

Any named character class may appear inside a bracketed `[...]` character
class. For example, `[\p{Greek}[:digit:]]` matches any Greek or ASCII
digit. `[\p{Greek}&&\pL]` matches Greek letters.

Precedence in character classes, from most binding to least:

1. Ranges: `a-cd` == `[a-c]d`
2. Union: `ab&&bc` == `[ab]&&[bc]`
3. Intersection: `^a-z&&b` == `^[a-z&&b]`
4. Negation

### Composites

<pre class="rust">
xy    concatenation (x followed by y)
x|y   alternation (x or y, prefer x)
</pre>

### Repetitions

<pre class="rust">
x*        zero or more of x (greedy)
x+        one or more of x (greedy)
x?        zero or one of x (greedy)
x*?       zero or more of x (ungreedy/lazy)
x+?       one or more of x (ungreedy/lazy)
x??       zero or one of x (ungreedy/lazy)
x{n,m}    at least n x and at most m x (greedy)
x{n,}     at least n x (greedy)
x{n}      exactly n x
x{n,m}?   at least n x and at most m x (ungreedy/lazy)
x{n,}?    at least n x (ungreedy/lazy)
x{n}?     exactly n x
</pre>

### Grouping and flags

<pre class="rust">
(exp)          numbered capture group (indexed by opening parenthesis)
(?P&lt;name&gt;exp)  named (also numbered) capture group (allowed chars: [_0-9a-zA-Z])
(?:exp)        non-capturing group
(?flags)       set flags within current group
(?flags:exp)   set flags for exp (non-capturing)
</pre>

Flags are each a single character. For example, `(?x)` sets the flag `x`
and `(?-x)` clears the flag `x`. Multiple flags can be set or cleared at
the same time: `(?xy)` sets both the `x` and `y` flags and `(?x-y)` sets
the `x` flag and clears the `y` flag.

All flags are by default disabled unless stated otherwise. They are:

<pre class="rust">
i     case-insensitive: letters match both upper and lower case
U     swap the meaning of x* and x*?
</pre>

Flags can be toggled within a pattern. Here's an example that matches
case-insensitively for the first part but case-sensitively for the second part:

```
(?i)a+(?-i)b+
```

Matches:
* `aabb`
* `aAabb`
* `Aab`
* `AAbb`

Doesn't match:
* `aaBb`
* `aabB`
* `aaBB`
* `aAbB`

Notice that the `a+` matches either `a` or `A`, but the `b+` only matches
`b`.

### Escape sequences

<pre class="rust">
\*          literal *, works for any punctuation character: \.+*?()|[]{}^$
\123        octal character code (up to three digits) (when enabled)
\x7F        hex character code (exactly two digits)
\x{10FFFF}  any hex character code corresponding to a Unicode code point
\u007F      hex character code (exactly four digits)
\u{7F}      any hex character code corresponding to a Unicode code point
\U0000007F  hex character code (exactly eight digits)
\U{7F}      any hex character code corresponding to a Unicode code point
</pre>

### Perl character classes (Unicode friendly)

These classes are based on the definitions provided in
[UTS#18](http://www.unicode.org/reports/tr18/#Compatibility_Properties):

<pre class="rust">
\d     digit (\p{Nd})
\D     not digit
\s     whitespace (\p{White_Space})
\S     not whitespace
\w     word character (\p{Alphabetic} + \p{M} + \d + \p{Pc} + \p{Join_Control})
\W     not word character
</pre>

### ASCII character classes

<pre class="rust">
[[:alnum:]]    alphanumeric ([0-9A-Za-z])
[[:alpha:]]    alphabetic ([A-Za-z])
[[:ascii:]]    ASCII ([\x00-\x7F])
[[:blank:]]    blank ([\t ])
[[:cntrl:]]    control ([\x00-\x1F\x7F])
[[:digit:]]    digits ([0-9])
[[:graph:]]    graphical ([!-~])
[[:lower:]]    lower case ([a-z])
[[:print:]]    printable ([ -~])
[[:punct:]]    punctuation ([!-/:-@\[-`{-~])
[[:upper:]]    upper case ([A-Z])
[[:word:]]     word characters ([0-9A-Za-z_])
[[:xdigit:]]   hex digit ([0-9A-Fa-f])
</pre>