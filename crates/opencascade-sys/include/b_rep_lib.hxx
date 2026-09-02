#include <BRepLib.hxx>
#include <BRepLib_ToolTriangulatedShape.hxx>
#include <Poly_Triangulation.hxx>
#include <TopoDS_Shape.hxx>
#include <bindings_common.hxx>

inline void BRepLib_ToolTriangulatedShape_ComputeNormals(const TopoDS_Face &face,
                                                        const Handle_Poly_Triangulation &triangulation) {
  BRepLib_ToolTriangulatedShape::ComputeNormals(face, triangulation);
}
