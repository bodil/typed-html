use std::fmt::{Display, Error, Formatter};
use std::marker::PhantomData;

extern crate web_sys;
use web_sys::{
    Event,
    EventTarget,
    MouseEvent,
    Document,
    Node,
};
use wasm_bindgen::{JsValue, JsCast};
use wasm_bindgen::convert::FromWasmAbi;
use wasm_bindgen::closure::Closure;

use crate::OutputType;
use dom::VNode;
use events::{EventHandler, IntoEventHandler};

pub struct EventListenerHandle {
    // TODO: Grok memory management
    // pub target: EventTarget,
    // pub func: Closure<dyn FnMut()>,
    // pub name: &'static str,
    pub r: Result<(), JsValue>
}

/// DOM output using the WebSys crate
pub struct WebSys;
impl OutputType for WebSys {
    type Events = Events;
    type EventTarget = EventTarget;
    type EventListenerHandle = EventListenerHandle;
}

trait EventName<E> {
    const EVENT_TYPE: &'static str = "unknown";
}

macro_rules! declare_events {
    ($($name:ident : $type:ty ,)*) => {
        /// Container type for DOM events.
        pub struct Events {
            $(
                pub $name: Option<Box<dyn EventHandler<WebSys, $type>>>,
            )*
        }
        $(
        impl<F, E> EventName<$type> for EFn<F, E> {
           const EVENT_TYPE: &'static str = stringify!($name);
        }
        )*

        impl Default for Events {
            fn default() -> Self {
                Events {
                    $(
                        $name: None,
                    )*
                }
            }
        }

        /// Iterate over the defined events on a DOM object.
        #[macro_export]
        macro_rules! for_ws_events {
            ($event:ident in $events:expr => $body:block) => {
                $(
                    if let Some(ref mut $event) = $events.$name $body
                )*
            }
        }
    }
}

// TODO? these are all the "on*" attributes defined in the HTML5 standard, with
// the ones I've been unable to match to WebSys event types commented out.
//
// This needs review.

declare_events! {
    // abort: ResourceAbortEvent,
    // autocompletE: Into<Event>,
    // autocompleteerror: Event,
    // blur: BlurEvent,
    // cancel: Event,
    // canplay: Event,
    // canplaythrough: Event,
    // change: ChangeEvent,
    click: MouseEvent,
    // closE: Into<Event>,
    // contextmenu: ContextMenuEvent,
    // cuechangE: Into<Event>,
    // dblclick: DoubleClickEvent,
    // drag: DragEvent,
    // dragend: DragEndEvent,
    // dragenter: DragEnterEvent,
    // dragexit: DragExitEvent,
    // dragleave: DragLeaveEvent,
    // dragover: DragOverEvent,
    // dragstart: DragStartEvent,
    // drop: DragDropEvent,
    // durationchangE: Into<Event>,
    // emptied: Event,
    // ended: Event,
    // error: ResourceErrorEvent,
    // focus: FocusEvent,
    // input: InputEvent,
    // invalid: Event,
    // keydown: KeyDownEvent,
    // keypress: KeyPressEvent,
    // keyup: KeyUpEvent,
    // load: ResourceLoadEvent,
    // loadeddata: Event,
    // loadedmetadata: Event,
    // loadstart: LoadStartEvent,
    // mousedown: MouseDownEvent,
    // mouseenter: MouseEnterEvent,
    // mouseleave: MouseLeaveEvent,
    // mousemove: MouseMoveEvent,
    // mouseout: MouseOutEvent,
    // mouseover: MouseOverEvent,
    // mouseup: MouseUpEvent,
    // mousewheel: MouseWheelEvent,
    // pausE: Into<Event>,
    // play: Event,
    // playing: Event,
    // progress: ProgressEvent,
    // ratechangE: Into<Event>,
    // reset: Event,
    // resize: ResizeEvent,
    // scroll: ScrollEvent,
    // seeked: Event,
    // seeking: Event,
    // select: Event,
    // show: Event,
    // sort: Event,
    // stalled: Event,
    // submit: SubmitEvent,
    // suspend: Event,
    // timeupdatE: Into<Event>,
    // togglE: Into<Event>,
    // volumechangE: Into<Event>,
    // waiting: Event,
}

impl Display for Events {
    fn fmt(&self, _f: &mut Formatter) -> Result<(), Error> {
        Ok(())
    }
}

/// Wrapper type for closures as event handlers.
pub struct EFn<F, E>(Option<F>, PhantomData<E>);

impl<F, E> EFn<F, E>
where
    F: FnMut(E) + 'static,
    E: Into<Event> + FromWasmAbi + 'static,
{
    pub fn new(f: F) -> Self {
        EFn(Some(f), PhantomData)
    }
}

impl<F, E> IntoEventHandler<WebSys, E> for F
where
    F: FnMut(E) + 'static,
    E: Into<Event> + FromWasmAbi + 'static,
{
    fn into_event_handler(self) -> Box<dyn EventHandler<WebSys, E>> {
        Box::new(EFn::new(self))
    }
}

impl<F, E> IntoEventHandler<WebSys, E> for EFn<F, E>
where
    F: FnMut(E) + 'static,
    E: Into<Event> + FromWasmAbi + 'static,
{
    fn into_event_handler(self) -> Box<dyn EventHandler<WebSys, E>> {
        Box::new(self)
    }
}

impl<F, E> EventHandler<WebSys, E> for EFn<F, E>
where
    F: FnMut(E) + 'static,
    E: Into<Event> + FromWasmAbi + 'static,
{
    fn attach(&mut self, target: &mut <WebSys as OutputType>::EventTarget) -> EventListenerHandle {
        let handler = self.0.take().unwrap();
        let name = Self::EVENT_TYPE;
        let func = Closure::wrap(Box::new(handler) as Box<dyn FnMut(_)>);
        let r = target.add_event_listener_with_callback(name, func.as_ref().unchecked_ref());
        func.forget();
        EventListenerHandle {
            r,
        }
    }

    fn render(&self) -> Option<String> {
        None
    }
}

impl WebSys {
    fn install_handlers(target: &mut EventTarget, handlers: &mut Events) {
        for_ws_events!(handler in handlers => {
            handler.attach(target);
        });
    }

    pub fn build(
        document: &Document,
        vnode: VNode<'_, WebSys>,
    ) -> Result<Node, JsValue> {
        match vnode {
            VNode::Text(text) => Ok(document.create_text_node(&text).into()),
            VNode::Element(element) => {
                let mut node = document.create_element(element.name)?;
                for (key, value) in element.attributes {
                    node.set_attribute(&key, &value)?;
                }

                let mut node_et: EventTarget = node.into();

                WebSys::install_handlers(&mut node_et, element.events);

                let mut node = node_et.dyn_into::<web_sys::Element>()?;

                for child in element.children {
                    let child_node = WebSys::build(document, child)?;
                    node.append_child(&child_node)?;
                }
                Ok(node.into())
            }
        }
    }
}
