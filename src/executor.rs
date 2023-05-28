use proc_macro2::{Span, TokenStream};
use quote::quote;

use crate::types;

///
/// ## The whole point
/// ---
///
/// Function to generate the token stream for the provided data, in order to allow for the
/// concurrent async execution and struct reconstruction.
///
///
pub(crate) fn concurrent_struct_constructor(
    data: types::OutputStructAbstraction,
    triable_futures: bool,
) -> TokenStream {
    let types::OutputStructAbstraction {
        struct_name,
        fields,
    } = data;

    let ExprContainer {
        resolvable_fields,
        normal_fields,
    } = fields.into();

    let (normal_fields, normal_fields_expr): (Vec<_>, Vec<_>) = normal_fields.into_iter().unzip();

    let (concurrent_code, async_fields) =
        match concurrent_join_expression(resolvable_fields, triable_futures) {
            Some(inner) => inner,
            None => {
                return syn::Error::new(
                    Span::mixed_site(),
                    "No futures provided to resolve. (consider removing the macro)",
                )
                .to_compile_error()
            }
        };

    let struct_code = match triable_futures {
        false => quote! {
            #struct_name {
                #(#async_fields),*,
                #(#normal_fields : #normal_fields_expr),*
            }
        },
        true => quote! {
            Ok(#struct_name {
                #(#async_fields),*,
                #(#normal_fields : #normal_fields_expr),*
            })
        },
    };

    quote! {
        async {
            #concurrent_code

            #struct_code
        }
    }
}

///
/// struct used to separate `Vec<FuturedField>` into fields that are and aren't with async
/// expression
///
struct ExprContainer {
    resolvable_fields: Vec<(syn::Ident, syn::Expr)>,
    normal_fields: Vec<(syn::Ident, syn::Expr)>,
}

impl From<Vec<types::FuturedField>> for ExprContainer {
    fn from(value: Vec<types::FuturedField>) -> Self {
        let (resolvable_fields, normal_fields): (Vec<_>, Vec<_>) = value
            .into_iter()
            .map(|inner| match inner.value {
                types::FieldType::Future(async_expr) => {
                    (Some((inner.name, syn::Expr::Async(async_expr))), None)
                }
                types::FieldType::Normal(expr) => (None, Some((inner.name, expr))),
            })
            .unzip();

        Self {
            resolvable_fields: resolvable_fields.into_iter().flatten().collect(),
            normal_fields: normal_fields.into_iter().flatten().collect(),
        }
    }
}

///
/// Function to generate the `tokio::join!` or `tokio::try_join!` function calls to perform
/// concurrent future resolution
///
fn concurrent_join_expression(
    fields: Vec<(syn::Ident, syn::Expr)>,
    triable_futures: bool,
) -> Option<(proc_macro2::TokenStream, Vec<syn::Ident>)> {
    if fields.is_empty() {
        return None;
    }
    let (field, expr): (Vec<_>, Vec<_>) = fields.into_iter().unzip();

    match triable_futures {
        true => Some((
            quote! {
                let (#(#field),*,) = tokio::try_join!(#(#expr),*)?;
            },
            field,
        )),
        false => Some((
            quote! {
                let (#(#field),*,) = tokio::join!(#(#expr),*);
            },
            field,
        )),
    }
}
