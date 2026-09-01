pub use inner::*;

#[cxx::bridge]
mod inner {
    unsafe extern "C++" {
        include!("opencascade-sys/include/b_rep_algo_api.hxx");

        type TopoDS_Shape = crate::topo_ds::TopoDS_Shape;
        type TopTools_ListOfShape = crate::top_tools::TopTools_ListOfShape;
        type Message_ProgressRange = crate::message::Message_ProgressRange;
        type BOPAlgo_GlueEnum = crate::bop_algo::BOPAlgo_GlueEnum;

        pub type BRepAlgoAPI_BuilderAlgo;
        pub fn SectionEdges(self: Pin<&mut BRepAlgoAPI_BuilderAlgo>) -> &TopTools_ListOfShape;

        pub type BRepAlgoAPI_Fuse;
        #[cxx_name = "construct_unique"]
        pub fn BRepAlgoAPI_Fuse_new(
            shape_1: &TopoDS_Shape,
            shape_2: &TopoDS_Shape,
        ) -> UniquePtr<BRepAlgoAPI_Fuse>;
        pub fn BRepAlgoAPI_Fuse_ctor_checked(
            shape_1: &TopoDS_Shape,
            shape_2: &TopoDS_Shape,
        ) -> Result<UniquePtr<BRepAlgoAPI_Fuse>>;
        pub fn Shape(self: Pin<&mut BRepAlgoAPI_Fuse>) -> &TopoDS_Shape;
        pub fn BRepAlgoAPI_Fuse_shape_checked(
            fuse_operation: Pin<&mut BRepAlgoAPI_Fuse>,
        ) -> Result<&TopoDS_Shape>;
        pub fn Build(self: Pin<&mut BRepAlgoAPI_Fuse>, progress: &Message_ProgressRange);
        pub fn IsDone(self: &BRepAlgoAPI_Fuse) -> bool;
        pub fn SectionEdges(self: Pin<&mut BRepAlgoAPI_Fuse>) -> &TopTools_ListOfShape;
        pub fn SetGlue(self: Pin<&mut BRepAlgoAPI_Fuse>, glue: BOPAlgo_GlueEnum);

        type BRepAlgoAPI_Cut;
        #[cxx_name = "construct_unique"]
        pub fn BRepAlgoAPI_Cut_new(
            shape_1: &TopoDS_Shape,
            shape_2: &TopoDS_Shape,
        ) -> UniquePtr<BRepAlgoAPI_Cut>;
        pub fn BRepAlgoAPI_Cut_ctor_checked(
            shape_1: &TopoDS_Shape,
            shape_2: &TopoDS_Shape,
        ) -> Result<UniquePtr<BRepAlgoAPI_Cut>>;
        pub fn Shape(self: Pin<&mut BRepAlgoAPI_Cut>) -> &TopoDS_Shape;
        pub fn BRepAlgoAPI_Cut_shape_checked(
            cut_operation: Pin<&mut BRepAlgoAPI_Cut>,
        ) -> Result<&TopoDS_Shape>;
        pub fn Build(self: Pin<&mut BRepAlgoAPI_Cut>, progress: &Message_ProgressRange);
        pub fn IsDone(self: &BRepAlgoAPI_Cut) -> bool;
        pub fn Generated<'a>(
            self: Pin<&'a mut BRepAlgoAPI_Cut>,
            shape: &'a TopoDS_Shape,
        ) -> &'a TopTools_ListOfShape;
        pub fn SectionEdges(self: Pin<&mut BRepAlgoAPI_Cut>) -> &TopTools_ListOfShape;

        type BRepAlgoAPI_Common;
        #[cxx_name = "construct_unique"]
        pub fn BRepAlgoAPI_Common_new(
            shape_1: &TopoDS_Shape,
            shape_2: &TopoDS_Shape,
        ) -> UniquePtr<BRepAlgoAPI_Common>;
        pub fn BRepAlgoAPI_Common_ctor_checked(
            shape_1: &TopoDS_Shape,
            shape_2: &TopoDS_Shape,
        ) -> Result<UniquePtr<BRepAlgoAPI_Common>>;
        pub fn Shape(self: Pin<&mut BRepAlgoAPI_Common>) -> &TopoDS_Shape;
        pub fn BRepAlgoAPI_Common_shape_checked(
            common_operation: Pin<&mut BRepAlgoAPI_Common>,
        ) -> Result<&TopoDS_Shape>;
        pub fn Build(self: Pin<&mut BRepAlgoAPI_Common>, progress: &Message_ProgressRange);
        pub fn IsDone(self: &BRepAlgoAPI_Common) -> bool;
        pub fn SectionEdges(self: Pin<&mut BRepAlgoAPI_Common>) -> &TopTools_ListOfShape;

        type BRepAlgoAPI_Section;
        #[cxx_name = "construct_unique"]
        pub fn BRepAlgoAPI_Section_new(
            shape_1: &TopoDS_Shape,
            shape_2: &TopoDS_Shape,
        ) -> UniquePtr<BRepAlgoAPI_Section>;
        pub fn Shape(self: Pin<&mut BRepAlgoAPI_Section>) -> &TopoDS_Shape;
        pub fn Build(self: Pin<&mut BRepAlgoAPI_Section>, progress: &Message_ProgressRange);
        pub fn IsDone(self: &BRepAlgoAPI_Section) -> bool;
        pub fn SectionEdges(self: Pin<&mut BRepAlgoAPI_Section>) -> &TopTools_ListOfShape;

        pub fn split_shape_with_plane(
            shape: &TopoDS_Shape,
            px: f64,
            py: f64,
            pz: f64,
            nx: f64,
            ny: f64,
            nz: f64,
        ) -> Result<UniquePtr<CxxVector<TopoDS_Shape>>>;

        pub fn split_shape_with_tool(
            shape: &TopoDS_Shape,
            tool_shape: &TopoDS_Shape,
        ) -> Result<UniquePtr<CxxVector<TopoDS_Shape>>>;

        pub fn split_faces_with_plane(
            shape: &TopoDS_Shape,
            px: f64,
            py: f64,
            pz: f64,
            nx: f64,
            ny: f64,
            nz: f64,
        ) -> Result<UniquePtr<TopoDS_Shape>>;

        pub fn section_shape_with_plane(
            shape: &TopoDS_Shape,
            px: f64,
            py: f64,
            pz: f64,
            nx: f64,
            ny: f64,
            nz: f64,
        ) -> Result<UniquePtr<CxxVector<TopoDS_Shape>>>;
    }
}
