package main

fun runPart2(input: List<Int>) : Int {
    for (i in input) {
        for (j in input) {
            for(k in input) {
                if (i + j + k == 2020) {
                    return i*j*k
                }
            }
        }
    }
    return  0
}

fun part2() {

    val numbers = readInputFile("day01/src/main/resources/input.txt")
        .map { line -> line.toInt() }

    val result = runPart2(numbers)
    println("Le résultat part 2 est $result")

}