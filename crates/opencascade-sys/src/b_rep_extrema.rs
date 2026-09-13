pub use inner::*;

#[cxx::bridge]
mod inner {
    unsafe extern "C++" {
        include!("opencascade-sys/include/b_rep_extrema.hxx");

        type TopoDS_Shape = crate::topo_ds::TopoDS_Shape;

        /// Jarak minimum EKSAK antara dua shape B-rep
        /// (`BRepExtrema_DistShapeShape`).
        ///
        /// Berbeda dari jarak antar bounding box atau antar titik mesh:
        /// nilainya dihitung terhadap permukaan analitik sesungguhnya, jadi
        /// benar untuk silinder dan permukaan lengkung lain yang justru
        /// paling sering menentukan celah pada perakitan.
        type DucadDistance;
        #[cxx_name = "construct_unique"]
        fn DucadDistance_ctor(a: &TopoDS_Shape, b: &TopoDS_Shape) -> UniquePtr<DucadDistance>;
        fn is_done(self: &DucadDistance) -> bool;
        fn value(self: &DucadDistance) -> f64;
        fn p1x(self: &DucadDistance) -> f64;
        fn p1y(self: &DucadDistance) -> f64;
        fn p1z(self: &DucadDistance) -> f64;
        fn p2x(self: &DucadDistance) -> f64;
        fn p2y(self: &DucadDistance) -> f64;
        fn p2z(self: &DucadDistance) -> f64;
    }
}
