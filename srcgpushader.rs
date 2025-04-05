#[derive(Debug)]
pub struct ShaderDescriptor {
    pub label: Option<String>,
    pub source: ShaderSource,
    pub stage: ShaderStage,
}

#[derive(Debug)]
pub enum ShaderSource {
    SpirV(Vec<u8>),
    // İleride diğer kaynak türleri (örneğin, Glsl veya Hlsl) eklenebilir,
    // potansiyel olarak derleme gerektirebilir.
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShaderStage {
    Vertex,
    Fragment,
    Compute,
    // İhtiyaca göre diğer aşamalar eklenebilir (örneğin, Tessellation Control, Tessellation Evaluation, Geometry).
}

#[derive(Debug)]
pub struct Shader {
    pub(crate) native_shader: (), // Şimdilik yer tutucu
    pub descriptor: ShaderDescriptor,
}

impl Shader {
    pub fn new(descriptor: ShaderDescriptor) -> Self {
        println!("Oluşturulan shader: {:?}", descriptor);
        Shader {
            native_shader: (),
            descriptor,
        }
    }
}