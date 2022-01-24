# Decision Model and Notation Toolkit

## Overview

**DMNTK** is a set of tools for building, testing and evaluating decision models.
**DMNTK** is based on the [Decision Model and Notation (DMN™)](https://www.omg.org/dmn/) specification,
the industry standard led by the [Object Management Group (OMG®)](https://www.omg.org/),
the institution behind such standards like UML®, BPMN™ and CORBA®.
**DMNTK** is written in [Rust](https://www.rust-lang.org/), a programming language that empowers
building reliable and efficient software.
**DMNTK** aspires to be the fastest and fully compliant with [DMN™ specification](https://www.omg.org/spec/DMN)
decision model evaluator.

Main **DMNTK** features:

- Evaluation of DMN models as a service.
- Evaluation of decision tables.
- Evaluation of FEEL expressions.
- Parsing and validating DMN models.
- Parsing and recognizing decision tables.
- Parsing and validating FEEL expressions.
- Testing DMN models.
- Testing decision tables.
- Testing FEEL expressions.
- Exporting DMN decision models to HTML.
- Exporting decision tables to HTML.
- Exporting FEEL expressions to HTML.

## Installation

**DMNTK** ships as a single executable with no dependencies.

Binary version of **DMNTK** may be
- obtained directly from [GitHub releases](https://github.com/dmntk/dmntk.rs/releases),
- built and installed from source using [Cargo](https://crates.io/crates/dmntk).

### Installing DMNTK using [Cargo](https://crates.io/crates/dmntk):

```shell
$ cargo install dmntk
```

## Getting started

**DMNTK** provides examples ready to play with.

To install the examples run:

```
$ dmntk exs
```

### Evaluate FEEL expression

```text
$ cd ./examples/e1
$ dmntk efe e1.ctx e1.feel
3
```

### Evaluate DMN model

```text
$ cd ./examples/e2
$ dmntk edm e2.ctx e2.dmn -i "Greeting Message"
"Hello John Doe"
```

### Evaluate decision table

```text
$ cd ./examples/e3
$ dmntk edt e3.ctx e3.dtb
0.15
```

### Run as a service

```text
$ cd ./examples/e2
$ dmntk srv -D .
Loaded 1 file(s) from directory: .
dmntk 0.0.0.0:22022
```

Switch to another terminal window and run:

```text
$ curl -s -d '{"Full Name":"John Doe"}' -H "Content-Type: application/json" -X POST http://0.0.0.0:22022/evaluate/compliance-level-2-test-0001/Greeting%20Message
{"data":"Hello John Doe"}
```

## Documentation

**DMNTK** documentation can be found on [dmntk.io](https://dmntk.io)

## DESIGN DMNTK LOGO!

Feel free to design an original logo for **Decision Model and Notation Toolkit**.
Please send the logo to [logo@dmntk.io](mailto:logo@dmntk.io).
Logo should be licensed under CC BY-SA 3.0 or similar.
