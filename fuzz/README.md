# Fuzzing

<!--toc:start-->
- [Fuzzing](#fuzzing)
  - [Usage](#usage)
    - [Debugging a crash](#debugging-a-crash)
  - [A brief introduction to fuzzers](#a-brief-introduction-to-fuzzers)
  - [Each fuzzer harness in detail](#each-fuzzer-harness-in-detail)
    - [`fuzz_target_1`](#fuzztarget1)
  - [Acknowledgments](#acknowledgments)
<!--toc:end-->

[Fuzz testing](https://en.wikipedia.org/wiki/Fuzzing) is a software testing
technique used to find security and stability issues by providing pseudo-random
data as input to the software.

## Usage

To use the fuzzers provided in this directory, start by installing cargo-fuzz:

```bash
cargo install cargo-fuzz
```

Once you have installed the cargo-fuzz, you can then execute any fuzzer with:

```bash
cargo fuzz run name_of_fuzzer
```

### Debugging a crash

Once you've found a crash, you'll need to debug it. The easiest first step in
this process is to minimise the input such that the crash is still triggered
with a smaller input. `cargo-fuzz` supports this out of the box with:

```bash
cargo fuzz tmin name_of_fuzzer artifacts/name_of_fuzzer/crash-...
```

From here, you will need to analyse the input and potentially the behaviour of
the program. The debugging process from here is unfortunately less well-defined,
so you will need to apply some expertise here. Happy hunting!

## A brief introduction to fuzzers

Fuzzing, or fuzz testing, is the process of providing generated data to a
program under test. The most common variety of fuzzers are mutational fuzzers;
given a set of existing inputs (a "corpus"), it will attempt to slightly change
(or "mutate") these inputs into new inputs that cover parts of the code that
haven't yet been observed. Using this strategy, we can quite efficiently
generate test cases which cover significant portions of the program, both with
expected and unexpected data.
[This is really quite effective for finding bugs.](https://github.com/rust-fuzz/trophy-case)

The fuzzers here use [`cargo-fuzz`](https://github.com/rust-fuzz/cargo-fuzz), a
utility which allows Rust to integrate with
[libFuzzer](https://llvm.org/docs/LibFuzzer.html), the fuzzer library built into
LLVM. Each source file present in [`fuzz_targets`](fuzz_targets) is a harness,
which is, in effect, a unit test which can handle different inputs. When an
input is provided to a harness, the harness processes this data and libFuzzer
observes the code coverage and any special values used in comparisons over the
course of the run. Special values are preserved for future mutations and inputs
which cover new regions of code are added to the corpus.

## Each fuzzer harness in detail

Each fuzzer harness in [`fuzz_targets`](fuzz_targets) targets a different aspect
of falion and tests them in different ways. While there is
implementation-specific documentation in the source code itself, each harness is
briefly described below.

### `fuzz_target_1`

This fuzz harness... CHANGEME_MANUAL

## Acknowledgments

- [Original README this is based on](https://github.com/astral-sh/ruff/blob/main/fuzz/README.md)
