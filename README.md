# beepboop
beep boop bop ding ding whirr

## Overview
Beepboop is a minimalist, un-ergonomic (but Turing-complete!) programming language implemented in Rust, with all syntax elements made up of machine or robot noises. Code examples can be found in the `code` folder.

## Basics

Integer literals are created in a binary representation, where beep represents 1 and boop represents 0. E.g. 13 could be represented as `boop beep beep boop beep`. The leading zero marks it as a number to the parser.

Boolean literals cannot be directly constructed, because that would be too easy. They are only obtainable as results from logical comparisons, which will coerce integer inputs into booleans where appropriate.

Order of operations in expressions can be specified with parentheses, with `clank` denoting the start of an expression and `clonk` ending it. Parentheses may also be necessary for nested or multi-argument operations if the code is otherwise unclear.

Variable assignment:

    whirr <var> <value>

Variable names can be arbitrary strings, but convention dictates that they should be in-keeping with the language's robotic aesthetic.

Variable reference:

    brrring <var>
    
The variable referenced must be defined or the code will result in a `ParseError`.

If statement:

    bip <condition> <value_if_true> <value_if_false>

The `condition` can be either a boolean or a number (where 0 represents false, and anything else is true).

For loop:

    ratatat <n_loops> <body>

The variable holding the number of loops can be modified within the body to change the number of iterations at runtime.

## Math
Defined only on numbers.

Addition:

    plop <a> <b>

Multiplication:

    ting <a> <b>


## Logic
And:

    zap <a> <b>
    
Accepts either two booleans or two integers (where 0 represents false, and anything else is true).

Or:

    zorp <a> <b>

Accepts either two booleans or two integers (where 0 represents false, and anything else is true).

Negation:
    
    boing <a>

Flips an integer between positive and negative, or a boolean between true and false.

Greater than:

    zeep <a> <b>

Less than:

    zip <a> <b>

Equal:

    bzz <a> <b>

Accepts either booleans or integers.

