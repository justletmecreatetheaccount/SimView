use std::f32::consts::PI;

use bytemuck::{Pod, Zeroable};

pub struct Camera {
    // Config
    fov: f32,          // Vertical in radians
    aspect_ratio: f32, // If I ever render in only a part of the window
    max_view_dist: f32,
    min_view_dist: f32,
    orbit_sensitivity: f32,

    // State
    // Don't bother with SIMD. Not enough math and alignement to deal with
    position: glam::Vec3,    // Of the pivot point, not the camera itself
    orientation: glam::Quat, // Orientation to the axis to the camera (x right, y down, z towards)
    distance: f32,           // Distance between the camera and the pivot point
}

impl Camera {
    pub fn new(aspect_ratio: f32) -> Camera {
        Camera {
            fov: PI / 2.0,
            aspect_ratio,
            max_view_dist: 10.0,
            min_view_dist: 0.1,
            orbit_sensitivity: 0.3,

            position: glam::vec3(0.0, 0.0, 0.0),
            orientation: glam::quat(0.0, 0.0, 1.0, 0.0),
            distance: 1.0,
        }
    }

    pub fn orbit(&mut self, dx: f32, dy: f32) {
        let rotation = glam::Quat::from_euler(
            glam::EulerRot::XYZ,
            dy * self.orbit_sensitivity,
            dx * self.orbit_sensitivity,
            0.0,
        );

        self.orientation = self.orientation * rotation; // Hamiltion product
        self.orientation = self.orientation.normalize();
    }

    pub fn camera_true_position_and_orientation(&self) -> (glam::Vec3, glam::Quat) {
        let true_pos = self.position + self.orientation.xyz() * self.distance;

        let true_orientation = -self.orientation;

        (true_pos, true_orientation)
    }

    pub fn fov(&self) -> f32 {
        self.fov
    }

    pub fn aspect_ratio(&self) -> f32 {
        self.aspect_ratio
    }
}

#[repr(C)]
#[derive(Clone, Copy, Zeroable, Pod)]
// Data actually sent to the gpu
pub struct CameraUniforms {
    view_proj_matrix: glam::Mat4,
}

impl CameraUniforms {
    pub fn from_cam(camera: &Camera) -> CameraUniforms {
        let pos_or = camera.camera_true_position_and_orientation();
        let view_mat = glam::Mat4::from_rotation_translation(pos_or.1, pos_or.0);
        let proj_mat = glam::camera::rh::proj::directx::perspective(
            camera.fov,
            camera.aspect_ratio,
            camera.min_view_dist,
            camera.max_view_dist,
        );

        let view_proj_matrix = view_mat * proj_mat;
        CameraUniforms { view_proj_matrix }
    }
}
