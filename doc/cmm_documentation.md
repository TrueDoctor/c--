C-- Documentation
=======

## C-- is a language with syntax similar to C but but only a fractions of its features. It is compiled to brainfuck.

# Types
There are two types in C--, void and int.
void is used as return type of a function, to indicate that it doesn't return anything. A variable can't have void as type.
int is an integer type. It is represented by the brainfuck cells in the underlying brainfuck implementation, so its size may vary.
int literals can be normal (positive) decimal integers ([0-9]+), true, which gets interpreted as 1, false, which gets interpreted as 0 or char literals '(.)' which get interpreted as their ASCII value.

---------

# Expressions
Expressions in C-- are similar to C expressions, but with less operators. Note that assignment is NO operator in C-- and is instead interpreted as statement. Standalone expressions are not allowed.
Operator Precedence
    1. ()         function call
    2. + -        unary plus/minus
    3. * / %      multiplication, division, remainder
    4. < > <= >=  relational
    5. == !=      relational
    6. not        logical not
    7. and        logical and
    8. or         logical or
Note: Function calls in expressions must return int.

Declarations
Variables can only be declared as int and optionally initialized with = (same syntax as in C). Functions can't be declared.

----------

# Statements
* blocks: { {<block-item>}* }
    like in C, block items can be statements or declarations, but the declarations don't need to be before the statements
* if-else: if ( <exp> ) <statement1>
           if ( <exp> ) <statement1> else <statement2>
    if <exp> is true (nonzero), <statement1> is executed, else (if an else is present) <statement2> is executed
 * while: while ( <exp> ) <statement>
    like in C, <statement> is repeated until <exp> is false (zero)
 * repeat: repeat ( <exp> ) <statement>
    <statement> is repeated <exp> amount of times
 * assignments: <id> <assign-op> <exp> ;
    just like in C, but assignments are no expressions, <assign-op> can be =, +=, -=, *=, /= or %=
 * function calls: <id> ( <args> ) ;
    return type must be void

-------

#Functions

Because brainfuck has no functions, C-- only has inline functions. That means functions can't be recursive. The syntax for function definition is the same as in C, but they don't need to be declared. 
