pub mod prelude {
    pub use crate::{SoftBufferOptions, SoftBufferPlugin, SoftBufferResource, SoftBufferStage};
}

pub use softbuffer;

use bevy::{
    diagnostic::{Diagnostic, DiagnosticId, Diagnostics},
    prelude::*,
    window::{WindowBackendScaleFactorChanged, WindowId, WindowResized},
    winit::WinitWindows,    
};
use softbuffer::GraphicsContext;
use std::time::Instant;

#[derive(Debug, Hash, PartialEq, Eq, Clone, StageLabel)]
pub enum SoftBufferStage {
    Draw,
    Render,    
}

#[derive(Debug, Clone)]
pub struct SoftBufferOptions {
    /// Width of the pixel buffer
    pub width: u32,
    /// Height of the pixel buffer
    pub height: u32,
}

impl Default for SoftBufferOptions {
    fn default() -> Self {
        SoftBufferOptions {
            width: 180,
            height: 120,
        }
    }
}

pub struct SoftBufferResource {
    pub buffer: Vec<u32>,    
    pub window_id: WindowId,    
}

pub struct SoftBufferPlugin;

impl Plugin for SoftBufferPlugin {
    fn build(&self, app: &mut App) {
        app.add_stage_after(
            CoreStage::PostUpdate,
            SoftBufferStage::Draw,
            SystemStage::parallel(),
        )
        .add_stage_after(
            SoftBufferStage::Draw,
            SoftBufferStage::Render,
            SystemStage::parallel(),
        )        
        .init_resource::<SoftBufferOptions>()
        .add_startup_system_to_stage(StartupStage::PreStartup, Self::setup)        
        .add_system_to_stage(SoftBufferStage::Render, Self::render);
    }
}

impl SoftBufferPlugin {
    pub const RENDER_TIME: DiagnosticId =
        DiagnosticId::from_u128(1187582084072339577959028643519383692);

    pub fn setup(
        mut commands: Commands,
        mut diagnostics: ResMut<Diagnostics>,
        options: Res<SoftBufferOptions>,
        windows: Res<Windows>,
        winit_windows: NonSend<WinitWindows>,
    ) {
        diagnostics.add(Diagnostic::new(Self::RENDER_TIME, "render_time", 20).with_suffix("s"));

        let window_id = windows
            .get_primary()
            .expect("primary window not found")
            .id();

        let winit_window = winit_windows
            .get_window(window_id)
            .expect("failed to get primary winit window");

        let window_size = winit_window.inner_size();
        let buffer = vec![0u32; window_size.width as usize * window_size.height as usize];

        let mut graphics_context = unsafe { GraphicsContext::new(winit_window) }.unwrap();

        commands.insert_resource(SoftBufferResource { buffer, window_id });
    }    

    pub fn render(resource: Res<SoftBufferResource>, mut diagnostics: ResMut<Diagnostics>) {
        let start = Instant::now();

        //resource.pixels.render().expect("failed to render pixels");

        let end = Instant::now();
        //let render_time = end.duration_since(start);
        //diagnostics.add_measurement(Self::RENDER_TIME, render_time.as_secs_f64());
    }
}
