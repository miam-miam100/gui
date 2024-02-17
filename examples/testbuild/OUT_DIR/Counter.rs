#[allow(clippy::suspicious_else_formatting)]
mod gen {
    use super::__private_CompStruct as CompStruct;
    use gui::gui_core::vello::SceneBuilder;
    use gui::gui_core::glazier::kurbo::Rect;
    use gui::gui_core::widget::{
        Widget, WidgetID, RenderHandle, ResizeHandle, EventHandle, UpdateHandle,
        WidgetEvent, Handle,
    };
    use gui::gui_core::{
        Component, LayoutConstraints, Size, ToComponent, ToHandler, Update, Variable,
    };
    enum WidgetSet0 {
        W0(::gui::gui_widget::Text),
        W1(::gui::gui_widget::Button<IncrementBtn, CompStruct, ::gui::gui_widget::Text>),
        W2(::gui::gui_widget::Button<DecrementBtn, CompStruct, ::gui::gui_widget::Text>),
    }
    impl WidgetSet0 {
        pub fn w0(&mut self) -> &mut ::gui::gui_widget::Text {
            if let WidgetSet0::W0(val) = self {
                val
            } else {
                panic!("Incorrect wrapped type.")
            }
        }
        pub fn w1(
            &mut self,
        ) -> &mut ::gui::gui_widget::Button<
            IncrementBtn,
            CompStruct,
            ::gui::gui_widget::Text,
        > {
            if let WidgetSet0::W1(val) = self {
                val
            } else {
                panic!("Incorrect wrapped type.")
            }
        }
        pub fn w2(
            &mut self,
        ) -> &mut ::gui::gui_widget::Button<
            DecrementBtn,
            CompStruct,
            ::gui::gui_widget::Text,
        > {
            if let WidgetSet0::W2(val) = self {
                val
            } else {
                panic!("Incorrect wrapped type.")
            }
        }
    }
    impl Widget<CompStruct> for WidgetSet0 {
        fn id(&self) -> WidgetID {
            match self {
                WidgetSet0::W0(_) => WidgetID::new(0u32, 1u32),
                WidgetSet0::W1(_) => WidgetID::new(0u32, 2u32),
                WidgetSet0::W2(_) => WidgetID::new(0u32, 4u32),
            }
        }
        fn render(
            &mut self,
            scene: &mut SceneBuilder,
            handle: &mut RenderHandle<CompStruct>,
        ) {
            match self {
                WidgetSet0::W0(w) => {
                    <::gui::gui_widget::Text as Widget<
                        CompStruct,
                    >>::render(w, scene, handle)
                }
                WidgetSet0::W1(w) => {
                    <::gui::gui_widget::Button<
                        IncrementBtn,
                        CompStruct,
                        ::gui::gui_widget::Text,
                    > as Widget<CompStruct>>::render(w, scene, handle)
                }
                WidgetSet0::W2(w) => {
                    <::gui::gui_widget::Button<
                        DecrementBtn,
                        CompStruct,
                        ::gui::gui_widget::Text,
                    > as Widget<CompStruct>>::render(w, scene, handle)
                }
            }
        }
        fn resize(
            &mut self,
            constraints: LayoutConstraints,
            handle: &mut ResizeHandle<CompStruct>,
        ) -> Size {
            match self {
                WidgetSet0::W0(w) => {
                    <::gui::gui_widget::Text as Widget<
                        CompStruct,
                    >>::resize(w, constraints, handle)
                }
                WidgetSet0::W1(w) => {
                    <::gui::gui_widget::Button<
                        IncrementBtn,
                        CompStruct,
                        ::gui::gui_widget::Text,
                    > as Widget<CompStruct>>::resize(w, constraints, handle)
                }
                WidgetSet0::W2(w) => {
                    <::gui::gui_widget::Button<
                        DecrementBtn,
                        CompStruct,
                        ::gui::gui_widget::Text,
                    > as Widget<CompStruct>>::resize(w, constraints, handle)
                }
            }
        }
        fn event(&mut self, event: WidgetEvent, handle: &mut EventHandle<CompStruct>) {
            match self {
                WidgetSet0::W0(w) => {
                    <::gui::gui_widget::Text as Widget<
                        CompStruct,
                    >>::event(w, event, handle)
                }
                WidgetSet0::W1(w) => {
                    <::gui::gui_widget::Button<
                        IncrementBtn,
                        CompStruct,
                        ::gui::gui_widget::Text,
                    > as Widget<CompStruct>>::event(w, event, handle)
                }
                WidgetSet0::W2(w) => {
                    <::gui::gui_widget::Button<
                        DecrementBtn,
                        CompStruct,
                        ::gui::gui_widget::Text,
                    > as Widget<CompStruct>>::event(w, event, handle)
                }
            }
        }
    }
    use gui::{FluentBundle, FluentArgs, FluentResource};
    use std::borrow::Cow;
    fn get_bundle_message<'a>(
        message: &'a str,
        args: Option<&'a FluentArgs<'_>>,
    ) -> Cow<'a, str> {
        use std::sync::OnceLock;
        use gui::langid;
        static BUNDLE: OnceLock<FluentBundle<FluentResource>> = OnceLock::new();
        const FTL_STRING: &str = include_str!(concat!(env!("OUT_DIR"), "/Counter.ftl"));
        let mut errors = vec![];
        let bundle = BUNDLE
            .get_or_init(|| {
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
    #[allow(non_camel_case_types)]
    pub(crate) struct count;
    impl Variable for count {
        type VarType = u32;
    }
    #[allow(non_camel_case_types)]
    pub(crate) struct disabled_decrement;
    impl Variable for disabled_decrement {
        type VarType = bool;
    }
    pub(crate) struct IncrementBtn;
    impl ToHandler for IncrementBtn {
        type BaseHandler = CompStruct;
    }
    pub(crate) struct DecrementBtn;
    impl ToHandler for DecrementBtn {
        type BaseHandler = CompStruct;
    }
    #[allow(non_snake_case)]
    pub struct CounterHolder {
        comp_struct: CompStruct,
        widget: ::gui::gui_widget::HVStack<CompStruct, WidgetSet0>,
        Counter_Count_text: FluentArgs<'static>,
    }
    #[automatically_derived]
    impl ToComponent for CompStruct {
        type Component = CounterHolder;
        fn to_component_holder(self) -> Self::Component {
            CounterHolder {
                widget: ::gui::gui_widget::HVStack::new_vertical(
                    WidgetID::new(0u32, 0u32),
                    40f32,
                    vec![
                        WidgetSet0::W0(::gui::gui_widget::Text::new(WidgetID::new(0u32,
                        1u32), ::gui::gui_core::Colour::rgba8(33u8, 37u8, 41u8, 255u8),
                        40f32)),
                        WidgetSet0::W1(::gui::gui_widget::Button::new(WidgetID::new(0u32,
                        2u32), ::gui::gui_core::Colour::rgba8(255u8, 255u8, 255u8,
                        255u8), ::gui::gui_core::Colour::rgba8(241u8, 243u8, 245u8,
                        255u8), ::gui::gui_core::Colour::rgba8(248u8, 249u8, 250u8,
                        255u8), ::gui::gui_core::Colour::rgba8(248u8, 249u8, 250u8,
                        255u8), ::gui::gui_core::Colour::rgba8(206u8, 212u8, 218u8,
                        255u8), false, ::gui::gui_widget::Text::new(WidgetID::new(0u32,
                        3u32), ::gui::gui_core::Colour::rgba8(33u8, 37u8, 41u8, 255u8),
                        40f32))),
                        WidgetSet0::W2(::gui::gui_widget::Button::new(WidgetID::new(0u32,
                        4u32), ::gui::gui_core::Colour::rgba8(255u8, 255u8, 255u8,
                        255u8), ::gui::gui_core::Colour::rgba8(241u8, 243u8, 245u8,
                        255u8), ::gui::gui_core::Colour::rgba8(248u8, 249u8, 250u8,
                        255u8), ::gui::gui_core::Colour::rgba8(248u8, 249u8, 250u8,
                        255u8), ::gui::gui_core::Colour::rgba8(206u8, 212u8, 218u8,
                        255u8), false, ::gui::gui_widget::Text::new(WidgetID::new(0u32,
                        5u32), ::gui::gui_core::Colour::rgba8(33u8, 37u8, 41u8, 255u8),
                        40f32)))
                    ],
                ),
                comp_struct: self,
                Counter_Count_text: FluentArgs::new(),
            }
        }
        fn largest_id(&self) -> WidgetID {
            WidgetID::new(0u32, 5u32)
        }
        fn get_parent(&self, id: WidgetID) -> Option<WidgetID> {
            match (id.component_id(), id.widget_id()) {
                (0u32, 3u32) => Some(WidgetID::new(0u32, 2u32)),
                (0u32, 5u32) => Some(WidgetID::new(0u32, 4u32)),
                (0u32, 1u32) | (0u32, 2u32) | (0u32, 4u32) => {
                    Some(WidgetID::new(0u32, 0u32))
                }
                _ => None,
            }
        }
    }
    #[automatically_derived]
    impl Component for CounterHolder {
        fn render<'a>(
            &mut self,
            mut scene: SceneBuilder,
            handle: &'a mut Handle,
            global_positions: &'a mut [Rect],
            active_widget: &'a mut Option<WidgetID>,
            hovered_widgets: &'a [WidgetID],
        ) -> bool {
            let mut render_handle = RenderHandle::new(
                handle,
                global_positions,
                active_widget,
                hovered_widgets,
                &mut self.comp_struct,
            );
            self.widget.render(&mut scene, &mut render_handle);
            render_handle.unwrap()
        }
        fn update_vars<'a>(
            &mut self,
            force_update: bool,
            handle: &'a mut Handle,
            global_positions: &'a [Rect],
        ) -> bool {
            let mut update_handle = UpdateHandle::new(handle, global_positions);
            let handle_ref = &mut update_handle;
            let mut text = false;
            if force_update
                || <CompStruct as Update<count>>::is_updated(&self.comp_struct)
            {
                let value = <CompStruct as Update<count>>::value(&self.comp_struct);
                text = true;
                self.Counter_Count_text.set("count", value);
            }
            if force_update
                || <CompStruct as Update<
                    disabled_decrement,
                >>::is_updated(&self.comp_struct)
            {
                let value = <CompStruct as Update<
                    disabled_decrement,
                >>::value(&self.comp_struct);
                let widget = &mut self.widget.widgets(2usize).w2();
                widget.set_disabled(value, handle_ref);
            }
            if force_update || text {
                let value = get_bundle_message(
                    "Counter-Count-text",
                    Some(&self.Counter_Count_text),
                );
                let widget = &mut self.widget.widgets(0usize).w0();
                widget.set_text(value, handle_ref);
            }
            if force_update {
                let value = get_bundle_message("Counter-IncrText-text", None);
                let widget = &mut self.widget.widgets(1usize).w1().get_widget();
                widget.set_text(value, handle_ref);
            }
            if force_update {
                let value = get_bundle_message("Counter-DecrText-text", None);
                let widget = &mut self.widget.widgets(2usize).w2().get_widget();
                widget.set_text(value, handle_ref);
            }
            <CompStruct as Update<count>>::reset(&mut self.comp_struct);
            <CompStruct as Update<disabled_decrement>>::reset(&mut self.comp_struct);
            update_handle.unwrap()
        }
        fn resize<'a>(
            &mut self,
            constraints: LayoutConstraints,
            handle: &'a mut Handle,
            local_positions: &'a mut [Rect],
        ) -> Size {
            let mut resize_handle = ResizeHandle::new(
                handle,
                local_positions,
                &mut self.comp_struct,
            );
            self.widget.resize(constraints, &mut resize_handle)
        }
        fn propagate_event<'a>(
            &mut self,
            event: WidgetEvent,
            handle: &'a mut Handle,
            global_positions: &'a [Rect],
            active_widget: &'a mut Option<WidgetID>,
            hovered_widgets: &'a mut Vec<WidgetID>,
        ) -> bool {
            let mut event_handle = EventHandle::new(
                handle,
                global_positions,
                active_widget,
                hovered_widgets,
                &mut self.comp_struct,
            );
            self.widget.event(event, &mut event_handle);
            let (mut resize, events) = event_handle.unwrap();
            for (id, e) in events {
                if self
                    .event(
                        id,
                        e,
                        handle,
                        global_positions,
                        active_widget,
                        hovered_widgets,
                    )
                {
                    resize = true;
                }
            }
            resize
        }
        fn largest_id(&self) -> WidgetID {
            self.comp_struct.largest_id()
        }
        fn get_parent(&self, id: WidgetID) -> Option<WidgetID> {
            self.comp_struct.get_parent(id)
        }
        fn event<'a>(
            &mut self,
            id: WidgetID,
            event: WidgetEvent,
            handle: &'a mut Handle,
            global_positions: &'a [Rect],
            active_widget: &'a mut Option<WidgetID>,
            hovered_widgets: &'a mut Vec<WidgetID>,
        ) -> bool {
            let mut event_handle = EventHandle::new(
                handle,
                global_positions,
                active_widget,
                hovered_widgets,
                &mut self.comp_struct,
            );
            let handle_ref = &mut event_handle;
            match (id.component_id(), id.widget_id()) {
                (0u32, 1u32) => {
                    self.widget.widgets(0usize).w0().event(event, handle_ref);
                }
                (0u32, 3u32) => {
                    self.widget
                        .widgets(1usize)
                        .w1()
                        .get_widget()
                        .event(event, handle_ref);
                }
                (0u32, 2u32) => {
                    self.widget.widgets(1usize).w1().event(event, handle_ref);
                }
                (0u32, 5u32) => {
                    self.widget
                        .widgets(2usize)
                        .w2()
                        .get_widget()
                        .event(event, handle_ref);
                }
                (0u32, 4u32) => {
                    self.widget.widgets(2usize).w2().event(event, handle_ref);
                }
                (0u32, 0u32) => {
                    self.widget.event(event, handle_ref);
                }
                _ => {}
            }
            let (mut resize, events) = event_handle.unwrap();
            for (id, e) in events {
                if self
                    .event(
                        id,
                        e,
                        handle,
                        global_positions,
                        active_widget,
                        hovered_widgets,
                    )
                {
                    resize = true;
                }
            }
            resize
        }
    }
}
