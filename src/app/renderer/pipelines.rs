use std::hash::Hash;

use wgpu::{
    BindGroupLayout, BindGroupLayoutDescriptor, BindGroupLayoutEntry, ColorTargetState,
    ColorWrites, Device, FragmentState, MultisampleState, PipelineLayout, PipelineLayoutDescriptor,
    PrimitiveState, RenderPipeline, RenderPipelineDescriptor, TextureFormat, VertexAttribute,
    include_wgsl,
};

pub struct Pipelines {
    pub each_frame_bgl: BindGroupLayout,
    pub circle: RenderPipeline,
}

impl Pipelines {
    pub fn new(device: &Device, preferred_format: TextureFormat) -> Pipelines {
        let shader_desc = include_wgsl!("../../shaders/render/circle.wgsl");
        let shader = device.create_shader_module(shader_desc);

        let camera_entry = BindGroupLayoutEntry {
            binding: 0,
            visibility: wgpu::ShaderStages::VERTEX,
            ty: wgpu::BindingType::Buffer {
                ty: wgpu::BufferBindingType::Uniform,
                has_dynamic_offset: false,
                min_binding_size: None, // Won't bother now
            },
            count: None, // Again won't bother
        };

        let positions_entry = BindGroupLayoutEntry {
            binding: 1,
            visibility: wgpu::ShaderStages::VERTEX,
            ty: wgpu::BindingType::Buffer {
                ty: wgpu::BufferBindingType::Storage { read_only: true }, // Not modified here, requires feature
                has_dynamic_offset: false,
                min_binding_size: None, // Won't bother now
            },
            count: None, // Again won't bother
        };

        let radii_entry = BindGroupLayoutEntry {
            binding: 2,
            visibility: wgpu::ShaderStages::VERTEX,
            ty: wgpu::BindingType::Buffer {
                ty: wgpu::BufferBindingType::Storage { read_only: true },
                has_dynamic_offset: false,
                min_binding_size: None, // Won't bother now
            },
            count: None, // Again won't bother
        };

        let bind_group_layout_desc = &BindGroupLayoutDescriptor {
            label: Some("Each Frame BG Layout"),
            entries: &[camera_entry, positions_entry, radii_entry],
        };
        let each_frame_bgl = device.create_bind_group_layout(bind_group_layout_desc);

        let layout_desc = PipelineLayoutDescriptor {
            label: Some("Circle Pipeline Layout"),
            bind_group_layouts: &[Some(&each_frame_bgl)],
            immediate_size: 0,
        };
        let layout = &device.create_pipeline_layout(&layout_desc);

        let position_attribute = VertexAttribute {
            format: wgpu::VertexFormat::Float32x3,
            offset: 0,
            shader_location: 0,
        };

        let uv_attribute = VertexAttribute {
            format: wgpu::VertexFormat::Float32x2,
            offset: 12,
            shader_location: 1,
        };

        let vertex_buffer_layout = wgpu::VertexBufferLayout {
            array_stride: size_of::<super::buffers::Vertex>() as u64,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[position_attribute, uv_attribute],
        };

        let vertex_state = wgpu::VertexState {
            module: &shader,
            entry_point: Some("vtx_main"),
            compilation_options: Default::default(),
            buffers: &[Some(vertex_buffer_layout)],
        };

        let primitive_state = PrimitiveState {
            topology: wgpu::PrimitiveTopology::TriangleList,
            strip_index_format: None,
            front_face: wgpu::FrontFace::Ccw,
            cull_mode: None,
            unclipped_depth: false, //Didn't bother to enable
            polygon_mode: wgpu::PolygonMode::Fill,
            conservative: false,
        };

        let depth_stencil_state = wgpu::DepthStencilState {
            format: wgpu::TextureFormat::Depth32Float,
            depth_write_enabled: Some(true),
            depth_compare: Some(wgpu::CompareFunction::Less),
            stencil: Default::default(),
            bias: Default::default(),
        };

        let multisample_state = MultisampleState {
            count: 1,
            mask: !0, // Mask nothing
            alpha_to_coverage_enabled: false,
        };

        let color_target_state = ColorTargetState {
            format: preferred_format,
            blend: Some(wgpu::BlendState::ALPHA_BLENDING),
            write_mask: ColorWrites::ALL, // Write all colors
        };

        let frag_state = Some(FragmentState {
            module: &shader,
            entry_point: Some("frag_main"),
            compilation_options: Default::default(),
            targets: &[Some(color_target_state)],
        });

        let circle_desc = RenderPipelineDescriptor {
            label: Some("Circle pipeline"),
            layout: Some(layout),
            vertex: vertex_state,
            primitive: primitive_state,
            depth_stencil: Some(depth_stencil_state),
            multisample: multisample_state,
            fragment: frag_state,
            multiview_mask: None,
            cache: None,
        };

        let circle = device.create_render_pipeline(&circle_desc);

        Pipelines {
            each_frame_bgl,
            circle,
        }
    }
}
