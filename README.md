**DMNTK** | Decision Model and Notation Toolkit

# dmntk

Decision Model and Notation Toolkit

[![Crates.io][crates-badge]][crates-url]
[![MIT licensed][mit-badge]][mit-url]
[![Apache 2.0 licensed][apache-badge]][apache-url]
[![Code coverage][coverage-badge]][coverage-url]

[crates-badge]: https://img.shields.io/crates/v/dmntk.svg
[crates-url]: https://crates.io/crates/dmntk
[mit-badge]: https://img.shields.io/badge/License-MIT-blue.svg
[mit-url]: LICENSE-MIT
[apache-badge]: https://img.shields.io/badge/License-Apache%202.0-blue.svg
[apache-url]: LICENSE-APACHE
[coverage-badge]: https://img.shields.io/badge/Coverage-0%25-green.svg
[coverage-url]: COVERAGE.md

## Installation

The fastest way to install DMNTK is using `cargo`:

```shell
$ cargo install dmntk
```

## Usage

Now run DMNTK:

```shell
$ dmntk
```

# Quick example

Create a text file named **ex1.ctx**, copy the content shown below and save in this file,
this is the evaluation context of the `FEEL` expression (empty context in this case).

```text
{}
```

Create a text file named **ex1.feel**, copy the content shown below and save in this file,
this is the evaluated `FEEL` expression (addition in this case).

```text
1 + 2
```

Evaluate `FEEL` expression by typing:

```shell
# dmntk efl ex1.ctx ex1.feel
```

The result should be `3`

```shell
3
```

## License

**dmntk** is distributed under the terms of both
the MIT&nbsp;license and the Apache&nbsp;License&nbsp;(Version&nbsp;2.0).
See [LICENSE-MIT](LICENSE-MIT) and [LICENSE-APACHE](LICENSE-APACHE) for details.


