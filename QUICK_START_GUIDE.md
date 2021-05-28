# Quick Start Guide

Install `dmntk` as an application (see [Usage](USAGE.md) for details).

Create a text file named `ex1.ctx`, copy the following content and save.

This is the evaluation context:

```text
{}
```

Create a text file named `ex1.feel`, copy the following content and save.

This is the evaluated `FEEL` textual expression:

```text
1 + 2
```

Now, to evaluate this expression, type:

```shell
# dmntk etx ex1.feel ex1.ctx
```

The result should be:

```shell
3
```
