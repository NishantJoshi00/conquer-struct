//!
//! # Conquer Struct
//!
//!
//! Introducing `conquer_struct`, as the name suggest it will
//! conquer concurreny for you and you can avoid writing any boilerplate
//! code. Let's take the same example as above, look at how to make it concurrent
//!
//! ```rust
//!
//! async fn get_contact_info(contact_id: usize) -> Contact {
//!     conquer_future!(Contact {
//!         name: async { get_contact_name(contact_id).await },
//!         number: async { get_contact_number(contact_id).await },
//!         is_verified: true
//!     }).await
//! }
//!
//! ```
//!
//! Done! Wasn't is simple, with minimal change our code is transformed into
//! concurrent code.
//!
//! ## Usage
//!
//! `conquer_struct` provides use with 2 macros.
//!
//! [`conquer_future!`] this macro resolves the futures provided inside the struct.
//! the macro has a rough signature:
//!
//! ```rust
//!
//! conquer_future!(StructName {
//!     value1: T,
//!     value2: async { .. } // ~ impl Future<Output = V>, considering the field
//!                         // accepts type V
//! }) -> impl Future<Output = StructName>
//!
//! ```
//!
//! [`try_conquer_future!`] this macro resolves the futures provided inside the struct,
//! as well as consider the result of the future that are present inside.
//!
//! ```rust
//!
//! try_conquer_future!(StructName {
//!     value1: T,
//!     value2: async { .. } // ~ impl Future<Output = Result<V, E>>, consider the field
//!                         // accepts type V
//! }) -> impl Future<Output = Result<StructName, E>>
//!
//! ```
//!
//!
//! ### Support
//!
//! Supported runtime for concurrent execution
//! - `tokio`
//!

mod executor;
mod types;

use proc_macro::TokenStream;
use syn::parse_macro_input;

///
/// Macro function to use for resolving the futures concurrently, for fields of struct and
/// returning the struct itself.
///
/// ## Example
///  Following is a usecase where we are trying to get the some details from some async functions,
///  but don't wish to write all the boilerplate code for it. `conquer_future` internally uses
///  `tokio::join` to achieve concurrent execution.
///
/// ```
/// #[derive(PartialEq, Debug)]
/// struct Contact {
///     name: String,
///     phone_no: String,
///     address: String,
///
/// }
///
/// #[tokio::main]
/// async fn main() {
///     let contact = conquer_struct::conquer_future!(Contact {
///         name: "John Doe".to_string(),
///         phone_no: async { get_contact_no().await },
///         address: async { get_address().await }
///     }).await;
///
///     assert_eq!(contact, Contact { name: "John Doe".to_string(), phone_no:
///     "1234567890".to_string(), address: "221B Baker Street".to_string() })
/// }
/// async fn get_contact_no() -> String { "1234567890".to_string() }
/// async fn get_address() -> String { "221B Baker Street".to_string() }
/// ```
/// ---
///
/// In case you haven't provided any async field to the struct within the macro, the macro will
/// throw a compile time error, not to use the macro.
/// As the macro internally creates a new `async {  }` block to store the code that is generated,
/// in case of no async execution this creates a redundant future.
///  
/// ```compile_fail
/// #[derive(PartialEq, Debug)]
/// struct Contact {
///     name: String,
///     phone_no: String,
///     address: String,
///
/// }
///
/// #[tokio::main]
/// async fn main() {
///     let contact = conquer_struct::conquer_future!(Contact {
///         name: "John Doe".to_string(),
///         phone_no: "1234567890".to_string(),
///         address: "221B Baker Street".to_string(),
///     }).await;
/// }
/// ```
///
///
#[proc_macro]
pub fn conquer_future(input: TokenStream) -> TokenStream {
    let container = parse_macro_input!(input as types::OutputStructAbstraction);

    executor::concurrent_struct_constructor(container, false).into()
}

///
/// Macro function to use for resolving the futures concurrently, for fields of struct and
/// returning the struct itself. Additionally, `try_conquer_future` resolve the `Result<T, E>`
/// which is expected to be yielded by the future.
///
/// note: the `E` of the result type should match the `E` on the current scope. Internally, `?` is
/// used to get the fields.
///
///
/// # Example
/// similar to `conquer_future`, only difference being it uses `tokio::try_join` internally to try
/// and resolve the futures and get the `Ok(...)` of the `Result<T, E>` returned by the futures
/// ```
/// #[derive(PartialEq, Debug)]
/// struct Contact {
///     name: String,
///     phone_no: String,
///     address: String,
///
/// }
///
/// #[tokio::main]
/// async fn main() -> Result<(), ()> {
///     let contact = conquer_struct::try_conquer_future!(Contact {
///         name: "John Doe".to_string(),
///         phone_no: async { get_contact_no().await },
///         address: async { get_address().await }
///     }).await?;
///
///     assert_eq!(contact, Contact { name: "John Doe".to_string(), phone_no:
///     "1234567890".to_string(), address: "221B Baker Street".to_string() });
///     Ok(())
/// }
/// async fn get_contact_no() -> Result<String, ()> { Ok("1234567890".to_string()) }
/// async fn get_address() -> Result<String, ()> { Ok("221B Baker Street".to_string()) }
/// ```
///
/// ---
///
///
#[proc_macro]
pub fn try_conquer_future(input: TokenStream) -> TokenStream {
    let container = parse_macro_input!(input as types::OutputStructAbstraction);

    executor::concurrent_struct_constructor(container, true).into()
}
