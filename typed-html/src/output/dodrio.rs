use std::fmt::{Display, Error, Formatter};

use crate::OutputType;

/// DOM output using the Dodrio virtual DOM
pub struct Dodrio;
impl OutputType for Dodrio {
    type Events = Events;
    type EventTarget = ();
    type EventListenerHandle = ();
}

#[derive(Default)]
pub struct Events;

impl Display for Events {
    fn fmt(&self, _: &mut Formatter) -> Result<(), Error> {
        unimplemented!()
    }
}
