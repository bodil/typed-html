use std::fmt::{Display, Error, Formatter};
use std::marker::PhantomData;

use yew::html;
use yew::html::{Component, Html, Renderable};
use yew::virtual_dom::vtag::VTag;
use yew::virtual_dom::vtext::VText;
use yew::virtual_dom::Listener;

use crate::dom::VNode as DomVNode;
use crate::events::EventHandler;
use crate::OutputType;

/// DOM output using the stdweb crate
pub struct Yew<C: Component + Renderable<C>> {
    component_type: PhantomData<C>,
}

impl<C: Component + Renderable<C>> OutputType for Yew<C> {
    type Events = Events<C>;
    type EventTarget = VTag<C>;
    type EventListenerHandle = ();
}

macro_rules! declare_events_yew {
    ($($name:ident : $action:ident ,)*) => {
        /// Container type for DOM events.
        pub struct Events<C: Component + Renderable<C>> {
            $(
                pub $name: Option<Box<dyn EventHandler<Yew<C>, html::$action::Event>>>,
            )*
        }

        $(
            impl private::Sealed for html::$action::Event {}
            impl ConcreteEvent for html::$action::Event {}

            impl<F, C> From<F> for BoxedListener<C, html::$action::Event>
            where
                F: Fn(html::$action::Event) -> C::Message + 'static,
                C: Component + Renderable<C>,
            {
                fn from(f: F) -> Self {
                    BoxedListener(Some(Box::new(html::$action::Wrapper::from(f))), PhantomData)
                }
            }

            impl<F, C> From<F> for Box<dyn EventHandler<Yew<C>, html::$action::Event>>
            where
                F: Fn(html::$action::Event) -> C::Message + 'static,
                C: Component + Renderable<C>,
            {
                fn from(f: F) -> Self {
                    Box::new(BoxedListener::from(f))
                }
            }
        )*

        impl<C: Component + Renderable<C>> Default for Events<C> {
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
        macro_rules! for_events_yew {
            ($event:ident in $events:expr => $body:block) => {
                $(
                    if let Some(ref mut $event) = $events.$name $body
                )*
            }
        }
    }
}

// TODO? these are all the "on*" attributes used in typed-html, with
// the ones I've been unable to match to yew event types commented out.
// Yew also includes pointer events, which have been left disabled for now.
//
// This needs review.

declare_events_yew! {
    // abort: ?,
    // autocomplete: ?,
    // autocompleteerror: ?,
    blur: onblur,
    // cancel: ?,
    // canplay: ?,
    // canplaythrough: ?,
    change: onchange,
    click: onclick,
    // close: ?,
    contextmenu: oncontextmenu,
    // cuechange: ?,
    dblclick: ondoubleclick,
    drag: ondrag,
    dragend: ondragend,
    dragenter: ondragenter,
    dragexit: ondragexit,
    dragleave: ondragleave,
    dragover: ondragover,
    dragstart: ondragstart,
    drop: ondrop,
    // durationchange: ?,
    // emptied: ?,
    // ended: ?,
    // error: ?,
    focus: onfocus,
    // gotpointercapture: ongotpointercapture,
    input: oninput,
    // invalid: ?,
    keydown: onkeydown,
    keypress: onkeypress,
    keyup: onkeyup,
    // load: ?,
    // loadeddata: ?,
    // loadedmetadata: ?,
    // loadstart: ?,
    // lostpointercapture: onlostpointercapture,
    mousedown: onmousedown,
    mouseenter: onmouseenter,
    mouseleave: onmouseleave,
    mousemove: onmousemove,
    mouseout: onmouseout,
    mouseover: onmouseover,
    mouseup: onmouseup,
    mousewheel: onmousewheel,
    // pause: ?,
    // play: ?,
    // playing: ?,
    // pointercancel: onpointercancel,
    // pointerdown: onpointerdown,
    // pointerenter: onpointerenter,
    // pointerleave: onpointerleave,
    // pointermove: onpointermove,
    // pointerout: onpointerout,
    // pointerover: onpointerover,
    // pointerup: onpointerup,
    // progress: ?,
    // ratechange: ?,
    // reset: ?,
    // resize: ?,
    scroll: onscroll,
    // seeked: ?,
    // seeking: ?,
    // select: ?,
    // show: ?,
    // sort: ?,
    // stalled: ?,
    submit: onsubmit,
    // suspend: ?,
    // timeupdate: ?,
    // toggle: ?,
    // volumechange: ?,
    // waiting: ?,
}

impl<C: Component + Renderable<C>> Display for Events<C> {
    fn fmt(&self, _f: &mut Formatter) -> Result<(), Error> {
        Ok(())
    }
}

/// A trait representing any concrete event type, as Yew doesn't have one.
/// Cannot be implemented externally, as it's intended as a marker.
pub trait ConcreteEvent: private::Sealed {}

mod private {
    pub trait Sealed {}
}

pub struct BoxedListener<C: Component + Renderable<C>, E: ConcreteEvent>(
    Option<Box<dyn Listener<C>>>,
    PhantomData<E>,
);

impl<E, C> EventHandler<Yew<C>, E> for BoxedListener<C, E>
where
    E: ConcreteEvent,
    C: Component + Renderable<C>,
{
    fn attach(&mut self, target: &mut <Yew<C> as OutputType>::EventTarget) -> () {
        let handler = self.0.take().unwrap();
        target.add_listener(handler)
    }

    fn render(&self) -> Option<String> {
        None
    }
}

impl<C: Component + Renderable<C>> Yew<C> {
    pub fn install_handlers(target: &mut VTag<C>, handlers: &mut Events<C>) {
        for_events_yew!(handler in handlers => {
            handler.attach(target);
        });
    }

    pub fn build(vnode: DomVNode<'_, Yew<C>>) -> Html<C> {
        match vnode {
            DomVNode::Text(text) => VText::new(text.to_owned()).into(),
            DomVNode::UnsafeText(text) => VText::new(text.to_owned()).into(),
            DomVNode::Element(element) => {
                let mut tag = VTag::new(element.name);
                tag.attributes = element
                    .attributes
                    .into_iter()
                    .map(|(k, v)| (k.to_owned(), v))
                    .collect();
                Yew::<C>::install_handlers(&mut tag, element.events);
                for child in element.children {
                    tag.add_child(Yew::<C>::build(child))
                }
                tag.into()
            }
        }
    }
}
