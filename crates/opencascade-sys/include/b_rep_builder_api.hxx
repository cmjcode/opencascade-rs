#include <BRepBuilderAPI_GTransform.hxx>
#include <BRepBuilderAPI_MakeEdge.hxx>
#include <BRepBuilderAPI_MakeFace.hxx>
#include <BRepBuilderAPI_MakeShapeOnMesh.hxx>
#include <BRepBuilderAPI_MakeSolid.hxx>
#include <BRepBuilderAPI_MakeVertex.hxx>
#include <BRepBuilderAPI_MakeWire.hxx>
#include <BRepBuilderAPI_Transform.hxx>
#include <Geom2d_Curve.hxx>
#include <Geom_Curve.hxx>
#include <Geom_Surface.hxx>
#include <bindings_common.hxx>
#include <gp_Elips.hxx>
#include <gp_Pln.hxx>

inline std::unique_ptr<BRepBuilderAPI_MakeEdge> BRepBuilderAPI_MakeEdge_elips(const gp_Elips &elips) {
  return std::unique_ptr<BRepBuilderAPI_MakeEdge>(new BRepBuilderAPI_MakeEdge(elips));
}

inline std::unique_ptr<BRepBuilderAPI_MakeFace> BRepBuilderAPI_MakeFace_pln(const gp_Pln &pln, double umin, double umax, double vmin, double vmax) {
  return std::unique_ptr<BRepBuilderAPI_MakeFace>(new BRepBuilderAPI_MakeFace(pln, umin, umax, vmin, vmax));
}
