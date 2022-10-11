# jvmti-sample

The simple Java native agent written by rust.

## Build and Run

```sh
$ cargo build
$ java -agentpath:./target/debug/libjvmti_sample.dylib -version
```
