use glam::DMat3;

use crate::{EdgeShape, IsohedralTiling};

pub struct Shape {
    t: DMat3,
    id: u8,
    shape: EdgeShape,
    rev: bool,
    second: bool,
}

impl Shape {
    /// Get a reference to the shape data's t.
    pub fn t(&self) -> DMat3 {
        self.t
    }

    /// Get a reference to the shape data's id.
    pub fn id(&self) -> u8 {
        self.id
    }

    /// Get a reference to the shape data's shape.
    pub fn shape(&self) -> EdgeShape {
        self.shape
    }

    /// Get a reference to the shape data's rev.
    pub fn is_reversed(&self) -> bool {
        self.rev
    }

    /// Get a reference to the shape's second.
    pub fn second(&self) -> bool {
        self.second
    }
}

pub struct TilingShapeIterator<'tiling> {
    pub(crate) idx: usize,
    pub(crate) tiling: &'tiling IsohedralTiling,
}

impl<'tiling> Iterator for TilingShapeIterator<'tiling> {
    type Item = Shape;

    fn next(&mut self) -> Option<Self::Item> {
        if self.idx < self.tiling.num_vertices() as usize {
            let an_id = self.tiling.edge_shape_ids[self.idx];
            let data = Shape {
                t: self.tiling.edges[self.idx],
                id: an_id,
                shape: self.tiling.edge_shapes[an_id as usize],
                rev: self.tiling.reversals[self.idx],
                second: false,
            };

            self.idx += 1;
            Some(data)
        } else {
            None
        }
    }
}

pub struct TilingShapePartIterator<'tiling> {
    pub(crate) idx: usize,
    pub(crate) second: bool,
    pub(crate) tiling: &'tiling IsohedralTiling,
}

impl<'tiling> Iterator for TilingShapePartIterator<'tiling> {
    type Item = Shape;

    fn next(&mut self) -> Option<Self::Item> {
        if self.idx < self.tiling.num_vertices() as usize {
            let an_id = self.tiling.edge_shape_ids[self.idx];
            let shp = self.tiling.edge_shapes[an_id as usize];

            if (shp == EdgeShape::J) || (shp == EdgeShape::I) {
                let data = Shape {
                    t: self.tiling.edges[self.idx],
                    id: an_id,
                    shape: shp,
                    rev: self.tiling.reversals[self.idx],
                    second: false,
                };
                self.idx += 1;
                Some(data)
            } else {
                let indices = if self.tiling.reversals[self.idx] {
                    [1, 0]
                } else {
                    [0, 1]
                };
                let ms = if shp == EdgeShape::U {
                    &crate::utils::TSPI_U[..]
                } else {
                    &crate::utils::TSPI_S[..]
                };

                let data = if !self.second {
                    let data = Shape {
                        t: (self.tiling.edges[self.idx] * ms[indices[0]]),
                        id: an_id,
                        shape: shp,
                        rev: false,
                        second: false,
                    };
                    self.second = true;
                    data
                } else {
                    let data = Shape {
                        t: (self.tiling.edges[self.idx] * ms[indices[1]]),
                        id: an_id,
                        shape: shp,
                        rev: true,
                        second: true,
                    };
                    self.second = false;
                    self.idx += 1;
                    data
                };
                Some(data)
            }
        } else {
            None
        }
    }
}
