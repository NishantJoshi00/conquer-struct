# Conquer Struct

Macros that allow, constructing struct by providing fields as futures.
These macros, automatically perform, future resolution concurrently,
and construct the struct.

## Why

Consider a scenerio, where you are trying to construct a struct,
and have to fetch it's fields asynchronously.
The code to perform this action would looks something like this:

```rust

async fn get_contact_info(contact_id: usize) -> Contact {
    Contact {
        name: get_contact_name(contact_id).await,
        number: get_contact_number(contact_id).await,
        is_verified: true
    }
}

```

Seems simple, now as a performance improvement, we need to call those functions
concurrently. Oh, that's a refactor. It would require us to add more code,
and keep track of the functions that are being executed.
It's definitely possible, but consider huge structs it would
definitely be difficult to keep track and you might
assign outputs of different functions to difficult variables.

## How

Introducing `conquer_struct`, as the name suggest it will
conquer concurreny for you and you can avoid writing any boilerplate
code. Let's take the same example as above, look at how to make it concurrent

```rust

async fn get_contact_info(contact_id: usize) -> Contact {
    conquer_future!(Contact {
        name: async { get_contact_name(contact_id).await },
        number: async { get_contact_number(contact_id).await },
        is_verified: true
    }).await
}

```

Done! Wasn't is simple, with minimal change our code is transformed into
concurrent code.

## Usage

`conquer_struct` provides use with 2 macros.

`conquer_future` this macro resolves the futures provided inside the struct.
the macro has a rough signature:

```rust

conquer_future!(StructName {
    value1: T,
    value2: async { .. } // ~ impl Future<Output = V>, considering the field
                        // accepts type V
}) -> impl Future<Output = StructName>

```

`try_conquer_future` this macro resolves the futures provided inside the struct,
as well as consider the result of the future that are present inside.

```rust

try_conquer_future!(StructName {
    value1: T,
    value2: async { .. } // ~ impl Future<Output = Result<V, E>>, consider the field
                        // accepts type V
}) -> impl Future<Output = Result<StructName, E>>

```

## Contributions

Contributions as always welcome, before contributing please checkout
the issues section, for any previous development that might be done.
For, any improvement, features, bugfixes please create a issue first to track it.

## Code Guidelines

Any code that is being added should pass, `+nightly fmt` formatting checks, clippy checks, and tests.
