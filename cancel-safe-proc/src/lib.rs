extern crate proc_macro;
use proc_macro::TokenStream;
use proc_macro2::Span;

use quote::quote;
use syn::parse::Parse;
use syn::{parse_macro_input, parse_quote, Block, Signature, Token};

#[proc_macro_attribute]
pub fn cancel_safe(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut input = parse_macro_input!(item as Input);
    input.transform();

    input.into()
}

enum Input {
    FreeStandingFn(Signature, Block),
}

impl Parse for Input {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let sig = input.parse::<Signature>()?;
        let block = input.parse::<Block>()?;

        Ok(Self::FreeStandingFn(sig, block))
    }
}

impl Input {
    fn transform(&mut self) {
        match self {
            Input::FreeStandingFn(s, _) => desugar(s),
        }
    }
}

impl From<Input> for TokenStream {
    fn from(value: Input) -> Self {
        let x = match value {
            Input::FreeStandingFn(s, b) => {
                let stmts = b.stmts;

                quote! {
                    #s
                    {
                        let fut = async move {
                            #(#stmts)*
                        };

                        ::cancel_safe::Safe::new(fut)
                    }
                }
            }
        };

        x.into()
    }
}

// async fn<'a, 'b, T, U>foo(a: &'a: T, b: &'b U) -> R
// into
// fn<'a, 'b, T, U>foo(a: &'a T, b: &'b U) -> impl Future<Output = R>
fn desugar(sig: &mut Signature) {
    use syn::ReturnType;

    sig.fn_token.span = sig.asyncness.take().unwrap().span;
    // No longer async
    sig.asyncness = None;

    let (rarrow, ret) = match &sig.output {
        ReturnType::Default => (Token![->](Span::call_site()), quote!(())),
        ReturnType::Type(rarrow, ret) => (*rarrow, quote!(#ret)),
    };

    sig.output = parse_quote!(#rarrow impl ::cancel_safe::CancelSafe<Output = #ret>);
}
