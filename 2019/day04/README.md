# [Day 4](https://adventofcode.com/2019/day/4)

## Part 1

### Problem

The idea of the puzzle is to find all passwords compliant with several rules:

- It's a six digit number between `264793` and `803935`
- Digits from left to right can't be decreasing, the next digit can't be less previous left one:
    - `11`      ✔
    - `12`      ✔
    - `21`      ❌
    - `1231`    ❌
    - `111123`  ✔
    - `135679`  ❌
    - `223450`  ❌
- The password must have at least one couple of digits:
    - `1`       ❌
    - `11`      ✔
    - `111`     ✔
    - `122`     ✔
    - `1221`    ✔
    - `111111`  ✔
    - `122345`  ✔
    - `123789`  ❌
    
The answer to the puzzle is the number of password that fit all rules.

### Resolution

There is three parts:

First, we need create a filter on password compliant to non-decreasing rule. 

To do so, for each password we first split the
password string in individual char, then parse them into an integer.

We loop on digit if one digit is less than previous one we return false

If all digit pass without triggering the false return, we return true

___

After, we create a filter on the rule specifying 
that the password involves a group of two same consecutive digit.

Like the first rule we split the password into char and then we check if get to consecutive 
equal digit. If so we return true.

If all digits are exhausted without triggering the true return, we return false.

___

Finally

We initialise a count to `0`.

We loop on number between `264793` and `803935`, each must be parse as string then we apply on it the two rules. 
If password is compliant to the two rules define above, we increment the counter.

Then return the counter.

### What I've learned ?

How to use `Chars` iterators.

## Part 2

This time we add a new rule to previous part.

This new rul is like the rule which says that the password involves a group of two same consecutive digit, but stricter.

The group must be exactly a group of 2 digits not a larger group.

Example:

- `1`       ❌ : not compliant to the only double rule only one '1' then end of string
- `11`      ✔ : compliant to the only double rule at the end of string
- `111`     ❌ : not compliant to the only double rule there is three '1' in row
- `1111`    ❌ : not compliant to the only double rule there is four '1' in row
- `11111`   ❌ : not compliant to the only double rule there is five '1' in row
- `111111`  ❌ : not compliant to the only double rule there is six '1' in row
- `113311`  ✔ : compliant to rule there is a least one strict double
- `113334`  ✔ : compliant to rule there is a least one double, there two '1' at the beginning
- `123334`  ❌ : not compliant to strict double rule, there isn't a strict double digit
- `112233`  ✔ : compliant to the strict double rule, there is a double '2' in the middle of string
- `123444`  ❌ : not compliant to strict double rule, there three '4' at the end of string
- `111122`  ✔ : compliant to double rule, there is a strict double '2' a the end of string

### Resolution

The code is heavily commented [rule implementation ](./src/lib.rs) line 59

### What I've learned

How to use Peekable iterators, to get value from iterator without consume it.

## Conclusion

The part 2 was very hard for me bevause I get the idea of the last rule at first and start with a wrong implementation. 
I've been also fooled by the Iterator behavior that consume data each step.

My tchat on Twitch was a really appreciated help ❤️

## Puzzle mark

Language skills: ⭐⭐⭐⭐☆

Puzzle: ⭐⭐⭐☆☆

Background story : ⭐⭐☆☆☆ 