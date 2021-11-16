use glam::{const_dmat3, dmat3, dvec3, DMat3, DVec2};

// Utility functions
pub(crate) fn ddot(coeffs: &[f64], params: &[f64], np: usize) -> f64 {
    let mut total = 0.0;
    for idx in 0..np {
        total += coeffs[idx] * params[idx];
    }
    // Affine term
    total += coeffs[np];
    total
}

pub(crate) fn fill_vector(coeffs: &[f64], params: &[f64], np: usize, v: &mut DVec2) {
    v.x = ddot(coeffs, params, np);
    v.y = ddot(&coeffs[(np + 1)..], params, np);
}

pub(crate) fn fill_matrix(coeffs: &[f64], params: &[f64], np: usize, m: &mut DMat3) {
    let mut coeffs = coeffs;
    for row in 0..2 {
        for col in 0..3 {
            m.col_mut(col)[row] = ddot(coeffs, params, np);
            coeffs = &coeffs[(np + 1)..];
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

pub(crate) static M_ORIENTS: [DMat3; 4] = [
    const_dmat3!([1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]), // IDENTITY
    const_dmat3!([-1.0, 0.0, 0.0], [0.0, -1.0, 0.0], [1.0, 0.0, 1.0]), // ROT
    const_dmat3!([-1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [1.0, 0.0, 1.0]), // FLIP
    const_dmat3!([1.0, 0.0, 0.0], [0.0, -1.0, 0.0], [0.0, 0.0, 1.0]), // ROFL
];

pub(crate) static TSPI_U: [DMat3; 2] = [
    const_dmat3!([0.5, 0.0, 0.0], [0.0, 0.5, 0.0], [0.0, 0.0, 1.0]),
    const_dmat3!([-0.5, 0.0, 0.0], [0.0, 0.5, 0.0], [1.0, 0.0, 1.0]),
];

pub(crate) static TSPI_S: [DMat3; 2] = [
    const_dmat3!([0.5, 0.0, 0.0], [0.0, 0.5, 0.0], [0.0, 0.0, 1.0]),
    const_dmat3!([-0.5, 0.0, 0.0], [0.0, -0.5, 0.0], [1.0, 0.0, 1.0]),
];
