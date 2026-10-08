# RLox

An idiomatic Rust implementation of the tree-walking interpreter from *Crafting Interpreters*.

RLox is currently a work in progress. Currently the lexer and AST have been implemented, but nothing more.

## Differences From the Book

While RLox mostly follows the Java implementation of Lox in the book, it has a couple of architectural differences to bring it more in line with Rust idioms. These differences and the rationalle behind them are listed below.

Lexer: In JLox the lexer holds mutable state which is modified in many different places, errors are also reported in this decentralized manner. In RLox, each function which would in JLox modify the error flag or tokens vector instead returns a Result<Token, Error> which gets bubbled up and handled by the main `scan_tokens` function. Tokens are also slightly restructured. The TokenType variants that represent literals now hold their literal value. This is necessary because Rust doesn't have an equivalent of Java's `Object` type, and honestly `Object` is far too broad for storing literally just a float or a String.

AST: This is the area with the biggest change so far. In JLox, the AST is spread across many files and subclasses, and uses a visitor pattern to operate on the nodes. In RLox, the AST uses a single enum and uses a single `impl` block with a switch statement in order to achieve this same functionality.
<!-- TODO: Add comparison between our implementation and JLox -->


