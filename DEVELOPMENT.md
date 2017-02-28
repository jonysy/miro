[Conditional Compilation](https://doc.rust-lang.org/book/conditional-compilation.html):

# Experimental/Pilot/Exploratory flag

## Guarded Float Points

* Easier to work with. For example, `FloatGuard` implements `Step`.
* Performance costs. For example, checks may cause certain algorithms to be a bit slower.

```sh
cargo [command] --features "pilot"
```

## TODO

[ ] Use "checked" operations instead of `unwrap`ing everything.

[ ] Check for small performance cost: [How copying an int made my code 11 times faster]

[How copying an int made my code 11 times faster]: https://medium.com/@robertgrosse/how-copying-an-int-made-my-code-11-times-faster-f76c66312e0f