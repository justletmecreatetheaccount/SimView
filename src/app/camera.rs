use std::f32::consts::PI;

use bytemuck::{Pod, Zeroable};

pub struct Camera {
    // Config
    fov: f32,          // Vertical in radians
    aspect_ratio: f32, // If I ever render in only a part of the window
    max_view_dist: f32,
    min_view_dist: f32,
    orbit_sensitivity: f32,
    pan_sensivity: f32,
    zoom_sensitivity: f32,

    // State
    // Don't bother with SIMD. Not enough math and alignement to deal with
    position: glam::Vec3,    // Of the pivot point, not the camera itself
    orientation: glam::Quat, // Orientation of the camera (z negative in front of the camera)
    distance: f32,           // Distance between the camera and the pivot point
}

impl Camera {
    pub fn new(aspect_ratio: f32) -> Camera {
        Camera {
            fov: PI / 2.0,
            aspect_ratio,
            max_view_dist: 10.0,
            min_view_dist: 0.1,
            orbit_sensitivity: 0.01,
            pan_sensivity: 0.005,
            zoom_sensitivity: 0.05,

            position: glam::vec3(0.0, 0.0, 0.0),
            orientation: glam::Quat::from_axis_angle(glam::vec3(0.0, 0.0, 1.0), 0.0),
            distance: 1.0,
        }
    }

    pub fn orbit(&mut self, dx: f32, dy: f32) {
        let x_rotation = glam::Quat::from_rotation_x(-dy * self.orbit_sensitivity); // Local frame
        let y_rotation = glam::Quat::from_rotation_y(-dx * self.orbit_sensitivity); // World frame

        self.orientation = y_rotation * self.orientation * x_rotation; // Hamiltion product
        self.orientation = self.orientation.normalize();
    }

    pub fn pan(&mut self, dx: f32, dy: f32) {
        let scale = self.pan_sensivity * self.distance;
        let x_pan = -dx * scale * (self.orientation * glam::vec3(1.0, 0.0, 0.0)); // Local frame
        let y_pan = dy * scale * glam::vec3(0.0, 1.0, 0.0); //World frame

        self.position = self.position + x_pan + y_pan;
    }

    pub fn zoom(&mut self, dy: f32) {
        self.distance = self.distance - dy * self.zoom_sensitivity;
    }

    pub fn camera_true_position(&self) -> glam::Vec3 {
        let vec_to_cam = self.orientation * glam::vec3(0.0, 0.0, 1.0); // Simplification thanks to glam
        let true_pos = self.position + vec_to_cam * self.distance;

        true_pos
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
        let pos = camera.camera_true_position();
        // Glam creates local -> world matrix
        let view_mat = glam::Mat4::from_rotation_translation(camera.orientation, pos).inverse();
        // View Space -> Clip Space
        let proj_mat = glam::camera::rh::proj::directx::perspective(
            camera.fov,
            camera.aspect_ratio,
            camera.min_view_dist,
            camera.max_view_dist,
        );

        // View -> Clip not the opposite
        let view_proj_matrix = proj_mat * view_mat;
        CameraUniforms { view_proj_matrix }
    }

    pub fn view_proj_matrix(&self) -> glam::Mat4 {
        self.view_proj_matrix
    }
}
