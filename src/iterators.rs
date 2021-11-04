use glam::{dmat2, dvec2, DMat3, DVec2};

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

#[derive(Debug)]
pub struct FillRegionStep {
    transform: DMat3,
}

pub struct FillRegionIterator<'tiling> {
    tiling: &'tiling IsohedralTiling,
    done: bool,
    x: f64,
    y: f64,
    xlo: f64,
    xhi: f64,
}

impl<'tiling> Iterator for FillRegionIterator<'tiling> {
    type Item = FillRegionStep;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}

impl<'tiling> FillRegionIterator<'tiling> {
    pub fn new(tiling: &'tiling IsohedralTiling, x: f64, y: f64, xlo: f64, xhi: f64) -> Self {
        Self {
            tiling,
            x,
            y,
            xlo,
            xhi,
            done: false,
        }
    }
}

pub struct FillAlgorithm<'tiling> {
    tiling: &'tiling IsohedralTiling,
    num_calls: usize,
    data: [AlgoData; 3],
}

impl<'tiling> FillAlgorithm<'tiling> {
    pub fn new(
        tiling: &'tiling IsohedralTiling,
        a: &DVec2,
        b: &DVec2,
        c: &DVec2,
        d: &DVec2,
    ) -> Self {
        let mut algo = Self { tiling, num_calls: 0, data: [AlgoData::default(); 3] };

        let t1 = tiling.t1();
        let t2 = tiling.t2();

        let det = 1.0 / (t1.x * t2.y - t2.x * t1.y);

        let m_bc = dmat2(
            dvec2(t2.y * det, -t1.y * det),
            dvec2(-t2.x * det, t1.x * det),
        );
        let mut pts = [m_bc * *a, m_bc * *b, m_bc * *c, m_bc * *d];
        if det < 0.0 {
            pts.swap(1, 3);
        }

        if (pts[0].y - pts[1].y).abs() < 1e-7 {
            algo.fill_fix_y(&pts[0], &pts[1], &pts[2], &pts[3], true);
        } else if (pts[1].y - pts[2].y).abs() < 1e-7 {
            algo.fill_fix_y(&pts[1], &pts[2], &pts[3], &pts[0], true);
        } else {
            let mut lowest = 0;
            for idx in 1..4 {
                if pts[idx].y < pts[lowest].y {
                    lowest = idx;
                }
            }

            let bottom = pts[lowest];
            let mut left = pts[(lowest + 1) %4];
            let top = pts[(lowest + 2) %4];
            let mut right = pts[(lowest + 3) %4];

            if left.x < right.x {
                std::mem::swap(&mut left, &mut right);
            }

            if left.y < right.y {
                let r1 = sample_at_height(&bottom,&right, left.y);
                let l2 = sample_at_height(&left, &top, right.y);
                algo.fill_fix_x(&bottom, &bottom, &r1, &left, false);
                algo.fill_fix_x(&left, &r1, &right, &l2, false);
                algo.fill_fix_x(&l2, &right, &top, &top, true);
            } else {
                let l1 = sample_at_height(&bottom, &left, right.y);
                let r2 = sample_at_height(&right, &top, left.y);
                algo.fill_fix_x(&bottom, &bottom, &right, &l1, false);
                algo.fill_fix_x(&l1, &right, &r2, &left, false);
                algo.fill_fix_x(&left, &r2, &top, &top, true);
            }
        }



        algo
    }

    fn fill_fix_x(&mut self, a: &DVec2, b: &DVec2, c: &DVec2, d: &DVec2, do_top: bool) {
        if a.x > b.x {
            self.do_fill(b, a, d, c, do_top);
        } else {
            self.do_fill(a, b, c, d, do_top);
        }
    }

    fn fill_fix_y(&mut self, a: &DVec2, b: &DVec2, c: &DVec2, d: &DVec2, do_top: bool) {
        if a.y > c.y {
            self.do_fill(c, d, a, b, do_top);
        } else {
            self.do_fill(a, b, c, d, do_top);
        }
    }

    fn do_fill(&mut self, a: &DVec2, b: &DVec2, c: &DVec2, d: &DVec2, do_top: bool) {
        self.data[self.num_calls].xlo = a.x;
        self.data[self.num_calls].dxlo = (d.x - a.x) / (d.y - a.y);
        self.data[self.num_calls].xhi = b.x;
        self.data[self.num_calls].dxhi = (c.x - b.x) / (c.y - b.y);
        self.data[self.num_calls].ymin = a.y;
        self.data[self.num_calls].ymax = c.y;

        if do_top {
            self.data[self.num_calls].ymax += 1.0;
        }

        self.num_calls += 1;
    }
}

fn sample_at_height(p: &DVec2, q: &DVec2, y: f64) -> DVec2 {
    let t = (y - p.y) / (q.y - p.y);
    dvec2((1.0 - t) * p.x + t * q.x, y)
}

#[derive(Debug, Default, Clone, Copy)]
struct AlgoData {
    ymin: f64,
    ymax: f64,
    xlo: f64,
    xhi: f64,
    dxlo: f64,
    dxhi: f64,
}
