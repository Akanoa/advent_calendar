plugins {
    java
    kotlin("jvm") version "1.4.10"
}

group = "me.noa"
version = "1.0-SNAPSHOT"

repositories {
    mavenCentral()
}

dependencies {
    implementation(kotlin("stdlib"))
    testCompile("junit", "junit", "4.12")
}
