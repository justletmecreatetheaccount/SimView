use std::sync::Arc;

use glfw::PWindow;
use pollster::FutureExt;
use wgpu::{
    Backends, InstanceDescriptor, RequestAdapterOptions, SurfaceConfiguration, TextureUsages,
    Trace, wgt::DeviceDescriptor,
};

use crate::app::window::WindowManager;

//This struct is not separated in two because of the borrow checker
pub struct State {
    // Window
    window: Arc<PWindow>, //Share window ownership with surface
    events: glfw::GlfwReceiver<(f64, glfw::WindowEvent)>,
    window_size: (u32, u32),
    window_title: String,

    //Gpu
    surface: wgpu::Surface<'static>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    is_surface_configured: bool,
}

impl State {
    pub fn new() -> State {
        use glfw::fail_on_errors;
        let mut glfw = glfw::init(fail_on_errors!()).unwrap();

        // Create a windowed mode window
        let (mut glfw_window, events) = glfw
            .create_window(680, 480, "Hello this is window", glfw::WindowMode::Windowed)
            .expect("Failed to create GLFW window.");

        glfw_window.set_key_polling(true);

        let window = Arc::new(glfw_window);

        let instance = wgpu::Instance::new(InstanceDescriptor {
            backends: Backends::PRIMARY,
            flags: Default::default(),
            memory_budget_thresholds: Default::default(),
            backend_options: Default::default(),
            display: None,
        });

        let surface = instance
            .create_surface(window.clone())
            .expect("Can't create a surface find out why");

        let adapter = instance
            .request_adapter(&RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::LowPower,
                force_fallback_adapter: Default::default(),
                compatible_surface: Some(&surface),
                apply_limit_buckets: Default::default(),
            })
            .block_on()
            .expect("No compatible Adapter, cry");

        // let adapter = instance.request_adapter(RequestAdapterOptions {});

        println!("Chosan adapah : {:?}", adapter.get_info());

        let (device, queue) = adapter
            .request_device(&DeviceDescriptor {
                label: Some("LowPower Rendering Device"),
                required_features: Default::default(),
                required_limits: Default::default(),
                experimental_features: Default::default(),
                memory_hints: wgpu::MemoryHints::MemoryUsage,
                trace: Trace::Off,
            })
            .block_on()
            .expect("Can't create a Device and Queue, don't know how that could happen");

        let surface_configuration = SurfaceConfiguration {
            usage: TextureUsages::RENDER_ATTACHMENT,
            format: wgpu::TextureFormat::Bgra8Unorm,
            color_space: wgpu::SurfaceColorSpace::Auto,
            width: window.get_window_size().0,
            height: window.get_window_size().1,
            present_mode: wgpu::PresentMode::Fifo,
            desired_maximum_frame_latency: Default::default(),
            alpha_mode: wgpu::CompositeAlphaMode::Opaque,
            view_formats: vec![wgpu::TextureFormat::Bgra8Unorm],
        };

        surface.configure(&device, &surface_configuration);

        State {
            surface,
            device,
            queue,
            config: surface_configuration,
            is_surface_configured: true,
        }
    }
}
