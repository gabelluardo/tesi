fn main() -> Result<()> {
    // ...

    let vec_a = Vec::new();
    let vec_b = Vec::new();

    let memory_allocator = StandardMemoryAllocator::new_default(device.clone());
    let command_buffer_allocator =
        StandardCommandBufferAllocator::new(device.clone(), Default::default());

    let buf_a = Buffer::from_iter(
        &memory_allocator,
        BufferCreateInfo {
            usage: BufferUsage::STORAGE_BUFFER,
            ..Default::default()
        },
        AllocationCreateInfo {
            usage: MemoryUsage::Upload,
            ..Default::default()
        },
        vec_a.iter(),
    );
    let buf_b = Buffer::from_iter(
        &memory_allocator,
        BufferCreateInfo {
            usage: BufferUsage::STORAGE_BUFFER,
            ..Default::default()
        },
        AllocationCreateInfo {
            usage: MemoryUsage::Upload,
            ..Default::default()
        },
        vec_b.iter(),
    );
    let buf_res = Buffer::new_slice(
        &memory_allocator,
        BufferCreateInfo {
            usage: BufferUsage::STORAGE_BUFFER,
            ..Default::default()
        },
        AllocationCreateInfo {
            usage: MemoryUsage::Download,
            ..Default::default()
        },
        len as u64,
    )?;

    let descriptor_set_allocator = StandardDescriptorSetAllocator::new(device.clone());

    let shader = loader::vec_sum::load(device)?;
    let pipeline = ComputePipeline::new(
        device.clone(),
        shader
            .entry_point("main")
            .ok_or(anyhow!("No entry point in shader"))?,
        &(),
        None,
        |_| {},
    )?;
    let layout = pipeline.layout.set_layouts().get(0).unwrap();
    let set = PersistentDescriptorSet::new(
        &descriptor_set_allocator,
        layout.clone(),
        [
            WriteDescriptorSet::buffer(0, buf_a),
            WriteDescriptorSet::buffer(1, buf_b),
            WriteDescriptorSet::buffer(2, buf_res.clone()),
        ],
    )?;

    let push_constants = PushConstants { n: len as u32 };

    let mut builder = AutoCommandBufferBuilder::primary(
        &command_buffer_allocator,
        queue.queue_family_index(),
        CommandBufferUsage::OneTimeSubmit,
    )?;
    builder
        .bind_pipeline_compute(pipeline.clone())
        .bind_descriptor_sets(
            PipelineBindPoint::Compute,
            pipeline.layout().clone(),
            0,
            set,
        )
        .push_constants(pipeline.layout().clone(), 0, push_constants)
        .dispatch([len as u32 / 64, 1, 1])?;
    let command_buffer = builder.build()?;

    sync::now(device.clone())
        .then_execute(queue.clone(), command_buffer)?
        .then_signal_fence_and_flush()?
        .wait(None)?;

    let vec_res = buf_res.to_vec();

    Ok(())
}
