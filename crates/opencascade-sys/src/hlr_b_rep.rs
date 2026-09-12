pub use inner::*;

#[cxx::bridge]
mod inner {
    unsafe extern "C++" {
        include!("opencascade-sys/include/hlr_b_rep.hxx");

        type TopoDS_Shape = crate::topo_ds::TopoDS_Shape;

        /// Hidden Line Removal EKSAK (`HLRBRep_Algo`).
        ///
        /// Berbeda dari HLR berbasis mesh, hasilnya berupa kurva B-rep
        /// sungguhan: lingkaran tetap lingkaran, bukan poligon 64 sisi, dan
        /// garis tersembunyi ditentukan dari topologi — bukan dari uji
        /// oklusi terhadap segitiga hasil tesselasi.
        type DucadHlrSession;
        #[cxx_name = "construct_unique"]
        fn DucadHlrSession_ctor() -> UniquePtr<DucadHlrSession>;
        fn add_shape(self: Pin<&mut DucadHlrSession>, shape: &TopoDS_Shape);
        fn set_projection(
            self: Pin<&mut DucadHlrSession>,
            dx: f64,
            dy: f64,
            dz: f64,
            ux: f64,
            uy: f64,
            uz: f64,
        ) -> bool;
        fn compute(self: Pin<&mut DucadHlrSession>) -> bool;
        /// Rusuk tajam yang terlihat.
        fn visible_sharp(self: &DucadHlrSession) -> UniquePtr<TopoDS_Shape>;
        /// Siluet permukaan lengkung yang terlihat.
        fn visible_outline(self: &DucadHlrSession) -> UniquePtr<TopoDS_Shape>;
        /// Rusuk tajam yang tersembunyi (digambar putus-putus).
        fn hidden_sharp(self: &DucadHlrSession) -> UniquePtr<TopoDS_Shape>;
        /// Siluet yang tersembunyi.
        fn hidden_outline(self: &DucadHlrSession) -> UniquePtr<TopoDS_Shape>;
    }
}
