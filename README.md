# purescript-strings

[![Latest release](http://img.shields.io/github/release/purescript/purescript-strings.svg)](https://github.com/purescript/purescript-strings/releases)
[![Build status](https://github.com/purescript/purescript-strings/workflows/CI/badge.svg?branch=master)](https://github.com/purescript/purescript-strings/actions?query=workflow%3ACI+branch%3Amaster)
[![Pursuit](https://pursuit.purescript.org/packages/purescript-strings/badge)](https://pursuit.purescript.org/packages/purescript-strings)

String and char utility functions, regular expressions.

## Installation

```
spago install strings
```

## Documentation

Module documentation is [published on Pursuit](http://pursuit.purescript.org/packages/purescript-strings).

## Rust tests

Run the original PureScript test suite through the sibling `purust` compiler:

```sh
./bin/test -c
```

The `-c` option rebuilds `purust` and clears this package's Spago cache. Every
run regenerates this package's TAST and Rust output, compiles the native binary,
and checks its exit status, stderr and the output of all eight test modules.
The executable has a 60-second timeout. Other packages' caches are preserved.
Use `./bin/test` or `npm test` to reuse the existing compiler bundle.

The runner uses Spago from `../purust/node_modules/.bin` and prefers the sibling
TAST-enabled PureScript fork. Set `PURS` to select another build of that fork.
Node.js, Cargo and the compiler's installed npm dependencies are required;
the runner does not install dependencies.

The unchanged suite contains 481 assertion calls across `Data.String`,
`CodePoints`, `CodeUnits`, `Unsafe`, `Regex`, `CaseInsensitive`, `NonEmpty`
and `NonEmpty.CodeUnits`. It covers Unicode code points and UTF-16 positions,
searches and boundaries, string transformations, regex captures and callbacks,
and the nonempty string operations. The JavaScript test command remains
available as `npm run test:js`, including its `codePointAt` fallback check.

The Rust regex implementation uses `fancy-regex`; passing this suite does not
imply complete ECMAScript regex compatibility. In Unicode (`u`) mode, isolated
UTF-16 surrogates are rejected explicitly instead of being replaced silently.
