#include <BRepPrimAPI_MakeBox.hxx>
#include <BRepPrimAPI_MakeCone.hxx>
#include <BRepPrimAPI_MakeCylinder.hxx>
#include <BRepPrimAPI_MakePrism.hxx>
#include <BRepPrimAPI_MakeRevol.hxx>
#include <BRepPrimAPI_MakeSphere.hxx>
#include <BRepPrimAPI_MakeTorus.hxx>
#include <bindings_common.hxx>

inline std::unique_ptr<BRepPrimAPI_MakeRevol> BRepPrimAPI_MakeRevol_ctor_checked(const TopoDS_Shape &shape,
                                                                                  const gp_Ax1 &axis,
                                                                                  double angle,
                                                                                  bool copy) {
  try {
    return std::unique_ptr<BRepPrimAPI_MakeRevol>(new BRepPrimAPI_MakeRevol(shape, axis, angle, copy));
  } catch (const Standard_Failure &failure) {
    rethrow_standard_failure_as_runtime_error(failure, "BRepPrimAPI_MakeRevol: konstruksi revolve gagal (sumbu atau geometri profil tidak valid)");
  }
}

inline const TopoDS_Shape &BRepPrimAPI_MakeRevol_shape_checked(BRepPrimAPI_MakeRevol &make_revol) {
  try {
    if (!make_revol.IsDone()) {
      throw std::runtime_error("BRepPrimAPI_MakeRevol::Shape() gagal: operasi revolve tidak selesai (sumbu memotong profil atau profil tidak tertutup)");
    }
    return make_revol.Shape();
  } catch (const Standard_Failure &failure) {
    rethrow_standard_failure_as_runtime_error(failure, "BRepPrimAPI_MakeRevol::Shape() gagal: not done");
  }
}
