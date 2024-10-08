use crate::types::{Point2, PositiveI32};
use manifold_sys::{
    manifold_alloc_simple_polygon, manifold_simple_polygon, manifold_simple_polygon_get_point,
    manifold_simple_polygon_length, ManifoldSimplePolygon, ManifoldVec2,
};
use std::os::raw::{c_int, c_void};

pub struct SimplePolygon(*mut ManifoldSimplePolygon);

pub fn new_from_points(points: Vec<impl Into<ManifoldVec2>>) -> SimplePolygon {
    let mut points = points.into_iter().map(|x| x.into()).collect::<Vec<_>>();
    let points_length = points.len();

    let polygon_ptr = unsafe { manifold_alloc_simple_polygon() };
    unsafe {
        manifold_simple_polygon(
            polygon_ptr as *mut c_void,
            points.as_mut_ptr(),
            points_length,
        )
    };
    SimplePolygon(polygon_ptr)
}

pub(crate) fn from_ptr(ptr: *mut ManifoldSimplePolygon) -> SimplePolygon {
    SimplePolygon(ptr)
}

impl SimplePolygon {
    pub fn point_count(&self) -> usize {
        unsafe { manifold_simple_polygon_length(self.0) }
    }
    pub fn get_point(&self, index: impl Into<PositiveI32>) -> Option<Point2> {
        let index: i32 = index.into().get();
        if index as i64 >= self.point_count() as i64 {
            return None;
        }
        let vec = unsafe { manifold_simple_polygon_get_point(self.0, c_int::from(index)) };
        Some(Point2::from(vec))
    }

    pub(crate) fn ptr(&self) -> *mut ManifoldSimplePolygon {
        self.0
    }
}
