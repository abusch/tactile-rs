#![allow(non_upper_case_globals, dead_code)]
use crate::{EdgeShape, TilingTypeData};
use crate::EdgeShape::*;


pub static TILING_TYPES: [u8; 81] = [
	1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 61, 62, 64, 66, 67, 68, 69, 71, 72, 73, 74, 76, 77, 78, 79, 81, 82, 83, 84, 85, 86, 88, 90, 91, 93
];

pub(crate) static edge_shapes_array_00: &'static [EdgeShape] = &[ J, J, J ];
pub(crate) static edge_shapes_array_01: &'static [EdgeShape] = &[ S, J, S, S, S ];
pub(crate) static edge_shapes_array_02: &'static [EdgeShape] = &[ S, J, J, S ];
pub(crate) static edge_shapes_array_03: &'static [EdgeShape] = &[ S, J, S, J ];
pub(crate) static edge_shapes_array_04: &'static [EdgeShape] = &[ S, S, S ];
pub(crate) static edge_shapes_array_05: &'static [EdgeShape] = &[ S, J ];
pub(crate) static edge_shapes_array_06: &'static [EdgeShape] = &[ J ];
pub(crate) static edge_shapes_array_07: &'static [EdgeShape] = &[ S ];
pub(crate) static edge_shapes_array_08: &'static [EdgeShape] = &[ U, J ];
pub(crate) static edge_shapes_array_09: &'static [EdgeShape] = &[ U, S, S ];
pub(crate) static edge_shapes_array_10: &'static [EdgeShape] = &[ J, I ];
pub(crate) static edge_shapes_array_11: &'static [EdgeShape] = &[ S, I, S ];
pub(crate) static edge_shapes_array_12: &'static [EdgeShape] = &[ I, J ];
pub(crate) static edge_shapes_array_13: &'static [EdgeShape] = &[ I, S ];
pub(crate) static edge_shapes_array_14: &'static [EdgeShape] = &[ U ];
pub(crate) static edge_shapes_array_15: &'static [EdgeShape] = &[ I ];
pub(crate) static edge_shapes_array_16: &'static [EdgeShape] = &[ S, J, J ];
pub(crate) static edge_shapes_array_17: &'static [EdgeShape] = &[ J, J, I ];
pub(crate) static edge_shapes_array_18: &'static [EdgeShape] = &[ S, S, J, S ];
pub(crate) static edge_shapes_array_19: &'static [EdgeShape] = &[ S, S, J, I ];
pub(crate) static edge_shapes_array_20: &'static [EdgeShape] = &[ J, J, S ];
pub(crate) static edge_shapes_array_21: &'static [EdgeShape] = &[ S, I, I ];
pub(crate) static edge_shapes_array_22: &'static [EdgeShape] = &[ J, I, I ];
pub(crate) static edge_shapes_array_23: &'static [EdgeShape] = &[ J, J ];
pub(crate) static edge_shapes_array_24: &'static [EdgeShape] = &[ I, I ];
pub(crate) static edge_shapes_array_25: &'static [EdgeShape] = &[ J, S ];
pub(crate) static edge_shapes_array_26: &'static [EdgeShape] = &[ S, S, S, S ];
pub(crate) static edge_shapes_array_27: &'static [EdgeShape] = &[ J, S, S ];
pub(crate) static edge_shapes_array_28: &'static [EdgeShape] = &[ I, S, I, S ];
pub(crate) static edge_shapes_array_29: &'static [EdgeShape] = &[ J, I, S ];
pub(crate) static edge_shapes_array_30: &'static [EdgeShape] = &[ I, I, I, S ];
pub(crate) static edge_shapes_array_31: &'static [EdgeShape] = &[ S, S ];
pub(crate) static edge_shapes_array_32: &'static [EdgeShape] = &[ S, I ];
pub(crate) static edge_shapes_array_33: &'static [EdgeShape] = &[ U, I ];
pub(crate) static edge_shapes_array_34: &'static [EdgeShape] = &[ U, S ];
pub(crate) static edge_shapes_array_35: &'static [EdgeShape] = &[ I, I, I ];
pub(crate) static edge_shapes_array_36: &'static [EdgeShape] = &[ I, S, I ];
pub(crate) static edge_shapes_array_37: &'static [EdgeShape] = &[ I, S, S ];

pub(crate) static edge_shape_ids_array_00: &'static[u8] = &[ 0, 1, 2, 0, 1, 2 ];
pub(crate) static edge_shape_ids_array_01: &'static[u8] = &[ 0, 0, 1, 2, 2, 1 ];
pub(crate) static edge_shape_ids_array_02: &'static[u8] = &[ 0, 1, 0, 2, 1, 2 ];
pub(crate) static edge_shape_ids_array_03: &'static[u8] = &[ 0, 1, 2, 3, 1, 4 ];
pub(crate) static edge_shape_ids_array_04: &'static[u8] = &[ 0, 1, 2, 2, 1, 3 ];
pub(crate) static edge_shape_ids_array_05: &'static[u8] = &[ 0, 1, 2, 3, 1, 3 ];
pub(crate) static edge_shape_ids_array_06: &'static[u8] = &[ 0, 0, 1, 1, 2, 2 ];
pub(crate) static edge_shape_ids_array_07: &'static[u8] = &[ 0, 1, 1, 0, 1, 1 ];
pub(crate) static edge_shape_ids_array_08: &'static[u8] = &[ 0, 0, 0, 0, 0, 0 ];
pub(crate) static edge_shape_ids_array_09: &'static[u8] = &[ 0, 1, 2, 0, 2, 1 ];
pub(crate) static edge_shape_ids_array_10: &'static[u8] = &[ 0, 1, 0, 0, 1, 0 ];
pub(crate) static edge_shape_ids_array_11: &'static[u8] = &[ 0, 1, 2, 2, 1, 0 ];
pub(crate) static edge_shape_ids_array_12: &'static[u8] = &[ 0, 1, 1, 1, 1, 0 ];
pub(crate) static edge_shape_ids_array_13: &'static[u8] = &[ 0, 1, 1, 2, 2 ];
pub(crate) static edge_shape_ids_array_14: &'static[u8] = &[ 0, 0, 1, 2, 1 ];
pub(crate) static edge_shape_ids_array_15: &'static[u8] = &[ 0, 1, 2, 3, 2 ];
pub(crate) static edge_shape_ids_array_16: &'static[u8] = &[ 0, 1, 2, 1, 2 ];
pub(crate) static edge_shape_ids_array_17: &'static[u8] = &[ 0, 1, 1, 1, 1 ];
pub(crate) static edge_shape_ids_array_18: &'static[u8] = &[ 0, 1, 2, 0 ];
pub(crate) static edge_shape_ids_array_19: &'static[u8] = &[ 0, 1, 1, 0 ];
pub(crate) static edge_shape_ids_array_20: &'static[u8] = &[ 0, 0, 0, 0 ];
pub(crate) static edge_shape_ids_array_21: &'static[u8] = &[ 0, 1, 0 ];
pub(crate) static edge_shape_ids_array_22: &'static[u8] = &[ 0, 1, 0, 1 ];
pub(crate) static edge_shape_ids_array_23: &'static[u8] = &[ 0, 1, 0, 2 ];
pub(crate) static edge_shape_ids_array_24: &'static[u8] = &[ 0, 0, 1, 1 ];
pub(crate) static edge_shape_ids_array_25: &'static[u8] = &[ 0, 1, 2, 3 ];
pub(crate) static edge_shape_ids_array_26: &'static[u8] = &[ 0, 0, 1, 2 ];
pub(crate) static edge_shape_ids_array_27: &'static[u8] = &[ 0, 1, 2 ];
pub(crate) static edge_shape_ids_array_28: &'static[u8] = &[ 0, 0, 1 ];
pub(crate) static edge_shape_ids_array_29: &'static[u8] = &[ 0, 0, 0 ];

pub(crate) static edge_orientations_array_00: &'static [bool] = &[ false, false, false, false, false, false, false, true, false, true, false, true ];
pub(crate) static edge_orientations_array_01: &'static [bool] = &[ false, false, true, true, false, false, false, false, true, true, false, true ];
pub(crate) static edge_orientations_array_02: &'static [bool] = &[ false, false, false, false, true, true, false, false, false, true, true, true ];
pub(crate) static edge_orientations_array_03: &'static [bool] = &[ false, false, false, false, false, false, false, false, false, true, false, false ];
pub(crate) static edge_orientations_array_04: &'static [bool] = &[ false, false, false, false, false, false, true, true, false, true, false, false ];
pub(crate) static edge_orientations_array_05: &'static [bool] = &[ false, false, false, false, false, false, false, false, true, true, true, true ];
pub(crate) static edge_orientations_array_06: &'static [bool] = &[ false, false, false, true, false, false, false, true, false, false, false, true ];
pub(crate) static edge_orientations_array_07: &'static [bool] = &[ false, false, false, false, false, false, false, false, false, false, false, false ];
pub(crate) static edge_orientations_array_08: &'static [bool] = &[ false, false, false, false, true, true, false, false, false, false, true, true ];
pub(crate) static edge_orientations_array_09: &'static [bool] = &[ false, false, false, false, true, true, false, true, false, true, true, false ];
pub(crate) static edge_orientations_array_10: &'static [bool] = &[ false, false, false, false, false, false, false, true, true, false, true, false ];
pub(crate) static edge_orientations_array_11: &'static [bool] = &[ false, false, false, false, true, true, false, true, true, false, true, false ];
pub(crate) static edge_orientations_array_12: &'static [bool] = &[ false, false, false, false, false, false, true, false, true, false, true, false ];
pub(crate) static edge_orientations_array_13: &'static [bool] = &[ false, false, false, false, false, true, true, true, true, false, true, false ];
pub(crate) static edge_orientations_array_14: &'static [bool] = &[ false, false, false, false, true, false, false, false, false, false, true, false ];
pub(crate) static edge_orientations_array_15: &'static [bool] = &[ false, false, false, false, false, true, false, false, false, true ];
pub(crate) static edge_orientations_array_16: &'static [bool] = &[ false, false, true, true, false, false, false, false, false, true ];
pub(crate) static edge_orientations_array_17: &'static [bool] = &[ false, false, false, false, false, false, false, false, false, true ];
pub(crate) static edge_orientations_array_18: &'static [bool] = &[ false, false, true, false, false, false, false, false, true, false ];
pub(crate) static edge_orientations_array_19: &'static [bool] = &[ false, false, false, false, false, false, true, true, true, true ];
pub(crate) static edge_orientations_array_20: &'static [bool] = &[ false, false, false, false, false, true, true, true, true, false ];
pub(crate) static edge_orientations_array_21: &'static [bool] = &[ false, false, false, false, false, false, false, true ];
pub(crate) static edge_orientations_array_22: &'static [bool] = &[ false, false, false, false, false, true, false, true ];
pub(crate) static edge_orientations_array_23: &'static [bool] = &[ false, false, false, false, true, false, true, false ];
pub(crate) static edge_orientations_array_24: &'static [bool] = &[ false, false, false, true, false, false, false, true ];
pub(crate) static edge_orientations_array_25: &'static [bool] = &[ false, false, true, false, true, true, false, true ];
pub(crate) static edge_orientations_array_26: &'static [bool] = &[ false, false, true, false, false, false, true, false ];
pub(crate) static edge_orientations_array_27: &'static [bool] = &[ false, false, false, false, false, true ];
pub(crate) static edge_orientations_array_28: &'static [bool] = &[ false, false, false, false, true, false ];
pub(crate) static edge_orientations_array_29: &'static [bool] = &[ false, false, false, false, false, true, false, false ];
pub(crate) static edge_orientations_array_30: &'static [bool] = &[ false, false, false, false, false, true, true, true ];
pub(crate) static edge_orientations_array_31: &'static [bool] = &[ false, false, true, true, false, false, true, true ];
pub(crate) static edge_orientations_array_32: &'static [bool] = &[ false, false, false, false, true, true, false, false ];
pub(crate) static edge_orientations_array_33: &'static [bool] = &[ false, false, false, false, false, false, false, false ];
pub(crate) static edge_orientations_array_34: &'static [bool] = &[ false, false, false, false, true, true, true, true ];
pub(crate) static edge_orientations_array_35: &'static [bool] = &[ false, false, true, true, false, false, false, false ];
pub(crate) static edge_orientations_array_36: &'static [bool] = &[ false, false, false, true, false, false, false, false ];
pub(crate) static edge_orientations_array_37: &'static [bool] = &[ false, false, false, false, false, true, true, false ];
pub(crate) static edge_orientations_array_38: &'static [bool] = &[ false, false, false, false, true, false, false, false ];
pub(crate) static edge_orientations_array_39: &'static [bool] = &[ false, false, true, true, false, true, true, false ];
pub(crate) static edge_orientations_array_40: &'static [bool] = &[ false, false, false, true, true, true, true, false ];
pub(crate) static edge_orientations_array_41: &'static [bool] = &[ false, false, false, false, false, false ];
pub(crate) static edge_orientations_array_42: &'static [bool] = &[ false, false, true, true, false, false ];
pub(crate) static edge_orientations_array_43: &'static [bool] = &[ false, false, false, true, false, false ];
pub(crate) static edge_orientations_array_44: &'static [bool] = &[ false, false, true, false, false, false ];

pub(crate) static default_params_array_00: &'static[f64] = &[ 0.12239750492, 0.5, 0.143395479017, 0.625 ];
pub(crate) static default_params_array_01: &'static[f64] = &[ 0.12239750492, 0.5, 0.225335752741, 0.225335752741 ];
pub(crate) static default_params_array_02: &'static[f64] = &[ 0.12239750492, 0.5, 0.225335752741, 0.625 ];
pub(crate) static default_params_array_03: &'static[f64] = &[ 0.12239750492, 0.5, 0.315470053838, 0.5, 0.315470053838, 0.5 ];
pub(crate) static default_params_array_04: &'static[f64] = &[ 0.12239750492, 0.5, 0.225335752741, 0.225335752741, 0.5 ];
pub(crate) static default_params_array_05: &'static[f64] = &[ 0.12239750492, 0.5, 0.225335752741, 0.625, 0.5 ];
pub(crate) static default_params_array_06: &'static[f64] = &[ 0.6, 0.196416770201 ];
pub(crate) static default_params_array_07: &'static[f64] = &[ 0.12239750492, 0.5, 0.225335752741 ];
pub(crate) static default_params_array_08: &'static[f64] = &[ ];
pub(crate) static default_params_array_09: &'static[f64] = &[ 0.12239750492, 0.225335752741 ];
pub(crate) static default_params_array_10: &'static[f64] = &[ 0.12239750492, 0.225335752741, 0.5 ];
pub(crate) static default_params_array_11: &'static[f64] = &[ 0.12239750492, 0.225335752741, 0.225335752741 ];
pub(crate) static default_params_array_12: &'static[f64] = &[ 0.216506350946 ];
pub(crate) static default_params_array_13: &'static[f64] = &[ 0.104512294489, 0.65 ];
pub(crate) static default_params_array_14: &'static[f64] = &[ 0.230769230769, 0.5, 0.225335752741 ];
pub(crate) static default_params_array_15: &'static[f64] = &[ 0.230769230769, 0.5, 0.225335752741, 0.5 ];
pub(crate) static default_params_array_16: &'static[f64] = &[ 0.230769230769, 0.225335752741 ];
pub(crate) static default_params_array_17: &'static[f64] = &[ 0.141304, 0.465108, 0.534891 ];
pub(crate) static default_params_array_18: &'static[f64] = &[ 0.452827026611, 0.5 ];
pub(crate) static default_params_array_19: &'static[f64] = &[ 0.366873818946 ];
pub(crate) static default_params_array_20: &'static[f64] = &[ 0.230769230769 ];
pub(crate) static default_params_array_21: &'static[f64] = &[ 0.230769230769, 0.5 ];
pub(crate) static default_params_array_22: &'static[f64] = &[ 0.5, 0.102564102564 ];
pub(crate) static default_params_array_23: &'static[f64] = &[ 0.230769230769, 0.869565217391 ];
pub(crate) static default_params_array_24: &'static[f64] = &[ 0.5, 0.230769230769, 0.5, 0.5 ];
pub(crate) static default_params_array_25: &'static[f64] = &[ 0.230769230769, 0.5, 0.230769230769 ];
pub(crate) static default_params_array_26: &'static[f64] = &[ 0.5, 0.5, 0.6 ];
pub(crate) static default_params_array_27: &'static[f64] = &[ 0.5, 0.102564102564, 0.102564102564 ];
pub(crate) static default_params_array_28: &'static[f64] = &[ 0.230769230769, 0.230769230769 ];
pub(crate) static default_params_array_29: &'static[f64] = &[ 0.5 ];
pub(crate) static default_params_array_30: &'static[f64] = &[ 0.105263157895 ];
pub(crate) static default_params_array_31: &'static[f64] = &[ 0.196416770201 ];
pub(crate) static default_params_array_32: &'static[f64] = &[ 0.5, 0.196416770201 ];

pub(crate) static tiling_vertex_coefficients_array_00: &'static [f64] = &[ 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 3.9, 0.0, 0.0, 0.0, 0.1, 0.0, 5.0, 0.0, 0.0, -2.5, 3.9, 0.0, 5.5, 0.0, -0.4, 0.0, 5.0, 0.0, -4.0, 0.5, 3.9, 0.0, 0.0, 0.0, 0.1, 0.0, 5.0, 0.0, 0.0, -1.5, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, -5.5, 0.0, 0.5, 0.0, 0.0, 0.0, 4.0, -2.0 ];
pub(crate) static tiling_vertex_coefficients_array_01: &'static [f64] = &[ 3.9, 0.0, 0.0, 0.0, 0.1, 0.0, 5.0, 0.0, 0.0, -2.5, 3.9, 0.0, 0.0, 3.5, -0.4, 0.0, 5.0, 0.0, 0.0, -2.0, 3.9, 0.0, 0.0, 0.0, 0.1, 0.0, 5.0, 0.0, 0.0, -1.5, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, -3.5, 0.0, 0.5, 0.0, 0.0, 0.0, 0.0, 0.5, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0 ];
pub(crate) static tiling_vertex_coefficients_array_02: &'static [f64] = &[ 0.0, 0.0, -3.5, 0.0, 0.5, 0.0, 0.0, 0.0, 4.0, -2.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 3.9, 0.0, 0.0, 0.0, 0.1, 0.0, 5.0, 0.0, 0.0, -2.5, 3.9, 0.0, 3.5, 0.0, -0.4, 0.0, 5.0, 0.0, 4.0, -4.5, 3.9, 0.0, 0.0, 0.0, 0.1, 0.0, 5.0, 0.0, 0.0, -1.5, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0 ];
pub(crate) static tiling_vertex_coefficients_array_03: &'static [f64] = &[ 0.0, 0.0, -2.5, 0.0, 0.0, 0.0, 0.5, 0.0, 0.0, 0.0, 3.0, 0.0, 0.0, -1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 3.9, 0.0, 0.0, 0.0, 0.0, 0.0, 0.1, 0.0, 5.0, 0.0, 0.0, 0.0, 0.0, -2.5, 3.9, 0.0, 0.0, 0.0, 2.5, 0.0, -0.4, 0.0, 5.0, 0.0, 0.0, 0.0, 3.0, -3.5, 3.9, 0.0, 0.0, 0.0, 0.0, 0.0, 0.1, 0.0, 5.0, 0.0, 0.0, 0.0, 0.0, -1.5, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0 ];
pub(crate) static tiling_vertex_coefficients_array_04: &'static [f64] = &[ 3.9, 0.0, 0.0, 3.5, 0.0, -0.4, 0.0, 5.0, 0.0, 0.0, 5.0, -4.5, 3.9, 0.0, 0.0, 0.0, 0.0, 0.1, 0.0, 5.0, 0.0, 0.0, 0.0, -1.5, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, -3.5, 0.0, 0.0, 0.5, 0.0, 0.0, 0.0, 0.0, 0.0, 0.5, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 3.9, 0.0, 0.0, 0.0, 0.0, 0.1, 0.0, 5.0, 0.0, 0.0, 0.0, -2.5 ];
pub(crate) static tiling_vertex_coefficients_array_05: &'static [f64] = &[ 3.9, 0.0, 3.5, 0.0, 0.0, -0.4, 0.0, -5.0, 0.0, 4.0, 0.0, 0.5, 3.9, 0.0, 0.0, 0.0, 5.0, -2.4, 0.0, 5.0, 0.0, 0.0, 0.0, -1.5, 0.0, 0.0, 0.0, 0.0, 5.0, -2.5, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, -3.5, 0.0, 0.0, 0.5, 0.0, 0.0, 0.0, 4.0, 0.0, -2.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 3.9, 0.0, 0.0, 0.0, 0.0, 0.1, 0.0, -5.0, 0.0, 0.0, 0.0, 2.5 ];
pub(crate) static tiling_vertex_coefficients_array_06: &'static [f64] = &[ 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.5, 0.0, 0.0, -0.288675134595, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 2.5, 1.12583302492, -0.721132486541, -1.44337567297, 1.95, 1.06036297108, 5.0, 0.0, -2.5, 0.0, 3.9, 0.1, 2.5, -1.12583302492, -1.27886751346, 1.44337567297, 1.95, -0.671687836487 ];
pub(crate) static tiling_vertex_coefficients_array_07: &'static [f64] = &[ 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 3.9, 0.0, 0.0, 0.1, 0.0, 5.0, 0.0, -2.5, 3.9, 0.0, 3.5, -0.4, 0.0, 5.0, 0.0, -2.0, 3.9, 0.0, 0.0, 0.1, 0.0, 5.0, 0.0, -1.5, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, -3.5, 0.5, 0.0, 0.0, 0.0, 0.5 ];
pub(crate) static tiling_vertex_coefficients_array_08: &'static [f64] = &[ 1.0, 0.0, 0.5, 0.866025403784, -0.5, 0.866025403784, -1.0, 0.0, -0.5, -0.866025403784, 0.5, -0.866025403784 ];
pub(crate) static tiling_vertex_coefficients_array_09: &'static [f64] = &[ 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 3.9, 0.0, 0.1, 0.0, 0.0, 0.0, 3.9, 3.5, -0.4, 0.0, 0.0, 0.5, 3.9, 0.0, 0.1, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, -3.5, 0.5, 0.0, 0.0, 0.5 ];
pub(crate) static tiling_vertex_coefficients_array_10: &'static [f64] = &[ 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 3.9, 0.0, 0.0, 0.1, 0.0, 0.0, 0.0, 0.0, 3.9, 3.5, 0.0, -0.4, 0.0, 0.0, 5.0, -2.0, 3.9, 0.0, 0.0, 0.1, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, -3.5, 0.0, 0.5, 0.0, 0.0, 5.0, -2.0 ];
pub(crate) static tiling_vertex_coefficients_array_11: &'static [f64] = &[ 3.9, 3.5, -0.4, 0.0, 0.0, 0.5, 3.9, 0.0, 0.1, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, -3.5, 0.5, 0.0, 0.0, 0.5, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 3.9, 0.0, 0.1, 0.0, 0.0, 0.0 ];
pub(crate) static tiling_vertex_coefficients_array_12: &'static [f64] = &[ 0.0, -3.5, 0.0, 0.5, 0.0, 0.0, 0.0, 0.5, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 3.9, 0.0, 0.0, 0.1, 0.0, 0.0, 0.0, 0.0, 3.9, 0.0, 3.5, -0.4, 0.0, 0.0, 0.0, 0.5, 3.9, 0.0, 0.0, 0.1, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0 ];
pub(crate) static tiling_vertex_coefficients_array_13: &'static [f64] = &[ 0.0, 0.5, 0.0, -0.288675134595, 0.0, 1.0, 0.0, 0.0, 1.15470053838, 0.75, 2.0, 0.144337567297, 0.0, 0.5, 4.0, 0.0, -1.15470053838, 0.25, 2.0, 0.144337567297, 0.0, 0.0, 0.0, 0.0 ];
pub(crate) static tiling_vertex_coefficients_array_14: &'static [f64] = &[ 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 5.0, -2.5, 5.1, 0.0, -0.1, -1.47224318643, 2.5, -1.22113248654, 2.55, 1.44337567297, -0.771687836487, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.5, 0.0, 0.0, -0.866025403784 ];
pub(crate) static tiling_vertex_coefficients_array_15: &'static [f64] = &[ 3.9, 0.0, 0.0, 0.1, 0.0, 5.0, 0.0, -2.5, 3.9, 0.0, 3.5, -0.4, 0.0, 5.0, 0.0, -2.0, 3.9, 0.0, 0.0, 0.1, 0.0, 5.0, 0.0, -1.5, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0 ];
pub(crate) static tiling_vertex_coefficients_array_16: &'static [f64] = &[ 3.9, 0.0, 0.0, 0.0, 0.1, 0.0, 5.0, 0.0, 0.0, -2.5, 3.9, 0.0, 3.5, 0.0, -0.4, 0.0, 5.0, 0.0, 4.0, -4.0, 3.9, 0.0, 0.0, 0.0, 0.1, 0.0, 5.0, 0.0, 0.0, -1.5, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0 ];
pub(crate) static tiling_vertex_coefficients_array_17: &'static [f64] = &[ 3.9, 0.0, 0.1, 0.0, 0.0, 0.0, 3.9, 3.5, -0.4, 0.0, 0.0, 0.5, 3.9, 0.0, 0.1, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0 ];
pub(crate) static tiling_vertex_coefficients_array_18: &'static [f64] = &[ 0.0, 0.0, 5.0, -2.5, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, -5.0, 2.5, 0.0, 10.0, 0.0, -4.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 3.9, 0.0, 0.0, 0.1, 0.0, -5.0, 0.0, 2.5, 3.9, 0.0, 5.0, -2.4, 0.0, 5.0, 0.0, -1.5 ];
pub(crate) static tiling_vertex_coefficients_array_19: &'static [f64] = &[ 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.95, 2.5, -0.95, -1.95, 2.5, -1.05, 3.9, 0.0, 0.1, 0.0, 5.0, -2.0, 1.95, -2.5, 1.55, 1.95, 2.5, -0.45 ];
pub(crate) static tiling_vertex_coefficients_array_20: &'static [f64] = &[ 0.0, -1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 4.95, 0.55, 4.95, 0.55, 0.0, 0.0, 9.9, 0.1, -4.95, -0.55, 4.95, 0.55 ];
pub(crate) static tiling_vertex_coefficients_array_21: &'static [f64] = &[ 0.0, 1.0, 0.0, 0.0, 2.925, 0.075, 1.68874953738, 0.0433012701892, 0.0, 0.0, 0.0, 0.0, -2.925, 1.425, 1.68874953738, -0.822724133595 ];
pub(crate) static tiling_vertex_coefficients_array_22: &'static [f64] = &[ 1.0, 0.0, 0.75, 0.433012701892, 0.0, 0.0, 0.75, -0.433012701892 ];
pub(crate) static tiling_vertex_coefficients_array_23: &'static [f64] = &[ 0.5, 0.0, 0.0, 0.866025403784, -0.5, 0.0, 0.0, -0.866025403784 ];
pub(crate) static tiling_vertex_coefficients_array_24: &'static [f64] = &[ 0.0, 0.57735026919, -1.0, 0.0, 1.0, 0.0 ];
pub(crate) static tiling_vertex_coefficients_array_25: &'static [f64] = &[ 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 3.9, 0.0, 0.1, 0.0, 5.0, -2.5, 3.9, 0.0, 0.1, 0.0, 5.0, -1.5, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0 ];
pub(crate) static tiling_vertex_coefficients_array_26: &'static [f64] = &[ 5.0, 0.0, -2.0, 0.0, -3.9, -0.1, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 5.0, 0.0, -2.0, 0.0, 3.9, 0.1, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0 ];
pub(crate) static tiling_vertex_coefficients_array_27: &'static [f64] = &[ 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, -3.45, 4.0, 3.9, 0.0, 0.1, 0.0, 3.45, -3.0, 3.9, 0.0, 0.1, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0 ];
pub(crate) static tiling_vertex_coefficients_array_28: &'static [f64] = &[ 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 5.0, 0.0, 0.0, 0.0, -1.5, 0.0, 3.9, 0.0, 0.0, 0.1, 0.0, 0.0, 5.0, 0.0, -2.5, 0.0, 0.0, 0.0, 5.0, -1.5 ];
pub(crate) static tiling_vertex_coefficients_array_29: &'static [f64] = &[ 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, -5.0, 3.9, 2.6, 3.9, 0.0, 0.0, 0.1, 0.0, -5.0, 0.0, 2.5, 3.9, 0.0, 0.0, 0.1 ];
pub(crate) static tiling_vertex_coefficients_array_30: &'static [f64] = &[ 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, -10.0, 0.0, 0.0, 5.0, 0.0, 10.0, 0.0, -4.0, 10.0, 0.0, 10.0, -10.0, 0.0, 10.0, 0.0, -5.0, 0.0, 0.0, 10.0, -5.0 ];
pub(crate) static tiling_vertex_coefficients_array_31: &'static [f64] = &[ 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0, 3.9, 0.1, 0.0, 0.0, 3.9, 0.1 ];
pub(crate) static tiling_vertex_coefficients_array_32: &'static [f64] = &[ 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.5, 0.0, 0.0, 3.9, 0.1, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 5.0, 0.0, 0.0, -2.0, 0.0, -3.9, 0.0, -0.1 ];
pub(crate) static tiling_vertex_coefficients_array_33: &'static [f64] = &[ 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 3.9, 0.0, 0.1, 0.0, 0.0, 0.0, 3.9, 0.0, 0.1, 0.0, 3.9, 0.1 ];
pub(crate) static tiling_vertex_coefficients_array_34: &'static [f64] = &[ 1.0, 0.0, 1.0, 1.0, 0.0, 1.0, 0.0, 0.0 ];
pub(crate) static tiling_vertex_coefficients_array_35: &'static [f64] = &[ 1.8, 0.1, 0.0, 0.0, 0.0, 1.0, 0.0, 1.0, 0.0, 0.0, -1.8, 1.9, 0.0, 0.0, 0.0, 0.0 ];
pub(crate) static tiling_vertex_coefficients_array_36: &'static [f64] = &[ 3.8, 0.1, 0.0, 0.0, 0.0, 0.0, -3.8, 0.9, -3.8, -0.1, 0.0, 0.0, 0.0, 0.0, 3.8, -0.9 ];
pub(crate) static tiling_vertex_coefficients_array_37: &'static [f64] = &[ 0.0, 0.0, 0.57735026919, 0.0, 0.0, 1.0 ];
pub(crate) static tiling_vertex_coefficients_array_38: &'static [f64] = &[ 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 3.9, 0.1 ];
pub(crate) static tiling_vertex_coefficients_array_39: &'static [f64] = &[ 0.5, 0.5, 0.0, 0.0, 1.0, 0.0 ];
pub(crate) static tiling_vertex_coefficients_array_40: &'static [f64] = &[ 0.0, 1.0, 0.0, 0.0, 0.0, 0.5, 3.9, 0.1, 0.0, 0.0, 0.0, 0.0 ];
pub(crate) static tiling_vertex_coefficients_array_41: &'static [f64] = &[ 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 5.0, 0.0, -2.0, 0.0, 3.9, 0.1 ];
pub(crate) static tiling_vertex_coefficients_array_42: &'static [f64] = &[ 1.0, 0.0, -0.5, 0.866025403784, -0.5, -0.866025403784 ];

pub(crate) static translation_coefficients_array_00: &'static [f64] = &[ 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, -1.0, 3.9, 0.0, 5.5, 0.0, -0.4, 0.0, 5.0, 0.0, -4.0, -0.5 ];
pub(crate) static translation_coefficients_array_01: &'static [f64] = &[ 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 7.8, 0.0, 3.5, 3.5, -0.8, 0.0, 0.0, -1.66533453694e-16, 7.95659834315e-16, 0.0 ];
pub(crate) static translation_coefficients_array_02: &'static [f64] = &[ 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, -1.0, -7.8, -3.33066907388e-16, -7.0, 0.0, 0.8, 0.0, -2.22044604925e-16, -1.11022302463e-15, -2.22044604925e-16, -1.0 ];
pub(crate) static translation_coefficients_array_03: &'static [f64] = &[ 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, -1.0, -7.8, 1.11022302463e-16, -2.5, 0.0, -2.5, 0.0, 0.8, 0.0, -10.0, 0.0, 3.0, 0.0, -3.0, 4.0 ];
pub(crate) static translation_coefficients_array_04: &'static [f64] = &[ 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, -15.6, 0.0, -7.0, -7.0, 0.0, 1.6, 0.0, 0.0, 4.4408920985e-16, 0.0, 0.0, -2.0 ];
pub(crate) static translation_coefficients_array_05: &'static [f64] = &[ -2.65990932983e-16, 0.0, -3.70074341542e-17, 0.0, -2.22044604925e-16, 1.91976064675e-16, 8.881784197e-16, 10.0, 0.0, 4.4408920985e-16, 0.0, -3.0, 7.8, 3.33066907388e-16, 7.0, 3.33066907388e-16, 7.77156117238e-16, -0.8, -7.40148683083e-17, -1.48029736617e-16, 2.22044604925e-16, -1.48029736617e-16, 0.0, 7.40148683083e-17 ];
pub(crate) static translation_coefficients_array_06: &'static [f64] = &[ -2.5, -3.37749907476, 0.663397459622, 4.33012701892, -1.95, -3.08108891325, -2.5, 3.37749907476, 2.33660254038, -4.33012701892, -1.95, 2.11506350946 ];
pub(crate) static translation_coefficients_array_07: &'static [f64] = &[ 0.0, 0.0, 7.40148683083e-17, 0.0, 0.0, 0.0, 0.0, -1.0, 7.8, 0.0, 7.0, -0.8, 0.0, 0.0, 5.92118946467e-16, 0.0 ];
pub(crate) static translation_coefficients_array_08: &'static [f64] = &[ 1.5, 0.866025403784, 1.5, -0.866025403784 ];
pub(crate) static translation_coefficients_array_09: &'static [f64] = &[ 1.5, 0.866025403784, 7.40148683083e-17, 1.73205080757 ];
pub(crate) static translation_coefficients_array_10: &'static [f64] = &[ 0.0, 0.0, 0.0, 0.0, 0.0, -1.0, 3.9, 3.5, -0.4, 0.0, 1.11022302463e-16, -0.5 ];
pub(crate) static translation_coefficients_array_11: &'static [f64] = &[ 0.0, 0.0, 0.0, 0.0, -3.33066907388e-16, 0.0, -3.33066907388e-16, -1.0, 7.8, 7.0, 0.0, -0.8, 0.0, -4.4408920985e-16, -7.40148683083e-17, 0.0 ];
pub(crate) static translation_coefficients_array_12: &'static [f64] = &[ 3.9, 3.5, -0.4, 0.0, -1.66533453694e-16, 0.5, 3.9, 3.5, -0.4, 0.0, 0.0, -0.5 ];
pub(crate) static translation_coefficients_array_13: &'static [f64] = &[ 4.62592926927e-18, 1.66533453694e-16, -1.48029736617e-16, -4.62592926927e-18, -2.22044604925e-16, 0.0, 0.0, -1.0, -7.8, -3.5, -3.5, 0.8, 0.0, 0.0, 0.0, 0.0 ];
pub(crate) static translation_coefficients_array_14: &'static [f64] = &[ 6.38378239159e-16, 1.01770443924e-16, -4.0, -0.866025403784, 3.46410161514, 0.75, -2.0, -0.433012701892 ];
pub(crate) static translation_coefficients_array_15: &'static [f64] = &[ 4.4167295593, -2.5, 2.66339745962, -2.55, -4.33012701892, 1.34903810568, -1.7763568394e-15, -5.0, 2.5, -5.1, 1.99840144433e-15, -1.63205080757 ];
pub(crate) static translation_coefficients_array_16: &'static [f64] = &[ -7.8, 0.0, -3.5, 0.3, 0.0, 0.0, 6.66133814775e-16, -0.5, -7.8, 0.0, -3.5, 0.3, 0.0, 0.0, -1.66533453694e-16, 0.5 ];
pub(crate) static translation_coefficients_array_17: &'static [f64] = &[ 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 7.8, -1.66533453694e-16, 3.5, 0.0, -0.3, 0.0, 10.0, 0.0, 4.0, -7.5 ];
pub(crate) static translation_coefficients_array_18: &'static [f64] = &[ 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, -1.0, 15.6, 5.55111512313e-16, 7.0, 5.55111512313e-16, -0.6, 0.0, 6.2172489379e-16, 4.88498130835e-16, 4.88498130835e-16, -4.88498130835e-16 ];
pub(crate) static translation_coefficients_array_19: &'static [f64] = &[ 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, -15.6, 0.0, -7.0, 0.6, 0.0, 0.0, -4.66293670343e-16, 0.0 ];
pub(crate) static translation_coefficients_array_20: &'static [f64] = &[ -1.66533453694e-17, -1.99840144433e-16, 1.66533453694e-17, 0.0, 0.0, 1.0, -7.8, -3.5, 0.3, 0.0, 0.0, 0.5 ];
pub(crate) static translation_coefficients_array_21: &'static [f64] = &[ 3.99680288865e-16, 2.6645352591e-16, 1.33226762955e-15, -4.4408920985e-16, 0.0, 10.0, 0.0, -3.0, -7.8, 1.7763568394e-15, -10.0, 4.8, 0.0, 1.33226762955e-16, 0.0, -1.33226762955e-16 ];
pub(crate) static translation_coefficients_array_22: &'static [f64] = &[ -3.9, 5.0, -3.1, -3.9, -5.0, 1.9, -3.9, -5.0, 1.9, 3.9, -5.0, 3.1 ];
pub(crate) static translation_coefficients_array_23: &'static [f64] = &[ 9.9, 1.1, -9.9, -1.1, -9.9, -1.1, -9.9, -1.1 ];
pub(crate) static translation_coefficients_array_24: &'static [f64] = &[ -3.88578058619e-16, 2.77555756156e-16, 0.0, 1.73205080757, -2.22044604925e-16, 1.5, -2.22044604925e-16, -0.866025403784 ];
pub(crate) static translation_coefficients_array_25: &'static [f64] = &[ -1.5, 0.866025403784, -1.5, -0.866025403784 ];
pub(crate) static translation_coefficients_array_26: &'static [f64] = &[ 0.0, 1.73205080757, 1.5, -0.866025403784 ];
pub(crate) static translation_coefficients_array_27: &'static [f64] = &[ -1.0, 1.73205080757, 1.0, 1.73205080757 ];
pub(crate) static translation_coefficients_array_28: &'static [f64] = &[ 1.0, 1.73205080757, -1.0, 1.73205080757 ];
pub(crate) static translation_coefficients_array_29: &'static [f64] = &[ 1.0, 1.73205080757, 2.0, 7.40148683083e-17 ];
pub(crate) static translation_coefficients_array_30: &'static [f64] = &[ 0.0, 0.0, 0.0, 0.0, 0.0, -1.0, 3.9, 0.0, 0.1, 0.0, 5.0, -2.5 ];
pub(crate) static translation_coefficients_array_31: &'static [f64] = &[ 0.0, 0.0, 0.0, 0.0, 0.0, -1.0, 7.8, 0.0, 0.2, 0.0, 0.0, 0.0 ];
pub(crate) static translation_coefficients_array_32: &'static [f64] = &[ -1.94289029309e-16, -8.32667268469e-17, 8.32667268469e-17, 2.77555756156e-17, -7.8, -0.2, -6.66133814775e-16, 0.0, 1.0, 1.2490009027e-16, 4.16333634234e-17, -4.16333634234e-17 ];
pub(crate) static translation_coefficients_array_33: &'static [f64] = &[ 0.0, -6.9, 8.0, 0.0, 0.0, 0.0, 0.0, -3.45, 4.0, -3.9, -2.77555756156e-17, -0.1 ];
pub(crate) static translation_coefficients_array_34: &'static [f64] = &[ -5.0, 0.0, -5.0, 0.0, 5.0, 0.0, -3.9, 0.0, -5.0, 1.4, -5.0, 0.0, 0.0, 0.0, 1.5, 5.55111512313e-17, -3.9, 0.0, -5.55111512313e-17, -0.1 ];
pub(crate) static translation_coefficients_array_35: &'static [f64] = &[ 0.0, 0.0, 0.0, 0.0, 0.0, -1.0, 7.8, 0.0, 0.2, 0.0, 10.0, -5.0 ];
pub(crate) static translation_coefficients_array_36: &'static [f64] = &[ 0.0, 0.0, 0.0, 0.0, -7.8, 0.0, 0.0, -0.2, 0.0, 4.4408920985e-16, 3.9, 1.1, -8.881784197e-16, 2.42861286637e-17, -1.38777878078e-17, 0.0 ];
pub(crate) static translation_coefficients_array_37: &'static [f64] = &[ -15.6, 0.0, -0.4, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, -1.0 ];
pub(crate) static translation_coefficients_array_38: &'static [f64] = &[ 0.0, -4.4408920985e-16, 0.0, 0.0, -20.0, 0.0, -20.0, 20.0, 0.0, -4.4408920985e-16, 0.0, -2.0, -2.44249065418e-15, -3.5527136788e-15, -2.44249065418e-15, 2.44249065418e-15 ];
pub(crate) static translation_coefficients_array_39: &'static [f64] = &[ 4.4408920985e-16, 2.0, 2.08166817117e-17, -2.08166817117e-17, 0.0, 0.0, -7.8, -0.2 ];
pub(crate) static translation_coefficients_array_40: &'static [f64] = &[ 0.0, 0.0, 0.0, -1.0, 0.0, 0.0, 0.0, 0.0, 2.22044604925e-16, 0.0, 0.0, 0.0, 0.0, -7.8, -7.8, -0.4 ];
pub(crate) static translation_coefficients_array_41: &'static [f64] = &[ -7.8, 0.0, -0.2, 0.0, 6.24500451352e-17, -6.24500451352e-17, -3.9, -2.77555756156e-17, -0.1, 2.22044604925e-16, 3.9, 1.1 ];
pub(crate) static translation_coefficients_array_42: &'static [f64] = &[ 0.0, 2.0, 2.0, 0.0 ];
pub(crate) static translation_coefficients_array_43: &'static [f64] = &[ 1.80411241502e-17, -2.63677968348e-17, 8.881784197e-16, 4.0, 4.4408920985e-16, -2.0, 0.0, 2.0 ];
pub(crate) static translation_coefficients_array_44: &'static [f64] = &[ -3.12250225676e-17, 3.12250225676e-17, -7.6, 1.8, 7.6, 0.2, -7.6, 1.8 ];
pub(crate) static translation_coefficients_array_45: &'static [f64] = &[ 1.0, 1.0, 1.0, -1.0 ];
pub(crate) static translation_coefficients_array_46: &'static [f64] = &[ 1.0, 0.0, 0.0, 1.0 ];
pub(crate) static translation_coefficients_array_47: &'static [f64] = &[ 0.0, 0.0, -3.9, -0.1, 0.0, 1.0, 1.38777878078e-17, -1.38777878078e-17 ];
pub(crate) static translation_coefficients_array_48: &'static [f64] = &[ 0.0, 0.0, -3.9, -0.1, 0.0, 2.0, -1.38777878078e-17, 1.38777878078e-17 ];
pub(crate) static translation_coefficients_array_49: &'static [f64] = &[ 0.0, -3.45, 4.0, -3.9, 2.77555756156e-17, -0.1, 0.0, -3.45, 4.0, 3.9, 0.0, 0.1 ];
pub(crate) static translation_coefficients_array_50: &'static [f64] = &[ 3.8, 0.1, -3.8, 0.9, -3.8, -0.1, -3.8, 0.9 ];
pub(crate) static translation_coefficients_array_51: &'static [f64] = &[ -2.22044604925e-16, 2.0, -1.73205080757, 1.0 ];
pub(crate) static translation_coefficients_array_52: &'static [f64] = &[ 4.4408920985e-16, 2.0, -1.38777878078e-17, 1.38777878078e-17, 0.0, 1.0, 3.9, 0.1 ];
pub(crate) static translation_coefficients_array_53: &'static [f64] = &[ 0.0, 1.0, -1.0, 0.0 ];
pub(crate) static translation_coefficients_array_54: &'static [f64] = &[ -1.0, 1.0, -2.0, 0.0 ];
pub(crate) static translation_coefficients_array_55: &'static [f64] = &[ 0.0, 1.0, 1.0, 0.0 ];
pub(crate) static translation_coefficients_array_56: &'static [f64] = &[ 0.0, 0.5, -3.9, -0.1, 0.0, -0.5, -3.9, -0.1 ];
pub(crate) static translation_coefficients_array_57: &'static [f64] = &[ -5.0, 0.0, 2.0, 2.77555756156e-17, -3.9, -0.1, -5.0, 0.0, 3.0, 0.0, -3.9, -0.1 ];
pub(crate) static translation_coefficients_array_58: &'static [f64] = &[ 0.0, 0.0, 1.0, -2.77555756156e-17, -1.38777878078e-17, 1.38777878078e-17, 0.0, 0.0, 0.0, -8.32667268469e-17, 7.8, 0.2 ];
pub(crate) static translation_coefficients_array_59: &'static [f64] = &[ 2.22044604925e-16, 1.0, -3.00685402503e-16, 4.62592926927e-18, -3.33066907388e-16, 1.11022302463e-16, 7.8, 0.2 ];
pub(crate) static translation_coefficients_array_60: &'static [f64] = &[ -1.5, 2.59807621135, -3.0, -1.33226762955e-15 ];
pub(crate) static translation_coefficients_array_61: &'static [f64] = &[ 0.0, -0.5, -3.9, -0.1, 0.0, 0.5, -3.9, -0.1 ];

pub(crate) static aspect_coefficients_array_00: &'static [f64] = &[ 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0 ];
pub(crate) static aspect_coefficients_array_01: &'static [f64] = &[ 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, -2.22044604925e-16, -1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 7.8, 0.0, 0.0, 3.5, -0.3, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 2.22044604925e-16, 1.0, 0.0, 0.0, 0.0, 8.881784197e-16, -0.5 ];
pub(crate) static aspect_coefficients_array_02: &'static [f64] = &[ 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, -3.9, -2.77555756156e-16, -3.5, -2.77555756156e-16, 0.4, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, -1.0, 0.0, 5.0, -8.881784197e-16, 4.0, -4.5 ];
pub(crate) static aspect_coefficients_array_03: &'static [f64] = &[ 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, -1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, -2.5, 0.0, 0.0, 0.0, 0.5, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, -1.0, 0.0, 0.0, 0.0, 3.0, 0.0, 0.0, -1.0 ];
pub(crate) static aspect_coefficients_array_04: &'static [f64] = &[ 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, -1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 7.8, 5.55111512313e-17, 0.0, 3.5, 0.0, -0.3, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, -1.0, 0.0, 10.0, 0.0, 0.0, 5.0, -6.0, 0.0, 0.0, -2.22044604925e-16, 0.0, 0.0, -1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, -3.5, 0.0, 0.0, 0.5, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 2.22044604925e-16, 0.0, 0.0, 1.0, 0.0, 0.0, -2.22044604925e-16, 0.0, 0.0, -0.5, 0.0, 0.0, 2.22044604925e-16, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, -0.0, -7.8, 1.11022302463e-16, -3.5, -3.5, 1.11022302463e-16, 0.8, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, -2.22044604925e-16, 0.0, 0.0, -1.0, 0.0, 10.0, -1.7763568394e-15, 0.0, 5.0, -7.5 ];
pub(crate) static aspect_coefficients_array_05: &'static [f64] = &[ 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.11022302463e-16, 0.0, -1.0, 0.0, 0.0, 0.0, 5.55111512313e-17, 0.0, 0.0, 7.8, 0.0, 3.5, 0.0, 5.0, -2.8, 0.0, 0.0, 0.0, -5.55111512313e-17, 0.0, 0.0, 0.0, 0.0, 0.0, 1.11022302463e-16, 0.0, -1.0, 0.0, -4.4408920985e-16, 0.0, 4.0, 0.0, -1.0, 3.33066907388e-16, 0.0, 0.0, 0.0, 0.0, -1.0, 2.77555756156e-17, 5.55111512313e-17, 0.0, 0.0, 0.0, -2.77555756156e-17, 3.9, 0.0, 0.0, 0.0, 5.0, -2.4, 2.77555756156e-17, 5.55111512313e-17, 0.0, 0.0, 0.0, -2.77555756156e-17, -3.33066907388e-16, 0.0, 0.0, 0.0, 0.0, 1.0, 2.22044604925e-16, 5.0, 0.0, 0.0, 0.0, -1.5, -3.33066907388e-16, 0.0, 0.0, -2.22044604925e-16, 0.0, 1.0, -2.77555756156e-17, -5.55111512313e-17, 0.0, 5.55111512313e-17, 0.0, 2.77555756156e-17, 3.9, 0.0, 3.5, -4.4408920985e-16, 4.4408920985e-16, -0.4, -2.77555756156e-17, -5.55111512313e-17, 0.0, 5.55111512313e-17, 0.0, 2.77555756156e-17, 3.33066907388e-16, 0.0, 0.0, 2.22044604925e-16, 0.0, -1.0, -2.22044604925e-16, -5.0, 0.0, 4.0, 0.0, 0.5 ];
pub(crate) static aspect_coefficients_array_06: &'static [f64] = &[ 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, -0.5, 0.0, 0.0, -0.866025403784, 0.0, 0.0, 0.5, 0.0, 0.0, 0.866025403784, 0.0, 0.0, -0.5, 0.0, 0.0, -0.866025403784, 0.0, 0.0, -0.5, 0.0, 0.0, 0.866025403784, 0.0, 0.0, 1.0, 0.0, 0.0, -0.866025403784, 0.0, 0.0, -0.5, 0.0, 0.0, 0.0 ];
pub(crate) static aspect_coefficients_array_07: &'static [f64] = &[ 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, -2.22044604925e-16, -1.0, 0.0, 0.0, 0.0, 0.0, 7.8, 0.0, 3.5, -0.3, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 2.22044604925e-16, 1.0, 0.0, 0.0, 8.881784197e-16, -0.5 ];
pub(crate) static aspect_coefficients_array_08: &'static [f64] = &[ 1.0, 0.0, 0.0, 0.0, 1.0, 0.0 ];
pub(crate) static aspect_coefficients_array_09: &'static [f64] = &[ 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0 ];
pub(crate) static aspect_coefficients_array_10: &'static [f64] = &[ 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, -1.0, 0.0, 0.0, 0.0, 0.0, 7.8, 3.5, 0.0, -0.3, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, -1.0, 0.0, -4.4408920985e-16, 5.0, -2.0 ];
pub(crate) static aspect_coefficients_array_11: &'static [f64] = &[ 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, -1.0, 0.0, 0.0, 0.0, 0.0, 0.0, -3.5, 0.0, 0.5, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, -1.0, 0.0, 0.0, 0.0, 0.5 ];
pub(crate) static aspect_coefficients_array_12: &'static [f64] = &[ 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.5, 0.0, 0.866025403784, 0.0, 0.5, 0.0, 0.866025403784, 0.0, -0.5, 0.0, -0.866025403784, 0.0, -0.5, 0.0, -0.866025403784, 0.0, 0.5, 0.0, 0.866025403784, 0.0, -0.5, 0.0, -0.866025403784 ];
pub(crate) static aspect_coefficients_array_13: &'static [f64] = &[ 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.5, 0.0, 0.0, 0.866025403784, 0.0, 0.0, 1.0, 0.0, 0.0, -0.866025403784, 0.0, 0.0, 0.5, 0.0, 0.0, 0.0, 0.0, 0.0, -0.5, 0.0, 0.0, 0.866025403784, 0.0, 0.0, 1.5, 0.0, 0.0, -0.866025403784, 0.0, 0.0, -0.5, 0.0, 0.0, -0.866025403784, 0.0, 0.0, -1.0, 0.0, 0.0, -4.99600361081e-16, 0.0, 0.0, 1.0, 0.0, 0.0, 4.99600361081e-16, 0.0, 0.0, -1.0, 0.0, 0.0, -1.73205080757, 0.0, 0.0, -0.5, 0.0, 0.0, -0.866025403784, 0.0, 0.0, -8.881784197e-16, 0.0, 0.0, 0.866025403784, 0.0, 0.0, -0.5, 0.0, 0.0, -1.73205080757, 0.0, 0.0, 0.5, 0.0, 0.0, -0.866025403784, 0.0, 0.0, -0.5, 0.0, 0.0, 0.866025403784, 0.0, 0.0, 0.5, 0.0, 0.0, -0.866025403784 ];
pub(crate) static aspect_coefficients_array_14: &'static [f64] = &[ 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, -1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0 ];
pub(crate) static aspect_coefficients_array_15: &'static [f64] = &[ 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, -1.11022302463e-16, -1.11022302463e-16, -1.0, 0.0, 0.0, -5.55111512313e-17, -5.55111512313e-17, 5.55111512313e-17, 7.8, 0.0, 3.5, 1.11022302463e-16, -0.3, 0.0, 0.0, 5.55111512313e-17, 5.55111512313e-17, -5.55111512313e-17, 0.0, 0.0, -1.11022302463e-16, -1.11022302463e-16, -1.0, 0.0, 10.0, 0.0, 4.0, -6.5 ];
pub(crate) static aspect_coefficients_array_16: &'static [f64] = &[ 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, -1.11022302463e-16, -1.11022302463e-16, -1.0, 0.0, 0.0, -5.55111512313e-17, -5.55111512313e-17, 5.55111512313e-17, 7.8, 0.0, 3.5, 1.11022302463e-16, -0.3, 0.0, 0.0, 5.55111512313e-17, 5.55111512313e-17, -5.55111512313e-17, 0.0, 0.0, -1.11022302463e-16, -1.11022302463e-16, -1.0, 0.0, 10.0, 0.0, 4.0, -6.5, 0.0, 0.0, -2.22044604925e-16, -2.22044604925e-16, -1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 15.6, 5.55111512313e-16, 7.0, 5.55111512313e-16, -0.6, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 2.22044604925e-16, 2.22044604925e-16, 1.0, 0.0, 1.33226762955e-15, 8.881784197e-16, 8.881784197e-16, -8.881784197e-16, 0.0, 0.0, 1.11022302463e-16, 1.11022302463e-16, 1.0, 0.0, 0.0, -5.55111512313e-17, -5.55111512313e-17, 5.55111512313e-17, 7.8, 0.0, 3.5, 1.11022302463e-16, -0.3, 0.0, 0.0, -5.55111512313e-17, -5.55111512313e-17, 5.55111512313e-17, 0.0, 0.0, -1.11022302463e-16, -1.11022302463e-16, -1.0, 0.0, 10.0, 0.0, 4.0, -6.5 ];
pub(crate) static aspect_coefficients_array_17: &'static [f64] = &[ 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, -1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, -0.0, 0.0, 0.0, 0.0, -1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, -2.22044604925e-16, -1.0, 0.0, 0.0, 0.0, 0.0, -7.8, 0.0, -3.5, 0.3, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 2.22044604925e-16, 1.0, 0.0, 0.0, -2.22044604925e-16, -0.5, 0.0, 0.0, 2.22044604925e-16, 1.0, 0.0, 0.0, 0.0, 0.0, -7.8, 0.0, -3.5, 0.3, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, -2.22044604925e-16, -1.0, 0.0, 0.0, 0.0, 0.5 ];
pub(crate) static aspect_coefficients_array_18: &'static [f64] = &[ 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, -1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, -0.0, 0.0, 0.0, -1.0, 0.0, 0.0, 1.0 ];
pub(crate) static aspect_coefficients_array_19: &'static [f64] = &[ 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, -1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, -1.0, 0.0, 10.0, 0.0, -3.0, 0.0, 3.33066907388e-16, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, -3.9, 4.4408920985e-16, -5.0, 2.4, 0.0, 0.0, 0.0, 0.0, 0.0, -3.33066907388e-16, 0.0, -1.0, 0.0, 5.0, 2.22044604925e-16, -1.5, 0.0, -3.33066907388e-16, 0.0, -1.0, 0.0, 0.0, 0.0, -0.0, 3.9, -4.4408920985e-16, 5.0, -2.4, 0.0, 0.0, 0.0, 0.0, 0.0, 3.33066907388e-16, 0.0, 1.0, 0.0, 5.0, -2.22044604925e-16, -1.5 ];
pub(crate) static aspect_coefficients_array_20: &'static [f64] = &[ 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, -1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, -0.0, 0.0, 0.0, -1.0, 0.0, 0.0, 1.0, 1.66533453694e-16, 1.11022302463e-16, -1.11022302463e-16, 1.11022302463e-16, 2.22044604925e-16, -1.0, -3.9, -1.11022302463e-16, -0.1, -1.11022302463e-16, -2.22044604925e-16, 1.0, 1.66533453694e-16, 1.11022302463e-16, -1.11022302463e-16, 0.0, -5.0, 3.0, 0.0, -5.55111512313e-17, 0.0, 2.22044604925e-16, 4.4408920985e-16, 1.0, -3.9, -4.4408920985e-16, -1.1, -2.22044604925e-16, -4.4408920985e-16, -1.0, 0.0, -5.55111512313e-17, 0.0, 0.0, -5.0, 3.0 ];
pub(crate) static aspect_coefficients_array_21: &'static [f64] = &[ 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, -1.0, 0.0, -0.0, 0.0, 0.0, 0.0, -0.0, 0.0, -1.0, 0.0, 0.0, -1.11022302463e-16, 1.11022302463e-16, -1.11022302463e-16, -1.0, 9.9, 1.1, 1.11022302463e-16, 1.0, -1.11022302463e-16, 1.11022302463e-16, 2.08166817117e-17, -2.08166817117e-17, 8.32667268469e-17, -8.32667268469e-17, 0.0, 1.0, -9.9, -1.1, 0.0, 1.0, -8.32667268469e-17, 8.32667268469e-17, -1.7763568394e-15, 1.11022302463e-16 ];
pub(crate) static aspect_coefficients_array_22: &'static [f64] = &[ 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.11022302463e-16, -0.5, -1.11022302463e-16, 0.866025403784, 0.0, 1.5, 1.11022302463e-16, -0.866025403784, 1.11022302463e-16, -0.5, -1.11022302463e-16, 0.866025403784, 1.11022302463e-16, -0.5, 3.33066907388e-16, -0.866025403784, 0.0, 1.5, -3.33066907388e-16, 0.866025403784, 1.11022302463e-16, -0.5, 1.11022302463e-16, -0.866025403784, -2.22044604925e-16, 0.5, 0.0, 0.866025403784, 0.0, 0.0, 0.0, 0.866025403784, 2.22044604925e-16, -0.5, 0.0, 0.0, 3.33066907388e-16, -1.0, -1.11022302463e-16, 1.11022302463e-16, -4.4408920985e-16, 1.5, -1.11022302463e-16, 1.11022302463e-16, -3.33066907388e-16, 1.0, 2.22044604925e-16, 0.866025403784, -1.66533453694e-16, 0.5, 5.55111512313e-16, -0.866025403784, 2.22044604925e-16, 0.0, 5.55111512313e-16, -0.866025403784, 1.66533453694e-16, -0.5, -4.4408920985e-16, 1.73205080757 ];
pub(crate) static aspect_coefficients_array_23: &'static [f64] = &[ 1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.5, -0.866025403784, 0.0, 0.866025403784, 0.5, 0.0, -0.5, -0.866025403784, 0.0, 0.866025403784, -0.5, 0.0, -1.0, -1.11022302463e-16, 0.0, 1.11022302463e-16, -1.0, 0.0, -0.5, 0.866025403784, 0.0, -0.866025403784, -0.5, 0.0, 0.5, 0.866025403784, 0.0, -0.866025403784, 0.5, 0.0 ];
pub(crate) static aspect_coefficients_array_24: &'static [f64] = &[ 1.0, 0.0, 0.0, 0.0, 1.0, 0.0, -0.5, -0.866025403784, 1.5, 0.866025403784, -0.5, -0.866025403784, -0.5, 0.866025403784, 1.5, -0.866025403784, -0.5, 0.866025403784, 0.5, 0.866025403784, 0.0, 0.866025403784, -0.5, 0.0, 0.5, -0.866025403784, 0.0, -0.866025403784, -0.5, 1.73205080757, -1.0, -2.77555756156e-16, 1.5, -2.77555756156e-16, 1.0, 0.866025403784 ];
pub(crate) static aspect_coefficients_array_25: &'static [f64] = &[ 1.0, 0.0, 0.0, 0.0, 1.0, 0.0, -0.5, 0.866025403784, 0.75, -0.866025403784, -0.5, 0.433012701892, -0.5, -0.866025403784, 0.75, 0.866025403784, -0.5, -0.433012701892 ];
pub(crate) static aspect_coefficients_array_26: &'static [f64] = &[ 1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.5, -0.866025403784, 0.75, 0.866025403784, 0.5, 0.433012701892, -0.5, -0.866025403784, 0.75, 0.866025403784, -0.5, 1.29903810568 ];
pub(crate) static aspect_coefficients_array_27: &'static [f64] = &[ 1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.5, 0.866025403784, 0.75, 0.866025403784, -0.5, 0.433012701892, -0.5, -0.866025403784, 0.75, 0.866025403784, -0.5, -0.433012701892 ];
pub(crate) static aspect_coefficients_array_28: &'static [f64] = &[ 1.0, 0.0, 0.0, 0.0, 1.0, 0.0, -0.5, -0.866025403784, 0.75, -0.866025403784, 0.5, 0.433012701892, -0.5, 0.866025403784, 0.75, 0.866025403784, 0.5, -0.433012701892 ];
pub(crate) static aspect_coefficients_array_29: &'static [f64] = &[ 1.0, 0.0, 0.0, 0.0, 1.0, 0.0, -0.5, 0.866025403784, -0.5, -0.866025403784, -0.5, 0.866025403784, -0.5, -0.866025403784, 0.5, 0.866025403784, -0.5, 0.866025403784, -0.5, 0.866025403784, -1.5, 0.866025403784, 0.5, 0.866025403784, -0.5, -0.866025403784, -0.5, -0.866025403784, 0.5, 0.866025403784, 1.0, 0.0, -1.0, 0.0, -1.0, 1.73205080757 ];
pub(crate) static aspect_coefficients_array_30: &'static [f64] = &[ 1.0, 0.0, 0.0, 0.0, 1.0, 0.0, -0.5, 0.866025403784, -0.5, -0.866025403784, -0.5, 0.866025403784, -0.5, -0.866025403784, 0.5, 0.866025403784, -0.5, 0.866025403784, 0.5, -0.866025403784, -0.5, 0.866025403784, 0.5, 0.866025403784, 0.5, 0.866025403784, -1.5, -0.866025403784, 0.5, 0.866025403784, -1.0, -1.11022302463e-16, -1.0, 1.11022302463e-16, -1.0, 1.73205080757 ];
pub(crate) static aspect_coefficients_array_31: &'static [f64] = &[ 1.0, 0.0, 0.0, 0.0, 1.0, 0.0, -0.5, -0.866025403784, 0.5, 0.866025403784, -0.5, 0.866025403784, -0.5, 0.866025403784, -0.5, -0.866025403784, -0.5, 0.866025403784, 0.5, 0.866025403784, 0.5, -0.866025403784, 0.5, 0.866025403784, 0.5, -0.866025403784, 1.5, 0.866025403784, 0.5, 0.866025403784, -1.0, 1.11022302463e-16, 1.0, -1.11022302463e-16, -1.0, 1.73205080757 ];
pub(crate) static aspect_coefficients_array_32: &'static [f64] = &[ 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, -1.0, 0.0, 0.0, 0.0, 7.8, 0.0, 0.2, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0 ];
pub(crate) static aspect_coefficients_array_33: &'static [f64] = &[ 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 3.9, 0.0, 0.1, 0.0, 0.0, 0.0, 0.0, 0.0, -1.0, 0.0, 5.0, -1.5 ];
pub(crate) static aspect_coefficients_array_34: &'static [f64] = &[ 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, -3.33066907388e-16, -1.11022302463e-16, -1.0, 0.0, 0.0, 0.0, 5.0, 0.0, -1.0, 0.0, 0.0, 0.0, 3.33066907388e-16, 1.11022302463e-16, 1.0, 4.16333634234e-17, -3.9, -0.1 ];
pub(crate) static aspect_coefficients_array_35: &'static [f64] = &[ 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.11022302463e-16, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, -3.45, 4.0, 0.0, 0.0, 0.0, -1.11022302463e-16, 0.0, -1.0, 3.9, -4.16333634234e-17, 0.1 ];
pub(crate) static aspect_coefficients_array_36: &'static [f64] = &[ 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, -1.0, 0.0, 0.0, 0.0, 0.0, -0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, -0.0, 0.0, 0.0, 0.0, 0.0, -1.0, 0.0, 0.0, 0.0, 0.0, 0.0 ];
pub(crate) static aspect_coefficients_array_37: &'static [f64] = &[ 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, -1.0, 0.0, 0.0, -0.0, 7.8, 0.0, 0.2, 0.0, 0.0, 0.0, 0.0, 0.0, -1.0, 0.0, 10.0, -4.0 ];
pub(crate) static aspect_coefficients_array_38: &'static [f64] = &[ 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, -1.0, -5.55111512313e-17, 6.93889390391e-18, 0.0, 0.0, 0.0, -5.0, 3.9, 3.6, 5.55111512313e-17, -6.93889390391e-18, 0.0, 0.0, 0.0, 0.0, 0.0, -1.0, 3.9, 1.38777878078e-17, -1.38777878078e-17, 0.1, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, -1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, -1.0, 5.55111512313e-17, -6.93889390391e-18, 0.0, -0.0, 0.0, -5.0, 3.9, 3.6, 5.55111512313e-17, -6.93889390391e-18, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 3.9, 1.38777878078e-17, -1.38777878078e-17, 0.1 ];
pub(crate) static aspect_coefficients_array_39: &'static [f64] = &[ 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, -1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, -0.0, 0.0, 0.0, -1.0, 0.0, 0.0, 1.0, 0.0, 0.0, -1.0, 0.0, 0.0, 0.0, 7.8, 0.0, 0.2, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, -7.8, 0.0, -0.2, 0.0, 0.0, 0.0, 0.0, 0.0, -1.0, 0.0, 0.0, 1.0 ];
pub(crate) static aspect_coefficients_array_40: &'static [f64] = &[ 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, -1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 10.0, 0.0, -5.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, -1.0, 0.0, 0.0, 10.0, -5.0, 0.0, 0.0, 0.0, -1.0, 0.0, 0.0, 0.0, 0.0, -2.22044604925e-15, 10.0, -2.22044604925e-15, -4.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, -10.0, 0.0, -10.0, 10.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.7763568394e-15, 1.7763568394e-15, 1.7763568394e-15, -1.0, 0.0, 0.0, 0.0, -0.0, 0.0, 0.0, 0.0, -1.0, -10.0, 1.7763568394e-15, 0.0, 5.0 ];
pub(crate) static aspect_coefficients_array_41: &'static [f64] = &[ 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, -2.22044604925e-16, -1.0, 0.0, -0.0, 2.22044604925e-16, 2.0, 0.0, 0.0, -2.22044604925e-16, -1.0, 1.38777878078e-17, -1.38777878078e-17, 0.0, -1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0, -3.9, -0.1, 2.22044604925e-16, 1.0, 0.0, 0.0, 0.0, 1.0, 0.0, -0.0, -2.22044604925e-16, -1.0, 3.9, 0.1 ];
pub(crate) static aspect_coefficients_array_42: &'static [f64] = &[ 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, -1.0, 0.0, 0.0, 0.0, 0.0, 5.0, 0.0, 0.0, -2.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, -1.0, 0.0, -3.9, 0.0, -0.1, 0.0, 0.0, 3.33066907388e-16, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.5, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, -3.33066907388e-16, -1.0, 0.0, 0.0, 3.9, 0.1, 0.0, 0.0, -3.33066907388e-16, -1.0, 0.0, 0.0, 0.0, -0.0, 5.0, 0.0, 0.0, -2.5, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 3.33066907388e-16, 1.0, 0.0, -3.9, -3.9, -0.2 ];
pub(crate) static aspect_coefficients_array_43: &'static [f64] = &[ 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, -1.11022302463e-16, -1.11022302463e-16, -1.0, 4.16333634234e-17, 1.38777878078e-17, -1.38777878078e-17, 3.9, -1.38777878078e-17, 0.1, -4.16333634234e-17, -1.38777878078e-17, 1.38777878078e-17, -1.11022302463e-16, -1.11022302463e-16, -1.0, 2.22044604925e-16, 3.9, 1.1, 0.0, 0.0, -1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.11022302463e-16, 3.33066907388e-16, 1.0, 4.16333634234e-17, 1.38777878078e-17, -1.38777878078e-17, -3.9, -4.16333634234e-17, -0.1, 4.16333634234e-17, 1.38777878078e-17, -1.38777878078e-17, -1.11022302463e-16, -3.33066907388e-16, -1.0, 0.0, 3.9, 1.1 ];
pub(crate) static aspect_coefficients_array_44: &'static [f64] = &[ 1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0, -1.0, 0.0, 2.0, -1.0, 0.0, 2.0, -0.0, -1.0, 2.0, -0.0, -1.0, 2.0, 1.0, 0.0, 0.0 ];
pub(crate) static aspect_coefficients_array_45: &'static [f64] = &[ 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 5.55111512313e-17, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, -1.0, 5.55111512313e-17, 0.0, 0.0, 2.0, 0.0, -1.0, 1.11022302463e-16, 0.0, 0.0, 2.0, -1.11022302463e-16, -0.0, 0.0, -1.0, 0.0, 2.0, -1.66533453694e-16, -0.0, 0.0, -1.0, 0.0, 2.0, 0.0, 1.0, -1.66533453694e-16, -0.0, 0.0, 0.0, 6.66133814775e-16, 1.0, 3.69778549322e-32, 5.55111512313e-17, -1.38777878078e-17, -1.11022302463e-16, 3.69778549322e-32, 5.55111512313e-17, -6.66133814775e-16, -1.0, 1.33226762955e-15, 4.0, -5.55111512313e-17, 0.0, 6.66133814775e-16, 1.0, 0.0, 0.0, 6.66133814775e-16, 1.0, 5.55111512313e-17, 0.0, 0.0, 2.0, -6.66133814775e-16, -1.0, 0.0, 0.0, 1.33226762955e-15, 2.0, 0.0, 0.0, 6.66133814775e-16, 1.0, 0.0, 2.0, -5.55111512313e-17, -0.0, -6.66133814775e-16, -1.0, 1.33226762955e-15, 2.0, -6.66133814775e-16, -1.0, 5.55111512313e-17, 0.0, 1.33226762955e-15, 4.0 ];
pub(crate) static aspect_coefficients_array_46: &'static [f64] = &[ 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 3.33066907388e-16, 1.0, 0.0, 0.0, 3.8, 0.1, 0.0, 0.0, -3.33066907388e-16, -1.0, -3.8, 0.9 ];
pub(crate) static aspect_coefficients_array_47: &'static [f64] = &[ 1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, -1.0, 2.0, 1.0, 0.0, 0.0 ];
pub(crate) static aspect_coefficients_array_48: &'static [f64] = &[ 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0 ];
pub(crate) static aspect_coefficients_array_49: &'static [f64] = &[ 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, -1.11022302463e-16, -1.0, 0.0, -0.0, 0.0, 2.0, 0.0, 0.0, -1.11022302463e-16, -1.0, 3.9, 0.1 ];
pub(crate) static aspect_coefficients_array_50: &'static [f64] = &[ 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.11022302463e-16, -1.0, 0.0, 2.77555756156e-17, 0.0, 0.0, -3.45, 5.0, 0.0, -2.77555756156e-17, 0.0, 0.0, 1.11022302463e-16, -1.0, 3.9, 1.38777878078e-17, 0.1 ];
pub(crate) static aspect_coefficients_array_51: &'static [f64] = &[ 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, -1.0, 0.0, 0.0, 0.0, 5.0, 0.0, -1.0, 0.0, 0.0, 0.0, 0.0, 0.0, -1.0, 0.0, -3.9, -0.1 ];
pub(crate) static aspect_coefficients_array_52: &'static [f64] = &[ 1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, -1.0, 2.0, 1.0, 0.0, 0.0, -1.0, -0.0, 2.0, 0.0, -1.0, 2.0, -0.0, 1.0, 0.0, -1.0, -0.0, 2.0 ];
pub(crate) static aspect_coefficients_array_53: &'static [f64] = &[ 1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.5, 0.866025403784, -0.866025403784, -0.866025403784, 0.5, 0.5, -0.5, 0.866025403784, -0.866025403784, -0.866025403784, -0.5, 1.5, -1.0, 1.11022302463e-16, -1.11022302463e-16, -1.11022302463e-16, -1.0, 2.0, -0.5, -0.866025403784, 0.866025403784, 0.866025403784, -0.5, 1.5, 0.5, -0.866025403784, 0.866025403784, 0.866025403784, 0.5, 0.5, -1.0, 0.0, 0.0, 0.0, 1.0, 0.0, -0.5, 0.866025403784, -0.866025403784, 0.866025403784, 0.5, 0.5, 0.5, 0.866025403784, -0.866025403784, 0.866025403784, -0.5, 1.5, 1.0, 1.11022302463e-16, -1.11022302463e-16, 1.11022302463e-16, -1.0, 2.0, 0.5, -0.866025403784, 0.866025403784, -0.866025403784, -0.5, 1.5, -0.5, -0.866025403784, 0.866025403784, -0.866025403784, 0.5, 0.5 ];
pub(crate) static aspect_coefficients_array_54: &'static [f64] = &[ 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, -1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, -1.0, 3.9, 0.1, -2.22044604925e-16, -1.0, 0.0, 0.0, 2.22044604925e-16, 2.0, 0.0, 0.0, 2.22044604925e-16, 1.0, -2.77555756156e-17, 2.77555756156e-17, 2.22044604925e-16, 1.0, 0.0, 0.0, 0.0, 1.0, 0.0, -0.0, -2.22044604925e-16, -1.0, 3.9, 0.1 ];
pub(crate) static aspect_coefficients_array_55: &'static [f64] = &[ 1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0, -1.0, 0.0, 1.0, -1.0, 0.0, 1.0, -0.0, -1.0, 1.0, -0.0, -1.0, 1.0, 1.0, -0.0, 0.0 ];
pub(crate) static aspect_coefficients_array_56: &'static [f64] = &[ 1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0, -1.0, 0.0, 1.0, -1.0, 0.0, 1.0, -0.0, -1.0, 1.0, -0.0, -1.0, 1.0, 1.0, -0.0, 0.0, -1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, -1.0, 0.0, -1.0, -0.0, 1.0, 1.0, 0.0, -1.0, 0.0, -1.0, 1.0, 0.0, 1.0, -1.0, 1.0, 0.0, 0.0 ];
pub(crate) static aspect_coefficients_array_57: &'static [f64] = &[ 1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, -1.0, 1.0, 1.0, 0.0, 0.0, -1.0, -0.0, 1.0, 0.0, -1.0, 1.0, -0.0, 1.0, 0.0, -1.0, -0.0, 1.0 ];
pub(crate) static aspect_coefficients_array_58: &'static [f64] = &[ 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, -1.0, 0.0, 0.0 ];
pub(crate) static aspect_coefficients_array_59: &'static [f64] = &[ 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, -1.0, 0.0, 0.0, -0.0, 0.0, 0.0, 1.0, 0.0, 0.0, -0.0, 0.0, 0.0, -1.0, 0.0, 0.0, 0.0 ];
pub(crate) static aspect_coefficients_array_60: &'static [f64] = &[ 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, -1.0, 0.0, 0.0, 0.0, 5.0, 0.0, -1.0, 0.0, 0.0, 0.0, 0.0, 0.0, -1.0, -2.77555756156e-17, 3.9, 0.1, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, -1.0, -5.55111512313e-17, 7.8, 0.2, 0.0, 0.0, -1.0, 0.0, 0.0, -0.0, 5.0, 0.0, -1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, -2.77555756156e-17, 3.9, 0.1 ];
pub(crate) static aspect_coefficients_array_61: &'static [f64] = &[ 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, -6.66133814775e-16, -1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, -6.66133814775e-16, -1.0, 7.8, 0.2, 3.33066907388e-16, 1.0, 0.0, 0.0, 0.0, 0.5, 0.0, 0.0, -3.33066907388e-16, -1.0, 3.9, 0.1, -3.33066907388e-16, -1.0, 0.0, -0.0, 2.22044604925e-16, 1.5, 0.0, 0.0, 3.33066907388e-16, 1.0, 3.9, 0.1 ];
pub(crate) static aspect_coefficients_array_62: &'static [f64] = &[ 1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.5, -0.866025403784, 0.5, 0.866025403784, 0.5, 0.866025403784, -0.5, -0.866025403784, -2.22044604925e-16, 0.866025403784, -0.5, 1.73205080757, -1.0, 2.77555756156e-16, -1.0, -2.77555756156e-16, -1.0, 1.73205080757, -0.5, 0.866025403784, -1.5, -0.866025403784, -0.5, 0.866025403784, 0.5, 0.866025403784, -1.0, -0.866025403784, 0.5, -7.77156117238e-16 ];
pub(crate) static aspect_coefficients_array_63: &'static [f64] = &[ 1.0, 0.0, 0.0, 0.0, 1.0, 0.0, -1.0, 0.0, 0.5, 0.0, -1.0, 0.866025403784 ];
pub(crate) static aspect_coefficients_array_64: &'static [f64] = &[ 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, -1.0, 0.0, -0.0, 0.0, 1.0, 0.0, -0.0, 0.0, -1.0, 0.0, 0.0 ];

pub(crate) static colouring_array_00: &'static [u8] = &[ 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 2, 0, 2, 0, 1, 3 ];
pub(crate) static colouring_array_01: &'static [u8] = &[ 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 2, 0, 0, 1, 2, 3 ];
pub(crate) static colouring_array_02: &'static [u8] = &[ 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 1, 2, 0, 1, 3 ];
pub(crate) static colouring_array_03: &'static [u8] = &[ 0, 1, 2, 1, 0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 1, 2, 0, 1, 3 ];
pub(crate) static colouring_array_04: &'static [u8] = &[ 0, 1, 2, 2, 0, 0, 0, 0, 0, 0, 0, 0, 1, 2, 0, 0, 1, 2, 3 ];
pub(crate) static colouring_array_05: &'static [u8] = &[ 0, 1, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 2, 0, 1, 2, 3 ];
pub(crate) static colouring_array_06: &'static [u8] = &[ 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 1, 0, 1, 2, 3 ];
pub(crate) static colouring_array_07: &'static [u8] = &[ 0, 1, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 1, 2, 0, 2, 0, 1, 3 ];
pub(crate) static colouring_array_08: &'static [u8] = &[ 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 2, 0, 2, 0, 1, 3 ];
pub(crate) static colouring_array_09: &'static [u8] = &[ 0, 1, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 1, 1, 2, 0, 3 ];
pub(crate) static colouring_array_10: &'static [u8] = &[ 0, 1, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 1, 0, 1, 2, 3 ];
pub(crate) static colouring_array_11: &'static [u8] = &[ 0, 1, 2, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 2, 1, 2, 0, 3 ];
pub(crate) static colouring_array_12: &'static [u8] = &[ 0, 1, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 2, 0, 0, 1, 2, 3 ];
pub(crate) static colouring_array_13: &'static [u8] = &[ 0, 1, 2, 1, 2, 0, 0, 0, 0, 0, 0, 0, 1, 2, 0, 2, 0, 1, 3 ];
pub(crate) static colouring_array_14: &'static [u8] = &[ 0, 1, 2, 0, 1, 2, 0, 0, 0, 0, 0, 0, 0, 1, 2, 0, 1, 2, 3 ];
pub(crate) static colouring_array_15: &'static [u8] = &[ 0, 2, 1, 1, 0, 2, 0, 0, 0, 0, 0, 0, 1, 2, 0, 2, 0, 1, 3 ];
pub(crate) static colouring_array_16: &'static [u8] = &[ 0, 2, 1, 0, 1, 2, 0, 0, 0, 0, 0, 0, 2, 0, 1, 1, 2, 0, 3 ];
pub(crate) static colouring_array_17: &'static [u8] = &[ 1, 0, 2, 2, 0, 1, 0, 0, 0, 0, 0, 0, 1, 2, 0, 2, 0, 1, 3 ];
pub(crate) static colouring_array_18: &'static [u8] = &[ 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 2, 1, 0, 2, 2 ];
pub(crate) static colouring_array_19: &'static [u8] = &[ 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 2, 0, 1, 2, 2 ];
pub(crate) static colouring_array_20: &'static [u8] = &[ 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 2, 0, 1, 2, 2 ];
pub(crate) static colouring_array_21: &'static [u8] = &[ 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 2, 0, 1, 2, 2 ];
pub(crate) static colouring_array_22: &'static [u8] = &[ 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 2, 1, 0, 2, 2 ];
pub(crate) static colouring_array_23: &'static [u8] = &[ 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 2, 0, 1, 2, 2 ];
pub(crate) static colouring_array_24: &'static [u8] = &[ 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 2, 0, 1, 2, 2 ];
pub(crate) static colouring_array_25: &'static [u8] = &[ 0, 1, 0, 1, 1, 0, 1, 0, 0, 0, 0, 0, 0, 1, 2, 0, 1, 2, 2 ];
pub(crate) static colouring_array_26: &'static [u8] = &[ 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 0, 1, 2, 0, 1, 2, 2 ];
pub(crate) static colouring_array_27: &'static [u8] = &[ 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 2, 1, 0, 2, 2 ];
pub(crate) static colouring_array_28: &'static [u8] = &[ 0, 1, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 2, 0, 1, 2, 2 ];

/*
macro_rules! ttd_arr {
    ($({
       $a:literal, $b:literal, $c:literal, $d:literal,
    }),+) => {
     pub(crate) static tiling_type_data: [TilingTypeData; 94] = [
         TilingTypeData {
             num_params: $a, num_aspects: $b, num_vertices: $c, num$(tdd! $b,)*
         }
     ];
    };
} */

pub (crate) static tiling_type_data: [TilingTypeData; 94] = [
	// IH00 is undefined
    ttd_null(),

	// IH01
	ttd(
		4, 1, 6, 3,
		edge_shapes_array_00,
		edge_orientations_array_00,
		edge_shape_ids_array_00,
		default_params_array_00,
		tiling_vertex_coefficients_array_00,
		translation_coefficients_array_00,
		aspect_coefficients_array_00,
		colouring_array_00
	),

	// IH02
	ttd(
		4, 2, 6, 3,
		edge_shapes_array_00,
		edge_orientations_array_01,
		edge_shape_ids_array_01,
		default_params_array_01,
		tiling_vertex_coefficients_array_01,
		translation_coefficients_array_01,
		aspect_coefficients_array_01,
		colouring_array_01
	),

	// IH03
	ttd(
		4, 2, 6, 3,
		edge_shapes_array_00,
		edge_orientations_array_02,
		edge_shape_ids_array_02,
		default_params_array_02,
		tiling_vertex_coefficients_array_02,
		translation_coefficients_array_02,
		aspect_coefficients_array_02,
		colouring_array_02
	),

	// IH04
	ttd(
		6, 2, 6, 5,
		edge_shapes_array_01,
		edge_orientations_array_03,
		edge_shape_ids_array_03,
		default_params_array_03,
		tiling_vertex_coefficients_array_03,
		translation_coefficients_array_03,
		aspect_coefficients_array_03,
		colouring_array_02
	),

	// IH05
	ttd(
		5, 4, 6, 4,
		edge_shapes_array_02,
		edge_orientations_array_04,
		edge_shape_ids_array_04,
		default_params_array_04,
		tiling_vertex_coefficients_array_04,
		translation_coefficients_array_04,
		aspect_coefficients_array_04,
		colouring_array_03
	),

	// IH06
	ttd(
		5, 4, 6, 4,
		edge_shapes_array_03,
		edge_orientations_array_05,
		edge_shape_ids_array_05,
		default_params_array_05,
		tiling_vertex_coefficients_array_05,
		translation_coefficients_array_05,
		aspect_coefficients_array_05,
		colouring_array_04
	),

	// IH07
	ttd(
		2, 3, 6, 3,
		edge_shapes_array_00,
		edge_orientations_array_06,
		edge_shape_ids_array_06,
		default_params_array_06,
		tiling_vertex_coefficients_array_06,
		translation_coefficients_array_06,
		aspect_coefficients_array_06,
		colouring_array_05
	),

	// IH08
	ttd(
		4, 1, 6, 3,
		edge_shapes_array_04,
		edge_orientations_array_07,
		edge_shape_ids_array_00,
		default_params_array_00,
		tiling_vertex_coefficients_array_00,
		translation_coefficients_array_00,
		aspect_coefficients_array_00,
		colouring_array_00
	),

	// IH09
	ttd(
		3, 2, 6, 2,
		edge_shapes_array_05,
		edge_orientations_array_08,
		edge_shape_ids_array_07,
		default_params_array_07,
		tiling_vertex_coefficients_array_07,
		translation_coefficients_array_07,
		aspect_coefficients_array_07,
		colouring_array_06
	),

	// IH10
	ttd(
		0, 1, 6, 1,
		edge_shapes_array_06,
		edge_orientations_array_06,
		edge_shape_ids_array_08,
		default_params_array_08,
		tiling_vertex_coefficients_array_08,
		translation_coefficients_array_08,
		aspect_coefficients_array_08,
		colouring_array_00
	),

	// IH11
	ttd(
		0, 1, 6, 1,
		edge_shapes_array_07,
		edge_orientations_array_07,
		edge_shape_ids_array_08,
		default_params_array_08,
		tiling_vertex_coefficients_array_08,
		translation_coefficients_array_09,
		aspect_coefficients_array_08,
		colouring_array_00
	),

	// IH12
	ttd(
		2, 1, 6, 2,
		edge_shapes_array_08,
		edge_orientations_array_09,
		edge_shape_ids_array_07,
		default_params_array_09,
		tiling_vertex_coefficients_array_09,
		translation_coefficients_array_10,
		aspect_coefficients_array_09,
		colouring_array_00
	),

	// IH13
	ttd(
		3, 2, 6, 3,
		edge_shapes_array_09,
		edge_orientations_array_10,
		edge_shape_ids_array_09,
		default_params_array_10,
		tiling_vertex_coefficients_array_10,
		translation_coefficients_array_11,
		aspect_coefficients_array_10,
		colouring_array_06
	),

	// IH14
	ttd(
		2, 1, 6, 2,
		edge_shapes_array_10,
		edge_orientations_array_11,
		edge_shape_ids_array_10,
		default_params_array_09,
		tiling_vertex_coefficients_array_11,
		translation_coefficients_array_12,
		aspect_coefficients_array_09,
		colouring_array_00
	),

	// IH15
	ttd(
		3, 2, 6, 3,
		edge_shapes_array_11,
		edge_orientations_array_12,
		edge_shape_ids_array_11,
		default_params_array_11,
		tiling_vertex_coefficients_array_12,
		translation_coefficients_array_13,
		aspect_coefficients_array_11,
		colouring_array_06
	),

	// IH16
	ttd(
		1, 3, 6, 2,
		edge_shapes_array_12,
		edge_orientations_array_13,
		edge_shape_ids_array_12,
		default_params_array_12,
		tiling_vertex_coefficients_array_13,
		translation_coefficients_array_14,
		aspect_coefficients_array_12,
		colouring_array_05
	),

	// IH17
	ttd(
		2, 1, 6, 2,
		edge_shapes_array_13,
		edge_orientations_array_14,
		edge_shape_ids_array_07,
		default_params_array_09,
		tiling_vertex_coefficients_array_09,
		translation_coefficients_array_10,
		aspect_coefficients_array_09,
		colouring_array_00
	),

	// IH18
	ttd(
		0, 1, 6, 1,
		edge_shapes_array_14,
		edge_orientations_array_06,
		edge_shape_ids_array_08,
		default_params_array_08,
		tiling_vertex_coefficients_array_08,
		translation_coefficients_array_09,
		aspect_coefficients_array_08,
		colouring_array_00
	),

	// IH19 is undefined
    ttd_null(),

	// IH20
	ttd(
		0, 1, 6, 1,
		edge_shapes_array_15,
		edge_orientations_array_07,
		edge_shape_ids_array_08,
		default_params_array_08,
		tiling_vertex_coefficients_array_08,
		translation_coefficients_array_09,
		aspect_coefficients_array_08,
		colouring_array_00
	),

	// IH21
	ttd(
		2, 6, 5, 3,
		edge_shapes_array_16,
		edge_orientations_array_15,
		edge_shape_ids_array_13,
		default_params_array_13,
		tiling_vertex_coefficients_array_14,
		translation_coefficients_array_15,
		aspect_coefficients_array_13,
		colouring_array_07
	),

	// IH22
	ttd(
		3, 2, 5, 3,
		edge_shapes_array_17,
		edge_orientations_array_16,
		edge_shape_ids_array_14,
		default_params_array_14,
		tiling_vertex_coefficients_array_15,
		translation_coefficients_array_16,
		aspect_coefficients_array_14,
		colouring_array_06
	),

	// IH23
	ttd(
		4, 2, 5, 4,
		edge_shapes_array_18,
		edge_orientations_array_17,
		edge_shape_ids_array_15,
		default_params_array_15,
		tiling_vertex_coefficients_array_16,
		translation_coefficients_array_17,
		aspect_coefficients_array_15,
		colouring_array_08
	),

	// IH24
	ttd(
		4, 4, 5, 4,
		edge_shapes_array_19,
		edge_orientations_array_17,
		edge_shape_ids_array_15,
		default_params_array_15,
		tiling_vertex_coefficients_array_16,
		translation_coefficients_array_18,
		aspect_coefficients_array_16,
		colouring_array_09
	),

	// IH25
	ttd(
		3, 4, 5, 3,
		edge_shapes_array_20,
		edge_orientations_array_16,
		edge_shape_ids_array_14,
		default_params_array_14,
		tiling_vertex_coefficients_array_15,
		translation_coefficients_array_19,
		aspect_coefficients_array_17,
		colouring_array_10
	),

	// IH26
	ttd(
		2, 2, 5, 3,
		edge_shapes_array_21,
		edge_orientations_array_18,
		edge_shape_ids_array_14,
		default_params_array_16,
		tiling_vertex_coefficients_array_17,
		translation_coefficients_array_20,
		aspect_coefficients_array_18,
		colouring_array_01
	),

	// IH27
	ttd(
		3, 4, 5, 3,
		edge_shapes_array_16,
		edge_orientations_array_19,
		edge_shape_ids_array_16,
		default_params_array_17,
		tiling_vertex_coefficients_array_18,
		translation_coefficients_array_21,
		aspect_coefficients_array_19,
		colouring_array_11
	),

	// IH28
	ttd(
		2, 4, 5, 3,
		edge_shapes_array_16,
		edge_orientations_array_15,
		edge_shape_ids_array_13,
		default_params_array_18,
		tiling_vertex_coefficients_array_19,
		translation_coefficients_array_22,
		aspect_coefficients_array_20,
		colouring_array_12
	),

	// IH29
	ttd(
		1, 4, 5, 2,
		edge_shapes_array_12,
		edge_orientations_array_20,
		edge_shape_ids_array_17,
		default_params_array_19,
		tiling_vertex_coefficients_array_20,
		translation_coefficients_array_23,
		aspect_coefficients_array_21,
		colouring_array_04
	),

	// IH30
	ttd(
		1, 6, 4, 3,
		edge_shapes_array_22,
		edge_orientations_array_21,
		edge_shape_ids_array_18,
		default_params_array_20,
		tiling_vertex_coefficients_array_21,
		translation_coefficients_array_24,
		aspect_coefficients_array_22,
		colouring_array_13
	),

	// IH31
	ttd(
		0, 6, 4, 2,
		edge_shapes_array_23,
		edge_orientations_array_22,
		edge_shape_ids_array_19,
		default_params_array_08,
		tiling_vertex_coefficients_array_22,
		translation_coefficients_array_25,
		aspect_coefficients_array_23,
		colouring_array_14
	),

	// IH32
	ttd(
		0, 6, 4, 2,
		edge_shapes_array_24,
		edge_orientations_array_23,
		edge_shape_ids_array_19,
		default_params_array_08,
		tiling_vertex_coefficients_array_22,
		translation_coefficients_array_26,
		aspect_coefficients_array_24,
		colouring_array_15
	),

	// IH33
	ttd(
		0, 3, 4, 2,
		edge_shapes_array_23,
		edge_orientations_array_22,
		edge_shape_ids_array_19,
		default_params_array_08,
		tiling_vertex_coefficients_array_23,
		translation_coefficients_array_08,
		aspect_coefficients_array_25,
		colouring_array_05
	),

	// IH34
	ttd(
		0, 3, 4, 1,
		edge_shapes_array_06,
		edge_orientations_array_24,
		edge_shape_ids_array_20,
		default_params_array_08,
		tiling_vertex_coefficients_array_23,
		translation_coefficients_array_09,
		aspect_coefficients_array_26,
		colouring_array_05
	),

	// IH35 is undefined
    ttd_null(),

	// IH36
	ttd(
		0, 3, 4, 1,
		edge_shapes_array_06,
		edge_orientations_array_25,
		edge_shape_ids_array_20,
		default_params_array_08,
		tiling_vertex_coefficients_array_23,
		translation_coefficients_array_08,
		aspect_coefficients_array_27,
		colouring_array_05
	),

	// IH37
	ttd(
		0, 3, 4, 1,
		edge_shapes_array_15,
		edge_orientations_array_26,
		edge_shape_ids_array_20,
		default_params_array_08,
		tiling_vertex_coefficients_array_23,
		translation_coefficients_array_08,
		aspect_coefficients_array_28,
		colouring_array_05
	),

	// IH38
	ttd(
		0, 6, 3, 2,
		edge_shapes_array_10,
		edge_orientations_array_27,
		edge_shape_ids_array_21,
		default_params_array_08,
		tiling_vertex_coefficients_array_24,
		translation_coefficients_array_27,
		aspect_coefficients_array_29,
		colouring_array_15
	),

	// IH39
	ttd(
		0, 6, 3, 2,
		edge_shapes_array_25,
		edge_orientations_array_27,
		edge_shape_ids_array_21,
		default_params_array_08,
		tiling_vertex_coefficients_array_24,
		translation_coefficients_array_28,
		aspect_coefficients_array_30,
		colouring_array_16
	),

	// IH40
	ttd(
		0, 6, 3, 2,
		edge_shapes_array_24,
		edge_orientations_array_28,
		edge_shape_ids_array_21,
		default_params_array_08,
		tiling_vertex_coefficients_array_24,
		translation_coefficients_array_29,
		aspect_coefficients_array_31,
		colouring_array_17
	),

	// IH41
	ttd(
		2, 1, 4, 2,
		edge_shapes_array_23,
		edge_orientations_array_22,
		edge_shape_ids_array_22,
		default_params_array_21,
		tiling_vertex_coefficients_array_25,
		translation_coefficients_array_30,
		aspect_coefficients_array_09,
		colouring_array_18
	),

	// IH42
	ttd(
		2, 2, 4, 3,
		edge_shapes_array_22,
		edge_orientations_array_29,
		edge_shape_ids_array_23,
		default_params_array_21,
		tiling_vertex_coefficients_array_25,
		translation_coefficients_array_31,
		aspect_coefficients_array_32,
		colouring_array_19
	),

	// IH43
	ttd(
		2, 2, 4, 2,
		edge_shapes_array_23,
		edge_orientations_array_30,
		edge_shape_ids_array_22,
		default_params_array_21,
		tiling_vertex_coefficients_array_25,
		translation_coefficients_array_31,
		aspect_coefficients_array_33,
		colouring_array_19
	),

	// IH44
	ttd(
		2, 2, 4, 2,
		edge_shapes_array_23,
		edge_orientations_array_31,
		edge_shape_ids_array_24,
		default_params_array_22,
		tiling_vertex_coefficients_array_26,
		translation_coefficients_array_32,
		aspect_coefficients_array_34,
		colouring_array_20
	),

	// IH45
	ttd(
		2, 2, 4, 3,
		edge_shapes_array_22,
		edge_orientations_array_32,
		edge_shape_ids_array_23,
		default_params_array_23,
		tiling_vertex_coefficients_array_27,
		translation_coefficients_array_33,
		aspect_coefficients_array_35,
		colouring_array_20
	),

	// IH46
	ttd(
		4, 2, 4, 4,
		edge_shapes_array_26,
		edge_orientations_array_33,
		edge_shape_ids_array_25,
		default_params_array_24,
		tiling_vertex_coefficients_array_28,
		translation_coefficients_array_34,
		aspect_coefficients_array_36,
		colouring_array_20
	),

	// IH47
	ttd(
		2, 2, 4, 3,
		edge_shapes_array_27,
		edge_orientations_array_29,
		edge_shape_ids_array_23,
		default_params_array_21,
		tiling_vertex_coefficients_array_25,
		translation_coefficients_array_35,
		aspect_coefficients_array_37,
		colouring_array_19
	),

	// IH48 is undefined
    ttd_null(),

	// IH49
	ttd(
		3, 4, 4, 4,
		edge_shapes_array_28,
		edge_orientations_array_33,
		edge_shape_ids_array_25,
		default_params_array_25,
		tiling_vertex_coefficients_array_29,
		translation_coefficients_array_36,
		aspect_coefficients_array_38,
		colouring_array_21
	),

	// IH50
	ttd(
		2, 4, 4, 3,
		edge_shapes_array_29,
		edge_orientations_array_29,
		edge_shape_ids_array_23,
		default_params_array_21,
		tiling_vertex_coefficients_array_25,
		translation_coefficients_array_37,
		aspect_coefficients_array_39,
		colouring_array_22
	),

	// IH51
	ttd(
		3, 4, 4, 3,
		edge_shapes_array_27,
		edge_orientations_array_32,
		edge_shape_ids_array_23,
		default_params_array_26,
		tiling_vertex_coefficients_array_30,
		translation_coefficients_array_38,
		aspect_coefficients_array_40,
		colouring_array_21
	),

	// IH52
	ttd(
		1, 4, 4, 2,
		edge_shapes_array_23,
		edge_orientations_array_34,
		edge_shape_ids_array_22,
		default_params_array_20,
		tiling_vertex_coefficients_array_31,
		translation_coefficients_array_39,
		aspect_coefficients_array_41,
		colouring_array_23
	),

	// IH53
	ttd(
		3, 4, 4, 3,
		edge_shapes_array_27,
		edge_orientations_array_35,
		edge_shape_ids_array_26,
		default_params_array_27,
		tiling_vertex_coefficients_array_32,
		translation_coefficients_array_40,
		aspect_coefficients_array_42,
		colouring_array_21
	),

	// IH54
	ttd(
		2, 4, 4, 4,
		edge_shapes_array_30,
		edge_orientations_array_33,
		edge_shape_ids_array_25,
		default_params_array_28,
		tiling_vertex_coefficients_array_33,
		translation_coefficients_array_41,
		aspect_coefficients_array_43,
		colouring_array_22
	),

	// IH55
	ttd(
		0, 4, 4, 2,
		edge_shapes_array_23,
		edge_orientations_array_24,
		edge_shape_ids_array_24,
		default_params_array_08,
		tiling_vertex_coefficients_array_34,
		translation_coefficients_array_42,
		aspect_coefficients_array_44,
		colouring_array_24
	),

	// IH56
	ttd(
		1, 8, 4, 3,
		edge_shapes_array_22,
		edge_orientations_array_36,
		edge_shape_ids_array_26,
		default_params_array_29,
		tiling_vertex_coefficients_array_35,
		translation_coefficients_array_43,
		aspect_coefficients_array_45,
		colouring_array_25
	),

	// IH57
	ttd(
		2, 1, 4, 2,
		edge_shapes_array_31,
		edge_orientations_array_33,
		edge_shape_ids_array_22,
		default_params_array_21,
		tiling_vertex_coefficients_array_25,
		translation_coefficients_array_30,
		aspect_coefficients_array_09,
		colouring_array_18
	),

	// IH58
	ttd(
		2, 2, 4, 2,
		edge_shapes_array_32,
		edge_orientations_array_33,
		edge_shape_ids_array_22,
		default_params_array_21,
		tiling_vertex_coefficients_array_25,
		translation_coefficients_array_31,
		aspect_coefficients_array_32,
		colouring_array_19
	),

	// IH59
	ttd(
		1, 2, 4, 1,
		edge_shapes_array_06,
		edge_orientations_array_31,
		edge_shape_ids_array_20,
		default_params_array_30,
		tiling_vertex_coefficients_array_36,
		translation_coefficients_array_44,
		aspect_coefficients_array_46,
		colouring_array_20
	),

	// IH60 is undefined
    ttd_null(),
	// IH61
	ttd(
		0, 2, 4, 1,
		edge_shapes_array_06,
		edge_orientations_array_24,
		edge_shape_ids_array_20,
		default_params_array_08,
		tiling_vertex_coefficients_array_34,
		translation_coefficients_array_45,
		aspect_coefficients_array_47,
		colouring_array_20
	),

	// IH62
	ttd(
		0, 1, 4, 1,
		edge_shapes_array_07,
		edge_orientations_array_33,
		edge_shape_ids_array_20,
		default_params_array_08,
		tiling_vertex_coefficients_array_34,
		translation_coefficients_array_46,
		aspect_coefficients_array_08,
		colouring_array_18
	),

	// IH63 is undefined
    ttd_null(),

	// IH64
	ttd(
		1, 1, 4, 2,
		edge_shapes_array_33,
		edge_orientations_array_37,
		edge_shape_ids_array_22,
		default_params_array_20,
		tiling_vertex_coefficients_array_31,
		translation_coefficients_array_47,
		aspect_coefficients_array_48,
		colouring_array_18
	),

	// IH65 is undefined
    ttd_null(),

	// IH66
	ttd(
		1, 2, 4, 2,
		edge_shapes_array_34,
		edge_orientations_array_37,
		edge_shape_ids_array_22,
		default_params_array_20,
		tiling_vertex_coefficients_array_31,
		translation_coefficients_array_48,
		aspect_coefficients_array_49,
		colouring_array_19
	),

	// IH67
	ttd(
		2, 2, 4, 3,
		edge_shapes_array_21,
		edge_orientations_array_38,
		edge_shape_ids_array_23,
		default_params_array_23,
		tiling_vertex_coefficients_array_27,
		translation_coefficients_array_49,
		aspect_coefficients_array_50,
		colouring_array_20
	),

	// IH68
	ttd(
		1, 1, 4, 1,
		edge_shapes_array_06,
		edge_orientations_array_39,
		edge_shape_ids_array_20,
		default_params_array_30,
		tiling_vertex_coefficients_array_36,
		translation_coefficients_array_50,
		aspect_coefficients_array_48,
		colouring_array_18
	),

	// IH69
	ttd(
		2, 2, 4, 2,
		edge_shapes_array_31,
		edge_orientations_array_26,
		edge_shape_ids_array_24,
		default_params_array_22,
		tiling_vertex_coefficients_array_26,
		translation_coefficients_array_32,
		aspect_coefficients_array_51,
		colouring_array_20
	),

	// IH70 is undefined
    ttd_null(),

	// IH71
	ttd(
		0, 4, 4, 1,
		edge_shapes_array_06,
		edge_orientations_array_40,
		edge_shape_ids_array_20,
		default_params_array_08,
		tiling_vertex_coefficients_array_34,
		translation_coefficients_array_42,
		aspect_coefficients_array_52,
		colouring_array_24
	),

	// IH72
	ttd(
		1, 1, 4, 2,
		edge_shapes_array_24,
		edge_orientations_array_33,
		edge_shape_ids_array_22,
		default_params_array_20,
		tiling_vertex_coefficients_array_31,
		translation_coefficients_array_47,
		aspect_coefficients_array_48,
		colouring_array_18
	),

	// IH73
	ttd(
		0, 2, 4, 1,
		edge_shapes_array_14,
		edge_orientations_array_24,
		edge_shape_ids_array_20,
		default_params_array_08,
		tiling_vertex_coefficients_array_34,
		translation_coefficients_array_45,
		aspect_coefficients_array_47,
		colouring_array_20
	),

	// IH74
	ttd(
		1, 1, 4, 1,
		edge_shapes_array_07,
		edge_orientations_array_26,
		edge_shape_ids_array_20,
		default_params_array_30,
		tiling_vertex_coefficients_array_36,
		translation_coefficients_array_50,
		aspect_coefficients_array_48,
		colouring_array_18
	),

	// IH75 is undefined
    ttd_null(),

	// IH76
	ttd(
		0, 1, 4, 1,
		edge_shapes_array_15,
		edge_orientations_array_33,
		edge_shape_ids_array_20,
		default_params_array_08,
		tiling_vertex_coefficients_array_34,
		translation_coefficients_array_46,
		aspect_coefficients_array_08,
		colouring_array_18
	),

	// IH77
	ttd(
		0, 12, 3, 3,
		edge_shapes_array_35,
		edge_orientations_array_41,
		edge_shape_ids_array_27,
		default_params_array_08,
		tiling_vertex_coefficients_array_37,
		translation_coefficients_array_51,
		aspect_coefficients_array_53,
		colouring_array_26
	),

	// IH78
	ttd(
		1, 4, 3, 3,
		edge_shapes_array_36,
		edge_orientations_array_41,
		edge_shape_ids_array_27,
		default_params_array_20,
		tiling_vertex_coefficients_array_38,
		translation_coefficients_array_52,
		aspect_coefficients_array_54,
		colouring_array_22
	),

	// IH79
	ttd(
		0, 4, 3, 2,
		edge_shapes_array_25,
		edge_orientations_array_27,
		edge_shape_ids_array_21,
		default_params_array_08,
		tiling_vertex_coefficients_array_39,
		translation_coefficients_array_53,
		aspect_coefficients_array_55,
		colouring_array_27
	),

	// IH80 is undefined
    ttd_null(),

	// IH81
	ttd(
		0, 8, 3, 2,
		edge_shapes_array_10,
		edge_orientations_array_27,
		edge_shape_ids_array_21,
		default_params_array_08,
		tiling_vertex_coefficients_array_39,
		translation_coefficients_array_54,
		aspect_coefficients_array_56,
		colouring_array_25
	),

	// IH82
	ttd(
		0, 4, 3, 2,
		edge_shapes_array_24,
		edge_orientations_array_28,
		edge_shape_ids_array_21,
		default_params_array_08,
		tiling_vertex_coefficients_array_39,
		translation_coefficients_array_55,
		aspect_coefficients_array_57,
		colouring_array_27
	),

	// IH83
	ttd(
		1, 2, 3, 2,
		edge_shapes_array_10,
		edge_orientations_array_42,
		edge_shape_ids_array_28,
		default_params_array_31,
		tiling_vertex_coefficients_array_40,
		translation_coefficients_array_56,
		aspect_coefficients_array_58,
		colouring_array_20
	),

	// IH84
	ttd(
		2, 2, 3, 3,
		edge_shapes_array_04,
		edge_orientations_array_41,
		edge_shape_ids_array_27,
		default_params_array_32,
		tiling_vertex_coefficients_array_41,
		translation_coefficients_array_57,
		aspect_coefficients_array_59,
		colouring_array_20
	),

	// IH85
	ttd(
		2, 4, 3, 3,
		edge_shapes_array_37,
		edge_orientations_array_41,
		edge_shape_ids_array_27,
		default_params_array_32,
		tiling_vertex_coefficients_array_41,
		translation_coefficients_array_58,
		aspect_coefficients_array_60,
		colouring_array_21
	),

	// IH86
	ttd(
		1, 4, 3, 2,
		edge_shapes_array_25,
		edge_orientations_array_42,
		edge_shape_ids_array_28,
		default_params_array_31,
		tiling_vertex_coefficients_array_40,
		translation_coefficients_array_59,
		aspect_coefficients_array_61,
		colouring_array_21
	),

	// IH87 is undefined
    ttd_null(),

	// IH88
	ttd(
		0, 6, 3, 2,
		edge_shapes_array_25,
		edge_orientations_array_43,
		edge_shape_ids_array_28,
		default_params_array_08,
		tiling_vertex_coefficients_array_42,
		translation_coefficients_array_60,
		aspect_coefficients_array_62,
		colouring_array_28
	),

	// IH89 is undefined
    ttd_null(),

	// IH90
	ttd(
		0, 2, 3, 1,
		edge_shapes_array_07,
		edge_orientations_array_41,
		edge_shape_ids_array_29,
		default_params_array_08,
		tiling_vertex_coefficients_array_42,
		translation_coefficients_array_09,
		aspect_coefficients_array_63,
		colouring_array_20
	),

	// IH91
	ttd(
		1, 2, 3, 2,
		edge_shapes_array_32,
		edge_orientations_array_44,
		edge_shape_ids_array_28,
		default_params_array_31,
		tiling_vertex_coefficients_array_40,
		translation_coefficients_array_61,
		aspect_coefficients_array_64,
		colouring_array_20
	),

	// IH92 is undefined
    ttd_null(),

	// IH93
	ttd(
		0, 2, 3, 1,
		edge_shapes_array_15,
		edge_orientations_array_41,
		edge_shape_ids_array_29,
		default_params_array_08,
		tiling_vertex_coefficients_array_42,
		translation_coefficients_array_09,
		aspect_coefficients_array_63,
		colouring_array_20
	),
];

const fn ttd(
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
    colouring: &'static [u8]
) -> TilingTypeData {
    TilingTypeData {
        num_params,
        num_aspects,
        num_vertices,
        num_edge_shapes,
        edge_shapes,
        edge_orientations,
        edge_shape_ids,
        default_params,
        tiling_vertex_coeffs,
        translation_vertex_coeffs,
        aspect_xform_coeffs,
        colouring,
    }
}

const fn ttd_null() -> TilingTypeData {
    TilingTypeData { num_params: 0, num_aspects: 0, num_vertices: 0, num_edge_shapes: 0, edge_shapes: &[], edge_orientations: &[], edge_shape_ids: &[], default_params: &[], tiling_vertex_coeffs: &[], translation_vertex_coeffs: &[], aspect_xform_coeffs: &[], colouring: &[] }
}
