# BLT-Rust-V1.0.2.6

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

var says it's a variable, first attribute: not-exists means to create a new variable, second attribute: int means integer, third attribute: null is placeholder text since variable is new, last attribute: 2 is value.
;var not-exists int null 2
;var not-exists int null 1

second attribute: string means the value is a string.
;var not-exists string null Hello World!

print says to print to the screen, first attribute: string says the value is not a variable but a string, and last attribute: Hello vute Chelsea! is the value.
;print string null Hello cute Chelsea!

wait says to wait time, first attribute: variable says the value is a variable, and last attribute: 0 is the variable: 2
;wait variable 0
;print string null Hello cute Rizo!

;print string null \n
;wait variable 1

first attribute: variable says it is a variable, the second attribute: null is aying not to convert the variable from int to string (Converting to string is only necisary when compiling to c++ or machine code), and last attribute: 2 is the variable: Hello World!
;print variable null 2

first attribute: exists says this variable is not a new variable, and the third attribute: 2 is the variable to change: Hello World!
;var exists string 2 Hello Big City!
;print variable null 2

;print string null Please type something

The second attribute: user-input, is saying that the variable is taking user input.
;var not-exists user-input
;print variable null 3

;var not-exists int null 0

add says to use addition the first and third attributes: int say that both of the numbers are not variables, the second and fourth attributes: 50 are the values of the numbers, and the last attribute: 4 is the variable to write the answer to.
;add int 50 int 50 4
;print variable to-string 4

subtract says to use subtraction
;subtract int 678 int 159 4
;print variable to-string 4

multiply says to use multiplication
;multiply int 50 int 50 4
;print variable to-string 4

divide says to use division
;divide int 50 int 50 4
;print variable to-string 4

;print string null Please type something:
;var not-exists user-input

;print string null Please type something:
;var not-exists user-input

if says that it is an if statement, attribute 1 and 4: variable, declare that objects 1 and 2 are variables, attributes 2 and 5: 5 and 6 are the values of the two objects, attribute 3: = is saying that the operation is equal to (== is also excepted) and attribute 6: 3, is the length of the if statement in lines.
;if variable 5 = variable 6 3 { ('{' is not necisary)
;   print string null You typed the same thing twice!

    attribute 2: seconds says that the wait command is taking an int for an input and not a varaiable and attribute 1: 1 is the number of seconds.
;   wait 1 seconds
;   print string null I hope ;)

} is the end of an if statement (only necisary for compiling)
;}

attribute 3: != is saying that the operation is not equal to.
;if variable 5 != variable 6 3
;   print string null You did not typed the same thing twice!
;   wait 1 seconds
;   print string null I hope ;)
;}
This doesn't do much because it is at the end of the program, but break ends the program.
;break
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