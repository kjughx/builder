use proc_macro::TokenStream;
use quote::{quote, quote_spanned};
use syn::{
    self, Field, Ident, ItemStruct, Token, parse::Parse, parse_macro_input, spanned::Spanned,
};

#[derive(Default)]
struct Params {
    builder_name: Option<Ident>,
}

impl Parse for Params {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut this = Self::default();
        while !input.is_empty() {
            let ident = input.parse::<Ident>()?;
            match ident.to_string().as_str() {
                "name" => {
                    input.parse::<Token!(=)>()?;
                    let name = input.parse::<Ident>()?.to_string();

                    this.builder_name = Some(Ident::new(&name, proc_macro2::Span::call_site()));
                }
                _ => {}
            }
        }

        Ok(this)
    }
}

#[proc_macro_attribute]
pub fn builder(attr: TokenStream, item: TokenStream) -> TokenStream {
    let item = parse_macro_input!(item as ItemStruct);
    let original = item.clone();
    let name = item.ident;

    let params = parse_macro_input!(attr as Params);
    let builder_name = params.builder_name.unwrap_or(Ident::new(
        &format!("{name}Builder"),
        proc_macro2::Span::call_site(),
    ));

    let vis = item.vis;
    let mut fields = vec![];
    let mut setters = vec![];
    let mut builders = vec![];

    for Field { vis, ident, ty, .. } in item.fields {
        fields.push(quote! {
            #ident: Option<#ty>,
        });

        setters.push(quote! {
            #vis fn #ident(self, t: #ty) -> Self {
                let mut this = self;
                this.#ident = Some(t);
                this
            }
        });

        builders.push(quote! {
            if let Some(t) = self.#ident {
                this.#ident = t;
            }
        });
    }

    TokenStream::from(quote_spanned! {
        original.span() =>
        #original
        impl #name {
            #vis fn builder() -> #builder_name {
                #builder_name::new()
            }
        }

        #[derive(Default)]
        #vis struct #builder_name {
            #(#fields)*
        }

        impl #builder_name {
            fn new() -> Self {
                Self::default()
            }

            #(#setters)*

            fn build(self) -> #name {
                let mut this = #name::default();
                #(#builders)*

                this
            }
        }
    })
}
