#![allow(unused_macros)]

use std::iter;
use winit::{
    event::*,
    event_loop::EventLoop,
    keyboard::{KeyCode, PhysicalKey},
    window::{Window, WindowBuilder},
};

macro_rules! is_key {
    ($key:pat, $repeated:literal) => {
        WindowEvent::KeyboardInput {
            event: KeyEvent {
                physical_key: PhysicalKey::Code($key),
                repeat: $repeated,
                ..
            },
            ..
        }
    };
    ($key:pat, $state:pat, $repeated:literal) => {
        WindowEvent::KeyboardInput {
            event: KeyEvent {
                physical_key: PhysicalKey::Code($key),
                state: $state,
                repeat: $repeated,
                ..
            },
            ..
        }
    };
}
macro_rules! is_key_pressed {
    ($key:pat) => {
        is_key!($key, ElementState::Pressed, false)
    };
}
macro_rules! is_key_released {
    ($key:pat) => {
        is_key!($key, ElementState::Released, false)
    };
}
macro_rules! is_key_repeated {
    ($key:pat) => {
        is_key!($key, true)
    };
}

struct Game<'a> {
    surf: wgpu::Surface<'a>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    size: winit::dpi::PhysicalSize<u32>,
    window: &'a Window,
    bg : wgpu::Color
}

impl<'a> Game<'a> {
    async fn new(window: &'a Window) -> Game<'a> {
        let size = window.inner_size();

        let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor {
            backends: wgpu::Backends::GL,
            ..Default::default()
        });

        let surface = instance.create_surface(window).unwrap();

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .unwrap();

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: None,
                    required_limits: wgpu::Limits::default(),
                    memory_hints: Default::default(),
                    required_features: wgpu::Features::empty(),
                },
                None,
            )
            .await
            .unwrap();

        let surf_caps = surface.get_capabilities(&adapter);

        let surface_format = surf_caps
            .formats
            .iter()
            .find(|f| f.is_srgb())
            .copied()
            .unwrap_or(surf_caps.formats[0]);

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: surf_caps.present_modes[0],
            alpha_mode: surf_caps.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };

        Self {
            config,
            device,
            queue,
            surf: surface,
            window,
            size,
            bg: wgpu::Color{r:0.0,g:0.0,b:0.0,a:1.0}
        }
    }
    
    fn window(&self) -> &Window {
        &self.window
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.size = new_size;
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.surf.configure(&self.device, &self.config);
        }
    }

    #[allow(unused_variables)]
    fn input(&mut self, event: &WindowEvent) -> bool {
        false
    }

    fn update(&mut self) {}

    fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        let output = self.surf.get_current_texture()?;
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

        {
            let _render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(self.bg),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                occlusion_query_set: None,
                timestamp_writes: None,
            });
        }
        self.queue.submit(iter::once(encoder.finish()));
        output.present();
        Ok(())
    }
}

pub async fn run() {
    env_logger::init();

    let event_loop = EventLoop::new().unwrap();
    let window = WindowBuilder::new().build(&event_loop).unwrap();
    let mut state = Game::new(&window).await;
    let mut surface_configured = false;

    event_loop
        .run(move |event, control_flow| {
            match event {
                Event::WindowEvent {
                    ref event,
                    window_id,
                } if window_id == state.window().id() => {
                    if !state.input(event) {
                        match event {
                            WindowEvent::CloseRequested | is_key_released!(KeyCode::Escape) => {
                                control_flow.exit()
                            }
                            is_key_repeated!(KeyCode::KeyA) => {
                                state.bg.g -= 1.0 / 60.0;
                            }
                            is_key_repeated!(KeyCode::KeyD) => {
                                state.bg.g += 1.0 / 60.0;
                            }
                            is_key_repeated!(KeyCode::KeyW) => {
                                state.bg.b -= 1.0 / 60.0;
                            }
                            is_key_repeated!(KeyCode::KeyS) => {
                                state.bg.b += 1.0 / 60.0;
                            }
                            WindowEvent::Resized(physical_size) => {
                                surface_configured = true;
                                state.resize(*physical_size);
                            }
                            WindowEvent::RedrawRequested => {
                                state.window().request_redraw();
                                if !surface_configured {
                                    return;
                                }

                                state.update();
                                // Skiping Error Handling
                                let _ = state.render();
                            }
                            _ => {}
                        }
                    }
                }
                _ => {}
            }
        })
        .unwrap();
}
