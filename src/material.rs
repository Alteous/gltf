use {json, texture, Document};

pub use json::material::AlphaMode;

lazy_static! {
    static ref DEFAULT_MATERIAL: json::material::Material = Default::default();
}

/// The material appearance of a primitive.
pub struct Material<'a> {
    /// The parent `Document` struct.
    document: &'a Document,

    /// The corresponding JSON index - `None` when the default material.
    index: Option<usize>,

    /// The corresponding JSON struct.
    json: &'a json::material::Material,
}

impl<'a> Material<'a> {
    /// Constructs a `Material`.
    pub(crate) fn new(
        document: &'a Document,
        index: usize,
        json: &'a json::material::Material,
    ) -> Self {
        Self {
            document: document,
            index: Some(index),
            json: json,
        }
    }

    /// Constructs the default `Material`.
    pub(crate) fn default(document: &'a Document) -> Self {
        Self {
            document: document,
            index: None,
            json: &DEFAULT_MATERIAL,
        }
    }

    /// Returns the internal JSON index if this `Material` was explicity defined.
    ///
    /// This function returns `None` if the `Material` is the default material.
    pub fn index(&self) -> Option<usize> {
        self.index
    }

    ///  The alpha cutoff value of the material.
    pub fn alpha_cutoff(&self) -> f32 {
        self.json.alpha_cutoff.0
    }

    /// The alpha rendering mode of the material.  The material's alpha rendering
    /// mode enumeration specifying the interpretation of the alpha value of the main
    /// factor and texture.
    ///
    /// * In `Opaque` mode (default) the alpha value is ignored
    ///   and the rendered output is fully opaque.
    /// * In `Mask` mode, the rendered
    ///   output is either fully opaque or fully transparent depending on the alpha
    ///   value and the specified alpha cutoff value.
    /// * In `Blend` mode, the alpha value is used to composite the source and
    ///   destination areas and the rendered output is combined with the background
    ///   using the normal painting operation (i.e. the Porter and Duff over
    ///   operator).
    pub fn alpha_mode(&self) -> AlphaMode {
        self.json.alpha_mode.unwrap()
    }

    /// Specifies whether the material is double-sided.
    ///
    /// * When this value is false, back-face culling is enabled.
    /// * When this value is true, back-face culling is disabled and double sided
    ///   lighting is enabled.  The back-face must have its normals reversed before
    ///   the lighting equation is evaluated.
    pub fn double_sided(&self) -> bool {
        self.json.double_sided
    }

    /// Optional user-defined name for this object.
    #[cfg(feature = "names")]
    pub fn name(&self) -> Option<&str> {
        self.json.name.as_ref().map(String::as_str)
    }

    /// Parameter values that define the metallic-roughness material model from
    /// Physically-Based Rendering (PBR) methodology.
    pub fn pbr_metallic_roughness(&self) -> PbrMetallicRoughness<'a> {
        PbrMetallicRoughness::new(self.document, &self.json.pbr_metallic_roughness)
    }

    /// A tangent space normal map.
    ///
    /// The texture contains RGB components in linear space. Each texel represents
    /// the XYZ components of a normal vector in tangent space.
    ///
    /// * Red [0 to 255] maps to X [-1 to 1].
    /// * Green [0 to 255] maps to Y [-1 to 1].
    /// * Blue [128 to 255] maps to Z [1/255 to 1].
    ///
    /// The normal vectors use OpenGL conventions where +X is right, +Y is up, and
    /// +Z points toward the viewer.
    pub fn normal_texture(&self) -> Option<NormalTexture<'a>> {
        self.json.normal_texture.as_ref().map(|json| {
            let texture = self.document.textures().nth(json.index.value()).unwrap();
            NormalTexture::new(texture, json)
        })
    }

    /// The occlusion map texture.
    ///
    /// The occlusion values are sampled from the R channel. Higher values indicate
    /// areas that should receive full indirect lighting and lower values indicate
    /// no indirect lighting. These values are linear.
    ///
    /// If other channels are present (GBA), they are ignored for occlusion
    /// calculations.
    pub fn occlusion_texture(&self) -> Option<OcclusionTexture<'a>> {
        self.json.occlusion_texture.as_ref().map(|json| {
            let texture = self.document.textures().nth(json.index.value()).unwrap();
            OcclusionTexture::new(texture, json)
        })
    }

    /// The emissive map texture.
    ///
    /// The emissive map controls the color and intensity of the light being
    /// emitted by the material.
    ///
    /// This texture contains RGB components in sRGB color space. If a fourth
    /// component (A) is present, it is ignored.
    pub fn emissive_texture(&self) -> Option<texture::Info<'a>> {
        self.json.emissive_texture.as_ref().map(|json| {
            let texture = self.document.textures().nth(json.index.value()).unwrap();
            texture::Info::new(texture, json)
        })
    }

    /// The emissive color of the material.
    ///
    /// The default value is `[0.0, 0.0, 0.0]`.
    pub fn emissive_factor(&self) -> [f32; 3] {
        self.json.emissive_factor.0
    }

    /// Optional application specific data.
    pub fn extras(&self) -> &json::Extras {
        &self.json.extras
    }
}

/// A set of parameter values that are used to define the metallic-roughness
/// material model from Physically-Based Rendering (PBR) methodology.
pub struct PbrMetallicRoughness<'a> {
    /// The parent `Document` struct.
    document: &'a Document,

    /// The corresponding JSON struct.
    json: &'a json::material::PbrMetallicRoughness,
}

impl<'a> PbrMetallicRoughness<'a> {
    /// Constructs `PbrMetallicRoughness`.
    pub(crate) fn new(
        document: &'a Document,
        json: &'a json::material::PbrMetallicRoughness,
    ) -> Self {
        Self {
            document: document,
            json: json,
        }
    }

    /// Returns the material's base color factor.
    ///
    /// The default value is `[1.0, 1.0, 1.0, 1.0]`.
    pub fn base_color_factor(&self) -> [f32; 4] {
        self.json.base_color_factor.0
    }

    /// Returns the base color texture.
    pub fn base_color_texture(&self) -> Option<texture::Info<'a>> {
        self.json.base_color_texture.as_ref().map(|json| {
            let texture = self.document.textures().nth(json.index.value()).unwrap();
            texture::Info::new(texture, json)
        })
    }

    /// Returns the metalness factor of the material.
    ///
    /// The default value is `1.0`.
    pub fn metallic_factor(&self) -> f32 {
        self.json.metallic_factor.0
    }

    /// Returns the roughness factor of the material.
    ///
    /// * A value of 1.0 means the material is completely rough.
    /// * A value of 0.0 means the material is completely smooth.
    ///
    /// The default value is `1.0`.
    pub fn roughness_factor(&self) -> f32 {
        self.json.roughness_factor.0
    }

    /// The metallic-roughness texture.
    ///
    /// The metalness values are sampled from the B channel.
    /// The roughness values are sampled from the G channel.
    /// These values are linear. If other channels are present (R or A),
    /// they are ignored for metallic-roughness calculations.
    pub fn metallic_roughness_texture(&self) -> Option<texture::Info<'a>> {
        self.json.metallic_roughness_texture.as_ref().map(|json| {
            let texture = self.document.textures().nth(json.index.value()).unwrap();
            texture::Info::new(texture, json)
        })
    }

    /// Optional application specific data.
    pub fn extras(&self) -> &json::Extras {
        &self.json.extras
    }
}

/// Defines the normal texture of a material.
pub struct NormalTexture<'a> {
    /// The parent `Texture` struct.
    texture: texture::Texture<'a>,

    /// The corresponding JSON struct.
    json: &'a json::material::NormalTexture,
}

impl<'a> NormalTexture<'a> {
    /// Constructs a `NormalTexture`.
    pub(crate) fn new(
        texture: texture::Texture<'a>,
        json: &'a json::material::NormalTexture,
    ) -> Self {
        Self {
            texture: texture,
            json: json,
        }
    }

    /// Returns the scalar multiplier applied to each normal vector of the texture.
    ///
    /// This value is ignored if `normal_texture` is `None`.
    pub fn scale(&self) -> f32 {
        self.json.scale
    }

    /// The set index of the texture's `TEXCOORD` attribute.
    pub fn tex_coord(&self) -> u32 {
        self.json.tex_coord
    }

    /// Returns the referenced `Texture`.
    pub fn texture(&self) -> texture::Texture<'a> {
        self.texture.clone()
    }

    /// Optional application specific data.
    pub fn extras(&self) -> &json::Extras {
        &self.json.extras
    }
}

/// Defines the occlusion texture of a material.
pub struct OcclusionTexture<'a> {
    /// The parent `Texture` struct.
    texture: texture::Texture<'a>,

    /// The corresponding JSON struct.
    json: &'a json::material::OcclusionTexture,
}

impl<'a> OcclusionTexture<'a> {
    /// Constructs a `OcclusionTexture`.
    pub(crate) fn new(
        texture: texture::Texture<'a>,
        json: &'a json::material::OcclusionTexture,
    ) -> Self {
        Self {
            texture: texture,
            json: json,
        }
    }

    /// Returns the scalar multiplier controlling the amount of occlusion applied.
    pub fn strength(&self) -> f32 {
        self.json.strength.0
    }

    /// Returns the set index of the texture's `TEXCOORD` attribute.
    pub fn tex_coord(&self) -> u32 {
        self.json.tex_coord
    }

    /// Returns the referenced `Texture`.
    pub fn texture(&self) -> texture::Texture<'a> {
        self.texture.clone()
    }
    
    /// Optional application specific data.
    pub fn extras(&self) -> &json::Extras {
        &self.json.extras
    }
}

impl<'a> AsRef<texture::Texture<'a>> for NormalTexture<'a> {
    fn as_ref(&self) -> &texture::Texture<'a> {
        &self.texture
    }
}

impl<'a> AsRef<texture::Texture<'a>> for OcclusionTexture<'a> {
    fn as_ref(&self) -> &texture::Texture<'a> {
        &self.texture
    }
}
