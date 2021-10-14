use winit::window::Window;

use crate::SWAPCHAIN_FORMAT;

pub struct Manager {
    pub instance: wgpu::Instance,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub size: winit::dpi::PhysicalSize<u32>,
    pub surface: wgpu::Surface,
    pub sc_desc: wgpu::SwapChainDescriptor,
    pub swap_chain: wgpu::SwapChain,
}

#[cfg(target_os = "windows")]
const DEFAULT_BACKEND: wgpu::BackendBit = wgpu::BackendBit::DX12;

#[cfg(target_os = "macos")]
const DEFAULT_BACKEND: wgpu::BackendBit = wgpu::BackendBit::METAL;

#[cfg(target_os = "linux")]
const DEFAULT_BACKEND: wgpu::BackendBit = wgpu::BackendBit::VULKAN;

impl Manager {
    pub fn new(window: &Window, trace_path: Option<&std::path::Path>, backend_override: Option<wgpu::BackendBit>) -> Self {
        use futures::executor::block_on;
        let instance = wgpu::Instance::new(backend_override.unwrap_or(DEFAULT_BACKEND));

        let (size, surface) = unsafe {
            let size = window.inner_size();
            let surface = instance.create_surface(window);
            (size, surface)
        };

        let adapter_future = instance.request_adapter(
            &wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::LowPower, // TODO: HighPower caused an issue on at least one AMD discrete GPU card
                compatible_surface: Some(&surface),
            }
        );
        let adapter = block_on(adapter_future).expect("unable to open an adapter");

        let device_future = adapter.request_device(
            &wgpu::DeviceDescriptor {
                label: Some("requested device"),
                features: wgpu::Features::MAPPABLE_PRIMARY_BUFFERS,
                limits: wgpu::Limits {
                    max_storage_buffers_per_shader_stage: 6, // TODO: we need to make sure that every possible GPU supports this
                    .. Default::default()
                },
            },
            trace_path,
        );
        let (device, queue) = block_on(device_future).expect("unable to get a device and a queue");

        let sc_desc = wgpu::SwapChainDescriptor {
            usage: wgpu::TextureUsage::RENDER_ATTACHMENT,
            format: SWAPCHAIN_FORMAT,
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Fifo,
        };
        let swap_chain = Self::create_swapchain(&device, &surface, &sc_desc);

        Self {
            device,
            instance,
            queue,
            size,
            surface,
            swap_chain,
            sc_desc,
        }
    }

    fn create_swapchain(device: &wgpu::Device, surface: &wgpu::Surface, swapchain_descriptor: &wgpu::SwapChainDescriptor) -> wgpu::SwapChain {
        device.create_swap_chain(surface, swapchain_descriptor)
    }
}
