# Robo-identities

Robo-identities-wasm is a `wasm-pack` module that bundles `robonames` and `robohash` functionality in a easy to use NPM package. 

Current WASM size is `1.1MB` (bundles every background and green robot part as compressed webp). The different colors are generated via HUE rotation (saving 90% in the weight of the robot parts). Generating a 256x256 robohash takes ~40-50ms. The performance is slightly worse (+30%) than reading uncompressed `.png` files from the filesystem.

## 🚴 Usage

### 🛠️ Build with `wasm-pack build`

```
wasm-pack build
```

### 🔬 Test in Headless Browsers with `wasm-pack test`

```
wasm-pack test --headless --firefox
```

### 🎁 Publish to NPM with `wasm-pack publish`

```
wasm-pack publish
```

## License

Licensed under either of

* MIT license ([LICENSE-MIT](LICENSE) or http://opensource.org/licenses/MIT)