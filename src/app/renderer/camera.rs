use std::f32::consts::PI;

pub struct Camera {
    // Don't bother with SIMD. Not enough math and alignement to deal with
    position: glam::Vec3,    // Of the pivot point, not the camera itself
    orientation: glam::Quat, // Orientation to the axis to the camera (x right, y down, z towards)
    distance: f32,           // Distance between the camera and the pivot point
    fov: f32,                // Horizontal in radians
    max_view_dist: f32,
    min_view_dist: f32,
}

impl Camera {
    pub fn new() -> Camera {
        Camera {
            position: glam::vec3(0.0, 0.0, 0.0),
            orientation: glam::quat(0.0, 0.0, 1.0, 0.0),
            distance: 1.0,
            fov: PI / 2.0,
            max_view_dist: 10.0,
            min_view_dist: 0.1,
        }
    }
}
