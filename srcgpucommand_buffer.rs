use crate::gpu::commands::RenderCommand; // Assuming RenderCommand is defined here

#[derive(Debug)]
pub struct CommandBuffer {
    pub(crate) native_command_buffer: (), // Şimdilik yer tutucu
    commands: Vec<RenderCommand>,
    is_recording: bool,
}

impl CommandBuffer {
    pub fn new() -> Self {
        CommandBuffer {
            native_command_buffer: (),
            commands: Vec::new(),
            is_recording: false,
        }
    }

    pub fn begin_recording(&mut self) -> Result<(), &'static str> {
        if self.is_recording {
            return Err("Komut arabelleği zaten kayıtta.");
        }
        self.commands.clear();
        self.is_recording = true;
        println!("Komut arabelleği kaydı başladı (platforma özel işlemler gerekebilir).");
        Ok(())
    }

    pub fn end_recording(&mut self) -> Result<(), &'static str> {
        if !self.is_recording {
            return Err("Komut arabelleği kayıtta değil.");
        }
        self.is_recording = false;
        println!("Komut arabelleği kaydı sona erdi (platforma özel işlemler gerekebilir).");
        Ok(())
    }

    pub fn submit_command(&mut self, command: RenderCommand) -> Result<(), &'static str> {
        if !self.is_recording {
            return Err("Kayıtta olmayan bir komut arabelleğine komut gönderilemez.");
        }
        self.commands.push(command);
        println!("Gönderilen komut: {:?}", command);
        Ok(())
    }

    // Örnek özel komut gönderme metotları
    pub fn clear_color(&mut self, color: crate::core::color::Color8Bit) -> Result<(), &'static str> {
        self.submit_command(RenderCommand::Clear8Bit { color })
    }

    pub fn draw(&mut self, vertex_count: u32) -> Result<(), &'static str> {
        self.submit_command(RenderCommand::Draw { vertex_count })
    }

    pub fn set_viewport(&mut self, x: u32, y: u32, width: u32, height: u32) -> Result<(), &'static str> {
        self.submit_command(RenderCommand::SetViewport { x, y, width, height })
    }

    // İhtiyaca göre RenderCommand enum'ındaki diğer varyantlar için de metotlar eklenebilir.

    // Kaydedilen komutları almak için bir metot (platforma özel arka uç tarafından işlenmek üzere)
    pub fn get_commands(&self) -> &[RenderCommand] {
        &self.commands
    }
}