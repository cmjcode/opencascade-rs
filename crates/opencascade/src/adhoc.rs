use crate::primitives::Shape;
use cxx::UniquePtr;
use glam::DVec3;
use opencascade_sys as ffi;
use std::ops::{Deref, DerefMut};

/// Wrapper for the [`Shape`] struct that provides an "ad-hoc" API.
pub struct AdHocShape(pub Shape);

impl Deref for AdHocShape {
    type Target = Shape;

    fn deref(&self) -> &Shape {
        &self.0
    }
}

impl DerefMut for AdHocShape {
    fn deref_mut(&mut self) -> &mut Shape {
        &mut self.0
    }
}

impl AdHocShape {
    /// Internal helper to create [Self] from the FFI inner type.
    fn from_inner(inner: UniquePtr<ffi::topo_ds::TopoDS_Shape>) -> Self {
        Self(Shape { inner })
    }

    /// Make a box with a corner at (0,0,0) and with size (x,y,z)
    pub fn make_box(x: f64, y: f64, z: f64) -> Self {
        let point = ffi::gp::new_point(0.0, 0.0, 0.0);
        let mut my_box = ffi::b_rep_prim_api::BRepPrimAPI_MakeBox_new(&point, x, y, z);
        let inner = ffi::topo_ds::TopoDS_Shape_to_owned(my_box.pin_mut().Shape());

        Self::from_inner(inner)
    }

    /// Make a box with one corner at p1, and the opposite corner at p2.
    pub fn make_box_point_point(p1: DVec3, p2: DVec3) -> Self {
        let min_corner = p1.min(p2);
        let max_corner = p1.max(p2);

        let point = ffi::gp::new_point(min_corner.x, min_corner.y, min_corner.z);
        let diff = max_corner - min_corner;
        let mut my_box =
            ffi::b_rep_prim_api::BRepPrimAPI_MakeBox_new(&point, diff.x, diff.y, diff.z);
        let inner = ffi::topo_ds::TopoDS_Shape_to_owned(my_box.pin_mut().Shape());

        Self::from_inner(inner)
    }

    /// Make a cylinder with its bottom at point p, with radius r and height h.
    pub fn make_cylinder(p: DVec3, r: f64, h: f64) -> Self {
        let point = ffi::gp::new_point(p.x, p.y, p.z);
        let cylinder_axis = ffi::gp::gp::DZ();
        let cylinder_coord_system = ffi::gp::gp_Ax2_new(&point, cylinder_axis);

        let mut cylinder =
            ffi::b_rep_prim_api::BRepPrimAPI_MakeCylinder_new(&cylinder_coord_system, r, h);
        let inner = ffi::topo_ds::TopoDS_Shape_to_owned(cylinder.pin_mut().Shape());

        Self::from_inner(inner)
    }

    /// Make a sphere of radius r, centered at the origin.
    pub fn make_sphere(r: f64) -> Self {
        let point = ffi::gp::new_point(0.0, 0.0, 0.0);
        let axis = ffi::gp::gp_Ax2_new(&point, ffi::gp::gp::DZ());
        let mut sphere =
            ffi::b_rep_prim_api::BRepPrimAPI_MakeSphere_new(&axis, r, std::f64::consts::TAU);
        let inner = ffi::topo_ds::TopoDS_Shape_to_owned(sphere.pin_mut().Shape());

        Self::from_inner(inner)
    }

    pub fn fillet_edges(&mut self, radius: f64) {
        let _ = self.0.fillet_edges(radius, self.0.edges());
    }

    pub fn chamfer_edges(&mut self, distance: f64) {
        let _ = self.0.chamfer_edges(distance, self.0.edges());
    }

    pub fn subtract(&mut self, other: &Shape) -> Result<(), crate::Error> {
        let mut cut_operation =
            ffi::b_rep_algo_api::BRepAlgoAPI_Cut_ctor_checked(&self.inner, &other.inner)
                .map_err(|e| crate::Error::BooleanOpFailed(e.what().to_string()))?;

        let cut_shape = ffi::b_rep_algo_api::BRepAlgoAPI_Cut_shape_checked(cut_operation.pin_mut())
            .map_err(|e| crate::Error::BooleanOpFailed(e.what().to_string()))?;
        self.inner = ffi::topo_ds::TopoDS_Shape_to_owned(cut_shape);
        Ok(())
    }

    pub fn union(&mut self, other: &Shape) -> Result<(), crate::Error> {
        let mut fuse_operation =
            ffi::b_rep_algo_api::BRepAlgoAPI_Fuse_ctor_checked(&self.inner, &other.inner)
                .map_err(|e| crate::Error::BooleanOpFailed(e.what().to_string()))?;

        let fuse_shape =
            ffi::b_rep_algo_api::BRepAlgoAPI_Fuse_shape_checked(fuse_operation.pin_mut())
                .map_err(|e| crate::Error::BooleanOpFailed(e.what().to_string()))?;
        self.inner = ffi::topo_ds::TopoDS_Shape_to_owned(fuse_shape);
        Ok(())
    }

    pub fn intersect(&mut self, other: &Shape) -> Result<(), crate::Error> {
        let mut common_operation =
            ffi::b_rep_algo_api::BRepAlgoAPI_Common_ctor_checked(&self.inner, &other.inner)
                .map_err(|e| crate::Error::BooleanOpFailed(e.what().to_string()))?;

        let common_shape =
            ffi::b_rep_algo_api::BRepAlgoAPI_Common_shape_checked(common_operation.pin_mut())
                .map_err(|e| crate::Error::BooleanOpFailed(e.what().to_string()))?;
        self.inner = ffi::topo_ds::TopoDS_Shape_to_owned(common_shape);
        Ok(())
    }
}
