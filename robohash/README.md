# RoboHash

Rust implementation of [RoboHash](https://github.com/e1ven/Robohash/) by [e1ven](https://github.com/e1ven).

This is a fork of the [RoboHash rust implementation](https://github.com/kyco/robohash) by @kyco . 

This fork introduces custom art (new robot parts) for RoboSats. The goal is to build a compact WASM capable of generating robot avatars in the web frontend. Fot this, we might need to strongly compress the set1 part and embed them into the binary.