# Tracing SLF4j
SLF4j backend compatible with Rust's tracing crate.

## Purpose
The purpose of this crate is to allow Rust programs that embed Java programs to receive logging from
the Java parts, if those parts are using SLF4j.

## Compiling
This crate requires a Java compiler installer. the `JAVA_HOME` environmental variable should be set.

## Usage
When using JNI's invocation API, the JAR file embedded in this crate
should be added to the classpath:

1. Save the jarfile (const `DEPENDENCIES`) to disk
2. Add the option `-Djava.class.path=<PATH TO JARFILE>` to the JVM's start parameters.

After the JVM has been started, the setup `register_log_fn` function should be called:
```rs
tracing_slf4j::register_log_fn(&mut env).unwrap();
```
This function will register the Rust logging handler with the JVM.