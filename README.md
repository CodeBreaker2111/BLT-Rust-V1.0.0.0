# BLT-Rust-V1.4.7

Original BLT: https://github.com/CodeBreaker2111/BLT-2.0

### Sorry about the README not being very readable. I also probably have a lot of typoes. I'm not good at spelling.

## Tutorial

### Coding

#### Basic stuff

The BLT has 9 main commands:    These commands are from the original BLT and will be added but right now, as of BLT 1.0.2.5, only the print, var, break, wait, and add commands are fully added.

1. print: prints text to screen can be variable too
2. var: stores data including user input
3. wait: waits an amount of seconds can be a variable
4. break: ends the program with a return value of "break"
5. add: adds two numbers and writes the result to a variable
6. subtract: subtracts two numbers and writes the result to a variable
7. multiply: multiplies two numbers and writes the result to a variable
8. divide: divides two numbers and writes the result to a variable
9. if: Does basic logic with two objects like equal to or greater than

This next program here explains some stuff in the comments:

```Lines without a semicolon at the beggining are comments. Place a semicolon at the beggining to make a line of code.

Lines without a semicolon at the beggining are comments. Place a semicolon at the beggining to make a line of code.

var says it's a variable, first attribute: u32, is the type, second attribute: var1 var2, is the variable, and the third attribute: 2 1, is the value
;var u32 var1 2000
;var u32 var2 1000

first attribute: string means the value is a string.
;var string var3 Hello World!

print says to print to the screen, first attribute: string says the value is not a variable but a string, and last attribute: Hello cute Chelsea! is the value.
;print string Hello cute Chelsea!

wait says to wait time, first attribute: var says the value is a variable, and last attribute: var1 is the variable: 2000, and will wait 2000 milliseconds
;wait var var1
;print string null Hello cute Rizo!

;print string \n
;wait var var2

first attribute: variable says it is a variable, and last attribute: 2 is the variable: Hello World!
;print var var3

the second attribute: var3 is the variable to change: Hello World!
;var string var3 Hello Big City!
;print var var3

;print string Please type something

;var f32 var5 0.0

add says to use addition the first and third attributes: int say that both of the numbers are not variables, the second and fourth attributes: 50 are the values of the numbers, and the last attribute: var5 is the variable to write the answer to.
;add num 50.0 num 50.0 var5
;print var var5

subtract says to use subtraction
;subtract num 678.0 num 159.0 var5
;print var var5

multiply says to use multiplication
;multiply num 50.0 num 50.0 var5
;print var var5

divide says to use division
;divide num 50.0 num 50.0 var5
;print variable to-string 4

;var string user_input null

;print string Please type something

readln says to get user input, and the first attribute: user_input, is the variable to write to.
;readln user_input

This doesn't do much because it is at the end of the program, but '}' or 'break' ends the program.
;}
```

#### Some quirks about the BLT

1. Lines beggining with a ';' are commands, lines with no ';' are comments.
2. Lines do not end with semicolon.
3. Each command has a lot of attributes and most stuff is not automatick.
4. Sometimes attributes end up just being placeholders which are all the 'nulls' but can be anything you want.
5. The BLT is VERY hard to read and I will fix that soon (It's getting slightly better).
6. If the program does not end in a newline, the last line will not be run.

#### UI

This one is pretty selfexplanitory.

## How the BLT works

### UI

it does ui: 'ui.rs'

### Compiling

The 'compile_file.rs' does the compiling. It first splits up the code into lines (one line is semicolon to newline), then into tokens. The program iterates through the tokens and decides what to do. It also makes unecisary prints.

## What is next for the BLT?

The next main feature I am going to add is readability. Due to commands using numbers to define attributes and variables not having names, it is very hard to read. Those two things I mentioned I will change in the next update.