#include <BRepAlgoAPI_BuilderAlgo.hxx>
#include <BRepAlgoAPI_Common.hxx>
#include <BRepAlgoAPI_Cut.hxx>
#include <BRepAlgoAPI_Fuse.hxx>
#include <BRepAlgoAPI_Section.hxx>
#include <BRepAlgoAPI_Splitter.hxx>
#include <BRepBndLib.hxx>
#include <BRepBuilderAPI_MakeFace.hxx>
#include <BRepBuilderAPI_MakeSolid.hxx>
#include <BRepBuilderAPI_Sewing.hxx>
#include <Bnd_Box.hxx>
#include <TopExp_Explorer.hxx>
#include <TopoDS.hxx>
#include <TopoDS_Face.hxx>
#include <TopoDS_Shape.hxx>
#include <TopoDS_Shell.hxx>
#include <TopoDS_Solid.hxx>
#include <bindings_common.hxx>
#include <cmath>
#include <gp_Dir.hxx>
#include <gp_Pln.hxx>
#include <gp_Pnt.hxx>
#include <gp_Vec.hxx>

inline std::unique_ptr<BRepAlgoAPI_Fuse> BRepAlgoAPI_Fuse_ctor_checked(const TopoDS_Shape &shape_1,
                                                                        const TopoDS_Shape &shape_2) {
  try {
    return std::unique_ptr<BRepAlgoAPI_Fuse>(new BRepAlgoAPI_Fuse(shape_1, shape_2));
  } catch (const Standard_Failure &failure) {
    rethrow_standard_failure_as_runtime_error(failure, "BRepAlgoAPI_Fuse: fuse gagal (geometri tidak valid)");
  }
}

inline const TopoDS_Shape &BRepAlgoAPI_Fuse_shape_checked(BRepAlgoAPI_Fuse &fuse_operation) {
  try {
    return fuse_operation.Shape();
  } catch (const Standard_Failure &failure) {
    rethrow_standard_failure_as_runtime_error(failure, "BRepAlgoAPI_Fuse::Shape() failed: not done");
  }
}

inline std::unique_ptr<BRepAlgoAPI_Cut> BRepAlgoAPI_Cut_ctor_checked(const TopoDS_Shape &shape_1,
                                                                      const TopoDS_Shape &shape_2) {
  try {
    return std::unique_ptr<BRepAlgoAPI_Cut>(new BRepAlgoAPI_Cut(shape_1, shape_2));
  } catch (const Standard_Failure &failure) {
    rethrow_standard_failure_as_runtime_error(failure, "BRepAlgoAPI_Cut: cut gagal (geometri tidak valid)");
  }
}

inline const TopoDS_Shape &BRepAlgoAPI_Cut_shape_checked(BRepAlgoAPI_Cut &cut_operation) {
  try {
    return cut_operation.Shape();
  } catch (const Standard_Failure &failure) {
    rethrow_standard_failure_as_runtime_error(failure, "BRepAlgoAPI_Cut::Shape() failed: not done");
  }
}

inline std::unique_ptr<BRepAlgoAPI_Common> BRepAlgoAPI_Common_ctor_checked(const TopoDS_Shape &shape_1,
                                                                            const TopoDS_Shape &shape_2) {
  try {
    return std::unique_ptr<BRepAlgoAPI_Common>(new BRepAlgoAPI_Common(shape_1, shape_2));
  } catch (const Standard_Failure &failure) {
    rethrow_standard_failure_as_runtime_error(failure, "BRepAlgoAPI_Common: intersect gagal (geometri tidak valid)");
  }
}

inline const TopoDS_Shape &BRepAlgoAPI_Common_shape_checked(BRepAlgoAPI_Common &common_operation) {
  try {
    return common_operation.Shape();
  } catch (const Standard_Failure &failure) {
    rethrow_standard_failure_as_runtime_error(failure, "BRepAlgoAPI_Common::Shape() failed: not done");
  }
}

inline std::unique_ptr<std::vector<TopoDS_Shape>> split_shape_with_plane(
    const TopoDS_Shape &shape,
    double px, double py, double pz,
    double nx, double ny, double nz
) {
  try {
    Bnd_Box box;
    BRepBndLib::Add(shape, box);
    Standard_Real xmin = -1000.0, ymin = -1000.0, zmin = -1000.0;
    Standard_Real xmax = 1000.0, ymax = 1000.0, zmax = 1000.0;
    if (!box.IsVoid()) {
      box.Get(xmin, ymin, zmin, xmax, ymax, zmax);
    }
    double dx = xmax - xmin;
    double dy = ymax - ymin;
    double dz = zmax - zmin;
    double diag = std::sqrt(dx * dx + dy * dy + dz * dz);
    double size = (diag > 1.0 ? diag * 4.0 : 5000.0);

    double cx = (xmin + xmax) * 0.5;
    double cy = (ymin + ymax) * 0.5;
    double cz = (zmin + zmax) * 0.5;

    gp_Dir dir(nx, ny, nz);
    gp_Pnt p0(px, py, pz);
    gp_Pnt center_3d(cx, cy, cz);
    gp_Vec to_center(p0, center_3d);
    double dist_along_normal = to_center.Dot(gp_Vec(dir));
    gp_Pnt proj_center = center_3d.Translated(-gp_Vec(dir) * dist_along_normal);

    gp_Pln centered_pln(proj_center, dir);

    BRepBuilderAPI_MakeFace mk_face(centered_pln, -size, size, -size, size);
    if (!mk_face.IsDone()) {
      throw std::runtime_error("Gagal membuat bidang pemotong (cutting plane)");
    }
    TopoDS_Face cut_face = mk_face.Face();

    TopTools_ListOfShape args;
    args.Append(shape);

    TopTools_ListOfShape tools;
    tools.Append(cut_face);

    BRepAlgoAPI_Splitter splitter;
    splitter.SetArguments(args);
    splitter.SetTools(tools);
    splitter.Build();

    if (!splitter.IsDone()) {
      throw std::runtime_error("BRepAlgoAPI_Splitter gagal memotong objek");
    }

    TopoDS_Shape res = splitter.Shape();
    std::unique_ptr<std::vector<TopoDS_Shape>> solids(new std::vector<TopoDS_Shape>());

    for (TopExp_Explorer exp(res, TopAbs_SOLID); exp.More(); exp.Next()) {
      solids->push_back(exp.Current());
    }

    if (solids->empty()) {
      for (TopExp_Explorer exp(res, TopAbs_SHELL); exp.More(); exp.Next()) {
        solids->push_back(exp.Current());
      }
    }
    if (solids->empty()) {
      for (TopExp_Explorer exp(res, TopAbs_FACE); exp.More(); exp.Next()) {
        solids->push_back(exp.Current());
      }
    }

    return solids;
  } catch (const Standard_Failure &failure) {
    rethrow_standard_failure_as_runtime_error(failure, "Split shape gagal (OCCT error)");
  }
}

inline std::unique_ptr<std::vector<TopoDS_Shape>> split_shape_with_tool(
    const TopoDS_Shape &shape,
    const TopoDS_Shape &tool_shape
) {
  try {
    TopTools_ListOfShape args;
    args.Append(shape);

    TopTools_ListOfShape tools;
    tools.Append(tool_shape);

    BRepAlgoAPI_Splitter splitter;
    splitter.SetArguments(args);
    splitter.SetTools(tools);
    splitter.Build();

    if (!splitter.IsDone()) {
      throw std::runtime_error("BRepAlgoAPI_Splitter gagal memotong objek dengan tool");
    }

    TopoDS_Shape res = splitter.Shape();
    std::unique_ptr<std::vector<TopoDS_Shape>> solids(new std::vector<TopoDS_Shape>());

    for (TopExp_Explorer exp(res, TopAbs_SOLID); exp.More(); exp.Next()) {
      solids->push_back(exp.Current());
    }

    if (solids->empty()) {
      for (TopExp_Explorer exp(res, TopAbs_SHELL); exp.More(); exp.Next()) {
        solids->push_back(exp.Current());
      }
    }
    if (solids->empty()) {
      for (TopExp_Explorer exp(res, TopAbs_FACE); exp.More(); exp.Next()) {
        solids->push_back(exp.Current());
      }
    }

    return solids;
  } catch (const Standard_Failure &failure) {
    rethrow_standard_failure_as_runtime_error(failure, "Split shape gagal (OCCT error)");
  }
}

inline std::unique_ptr<TopoDS_Shape> split_faces_with_plane(
    const TopoDS_Shape &shape,
    double px, double py, double pz,
    double nx, double ny, double nz
) {
  try {
    Bnd_Box box;
    BRepBndLib::Add(shape, box);
    Standard_Real xmin = -1000.0, ymin = -1000.0, zmin = -1000.0;
    Standard_Real xmax = 1000.0, ymax = 1000.0, zmax = 1000.0;
    if (!box.IsVoid()) {
      box.Get(xmin, ymin, zmin, xmax, ymax, zmax);
    }
    double dx = xmax - xmin;
    double dy = ymax - ymin;
    double dz = zmax - zmin;
    double diag = std::sqrt(dx * dx + dy * dy + dz * dz);
    double size = (diag > 1.0 ? diag * 4.0 : 5000.0);

    double cx = (xmin + xmax) * 0.5;
    double cy = (ymin + ymax) * 0.5;
    double cz = (zmin + zmax) * 0.5;

    gp_Dir dir(nx, ny, nz);
    gp_Pnt p0(px, py, pz);
    gp_Pnt center_3d(cx, cy, cz);
    gp_Vec to_center(p0, center_3d);
    double dist_along_normal = to_center.Dot(gp_Vec(dir));
    gp_Pnt proj_center = center_3d.Translated(-gp_Vec(dir) * dist_along_normal);

    gp_Pln centered_pln(proj_center, dir);

    BRepBuilderAPI_MakeFace mk_face(centered_pln, -size, size, -size, size);
    if (!mk_face.IsDone()) {
      throw std::runtime_error("Gagal membuat bidang pemotong (cutting plane)");
    }
    TopoDS_Face cut_face = mk_face.Face();

    TopTools_ListOfShape args;
    for (TopExp_Explorer exp(shape, TopAbs_FACE); exp.More(); exp.Next()) {
      args.Append(exp.Current());
    }

    TopTools_ListOfShape tools;
    tools.Append(cut_face);

    BRepAlgoAPI_Splitter splitter;
    splitter.SetArguments(args);
    splitter.SetTools(tools);
    splitter.Build();

    if (!splitter.IsDone()) {
      throw std::runtime_error("BRepAlgoAPI_Splitter gagal membagi face");
    }

    TopoDS_Shape res = splitter.Shape();

    BRepBuilderAPI_Sewing sewing(1.0e-5);
    sewing.Add(res);
    sewing.Perform();
    TopoDS_Shape sewed = sewing.SewedShape();

    for (TopExp_Explorer exp(sewed, TopAbs_SHELL); exp.More(); exp.Next()) {
      TopoDS_Shell shell = TopoDS::Shell(exp.Current());
      BRepBuilderAPI_MakeSolid mk_solid(shell);
      if (mk_solid.IsDone()) {
        return std::unique_ptr<TopoDS_Shape>(new TopoDS_Shape(mk_solid.Solid()));
      }
    }

    return std::unique_ptr<TopoDS_Shape>(new TopoDS_Shape(sewed));
  } catch (const Standard_Failure &failure) {
    rethrow_standard_failure_as_runtime_error(failure, "Split face gagal (OCCT error)");
  }
}

inline std::unique_ptr<std::vector<TopoDS_Shape>> section_shape_with_plane(
    const TopoDS_Shape &shape,
    double px, double py, double pz,
    double nx, double ny, double nz
) {
  try {
    double len = std::sqrt(nx * nx + ny * ny + nz * nz);
    if (len < 1e-7) {
      throw std::runtime_error("Normal bidang potong tidak valid (panjang 0)");
    }
    gp_Dir dir(nx / len, ny / len, nz / len);
    gp_Pnt p0(px, py, pz);
    gp_Pln pln(p0, dir);

    BRepAlgoAPI_Section section_op(shape, pln, Standard_True);
    section_op.Build();
    if (!section_op.IsDone()) {
      throw std::runtime_error("BRepAlgoAPI_Section gagal menghitung irisan bidang potong");
    }

    TopoDS_Shape res = section_op.Shape();
    std::unique_ptr<std::vector<TopoDS_Shape>> edges(new std::vector<TopoDS_Shape>());

    for (TopExp_Explorer exp(res, TopAbs_EDGE); exp.More(); exp.Next()) {
      edges->push_back(exp.Current());
    }

    return edges;
  } catch (const Standard_Failure &failure) {
    rethrow_standard_failure_as_runtime_error(failure, "BRepAlgoAPI_Section gagal (OCCT error)");
  }
}
