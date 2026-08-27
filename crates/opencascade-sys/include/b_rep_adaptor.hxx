#include <BRepAdaptor_Curve.hxx>
#include <BRepAdaptor_Surface.hxx>
#include <bindings_common.hxx>
#include <gp_Cone.hxx>
#include <gp_Cylinder.hxx>
#include <gp_Pnt.hxx>
#include <gp_Sphere.hxx>

inline std::unique_ptr<gp_Pnt> BRepAdaptor_Curve_value(const BRepAdaptor_Curve &curve, const Standard_Real U) {
  return std::unique_ptr<gp_Pnt>(new gp_Pnt(curve.Value(U)));
}

inline std::unique_ptr<BRepAdaptor_Surface> BRepAdaptor_Surface_ctor(const TopoDS_Face &face,
                                                                      bool restriction) {
  return std::unique_ptr<BRepAdaptor_Surface>(new BRepAdaptor_Surface(face, restriction));
}

inline std::unique_ptr<gp_Cylinder> BRepAdaptor_Surface_cylinder(const BRepAdaptor_Surface &surface) {
  try {
    return std::unique_ptr<gp_Cylinder>(new gp_Cylinder(surface.Cylinder()));
  } catch (const Standard_Failure &failure) {
    rethrow_standard_failure_as_runtime_error(failure, "BRepAdaptor_Surface::Cylinder() failed: surface is not a cylinder");
  }
}

inline std::unique_ptr<gp_Cone> BRepAdaptor_Surface_cone(const BRepAdaptor_Surface &surface) {
  try {
    return std::unique_ptr<gp_Cone>(new gp_Cone(surface.Cone()));
  } catch (const Standard_Failure &failure) {
    rethrow_standard_failure_as_runtime_error(failure, "BRepAdaptor_Surface::Cone() failed: surface is not a cone");
  }
}

inline std::unique_ptr<gp_Sphere> BRepAdaptor_Surface_sphere(const BRepAdaptor_Surface &surface) {
  try {
    return std::unique_ptr<gp_Sphere>(new gp_Sphere(surface.Sphere()));
  } catch (const Standard_Failure &failure) {
    rethrow_standard_failure_as_runtime_error(failure, "BRepAdaptor_Surface::Sphere() failed: surface is not a sphere");
  }
}
