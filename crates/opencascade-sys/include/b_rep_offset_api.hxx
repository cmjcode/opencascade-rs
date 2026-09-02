#include <BRepOffsetAPI_DraftAngle.hxx>
#include <BRepOffsetAPI_MakeOffset.hxx>
#include <BRepOffsetAPI_MakePipe.hxx>
#include <BRepOffsetAPI_MakePipeShell.hxx>
#include <BRepOffsetAPI_MakeThickSolid.hxx>
#include <BRepOffsetAPI_ThruSections.hxx>
#include <BRepOffset_MakeOffset.hxx>
#include <Law_Function.hxx>
#include <TopTools_ListOfShape.hxx>
#include <TopoDS_Face.hxx>
#include <TopoDS_Shape.hxx>
#include <TopoDS_Wire.hxx>
#include <bindings_common.hxx>
#include <gp_Dir.hxx>
#include <gp_Pln.hxx>
#include <gp_Pnt.hxx>

inline std::unique_ptr<BRepOffset_MakeOffset> BRepOffset_MakeOffset_ctor() {
  return std::unique_ptr<BRepOffset_MakeOffset>(new BRepOffset_MakeOffset());
}

inline void BRepOffset_MakeOffset_Initialize(BRepOffset_MakeOffset &make_offset,
                                             const TopoDS_Shape &shape, Standard_Real offset,
                                             Standard_Real tolerance, BRepOffset_Mode mode,
                                             bool intersection, bool self_inter, GeomAbs_JoinType join,
                                             bool thickening, bool remove_int_edges) {
  try {
    make_offset.Initialize(shape, offset, tolerance, mode, intersection, self_inter, join, thickening,
                           remove_int_edges);
  } catch (const Standard_Failure &failure) {
    rethrow_standard_failure_as_runtime_error(failure, "BRepOffset_MakeOffset::Initialize() failed");
  }
}

inline void BRepOffset_MakeOffset_AddFace(BRepOffset_MakeOffset &make_offset,
                                          const TopoDS_Face &face) {
  try {
    make_offset.AddFace(face);
  } catch (const Standard_Failure &failure) {
    rethrow_standard_failure_as_runtime_error(failure, "BRepOffset_MakeOffset::AddFace() failed");
  }
}

inline void BRepOffset_MakeOffset_SetOffsetOnFace(BRepOffset_MakeOffset &make_offset,
                                                   const TopoDS_Face &face, Standard_Real offset) {
  try {
    make_offset.SetOffsetOnFace(face, offset);
  } catch (const Standard_Failure &failure) {
    rethrow_standard_failure_as_runtime_error(failure, "BRepOffset_MakeOffset::SetOffsetOnFace() failed");
  }
}

inline void BRepOffset_MakeOffset_MakeOffsetShape(BRepOffset_MakeOffset &make_offset) {
  try {
    make_offset.MakeOffsetShape();
  } catch (const Standard_Failure &failure) {
    rethrow_standard_failure_as_runtime_error(failure, "BRepOffset_MakeOffset::MakeOffsetShape() failed");
  }
}

inline const TopoDS_Shape &BRepOffset_MakeOffset_Shape(const BRepOffset_MakeOffset &make_offset) {
  try {
    return make_offset.Shape();
  } catch (const Standard_Failure &failure) {
    rethrow_standard_failure_as_runtime_error(failure, "BRepOffset_MakeOffset::Shape() failed: not done");
  }
}

inline const TopoDS_Shape &BRepOffsetAPI_MakeThickSolid_shape_checked(BRepOffsetAPI_MakeThickSolid &make_thick_solid) {
  try {
    if (!make_thick_solid.IsDone()) {
      throw std::runtime_error("BRepOffsetAPI_MakeThickSolid::Shape() failed: not done");
    }
    return make_thick_solid.Shape();
  } catch (const Standard_Failure &failure) {
    rethrow_standard_failure_as_runtime_error(failure, "BRepOffsetAPI_MakeThickSolid::Shape() failed: not done");
  }
}

inline std::unique_ptr<BRepOffsetAPI_MakePipe> BRepOffsetAPI_MakePipe_ctor_checked(const TopoDS_Wire &spine,
                                                                                    const TopoDS_Shape &profile) {
  try {
    return std::unique_ptr<BRepOffsetAPI_MakePipe>(new BRepOffsetAPI_MakePipe(spine, profile));
  } catch (const Standard_Failure &failure) {
    rethrow_standard_failure_as_runtime_error(failure, "BRepOffsetAPI_MakePipe: konstruksi sweep gagal (profil atau jalur kurva tidak valid)");
  }
}

inline const TopoDS_Shape &BRepOffsetAPI_MakePipe_shape_checked(BRepOffsetAPI_MakePipe &make_pipe) {
  try {
    if (!make_pipe.IsDone()) {
      throw std::runtime_error("BRepOffsetAPI_MakePipe::Shape() gagal: operasi sweep tidak selesai (jalur kurva atau profil bermasalah)");
    }
    return make_pipe.Shape();
  } catch (const Standard_Failure &failure) {
    rethrow_standard_failure_as_runtime_error(failure, "BRepOffsetAPI_MakePipe::Shape() gagal: not done");
  }
}

inline std::unique_ptr<BRepOffsetAPI_DraftAngle> BRepOffsetAPI_DraftAngle_ctor(const TopoDS_Shape &shape) {
  try {
    return std::unique_ptr<BRepOffsetAPI_DraftAngle>(new BRepOffsetAPI_DraftAngle(shape));
  } catch (const Standard_Failure &failure) {
    rethrow_standard_failure_as_runtime_error(failure, "BRepOffsetAPI_DraftAngle: konstruksi gagal (shape tidak valid)");
  }
}

inline void BRepOffsetAPI_DraftAngle_Add(BRepOffsetAPI_DraftAngle &draft,
                                          const TopoDS_Face &face,
                                          const gp_Dir &pull_dir,
                                          Standard_Real angle_rad,
                                          const gp_Pln &neutral_plane) {
  try {
    draft.Add(face, pull_dir, angle_rad, neutral_plane);
  } catch (const Standard_Failure &failure) {
    rethrow_standard_failure_as_runtime_error(failure, "BRepOffsetAPI_DraftAngle::Add() gagal (face bukan planar atau bidang netral tidak valid)");
  }
}

inline void BRepOffsetAPI_DraftAngle_Build(BRepOffsetAPI_DraftAngle &draft) {
  try {
    draft.Build();
  } catch (const Standard_Failure &failure) {
    rethrow_standard_failure_as_runtime_error(failure, "BRepOffsetAPI_DraftAngle::Build() gagal (sudut terlalu besar atau geometri face tidak kompatibel)");
  }
}

inline bool BRepOffsetAPI_DraftAngle_IsDone(const BRepOffsetAPI_DraftAngle &draft) {
  return draft.IsDone();
}

inline const TopoDS_Shape &BRepOffsetAPI_DraftAngle_shape_checked(BRepOffsetAPI_DraftAngle &draft) {
  try {
    if (!draft.IsDone()) {
      throw std::runtime_error("BRepOffsetAPI_DraftAngle::Shape() gagal: operasi draft tidak selesai");
    }
    return draft.Shape();
  } catch (const Standard_Failure &failure) {
    rethrow_standard_failure_as_runtime_error(failure, "BRepOffsetAPI_DraftAngle::Shape() gagal: not done");
  }
}

inline void BRepOffsetAPI_MakePipeShell_SetLaw(BRepOffsetAPI_MakePipeShell &shell,
                                              const TopoDS_Shape &profile,
                                              const Handle_Law_Function &law,
                                              bool with_contact,
                                              bool with_correction) {
  Handle(Law_Function) l = law;
  shell.SetLaw(profile, l, with_contact, with_correction);
}
