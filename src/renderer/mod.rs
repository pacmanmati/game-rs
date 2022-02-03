use wgpu::*;

pub mod attachment;
pub mod node;
pub mod scene;
pub struct Renderer {
    pub surface: Surface,
    pub adapter: Adapter,
    pub device: Device,
    pub queue: Queue,
    pub encoder: CommandEncoder,
    pub config: SurfaceConfiguration,
}

impl Renderer {
    pub fn new<S>(window_handle: S, window_height: u32, window_width: u32) -> Self
    where
        S: raw_window_handle::HasRawWindowHandle,
    {
        let instance = Instance::new(Backends::PRIMARY);
        let (surface, adapter, device, queue) = pollster::block_on(async {
            let surface = unsafe { instance.create_surface(&window_handle) };
            let adapter = instance
                .request_adapter(&RequestAdapterOptions {
                    compatible_surface: Some(&surface),
                    ..Default::default()
                })
                .await
                .expect("No suitable adapter found.");
            let (device, queue) = adapter
                .request_device(
                    &DeviceDescriptor {
                        label: "Device".into(),
                        // see if it's worth doing this instead of defaulting:
                        // limits: adapter.limits(),
                        // features: adapter.features(),
                        ..Default::default()
                    },
                    None,
                )
                .await
                .expect("No suitable device found.");
            (surface, adapter, device, queue)
        });

        let config = SurfaceConfiguration {
            usage: TextureUsages::RENDER_ATTACHMENT,
            format: surface
                .get_preferred_format(&adapter)
                .expect("Surface is incompatible with this adapter."),
            width: window_width,
            height: window_height,
            present_mode: PresentMode::Immediate,
        };

        surface.configure(&device, &config);

        let encoder = device.create_command_encoder(&CommandEncoderDescriptor {
            label: Some("Command Encoder"),
        });

        Self {
            surface,
            adapter,
            device,
            queue,
            encoder,
            config,
        }
    }

    pub fn resize_surface(&mut self, height: u32, width: u32) {
        self.config.height = height;
        self.config.width = width;
        self.surface.configure(&self.device, &self.config);
    }

    pub fn setup_mesh_pipeline() {}

    pub fn setup_ui_pipeline() {}

    pub fn render_ui() {}
}
