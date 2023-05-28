use syn::{braced, parse::Parse, spanned::Spanned, Expr, ExprAsync, Ident, Token};

#[derive(Debug)]
pub(crate) struct OutputStructAbstraction {
    pub struct_name: Ident,
    pub fields: Vec<FuturedField>,
}

#[derive(Debug)]
pub(crate) struct FuturedField {
    pub name: Ident,
    pub value: FieldType,
}

pub(crate) enum FieldType {
    Future(ExprAsync), // This expression will be a async expression which includes `async {  }`
    Normal(Expr),
}

impl std::fmt::Debug for FieldType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Future(arg0) => f.debug_tuple("Future").field(&arg0.span()).finish(),
            Self::Normal(arg0) => f.debug_tuple("Normal").field(&arg0.span()).finish(),
        }
    }
}

///
/// ## Parse syntax
/// Parse the following syntax, which will align with the real
///
/// `conquer_future` macro
///
/// ```ignore
/// conquer_future!( $struct_name {
///     $field : async { $expr_inner } | $expr, // here it could either be a async expression, or a
///     ...      ^^^^^^^^^^^^^^^^^^^^^- $expr  // normal expression
/// } )
/// ```
///
///
/// ## Example
/// ```ignore
///
/// struct DataPacket {
///     name: String,
///     user_id: usize
/// }
///
/// async fn get_user_id() -> usize;
///
/// let output = conquer_future!(DataPacket {
///     name: async { get_user_id().await },
///     user_id: 123
/// }).await;
///
///
/// ```
///
///
///
impl Parse for OutputStructAbstraction {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        // placeholder for the content inside the braces
        let content;

        let struct_name = input.parse()?;

        let _braces = braced!(content in input);

        let punk_fields = content.parse_terminated(FuturedField::parse, Token![,])?;
        let fields = punk_fields.into_iter().collect();

        Ok(Self {
            struct_name,
            fields,
        })
    }
}

///
/// Parse the following syntax, which will align with the real
/// ``ignore
///    $name : $value
/// ```
///
impl Parse for FuturedField {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let name = input.parse()?;
        input.parse::<Token![:]>()?;
        let value = input.parse()?;

        Ok(Self { name, value })
    }
}

///
/// Parse the following syntax, which will align with the real
///
/// either
/// > ```ignore
/// >   async { $expr_inner }
/// >   ^^^^^^^^^^^^^^^^^^^^^---- $expr  
/// >
/// > ```
/// or
/// > ```ignore
/// > $expr
/// >
/// > ```
///
///
///
impl Parse for FieldType {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let value = input.parse::<Expr>()?;
        Ok(match value {
            Expr::Async(expression) => Self::Future(expression),
            expression => Self::Normal(expression),
        })
    }
}

impl From<FieldType> for Expr {
    fn from(value: FieldType) -> Self {
        match value {
            FieldType::Future(expr) => Expr::Async(expr),
            FieldType::Normal(expr) => expr,
        }
    }
}
