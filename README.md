# RLox

An idiomatic Rust implementation of the tree-walking interpreter from [*Crafting Interpreters*](https://craftinginterpreters.com/).

RLox is currently a work in progress. I have so far implemented a lexer, parser, and interpreter capable of interpreting basic expressions.

I've also implemented a custom test runner to run my interpreter against a variation of the official Lox test suite.

## Differences From the Book

While RLox mostly follows the Java implementation of Lox in the book, it has a couple of architectural differences to bring it more in line with Rust idioms. These differences and the rationale behind them are listed below.

Error Handling: In JLox, errors are primarily modeled as exceptions. A function which encounters an error throws it as an exception, and it unwinds the call stack until it reaches a catch block. This is exactly the pattern you want in an interpreter, but it makes it difficult to reason about which functions can error and where the errors are caught when they're thrown. In RLox, I leverage Rust's robust `Option` and `Result` types in combination with the `?` operator to provide similar call stack unwinding functionality, while explicitly marking which parts of the code can error and making it much easier to trace an error's path through the call stack to its eventual handling.

AST: In JLox, the AST is spread across many files, repeating enough work that the author decided it would be worth writing an additional script to generate them. RLox forgoes this script entirely and instead models the AST as a couple of sum types, and forgoes the visitor classes used by JLox in favor of Rust traits.

## Testing

This repository contains a custom Haskell test runner which runs the interpreter against a slightly modified version of the official Crafting Interpreters test suite. The tests have been modified to support intentionally added additional functionality, and to match my slightly different error reporting style.

The Crafting Interpereters GitHub repository has a comprehensive test suite. It consists of around 250 `.lox` files annotated in comments with the expected output. The repository includes a test runner written in Dart that parses the expectations and runs a given interpreter against the file and compares it to the annotations.

Since I'm rewriting the interpreter in a new language, I wanted to make sure that at every step of the way I'm conforming to the spec, as going back and trying to fix issues later will get really annoying. As such, I knew I wanted to include the CI test suite in some form, but I also knew that I would want a way to exclude tests that I have not implemented the functionality to support yet, as a sea of tests failing due to unimplemented functionality would make it very hard for me to use the test runner to actually test the functionality I've implemented and catch regressions.

While I could have just implemented that feature in the Dart test runner, I chose to implement it from scratch in Haskell for a few reasons: 1. I don't know Dart. 2. A don't need some of the features that the test runner provides (eg. custom interpreter path). 3. I've been meaning to learn Haskell better and actually build something in it. 4. It seemed like a fun challenge.
