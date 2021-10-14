mod device_manager;

use std::env;
use winit::event_loop::EventLoop;

pub const SWAPCHAIN_FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Bgra8UnormSrgb;

#[allow(unused)]
#[derive(Debug)]
pub enum CustomEvent {
    NewFile,
    ShowOpenDialog,
    OpenFile(std::path::PathBuf),
    SaveFile(std::path::PathBuf),
    ExportPng(std::path::PathBuf),
    RequestExit,
    MouseFreeze,
    MouseThaw,
}

fn main() {
    let matches = clap::App::new("Franzplot")
        .version(clap::crate_version!())
        .author("Francesco Cattoglio")
        .about("a tool to create and visualize parametric curves and surfaces")
        .arg(clap::Arg::with_name("INPUT")
            .help("Open an existing file instead of starting from a new one")
            .required(false)
            .index(1))
        .arg(clap::Arg::with_name("tracing")
            .short("t")
            .long("tracing")
            .value_name("TRACE_PATH")
            .help("Sets a path for tracing output")
            .takes_value(true))
        .arg(clap::Arg::with_name("backend")
            .short("b")
            .long("backend")
            .value_name("WGPU_BACKEND")
            .help("Chose a different backend from the standard one")
            .takes_value(true))
        .arg(clap::Arg::with_name("p")
            .short("p")
            .multiple(true)
            .help("Pause the execution before creating a device. Useful for attaching a debugger to a process. Each p is 5 secs"))
        .get_matches();

    //wgpu_subscriber::initialize_default_subscriber(None);

    use std::{thread, time};
    let seconds_to_wait = 5 * matches.occurrences_of("p");
    thread::sleep(time::Duration::from_secs(seconds_to_wait));

    let maybe_tracing_path = matches.value_of("tracing");
    let tracing_path_option = maybe_tracing_path.map(|x| std::path::Path::new(x));

    let maybe_backend = matches.value_of("backend").map(|name| {
        match name.to_lowercase().as_str()  {
            "vulkan" => wgpu::BackendBit::VULKAN,
            "metal" => wgpu::BackendBit::METAL,
            "dx12" => wgpu::BackendBit::DX12,
            "dx11" => wgpu::BackendBit::DX11,
            "gl" => wgpu::BackendBit::GL,
            "webgpu" => wgpu::BackendBit::BROWSER_WEBGPU,
            other => panic!("Unknown backend: {}", other),
        }
    });

    let event_loop = EventLoop::<CustomEvent>::with_user_event();
    let builder = winit::window::WindowBuilder::new();
    let window = builder.build(&event_loop).unwrap();

    window.set_min_inner_size(Some(winit::dpi::LogicalSize::new(200.0, 100.0)));
    let device_manager = device_manager::Manager::new(&window, tracing_path_option, maybe_backend);

    panic!("right after device manager creation");
}
