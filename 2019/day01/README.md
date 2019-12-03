# [Day 1](https://adventofcode.com/2019/day/1)

## Part 1

### Problem

You want to launch a rocket and you have to define the amount of fuel required.

The fuel required is based on the mass with the relation:
    
    fuel = (  floor( mass / 3 )  - 2 )
    
For example:

- For a mass of `12`, divide by `3` and round down to get `4`, then subtract 2 to get `2`.
- For a mass of `14`, dividing by `3` and rounding down still yields `4`, so the fuel required is also `2`.
- For a mass of `1969`, the fuel required is `654`.
- For a mass of `100756`, the fuel required is `33583`.

But you don't know the total mass of your rocket, you only know the mass of its modules.

This mass module list is given by this file: [modules list](assets/modules_mass_list.txt). Each line matches to a module mass.

To know the fuel total required to launch the rocket, you've have to sum up the fuel required for each modules.

### Resolution

- Before begin everything I'll will write up all the problem as unit tests.
- Implement the method that converts mas to fuel amount
- Then create a method to load module mass
- And finally the method that sum up all the module fuel amount and display the wanted number.

### What I've learned

- How to open a file from a relative path
- How to read value from file and extract them as u32 vector

## Part 2

Quite the same thing but this time we take in account the real laws of the physics. To carry fuel in a rocket you 
need more fuel.

To model this we'll use the same relation as before but we need apply it recursively until we get the right amount of fuel.

To simplify the fuel density is 1.00, so 1 unit of fuel equals 1 mass.

As example:

- A module of mass  `14` requires `2` fuel. This fuel requires no further fuel (2 divided by 3 and rounded down is `0`, which would call for a negative fuel), so the total fuel required is still just `2`.
- At first, a module of mass `1969` requires `654` fuel. Then, this fuel requires `216` more fuel `(654 / 3 - 2)`. `216` then requires `70` more fuel, which requires `21` fuel, which requires `5` fuel, which requires no further fuel. So, the total fuel required for a module of mass `1969` is `654 + 216 + 70 + 21 + 5 = 966`.
- The fuel required by a module of mass `100756` and its fuel is: `33583 + 11192 + 3728 + 1240 + 411 + 135 + 43 + 12 + 2 = 50346`

###Resolution

Same as the part unless there is a recursive call to implement with a fuel negative amount as stop condition

### What I've learned

Not much, I was already familiar with the recursive concept.


## Conclusion

Reading data out of the file was the hardest part. I think I'll extract this method for the next days challenge.

The puzzle was quite fun but not really hard to solve.

## Puzzle mark
 ⭐⭐⭐☆☆