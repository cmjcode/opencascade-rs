pub use inner::*;

#[cxx::bridge]
mod inner {
    unsafe extern "C++" {
        include!("opencascade-sys/include/b_rep_check.hxx");

        type TopoDS_Shape = crate::topo_ds::TopoDS_Shape;

        /// Pemeriksa validitas topologi/geometri B-rep OCCT.
        ///
        /// Operasi boolean dan fillet OCCT bisa "berhasil" — mengembalikan
        /// shape tanpa melapor error — sambil menghasilkan solid yang
        /// sebetulnya tidak valid (face saling potong, wire tidak tertutup,
        /// orientasi terbalik). Tanpa pemeriksa ini, geometri rusak itu baru
        /// ketahuan jauh di hilir: saat diekspor ke STEP, saat di-mesh untuk
        /// 3D print, atau tidak ketahuan sama sekali.
        type BRepCheck_Analyzer;
        #[cxx_name = "construct_unique"]
        fn BRepCheck_Analyzer_ctor(
            shape: &TopoDS_Shape,
            geom_controls: bool,
        ) -> UniquePtr<BRepCheck_Analyzer>;
        #[cxx_name = "IsValid"]
        fn is_valid(self: &BRepCheck_Analyzer) -> bool;
    }
}
