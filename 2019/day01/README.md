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

Before begin everything I'll will write up all the problem as unit tests.

### What I've learned

- How to open a file from a relative path
- How to read value from file and extract them as u32 vector