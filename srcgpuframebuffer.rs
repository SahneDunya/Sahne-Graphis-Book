use crate::gpu::texture::Texture;

#[derive(Debug)]
pub struct FramebufferDescriptor<'a> {
    pub color_attachments: &'a [Option<&'a Texture>],
    pub depth_stencil_attachment: Option<&'a Texture>,
    pub width: u32,
    pub height: u32,
}

#[derive(Debug)]
pub struct Framebuffer {
    pub(crate) native_framebuffer: (), // Placeholder for now
    pub descriptor: FramebufferDescriptor<'static>, // Dikkat: Ömür yönetimi önemli
}

impl Framebuffer {
    pub fn new(descriptor: &FramebufferDescriptor) -> Result<Self, &'static str> {
        println!("Oluşturulan framebuffer: {:?}", descriptor);
        // Platforma özel framebuffer oluşturma mantığı burada yer alacak.
        // Bu, sağlanan dokuları framebuffer nesnesiyle ilişkilendirmeyi içerir.

        // Ömür yönetimi burada kritik öneme sahip. Şu anda descriptor'daki
        // referansların ömrünü statik olarak kabul ediyoruz, bu doğru olmayabilir.
        // Gerçek bir uygulamada, framebuffer'ın referans aldığı dokuların
        // ömründen daha uzun yaşamamasını sağlamak için daha dikkatli bir yönetim gerekebilir.
        // Örneğin, referans sayımı veya kütüphane seviyesinde bir ömür yönetimi sistemi kullanılabilir.
        let static_descriptor = FramebufferDescriptor {
            color_attachments: descriptor.color_attachments.iter().map(|opt| opt.copied()).collect::<Vec<_>>().leak(),
            depth_stencil_attachment: descriptor.depth_stencil_attachment.copied(),
            width: descriptor.width,
            height: descriptor.height,
        };

        Ok(Framebuffer {
            native_framebuffer: (),
            descriptor: static_descriptor,
        })
    }
}