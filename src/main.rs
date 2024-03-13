use std::sync::Arc;
use encase::ShaderType;
use winit::{
    event::{Event, KeyEvent, WindowEvent},
    keyboard::{Key, NamedKey},
    window::Window,
};

// const ZOOM_INCREMENT_FACTOR: f32 = 1.1;
// const CAMERA_POS_INCREMENT_FACTOR: f32 = 0.1;

#[derive(Debug, ShaderType)]
struct AppState {
    time: i32,
}

impl AppState {
    fn as_wgsl_bytes(&self) -> encase::internal::Result<Vec<u8>> {
        let mut buffer = encase::UniformBuffer::new(Vec::new());
        buffer.write(self)?;
        Ok(buffer.into_inner())
    }
}

impl Default for AppState {
    fn default() -> Self {
        AppState {
            time: 0,
        }
    }
}

struct WgpuContext {
    pub window: Arc<Window>,
    pub surface: wgpu::Surface<'static>,
    pub surface_config: wgpu::SurfaceConfiguration,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub pipeline: wgpu::RenderPipeline,
    pub bind_group: wgpu::BindGroup,
    pub uniform_buffer: wgpu::Buffer,
}

impl WgpuContext {
    async fn new(window: Arc<Window>) -> WgpuContext {
        let size = window.inner_size();

        let instance = wgpu::Instance::default();
        let surface = instance.create_surface(window.clone()).unwrap(); // FOR STRUCTURE
        let adapter = instance.request_adapter(
            &wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            }
        ).await.unwrap();

        // BOTH FOR STRUCTURE
        let (device, queue) = adapter.request_device(
                &wgpu::DeviceDescriptor::default(),
                None,
            )
            .await.unwrap();

        let shader = device.create_shader_module(
            wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(std::borrow::Cow::Borrowed(include_str!("shader.wgsl"))),
            }
        );

        // FOR STRUCTURE
        let uniform_buffer = device.create_buffer(
            &wgpu::BufferDescriptor {
                label: None,
                size: std::mem::size_of::<AppState>() as u64,
                // COPY_DST is so data can be copied in when being initialized
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,                 
                mapped_at_creation: false,

            }
        );

        let bind_group_layout = device.create_bind_group_layout(
            &wgpu::BindGroupLayoutDescriptor {
                label: None,
                entries: &[
                    wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
                        ty: wgpu::BindingType::Buffer {
                            ty: wgpu::BufferBindingType::Uniform,
                            has_dynamic_offset: false,
                            min_binding_size: None,
                        },
                        count: None,
                    }
                ],
            }
        );
        let bind_group = device.create_bind_group(
            &wgpu::BindGroupDescriptor {
                label: None,
                layout: &bind_group_layout,
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: wgpu::BindingResource::Buffer(wgpu::BufferBinding {
                            buffer: &uniform_buffer,
                            offset: 0,
                            size: None,
                        }),
                    }
                ],
            }
        );

        let pipeline_layout = device.create_pipeline_layout(
            &wgpu::PipelineLayoutDescriptor {
                label: None,
                bind_group_layouts: &[&bind_group_layout],
                push_constant_ranges: &[], // IDK what the fuck this is
            }
        );

        let swapchain_capabilities = surface.get_capabilities(&adapter);
        let swapchain_format = swapchain_capabilities.formats[0];

        // FOR STRUCTURE
        let pipeline = device.create_render_pipeline(
            &wgpu::RenderPipelineDescriptor {
                label: None,
                layout: Some(&pipeline_layout),
                vertex: wgpu::VertexState {
                    module: &shader,
                    entry_point: "vs_main",
                    buffers: &[],
                },
                fragment: Some(wgpu::FragmentState {
                    module: &shader,
                    entry_point: "fs_main",
                    targets: &[Some(swapchain_format.into())],
                }),
                primitive: wgpu::PrimitiveState::default(),
                depth_stencil: None,
                multisample: wgpu::MultisampleState::default(),
                multiview: None,
            }
        );

        // FOR STRUCTURE
        let surface_config = surface
            .get_default_config(&adapter, size.width, size.height)
            .unwrap();
        surface.configure(&device, &surface_config);

        WgpuContext {
            window,
            surface,
            surface_config,
            device,
            queue,
            pipeline,
            bind_group,
            uniform_buffer,
        }
    }

    fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        self.surface_config.width = new_size.width;
        self.surface_config.height = new_size.height;
        self.surface.configure(&self.device, &self.surface_config);
        self.window.request_redraw();
    }
}

fn keyboard_input(state_mut: &mut AppState, wgpu_context_ref: &WgpuContext, logical_key: Key, text: Option<winit::keyboard::SmolStr>) {
    if let Key::Named(key) = logical_key {
        match key {
            NamedKey::Escape => wgpu_context_ref.window.request_redraw(),
            // NamedKey::ArrowUp => state_mut.translate_view(1, 1),
            // NamedKey::ArrowDown => state_mut.translate_view(-1, 1),
            // NamedKey::ArrowLeft => state_mut.translate_view(-1, 0),
            // NamedKey::ArrowRight => state_mut.translate_view(1, 0),
            _ => {}
        }
    }

    if let Some(text) = text {
        // if text == "u" {
        //     state_mut.max_iterations += 3;
        // } else if text == "d" {
        //     state_mut.max_iterations -= 3;
        // }
    };
}

fn mousewheel(state_mut: &mut AppState, change: f32) {
    // state_mut.zoom(change);
}

fn draw() {

}

async fn run(event_loop: winit::event_loop::EventLoop<()>, window: Arc<Window>) {
    let mut wgpu_context = Some(WgpuContext::new(window.clone()).await);

    let mut state = Some(AppState::default());
    let main_window_id = wgpu_context.as_ref().unwrap().window.id();

    event_loop
        .run(move |event, target| {

            match event {
                Event::LoopExiting => {
                    wgpu_context = None;
                    state = None;
                }
                Event::WindowEvent { window_id, event } if window_id == main_window_id => {
                    match event {
                        WindowEvent::CloseRequested => {
                            target.exit();
                        }
                        WindowEvent::KeyboardInput {
                            event:
                                KeyEvent {
                                    logical_key, text, ..
                                },
                            ..
                        } => {
                            let state_mut = state.as_mut().unwrap();
                            let wgpu_context_ref = wgpu_context.as_ref().unwrap();

                            keyboard_input(state_mut, wgpu_context_ref, logical_key, text);

                            wgpu_context_ref.window.request_redraw();
                        }
                        WindowEvent::MouseWheel { delta, .. } => {
                            let change = match delta {
                                winit::event::MouseScrollDelta::LineDelta(_, vertical) => vertical,
                                winit::event::MouseScrollDelta::PixelDelta(pos) => {
                                    pos.y as f32 / 20.0
                                }
                            };
                            let state_mut = state.as_mut().unwrap();
                            let wgpu_context_ref = wgpu_context.as_ref().unwrap();
    
                            mousewheel(state_mut, change);

                            wgpu_context_ref.window.request_redraw();
                        }
                        WindowEvent::Resized(new_size) => {
                            let wgpu_context_mut = wgpu_context.as_mut().unwrap();
                            wgpu_context_mut.resize(new_size);
                            wgpu_context_mut.window.request_redraw();
                        }
                        WindowEvent::RedrawRequested => {
                            println!("Redraw requested");

                            let wgpu_context_ref = wgpu_context.as_ref().unwrap();

                            let state_ref = state.as_mut().unwrap();
                            state_ref.time += 1;

                            let frame = wgpu_context_ref.surface.get_current_texture().unwrap();
                            let view = frame
                                .texture
                                .create_view(&Default::default()); // default is of TextureViewDescriptor

                            wgpu_context_ref.queue.write_buffer(
                                &wgpu_context_ref.uniform_buffer,
                                0,
                                &state_ref.as_wgsl_bytes().expect(
                                    "Error in encase translating AppState \
                    struct to WGSL bytes.",
                                ),
                            );

                            let mut encoder = wgpu_context_ref.device.create_command_encoder(
                                &wgpu::CommandEncoderDescriptor { label: None },
                            );

                            {
                                let mut render_pass =
                                    encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                                        label: None,
                                        color_attachments: &[Some(
                                            wgpu::RenderPassColorAttachment {
                                                view: &view,
                                                resolve_target: None,
                                                ops: wgpu::Operations {
                                                    load: wgpu::LoadOp::Clear(wgpu::Color { r: 0.1, g: 0.2, b: 0.3, a: 1.0 }),
                                                    store: wgpu::StoreOp::Store,
                                                },
                                            },
                                        )],
                                        depth_stencil_attachment: None,
                                        occlusion_query_set: None,
                                        timestamp_writes: None,
                                    });
                                render_pass.set_pipeline(&wgpu_context_ref.pipeline);
                                // (9)
                                render_pass.set_bind_group(0, &wgpu_context_ref.bind_group, &[]);
                                render_pass.draw(0..3, 0..1);
                            }
                            wgpu_context_ref.queue.submit(Some(encoder.finish()));
                            frame.present();

                            window.request_redraw();

                        }
                        _ => {}
                    }
                }
                _ => {}
            }
        })
        .unwrap();
}

pub fn main() {
    let event_loop = winit::event_loop::EventLoop::new().unwrap();
    #[allow(unused_mut)]
    let mut builder = winit::window::WindowBuilder::new()
        .with_inner_size(winit::dpi::LogicalSize::new(900, 900));

    // #[cfg(target_arch = "wasm32")]
    // {
    //     use wasm_bindgen::JsCast;
    //     use winit::platform::web::WindowBuilderExtWebSys;
    //     let canvas = web_sys::window()
    //         .unwrap()
    //         .document()
    //         .unwrap()
    //         .get_element_by_id("canvas")
    //         .unwrap()
    //         .dyn_into::<web_sys::HtmlCanvasElement>()
    //         .unwrap();
    //     builder = builder.with_canvas(Some(canvas));
    // }
    let window = builder.build(&event_loop).unwrap();

    let window = Arc::new(window);
    #[cfg(not(target_arch = "wasm32"))]
    {
        env_logger::builder().format_timestamp_nanos().init();
        pollster::block_on(run(event_loop, window));
    }
//     #[cfg(target_arch = "wasm32")]
//     {
//         std::panic::set_hook(Box::new(console_error_panic_hook::hook));
//         console_log::init().expect("could not initialize logger");
//
//         let document = web_sys::window()
//             .and_then(|win| win.document())
//             .expect("Failed to get document.");
//         let body = document.body().unwrap();
//         let controls_text = document
//             .create_element("p")
//             .expect("Failed to create controls text as element.");
//         controls_text.set_inner_html(
//             "Controls: <br/>
// Up, Down, Left, Right: Move view, <br/>
// Scroll: Zoom, <br/>
// U, D: Increase / decrease sample count.",
//         );
//         body.append_child(&controls_text)
//             .expect("Failed to append controls text to body.");
//
//         wasm_bindgen_futures::spawn_local(run(event_loop, window));
//     }
}
