use vulkano::swapchain::Surface;
use vulkano::instance::Instance;
use vulkano::device::{Device, Queue};
use winit::{EventsLoop, Window};

use std::sync::Arc;

fn main() {
    let mut events_loop = EventsLoop::new();
    let instance = build_vk_instance();
    let surface = build_vk_surface(instance.clone(), &events_loop);
    let (device, queue) = pick_queue(instance.clone(), surface.clone());
    let window = surface.window();
    let dimensions = initial_physical_dimensions(&window);
    // let (mut swapchain, mut images) = {
    //     let caps = surface.capabilities(gpu)
    //         .expect("Failed to get surface capabilities");
        
    // };
    println!("{:?}", device);
    println!("{:?}", queue);
    println!("{:?}", dimensions);

    events_loop.run_forever(|event| {
        match event {
            winit::Event::WindowEvent { event: winit::WindowEvent::CloseRequested, window_id }
            if window_id == window.id() => winit::ControlFlow::Break,
            _ => winit::ControlFlow::Continue
        }
    });
}

fn build_vk_instance() -> Arc<Instance> {
    Instance::new(None, &vulkano_win::required_extensions(), None).unwrap()
}

fn build_vk_surface(instance: Arc<Instance>, events_loop: &EventsLoop) -> Arc<Surface<Window>> {
    use vulkano_win::VkSurfaceBuild;
    winit::WindowBuilder::new()
        .build_vk_surface(events_loop, instance)
        .expect("创建窗口")
}

fn pick_queue(instance: Arc<Instance>, surface: Arc<Surface<Window>>) -> (Arc<Device>, Arc<Queue>) {
    use vulkano::device::DeviceExtensions;
    for gpu in vulkano::instance::PhysicalDevice::enumerate(&instance) {
        println!("发现已安装的GPU！名称: {} 种类: {:?}", gpu.name(), gpu.ty());
    }
    let gpu = vulkano::instance::PhysicalDevice::enumerate(&instance).next()
        .expect("你电脑上没gpu呀，这咋整啊");
    println!("使用GPU：名称: {} 种类: {:?}", gpu.name(), gpu.ty());

    for family in gpu.queue_families() {
        println!("发现queue family！包含的queue数量： {:?} ", family.queues_count());
    }
    let queue_family = gpu.queue_families()
        .find(|&q| q.supports_graphics() && surface.is_supported(q).unwrap_or(false))
        .unwrap();
    println!("使用queue family: {:?}", queue_family);

    let (device, mut queues) = {
        let device_ext = DeviceExtensions { khr_swapchain: true, .. DeviceExtensions::none() };
        Device::new(gpu, gpu.supported_features(), &device_ext,
            [(queue_family, 0.5)].iter().cloned()).unwrap()
    };
    let queue = queues.next().unwrap();
    (device, queue)
}

fn initial_physical_dimensions(window: &Window) -> [u32; 2] {
    if let Some(dimensions) = window.get_inner_size() { 
        let dimensions: (u32, u32) = dimensions.to_physical(window.get_hidpi_factor()).into();
        [dimensions.0, dimensions.1]
    } else {
        unreachable!("No dimensions in window. This is a bug!")
    }
}
