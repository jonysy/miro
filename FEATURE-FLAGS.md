[Conditional Compilation](https://doc.rust-lang.org/book/conditional-compilation.html):

# Experimental/Pilot/Exploratory flag

## Guarded Float Points

* Easier to work with. For example, `FloatGuard` implements `Step`.
* Performance costs. For example, Checks may cause certain algorithms to be a bit slower

```sh
cargo [command] --features "pilot"
```