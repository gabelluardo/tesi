fn main() -> Result<()> {
    let instance = Instance::new(
        VulkanLibrary::new()?,
        InstanceCreateInfo::application_from_cargo_toml(),
    )?;

    let device_extensions = DeviceExtensions {
        khr_storage_buffer_storage_class: true,
        khr_shader_float_controls: true,
        nv_compute_shader_derivatives: true,
        ..DeviceExtensions::empty()
    };
    let device_features = Features {
        shader_float64: true,
        ..Features::empty()
    };

    let (physical_device, queue_family_index) = instance
        .enumerate_physical_devices()?
        .filter(|p| {
            p.supported_extensions().contains(&device_extensions)
                && p.supported_features().contains(&device_features)
        })
        .filter_map(|p| {
            p.queue_family_properties()
                .iter()
                .position(|q| q.queue_flags.contains(QueueFlags::COMPUTE))
                .map(|i| (p, i as u32))
        })
        .min_by_key(|(p, _)| match p.properties().device_type {
            PhysicalDeviceType::DiscreteGpu => 0,
            PhysicalDeviceType::IntegratedGpu => 1,
            PhysicalDeviceType::VirtualGpu => 2,
            PhysicalDeviceType::Cpu => 3,
            PhysicalDeviceType::Other => 4,
            _ => 5,
        })
        .ok_or(anyhow!("No suitable physical device detected"));

    let (device, mut queues) = Device::new(
        physical_device,
        DeviceCreateInfo {
            enabled_features: device_features,
            enabled_extensions: device_extensions,
            queue_create_infos: vec![QueueCreateInfo {
                queue_family_index,
                ..Default::default()
            }],
            ..Default::default()
        },
    )?;

    let queue = queues.next().unwrap();

    ...
}