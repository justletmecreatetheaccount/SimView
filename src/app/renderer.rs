mod buffers;
mod pipelines;

use simgrain::Simulator;
use wgpu::{
    CommandBuffer, CommandEncoder, CommandEncoderDescriptor, Device, Queue,
    RenderPassColorAttachment, RenderPassDescriptor, TextureFormat, TextureView,
    TextureViewDescriptor, VertexStepMode::Vertex, util::RenderEncoder,
};

use crate::app::{
    camera::{Camera, CameraUniforms},
    gpu::Gpu,
    renderer::{buffers::Buffers, pipelines::Pipelines},
    window::WindowManager,
};

pub struct Renderer {
    pipelines: Pipelines,
    buffers: Buffers,
}
impl Renderer {
    pub fn new(gpu: &Gpu, window: &WindowManager, sim: &Simulator) -> Renderer {
        let pipelines = Pipelines::new(gpu.device(), gpu.preferred_format());

        let buffers = Buffers::new(
            1,
            &pipelines.each_frame_bgl,
            gpu.device(),
            window.framebuffer_size(),
            sim,
        );

        buffers.init_buffers_data(gpu.queue());

        Renderer { pipelines, buffers }
    }

    pub fn render(&self, gpu: &Gpu, camera: &Camera, sim: &Simulator) {
        let current_texture = match gpu.surface().get_current_texture() {
            wgpu::CurrentSurfaceTexture::Success(texture) => texture,
            wgpu::CurrentSurfaceTexture::Occluded | wgpu::CurrentSurfaceTexture::Timeout => {
                println!("TO");
                return;
            }
            wgpu::CurrentSurfaceTexture::Suboptimal(texture) => {
                drop(texture);
                gpu.configure_surface();
                println!("Texture is subopt");
                return;
            }
            wgpu::CurrentSurfaceTexture::Outdated => {
                gpu.configure_surface();
                return;
            }
            wgpu::CurrentSurfaceTexture::Validation => {
                unreachable!("No error scope registered, so validation errors will panic")
            }
            // Will need to be reworked for better error handling
            wgpu::CurrentSurfaceTexture::Lost => {
                println!("LOST");
                panic!("can't be bothered to fix, AAAAAAAAA")
            }
        };
        let texture_view_desc = TextureViewDescriptor {
            label: Some("Main View Descriptor"),
            format: Some(gpu.preferred_format()),
            dimension: Some(wgpu::TextureViewDimension::D2),
            usage: Some(wgpu::TextureUsages::RENDER_ATTACHMENT),
            // See later what the rest means
            ..Default::default()
        };
        let current_texture_view = current_texture.texture.create_view(&texture_view_desc);

        let encoder_descriptor = CommandEncoderDescriptor {
            label: Some("Render Encoder"),
        };

        let mut encoder = gpu.device().create_command_encoder(&encoder_descriptor);

        self.record_render_pass(&mut encoder, &current_texture_view, sim);

        let cam_uniforms = CameraUniforms::from_cam(camera);
        let bytes: &[u8] = bytemuck::bytes_of(&cam_uniforms);

        gpu.queue()
            .write_buffer(&self.buffers.camera_buffer, 0, bytes);

        let command_buffer = encoder.finish();
        gpu.queue().submit([command_buffer]);
        gpu.queue().present(current_texture);
    }

    fn record_render_pass(
        &self,
        encoder: &mut CommandEncoder,
        view: &TextureView,
        sim: &Simulator,
    ) {
        let color_attachement = RenderPassColorAttachment {
            view: view,
            depth_slice: None,
            resolve_target: None,    //No multisampling
            ops: Default::default(), // Clear the frame before, write the whole new frame
        };

        let depth_stencil_attachment = wgpu::RenderPassDepthStencilAttachment {
            view: &self.buffers.depth_buffer.1,
            depth_ops: Some(wgpu::Operations {
                load: wgpu::LoadOp::Clear(1.0), // Everything is supposed far
                store: wgpu::StoreOp::Store,
            }),
            stencil_ops: None, // I ain't using it I think
        };

        let render_pass_desc = RenderPassDescriptor {
            label: Some("Ball Drawing Pass"),
            color_attachments: &[Some(color_attachement)],
            depth_stencil_attachment: Some(depth_stencil_attachment), //Will need one eventually
            timestamp_writes: None,
            occlusion_query_set: None, //God knows what this means
            multiview_mask: None,      //Layers
        };
        let mut render_pass = encoder.begin_render_pass(&render_pass_desc);

        render_pass.set_pipeline(&self.pipelines.circle);
        let vtx_buffer_range = 0..((size_of::<buffers::Vertex>() * 18) as u64);
        render_pass.set_vertex_buffer(0, self.buffers.vertex_buffer.slice(vtx_buffer_range));
        render_pass.set_bind_group(0, Some(&self.buffers.each_frame_bg), &[]);
        render_pass.draw(0..18, 0..(sim.sim_context().n_grains() as u32));
    }
}
