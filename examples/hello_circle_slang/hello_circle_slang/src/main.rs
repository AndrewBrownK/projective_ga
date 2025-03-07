use std::error::Error;
use std::fmt::Debug;
use std::sync::Arc;

use parking_lot::Mutex;
use wgpu::util::{BufferInitDescriptor, DeviceExt};
use wgpu::{include_spirv, include_wgsl, BindGroupDescriptor, BindGroupLayoutDescriptor, BindGroupLayoutEntry, BufferUsages, Instance, InstanceDescriptor, PipelineCompilationOptions, SurfaceTargetUnsafe};
use winit::application::ApplicationHandler;
use winit::dpi::PhysicalSize;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::window::{Window, WindowId};

fn main() -> Result<(), Box<dyn Error>> {
    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Poll);
    let mut app = App::new();
    event_loop.run_app(&mut app).unwrap();
    Ok(())
}

struct App {
    window: Option<Window>,
    handle_redraw: Box<dyn FnMut(&ActiveEventLoop, WindowId, WindowEvent)>,
    size: Arc<Mutex<PhysicalSize<u32>>>,
}
impl App {
    fn new() -> Self {
        App {
            window: None,
            handle_redraw: Box::new(|_, _, _| {}),
            size: Arc::new(Mutex::new(PhysicalSize { width: 1u32, height: 1u32 })),
        }
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        self.start_wgpu_if_necessary(&event_loop);
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::RedrawRequested => {
                self.handle_redraw.as_mut()(event_loop, id, event);
                self.window.as_ref().unwrap().request_redraw()
            }
            WindowEvent::Resized(new_size) => {
                let mut size = self.size.lock();
                *size = new_size;
            }
            _ => (),
        }
    }
}

impl App {
    fn start_wgpu_if_necessary(&mut self, event_loop: &ActiveEventLoop) {
        if self.window.is_some() {
            return;
        }
        pollster::block_on(self.initiate(event_loop));
    }

    async fn initiate(&mut self, event_loop: &ActiveEventLoop) {
        let window = event_loop.create_window(Default::default()).unwrap();
        let instance = Instance::new(InstanceDescriptor::default());
        let surface = unsafe { instance.create_surface_unsafe(SurfaceTargetUnsafe::from_window(&window).unwrap()).unwrap() };
        let adapter = instance.request_adapter(&Default::default()).await.unwrap();
        let (device, queue) = adapter.request_device(&Default::default(), None).await.unwrap();
        let window_size = window.inner_size();
        self.size = Arc::new(Mutex::new(window_size.clone()));

        let bg_layout = device.create_bind_group_layout(&BindGroupLayoutDescriptor {
            label: None,
            entries: &[BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
        });

        let screen_ratio_buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: None,
            contents: bytemuck::cast_slice(&[window_size.width as f32, window_size.height as f32]),
            usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
        });
        let bg = device.create_bind_group(&BindGroupDescriptor {
            label: None,
            layout: &bg_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: wgpu::BindingResource::Buffer(screen_ratio_buffer.as_entire_buffer_binding()),
            }],
        });

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: &[&bg_layout],
            push_constant_ranges: &[],
        });

        let swapchain_capabilities = surface.get_capabilities(&adapter);
        let swapchain_format = swapchain_capabilities.formats[0];

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: None,
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &device.create_shader_module(include_spirv!("../res/shader.vs.spv")),
                entry_point: Some("vs_main"),
                compilation_options: PipelineCompilationOptions::default(),
                buffers: &[],
            },
            fragment: Some(wgpu::FragmentState {
                module: &device.create_shader_module(include_spirv!("../res/shader.fs.spv")),
                entry_point: Some("fs_main"),
                compilation_options: PipelineCompilationOptions::default(),
                targets: &[Some(swapchain_format.into())],
            }),
            primitive: wgpu::PrimitiveState::default(),
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            multiview: None,
            cache: None,
        });

        let mut surface_configuration = surface.get_default_config(&adapter, window_size.width, window_size.height).unwrap();
        surface.configure(&device, &surface_configuration);

        window.request_redraw();
        self.window = Some(window);
        let window_size = self.size.clone();

        self.handle_redraw = Box::new(move |event_loop, id, event| {
            let window_size = window_size.lock();
            if window_size.width != surface_configuration.width || window_size.height != surface_configuration.height {
                surface_configuration.width = window_size.width;
                surface_configuration.height = window_size.height;
                surface.configure(&device, &surface_configuration);

                queue.write_buffer(&screen_ratio_buffer, 0, bytemuck::cast_slice(&[window_size.width as f32, window_size.height as f32]))
            }
            drop(window_size);

            let frame = surface.get_current_texture().expect("Failed to acquire next swap chain texture");
            let view = frame.texture.create_view(&wgpu::TextureViewDescriptor::default());
            let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
            {
                let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                    label: None,
                    color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                        view: &view,
                        resolve_target: None,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                            store: wgpu::StoreOp::Store,
                        },
                    })],
                    depth_stencil_attachment: None,
                    timestamp_writes: None,
                    occlusion_query_set: None,
                });
                rpass.set_pipeline(&render_pipeline);
                rpass.set_bind_group(0, &bg, &[]);

                // Render a single triangle takes up the entire screen.
                // Actual round objects will be rendered in the fragment shader.
                rpass.draw(0..3, 0..1);
            }

            queue.submit(Some(encoder.finish()));
            frame.present();
        });
    }
}
