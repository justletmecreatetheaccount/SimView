use wgpu::{
    BindGroup, BindGroupDescriptor, BindGroupEntry, BindGroupLayout, Buffer, Device, Queue,
};

use crate::app::camera::{Camera, CameraUniforms};

// Useful info since I don't wan't to use encase
// https://www.w3.org/TR/WGSL/#alignment-and-size
pub struct Buffers {
    pub each_frame_bg: BindGroup,
    pub camera_buffer: Buffer,
    pub vertex_buffer: Buffer,
    // Positions, Velocities
    pub hot_buffer: Buffer,
    // Radii
    pub cold_buffer: Buffer,
}

impl Buffers {
    pub fn new(particles: u64, bind_group_layout: &BindGroupLayout, device: &Device) -> Buffers {
        let camera_buffer_desc = wgpu::BufferDescriptor {
            label: Some("Camera buffer"),
            size: 64, //Mat4 view_proj_matrix
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        };

        // CHANGE TO AN IMMEDIATE
        let vertex_buffer_desc = wgpu::BufferDescriptor {
            label: Some("Vertex buffer"),
            size: 48, //Holds one instance of a square
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        };
        let hot_buffer_desc = wgpu::BufferDescriptor {
            label: Some("Hot buffer"),
            size: particles * 4 * 3 * 2, //24_000_000 -> 1 000 000 (1M) Positions and Velocities
            usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        };
        let cold_buffer_desc = wgpu::BufferDescriptor {
            label: Some("Cold buffer"),
            size: particles * 4 * 2, //8_000_000 -> 1 000 000 (1M) Radiuses and Masses
            usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        };

        let camera_buffer = device.create_buffer(&camera_buffer_desc);
        let vertex_buffer = device.create_buffer(&vertex_buffer_desc);
        let hot_buffer = device.create_buffer(&hot_buffer_desc);
        let cold_buffer = device.create_buffer(&cold_buffer_desc);

        let camera_uniform_entry = BindGroupEntry {
            binding: 0,
            resource: wgpu::BindingResource::Buffer(wgpu::BufferBinding {
                buffer: &camera_buffer,
                offset: 0,
                size: None,
            }),
        };
        let bind_group_desc = BindGroupDescriptor {
            label: Some("Each Frame BindGroup"),
            layout: bind_group_layout,
            entries: &[camera_uniform_entry],
        };
        let each_frame_bg = device.create_bind_group(&bind_group_desc);

        Buffers {
            each_frame_bg,
            camera_buffer,
            vertex_buffer,
            hot_buffer,
            cold_buffer,
        }
    }

    pub fn init_buffers_data(&self, queue: &Queue) {
        let position: [f32; 12] = [
            -0.5, 0.5, 0.0, 0.5, 0.5, 0.0, -0.5, -0.5, 0.0, 0.5, -0.5, 0.0,
        ];
        let bytes: &[u8] = bytemuck::cast_slice(&position);
        queue.write_buffer(&self.vertex_buffer, 0, bytes);

        let camera_uniforms = CameraUniforms::from_cam(&Camera::new(0.0));

        let bytes: &[u8] = bytemuck::bytes_of(&camera_uniforms);
        queue.write_buffer(&self.camera_buffer, 0, bytes);
    }
}
