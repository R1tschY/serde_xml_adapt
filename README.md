# serde-explicit-xml

More explicit XML Serialization and Deserialization

## Credits

This crate uses `quick-xml` as XML parser and writer.

Most code of this crate is based on `quick-xml` and `serde-xml-rs`. The main difference is that attributes are explicitly annotated (name starts with `@`):
```rust
#[derive(Serialize, Deserialize)]
struct Struct {
    #[serde(rename = "@attribute")]
    attribute: String,
    element: String,
}
```
```xml
<root attribute="attribute content">
    <element>element content</element>
</root>
```
It is therefore **not** a drop-in replacement for `quick-xml` or `serde-xml-rs`.

Other differences:
* `$value` works for serialisation
* Only `true`, `false`, `1`, `0` accepted for boolean

## Examples

Also look at the `serde` examples for inspiration: https://serde.rs/examples.html

### Attribute

```rust
#[derive(Serialize, Deserialize)]
struct Struct {
    #[serde(rename = "@string")]
    string: String,
    #[serde(rename = "@maybe_string")]
    maybe_string: Option<String>, // will be omitted when None
}
```
```xml
<root string="attribute content"/>
```

### Sequence

```rust
#[derive(Serialize, Deserialize)]
struct Struct {
    #[serde(rename = "string")]
    strings: Vec<String>,
}
```
```xml
<root>
    <string>one</string>
    <string>two</string>
</root>
```

### Inner value

Use the special `$value` as field name the model the inner value of an element:

```rust
#[derive(Serialize, Deserialize)]
struct Struct {
    #[serde(rename = "@attr")]
    attr: String,
    #[serde(rename = "$value")]
    value: String,
}
```
```xml
<root attr="">inner value</root>
```

### String enumeration

To model a string with only a strict set of values use an enum with the inner value (`$value`) as tag. 
For the example for the values `one`, `two`, `three`:
```rust
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "lowercase", tag = "$value")]
enum Enum {
    One,
    Two,
    Three,
}
```
```xml
<root>one</root>
```

