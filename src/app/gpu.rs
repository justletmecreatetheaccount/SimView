use pollster::FutureExt;
use wgpu::{
    Adapter, Backends, Features, FeaturesWGPU, Instance, InstanceDescriptor, RequestAdapterOptions,
    SurfaceConfiguration, TextureUsages, Trace, wgt::DeviceDescriptor,
};

use crate::app::window::WindowManager;

pub struct Gpu<'window> {
    surface: wgpu::Surface<'window>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    is_surface_configured: bool,
}

impl Gpu {
    pub fn new<'window>(window: &'window WindowManager) -> Gpu {
        let instance = wgpu::Instance::new(InstanceDescriptor {
            backends: Backends::PRIMARY,
            flags: Default::default(),
            memory_budget_thresholds: Default::default(),
            backend_options: Default::default(),
            display: None,
        });

        let surface = instance
            .create_surface(window.get_glfw_window())
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

        Gpu {
            surface,
            device,
            queue,
            config: surface_configuration,
            is_surface_configured: true,
        }
    }
}
