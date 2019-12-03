# [Day 2](https://adventofcode.com/2019/day/2)

## Part 1

### Problem

Your work on day 1 paid, you are in space but your computer is broken and you will crash on the Moon.
You've to fix your on board computer that give you the error `1202`.
Before anything you've to know whether your computer is working or not.

To do so, the Control room give you some information and some bench data.

The computer is relatively simple. It works with a primitive assembly language, called `IntList`.
The first integer is the `opcode` of the command. There is 3 opcodes:
- 1 : add
- 2 : multiply
- 99 : stop the program

For the `add` and `multiply` opcodes, the two following integer are the address of the operand and the third one is the
address where the value of operation must be stored. Then we move the instruction cursor of 4 integers. 

__Exemple__:

The IntList : `1, 0, 0, 0, 99` after running gives `2, 0, 0, 0, 99`. 

Because we read the first integer which is a `1` so we know tha we will add together the to following address values and 
store them at 3rd address value. 

In our case the 2nd integer is a `0` so the first operand is the value at address #0 which is
a `1`, with the way the 2nd integer is also `0` thus the second operand is also a `1`. 

Finally 3rd integer is also a `0` so the result must be stored at the address `#0`.

This give us `@0 = 1 + 1`

Because the command is accomplished, we can move our cursor to the next instruction

The next opcode is `99` == `stop`. We stop the program.  

    Note: `stop` can have following data but because the program must stop, those ones doesn't matter.

To help us in our reparation our teammates stayed at land, give us theses examples of working simulations:

- `1,0,0,0,99` becomes `2,0,0,0,99` (1 + 1 = 2).
- `2,3,0,3,99` becomes `2,3,0,6,99` (3 * 2 = 6).
- `2,4,4,5,99,0` becomes `2,4,4,5,99,9801` (99 * 99 = 9801).
- `1,1,1,4,99,5,6,0,99` becomes `30,1,1,4,2,5,6,0,99`.

### Resolution

- Before begin everything I'll will write up all the problem as unit tests.
- Then I implemented the computer method to fix tests
- After I wrote the read from file method

### What I've learned

- I know, now, how to implement a trait on enum, I use it to convert the opcode value as
an enum variation using the `From` trait.
- I learned how to split a string and convert its parts as integer