use data::tiling_type_data;
use glam::{DMat3, DVec2};
use iterators::{TilingShapeIterator, TilingShapePartIterator};
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
    tiling_vertex_coefficients: &'static [f64],
    edge_shape_ids: &'static [EdgeID],
    edge_shapes: &'static [EdgeShape],
    edge_shape_orientations: &'static [bool],
    aspect_xform_coefficients: &'static [f64],
    translation_vertex_coefficients: &'static [f64],
    colouring: &'static [u8],
}

impl IsohedralTiling {
    pub fn new(ihtype: TilingType) -> Self {
        let ttd = &tiling_type_data[ihtype as usize];

        let mut tiling = IsohedralTiling {
            tiling_type: ihtype,
            num_params: ttd.num_params,
            parameters: [0.0; 6],
            verts: [DVec2::default(); 6],
            edges: [DMat3::default(); 6],
            reversals: [false; 6],
            aspects: [DMat3::default(); 12],
            t1: DVec2::default(),
            t2: DVec2::default(),
            ttd,
            tiling_vertex_coefficients: ttd.tiling_vertex_coeffs,
            edge_shape_ids: ttd.edge_shape_ids,
            edge_shapes: ttd.edge_shapes,
            edge_shape_orientations: ttd.edge_orientations,
            aspect_xform_coefficients: ttd.aspect_xform_coeffs,
            translation_vertex_coefficients: ttd.translation_vertex_coeffs,
            colouring: ttd.colouring,
        };
        tiling.parameters[..ttd.num_params as usize].copy_from_slice(ttd.default_params);
        tiling.recompute();
        tiling
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
        self.edge_shapes[idx as usize]
    }

    pub fn vertex(&self, idx: u8) -> &DVec2 {
        &self.verts[idx as usize]
    }

    pub fn num_aspects(&self) -> u8 {
        self.ttd.num_aspects
    }

    pub fn aspect_transform(&self, idx: u8) -> &DMat3 {
        &self.aspects[idx as usize]
    }

    pub fn colour(&self, t1: isize, t2: isize, aspect: u8) -> u8 {
        let nc = self.colouring[18] as isize;

        let mut mt1 = t1 % nc;
        if mt1 < 0 {
            mt1 += nc;
        }
        let mut mt2 = t2 % nc;
        if mt2 < 0 {
            mt2 += nc;
        }
        let mut col = self.colouring[aspect as usize];

        for _ in 0..mt1 {
            col = self.colouring[12 + col as usize];
        }

        for _ in 0..mt2 {
            col = self.colouring[15 + col as usize];
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

    fn recompute(&mut self) {
        let ntv = self.ttd.num_vertices as usize;

        // Recompute tiling vertex locations
        let mut data = self.tiling_vertex_coefficients;
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
            let fl = self.edge_shape_orientations[2 * idx];
            let ro = self.edge_shape_orientations[2 * idx + 1];
            self.reversals[idx] = fl != ro;
            self.edges[idx] = r#match(&self.verts[idx], &self.verts[(idx + 1) % ntv])
                * utils::M_ORIENTS[2 * (fl as usize) + (ro as usize)];
        }

        // Recompute aspect xforms
        data = self.aspect_xform_coefficients;
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
        data = self.translation_vertex_coefficients;
        fill_vector(data, &self.parameters, self.num_params, &mut self.t1);
        fill_vector(
            &data[(2 * (self.num_params as usize + 1))..],
            &self.parameters,
            self.num_params,
            &mut self.t1,
        );
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
