use crate::widget::Widget;
use gui_core::parse::WidgetDeclaration;
use gui_core::widget::WidgetID;
use itertools::Itertools;
use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};
use std::sync::atomic::{AtomicU32, Ordering};

#[derive(Clone, Debug)]
pub struct WidgetSet<'a> {
    pub widgets: Vec<(TokenStream, Widget<'a>)>,
    count: Option<u32>,
}

impl<'a> WidgetSet<'a> {
    pub fn new(
        component_name: &str,
        widgets: Vec<(TokenStream, &'a WidgetDeclaration)>,
        component_id: u32,
    ) -> anyhow::Result<Self> {
        static COUNTER: AtomicU32 = AtomicU32::new(0);

        let widgets = widgets
            .into_iter()
            .map(|(s, w)| Ok((s, Widget::new_inner(component_name, w, component_id)?)))
            .collect::<anyhow::Result<Vec<_>>>()?;

        Ok(Self {
            count: (widgets.len() > 1).then(|| COUNTER.fetch_add(1, Ordering::Relaxed)),
            widgets,
        })
    }

    pub fn gen_widget_type(&self, component_holder: &Ident) -> TokenStream {
        match &self.widgets[..] {
            [(_, child)] => child.gen_widget_type(&component_holder),
            [] => quote!(()),
            _ => {
                let count = self.count.expect("widget set should be created.");
                let ident = format_ident!("WidgetSet{count}");
                quote!(#ident)
            }
        }
    }

    pub fn gen_widget_init(&self) -> TokenStream {
        match &self.widgets[..] {
            [(_, child)] => child.gen_widget_init(),
            [] => quote!(()),
            _ => {
                let count = self.count.expect("widget set should be created.");
                let widget_set = format_ident!("WidgetSet{count}");

                let inits = self
                    .widgets
                    .iter()
                    .map(|(_, child)| child.gen_widget_init());

                let variants = self.widgets.iter().enumerate().map(|(i, _)| {
                    let ident = format_ident!("W{i}");
                    quote!(#widget_set::#ident)
                });

                quote! {
                    #(#variants(#inits)),*
                }
            }
        }
    }

    pub fn gen_widget_set(&self, component_holder: &Ident, stream: &mut TokenStream) {
        if let Some(count) = self.count {
            let widget_set = format_ident!("WidgetSet{count}");

            let variants = self
                .widgets
                .iter()
                .enumerate()
                .map(|(i, _)| format_ident!("W{i}"))
                .collect_vec();

            let func_names = self
                .widgets
                .iter()
                .enumerate()
                .map(|(i, _)| format_ident!("w{i}"));

            let types = self
                .widgets
                .iter()
                .map(|(_, w)| w.gen_widget_type(component_holder))
                .collect_vec();

            let ids = self.widgets.iter().map(|(_, w)| w.id);

            stream.extend(quote! {
                enum #widget_set {
                    #( #variants(#types) ),*
                }

                impl #widget_set {
                    #(
                        pub fn #func_names(&mut self) -> &mut #types {
                            if let #widget_set::#variants(val) = self {
                                val
                            } else {
                                panic!("Incorrect wrapped type.")
                            }
                        }
                    )*
                }

                impl Widget<#component_holder> for #widget_set {
                    fn id(&self) -> WidgetID {
                        match self {
                            #( #widget_set::#variants(_) => #ids ),*
                        }
                    }

                     fn render(&mut self, scene: &mut SceneBuilder, handle: &mut RenderHandle<#component_holder>) {
                        match self {
                            #( #widget_set::#variants(w) => <#types as Widget<#component_holder>>::render(w, scene, handle) ),*
                        }
                    }

                    fn resize(&mut self, constraints: LayoutConstraints, handle: &mut ResizeHandle<#component_holder>) -> Size {
                        match self {
                            #( #widget_set::#variants(w) => <#types as Widget<#component_holder>>::resize(w, constraints, handle) ),*
                        }
                    }

                    fn event(&mut self, event: WidgetEvent, handle: &mut EventHandle<#component_holder>) {
                        match self {
                            #( #widget_set::#variants(w) => <#types as Widget<#component_holder>>::event(w, event, handle) ),*
                        }
                    }

                }
            });
        }

        for (_, w) in &self.widgets {
            w.gen_widget_set(component_holder, stream)
        }
    }

    pub fn gen_widget_gets<'b>(
        &'b self,
        stream: &'b TokenStream,
    ) -> impl Iterator<Item = (TokenStream, &Widget)> + '_ {
        self.widgets
            .iter()
            .enumerate()
            .map(move |(i, (get_widget, w))| {
                let mut s = stream.clone();
                s.extend(get_widget.clone());
                if self.count.is_some() {
                    let func = format_ident!("w{i}");
                    s.extend(quote!( .#func() ));
                }
                (s, w)
            })
    }

    pub fn largest_id(&self) -> Option<WidgetID> {
        self.widgets
            .iter()
            .map(|(_, w)| w.id)
            .max_by_key(|i| i.widget_id())
    }
}
