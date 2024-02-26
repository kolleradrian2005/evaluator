# Expression evaluator

This program requests an expression from the user and detects whether it is always false or it can be true.
<br>
For example
`x0 & !x0` is always false but `x0 & !x1`can be true if `x0` is true and `x1` is false.

### Usage

Enter the expression using `&`, `|`, `!` operators meaning and, or, negation respecively. Also use `0`, `1`, ... numbers meaning `x0`, `x1`, ...
<br>
Use the `-v` flag to print out all the considered combinations.
