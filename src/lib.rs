#![deny(clippy::all)]
#![forbid(unsafe_code)]

mod consts;
mod display;
mod helpers;
mod matrix;
mod mesh;
mod triangle;
mod vector;

use std::mem::swap;
use std::time::Instant;

use consts::*;
use display::{
    clear_color_buffer, draw_filled_triangle, draw_grid, draw_line, draw_pixel, draw_rect,
    draw_triangle,
};
use error_iter::ErrorIter as _;
use log::error;
use matrix::{
    mat4_make_rotation_x, mat4_make_rotation_y, mat4_make_rotation_z, mat4_make_scale,
    mat4_make_translation, mat4_mul_vec4,
};
use mesh::{load_obj_file_data, Mesh};
use pixels::{Error, Pixels, SurfaceTexture};
use triangle::Triangle;
use vector::{vec3_from_vec4, vec4_from_vec3, Vec2, Vec3, Vec4};
use winit::dpi::LogicalSize;
use winit::event::{Event, VirtualKeyCode};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;
use winit_input_helper::WinitInputHelper;

use crate::vector::{
    vec3_cross, vec3_dot, vec3_normalize, vec3_rotate_x, vec3_rotate_y, vec3_rotate_z, vec3_sub,
};

fn project(point: &Vec3, fov_factor: f32) -> Vec2 {
    Vec2 {
        x: point.x * fov_factor / point.z,
        y: point.y * fov_factor / point.z,
    }
}

/// Representation of the application state. In this example, a box will bounce around the screen.
struct Renderer {
    is_running: bool,
    previous_frame_time: u16,
    current_time: Instant,
    camera_pos: Vec3,
    fov_factor: f32,
    mesh: Mesh,
    triangles_to_render: Vec<Triangle>,
}

impl Renderer {
    /// Create a new `Renderer` instance that can draw a moving box.
    fn new() -> Self {
        let mesh =
            load_obj_file_data("assets/cube.obj".to_string()).expect("Error reading object data");

        let current_time = Instant::now();

        Self {
            is_running: true,
            previous_frame_time: 0,
            current_time,
            camera_pos: Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            fov_factor: 640.0,
            mesh,
            triangles_to_render: Vec::new(),
        }
    }

    /// Update the `Renderer` internal state; bounce the box around the screen.
    fn update(&mut self) {
        // control FPS by waiting the frame target time
        //todo!();

        // Clear array of triangles
        self.triangles_to_render.clear();

        // add rotation (temporal)
        self.mesh.rotation.x += 0.01;
        self.mesh.rotation.y += 0.01;
        self.mesh.rotation.z += 0.01;

        // change scale (temporal)
        /*self.mesh.scale.x += 0.002;
        self.mesh.scale.y += 0.001;*/

        // translate the vertex away from the camera
        self.mesh.translation.z = 5.0;

        // Change translation
        //self.mesh.translation.x += 0.01;

        // Create matrix transformations
        let scale_matrix = mat4_make_scale(self.mesh.scale.x, self.mesh.scale.y, self.mesh.scale.z);
        let translation_matrix = mat4_make_translation(
            self.mesh.translation.x,
            self.mesh.translation.y,
            self.mesh.translation.z,
        );
        let rotation_x_matrix = mat4_make_rotation_x(self.mesh.rotation.x);
        let rotation_y_matrix = mat4_make_rotation_y(self.mesh.rotation.y);
        let rotation_z_matrix = mat4_make_rotation_z(self.mesh.rotation.z);

        // loop all triangle faces
        for mesh_face in self.mesh.faces.iter() {
            let mut face_vertices: [Vec3; 3] = [
                {
                    Vec3 {
                        ..Default::default()
                    }
                },
                {
                    Vec3 {
                        ..Default::default()
                    }
                },
                {
                    Vec3 {
                        ..Default::default()
                    }
                },
            ];

            face_vertices[0] = self.mesh.vertices[(mesh_face.a - 1) as usize];
            face_vertices[1] = self.mesh.vertices[(mesh_face.b - 1) as usize];
            face_vertices[2] = self.mesh.vertices[(mesh_face.c - 1) as usize];

            let mut transformed_vertices: [Vec4; 3] = [
                {
                    Vec4 {
                        ..Default::default()
                    }
                },
                {
                    Vec4 {
                        ..Default::default()
                    }
                },
                {
                    Vec4 {
                        ..Default::default()
                    }
                },
            ];

            // * loop all 3 vertices of this current face and apply transformations *
            for j in 0..3 {
                let mut transformed_vertex = vec4_from_vec3(&face_vertices[j]);

                // Use a matrix to transform our original vertex
                transformed_vertex = mat4_mul_vec4(&scale_matrix, &transformed_vertex);
                transformed_vertex = mat4_mul_vec4(&rotation_x_matrix, &transformed_vertex);
                transformed_vertex = mat4_mul_vec4(&rotation_y_matrix, &transformed_vertex);
                transformed_vertex = mat4_mul_vec4(&rotation_z_matrix, &transformed_vertex);
                transformed_vertex = mat4_mul_vec4(&translation_matrix, &transformed_vertex);

                // save transformed vertex
                transformed_vertices[j] = transformed_vertex;
            }

            // * Check backface culling *
            let vec_a = vec3_from_vec4(&transformed_vertices[0]);
            let vec_b = vec3_from_vec4(&transformed_vertices[1]);
            let vec_c = vec3_from_vec4(&transformed_vertices[2]);

            let mut vec_ab = vec3_sub(&vec_b, &vec_a); // B-A
            let mut vec_ac = vec3_sub(&vec_c, &vec_a); // C-A
            vec3_normalize(&mut vec_ab);
            vec3_normalize(&mut vec_ac);

            let mut normal = vec3_cross(&vec_ab, &vec_ac); // Use cross prod to find perpendicular.
            vec3_normalize(&mut normal); // normalize normal vector

            let cam_ray = vec3_sub(&self.camera_pos, &vec_a);

            // Negative dot product -> not looking towards camera
            let dot_normal_cam = vec3_dot(&normal, &cam_ray);

            if dot_normal_cam < 0.0 {
                // Bypass the triangles that are not looking at the camera
                continue;
            }

            // * Loop all 3 vertices to perform projection
            let mut projected_triangle: Triangle = Triangle {
                ..Default::default()
            };
            for j in 0..3 {
                // project the current vertex
                let mut projected_point =
                    project(&vec3_from_vec4(&transformed_vertices[j]), self.fov_factor);

                // scale and translate the projected points to the middle of the screen
                projected_point.x += (WIDTH / 2) as f32;
                projected_point.y += (HEIGHT / 2) as f32;

                // save that point
                projected_triangle.points[j] = projected_point;
                projected_triangle.avg_depth += transformed_vertices[j].z;
            }

            // Finish calculating the averga depth for each face based on the vertices after transformation.
            projected_triangle.avg_depth /= 3.0;

            // set the color for that face
            projected_triangle.rgba = mesh_face.rgba;

            // save the projected triangle in the array of triangles to render
            // triangles_to_render[i] = projected_triangle;
            self.triangles_to_render.push(projected_triangle);
        }

        // Sort the triangles to render by their avg_depth
        //  TODO: THIS IS NOT OPTIMAL AND COULD *EASILY* BE OPTIMIZED WITH A BETTER ALGORITHM. (using bubble sort now)
        for i in 0..self.triangles_to_render.len() {
            for j in i..self.triangles_to_render.len() {
                if self.triangles_to_render[i].avg_depth < self.triangles_to_render[j].avg_depth {
                    // swap the triangles
                    let temp = self.triangles_to_render[i].clone();
                    self.triangles_to_render[i] = self.triangles_to_render[j];
                    self.triangles_to_render[i] = temp;
                }
            }
        }
    }

    /// Draw the `Renderer` state to the frame buffer.
    ///
    /// Assumes the default texture format: `wgpu::TextureFormat::Rgba8UnormSrgb`
    fn draw(&mut self, frame: &mut [u8]) {
        // Clear screen
        clear_color_buffer(frame, BACKGROUND_COLOR);

        // * draw stuff here *
        // loop all projected triangles to render
        for triangle in self.triangles_to_render.iter() {
            // draw filled faces
            draw_triangle(
                frame,
                triangle.rgba,
                true,
                triangle.points[0].x as i32,
                triangle.points[0].y as i32,
                triangle.points[1].x as i32,
                triangle.points[1].y as i32,
                triangle.points[2].x as i32,
                triangle.points[2].y as i32,
            );

            // draw edges
            draw_triangle(
                frame,
                C_GREEN,
                false,
                triangle.points[0].x as i32,
                triangle.points[0].y as i32,
                triangle.points[1].x as i32,
                triangle.points[1].y as i32,
                triangle.points[2].x as i32,
                triangle.points[2].y as i32,
            );
        }

        // Clear the array of triangles to render every frame
        self.triangles_to_render.clear();
    }
}

pub fn run() -> Result<(), Error> {
    env_logger::init();
    let event_loop = EventLoop::new();
    let mut input = WinitInputHelper::new();

    // Window setup
    let window = {
        let size = LogicalSize::new(WIDTH as f64, HEIGHT as f64);
        WindowBuilder::new()
            .with_title("Hello Pixels")
            .with_inner_size(size)
            .with_min_inner_size(size)
            .build(&event_loop)
            .unwrap()
    };

    let mut pixels = {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        Pixels::new(WIDTH, HEIGHT, surface_texture)?
    };

    let mut renderer = Renderer::new();

    event_loop.run(move |event, _, control_flow| {
        // Draw the current frame
        if let Event::RedrawRequested(_) = event {
            renderer.draw(pixels.frame_mut());
            if let Err(err) = pixels.render() {
                log_error("pixels.render", err);
                *control_flow = ControlFlow::Exit;
                return;
            }
        }

        // Handle input events
        if input.update(&event) {
            // Close events
            if input.key_pressed(VirtualKeyCode::Escape) || input.close_requested() {
                *control_flow = ControlFlow::Exit;
                return;
            }

            // Resize the window
            if let Some(size) = input.window_resized() {
                if let Err(err) = pixels.resize_surface(size.width, size.height) {
                    log_error("pixels.resize_surface", err);
                    *control_flow = ControlFlow::Exit;
                    return;
                }
            }

            // Update internal state and request a redraw
            renderer.update();
            window.request_redraw();
        }
    });
}

fn log_error<E: std::error::Error + 'static>(method_name: &str, err: E) {
    error!("{method_name}() failed: {err}");
    for source in err.sources().skip(1) {
        error!("  Caused by: {source}");
    }
}
