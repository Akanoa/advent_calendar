package main

import java.io.File

fun readInputFile(pathname: String) : List<String> {
    return File(pathname)
        .readLines()
}