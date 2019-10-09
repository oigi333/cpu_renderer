use std::marker::{PhantomData, PhantomPinned};
use std::pin::Pin;

use crate::texture::Texture;
use crate::vector::Vector2;

use super::utils::clamp;
use super::vector::Vector3;

pub struct Framebuffer {
    buffer: Vec<u32>,
    colors: Vec<Vector3>,
    depth_bits: Vec<f64>,
    pub width: usize,
    pub height: usize,
    _pinned: PhantomPinned,
}

impl Framebuffer {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            colors: vec![Vector3::zero(); width * height],
            buffer: vec![0; width * height],
            depth_bits: vec![std::f64::INFINITY; width * height],
            width,
            height,
            _pinned: PhantomPinned,
        }
    }

    pub fn set_color(&mut self, x: usize, y: usize, color: Vector3, depth: f64) {
        let index = self.pos_to_index(x, y);
        if self.depth_bits[index] > depth {
            self.colors[index] = color;
            self.depth_bits[index] = depth;
        }
    }

    pub fn clear(&mut self, clear_color: Vector3) {
        for color in self.colors.iter_mut() {
            *color = clear_color;
        }
        for depth in self.depth_bits.iter_mut() {
            *depth = std::f64::INFINITY;
        }
    }

    pub fn finish_rendering(&mut self) {
        self.buffer = self.colors.iter().map(|c| {
            let r = clamp(c.x, 0., 1.) * 255.0;
            let g = clamp(c.y, 0., 1.) * 255.0;
            let b = clamp(c.z, 0., 1.) * 255.0;
            r as u32 * 256 * 256 + g as u32 * 256 + b as u32
        }).collect();
    }

    pub fn colors(&self) -> &[u32] {
        &self.buffer
    }

    pub fn regions<'a>(&'a mut self, region_width: usize, region_height: usize) -> Vec<Vec<RegionBuffer<'a>>> {
        let width = self.width;
        let height = self.height;

        let count_x = if width % region_width != 0 {
            width / region_width + 1
        } else {
            width / region_width
        };
        let count_y = if self.height % region_height != 0 {
            height / region_height + 1
        } else {
            height / region_height
        };

        let mut regions = vec![vec![RegionBuffer::without_dimensions(self); count_x]; count_y];

        for x in 0..count_x {
            for y in 0..count_y {
                regions[y][x].from_x = x * region_width;
                regions[y][x].from_y = y * region_height;
                regions[y][x].height = region_height;
                regions[y][x].width = region_height;
            }
            if height % region_height != 0 {
                regions[count_y - 1][x].height = height % region_height;
            }
        }

        if width % region_width != 0 {
            for y in 0..count_y {
                regions[y][count_x - 1].width = width % region_width;
            }
        }

        regions
    }


    fn pos_to_index(&self, x: usize, y: usize) -> usize {
        y * self.width + x
    }
}

impl Texture for Framebuffer {
    fn sample(&self, uv: Vector2) -> Vector3 {
        let x = (uv.x * (self.width as f64 - 1.0)) as usize;
        let y = (uv.y * (self.height as f64 - 1.0)) as usize;
        let index = self.pos_to_index(x, y);
        self.colors[index]
    }
}

#[derive(Clone)]
pub struct RegionBuffer<'a> {
    framebuffer: *mut Framebuffer,
    from_x: usize,
    from_y: usize,
    width: usize,
    height: usize,
    _marker: PhantomData<&'a ()>,
}


impl<'a> RegionBuffer<'a> {
    pub fn without_dimensions(framebuffer: &'a mut Framebuffer) -> Self {
        Self {
            framebuffer: framebuffer as *mut Framebuffer,
            from_x: 0,
            from_y: 0,
            width: 0,
            height: 0,
            _marker: PhantomData,
        }
    }

    pub fn set_color(&mut self, x: usize, y: usize, color: Vector3, depth: f64) {
        if x >= self.from_x && x < self.from_x + self.width && y >= self.from_y && y < self.from_y + self.height {
            unsafe {
                (*self.framebuffer).set_color(x, y, color, depth);
            }
        }
    }
}

unsafe impl<'a> Send for RegionBuffer<'a> {}