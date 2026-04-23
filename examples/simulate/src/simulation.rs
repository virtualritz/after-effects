use after_effects::{self as ae, aegp::ComputeClassId};
use bytemuck::{Pod, Zeroable};
use std::mem::size_of_val;

pub const CACHE_ID: ComputeClassId<SimOptions, SimStep> = ComputeClassId::new("particle_cache");

// Store the particle state every 8 frames - 6 times per second of footage.
// 16 bytes * 36 saves a minte * 1 million particles 500mb per minute of simulation max
const KEYFRAME_INTERVAL: u32 = 8;

#[derive(Clone, Copy, Debug, Pod, Zeroable)]
#[repr(C)]
pub struct SimParams {
    pub frame: u32,
    pub num_particles: u32,
    pub seed: i32,
    pub gravity_pt: [f32; 2],
    pub gravity_str: f32,
}

#[derive(Clone)]
pub struct SimOptions {
    pub params: SimParams,
    pub step: SimStep,
}

#[derive(Copy, Clone, Debug)]
pub struct Particle {
    pub pos: [f32; 2],
    pub vel: [f32; 2],
    pub drag: f32,
}

#[derive(Clone, Debug)]
pub struct SimStep(pub Vec<Particle>);

impl SimStep {
    pub fn generate_key(opts: &SimOptions) -> Result<ae::aegp::Guid, ae::Error> {
        let hash_suite = ae::aegp::suites::Hash::new()?;
        let key_data: &[u8] = bytemuck::bytes_of(&opts.params);
        hash_suite.create_hash_from_ptr(key_data)
    }

    pub fn delete(_value: SimStep) {
        log::info!("Dropping simstep");
    }

    // We pass the pre-computed result via SimOptions rather than computing here.
    // Otherwise simoptions would contain a bunch of function pointers and lifetimes,
    // it's just not a very good API design for lifetime-intuition and signature sanity.
    pub fn compute(opts: &SimOptions) -> Result<Self, ae::Error> { Ok(opts.step.clone()) }

    pub fn approx_size(&self) -> usize { size_of_val(self.0.as_slice()) }

    pub fn initial(num_particles: u32, seed: i32) -> Self {
        // just reinterpret - it's fine it's a seed
        let mut rng = fastrand::Rng::with_seed(seed as u64);
        Self(
            (0..num_particles)
                .map(|_| Particle {
                    pos: [rng.f32(), rng.f32()],
                    vel: [0.0, 0.0],
                    drag: 0.95 + rng.f32() * 0.04, // randomize friction s.t. it avoids visual uniformity
                })
                .collect(),
        )
    }

    pub fn step(&mut self, dt: f32, gravity_pt: [f32; 2], gravity_str: f32) {
        const MAX_VEL: f32 = 2.0;
        const MIN_DIST: f32 = 0.01;
        // dead simple euler sim just as an example
        for p in &mut self.0 {
            let dir = [gravity_pt[0] - p.pos[0], gravity_pt[1] - p.pos[1]];
            let dist = (dir[0].powi(2) + dir[1].powi(2)).sqrt().max(MIN_DIST);
            let norm = [dir[0] / dist, dir[1] / dist];

            let accel = [norm[0] * gravity_str, norm[1] * gravity_str];

            p.vel[0] = (p.vel[0] * p.drag + accel[0] * dt).clamp(-MAX_VEL, MAX_VEL);
            p.vel[1] = (p.vel[1] * p.drag + accel[1] * dt).clamp(-MAX_VEL, MAX_VEL);

            p.pos[0] += p.vel[0] * dt;
            p.pos[1] += p.vel[1] * dt;
        }
    }
}

/// Find the most recent cached frame, or None if no cache hits.
fn find_cached_frame<F>(
    cache: &ae::aegp::suites::ComputeCache,
    target_frame: u32,
    num_particles: u32,
    seed: i32,
    get_gravity_at: &F,
) -> Option<(u32, SimStep)>
where
    F: Fn(u32) -> Result<([f32; 2], f32), ae::Error>,
{
    let last_cacheable = (target_frame / KEYFRAME_INTERVAL) * KEYFRAME_INTERVAL;
    for frame in (0..=last_cacheable)
        .rev()
        .step_by(KEYFRAME_INTERVAL as usize)
    {
        let (gravity_pt, gravity_str) = get_gravity_at(frame).ok()?;

        let mut opts = SimOptions {
            params: SimParams {
                frame,
                num_particles,
                seed,
                gravity_pt,
                gravity_str,
            },
            // This is not great. but if you want to compute outside of the compute callback
            // (which in all likelihood you do), then you need a dummy for your computed data for cachelookups
            step: SimStep(Vec::new()),
        };

        if let Some(receipt) = cache.checkout_cached(&CACHE_ID, &mut opts).ok().flatten() {
            if let Ok(step) = cache.receipt_compute_value::<SimStep>(&receipt) {
                let result = step.clone();
                let _ = cache.check_in_compute_receipt(receipt);
                return Some((frame, result));
            }
        }
    }
    None
}

fn cache_frame(cache: &ae::aegp::suites::ComputeCache, params: SimParams, step: &SimStep) {
    let mut opts = SimOptions {
        params,
        step: step.clone(),
    };
    if let Ok(receipt) = cache.compute_if_needed_and_checkout(CACHE_ID, &mut opts, true) {
        let _ = cache.check_in_compute_receipt(receipt);
    }
}

/// Run the simulation to target_frame, using cached keyframes when available.
pub fn simulate_up_to_frame<F>(
    target_frame: u32,
    num_particles: u32,
    seed: i32,
    dt: f32,
    get_gravity_at: &F,
) -> Result<SimStep, ae::Error>
where
    F: Fn(u32) -> Result<([f32; 2], f32), ae::Error>,
{
    let cache = ae::aegp::suites::ComputeCache::new()?;

    let (start_frame, mut step) =
        find_cached_frame(&cache, target_frame, num_particles, seed, get_gravity_at)
            .unwrap_or_else(|| (0, SimStep::initial(num_particles, seed)));

    for frame in (start_frame + 1)..=target_frame {
        let (gravity_pt, gravity_str) = get_gravity_at(frame)?;
        step.step(dt, gravity_pt, gravity_str);

        if frame.is_multiple_of(KEYFRAME_INTERVAL) {
            let params = SimParams {
                frame,
                num_particles,
                seed,
                gravity_pt,
                gravity_str,
            };
            cache_frame(&cache, params, &step);
        }
    }

    Ok(step)
}

/// Purely to demonstrate the speed of the simulation without caching
pub fn simulate_to_frame_no_cache<F>(
    target_frame: u32,
    num_particles: u32,
    seed: i32,
    dt: f32,
    get_gravity_at: &F,
) -> Result<SimStep, ae::Error>
where
    F: Fn(u32) -> Result<([f32; 2], f32), ae::Error>,
{
    let mut current_step = SimStep::initial(num_particles, seed);

    for frame in 1..=target_frame {
        let (gravity_pt, gravity_str) = get_gravity_at(frame)?;
        current_step.step(dt, gravity_pt, gravity_str);
    }

    Ok(current_step)
}

pub fn blit_particles(
    layer: &mut ae::Layer,
    particles: &[Particle],
    size: usize,
    show_velocity: bool,
) {
    let (w, h) = (layer.width(), layer.height());
    let depth = layer.bit_depth();

    const MAX_VEL: f32 = 2.0;
    let vel_to_unit = |v: f32| ((v / MAX_VEL) * 0.5 + 0.5).clamp(0.0, 1.0);

    for p in particles {
        let px = (p.pos[0] * w as f32) as usize;
        let py = (p.pos[1] * h as f32) as usize;

        for dy in 0..size {
            for dx in 0..size {
                let (x, y) = (px.saturating_add(dx), py.saturating_add(dy));
                if x >= w || y >= h {
                    continue;
                }

                match depth {
                    8 => {
                        let pixel = if show_velocity {
                            ae::Pixel8 {
                                alpha: 255,
                                red: (vel_to_unit(p.vel[0]) * 255.0) as u8,
                                green: (vel_to_unit(p.vel[1]) * 255.0) as u8,
                                blue: 128,
                            }
                        } else {
                            ae::Pixel8 {
                                alpha: 255,
                                red: 255,
                                green: 255,
                                blue: 255,
                            }
                        };
                        *layer.as_pixel8_mut(x, y) = pixel;
                    }
                    16 => {
                        let max = ae::MAX_CHANNEL16 as f32;
                        let pixel = if show_velocity {
                            ae::Pixel16 {
                                alpha: ae::MAX_CHANNEL16 as u16,
                                red: (vel_to_unit(p.vel[0]) * max) as u16,
                                green: (vel_to_unit(p.vel[1]) * max) as u16,
                                blue: (max * 0.5) as u16,
                            }
                        } else {
                            ae::Pixel16 {
                                alpha: ae::MAX_CHANNEL16 as u16,
                                red: ae::MAX_CHANNEL16 as u16,
                                green: ae::MAX_CHANNEL16 as u16,
                                blue: ae::MAX_CHANNEL16 as u16,
                            }
                        };
                        *layer.as_pixel16_mut(x, y) = pixel;
                    }
                    _ => {}
                }
            }
        }
    }
}
