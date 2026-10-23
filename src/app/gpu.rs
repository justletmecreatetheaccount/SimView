use pollster::FutureExt;
use wgpu::{
    Adapter, Backends, Device, DeviceDescriptor, InstanceDescriptor, Queue, RequestAdapterOptions,
    Surface, SurfaceConfiguration, TextureFormat, TextureUsages, Trace,
};

use crate::app::window::WindowManager;

pub struct Gpu {
    surface: Surface<'static>,
    adapter: Adapter,
    device: Device,
    queue: Queue,
    surface_config: SurfaceConfiguration,
    preferred_format: TextureFormat,
}

impl Gpu {
    pub fn new(window: &WindowManager) -> Gpu {
        let instance = wgpu::Instance::new(InstanceDescriptor {
            backends: Backends::PRIMARY,
            flags: Default::default(),
            memory_budget_thresholds: Default::default(),
            backend_options: Default::default(),
            display: None,
        });

        // Unsafe required to keep mutability of window while wgpu keeps a borrow of window
        // Rust makes it impossible to do otherwise while keeping current structure, at least I don't see how
        let surface = unsafe {
            instance
                .create_surface_unsafe(
                    wgpu::SurfaceTargetUnsafe::from_display_and_window(
                        window.get_glfw_window(),
                        window.get_glfw_window(),
                    )
                    .expect("Can't create surface target from window"),
                )
                .expect("Can't create a surface, find out why")
        };

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

        let caps = surface.get_capabilities(&adapter);
        let preferred_format = caps
            .formats
            .iter()
            .find(|f| f.is_srgb())
            .copied()
            .unwrap_or(caps.formats[0]);

        let surface_configuration = SurfaceConfiguration {
            usage: TextureUsages::RENDER_ATTACHMENT,
            format: preferred_format,
            color_space: wgpu::SurfaceColorSpace::Auto,
            width: window.get_window_size().0,
            height: window.get_window_size().1,
            present_mode: wgpu::PresentMode::Fifo,
            desired_maximum_frame_latency: Default::default(),
            alpha_mode: caps.alpha_modes[0],
            view_formats: vec![],
        };

        surface.configure(&device, &surface_configuration);

        Gpu {
            surface,
            adapter,
            device,
            queue,
            surface_config: surface_configuration,
            preferred_format,
        }
    }

    pub fn preferred_format(&self) -> TextureFormat {
        self.preferred_format
    }

    pub fn surface(&self) -> &Surface<'_> {
        &self.surface
    }

    pub fn device(&self) -> &Device {
        &self.device
    }

    pub fn queue(&self) -> &Queue {
        &self.queue
    }

    pub fn configure_surface(&self) {
        self.surface.configure(&self.device, &self.surface_config);
    }
}
