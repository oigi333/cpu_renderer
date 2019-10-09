#[macro_use]
extern crate impl_ops;

use minifb::{Scale, Window, WindowOptions};
use scoped_threadpool::Pool;

use framebuffer::Framebuffer;
use renderer::Program;
use vector::Vector3;

use crate::matrix::Matrix4;
use crate::renderer::RenderRegion;
use crate::vector::Vector4;

mod framebuffer;
mod texture;
mod utils;
mod vector;
mod matrix;
mod renderer;

const WIDTH: usize = 175 * 2;
const HEIGHT: usize = 100 * 2;


fn sierpinski(v: Vector3, _: f64, t: &f64) -> Vector3 {
    let x = (WIDTH as f64 * (v.x + *t)) as usize;
    let y = (HEIGHT as f64 * v.y) as usize;
    Vector3::new((x & y) as f64, (x & 15) as f64 / 15.0, (y & 15) as f64 / 15.0)
}

fn ojascki(v: Vector3, _: f64, t: &f64) -> Vector3 {
    let mut x = v.x - 0.5;
    let mut y = (v.y - 0.5) * 9.0 / 16.0;

    let mut a = t + (t * 0.5).sin() * 0.25;
    let sina = a.sin();
    let cosa = a.cos();

    x = x * cosa - y * sina;
    y = x * sina + y * cosa;
    let mut length = (x * x + y * y).sqrt();
    let factor = (length * (6.0 + (a * 0.4).sin() * 3.0)).powi(2);
    x /= factor;
    y /= factor;

    let col = Vector3::zero();
    length = (x * x + y * y).sqrt();
    let x1 = x;
    x = x.atan2(y);
    y = length * 20.0;

    x += (y - x).floor() + 1.0;
    y = (y - x1).fract();

    x = (x * x * 4.0 - (0.25 - y * y).sqrt()) * 3.0;

    Vector3::new(x, y, 0.0)
}

// ███╗   ███╗ █████╗ ████████╗██╗  ██╗
// ████╗ ████║██╔══██╗╚══██╔══╝██║  ██║
// ██╔████╔██║███████║   ██║   ███████║
// ██║╚██╔╝██║██╔══██║   ██║   ██╔══██║
// ██║ ╚═╝ ██║██║  ██║   ██║   ██║  ██║
// ╚═╝     ╚═╝╚═╝  ╚═╝   ╚═╝   ╚═╝  ╚═╝

fn rotate_x(v: Vector3, angle: f64) -> Vector3 {
    let cosa = angle.cos();
    let sina = angle.sin();
    Vector3::new(v.x, v.y * cosa - v.z * sina, v.y * sina + v.z * cosa)
}

fn rotate_y(v: Vector3, angle: f64) -> Vector3 {
    let cosa = angle.cos();
    let sina = angle.sin();
    Vector3::new(v.x * cosa - v.z * sina, v.y, v.x * sina + v.z * cosa)
}

fn scale(v: Vector3, factor: f64) -> Vector3 {
    v * factor
}

fn perspective(v: Vector3, near: f64, far: f64) -> Vector3 {
    let w = far * near / (far - near) * v.z;
    Vector3::new(v.x, v.y, far / (far - near)) * (1.0 / w)
}


fn basic_perspective(v: Vector3, t: &(f64, Vector3)) -> (Vector3, f64) {
    (perspective(rotate_x(rotate_y(v, t.0), std::f64::consts::PI / 4.0) + Vector3::new(0.0, 0.0, 50.0), 0.1, 100.0) * 0.5 + Vector3::new(0.5, 0.5, 0.0), 0.1)
}

fn lighting(v: Vector3, _: f64, (t, n): &(f64, Vector3)) -> Vector3 {
    let normal = rotate_x(rotate_y(*n, *t), -std::f64::consts::PI / 4.0);
    let light_dir = Vector3::new(0.0, 0.0, -1.0);
    let view_dir = v * (1.0 / (v * v).sqrt());
    let reflect_dir = view_dir - normal * (2.0 * (view_dir * normal));

    let ambient = 0.3;
    let diff = 0.7 * (normal * light_dir).max(0.0);
    let spec = (view_dir * reflect_dir).max(0.0).powi(32);

    Vector3::new(0.4, 0.5, 0.0) * (ambient + diff + 0.5 * spec)
}


fn main() {
    let mut window = Window::new(
        "CPU Renderer",
        WIDTH,
        HEIGHT,
        WindowOptions { borderless: true, title: true, resize: false, scale: Scale::X4 },
    ).unwrap();
    let mut buffer = Framebuffer::new(WIDTH, HEIGHT);
    let mut pool = Pool::new(48);

    let mut program = Program::new(
        basic_perspective,
        lighting,
        (0.0, Vector3::new(0.0, 0.0, 0.0)),
        WIDTH,
        HEIGHT,
        30,
        30,
    );

    let m0 = Matrix4 {
        m00: 2.0,
        m11: 2.0,
        m22: 2.0,
        m33: 1.0,
        ..Matrix4::default()
    };
    let m1 = Matrix4 {
        m00: 1.0,
        m11: 1.0,
        m22: 1.0,
        m33: 1.0,
        m30: 1.0,
        m31: 1.0,
        m32: 1.0,
        ..Matrix4::default()
    };

    println!("{:?} {:?}", m0 * m1 * Vector4::new(0.0, 0.0, 1.0, 1.0), m1 * m0 * Vector4::new(0.0, 0.0, 1.0, 1.0));

    //println!("{:?}", program.regions);
    program.enqueue_triangle(Vector3::new(0.0, 0.0, 0.0), Vector3::new(0.0, 1.0, 0.0), Vector3::new(1.0, 0.0, 0.0));
    //program.enqueue_triangle(Vector3::new(0.0, 0.0, 1.0),Vector3::new(0.0, 0.5, 0.0), Vector3::new(1.0, 0.0, 0.0));
    //println!("{:?}", program.regions);

    let mut clock = std::time::Instant::now();
    while window.is_open() {
        buffer.clear(Vector3::zero());
        program.reset();
        program.uniform.0 += 1.0 / 90.0;
        program.uniform.1 = Vector3::new(0.0, 0.0, -1.0);


        // roller
        let n = 200.0;
        let theta = std::f64::consts::PI / (n / 2.0);
        for i in 0..=(n as usize) {
            let edge0 = rotate_y(Vector3::new(0.0, 0.0, 1.0), (i as f64) * theta);
            let edge1 = rotate_y(Vector3::new(0.0, 0.0, 1.0), (i as f64 + 1.0) * theta);

            program.uniform.1 = rotate_y(Vector3::new(0.0, 0.0, 1.0), (i as f64 + 0.5) * theta) * (-1.0);
            program.enqueue_triangle(Vector3::new(edge0.x, -1.0, edge0.z), Vector3::new(edge1.x, -1.0, edge1.z), Vector3::new(edge1.x, 1.0, edge1.z));
            program.enqueue_triangle(Vector3::new(edge0.x, 1.0, edge0.z), Vector3::new(edge0.x, -1.0, edge0.z), Vector3::new(edge1.x, 1.0, edge1.z));
        }


//        program.uniform.1 = Vector3::new(0.0, 0.0, 1.0);
//        program.enqueue_triangle(Vector3::new(-1.0, -1.0, -1.0),Vector3::new(-1.0, 1.0, -1.0), Vector3::new(1.0, -1.0, -1.0));
//        program.enqueue_triangle(Vector3::new(1.0, 1.0, -1.0),Vector3::new(-1.0, 1.0, -1.0), Vector3::new(1.0, -1.0, -1.0));
//        program.uniform.1 = Vector3::new(0.0, 0.0, -1.0);
//        program.enqueue_triangle(Vector3::new(-1.0, -1.0, 1.0),Vector3::new(-1.0, 1.0, 1.0), Vector3::new(1.0, -1.0, 1.0));
//        program.enqueue_triangle(Vector3::new(1.0, 1.0, 1.0),Vector3::new(-1.0, 1.0, 1.0), Vector3::new(1.0, -1.0, 1.0));
//        program.uniform.1 = Vector3::new(-1.0, 0.0, 0.0);
//        program.enqueue_triangle(Vector3::new(1.0, -1.0, -1.0),Vector3::new(1.0, 1.0, -1.0), Vector3::new(1.0, -1.0, 1.0));
//        program.enqueue_triangle(Vector3::new(1.0, 1.0, 1.0),Vector3::new(1.0, 1.0, -1.0), Vector3::new(1.0, -1.0, 1.0));
//        program.uniform.1 = Vector3::new(1.0, 0.0, 0.0);
//        program.enqueue_triangle(Vector3::new(-1.0, -1.0, -1.0),Vector3::new(-1.0, 1.0, -1.0), Vector3::new(-1.0, -1.0, 1.0));
//        program.enqueue_triangle(Vector3::new(-1.0, 1.0, 1.0),Vector3::new(-1.0, 1.0, -1.0), Vector3::new(-1.0, -1.0, 1.0));

        let mut i = 13;
        let mut renderers = &program.regions;
        let mut regions = buffer.regions(30, 30);


        let count_y = renderers.len();
        let count_x = renderers[0].len();

        pool.scoped(|scoped| {
            for y in 0..count_y {
                for x in 0..count_x {
                    let renderer = &renderers[y][x];
                    unsafe {
                        let mut region = &mut *((&mut regions).get_unchecked_mut(y).get_unchecked_mut(x) as *mut _);
                        scoped.execute(move || renderer.render_region(&mut region));
                    }
                }
            }
        });
        //break;

        //break;
        window.set_title(&(1000.0 / clock.elapsed().as_millis() as f64).to_string());
        buffer.finish_rendering();
        window.update_with_buffer(&buffer.colors()).unwrap();
        clock = std::time::Instant::now();
    }

    println!("Hello, world!");
}
