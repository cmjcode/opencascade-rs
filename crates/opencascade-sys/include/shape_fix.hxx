#include <ShapeFix_Shape.hxx>
#include <bindings_common.hxx>

inline std::unique_ptr<ShapeFix_Shape> ShapeFix_Shape_ctor(const TopoDS_Shape &shape) {
  return std::unique_ptr<ShapeFix_Shape>(new ShapeFix_Shape(shape));
}

inline bool ShapeFix_Shape_perform(ShapeFix_Shape &fixer) {
  return fixer.Perform();
}

inline std::unique_ptr<TopoDS_Shape> ShapeFix_Shape_result(const ShapeFix_Shape &fixer) {
  return std::unique_ptr<TopoDS_Shape>(new TopoDS_Shape(fixer.Shape()));
}
