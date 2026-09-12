#include <BRepCheck_Analyzer.hxx>
#include <bindings_common.hxx>

inline std::unique_ptr<BRepCheck_Analyzer> BRepCheck_Analyzer_ctor(const TopoDS_Shape &shape,
                                                                   const Standard_Boolean geom_controls) {
  return std::unique_ptr<BRepCheck_Analyzer>(new BRepCheck_Analyzer(shape, geom_controls));
}
