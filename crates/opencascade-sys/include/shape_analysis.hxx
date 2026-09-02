#include <ShapeAnalysis.hxx>
#include <ShapeAnalysis_FreeBounds.hxx>
#include <TopTools_HSequenceOfShape.hxx>
#include <bindings_common.hxx>

inline void ShapeAnalysis_FreeBounds_ConnectEdgesToWires(Handle_TopTools_HSequenceOfShape &edges,
                                                        const Standard_Real toler,
                                                        const Standard_Boolean shared,
                                                        Handle_TopTools_HSequenceOfShape &wires) {
  Handle(TopTools_HSequenceOfShape) edges_h = edges;
  Handle(TopTools_HSequenceOfShape) wires_h = wires;
  ShapeAnalysis_FreeBounds::ConnectEdgesToWires(edges_h, toler, shared, wires_h);
  edges = edges_h;
  wires = wires_h;
}
