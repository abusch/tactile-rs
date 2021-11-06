use data::tiling_type_data;
use glam::{dvec2, DMat3, DVec2};
use iterators::{FillAlgorithm, TilingShapeIterator, TilingShapePartIterator};
use utils::{fill_matrix, fill_vector, r#match};

mod data;
mod iterators;
mod utils;

// Type aliases (TODO should they be newtypes?)
pub type EdgeID = u8;
pub type TilingType = u8;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum EdgeShape {
    /// Edges that can be of any shape
    J,
    /// Edges that must look the same after reflecting across their length (like the letter `U`)
    U,
    /// Edges that must look the same after a 180° rotation (like the letter `S`)
    S,
    /// Edges that must look the same after both a 180° and a reflection (like the letter `I`)
    I,
}

#[derive(Debug, Default)]
pub struct TilingTypeData {
    num_params: u8,
    num_aspects: u8,
    num_vertices: u8,
    num_edge_shapes: u8,

    edge_shapes: &'static [EdgeShape],
    edge_orientations: &'static [bool],
    edge_shape_ids: &'static [u8],
    default_params: &'static [f64],
    tiling_vertex_coeffs: &'static [f64],
    translation_vertex_coeffs: &'static [f64],
    aspect_xform_coeffs: &'static [f64],
    colouring: &'static [u8],
}

lazy_static::lazy_static! {
    static ref DEFAULT_TTD: TilingTypeData = TilingTypeData::default();
}

#[derive(Debug, Default)]
pub struct IsohedralTiling {
    tiling_type: TilingType,
    num_params: u8,
    parameters: [f64; 6],
    verts: [DVec2; 6],
    edges: [DMat3; 6],
    reversals: [bool; 6],
    aspects: [DMat3; 12],
    t1: DVec2,
    t2: DVec2,
    ttd: &'static TilingTypeData,
}

impl IsohedralTiling {
    pub fn new(ihtype: TilingType) -> Self {
        let mut tiling = Self::default();
        tiling.reset(ihtype);

        tiling
    }

    pub fn reset(&mut self, ihtype: TilingType) {
        self.tiling_type = ihtype;
        let ttd = &tiling_type_data[ihtype as usize];

        self.num_params = ttd.num_params;
        self.ttd = ttd;

        self.parameters[..ttd.num_params as usize].copy_from_slice(ttd.default_params);
        self.recompute();
    }
    // accessors

    pub fn tiling_type(&self) -> u8 {
        self.tiling_type
    }

    /// Get a reference to the isohedral tiling's num params.
    pub fn num_params(&self) -> u8 {
        self.num_params
    }

    pub fn num_edge_shapes(&self) -> u8 {
        self.ttd.num_edge_shapes
    }

    pub fn num_vertices(&self) -> u8 {
        self.ttd.num_vertices
    }

    pub fn edge_shape(&self, idx: EdgeID) -> EdgeShape {
        self.ttd.edge_shapes[idx as usize]
    }

    pub fn vertex(&self, idx: u8) -> &DVec2 {
        &self.verts[idx as usize]
    }

    pub fn num_aspects(&self) -> u8 {
        self.ttd.num_aspects
    }

    pub fn aspect_transform(&self, idx: usize) -> &DMat3 {
        &self.aspects[idx]
    }

    pub fn colour(&self, t1: isize, t2: isize, aspect: usize) -> u8 {
        let nc = self.ttd.colouring[18] as isize;

        let mut mt1 = t1 % nc;
        if mt1 < 0 {
            mt1 += nc;
        }
        let mut mt2 = t2 % nc;
        if mt2 < 0 {
            mt2 += nc;
        }
        let mut col = self.ttd.colouring[aspect];

        for _ in 0..mt1 {
            col = self.ttd.colouring[12 + col as usize];
        }

        for _ in 0..mt2 {
            col = self.ttd.colouring[15 + col as usize];
        }

        col
    }

    pub fn t1(&self) -> &DVec2 {
        &self.t1
    }

    pub fn t2(&self) -> &DVec2 {
        &self.t2
    }
    // iterators

    /// Iterate over all the shapes
    pub fn shapes(&self) -> TilingShapeIterator {
        TilingShapeIterator {
            idx: 0,
            tiling: self,
        }
    }

    /// Iterate over all the shape parts
    pub fn parts(&self) -> TilingShapePartIterator {
        TilingShapePartIterator {
            idx: 0,
            tiling: self,
            second: false,
        }
    }

    pub fn fill_region(&self, xmin: f64, ymin: f64, xmax: f64, ymax: f64) -> FillAlgorithm<'_> {
        FillAlgorithm::new(
            self,
            dvec2(xmin, ymin),
            dvec2(xmax, ymin),
            dvec2(xmax, ymax),
            dvec2(xmin, ymax),
        )
    }

    fn recompute(&mut self) {
        let ntv = self.ttd.num_vertices as usize;

        // Recompute tiling vertex locations
        let mut data = self.ttd.tiling_vertex_coeffs;
        for idx in 0..ntv {
            fill_vector(
                data,
                &self.parameters,
                self.num_params,
                &mut self.verts[idx],
            );
            data = &data[(2 * (self.num_params as usize + 1))..];
        }

        // Recompute edge transforms and reversals from orientation information
        for idx in 0..ntv {
            let fl = self.ttd.edge_orientations[2 * idx];
            let ro = self.ttd.edge_orientations[2 * idx + 1];
            self.reversals[idx] = fl != ro;
            self.edges[idx] = r#match(&self.verts[idx], &self.verts[(idx + 1) % ntv])
                * utils::M_ORIENTS[2 * (fl as usize) + (ro as usize)];
        }

        // Recompute aspect xforms
        data = self.ttd.aspect_xform_coeffs;
        let sz = self.ttd.num_aspects as usize;
        for idx in 0..sz {
            fill_matrix(
                data,
                &self.parameters,
                self.num_params,
                &mut self.aspects[idx],
            );
            data = &data[(6 * (self.num_params as usize + 1))..];
        }

        // Recompute translation vectors
        data = self.ttd.translation_vertex_coeffs;
        fill_vector(data, &self.parameters, self.num_params, &mut self.t1);
        fill_vector(
            &data[(2 * (self.num_params as usize + 1))..],
            &self.parameters,
            self.num_params,
            &mut self.t2,
        );
    }
}

impl Default for &'static TilingTypeData {
    fn default() -> Self {
        &DEFAULT_TTD
    }
}

#[cfg(test)]
mod tests {
    use crate::IsohedralTiling;

    #[test]
    fn it_works() {
        let tiling = IsohedralTiling::new(1);
        println!("{:?}", tiling.aspect_transform(1).to_cols_array());
        let mut cnt = 0;
        for v in tiling.fill_region(-5.0, -5.0, 5.0, 5.0).iter() {
            println!("t1={}, t2={}, transform={:?}", v.t1, v.t2, v.transform.to_cols_array());
            cnt += 1;
        }
        println!("Got {} tiles", cnt);
    }
}
