use crate::gpu::buffer::Buffer;
use crate::rendering::shader::Shader;
use bitflags::bitflags;

#[derive(Debug, Default, Clone)]
pub struct VertexAttributeDescriptor {
    pub location: u32, // Shader'daki attribute konumu
    pub offset: u32,   // Vertex buffer içindeki ofset (byte cinsinden)
    pub format: VertexFormat,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VertexFormat {
    Float32x3, // 3 adet 32-bit kayan nokta (örneğin, pozisyon)
    Float32x2, // 2 adet 32-bit kayan nokta (örneğin, UV koordinatları)
    Float32x4,
    Uint32,
    Sint32,
    // İhtiyaca göre diğer formatlar eklenebilir.
}

#[derive(Debug, Default, Clone)]
pub struct VertexBufferLayoutDescriptor {
    pub stride: u32,                    // Bir vertex'in boyutu (byte cinsinden)
    pub step_mode: VertexStepMode, // Vertex başına mı yoksa instance başına mı veri okunacak?
    pub attributes: Vec<VertexAttributeDescriptor>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VertexStepMode {
    Vertex,
    Instance,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PrimitiveTopology {
    PointList,
    LineList,
    LineStrip,
    TriangleList,
    TriangleStrip,
}

#[derive(Debug, Default, Clone)]
pub struct RasterizationStateDescriptor {
    pub front_face: FrontFace,
    pub cull_mode: CullMode,
    pub depth_bias: f32,
    pub depth_bias_slope_factor: f32,
    pub depth_bias_clamp: f32,
    pub polygon_mode: PolygonMode,
    pub conservative: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FrontFace {
    Clockwise,
    CounterClockwise,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CullMode {
    None,
    Front,
    Back,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PolygonMode {
    Fill,
    Line,
    Point,
}

#[derive(Debug, Default, Clone)]
pub struct DepthStencilStateDescriptor {
    pub format: DepthStencilFormat, // İleride tanımlanacak
    pub depth_write_enabled: bool,
    pub depth_compare: CompareFunction, // Tanımlanmıştı
    pub stencil_front: StencilFaceState, // İleride tanımlanacak
    pub stencil_back: StencilFaceState,  // İleride tanımlanacak
    pub stencil_read_mask: u32,
    pub stencil_write_mask: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DepthStencilFormat {
    Depth32Float,
    Depth24PlusStencil8,
    // İhtiyaca göre diğer formatlar
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct StencilFaceState {
    pub compare: CompareFunction, // Tanımlanmıştı
    pub fail_op: StencilOperation, // İleride tanımlanacak
    pub depth_fail_op: StencilOperation, // İleride tanımlanacak
    pub pass_op: StencilOperation, // İleride tanımlanacak
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StencilOperation {
    Keep,
    Zero,
    Replace,
    IncrementClamp,
    DecrementClamp,
    Invert,
    IncrementWrap,
    DecrementWrap,
}

#[derive(Debug, Default, Clone)]
pub struct MultisampleStateDescriptor {
    pub count: u32,         // Örnek sayısı (1 multisampling kapalı demektir)
    pub mask: u32,          // Örnek maskesi
    pub alpha_to_coverage_enabled: bool,
}

#[derive(Debug, Default, Clone)]
pub struct BlendStateDescriptor {
    pub color: BlendComponent, // Renk için blend ayarları
    pub alpha: BlendComponent, // Alfa için blend ayarları
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BlendFactor {
    Zero,
    One,
    Src,
    OneMinusSrc,
    Dst,
    OneMinusDst,
    SrcAlpha,
    OneMinusSrcAlpha,
    DstAlpha,
    OneMinusDstAlpha,
    Constant,
    OneMinusConstant,
    SrcAlphaSaturated,
    Src1,
    OneMinusSrc1,
    Src1Alpha,
    OneMinusSrc1Alpha,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BlendOperation {
    Add,
    Subtract,
    ReverseSubtract,
    Min,
    Max,
}

#[derive(Debug, Default, Clone)]
pub struct BlendComponent {
    pub src_factor: BlendFactor,
    pub dst_factor: BlendFactor,
    pub operation: BlendOperation,
}

#[derive(Debug, Default, Clone)]
pub struct PipelineDescriptor {
    pub vertex: VertexStateDescriptor,
    pub fragment: Option<FragmentStateDescriptor>,
    pub primitive: PrimitiveStateDescriptor,
    pub rasterization: Option<RasterizationStateDescriptor>,
    pub depth_stencil: Option<DepthStencilStateDescriptor>, // Eklendi
    pub multisample: Option<MultisampleStateDescriptor>,   // Eklendi
    // İleride blend state gibi diğer ayarlar eklenebilir. (Blend state descriptor eklendi)
}

#[derive(Debug, Default, Clone)]
pub struct VertexStateDescriptor {
    pub module: Option<Shader>,
    pub entry_point: String,
    pub buffers: Vec<VertexBufferLayoutDescriptor>,
}

#[derive(Debug, Default, Clone)]
pub struct FragmentStateDescriptor {
    pub module: Option<Shader>,
    pub entry_point: String,
    pub targets: Vec<ColorTargetStateDescriptor>,
}

#[derive(Debug, Default, Clone)]
pub struct PrimitiveStateDescriptor {
    pub topology: PrimitiveTopology,
    pub strip_index_format: Option<IndexFormat>,
    pub front_face: FrontFace,
    pub cull_mode: CullMode,
    pub unclipped_depth: bool,
    pub conservative: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IndexFormat {
    Uint16,
    Uint32,
}

#[derive(Debug, Default, Clone)]
pub struct ColorTargetStateDescriptor {
    pub format: TextureFormat, // İleride tanımlanacak
    pub blend: Option<BlendStateDescriptor>, // BlendStateDescriptor kullanıldı
    pub write_mask: ColorWrite,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextureFormat {
    // İleride tanımlanacak çeşitli texture formatları
    Rgba8Unorm,
    Rgba8Snorm,
    Rgba8Uint,
    Rgba8Sint,
    Rgba16Float,
    Rgba16Unorm,
    Rgba16Snorm,
    Rgba16Uint,
    Rgba16Sint,
    Rgba32Float,
    R32Float,
    R32Uint,
    R32Sint,
    Depth32Float,
    Depth24PlusStencil8,
    // ... diğer formatlar
}

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
    pub struct ColorWrite: u32 {
        const RED     = 0b0001;
        const GREEN   = 0b0010;
        const BLUE    = 0b0100;
        const ALPHA   = 0b1000;
        const ALL     = Self::RED.bits() | Self::GREEN.bits() | Self::BLUE.bits() | Self::ALPHA.bits();
    }
}

#[derive(Debug)]
pub struct Pipeline {
    // Bu yapı, GPU üzerinde oluşturulan renderleme hattını temsil edebilir.
    // İçinde GPU'ya özgü bir "handle" veya tanımlayıcı tutabilir.
}

impl PipelineDescriptor {
    // İleride pipeline oluşturmak için bir metot eklenebilir.
    // pub fn build(&self, device: &crate::gpu::Device) -> Pipeline { ... }
}