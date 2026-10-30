use std::{f32::consts::PI, thread::AccessError};

use simgrain::Simulator;
use wgpu::{
    BindGroup, BindGroupDescriptor, BindGroupEntry, BindGroupLayout, Buffer, Device, Queue,
    Texture, TextureView,
};

use crate::app::camera::{Camera, CameraUniforms};

// Useful info since I don't wan't to use encase
// https://www.w3.org/TR/WGSL/#alignment-and-size
pub struct Buffers {
    pub depth_buffer: (Texture, TextureView),
    pub each_frame_bg: BindGroup,
    pub camera_buffer: Buffer,
    pub vertex_buffer: Buffer,
}

impl Buffers {
    pub fn new(
        _particles: u64,
        bind_group_layout: &BindGroupLayout,
        device: &Device,
        framebuffer_size: (u32, u32),
        sim: &Simulator,
    ) -> Buffers {
        let depth_buffer_desc = wgpu::TextureDescriptor {
            label: Some("Depth Buffer"),
            size: wgpu::Extent3d {
                width: framebuffer_size.0,
                height: framebuffer_size.1,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Depth32Float,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT, // If sample later add TEXTURE_BINDING
            view_formats: &[],
        };

        let depth_buffer_tex = device.create_texture(&depth_buffer_desc);

        let depth_buffer_view =
            depth_buffer_tex.create_view(&wgpu::TextureViewDescriptor::default());

        let depth_buffer = (depth_buffer_tex, depth_buffer_view);

        let camera_buffer_desc = wgpu::BufferDescriptor {
            label: Some("Camera buffer"),
            size: size_of::<glam::Mat4>() as u64, //Mat4 view_proj_matrix
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        };

        // CHANGE TO AN IMMEDIATE
        let vertex_buffer_desc = wgpu::BufferDescriptor {
            label: Some("Vertex buffer"),
            size: 18 * size_of::<Vertex>() as u64, //Holds one instance of a square and uv coordinates
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        };

        let camera_buffer = device.create_buffer(&camera_buffer_desc);
        let vertex_buffer = device.create_buffer(&vertex_buffer_desc);

        let camera_entry = BindGroupEntry {
            binding: 0,
            resource: wgpu::BindingResource::Buffer(wgpu::BufferBinding {
                buffer: &camera_buffer,
                offset: 0,
                size: None,
            }),
        };

        let positions_entry = BindGroupEntry {
            binding: 1,
            resource: wgpu::BindingResource::Buffer(wgpu::BufferBinding {
                buffer: sim.context().buffers().hot_buffer(),
                offset: 0,
                size: None,
            }),
        };

        let radii_entry = BindGroupEntry {
            binding: 2,
            resource: wgpu::BindingResource::Buffer(wgpu::BufferBinding {
                buffer: sim.context().buffers().cold_buffer(),
                offset: 0,
                size: None,
            }),
        };

        let bind_group_desc = BindGroupDescriptor {
            label: Some("Each Frame BindGroup"),
            layout: bind_group_layout,
            entries: &[camera_entry, positions_entry, radii_entry],
        };
        let each_frame_bg = device.create_bind_group(&bind_group_desc);

        Buffers {
            depth_buffer,
            each_frame_bg,
            camera_buffer,
            vertex_buffer,
        }
    }

    pub fn init_buffers_data(&self, queue: &Queue) {
        let square_verticies: [Vertex; 6] = [
            Vertex {
                position: glam::vec3(-0.5, 0.5, 0.0),
                uv: glam::vec2(-1.0, 1.0),
            },
            Vertex {
                position: glam::vec3(0.5, 0.5, 0.0),
                uv: glam::vec2(1.0, 1.0),
            },
            Vertex {
                position: glam::vec3(-0.5, -0.5, 0.0),

                uv: glam::vec2(-1.0, -1.0),
            },
            Vertex {
                position: glam::vec3(0.5, 0.5, 0.0),
                uv: glam::vec2(1.0, 1.0),
            },
            Vertex {
                position: glam::vec3(-0.5, -0.5, 0.0),

                uv: glam::vec2(-1.0, -1.0),
            },
            Vertex {
                position: glam::vec3(0.5, -0.5, 0.0),
                uv: glam::vec2(1.0, -1.0),
            },
        ];

        let mut square_star_verticies: [Vertex; 18] = core::array::repeat(Vertex {
            position: glam::vec3(0.0, 0.0, 0.0),
            uv: glam::vec2(0.0, 0.0),
        });

        for i in 0..3 {
            if i == 1 {
                for j in 0..6 {
                    let rot = glam::Mat3::from_rotation_x(PI / 2.0);
                    square_star_verticies[i * 6 + j] = Vertex {
                        position: rot * square_verticies[j].position,
                        uv: square_verticies[j].uv,
                    }
                }
            } else if i == 2 {
                for j in 0..6 {
                    let rot = glam::Mat3::from_rotation_y(PI / 2.0);
                    square_star_verticies[i * 6 + j] = Vertex {
                        position: rot * square_verticies[j].position,
                        uv: square_verticies[j].uv,
                    }
                }
            } else {
                for j in 0..6 {
                    square_star_verticies[i * 6 + j] = square_verticies[j]
                }
            }
        }

        let bytes: &[u8] = bytemuck::cast_slice(&square_star_verticies);
        queue.write_buffer(&self.vertex_buffer, 0, bytes);

        let camera_uniforms = CameraUniforms::from_cam(&Camera::new(0.0));

        let bytes: &[u8] = bytemuck::bytes_of(&camera_uniforms);
        queue.write_buffer(&self.camera_buffer, 0, bytes);
    }
}

#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    position: glam::Vec3,
    uv: glam::Vec2,
}
