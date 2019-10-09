use std::marker::PhantomData;
use std::ops::{Add, Mul};

use crate::framebuffer::{Framebuffer, RegionBuffer};
use crate::vector::Vector3;

#[derive(Clone, Debug)]
struct Triangle {
    indices: (usize, usize, usize),
    // cached values
    d00: f64,
    d01: f64,
    d11: f64,
    inv: f64,
    zx: f64,
    zy: f64,
    v0: Vector3,
    v1: Vector3,
    a: Vector3,
}

impl Triangle {
    pub fn interpolate<Attr>(&self, point: Vector3, a0: &Attr, a1: &Attr, a2: &Attr) -> Option<Attr>
        where
                for<'a> &'a Attr: Add<Output=Attr> + Mul<f64, Output=Attr>,
                Attr: Add<Output=Attr> {
        let v2 = point - self.a;
        let d20 = v2 * self.v0;
        let d21 = v2 * self.v1;

        let v = (self.d11 * d20 - self.d01 * d21) * self.inv;
        let w = (self.d00 * d21 - self.d01 * d20) * self.inv;
        let u = 1.0 - v - w;

        if v < 0.0 || w < 0.0 || u < 0.0 {
            None
        } else {
            Some(a0 * v + a1 * w + a2 * u)
        }
    }
}


pub struct Program<In, U, Attr>
    where
        U: Clone,
        for<'a> &'a Attr: Add<&'a Attr, Output=Attr> + Mul<f64, Output=Attr>,
        Attr: Add<Attr, Output=Attr> + Mul<f64, Output=Attr> + Clone {
    vertex_shader: fn(In, &U) -> (Vector3, Attr),
    fragment_shader: fn(Vector3, Attr, &U) -> Vector3,
    pub uniform: U,
    width: usize,
    height: usize,
    region_width: usize,
    region_height: usize,
    pub regions: Vec<Vec<RegionRenderer<U, Attr>>>,
    _marker: PhantomData<In>,
}

impl<In, U, Attr> Program<In, U, Attr>
    where
        U: Clone,
        Attr: Add<Output=Attr> + Clone + Mul<f64, Output=Attr>,
        for<'a> &'a Attr: Add<Output=Attr> + Clone + Mul<f64, Output=Attr> {
    pub fn new(
        vertex_shader: fn(In, &U) -> (Vector3, Attr),
        fragment_shader: fn(Vector3, Attr, &U) -> Vector3,
        uniform: U,
        width: usize,
        height: usize,
        region_width: usize,
        region_height: usize,
    ) -> Self {
        let count_x = if width % region_width != 0 {
            width / region_width + 1
        } else {
            width / region_width
        };
        let count_y = if height % region_height != 0 {
            height / region_height + 1
        } else {
            height / region_height
        };

        //println!("{} {}", count_x, count_y);

        let mut regions =
            vec![vec![RegionRenderer::without_dimensions(fragment_shader, width, height, region_width, region_height); count_x]; count_y];

        for x in 0..count_x {
            for y in 0..count_y {
                regions[y][x].from = (x * region_width, y * region_height);
            }
        }

        Self {
            vertex_shader,
            fragment_shader,
            uniform,
            width,
            height,
            region_width,
            region_height,
            regions,
            _marker: PhantomData,
        }
    }

    pub fn enqueue_triangle(&mut self, i0: In, i1: In, i2: In) {
        let uniform = &self.uniform;
        let v0 = (self.vertex_shader)(i0, uniform);
        let v1 = (self.vertex_shader)(i1, uniform);
        let v2 = (self.vertex_shader)(i2, uniform);

        let x_min = v0.0.x.min(v1.0.x.min(v2.0.x));
        let x_max = v0.0.x.max(v1.0.x.max(v2.0.x));
        let y_min = v0.0.y.min(v1.0.y.min(v2.0.y));
        let y_max = v0.0.y.max(v1.0.y.max(v2.0.y));

        if x_max < 0.0 || x_min > 1.0 || y_max < 0.0 || y_min > 1.0 {
            return;
        }

        let width = self.width as f64 - 1.0;
        let height = self.height as f64 - 1.0;


        let x_min = (x_min.max(0.0) * width) as usize;
        let y_min = (y_min.max(0.0) * height) as usize;
        let x_max = (x_max.min(1.0) * width) as usize;
        let y_max = (y_max.min(1.0) * height) as usize;

        let from_region_x = x_min / self.region_width;
        let from_region_y = y_min / self.region_height;
        let to_region_x = x_max / self.region_width + 1;
        let to_region_y = y_max / self.region_height + 1;

// Compute interpolation coefficients
        let edge0 = v1.0 - v0.0;
        let edge1 = v2.0 - v0.0;
// Semimagic values
        let d00 = edge0 * edge0;
        let d01 = edge0 * edge1;
        let d11 = edge1 * edge1;
        let inv = 1.0 / (d00 * d11 - d01 * d01);

        let invz = 1.0 / (edge1.x * edge0.y - edge1.y * edge0.x);
        let zx = (edge0.y * edge1.z - edge1.y * edge0.z) * invz;
        let zy = (edge1.x * edge0.z - edge0.x * edge1.z) * invz;


        for y in from_region_y..to_region_y {
            for x in from_region_x..to_region_x {
                let region = self.regions.get_mut(y).unwrap().get_mut(x).unwrap();

                let attr_len = region.triangles_attrs.len();
                region.triangles.push(Triangle {
                    indices: (attr_len, attr_len + 1, attr_len + 2),
                    d00,
                    d01,
                    d11,
                    zx,
                    zy,
                    v0: edge0,
                    v1: edge1,
                    a: v0.0,
                    inv,
                });
                region.triangles_attrs.push(v0.1.clone());
                region.triangles_attrs.push(v1.1.clone());
                region.triangles_attrs.push(v2.1.clone());
                region.vertices_positions.push(v0.0);
                region.vertices_positions.push(v1.0);
                region.vertices_positions.push(v2.0);
                region.uniforms.push(uniform.clone());
            }
        }
    }

    pub fn reset(&mut self) {
        let count_y = self.regions.len();
        let count_x = self.regions[0].len();
        self.regions = vec![vec![RegionRenderer::without_dimensions(self.fragment_shader, self.width, self.height, self.region_width, self.region_height); count_x]; count_y];


        for x in 0..count_x {
            for y in 0..count_y {
                self.regions[y][x].from = (x * self.region_width, y * self.region_height);
            }
            if self.height % self.region_height != 0 {
                self.regions[count_y - 1][x].region_height = self.height % self.region_height;
            }
        }

        if self.width % self.region_width != 0 {
            for y in 0..count_y {
                self.regions[y][count_x - 1].region_width = self.width % self.region_width;
            }
        }
    }

    pub fn region_renderers(&mut self) -> Vec<Vec<&mut dyn RenderRegion>> {
        self.regions.iter_mut().map(|row| row
            .iter_mut()
            .map(|rr| rr as &mut dyn RenderRegion)
            .collect()
        ).collect()
    }
}

#[derive(Clone)]
pub struct RegionRenderer<U, Attr>
    where
        U: Clone,
        Attr: Add<Output=Attr> + Clone + Mul<f64, Output=Attr>,
        for<'a> &'a Attr: Add<Output=Attr> + Clone + Mul<f64, Output=Attr> {
    uniforms: Vec<U>,
    triangles_attrs: Vec<Attr>,
    vertices_positions: Vec<Vector3>,
    triangles: Vec<Triangle>,
    fragment_shader: fn(Vector3, Attr, &U) -> Vector3,
    from: (usize, usize),
    width: usize,
    height: usize,
    region_width: usize,
    region_height: usize,
}

impl<U, Attr> RegionRenderer<U, Attr>
    where
        U: Clone,
        Attr: Add<Output=Attr> + Clone + Mul<f64, Output=Attr>,
        for<'a> &'a Attr: Add<Output=Attr> + Clone + Mul<f64, Output=Attr> {
    pub fn without_dimensions(fragment_shader: fn(Vector3, Attr, &U) -> Vector3, width: usize, height: usize, region_width: usize, region_height: usize) -> Self {
        Self {
            uniforms: Vec::new(),
            triangles_attrs: Vec::new(),
            triangles: Vec::new(),
            vertices_positions: Vec::new(),
            fragment_shader,
            from: (0, 0),
            width,
            height,
            region_width,
            region_height,
        }
    }
}

pub trait RenderRegion {
    fn render_region(&self, buffer: &mut RegionBuffer);
}


impl<U, Attr> RenderRegion for RegionRenderer<U, Attr>
    where
        U: Clone,
        Attr: Add<Output=Attr> + Clone + Mul<f64, Output=Attr>,
        for<'a> &'a Attr: Add<Output=Attr> + Clone + Mul<f64, Output=Attr> {
    fn render_region(&self, buffer: &mut RegionBuffer) {
        let mut i = 0;
        for triangle in self.triangles.iter() {
            //println!("{} {} {} {}", self.from.0, (self.from.0 + self.region_width), self.from.1, (self.from.1 + self.region_height));
            let a0 = &self.triangles_attrs[triangle.indices.0];
            let a1 = &self.triangles_attrs[triangle.indices.1];
            let a2 = &self.triangles_attrs[triangle.indices.2];
            for y in self.from.1..(self.from.1 + self.region_height) {
                for x in self.from.0..(self.from.0 + self.region_width) {
                    let norm_x = x as f64 / self.width as f64;
                    let norm_y = y as f64 / self.height as f64;

                    //println!("{} {} {}", norm_x, norm_y, triangle.zx * norm_x + triangle.zy * norm_y);
                    let point = Vector3::new(norm_x, norm_y, triangle.a.z + triangle.zx * (norm_x - triangle.a.x) + triangle.zy * (norm_y - triangle.a.y));
                    let interpolated = triangle.interpolate(point, &a0, &a1, &a2);

                    if let Some(attr) = interpolated {
                        buffer.set_color(x, y, (self.fragment_shader)(point, attr, &self.uniforms[i]), point.z);
                    }
                }
            }
            i += 1;
        }
    }
}