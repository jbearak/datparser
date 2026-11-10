# AGENTS.md

datparser is an R package whose one function, `read_fixed_width()`,
reads fixed-width text through its SAS setup file. The Rust reader is
in `src/rust/fixed_width`, its tests in `rust-tests/`, the R tests in
`tests/testthat/`. Decisions are in `docs/adr/`, plans in `docs/plans/`
(both copied from dta-parser; their numbers refer to that repository).

## Rules that override everything else

- Never run Stata. Do not install it or look for a copy.
- Never read Stata's installed files (ado files, help files, anything
  that ships with Stata), and do not use tools or data taken from them.
- Do not look up or use official Stata documentation, and never copy
  text from it. Take facts about Stata's formats and missing values
  only from general knowledge and from open-source projects'
  documentation and code; write in your own words.
- Never use output Stata produced as an expected result. Expected
  values come only from open-source libraries: asciiSetupReader and
  haven here, and the like.
- Make fixtures without Stata: write them with R or this package from
  data you create, as `compat/fixed_width/make_fixed_width.R` does. Do
  not use example datasets that ship with Stata.
- Copy open-source code only if its license allows, with credit.
- If you think you have found a bug in another project
  (asciiSetupReader, haven or any other), never file an issue, pull
  request or comment there without the user's approval of that
  specific report.
- Ask before publishing anything to CRAN, r-universe or any other
  registry.
- NSFG data and setup files are CDC's. Never commit them, or anything
  derived from them, to this repository. `scripts/check-nsfg.R` reads
  them from local directories.
