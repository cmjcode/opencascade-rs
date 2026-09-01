#include <BRepFilletAPI_MakeChamfer.hxx>
#include <BRepFilletAPI_MakeFillet.hxx>
#include <BRepFilletAPI_MakeFillet2d.hxx>
#include <bindings_common.hxx>

inline std::unique_ptr<TopoDS_Edge> BRepFilletAPI_MakeFillet2d_add_fillet(BRepFilletAPI_MakeFillet2d &make_fillet,
                                                                          const TopoDS_Vertex &vertex,
                                                                          Standard_Real radius) {
  return std::unique_ptr<TopoDS_Edge>(new TopoDS_Edge(make_fillet.AddFillet(vertex, radius)));
}

// Chamfers
inline std::unique_ptr<TopoDS_Edge>
BRepFilletAPI_MakeFillet2d_add_chamfer(BRepFilletAPI_MakeFillet2d &make_fillet, const TopoDS_Edge &edge1,
                                       const TopoDS_Edge &edge2, const Standard_Real dist1, const Standard_Real dist2) {
  return std::unique_ptr<TopoDS_Edge>(new TopoDS_Edge(make_fillet.AddChamfer(edge1, edge2, dist1, dist2)));
}

inline std::unique_ptr<TopoDS_Edge>
BRepFilletAPI_MakeFillet2d_add_chamfer_angle(BRepFilletAPI_MakeFillet2d &make_fillet, const TopoDS_Edge &edge,
                                             const TopoDS_Vertex &vertex, const Standard_Real dist,
                                             const Standard_Real angle) {
  return std::unique_ptr<TopoDS_Edge>(new TopoDS_Edge(make_fillet.AddChamfer(edge, vertex, dist, angle)));
}

inline const TopoDS_Shape &BRepFilletAPI_MakeFillet_shape_checked(BRepFilletAPI_MakeFillet &make_fillet) {
  try {
    return make_fillet.Shape();
  } catch (const Standard_Failure &failure) {
    rethrow_standard_failure_as_runtime_error(failure, "BRepFilletAPI_MakeFillet::Shape() failed: not done");
  }
}

inline const TopoDS_Shape &BRepFilletAPI_MakeChamfer_shape_checked(BRepFilletAPI_MakeChamfer &make_chamfer) {
  try {
    return make_chamfer.Shape();
  } catch (const Standard_Failure &failure) {
    rethrow_standard_failure_as_runtime_error(failure, "BRepFilletAPI_MakeChamfer::Shape() failed: not done");
  }
}
