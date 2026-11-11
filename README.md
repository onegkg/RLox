# RLox

An idiomatic Rust implementation of the tree-walking interpreter from [*Crafting Interpreters*](https://craftinginterpreters.com/).

RLox is currently a work in progress. Currently the lexer, a minimal AST, a minimal Parser, and a minimal interpreter have been implemented, but nothing more.

## Differences From the Book

While RLox mostly follows the Java implementation of Lox in the book, it has a couple of architectural differences to bring it more in line with Rust idioms. These differences and the rationale behind them are listed below.

In JLox the lexer holds mutable state which is modified in many different places, errors are also reported in this decentralized manner. In RLox, each function which would in JLox modify the error flag or tokens vector instead returns a Result<Token, Error> which gets bubbled up and handled by the main `scan_tokens` function. Tokens are also slightly restructured. The TokenType variants that represent literals now hold their literal value. This is necessary because Rust doesn't have an equivalent of Java's `Object` type, and honestly `Object` is far too broad for storing literally just a float or a String. The same pattern is used throughout the parser and interpreter.

In JLox, the AST is spread across many files and subclasses, and uses a visitor pattern to operate on the nodes. In RLox, the AST uses a single enum which implements a trait for each "visitor" in the original implementation.


## Testing

This repository contains a custom Haskell test runner which runs the interpreter against a slightly modified version of the official Crafting Interpreters test suite. The tests have been modified to support intentionally added additional functionality, and to match my slightly different error reporting style.

The Crafting Interpereters GitHub repository has a comprehensive test suite. It consists of around 250 `.lox` files annotated in comments with the expected output. The repository includes a test runner written in Dart that parses the expectations and runs a given interpreter against the file and compares it to the annotations.

Since I'm rewriting the interpreter in a new language, I wanted to make sure that at every step of the way I'm conforming to the spec, as going back and trying to fix issues later will get really annoying. As such, I knew I wanted to include the CI test suite in some form, but I also knew that I would want a way to exclude tests that I have not implemented the functionality to support yet, as a sea of tests failing due to unimplemented functionality would make it very hard for me to use the test runner to actually test the functionality I've implemented and catch regressions.

While I could have just implemented that feature in the Dart test runner, I chose to implement it from scratch in Haskell for a few reasons: 1. I don't know Dart. 2. A don't need some of the features that the test runner provides (eg. custom interpreter path). 3. I've been meaning to learn Haskell better and actually build something in it. 4. It seemed like a fun challenge.
