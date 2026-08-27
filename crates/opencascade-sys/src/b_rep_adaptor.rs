pub use inner::*;

#[cxx::bridge]
mod inner {
    unsafe extern "C++" {
        include!("opencascade-sys/include/b_rep_adaptor.hxx");

        type gp_Pnt = crate::gp::gp_Pnt;
        type gp_Cylinder = crate::gp::gp_Cylinder;
        type gp_Cone = crate::gp::gp_Cone;
        type gp_Sphere = crate::gp::gp_Sphere;
        type GeomAbs_CurveType = crate::geom_abs::GeomAbs_CurveType;
        type GeomAbs_SurfaceType = crate::geom_abs::GeomAbs_SurfaceType;
        type TopoDS_Edge = crate::topo_ds::TopoDS_Edge;
        type TopoDS_Face = crate::topo_ds::TopoDS_Face;

        type BRepAdaptor_Curve;
        #[cxx_name = "construct_unique"]
        pub fn BRepAdaptor_Curve_new(edge: &TopoDS_Edge) -> UniquePtr<BRepAdaptor_Curve>;
        pub fn FirstParameter(self: &BRepAdaptor_Curve) -> f64;
        pub fn LastParameter(self: &BRepAdaptor_Curve) -> f64;
        pub fn BRepAdaptor_Curve_value(curve: &BRepAdaptor_Curve, u: f64) -> UniquePtr<gp_Pnt>;
        pub fn GetType(self: &BRepAdaptor_Curve) -> GeomAbs_CurveType;

        type BRepAdaptor_Surface;
        pub fn BRepAdaptor_Surface_ctor(
            face: &TopoDS_Face,
            restriction: bool,
        ) -> UniquePtr<BRepAdaptor_Surface>;
        pub fn GetType(self: &BRepAdaptor_Surface) -> GeomAbs_SurfaceType;
        pub fn BRepAdaptor_Surface_cylinder(
            surface: &BRepAdaptor_Surface,
        ) -> Result<UniquePtr<gp_Cylinder>>;
        pub fn BRepAdaptor_Surface_cone(
            surface: &BRepAdaptor_Surface,
        ) -> Result<UniquePtr<gp_Cone>>;
        pub fn BRepAdaptor_Surface_sphere(
            surface: &BRepAdaptor_Surface,
        ) -> Result<UniquePtr<gp_Sphere>>;
    }
}
