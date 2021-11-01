use data::tiling_type_data;
use glam::{dmat3, dvec3, DMat3, DVec2};

mod data;

// Type aliases (TODO should they be newtypes?)
pub type EdgeID = u8;
pub type TilingType = u8;

pub enum EdgeShape {
    J,
    U,
    S,
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

    pub fn shape(&self) -> TilingShapeIterator {
        todo!()
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
                * M_ORIENTS[2 * (fl as usize) + (ro as usize)];
        }

        // Recompute aspect xforms
        data = self.aspect_xform_coefficients;
        let sz = self.ttd.num_aspects as usize;
        for idx in 0..sz {
            fill_matrix(data, &self.parameters, self.num_params, &mut self.aspects[idx]);
            data = &data[(6 * (self.num_params as usize + 1))..];
        }

        // Recompute translation vectors
        data = self.translation_vertex_coefficients;
        fill_vector(data, &self.parameters, self.num_params, &mut self.t1);
        fill_vector(&data[(2 * (self.num_params as usize + 1))..], &self.parameters, self.num_params, &mut self.t1);
    }
}

// Utility functions
fn ddot(coeffs: &[f64], params: &[f64], np: u8) -> f64 {
    let mut total = 0.0;
    for idx in 0..np as usize {
        total += coeffs[idx] * params[idx];
    }
    // Affine term
    total += coeffs[np as usize];
    total
}

fn fill_vector(coeffs: &[f64], params: &[f64], np: u8, v: &mut DVec2) {
    v.x = ddot(coeffs, params, np);
    v.y = ddot(&coeffs[(np as usize + 1)..], params, np);
}

fn fill_matrix(coeffs: &[f64], params: &[f64], np: u8, m: &mut DMat3) {
    let mut coeffs = coeffs;
    for row in 0..2 {
        for col in 0..3 {
            m.col_mut(col)[row] = ddot(coeffs, params, np);
            coeffs = &coeffs[(np as usize + 1)..];
        }
    }
    m.col_mut(0)[2] = 0.0;
    m.col_mut(1)[2] = 0.0;
    m.col_mut(2)[2] = 1.0;
}

fn r#match(p: &DVec2, q: &DVec2) -> DMat3 {
    dmat3(
        dvec3(q.x - p.x, q.y - p.y, 0.0),
        dvec3(p.y - q.y, q.x - p.x, 0.0),
        dvec3(p.x, p.y, 1.0),
    )
}

lazy_static::lazy_static! {
static ref M_ORIENTS: [DMat3; 4] = [
    dmat3(dvec3(1.0, 0.0, 0.0), dvec3(0.0, 1.0, 0.0), dvec3(0.0, 0.0, 1.0)),   // IDENTITY
    dmat3(dvec3(-1.0, 0.0, 0.0), dvec3(0.0, -1.0, 0.0), dvec3(1.0, 0.0, 1.0)), // ROT
    dmat3(dvec3(-1.0, 0.0, 0.0), dvec3(0.0, 1.0, 0.0), dvec3(1.0, 0.0, 1.0)),  // FLIP
    dmat3(dvec3(1.0, 0.0, 0.0), dvec3(0.0, -1.0, 0.0), dvec3(0.0, 0.0, 1.0)),  // ROFL
];
}

pub struct TilingShapeIterator {}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
