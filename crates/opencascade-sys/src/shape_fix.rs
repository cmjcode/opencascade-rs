pub use inner::*;

#[cxx::bridge]
mod inner {
    unsafe extern "C++" {
        include!("opencascade-sys/include/shape_fix.hxx");

        type TopoDS_Shape = crate::topo_ds::TopoDS_Shape;

        /// Perbaikan otomatis shape B-rep yang rusak ringan (wire tidak
        /// tertutup, orientasi face terbalik, toleransi tidak konsisten).
        /// Tidak menjamin berhasil — pemanggil WAJIB memeriksa ulang lewat
        /// `BRepCheck_Analyzer`.
        type ShapeFix_Shape;
        #[cxx_name = "construct_unique"]
        fn ShapeFix_Shape_ctor(shape: &TopoDS_Shape) -> UniquePtr<ShapeFix_Shape>;
        fn ShapeFix_Shape_perform(fixer: Pin<&mut ShapeFix_Shape>) -> bool;
        fn ShapeFix_Shape_result(fixer: &ShapeFix_Shape) -> UniquePtr<TopoDS_Shape>;
    }
}
