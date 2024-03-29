import org.gradle.internal.jvm.Jvm
import java.io.ByteArrayOutputStream

plugins {
    // Apply the application plugin to add support for building a CLI application in Java.
    application
}

repositories {
    // Use Maven Central for resolving dependencies.
    mavenCentral()
}

dependencies {
    // Use JUnit Jupiter for testing.
    testImplementation(libs.junit.jupiter)

    testRuntimeOnly("org.junit.platform:junit-platform-launcher")

    // This dependency is used by the application.
    implementation(libs.guava)
}

// Apply a specific Java toolchain to ease working on different environments.
java {
    toolchain {
        languageVersion = JavaLanguageVersion.of(21)
    }
}

application {
    // Define the main class for the application.
    mainClass = "org.example.App"
}

tasks.named<Test>("test") {
    // Use JUnit Platform for unit tests.
    useJUnitPlatform()
}

val generateJniHeaders: Task by tasks.creating {
    val jniHeaderDir = file("src/main/generated/jni")

    group = "build"
    dependsOn(tasks.getByName("compileJava"))

    inputs.dir("src/main/java")
    outputs.dir(jniHeaderDir)

    doLast {
        val javaHome = Jvm.current().javaHome
        val javap = javaHome.resolve("bin").walk().firstOrNull { it.name.startsWith("javap") }?.absolutePath ?: error("javap not found")
        val javac = javaHome.resolve("bin").walk().firstOrNull { it.name.startsWith("javac") }?.absolutePath ?: error("javac not found")
        val buildDir = file("build/classes/java/main")
        val tmpDir = file("build/tmp/jvmJni").apply { mkdirs() }

        val bodyExtractingRegex = """^.+\Rpublic \w* ?class ([^\s]+).*\{\R((?s:.+))\}\R$""".toRegex()
        val nativeMethodExtractingRegex = """.*\bnative\b.*""".toRegex()

        println("Beginning to generate JNI headers.")
        println("javaHome is ${javaHome.absolutePath}")
        println("javap is $javap")
        println("javac is $javac")

        buildDir.walkTopDown()
                .filter { "META" !in it.absolutePath }
                .forEach { file ->
                    if (!file.isFile) return@forEach

                    val output = ByteArrayOutputStream().use {
                        project.exec {
                            commandLine(javap, "-private", "-cp", buildDir.absolutePath, file.absolutePath)
                            standardOutput = it
                        }.assertNormalExitValue()
                        it.toString()
                    }

                    val (qualifiedName, methodInfo) = bodyExtractingRegex.find(output)?.destructured ?: return@forEach

                    val lastDot = qualifiedName.lastIndexOf('.')
                    val packageName = qualifiedName.substring(0, lastDot)
                    val className = qualifiedName.substring(lastDot+1, qualifiedName.length)

                    val nativeMethods =
                            nativeMethodExtractingRegex.findAll(methodInfo).mapNotNull { it.groups }.flatMap { it.asSequence().mapNotNull { group -> group?.value } }.toList()
                    if (nativeMethods.isEmpty()) return@forEach

                    val source = buildString {
                        appendln("package $packageName;")
                        appendln("public class $className {")
                        for (method in nativeMethods) {
                            if ("()" in method) appendln(method)
                            else {
                                val updatedMethod = StringBuilder(method).apply {
                                    var count = 0
                                    var i = 0
                                    while (i < length) {
                                        if (this[i] == ',' || this[i] == ')') insert(i, " arg${count++}".also { i += it.length + 1 })
                                        else i++
                                    }
                                }
                                appendln(updatedMethod)
                            }
                        }
                        appendln("}")
                    }
                    val outputFile = tmpDir.resolve(packageName.replace(".", "/")).apply { mkdirs() }.resolve("$className.java").apply { delete() }.apply { createNewFile() }
                    outputFile.writeText(source)

                    println("Generating for ${outputFile.absolutePath} into ${jniHeaderDir.absolutePath}")
                    project.exec {
                        commandLine(javac, "-h", jniHeaderDir.absolutePath, outputFile.absolutePath)
                    }.assertNormalExitValue()
                }
    }
}
