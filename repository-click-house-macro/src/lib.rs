extern crate self as repository_click_house_macro;
extern crate proc_macro;

use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn;

#[proc_macro_derive(Event, attributes(event_type))]
pub fn event_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_event_macro(&ast)
}

fn impl_event_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;

    let fields = match &ast.data {
        syn::Data::Struct(syn::DataStruct { fields: syn::Fields::Named(fields), .. }) => &fields.named,
        _ => panic!("#[derive(EventBuilder)] only supports structs with named fields"),
    };
    let field_names: Vec<_> =
        fields
            .iter()
            .filter_map(|f| {
                let ident = f.ident.as_ref().unwrap();
                if ident == "client_id" || ident == "uuid" {
                    None
                } else {
                    Some(ident)
                }
            })
            .collect();

    let mut event_type_variant: Option<syn::LitStr> = None;

    for attr in &ast.attrs {
        if attr.path().is_ident("event_type") {
            let lit: syn::LitStr = attr.parse_args().expect("expected #[event_type(\"Foo\")]");
            event_type_variant = Some(lit);
        }
    }

    let event_type_expr = event_type_variant
        .map(|lit| {
            let variant_ident = syn::Ident::new(&lit.value(), lit.span());
            quote!(EventType::#variant_ident)
        })
        .unwrap_or_else(|| quote!(None));

    let name = &ast.ident;
    let stream = quote! {
        impl EventBuilder for #name {
            fn into_inner(self) -> Event {
                Event {
                    event_id: uuid::Uuid::new_v4(),
                    created_at: time::OffsetDateTime::now_utc(),
                    event_type: #event_type_expr,
                    client_id: self.client_id.unwrap(),
                    uuid: self.uuid.unwrap(),
                    #(
                        #field_names: self.#field_names,
                    )*
                    ..Default::default()
                }
            }
        }
    };
    stream.into()
}
