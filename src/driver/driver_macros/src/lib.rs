#[allow(dead_code)]
mod sled {
    struct Sled;
    struct BufferContainer;
    struct Filters;
    struct TimeInfo;
}

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, parse_quote, FnArg, ItemFn, PatType, Type, TypePath};

/// Autofills parameters missing for a startup command in a driver at compile time.
/// 
/// This saves you from having to write lengthy method signatures and bringing structs into scope that you won't use.
///
/// For example:
/// ```rust, no-run
/// #[startup_commands]
/// fn startup(buffers: &mut BufferContainer) -> SledResult {
///     //--snip--//
/// }
/// ```
/// Gets turned into:
/// ```rust, no-run
/// fn startup(
///     _: &mut Sled,
///     buffers: &mut BufferContainer,
///     _: &mut Filters
/// ) -> SledResult {
///     //--snip--//
/// }
/// ```
#[proc_macro_attribute]
pub fn startup_commands(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let params_template = vec![
        parse_quote!(&mut sled::Sled),
        parse_quote!(&mut sled::BufferContainer),
        parse_quote!(&mut sled::Filters),
    ];

    auto_fill_params(item, params_template)
}

/// Autofills parameters missing for a draw command in a driver at compile time.
/// 
/// This saves you from having to write lengthy method signatures and bringing structs into scope that you won't use.
///
/// For example:
/// ```rust, no-run
/// #[draw_commands]
/// fn draw(sled: &mut Sled, time: &TimeInfo) -> SledResult {
///     //--snip--//
/// }
/// ```
/// Gets turned into:
/// ```rust, no-run
/// fn draw(
///     sled: &mut Sled,
///     _: &BufferContainer,
///     _: &Filters,
///     time: &TimeInfo
/// ) -> SledResult {
///     //--snip--//
/// }
/// ```
#[proc_macro_attribute]
pub fn draw_commands(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let params_template = vec![
        parse_quote!(&mut sled::Sled),
        parse_quote!(&sled::BufferContainer),
        parse_quote!(&sled::Filters),
        parse_quote!(&sled::TimeInfo),
    ];

    auto_fill_params(item, params_template)
}


/// Autofills parameters missing for a compute command in a driver at compile time.
/// 
/// This saves you from having to write lengthy method signatures and bringing structs into scope that you won't use.
///
/// For example:
/// ```rust, no-run
/// #[draw_commands]
/// fn compute(filters: &mut Filters, time: &TimeInfo) -> SledResult {
///     //--snip--//
/// }
/// ```
/// Gets turned into:
/// ```rust, no-run
/// fn compute(
///     _: &Sled,
///     _: &mut BufferContainer,
///     filters: &mut Filters,
///     time: &TimeInfo
/// ) -> SledResult {
///     //--snip--//
/// }
/// ```
#[proc_macro_attribute]
pub fn compute_commands(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let params_template = vec![
        parse_quote!(&sled::Sled),
        parse_quote!(&mut sled::BufferContainer),
        parse_quote!(&mut sled::Filters),
        parse_quote!(&sled::TimeInfo),
    ];

    auto_fill_params(item, params_template)
}

fn auto_fill_params(item: TokenStream, params_template: Vec<Type>) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);
    let mut original_params = input.sig.inputs.clone();

    let mut param_index = 0;
    for ty in params_template {
        let exists = original_params.iter().any(|arg| match arg {
            FnArg::Typed(PatType { ty: param_type, .. }) => types_are_equal(&*param_type, &ty),
            _ => false,
        });

        if !exists {
            let new_param: FnArg = syn::parse_quote! {
                _: #ty
            };
            original_params.insert(param_index, new_param);
        }
        param_index += 1;
    }

    let param_tokens = original_params.iter().map(|param| {
        quote! { #param }
    });

    // Rebuild the function with the new parameters and body
    let fn_name = &input.sig.ident;
    let fn_body = &input.block;
    let fn_return = &input.sig.output;

    let expanded = quote! {
        pub fn #fn_name(#(#param_tokens),*) #fn_return #fn_body
    };

    TokenStream::from(expanded)
}

fn types_are_equal(t1: &Type, t2: &Type) -> bool {
    match (t1, t2) {
        (Type::Reference(ref1), Type::Reference(ref2)) => {
            ref1.mutability == ref2.mutability && types_are_equal(&ref1.elem, &ref2.elem)
        }
        (Type::Path(TypePath { path: p1, .. }), Type::Path(TypePath { path: p2, .. })) => {
            paths_are_equal(p1, p2)
        }
        _ => false,
    }
}

/// Helper function to compare two paths, including module paths (e.g., `some_module::Point`)
fn paths_are_equal(p1: &syn::Path, p2: &syn::Path) -> bool {
    p1.segments.iter().last().unwrap().ident == p2.segments.iter().last().unwrap().ident
}
