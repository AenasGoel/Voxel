use vulkano::VulkanLibrary;
use vulkano::instance::{Instance, InstanceCreateInfo};
use vulkano::device::{Device, DeviceCreateInfo, QueueCreateInfo};

fn main() {
    println!("Hello, world!");
    let library = VulkanLibrary::new().expect("no local Vulkan library/DLL");
    let instance = Instance::new(library, InstanceCreateInfo::default()).expect("failed to create instance");
    let physical = instance
    .enumerate_physical_devices()
    .expect("could not enumerate devices")
    .next()
    .expect("no devices available");
    for family in physical.queue_family_properties() {
        println!("Found a queue family with {:?} queue(s)", family.queue_count);
    }
    let queue_family_index = physical
    .queue_family_properties()
    .iter()
    .enumerate()
    .position(|(_, q)| q.queue_flags.graphics)
    .expect("couldn't find a graphical queue family") as u32;
    let (device, mut queues) = Device::new(
        physical,
        DeviceCreateInfo {
            // here we pass the desired queue family to use by index
            queue_create_infos: vec![QueueCreateInfo {
                queue_family_index,
                ..Default::default()
            }],
            ..Default::default()
        },
    )
        .expect("failed to create device");

    let queue - queues.next().unwrap();


}
