use wgpu::*;
use parking_lot::RwLock;
use std::collections::HashMap;
use std::sync::atomic::AtomicUsize;

pub struct BufferState {
    pub in_size: (usize, usize, usize),
    pub out_size: (usize, usize, usize),
    pub in_texture: Texture,
    pub out_texture: Texture,
    pub bind_group: BindGroup,
    pub params: Buffer,
    pub staging_buffer: Buffer,
    pub padded_out_stride: u32,
    pub last_access: AtomicUsize
}

pub struct WgpuProcessing<T: Sized> {
    _adapter: Adapter,
    pub device: Device,
    pub queue: Queue,
    pub shader: ShaderModule,
    pub pipeline: ComputePipeline,
    pub state: RwLock<HashMap<std::thread::ThreadId, BufferState>>,
    _marker: std::marker::PhantomData<T>,
}

#[allow(dead_code)]
pub enum ProcShaderSource<'a> {
    Wgsl(&'a str),
    SpirV(&'a [u8])
}

impl<T: Sized> WgpuProcessing<T> {
    pub fn new(shader: ProcShaderSource) -> Self {
        let power_preference = util::power_preference_from_env().unwrap_or(PowerPreference::HighPerformance);
        let instance = Instance::new(InstanceDescriptor::default());

        let adapter = pollster::block_on(instance.request_adapter(&RequestAdapterOptions { power_preference, ..Default::default() })).unwrap();

        let (device, queue) = pollster::block_on(
            adapter.request_device(&DeviceDescriptor {
                label: None,
                required_features: adapter.features(),
                required_limits: adapter.limits()
            }, None)
        ).unwrap();

        let info = adapter.get_info();
        log::info!("Using {} ({}) - {:#?}.", info.name, info.device, info.backend);

        let shader = device.create_shader_module(ShaderModuleDescriptor {
            label: None,
            source: match shader {
                ProcShaderSource::SpirV(bytes) => util::make_spirv(&bytes),
                ProcShaderSource::Wgsl(wgsl)   => ShaderSource::Wgsl(std::borrow::Cow::Borrowed(wgsl)),
            }
        });

        let layout = device.create_bind_group_layout(&BindGroupLayoutDescriptor {
            entries: &[
                BindGroupLayoutEntry { binding: 0, visibility: ShaderStages::COMPUTE, ty: BindingType::Buffer { ty: BufferBindingType::Uniform, has_dynamic_offset: false, min_binding_size: BufferSize::new(std::mem::size_of::<T>() as _) }, count: None },
                BindGroupLayoutEntry { binding: 1, visibility: ShaderStages::COMPUTE, ty: BindingType::Texture { sample_type: TextureSampleType::Uint, view_dimension: TextureViewDimension::D2, multisampled: false }, count: None },
                BindGroupLayoutEntry { binding: 2, visibility: ShaderStages::COMPUTE, ty: BindingType::StorageTexture { access: StorageTextureAccess::ReadWrite, format: TextureFormat::Rgba8Uint, view_dimension: TextureViewDimension::D2 }, count: None },
            ],
            label: None,
        });

        let pipeline_layout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: &[&layout],
            push_constant_ranges: &[],
        });

        let pipeline = device.create_compute_pipeline(&ComputePipelineDescriptor {
            module: &shader,
            entry_point: "main",
            label: None,
            layout: Some(&pipeline_layout),
        });

        Self {
            _adapter: adapter,
            device,
            queue,
            shader,
            pipeline,
            _marker: std::marker::PhantomData,
            state: RwLock::new(HashMap::new()),
        }
    }

    pub fn create_buffers(&self, in_size: (usize, usize, usize), out_size: (usize, usize, usize)) -> BufferState {
        let (iw, ih, _)  = (in_size.0  as u32, in_size.1  as u32, in_size.2  as u32);
        let (ow, oh, os) = (out_size.0 as u32, out_size.1 as u32, out_size.2 as u32);

        let align = COPY_BYTES_PER_ROW_ALIGNMENT as u32;
        let padding = (align - os % align) % align;
        let padded_out_stride = os + padding;
        let staging_size = padded_out_stride * oh;

        let in_desc = TextureDescriptor {
            label: None,
            size: Extent3d { width: iw, height: ih, depth_or_array_layers: 1 },
            mip_level_count: 1,
            sample_count: 1,
            dimension: TextureDimension::D2,
            format: TextureFormat::Rgba8Uint,
            usage: TextureUsages::TEXTURE_BINDING | TextureUsages::COPY_DST,
            view_formats: &[]
        };
        let out_desc = TextureDescriptor {
            label: None,
            size: Extent3d { width: ow, height: oh, depth_or_array_layers: 1 },
            mip_level_count: 1,
            sample_count: 1,
            dimension: TextureDimension::D2,
            format: TextureFormat::Rgba8Uint,
            usage: TextureUsages::STORAGE_BINDING | TextureUsages::COPY_SRC,
            view_formats: &[]
        };

        let in_texture = self.device.create_texture(&in_desc);
        let out_texture = self.device.create_texture(&out_desc);
        let staging_buffer = self.device.create_buffer(&BufferDescriptor {
            size: staging_size as u64,
            usage: BufferUsages::MAP_READ | BufferUsages::COPY_DST,
            label: None,
            mapped_at_creation: false
        });

        let in_view = in_texture.create_view(&TextureViewDescriptor::default());
        let out_view = out_texture.create_view(&TextureViewDescriptor::default());

        let params = self.device.create_buffer(&BufferDescriptor {
            size: std::mem::size_of::<T>() as u64,
            usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
            label: None,
            mapped_at_creation: false
        });

        let bind_group = self.device.create_bind_group(&BindGroupDescriptor {
            label: None,
            layout: &self.pipeline.get_bind_group_layout(0),
            entries: &[
                BindGroupEntry { binding: 0, resource: params.as_entire_binding() },
                BindGroupEntry { binding: 1, resource: BindingResource::TextureView(&in_view) },
                BindGroupEntry { binding: 2, resource: BindingResource::TextureView(&out_view) },
            ],
        });

        log::info!("Creating buffers {in_size:?} {out_size:?}, thread: {:?}", std::thread::current().id());

        BufferState {
            in_size,
            out_size,
            in_texture,
            out_texture,
            bind_group,
            params,
            staging_buffer,
            padded_out_stride,
            last_access: AtomicUsize::new(Self::timestamp()),
        }
    }

    fn timestamp() -> usize {
        std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_millis() as usize
    }

    fn get_buffer_for_thread(&self, in_size: (usize, usize, usize), out_size: (usize, usize, usize)) -> parking_lot::lock_api::RwLockUpgradableReadGuard<'_, parking_lot::RawRwLock, HashMap<std::thread::ThreadId, BufferState>> {
        let mut lock = self.state.upgradable_read();
        let mut state = lock.get(&std::thread::current().id());
        if state.map(|x| (x.in_size, x.out_size)) != Some((in_size, out_size)) {
            lock.with_upgraded(|x| {
                // Don't keep too many buffers around
                let max = num_cpus::get() - 1;
                if x.len() > max {
                    // Remove least recently used buffers
                    let mut keys = x.iter().map(|(k, v)| (*k, v.last_access.load(std::sync::atomic::Ordering::Relaxed))).collect::<Vec<_>>();
                    keys.sort_by(|a, b| a.1.cmp(&b.1));
                    for (k, _) in keys.iter().take(x.len() - max) {
                        log::info!("Removing {k:?}");
                        x.remove(k);
                    }
                }
                x.insert(std::thread::current().id(), self.create_buffers(in_size, out_size));
            });
            state = lock.get(&std::thread::current().id());
        }
        let state = state.unwrap();
        state.last_access.store(Self::timestamp(), std::sync::atomic::Ordering::Relaxed);
        lock
    }

    pub fn run_compute(&self, params: &T, in_size: (usize, usize, usize), out_size: (usize, usize, usize), in_buffer: &[u8], out_buffer: &mut [u8]) -> bool {
        let lock = self.get_buffer_for_thread(in_size, out_size);
        let state = lock.get(&std::thread::current().id()).unwrap();

        let width = out_size.0 as u32;
        let height = out_size.1 as u32;

        let mut encoder = self.device.create_command_encoder(&CommandEncoderDescriptor { label: None });

        // Write params uniform
        self.queue.write_buffer(
            &state.params,
            0,
            unsafe { std::slice::from_raw_parts(params as *const _ as _, std::mem::size_of::<T>() ) }
        );

        // Write input texture
        self.queue.write_texture(
            state.in_texture.as_image_copy(),
            in_buffer,
            ImageDataLayout { offset: 0, bytes_per_row: Some(in_size.2 as u32), rows_per_image: None },
            Extent3d { width: in_size.0 as u32, height: in_size.1 as u32, depth_or_array_layers: 1 },
        );

        // Run the compute pass
        {
            let mut cpass = encoder.begin_compute_pass(&ComputePassDescriptor { label: None, timestamp_writes: None });
            cpass.set_pipeline(&self.pipeline);
            cpass.set_bind_group(0, &state.bind_group, &[]);
            cpass.dispatch_workgroups((width as f32 / 16.0).ceil() as u32, (height as f32 / 16.0).ceil() as u32, 1);
        }

        // Copy output texture to buffer that we can read
        encoder.copy_texture_to_buffer(
            ImageCopyTexture { texture: &state.out_texture, mip_level: 0, origin: Origin3d::ZERO, aspect: TextureAspect::All },
            ImageCopyBuffer { buffer: &state.staging_buffer, layout: ImageDataLayout { offset: 0, bytes_per_row: Some(state.padded_out_stride), rows_per_image: None } },
            Extent3d { width: width as u32, height: height as u32, depth_or_array_layers: 1 }
        );

        self.queue.submit(Some(encoder.finish()));

        // Read the output buffer
        let buffer_slice = state.staging_buffer.slice(..);
        let (sender, receiver) = futures_intrusive::channel::shared::oneshot_channel();
        buffer_slice.map_async(MapMode::Read, move |v| sender.send(v).unwrap());

        self.device.poll(Maintain::Wait);

        if let Some(Ok(())) = pollster::block_on(receiver.receive()) {
            let out_stride = out_size.2;

            let data = buffer_slice.get_mapped_range();
            if state.padded_out_stride == out_stride as u32 {
                // Fast path
                (&mut out_buffer[..height as usize * out_stride]).copy_from_slice(data.as_ref());
            } else {
                data.as_ref()
                    .chunks(state.padded_out_stride as usize)
                    .zip(out_buffer.chunks_mut(out_stride))
                    .for_each(|(src, dest)| {
                        dest.copy_from_slice(&src[0..out_stride]);
                    });
            }

            // We have to make sure all mapped views are dropped before we unmap the buffer.
            drop(data);
            state.staging_buffer.unmap();
        } else {
            log::error!("failed to run compute on wgpu!");
            return false;
        }
        true
    }
}
