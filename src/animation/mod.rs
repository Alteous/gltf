use {accessor, json, scene, Document, Buffer};

pub use json::animation::{Interpolation, Property};

/// Iterators.
pub mod iter;

/// Utility functions.
#[cfg(feature = "utils")]
pub mod util;

#[cfg(feature = "utils")]
#[doc(inline)]
pub use self::util::{ReadInputs, ReadOutputs};

/// A keyframe animation.
#[derive(Clone, Debug)]
pub struct Animation<'a> {
    /// The parent `Document` struct.
    document: &'a Document,

    /// The corresponding JSON index.
    index: usize,

    /// The corresponding JSON struct.
    json: &'a json::animation::Animation,
}

/// Targets an animation's sampler at a node's property.
#[derive(Clone, Debug)]
pub struct Channel<'a> {
    /// The parent `Animation` struct.
    anim: Animation<'a>,

    /// The corresponding JSON struct.
    json: &'a json::animation::Channel,
}

/// Defines a keyframe graph (but not its target).
#[derive(Clone, Debug)]
pub struct Sampler<'a> {
    /// The parent `Animation` struct.
    anim: Animation<'a>,

    /// The corresponding JSON struct.
    json: &'a json::animation::Sampler,
}

/// The node and TRS property that an animation channel targets.
#[derive(Clone, Debug)]
pub struct Target<'a> {
    /// The parent `Animation` struct.
    anim: Animation<'a>,

    /// The corresponding JSON struct.
    json: &'a json::animation::Target,
}

/// Animation channel reader.
#[cfg(feature = "utils")]
#[derive(Clone, Debug)]
pub struct Reader<'a, 's, F>
where
    F: Clone + Fn(Buffer<'a>) -> Option<&'s [u8]>,
{
    channel: Channel<'a>,
    get_buffer_data: F,
}

impl<'a> Animation<'a> {
    /// Constructs an `Animation`.
    pub(crate) fn new(
        document: &'a Document, index: usize,
        json: &'a json::animation::Animation,
    ) -> Self {
        Self {
            document: document,
            index: index,
            json: json,
        }
    }

    /// Returns the internal JSON index.
    pub fn index(&self) -> usize {
        self.index
    }

    /// Optional application specific data.
    pub fn extras(&self) -> &json::Extras {
        &self.json.extras
    }

    /// Returns an `Iterator` over the animation channels.
    ///
    /// Each channel targets an animation's sampler at a node's property.
    pub fn channels(&self) -> iter::Channels<'a> {
        iter::Channels {
            anim: self.clone(),
            iter: self.json.channels.iter(),
        }
    }

    /// Optional user-defined name for this object.
    #[cfg(feature = "names")]
    pub fn name(&self) -> Option<&str> {
        self.json.name.as_ref().map(String::as_str)
    }

    /// Returns an `Iterator` over the animation samplers.
    ///
    /// Each sampler combines input and output accessors with an
    /// interpolation algorithm to define a keyframe graph (but not its target).
    pub fn samplers(&self) -> iter::Samplers<'a> {
        iter::Samplers {
            anim: self.clone(),
            iter: self.json.samplers.iter(),
        }
    }
}

impl<'a> Channel<'a> {
    /// Constructs a `Channel`.
    pub(crate) fn new(
        anim: Animation<'a>,
        json: &'a json::animation::Channel,
    ) -> Self {
        Self {
            anim: anim,
            json: json,
        }
    }

    /// Returns the parent `Animation` struct.
    pub fn animation(&self) -> Animation<'a> {
        self.anim.clone()
    }

    /// Returns the sampler in this animation used to compute the value for the
    /// target.
    pub fn sampler(&self) -> Sampler<'a> {
        self.anim.samplers().nth(self.json.sampler.value()).unwrap()
    }

    /// Returns the node and property to target.
    pub fn target(&self) -> Target<'a> {
        Target::new(self.anim.clone(), &self.json.target)
    }

    /// Constructs an animation channel reader.
    #[cfg(feature = "utils")]
    pub fn reader<'s, F>(&self, get_buffer_data: F) -> Reader<'a, 's, F>
    where
        F: Clone + Fn(Buffer<'a>) -> Option<&'s [u8]>,
    {
        Reader {
            channel: self.clone(),
            get_buffer_data,
        }
    }

    /// Optional application specific data.
    pub fn extras(&self) -> &json::Extras {
        &self.json.extras
    }
}

#[cfg(feature = "utils")]
impl<'a, 's, F> Reader<'a, 's, F>
where F: Clone + Fn(Buffer<'a>) -> Option<&'s [u8]>,
{
    /// Visits the input samples of a channel.
    pub fn read_inputs(&self) -> Option<util::ReadInputs<'s>> {
        let buffer = self.channel.sampler().input().view().buffer();
        if let Some(slice) = (self.get_buffer_data)(buffer) {
            Some(accessor::Iter::new(self.channel.sampler().input(), slice))
        } else {
            None
        }
    }

    /// Visits the output samples of a channel.
    pub fn read_outputs(&self) -> Option<util::ReadOutputs<'s>> {
        use accessor::{DataType, Iter};
        use animation::Property;
        use self::util::{Rotations, ReadOutputs, MorphTargetWeights};

        let output = self.channel.sampler().output();
        if let Some(slice) = (self.get_buffer_data)(output.view().buffer()) {
            Some(
                match self.channel.target().property() {
                    Property::Translation => ReadOutputs::Translations(Iter::new(output, slice)),
                    Property::Rotation => ReadOutputs::Rotations(match output.data_type() {
                        DataType::I8 => Rotations::I8(Iter::new(output, slice)),
                        DataType::U8 => Rotations::U8(Iter::new(output, slice)),
                        DataType::I16 => Rotations::I16(Iter::new(output, slice)),
                        DataType::U16 => Rotations::U16(Iter::new(output, slice)),
                        DataType::F32 => Rotations::F32(Iter::new(output, slice)),
                        _ => unreachable!()
                    }),
                    Property::Scale => ReadOutputs::Scales(Iter::new(output, slice)),
                    Property::MorphTargetWeights => ReadOutputs::MorphTargetWeights(match output.data_type() {
                        DataType::I8 => MorphTargetWeights::I8(Iter::new(output, slice)),
                        DataType::U8 => MorphTargetWeights::U8(Iter::new(output, slice)),
                        DataType::I16 => MorphTargetWeights::I16(Iter::new(output, slice)),
                        DataType::U16 => MorphTargetWeights::U16(Iter::new(output, slice)),
                        DataType::F32 => MorphTargetWeights::F32(Iter::new(output, slice)),
                        _ => unreachable!()
                    }),
                }
            )            
        } else {
            None
        }
    }
}

impl<'a> Target<'a> {
    /// Constructs a `Target`.
    pub(crate) fn new(
        anim: Animation<'a>,
        json: &'a json::animation::Target,
    ) -> Self {
        Self {
            anim: anim,
            json: json,
        }
    }

    /// Returns the parent `Animation` struct.
    pub fn animation(&self) -> Animation<'a> {
        self.anim.clone()
    }

    /// Optional application specific data.
    pub fn extras(&self) -> &json::Extras {
        &self.json.extras
    }

    /// Returns the target node.
    pub fn node(&self) -> scene::Node {
        self.anim.document.nodes().nth(self.json.node.value()).unwrap()
    }

    /// Returns the node's property to modify or the 'weights' of the morph
    /// targets it instantiates.
    pub fn property(&self) -> Property {
        self.json.path.unwrap()
    }
}

impl<'a> Sampler<'a> {
    /// Constructs a `Sampler`.
    pub(crate) fn new(
        anim: Animation<'a>,
        json: &'a json::animation::Sampler,
    ) -> Self {
        Self {
            anim: anim,
            json: json,
        }
    }

    /// Returns the parent `Animation` struct.
    pub fn animation(&self) -> Animation<'a> {
        self.anim.clone()
    }

    /// Optional application specific data.
    pub fn extras(&self) -> &json::Extras {
        &self.json.extras
    }

    /// Returns the accessor containing the keyframe input values (e.g. time).
    pub fn input(&self) -> accessor::Accessor<'a> {
        self.anim.document.accessors().nth(self.json.input.value()).unwrap()
    }

    /// Returns the keyframe interpolation algorithm.
    pub fn interpolation(&self) -> Interpolation {
        self.json.interpolation.unwrap()
    }

    /// Returns the accessor containing the keyframe output values.
    pub fn output(&self) -> accessor::Accessor<'a> {
        self.anim.document.accessors().nth(self.json.output.value()).unwrap()
    }
}
