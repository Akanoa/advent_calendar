package main

fun runPart1(input: List<Int>) : Int {
    for (i in input) {
        for (j in input) {
            if (i + j == 2020) {
                return i*j
            }
        }
    }
    return  0
}

fun part1() {

    val numbers = readInputFile("day01/ressources/input.txt")
        .map { line -> line.toInt() }

    val result = runPart1(numbers)
    println("Le résultat part 1 est $result")

}