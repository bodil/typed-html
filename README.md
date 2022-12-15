[![Build Status](https://travis-ci.org/bodil/typed-html.svg?branch=master)](https://travis-ci.org/bodil/typed-html)

# axohtml

This crate provides the `html!` macro for building fully type checked HTML
documents inside your Rust code using roughly [JSX] compatible syntax. 

This crate is a fork of the great [Bodil Stokke's] [typed-html] crate. Opted 
for a fork instead of maintainership because not currently intending to use or
maintain the Wasm compatibility (for now).

[Bodil Stokke's]: https://github.com/bodil
[typed-html]: https://github.com/bodil/typed-html

## Company-focused Maintenance

This project is currently being maintained soley for use in [@axodotdev's]
projects. Feel free to file issues or PRs but anticipate that they won't be
prioritized.

## Quick Preview

```rust
let mut doc: DOMTree<String> = html!(
    <html>
        <head>
            <title>"Hello Axo"</title>
            <meta name=Metadata::Author content="Axo Developer Co."/>
        </head>
        <body>
            <h1>">o_o<"</h1>
            <p class="official">
                "The tool company for tool companies"
            </p>
            { (0..3).map(|_| html!(
                <p class="emphasis">
                    ">o_o<"
                </p>
            )) }
            <p class="citation-needed">
                "Every company should be a developer experience company"
            </p>
        </body>
    </html>
);
let doc_str = doc.to_string();
```

## Syntax

This macro largely follows [JSX] syntax, but with some differences:

* Text nodes must be quoted, because there's only so much Rust's tokeniser can
  handle outside string literals. So, instead of `<p>Hello</p>`, you need to
  write `<p>"Hello"</p>`. (The parser will throw an error asking you to do this
  if you forget.)
* Element attributes will accept simple Rust expressions, but the parser has
  its limits, as it's not a full Rust parser. You can use literals,
  variables, dotted properties, type constructors and single function or
  method calls. If you use something the parser isn't currently capable of
  handling, it will complain. You can put braces or parentheses around the
  expression if the parser doesn't understand
  it. You can use any Rust code inside a brace or parenthesis block.

## Valid HTML5

The macro will only accept valid HTML5 tags, with no tags or attributes marked
experimental or obsolete. If it won't accept something you want it to accept, we
can discuss it over a pull request (experimental tags and attributes, in
particular, are mostly omitted just for brevity, and you're welcome to implement
them).

The structure validation is simplistic by necessity, as it defers to the type
system: a few elements will have one or more required children, and any element
which accepts children will have a restriction on the type of the children,
usually a broad group as defined by the HTML spec. Many elements have
restrictions on children of children, or require a particular ordering of
optional elements, which isn't currently validated.

## Attribute Values

Brace blocks in the attribute value position should return the expected type for
the attribute. The type checker will complain if you return an unsupported type.
You can also use literals or a few simple Rust expressions as attribute values
(see the Syntax section above).

The `html!` macro will add an [`.into()`][Into::into] call to the value
expression, so that you can use any type that has an [`Into<A>`][Into] trait
defined for the actual attribute type `A`.

As a special case, if you use a string literal, the macro will instead use the
[`FromStr<A>`][FromStr] trait to try and parse the string literal into the
expected type. This is extremely useful for eg. CSS classes, letting you type
`class="css-class-1 css-class-2"` instead of going to the trouble of
constructing a [`SpacedSet<Class>`][SpacedSet]. The big caveat for this:
currently, the macro is not able to validate the string at compile time, and the
conversion will panic at runtime if the string is invalid.

### Example

```rust
let classList: SpacedSet<Class> = ["foo", "bar", "baz"].into();
html!(
    <div>
        <div class="foo bar baz" />         // parses a string literal
        <div class=["foo", "bar", "baz"] /> // uses From<[&str, &str, &str]>
        <div class=classList />             // uses a variable in scope
        <div class={                        // evaluates a code block
            SpacedSet::from(["foo", "bar", "baz"])
        } />
    </div>
)
```

## Generated Nodes

Brace blocks in the child node position are expected to return an
[`IntoIterator`][IntoIterator] of [`DOMTree`][DOMTree]s. You can return single
elements or text nodes, as they both implement `IntoIterator` for themselves.
The macro will consume this iterator at runtime and insert the generated nodes
as children in the expected position.

### Example

```rust
html!(
    <ul>
        { (1..=5).map(|i| html!(
            <li>{ text!("{}", i) }</li>
        )) }
    </ul>
)
```

## Rendering

You have two options for actually producing something useful from the DOM tree
that comes out of the macro.

### Render to a string

The DOM tree data structure implements [`Display`][Display], so you can call
[`to_string()`][to_string] on it to render it to a [`String`][String]. If you
plan to do this, the type of the tree should be [`DOMTree<String>`][DOMTree] to
ensure you're not using any event handlers that can't be printed.

```rust
let doc: DOMTree<String> = html!(
    <p>"Hello Axo"</p>
);
let doc_str = doc.to_string();
assert_eq!("<p>Hello Axo</p>", doc_str);
```

### Render to a virtual DOM

The DOM tree structure also implements a method called `vnode()`, which renders
the tree to a tree of [`Node`][Node]s, which is a mirror of the generated tree
with every attribute value rendered into `String`s. You can walk this virtual
DOM tree and use it to build an actual DOM tree with `stdweb` or pass it on to
your favourite virtual DOM system.

## License


This software is subject to the terms of the Mozilla Public License, v. 2.0. If
a copy of the MPL was not distributed with this file, You can obtain one at
<http://mozilla.org/MPL/2.0/>.

Copyright 2018 Bodil Stokke, 2022 Axo Developer Co.

[JSX]: https://reactjs.org/docs/introducing-jsx.html
[Display]: https://doc.rust-lang.org/std/fmt/trait.Display.html
[String]: https://doc.rust-lang.org/std/string/struct.String.html
[to_string]: https://doc.rust-lang.org/std/string/trait.ToString.html#tymethod.to_string
[Node]: dom/trait.Node.html
[FromStr]: https://doc.rust-lang.org/std/str/trait.FromStr.html
[SpacedSet]: types/struct.SpacedSet.html
[IntoIterator]: https://doc.rust-lang.org/std/iter/trait.IntoIterator.html
[Into]: https://doc.rust-lang.org/std/convert/trait.Into.html
[Into::into]: https://doc.rust-lang.org/std/convert/trait.Into.html#method.into
[DOMTree]: dom/type.DOMTree.html
