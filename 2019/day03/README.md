# [Day 2](https://adventofcode.com/2019/day/2)

## Part 1

### Problem

TODO

To help us the teammates gives these configurations and waited distance:

- wire 1 : `R8,U5,L5,D3`
- wire 2 : `U7,R6,D4,L4`
- distance : `6`

- wire 1 : `R75,D30,R83,U83,L12,D49,R71,U7,L72`
- wire 2 : `U62,R66,U55,R34,D71,R55,D58,R83`
- distance : `159`


- wire 1 : `R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51`
- wire 2 : `U98,R91,D20,R16,D67,R40,U7,R15,U6,R7`
- distance : `135`


### Resolution

Example of path followed by two wires:

- `R8,U5,L5,D3`
- `U7,R6,D4,L4`

Their intersections are the purple cross

![](./wires.png)

- First we have write a function that loads wires path
- Then we need to find intersection between these two path
- Finally we need to implement a function that calculate the Manhattan distance from Origin (0,0)