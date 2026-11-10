# datparser

datparser is an R package with one function, `read_fixed_width()`. It
reads fixed-width text data through the SAS setup file (`.sas`) that
describes it, and returns a tibble. CDC publishes the National Survey
of Family Growth (NSFG) this way for every cycle from 1973 to
2017-2019; the reader was written for those files and checked against
all 21 female respondent and pregnancy files.

From the setup it takes:

- each variable's columns, from `INPUT`;
- variable labels, from `LABEL`;
- value labels, from `PROC FORMAT` and `FORMAT`;
- record-type selection, such as
  `INPUT rec_type 1 @; IF rec_type < 5 THEN DELETE; ELSE INPUT ...`;
- missing-value recodes, such as `IF (KIDS = 98 OR KIDS = 99) THEN KIDS = .;`.

The reading is done in Rust (`src/rust`), which uses no crates from
crates.io.

## Install

From GitHub, which builds the package from source:

```r
remotes::install_github("jbearak/datparser")
```

From the package repository on GitHub Pages, which has binaries for
macOS (arm64) and Windows (x86_64) and the source package:

```r
install.packages("datparser", repos = c("https://jbearak.github.io/datparser", getOption("repos")))
```

On Ubuntu 24.04 (x86_64), binaries are under a path for the R version:

```r
install.packages("datparser", repos = c(
  sprintf("https://jbearak.github.io/datparser/bin/linux/noble-x86_64/%s", getRversion()[, 1:2]),
  getOption("repos")
))
```

Building from source needs Rust's `cargo` and `rustc` on the `PATH`
(see <https://rustup.rs>). The build does not use the network. On
Windows, R builds with Rtools, so Rust needs the GNU target:
`rustup target add x86_64-pc-windows-gnu`.

datparser imports rlang, tibble, tidyselect, stats, and utils.

## Read a file

```r
library(datparser)

writeLines(c(
  "PROC FORMAT;",
  "  VALUE kidsf 0='(0) None' 98='(98) DK' 99='(99) Not ascertained';",
  "DATA;",
  "INPUT ID 1-3 KIDS 4-5;",
  "LABEL KIDS = 'Number of children';",
  "IF (KIDS = 98 OR KIDS = 99) THEN KIDS = .;",
  "FORMAT KIDS kidsf.;"
), "survey.sas")
writeLines(c("001 2", "00298", "00399", "004 0"), "survey.dat")

df <- read_fixed_width("survey.dat", "survey.sas")
df$KIDS                  # 2 NA NA 0
attr(df$KIDS, "label")   # "Number of children"
attr(df$KIDS, "labels")  # (0) None = 0
```

Columns are plain R vectors. A variable label is the attribute
`label`. A numeric column with value labels has the attribute
`labels`: the codes, named by their text, in code order. These
attributes survive `filter()` and `arrange()` but not `x[i]` or
`bind_rows()`. Pass a column and its labels to `haven::labelled()` if
you want haven's class.

Numeric columns are integers when every value is a whole number from
-2,147,483,647 to 2,147,483,620, and doubles otherwise. A column that
holds tagged NAs (below) in the rows read is a double, so a column's
type can depend on `skip` and `n_max`. String columns are character,
with `""` for a blank field. Records shorter than the setup's columns
read as blank, which is `NA` for a number.

## Missing-value recodes

`recodes` says what to do with the setup's recodes:

- `"setup"` (the default) applies them as written. A code recoded to
  `.` becomes `NA`, and its value label is dropped. A code recoded to
  `.A` to `.Z` becomes a tagged NA, as haven makes them.
- `"tagged"` keeps the reason each value is missing. For each variable,
  the codes recoded to `.` become the tagged NAs `.a`, `.b`, ... in the
  order the setup names them, and each code's value label moves to its
  letter. Letters the setup assigns itself are not reused.
- `"none"` ignores the recodes, so the codes stay numbers.

```r
df <- read_fixed_width("survey.dat", "survey.sas", recodes = "tagged")
haven::na_tag(df$KIDS)                   # NA "a" "b" NA
is.na(df$KIDS)                           # FALSE TRUE TRUE FALSE
attr(df$KIDS, "labels")                  # (0) None = 0, (98) DK = NA, (99) Not ascertained = NA
haven::na_tag(attr(df$KIDS, "labels"))   # NA "a" "b"

read_fixed_width("survey.dat", "survey.sas", recodes = "none")$KIDS  # 2 98 99 0
```

A letter means what that variable's labels say. Here `.a` is 98, "DK";
in a variable whose setup recodes 97 "Refused" first, `.a` is
"Refused". If a variable recodes more codes to `.` than it has letters
free, `recodes = "tagged"` stops with an error; use `"setup"` for it.

With `factors = TRUE`, columns with value labels become factors. A
factor cannot hold tagged NAs, so they become plain `NA`, with a
warning of class `datparser_extended_missing` unless
`warn_extended_missing = FALSE`.

## Setups in several files, and commented-out formats

CDC's 2002 setups come in three files (`2002FemPregInput.sas`,
`2002FemPregVarLabel.sas`, `2002FemPregValueLabel.sas`). Give `setup`
the paths in that order and they are read as one setup. A statement
may not run from one file into the next, and errors and warnings name a
file by its place (`"file 2 line 8"`).

```r
writeLines(c("DATA;", "INPUT ID 1-3 KIDS 4-5;", "RUN;"), "input.sas")
writeLines(c(
  "PROC FORMAT;",
  "  VALUE kidsf 0='(0) None' 98='(98) DK';",
  "RUN;",
  "FORMAT KIDS kidsf.;"
), "labels.sas")
df <- read_fixed_width("survey.dat", c("input.sas", "labels.sas"), recodes = "none")
attr(df$KIDS, "labels")  # (0) None = 0, (98) DK = 98
```

From 2006-2010 on, CDC puts the `PROC FORMAT` step and the `FORMAT`
statement each in a block comment (`/* ... */`), so that SAS users
without CDC's format catalog can run the file. The value labels are
still there, so by default a comment that holds nothing but a
`PROC FORMAT` step or a `FORMAT` statement is read as code. A comment
that only mentions them stays a comment. `commented_formats = FALSE`
reads every comment as a comment, and those files then have no value
labels:

```r
writeLines(c(
  "/*", "PROC FORMAT;", "  VALUE kidsf 0='(0) None' 98='(98) DK';", "*/",
  "DATA;", "INPUT ID 1-3 KIDS 4-5;",
  "/*", "FORMAT KIDS kidsf.;", "*/"
), "commented.sas")
attr(read_fixed_width("survey.dat", "commented.sas", recodes = "none")$KIDS, "labels")
# (0) None = 0, (98) DK = 98
attr(read_fixed_width("survey.dat", "commented.sas", recodes = "none",
                      commented_formats = FALSE)$KIDS, "labels")
# NULL
```

## Other options and warnings

`col_select` takes column names or a tidyselect expression. Only the
selected columns are read and warned about, in the order selected.
`skip` and `n_max` count the records the setup keeps.

Statements the reader does not run, such as other data-step code or a
range in `VALUE`, are skipped with one warning of class
`datparser_fixed_width_warning`; the tibble's `warnings` attribute
lists them all. A `LENGTH` statement for numbers, which only sets how
SAS stores them, is skipped without a warning. A variable label with
stray quotes inside it, such as
`X = "WHETHER " SAME AS LAST MONTH" WAS ANSWERED"` in CDC's 2011-2013
to 2015-2017 setups, is read from its first quote to its last, without
a warning.

Large files are read on up to three threads (on x86, half the
processor count, up to three). Set the environment variable
`DTA_THREADS` to change the most; `DTA_THREADS=1` uses one.

## Tests and the oracle

Expected values come from open-source readers only. The oracle for
fixed-width reads is asciiSetupReader (MIT), with haven for tagged NAs
(`docs/adr/0005-oracles.md`).

- `compat/fixed_width/make_fixed_width.R` writes a synthetic setup and
  data file in CDC's style from a fixed seed.
  `compat/fixed_width/expected_fixed_width.R` reads them with
  asciiSetupReader and haven and writes
  `compat/expected/fixed_width.json`. Run both from the repository root.
- `tests/testthat` checks the R package, including against that JSON
  file when run from a checkout.
- `rust-tests/` holds the Rust tests of `src/rust/fixed_width`:
  `cd rust-tests && cargo test`.
- `scripts/check-nsfg.R` compares `read_fixed_width()` with
  asciiSetupReader on all 21 NSFG female respondent and pregnancy files.
  It needs CDC's files, which are not in this repository; the script
  says where to get them.

Run the R tests against an installed package with
`testthat::test_dir("tests/testthat", package = "datparser", load_package = "installed")`.

`docs/adr/` and `docs/plans/` hold the design records for the reader,
copied from dta-parser.

## Credits

The reader began as part of
[dta-parser](https://github.com/jbearak/dta-parser) and was moved here
with its tests. asciiSetupReader, by Jacob Kaplan, is the oracle for
its tests. haven's tagged NAs are the layout for `.a` to `.z`.

## License

GPL-3. See `LICENSE`.
