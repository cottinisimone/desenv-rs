# DesEnv-rs

Environment variables deserializer library.

With this library is possible to deserialize a bunch of environment variables in a more structured data type.

## Install

```toml
# Cargo.toml
[dependencies]
desenv = "0.1.0"
```

## Usage

The library expose a utility function to load the configuration from environment variables
```rust
fn main() {
    let _config: Config = desenv::load().expect("Failed to load configuration");
}
```

Then `Config` should be something like this

```rust
use desenv::Desenv;

#[derive(Desenv)]
pub struct Config {
    field1: String,
    field2: String,
}
```

In order to have a successful loaded `Config` it's needed that in the env there are two variables:
- `FIELD1`
- `FIELD2`

### Customize configuration

It is possible to customize how the configuration is loaded.

#### Rename

Using `rename` modifier on `desenv` field attribute is possible to instruct the library to look for a different variable.
In the example above, while loading the configuration, the library will try to find the `FIELD_1` variable.

```rust
use desenv::Desenv;

#[derive(Desenv)]
pub struct Config {
    #[desenv(rename = "FIELD_1")]
    field1: String,
}
```

Pay attention because rename attribute is case-sensitive. So in an example like the above one the library will try to find
an environment variable called `downcased_field_1`.

```rust
use desenv::Desenv;

#[derive(Desenv)]
pub struct Config {
    #[desenv(rename = "down_case_field_1")]
    field1: String,
}
```

#### Default

With this modifier is possible to instruct the library to deserialize an environment variable with a default value.

#### With value

If the `FIELD1` environment variable does not exist the library will fill `field1` with `value`.

```rust
use desenv::Desenv;

#[derive(Desenv)]
pub struct Config {
    #[desenv(default = "value")]
    field1: String,
}
```

or

```rust
use desenv::Desenv;

#[derive(Desenv)]
pub struct Config {
    #[desenv(default(value = "value"))]
    field1: String,
}
```

#### With env

If the `FIELD1` environment variable does not exist the library will try to load, as fallback, the `FALLBACK_FIELD1`
environment variable. If `FALLBACK_FIELD1` does not exist too, then the load will fail.

```rust
use desenv::Desenv;

#[derive(Desenv)]
pub struct Config {
    #[desenv(default(env = "FALLBACK_FIELD1"))]
    field1: String,
}
```

#### With std default

If the `FIELD1` environment variable does not exist the library will fill `field1` with the `Default` implementation for
that type. So, for example, for a string the default will be an empty string.

```rust
use desenv::Desenv;

#[derive(Desenv)]
pub struct Config {
    #[desenv(default)]
    field1: String,
}
```

#### Separator

Instruct the library on how to deserialize a string to a vector performing a `split` over the string. The provided value
must be a char. Cannot be used on non-vector fields.

```rust
use desenv::Desenv;

#[derive(Desenv)]
pub struct Config {
    #[desenv(separator = ';')]
    field1: String,
}
```

#### Nested

Tells the library that the specified field shouldn't be deserialized using an environment variable but using `Desenv`
macro too.

```rust
use desenv::Desenv;

#[derive(Desenv)]
pub struct Config {
    #[desenv(nested)]
    field1: NestedConfig,
}

#[derive(Desenv)]
pub struct NestedConfig {
    field: String,
}
```


#### Prefix

If set apply that prefix for every environment variable deserialized. In the example below `field1` will be deserialized
as `PREFIXED_FIELD1`. Note that prefix is case-sensitive.

```rust
use desenv::Desenv;

#[derive(Desenv)]
#[desenv(prefix = "PREFIX_")]
pub struct Config {
    field1: String,
}
```

If prefixed struct has a prefixed nested struct the result will be a concatenation of both the prefixes. In the example
the field in `NestedConfig` will be resolved using `PARENT_NESTED_FIELD` environment variable.

```rust
use desenv::Desenv;

#[derive(Desenv)]
#[desenv(prefix = "PARENT_")]
pub struct Config {
    #[desenv(nested)]
    field1: NestedConfig,
}

#[derive(Desenv)]
#[desenv(prefix = "NESTED_")]
pub struct NestedConfig {
    field: String,
}
```

#### OsString

`OsString`s are supported for some specific cases where an environment variable is not utf8 encodable.

```rust
use std::ffi::OsString;
use desenv::Desenv;

#[derive(Desenv)]
pub struct Config {
    field1: OsString,
}
```

### Supported types

Right now every `T` that mix-in the `FromStr` trait could be used as simple fields. Other supported types are:
- `Option<T>`
- `Vec<T>`
- `OsString`
