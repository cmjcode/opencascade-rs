use thiserror::Error;

pub mod adhoc;
pub mod angle;
pub mod bounding_box;
pub mod kicad;
pub mod mesh;
pub mod primitives;
pub mod section;
pub mod workplane;

mod law_function;
mod make_pipe_shell;

#[derive(Error, Debug)]
pub enum Error {
    #[error("failed to write STL file")]
    StlWriteFailed,
    #[error("failed to read STEP file")]
    StepReadFailed,
    #[error("failed to read IGES file")]
    IgesReadFailed,
    #[error("failed to read KiCAD PCB file: {0}")]
    KicadReadFailed(#[from] opencascade_kicad_parser::Error),
    #[error("at least one shape is required to write a STEP file")]
    StepWriteNoShapes,
    #[error("failed to transfer shape to STEP writer")]
    StepWriteTransferFailed,
    #[error("failed to write STEP file")]
    StepWriteFailed,
    #[error("failed to write IGES file")]
    IgesWriteFailed,
    #[error("failed to read BREP file")]
    BrepReadFailed,
    #[error("failed to write BREP file")]
    BrepWriteFailed,
    #[error("failed to triangulate Shape")]
    TriangulationFailed,
    #[error("encountered a face with no triangulation")]
    UntriangulatedFace,
    #[error("at least 2 points are required for creating a wire")]
    NotEnoughPoints,
    #[error("BRepOffset_MakeOffset gagal: {0}")]
    OffsetOnFaceFailed(String),
    #[error("BRepFilletAPI_MakeFillet/MakeChamfer gagal: {0}")]
    FilletFailed(String),
    #[error("BRepAlgoAPI_Fuse/Cut/Common gagal: {0}")]
    BooleanOpFailed(String),
    #[error("BRepPrimAPI_MakeRevol gagal: {0}")]
    RevolveFailed(String),
    #[error("BRepOffsetAPI_MakeThickSolid gagal: {0}")]
    HollowFailed(String),
    #[error("BRepOffsetAPI_MakePipe gagal: {0}")]
    PipeFailed(String),
    #[error("BRepOffsetAPI_DraftAngle gagal: {0}")]
    DraftAngleFailed(String),
    #[error("BRepAlgoAPI_Splitter gagal: {0}")]
    SplitFailed(String),
}
