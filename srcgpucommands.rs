#[derive(Debug)]
pub enum RenderCommand {
    Clear8Bit {
        color: crate::core::color::Color8Bit,
    },
    Clear10Bit {
        color: crate::core::color::Color10Bit,
    },
    Clear12Bit {
        color: crate::core::color::Color12Bit,
    },
    ClearHDR {
        color: crate::core::color::ColorHDR,
    },
    Draw {
        vertex_count: u32,
    },
    DrawIndexed {
        index_count: u32,
    },
    SetViewport {
        x: u32,
        y: u32,
        width: u32,
        height: u32,
    },
    SetScissorRect {
        x: u32,
        y: u32,
        width: u32,
        height: u32,
    },
    BindVertexBuffer {
        buffer: u32, // GPU buffer ID veya referansı
        slot: u32,   // Vertex buffer'ın bağlanacağı slot
        offset: u64, // Tampon içindeki başlangıç ofseti
    },
    BindIndexBuffer {
        buffer: u32, // GPU buffer ID veya referansı
        index_type: IndexType,
        offset: u64, // Tampon içindeki başlangıç ofseti
    },
    BindShaderProgram {
        program: u32, // Shader program ID veya referansı
    },
    SetUniformInt {
        location: u32, // Uniform değişkenin konumu
        value: i32,
    },
    SetUniformFloat {
        location: u32, // Uniform değişkenin konumu
        value: f32,
    },
    SetUniformVec2 {
        location: u32, // Uniform değişkenin konumu
        value: crate::core::math::Vec2,
    },
    SetUniformVec3 {
        location: u32, // Uniform değişkenin konumu
        value: crate::core::math::Vec3,
    },
    SetUniformVec4 {
        location: u32, // Uniform değişkenin konumu
        value: (f32, f32, f32, f32), // Veya kendi Vec4 yapınızı kullanabilirsiniz
    },
    SetUniformMat4 {
        location: u32, // Uniform değişkenin konumu
        value: crate::core::math::Mat4,
    },
    BindTexture {
        texture: u32,     // Texture ID veya referansı
        unit: u32,        // Texture unit (sampler slot)
    },
    SetRenderTarget {
        render_target: u32, // Render target ID veya referansı (0 genellikle varsayılan framebuffer'ı temsil eder)
    },
    SetDepthTestEnabled {
        enabled: bool,
    },
    SetDepthMaskEnabled {
        enabled: bool,
    },
    SetDepthCompareFunction {
        function: CompareFunction,
    },
    SetBlendEnabled {
        enabled: bool,
    },
    SetBlendFunction {
        src_factor: BlendFactor,
        dst_factor: BlendFactor,
        operation: BlendOperation,
    },
    SetPrimitiveTopology {
        topology: PrimitiveTopology,
    },
    // İleride eklenebilecek diğer komutlar...
}

#[derive(Debug)]
pub enum IndexType {
    Uint16,
    Uint32,
}

#[derive(Debug)]
pub enum CompareFunction {
    Never,
    Less,
    Equal,
    LessOrEqual,
    Greater,
    NotEqual,
    GreaterOrEqual,
    Always,
}

#[derive(Debug)]
pub enum BlendFactor {
    Zero,
    One,
    SrcColor,
    OneMinusSrcColor,
    DstColor,
    OneMinusDstColor,
    SrcAlpha,
    OneMinusSrcAlpha,
    DstAlpha,
    OneMinusDstAlpha,
    ConstantColor,
    OneMinusConstantColor,
    ConstantAlpha,
    OneMinusConstantAlpha,
    SrcAlphaSaturate,
    Src1Color,
    OneMinusSrc1Color,
    Src1Alpha,
    OneMinusSrc1Alpha,
}

#[derive(Debug)]
pub enum BlendOperation {
    Add,
    Subtract,
    ReverseSubtract,
    Min,
    Max,
}

#[derive(Debug)]
pub enum PrimitiveTopology {
    PointList,
    LineList,
    LineStrip,
    TriangleList,
    TriangleStrip,
}