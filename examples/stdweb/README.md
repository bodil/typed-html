# typed-html WASM example

Simple example of compiling app consuming typed-html to WebAssembly.

## Configure & Build

Make sure you have `cargo-web` installed: [Instructions](https://github.com/koute/cargo-web/#installation)

Build using `cargo web build --release`

_Note: There may be an issue that can be worked around as described [here](https://github.com/bodil/typed-html/issues/6), due to a bug in stdweb_

## Serve Through HTTP

We've also provided some simple scaffolding to easily serve the app through http:

```
$ cd www
$ yarn
$ node index.js
```

If the build is in a different directory, than the default, you can provide the directory as an argument:

```
$ node index.js /path/to/build
```
