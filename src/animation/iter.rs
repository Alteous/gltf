use json;
use std::slice;

use animation::{Animation, Channel, Sampler};

/// An `Iterator` that visits the channels of an animation.
#[derive(Clone, Debug)]
pub struct Channels<'a> {
    /// The parent `Animation` struct.
    pub(crate) anim: Animation<'a>,

    /// The internal channel iterator.
    pub(crate) iter: slice::Iter<'a, json::animation::Channel>,
}

/// An `Iterator` that visits the samplers of an animation.
#[derive(Clone, Debug)]
pub struct Samplers<'a> {
    /// The parent `Channel` struct.
    pub(crate) anim: Animation<'a>,

    /// The internal channel iterator.
    pub(crate) iter: slice::Iter<'a, json::animation::Sampler>,
}

impl<'a> Iterator for Channels<'a> {
    type Item = Channel<'a>;
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|json| Channel::new(self.anim.clone(), json))
    }
}

impl<'a> Iterator for Samplers<'a> {
    type Item = Sampler<'a>;
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|json| Sampler::new(self.anim.clone(), json))
    }
}
