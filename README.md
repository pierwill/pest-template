# pest-template

To make a new parser project using [`pest`](https://pest.rs):

```
cargo install cargo-generate
cargo generate --git https://github.com/pierwill/pest-template
```

## Example

The example uses the following Lisp-like grammar:

```
WHITESPACE = _{ " " }

symbol = { (ASCII_ALPHA | ASCII_DIGIT)+ }

list = { "(" ~ (symbol | list)+ ~ ")" }
```

Run `cargo run` to see the output of parsing `(hello (world))`
with the above grammar:

```
Pair {
    rule: list,
    span: Span {
        str: "(Hello (world))",
        start: 0,
        end: 15,
    },
    inner: [
        Pair {
            rule: symbol,
            span: Span {
                str: "Hello",
                start: 1,
                end: 6,
            },
            inner: [],
        },
        Pair {
            rule: list,
            span: Span {
                str: "(world)",
                start: 7,
                end: 14,
            },
            inner: [
                Pair {
                    rule: symbol,
                    span: Span {
                        str: "world",
                        start: 8,
                        end: 13,
                    },
                    inner: [],
                },
            ],
        },
    ],
}
```
