# dicelang: dice expression parser and evaluator in Rust

Rust crate capable of parsing and evaluating dice expressions like
`d20+1d4-1d6+3`. The main crate (here) provides parsing and evaluation
facilities. There are also sub-crates for CLI and WASM support. See their
READMEs for more more information.

Evaluation returns the overall result as well as each individual roll. This can
be helpful when looking for critical successes or failures.
