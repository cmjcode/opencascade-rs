pub use inner::*;

#[cxx::bridge]
mod inner {
    #[derive(Debug)]
    #[repr(u32)]
    pub enum BRepOffset_Mode {
        BRepOffset_Skin,
        BRepOffset_Pipe,
        BRepOffset_RectoVerso,
    }

    unsafe extern "C++" {
        include!("opencascade-sys/include/b_rep_offset_api.hxx");

        type TopoDS_Shape = crate::topo_ds::TopoDS_Shape;
        type TopoDS_Face = crate::topo_ds::TopoDS_Face;
        type TopoDS_Wire = crate::topo_ds::TopoDS_Wire;
        type GeomAbs_JoinType = crate::geom_abs::GeomAbs_JoinType;
        type TopTools_ListOfShape = crate::top_tools::TopTools_ListOfShape;
        type Message_ProgressRange = crate::message::Message_ProgressRange;
        type Handle_Law_Function = crate::law::Handle_Law_Function;
        type gp_Pnt = crate::gp::gp_Pnt;
        type gp_Dir = crate::gp::gp_Dir;
        type gp_Pln = crate::gp::gp_Pln;

        type BRepOffset_Mode;

        type BRepOffset_MakeOffset;
        pub fn BRepOffset_MakeOffset_ctor() -> UniquePtr<BRepOffset_MakeOffset>;
        #[allow(clippy::too_many_arguments)]
        pub fn BRepOffset_MakeOffset_Initialize(
            make_offset: Pin<&mut BRepOffset_MakeOffset>,
            shape: &TopoDS_Shape,
            offset: f64,
            tolerance: f64,
            mode: BRepOffset_Mode,
            intersection: bool,
            self_inter: bool,
            join: GeomAbs_JoinType,
            thickening: bool,
            remove_int_edges: bool,
        ) -> Result<()>;
        pub fn BRepOffset_MakeOffset_AddFace(
            make_offset: Pin<&mut BRepOffset_MakeOffset>,
            face: &TopoDS_Face,
        ) -> Result<()>;
        pub fn BRepOffset_MakeOffset_SetOffsetOnFace(
            make_offset: Pin<&mut BRepOffset_MakeOffset>,
            face: &TopoDS_Face,
            offset: f64,
        ) -> Result<()>;
        pub fn BRepOffset_MakeOffset_MakeOffsetShape(
            make_offset: Pin<&mut BRepOffset_MakeOffset>,
        ) -> Result<()>;
        pub fn BRepOffset_MakeOffset_Shape(
            make_offset: &BRepOffset_MakeOffset,
        ) -> Result<&TopoDS_Shape>;

        type BRepOffsetAPI_MakeOffset;
        #[cxx_name = "construct_unique"]
        pub fn BRepOffsetAPI_MakeOffset_face_new(
            face: &TopoDS_Face,
            join: GeomAbs_JoinType,
        ) -> UniquePtr<BRepOffsetAPI_MakeOffset>;
        #[cxx_name = "construct_unique"]
        pub fn BRepOffsetAPI_MakeOffset_wire_new(
            wire: &TopoDS_Wire,
            join: GeomAbs_JoinType,
        ) -> UniquePtr<BRepOffsetAPI_MakeOffset>;
        pub fn Perform(self: Pin<&mut BRepOffsetAPI_MakeOffset>, offset: f64, alt: f64);
        pub fn Shape(self: Pin<&mut BRepOffsetAPI_MakeOffset>) -> &TopoDS_Shape;
        pub fn Build(self: Pin<&mut BRepOffsetAPI_MakeOffset>, progress: &Message_ProgressRange);
        pub fn IsDone(self: &BRepOffsetAPI_MakeOffset) -> bool;

        type BRepOffsetAPI_MakeThickSolid;
        #[cxx_name = "construct_unique"]
        pub fn BRepOffsetAPI_MakeThickSolid_new() -> UniquePtr<BRepOffsetAPI_MakeThickSolid>;
        #[allow(clippy::too_many_arguments)]
        pub fn MakeThickSolidByJoin(
            self: Pin<&mut BRepOffsetAPI_MakeThickSolid>,
            shape: &TopoDS_Shape,
            closing_faces: &TopTools_ListOfShape,
            offset: f64,
            tolerance: f64,
            offset_mode: BRepOffset_Mode,
            intersection: bool,
            self_intersection: bool,
            join_type: GeomAbs_JoinType,
            remove_intersecting_edges: bool,
            progress: &Message_ProgressRange,
        );
        pub fn Shape(self: Pin<&mut BRepOffsetAPI_MakeThickSolid>) -> &TopoDS_Shape;
        pub fn BRepOffsetAPI_MakeThickSolid_shape_checked(
            make_thick_solid: Pin<&mut BRepOffsetAPI_MakeThickSolid>,
        ) -> Result<&TopoDS_Shape>;
        pub fn Build(
            self: Pin<&mut BRepOffsetAPI_MakeThickSolid>,
            progress: &Message_ProgressRange,
        );
        pub fn IsDone(self: &BRepOffsetAPI_MakeThickSolid) -> bool;

        type BRepOffsetAPI_MakePipe;
        #[cxx_name = "construct_unique"]
        pub fn BRepOffsetAPI_MakePipe_new(
            spine: &TopoDS_Wire,
            profile: &TopoDS_Shape,
        ) -> UniquePtr<BRepOffsetAPI_MakePipe>;
        pub fn BRepOffsetAPI_MakePipe_ctor_checked(
            spine: &TopoDS_Wire,
            profile: &TopoDS_Shape,
        ) -> Result<UniquePtr<BRepOffsetAPI_MakePipe>>;
        pub fn Shape(self: Pin<&mut BRepOffsetAPI_MakePipe>) -> &TopoDS_Shape;
        pub fn BRepOffsetAPI_MakePipe_shape_checked(
            make_pipe: Pin<&mut BRepOffsetAPI_MakePipe>,
        ) -> Result<&TopoDS_Shape>;

        type BRepOffsetAPI_MakePipeShell;
        #[cxx_name = "construct_unique"]
        pub fn BRepOffsetAPI_MakePipeShell_new(
            spine: &TopoDS_Wire,
        ) -> UniquePtr<BRepOffsetAPI_MakePipeShell>;
        pub fn SetMode(self: Pin<&mut BRepOffsetAPI_MakePipeShell>, is_frenet: bool);
        pub fn Add(
            self: Pin<&mut BRepOffsetAPI_MakePipeShell>,
            profile: &TopoDS_Shape,
            with_contact: bool,
            with_correction: bool,
        );
        pub fn SetLaw(
            self: Pin<&mut BRepOffsetAPI_MakePipeShell>,
            profile: &TopoDS_Shape,
            law: &Handle_Law_Function,
            with_contact: bool,
            with_correction: bool,
        );
        pub fn Build(self: Pin<&mut BRepOffsetAPI_MakePipeShell>, progress: &Message_ProgressRange);
        pub fn MakeSolid(self: Pin<&mut BRepOffsetAPI_MakePipeShell>) -> bool;
        pub fn Shape(self: Pin<&mut BRepOffsetAPI_MakePipeShell>) -> &TopoDS_Shape;

        type BRepOffsetAPI_ThruSections;
        #[cxx_name = "construct_unique"]
        pub fn BRepOffsetAPI_ThruSections_new(
            is_solid: bool,
        ) -> UniquePtr<BRepOffsetAPI_ThruSections>;
        pub fn AddWire(self: Pin<&mut BRepOffsetAPI_ThruSections>, wire: &TopoDS_Wire);
        pub fn CheckCompatibility(self: Pin<&mut BRepOffsetAPI_ThruSections>, check: bool);
        pub fn Shape(self: Pin<&mut BRepOffsetAPI_ThruSections>) -> &TopoDS_Shape;
        pub fn Build(self: Pin<&mut BRepOffsetAPI_ThruSections>, progress: &Message_ProgressRange);
        pub fn IsDone(self: &BRepOffsetAPI_ThruSections) -> bool;

        type BRepOffsetAPI_DraftAngle;
        pub fn BRepOffsetAPI_DraftAngle_ctor(
            shape: &TopoDS_Shape,
        ) -> Result<UniquePtr<BRepOffsetAPI_DraftAngle>>;
        pub fn BRepOffsetAPI_DraftAngle_Add(
            draft: Pin<&mut BRepOffsetAPI_DraftAngle>,
            face: &TopoDS_Face,
            pull_dir: &gp_Dir,
            angle_rad: f64,
            neutral_plane: &gp_Pln,
        ) -> Result<()>;
        pub fn BRepOffsetAPI_DraftAngle_Build(
            draft: Pin<&mut BRepOffsetAPI_DraftAngle>,
        ) -> Result<()>;
        pub fn BRepOffsetAPI_DraftAngle_IsDone(draft: &BRepOffsetAPI_DraftAngle) -> bool;
        pub fn BRepOffsetAPI_DraftAngle_shape_checked(
            draft: Pin<&mut BRepOffsetAPI_DraftAngle>,
        ) -> Result<&TopoDS_Shape>;
    }
}
