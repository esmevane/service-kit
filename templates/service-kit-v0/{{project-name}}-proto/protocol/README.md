# Protocol definitions

This is a protocol definitions / output folder.

## What is this?

This folder contains protocol definitions written with Protobuf, and the output which prost creates when it analyzes them.

## Why is this here?

These are accessed by the prost portion of the the application, and they are kept here in order to expose them to source control as well as human observation, so the generated output from prost is easy to fact check and reference.

## Where does this connect to the kit?

The `build.rs` script manages the build process, and the `include!` statement in the core crate leverages the built artifacts.

## What's the intent to all of this?

There are 2 client interfaces and one service interface bundled with this application out of the box. Moving them all in one single go could be tough to do without introducing regressions. The added step of leveraging a service for them to implement creates a place where you can introduce compiler errors unless they don't match.

Feel free to discard all of it if you don't like the protocol buffers!
