+++
title = "callisp - Callum's Lisp"
description = "A simple Lisp interpreter I made for fun."
+++

# callisp - Callum's Lisp

callisp is an interpreter for a Lisp I created. If you want to learn more about
Lisp programming languages, check out the [wikipedia page](https://en.wikipedia.org/wiki/Lisp_(programming_language)).
callisp supports floating point arithmetic (+, -, *, /) and strings with some
very basic IO.

The features that make the language usable are if statements, definitions, and
lambda functions. Here is a brief overview of how to use these:

- `(if cond do else)` where `cond`, `do`, and `else` are all expressions. If `cond`
evaluates to `true`, then `do` is evaluated. Otherwise, `else` is evaluated.
- `(def name expr)` evaluates `expr` and assigns the result to `name`.
- `(lambda (params) expr)` returns a function using `params` as the parameters
and `expr` as the body of the function.

For a full list of functions and instructions, check out the Github repo
[here](https://github.com/Callum-Irving/callisp).
