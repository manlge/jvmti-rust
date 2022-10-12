jvmti-rust
==========

An extensible, safe native JVM agent implemented in pure Rust.

## A word of warning

This project is far from being complete or usable to say the least and contains
a healthy dose of proof-of-concept code that is guaranteed to either work or not.

## Abstract

Rust JVMTI is intended to become a slim JVM
application performance management (APM) tool leveraging both safe access to native
JVM functionality via Rust and byte code instrumentation using Java code.  

## Already implemented (probably poorly)

* Ability to connect to a JVM as a native agent library
* Read and parse loaded class files
* Generate byte code from loaded or created class files
* Gathering and displaying statistics about method class, class loading and synchronization times
* Read basic command line configuration
* Basic JVM emulator for implementing unit tests without the need for an actual JVM

## Usage

Please see [the example](../sample/README.md).
