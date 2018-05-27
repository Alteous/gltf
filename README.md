
<h1 align="center">
    gltf
</h1>
<p align="center">
   <a href="https://travis-ci.org/gltf-rs/gltf">
      <img src="https://travis-ci.org/gltf-rs/gltf.svg?branch=master" alt="travis">
   </a>
   <a href="https://crates.io/crates/gltf">
      <img src="https://img.shields.io/crates/v/gltf.svg" alt="crates.io">
   </a>
   <a href="https://docs.rs/gltf">
      <img src="https://docs.rs/gltf/badge.svg" alt="docs.rs">
   </a>
   <a href="https://gitter.im/gltf-rs/Lobby">
      <img src="https://badges.gitter.im/gltf-rs/Lobby.svg" alt="gitter">
   </a>
</p>
<hr>

This crate is intended to load [glTF 2.0](https://www.khronos.org/gltf), a file format designed for the efficient transmission of 3D assets.

`rustc` version 1.19 or above is required.

### Reference infographic

![infographic](https://github.com/KhronosGroup/glTF/blob/master/specification/2.0/figures/gltfOverview-2.0.0a.png)

<p align="center">From <a href="github.com/javagl/gltfOverview">javagl/gltfOverview.</a></p>

### Usage

See the [crate documentation](https://docs.rs/gltf) for example usage.

### Extras and Names

By default, `gltf` ignores all `extras` and `names` included with glTF assets. You can negate this by enabling the `extras` and `names` features, respectively.

```toml
[dependencies.gltf]
version = "0.10"
features = ["extras", "names"]
```

### Examples

#### gltf-display

Demonstrates how the glTF JSON is deserialized.

```sh
cargo run --example gltf-display path/to/asset.gltf
```

#### gltf-tree

Visualises the scene heirarchy of a glTF asset, which is a strict tree of nodes.

```sh
cargo run --example gltf-tree path/to/asset.gltf
```

