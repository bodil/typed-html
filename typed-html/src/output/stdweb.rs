use std::fmt::{Display, Error, Formatter};
use std::marker::PhantomData;

use stdweb::web::event::*;
use stdweb::web::{self, Element, EventListenerHandle, IElement, IEventTarget, INode};

use crate::OutputType;
use crate::dom::VNode;
use crate::events::EventHandler;

/// DOM output using the stdweb crate
pub struct Stdweb;
impl OutputType for Stdweb {
    type Events = Events;
    type EventTarget = Element;
    type EventListenerHandle = EventListenerHandle;
}

macro_rules! declare_events {
    ($($name:ident : $type:ty ,)*) => {
        /// Container type for DOM events.
        pub struct Events {
            $(
                pub $name: Option<Box<dyn EventHandler<Stdweb, $type> + Send>>,
            )*
        }

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
        macro_rules! for_events {
            ($event:ident in $events:expr => $body:block) => {
                $(
                    if let Some(ref mut $event) = $events.$name $body
                )*
            }
        }
    }
}

// TODO? these are all the "on*" attributes defined in the HTML5 standard, with
// the ones I've been unable to match to stdweb event types commented out.
//
// This needs review.

declare_events! {
    abort: ResourceAbortEvent,
    // autocomplete: Event,
    // autocompleteerror: Event,
    blur: BlurEvent,
    // cancel: Event,
    // canplay: Event,
    // canplaythrough: Event,
    change: ChangeEvent,
    click: ClickEvent,
    // close: Event,
    contextmenu: ContextMenuEvent,
    // cuechange: Event,
    dblclick: DoubleClickEvent,
    drag: DragEvent,
    dragend: DragEndEvent,
    dragenter: DragEnterEvent,
    dragexit: DragExitEvent,
    dragleave: DragLeaveEvent,
    dragover: DragOverEvent,
    dragstart: DragStartEvent,
    drop: DragDropEvent,
    // durationchange: Event,
    // emptied: Event,
    // ended: Event,
    error: ResourceErrorEvent,
    focus: FocusEvent,
    input: InputEvent,
    // invalid: Event,
    keydown: KeyDownEvent,
    keypress: KeyPressEvent,
    keyup: KeyUpEvent,
    load: ResourceLoadEvent,
    // loadeddata: Event,
    // loadedmetadata: Event,
    loadstart: LoadStartEvent,
    mousedown: MouseDownEvent,
    mouseenter: MouseEnterEvent,
    mouseleave: MouseLeaveEvent,
    mousemove: MouseMoveEvent,
    mouseout: MouseOutEvent,
    mouseover: MouseOverEvent,
    mouseup: MouseUpEvent,
    mousewheel: MouseWheelEvent,
    // pause: Event,
    // play: Event,
    // playing: Event,
    progress: ProgressEvent,
    // ratechange: Event,
    // reset: Event,
    resize: ResizeEvent,
    scroll: ScrollEvent,
    // seeked: Event,
    // seeking: Event,
    // select: Event,
    // show: Event,
    // sort: Event,
    // stalled: Event,
    submit: SubmitEvent,
    // suspend: Event,
    // timeupdate: Event,
    // toggle: Event,
    // volumechange: Event,
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
    F: FnMut(E) + 'static + Send,
{
    pub fn new(f: F) -> Self {
        EFn(Some(f), PhantomData)
    }
}

impl<F, E> From<F> for Box<dyn EventHandler<Stdweb, E> + Send>
where
    F: FnMut(E) + 'static + Send,
    E: ConcreteEvent + 'static + Send,
{
    fn from(f: F) -> Self {
        Box::new(EFn::new(f))
    }
}

impl<F, E> EventHandler<Stdweb, E> for EFn<F, E>
where
    F: FnMut(E) + 'static + Send,
    E: ConcreteEvent + 'static + Send,
{
    fn attach(&mut self, target: &mut <Stdweb as OutputType>::EventTarget) -> EventListenerHandle {
        let handler = self.0.take().unwrap();
        target.add_event_listener(handler)
    }

    fn render(&self) -> Option<String> {
        None
    }
}

impl Stdweb {
    pub fn install_handlers(target: &mut Element, handlers: &mut Events) {
        for_events!(handler in handlers => {
            handler.attach(target);
        });
    }

    pub fn build(
        document: &web::Document,
        vnode: VNode<'_, Stdweb>,
    ) -> Result<web::Node, web::error::InvalidCharacterError> {
        match vnode {
            VNode::Text(text) => Ok(document.create_text_node(text).into()),
            VNode::UnsafeText(text) => Ok(document.create_text_node(text).into()),
            VNode::Element(element) => {
                let mut node = document.create_element(element.name)?;
                for (key, value) in element.attributes {
                    node.set_attribute(key, &value)?;
                }
                Stdweb::install_handlers(&mut node, element.events);
                for child in element.children {
                    let child_node = Stdweb::build(document, child)?;
                    node.append_child(&child_node);
                }
                Ok(node.into())
            }
        }
    }
}
