//! `#[timed]` attribute for lind-perf. See the `lind-perf` crate docs.

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Expr, ItemFn, LitStr};

/// Time every call of the annotated function.
///
/// ```ignore
/// #[lind_perf::timed]                         // name = "<module_path>::<fn>"
/// fn foo() {}
///
/// #[lind_perf::timed(key = syscall_num)]      // one bucket per key value
/// pub fn make_syscall(self_cageid: u64, syscall_num: u64, /* ... */) -> i32 { /* ... */ }
///
/// #[lind_perf::timed(name = "custom::name")]
/// fn bar() {}
///
/// #[lind_perf::timed(cond = syscall_num == 107)] // only time calls where this is true
/// pub fn make_syscall(self_cageid: u64, syscall_num: u64, /* ... */) -> i32 { /* ... */ }
/// ```
///
/// `key` is any expression convertible with `as u64`, `cond` any `bool` expression. Both are
/// evaluated on entry, so they can name the function's arguments.
///
/// Expands to `let _guard = lind_perf::scope!(...); <original body>`. When lind-perf is built
/// without its `enabled` feature, `scope!` expands to a zero-sized value and the key expression
/// is never evaluated.
#[proc_macro_attribute]
pub fn timed(attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut name: Option<LitStr> = None;
    let mut key: Option<Expr> = None;
    let mut cond: Option<Expr> = None;

    let parser = syn::meta::parser(|meta| {
        if meta.path.is_ident("name") {
            name = Some(meta.value()?.parse()?);
            Ok(())
        } else if meta.path.is_ident("key") {
            key = Some(meta.value()?.parse()?);
            Ok(())
        } else if meta.path.is_ident("cond") {
            cond = Some(meta.value()?.parse()?);
            Ok(())
        } else {
            Err(meta.error("expected `name = \"...\"`, `key = <expr>` or `cond = <bool expr>`"))
        }
    });
    parse_macro_input!(attr with parser);

    let ItemFn {
        attrs,
        vis,
        sig,
        block,
    } = parse_macro_input!(item as ItemFn);

    if let Some(t) = &sig.asyncness {
        return syn::Error::new_spanned(t, "#[timed] does not support async fn")
            .to_compile_error()
            .into();
    }
    if let Some(t) = &sig.constness {
        return syn::Error::new_spanned(t, "#[timed] cannot be used on const fn")
            .to_compile_error()
            .into();
    }

    let ident = &sig.ident;
    let name = match name {
        Some(lit) => quote!(#lit),
        None => quote!(::core::concat!(::core::module_path!(), "::", ::core::stringify!(#ident))),
    };
    let key = key.map(|k| quote!(, key = #k));
    let cond = cond.map(|c| quote!(, cond = #c));
    let scope = quote!(::lind_perf::scope!(#name #key #cond));

    quote!(
        #(#attrs)*
        #vis #sig {
            let __lind_perf_guard = #scope;
            #block
        }
    )
    .into()
}
