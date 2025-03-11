use proc_macro::TokenStream;
use quote::{quote, quote_spanned};
use syn::{
    self, Expr, Field, Ident, ItemStruct, Token,
    parse::{Parse, ParseStream},
    parse_macro_input,
    spanned::Spanned,
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
                    this.builder_name = Some(input.parse::<Ident>()?);
                }
                _ => {}
            }
        }

        Ok(this)
    }
}

#[derive(Default)]
struct FieldParam {
    default_value: Option<syn::Expr>,
    hidden: bool,
    name: Option<Ident>,
    custom_setter: bool,
}

impl Parse for FieldParam {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut this = Self::default();
        loop {
            let ident = input.parse::<Ident>()?;
            match ident.to_string().as_str() {
                "hidden" => {
                    this.hidden = true;
                }
                "default_value" => {
                    input.parse::<Token!(=)>()?;
                    this.default_value = Some(input.parse::<Expr>()?);
                }
                "name" => {
                    input.parse::<Token!(=)>()?;
                    this.name = Some(input.parse::<Ident>()?);
                }
                "custom_setter" => {
                    this.custom_setter = true;
                }
                _ => {}
            }
            if input.is_empty() {
                break;
            }

            input.parse::<Token!(,)>()?;
        }

        Ok(this)
    }
}

impl FieldParam {
    fn from_attrs(attrs: &mut Vec<syn::Attribute>) -> Option<Self> {
        let mut i: Option<usize> = None;
        let mut this: Option<Self> = None;
        for (_i, attr) in attrs.iter().enumerate() {
            if attr.path.is_ident("build") {
                i = Some(_i);
                this = Some(attr.parse_args::<FieldParam>().unwrap());
                break;
            }
        }

        if let Some(i) = i {
            attrs.remove(i);
        }

        this
    }
}

#[proc_macro_attribute]
pub fn builder(attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut item = parse_macro_input!(item as ItemStruct);
    let name = &item.ident;

    let params = parse_macro_input!(attr as Params);
    let builder_name = params.builder_name.unwrap_or(Ident::new(
        &format!("{name}Builder"),
        proc_macro2::Span::call_site(),
    ));

    let vis = &item.vis;
    let mut fields = vec![];
    let mut setters = vec![];
    let mut builders = vec![];
    let mut defaults = vec![];

    for Field {
        vis,
        ident,
        ty,
        attrs,
        ..
    } in &mut item.fields
    {
        let FieldParam {
            hidden,
            default_value,
            name,
            custom_setter,
            ..
        } = FieldParam::from_attrs(attrs).unwrap_or_default();

        defaults.push(if let Some(default) = default_value {
            quote! {
                #ident: #default,
            }
        } else {
            quote! {
                #ident: #ty::default(),
            }
        });

        if hidden {
            continue;
        }

        fields.push(quote! {
            #ident: Option<#ty>,
        });

        if !custom_setter {
            let setter_name = name.as_ref().or(ident.as_ref());

            setters.push(quote! {
                #vis fn #setter_name(self, t: #ty) -> Self {
                    let mut this = self;
                    this.#ident = Some(t);
                    this
                }
            });
        }

        builders.push(quote! {
            if let Some(t) = self.#ident {
                this.#ident = t;
            }
        });
    }

    TokenStream::from(quote_spanned! {
        item.span() =>
        #item
        impl #name {
            #[allow(dead_code)]
            #vis fn builder() -> #builder_name {
                #builder_name::new()
            }
        }

        #[derive(Default)]
        #vis struct #builder_name {
            #(#fields)*
        }

        impl #builder_name {
            #vis fn new() -> Self {
                Self::default()
            }

            #(#setters)*

            fn build(self) -> #name {
                let mut this = #name {
                    #(#defaults)*
                };

                #(#builders)*

                this
            }
        }
    })
}
