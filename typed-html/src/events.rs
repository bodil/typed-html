//! Event handlers.

use std::marker::PhantomData;
use stdweb::web::event::*;
use stdweb::web::{Element, EventListenerHandle, IEventTarget};

use super::{OutputType, DOM};

macro_rules! declare_events {
    ($($name:ident : $type:ty ,)*) => {
        /// Container type for DOM events.
        pub struct Events<T: OutputType> {
            $(
                pub $name: Option<Box<dyn EventHandler<T, $type>>>,
            )*
        }

        impl<T: OutputType> Default for Events<T> {
            fn default() -> Self {
                Events {
                    $(
                        $name: None,
                    )*
                }
            }
        }

        /// Iterate over the defined events on a DOM object.
        ///
        /// # Examples
        ///
        /// ```
        /// # #![feature(proc_macro_hygiene)]
        /// # use typed_html::{html, for_events};
        /// # use typed_html::dom::{DOMTree, VNode};
        /// # fn main() {
        /// let mut doc: DOMTree<String> = html!(
        ///     <button onclick="alert('clicked!')"/>
        /// );
        /// if let VNode::Element(element) = doc.vnode() {
        ///     for_events!(event in element.events => {
        ///         assert_eq!("alert('clicked!')", event.render().unwrap());
        ///     });
        /// }
        /// # }
        /// ```
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

/// Trait for event handlers.
pub trait EventHandler<T: OutputType, E> {
    /// Build a callback function from this event handler.
    ///
    /// Returns `None` is this event handler can't be used to build a callback
    /// function. This is usually the case if the event handler is a string
    /// intended for server side rendering.
    // fn build(self) -> Option<Box<FnMut(EventType) + 'static>>;

    fn attach(&mut self, target: &Element) -> EventListenerHandle;

    /// Render this event handler as a string.
    ///
    /// Returns `None` if this event handler cannot be rendered. Normally, the
    /// only event handlers that can be rendered are string values intended for
    /// server side rendering.
    fn render(&self) -> Option<String>;
}

/// Trait for building event handlers from other types.
pub trait IntoEventHandler<T: OutputType, E> {
    /// Construct an event handler from an instance of the source type.
    fn into_event_handler(self) -> Box<dyn EventHandler<T, E>>;
}

/// Wrapper type for closures as event handlers.
pub struct EFn<F, E>(Option<F>, PhantomData<E>);

impl<F, E> EFn<F, E>
where
    F: FnMut(E) + 'static,
    E: ConcreteEvent,
{
    pub fn new(f: F) -> Self {
        EFn(Some(f), PhantomData)
    }
}

impl<F, E> IntoEventHandler<DOM, E> for EFn<F, E>
where
    F: FnMut(E) + 'static,
    E: ConcreteEvent + 'static,
{
    fn into_event_handler(self) -> Box<dyn EventHandler<DOM, E>> {
        Box::new(self)
    }
}

impl<F, E> IntoEventHandler<DOM, E> for F
where
    F: FnMut(E) + 'static,
    E: ConcreteEvent + 'static,
{
    fn into_event_handler(self) -> Box<dyn EventHandler<DOM, E>> {
        Box::new(EFn::new(self))
    }
}

impl<F, E> EventHandler<DOM, E> for EFn<F, E>
where
    F: FnMut(E) + 'static,
    E: ConcreteEvent,
{
    fn attach(&mut self, target: &Element) -> EventListenerHandle {
        let handler = self.0.take().unwrap();
        target.add_event_listener(handler)
    }

    fn render(&self) -> Option<String> {
        None
    }
}

impl<E> EventHandler<String, E> for &'static str {
    fn attach(&mut self, _target: &Element) -> EventListenerHandle {
        panic!("Silly wabbit, strings as event handlers are only for printing.");
    }

    fn render(&self) -> Option<String> {
        Some(self.to_string())
    }
}

impl<E> IntoEventHandler<String, E> for &'static str {
    fn into_event_handler(self) -> Box<dyn EventHandler<String, E>> {
        Box::new(self)
    }
}
