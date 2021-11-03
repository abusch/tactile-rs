use glam::{dmat3, dvec3, DMat3, DVec2};
use lazy_static::lazy_static;

// Utility functions
pub(crate) fn ddot(coeffs: &[f64], params: &[f64], np: u8) -> f64 {
    let mut total = 0.0;
    for idx in 0..np as usize {
        total += coeffs[idx] * params[idx];
    }
    // Affine term
    total += coeffs[np as usize];
    total
}

pub(crate) fn fill_vector(coeffs: &[f64], params: &[f64], np: u8, v: &mut DVec2) {
    v.x = ddot(coeffs, params, np);
    v.y = ddot(&coeffs[(np as usize + 1)..], params, np);
}

pub(crate) fn fill_matrix(coeffs: &[f64], params: &[f64], np: u8, m: &mut DMat3) {
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

pub(crate) fn r#match(p: &DVec2, q: &DVec2) -> DMat3 {
    dmat3(
        dvec3(q.x - p.x, q.y - p.y, 0.0),
        dvec3(p.y - q.y, q.x - p.x, 0.0),
        dvec3(p.x, p.y, 1.0),
    )
}

lazy_static! {
pub(crate) static ref M_ORIENTS: [DMat3; 4] = [
    dmat3(dvec3(1.0, 0.0, 0.0), dvec3(0.0, 1.0, 0.0), dvec3(0.0, 0.0, 1.0)),   // IDENTITY
    dmat3(dvec3(-1.0, 0.0, 0.0), dvec3(0.0, -1.0, 0.0), dvec3(1.0, 0.0, 1.0)), // ROT
    dmat3(dvec3(-1.0, 0.0, 0.0), dvec3(0.0, 1.0, 0.0), dvec3(1.0, 0.0, 1.0)),  // FLIP
    dmat3(dvec3(1.0, 0.0, 0.0), dvec3(0.0, -1.0, 0.0), dvec3(0.0, 0.0, 1.0)),  // ROFL
];
}

lazy_static! {
pub(crate) static ref TSPI_U: [DMat3; 2] = [
    dmat3(dvec3(0.5, 0.0, 0.0), dvec3(0.0, 0.5, 0.0), dvec3(0.0, 0.0, 1.0)),
    dmat3(dvec3(-0.5, 0.0, 0.0), dvec3(0.0, 0.5, 0.0), dvec3(1.0, 0.0, 1.0)),
];
}

lazy_static! {
pub(crate) static ref TSPI_S: [DMat3; 2] = [
    dmat3(dvec3(0.5, 0.0, 0.0), dvec3(0.0, 0.5, 0.0), dvec3(0.0, 0.0, 1.0)),
    dmat3(dvec3(-0.5, 0.0, 0.0), dvec3(0.0, -0.5, 0.0), dvec3(1.0, 0.0, 1.0)),
];
}
