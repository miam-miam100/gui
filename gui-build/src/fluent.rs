use gui_core::parse::fluent::Fluent;
use gui_core::parse::{ComponentDeclaration, NormalVariableDeclaration};
use proc_macro2::{Ident, Span, TokenStream};
use quote::{format_ident, quote};
use std::collections::HashMap;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FluentIdent<'a> {
    pub fluent: &'a Fluent,
    pub ident: Ident,
    pub name: String,
    pub property: &'static str,
    pub property_ident: Ident,
}

impl<'a> FluentIdent<'a> {
    pub fn new(
        property: &'static str,
        fluent: &'a Fluent,
        component_name: &str,
        widget_name: Option<&str>,
        widget_type_name: &str,
    ) -> Self {
        let fluent_widget_name = widget_name.unwrap_or(widget_type_name);
        Self {
            fluent,
            property,
            ident: format_ident!("{component_name}_{fluent_widget_name}_{property}"),
            name: format!("{component_name}-{fluent_widget_name}-{property}"),
            property_ident: Ident::new(property, Span::call_site()),
        }
    }
}

pub fn gen_var_update(
    component: &ComponentDeclaration,
    normal_variables: &[&NormalVariableDeclaration],
    var_to_fluent: HashMap<&str, Vec<&FluentIdent>>,
    widget_vars: HashMap<&str, Vec<&'static str>>,
) -> TokenStream {
    normal_variables
        .iter()
        .map(|v| {
            let var_name = Ident::new(&v.name, Span::call_site());
            let widget_ident = Ident::new("widget", Span::call_site());
            let value_ident = Ident::new("value", Span::call_site());
            let string_var_name = &v.name;
            let mut update_var_props = TokenStream::new();

            for prop in widget_vars
                .get(v.name.as_str())
                .into_iter()
                .flat_map(|props| props.iter())
            {
                component
                    .child
                    .widget
                    .on_property_update(prop, &widget_ident, &value_ident, &mut update_var_props);
            }

            let update_fluent_args = var_to_fluent
                .get(v.name.as_str())
                .into_iter()
                .flat_map(|fluents| fluents.iter())
                .map(|fluent| {
                    let fluent_ident = &fluent.ident;
                    let prop = Ident::new(fluent.property, Span::call_site());
                    quote! {
                        #prop = true;
                        self.#fluent_ident.set(#string_var_name, #value_ident);
                    }
                });

            quote! {
                if force_update || <CompStruct as Update<#var_name>>::is_updated(&self.comp_struct) {
                    let #value_ident = <CompStruct as Update<#var_name>>::value(&self.comp_struct);
                    let #widget_ident = &mut self.widget;
                    #update_var_props
                    #( #update_fluent_args )*
                }
            }
        })
        .collect()
}

pub fn gen_bundle_function() -> TokenStream {
    quote! {
        use gui::{FluentBundle, FluentArgs, FluentResource};
        use std::borrow::Cow;

        fn get_bundle_message<'a>(message: &'a str, args: Option<&'a FluentArgs<'_>>) -> Cow<'a, str> {
            use std::sync::OnceLock;
            use gui::langid;

            static BUNDLE: OnceLock<FluentBundle<FluentResource>> = OnceLock::new();
            const FTL_STRING: &str = include_str!(concat!(env!("OUT_DIR"), "/Counter.ftl"));
            let mut errors = vec![];
            let bundle = BUNDLE.get_or_init(|| {
                let mut bundle = FluentBundle::new_concurrent(vec![langid!("en-GB")]);
                let resource = FluentResource::try_new(FTL_STRING.to_string())
                    .expect("FTL string is valid.");
                bundle.add_resource(resource).expect("No identifiers are overlapping.");
                bundle
            });
            let message = bundle.get_message(message).expect("Message exists.");
            let pattern = message.value().expect("Value exists.");
            bundle.format_pattern(pattern, args, &mut errors)
        }
    }
}
