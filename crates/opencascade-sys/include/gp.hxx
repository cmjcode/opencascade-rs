#include <bindings_common.hxx>
#include <gp.hxx>
#include <gp_Ax2.hxx>
#include <gp_Ax3.hxx>
#include <gp_Circ.hxx>
#include <gp_Cone.hxx>
#include <gp_Cylinder.hxx>
#include <gp_Elips.hxx>
#include <gp_GTrsf.hxx>
#include <gp_Lin.hxx>
#include <gp_Pln.hxx>
#include <gp_Pnt.hxx>
#include <gp_Trsf.hxx>
#include <gp_Vec.hxx>

inline std::unique_ptr<gp_Ax2> gp_Ax2_new_with_x_dir(const gp_Pnt &origin, const gp_Dir &main_dir, const gp_Dir &x_dir) {
  return std::unique_ptr<gp_Ax2>(new gp_Ax2(origin, main_dir, x_dir));
}

inline std::unique_ptr<gp_Pln> gp_Pln_ctor_point_and_dir(const gp_Pnt &point, const gp_Dir &dir) {
  return std::unique_ptr<gp_Pln>(new gp_Pln(point, dir));
}

inline std::unique_ptr<gp_Elips> gp_Elips_ctor(const gp_Ax2 &axis, Standard_Real major_radius, Standard_Real minor_radius) {
  return std::unique_ptr<gp_Elips>(new gp_Elips(axis, major_radius, minor_radius));
}

inline std::unique_ptr<gp_Pnt> gp_Cylinder_location(const gp_Cylinder &cylinder) {
  return std::unique_ptr<gp_Pnt>(new gp_Pnt(cylinder.Location()));
}

inline std::unique_ptr<gp_Dir> gp_Cylinder_direction(const gp_Cylinder &cylinder) {
  return std::unique_ptr<gp_Dir>(new gp_Dir(cylinder.Axis().Direction()));
}

inline double gp_Cylinder_radius(const gp_Cylinder &cylinder) { return cylinder.Radius(); }

inline std::unique_ptr<gp_Pnt> gp_Cone_location(const gp_Cone &cone) {
  return std::unique_ptr<gp_Pnt>(new gp_Pnt(cone.Location()));
}

inline std::unique_ptr<gp_Dir> gp_Cone_direction(const gp_Cone &cone) {
  return std::unique_ptr<gp_Dir>(new gp_Dir(cone.Axis().Direction()));
}

inline double gp_Cone_radius(const gp_Cone &cone) { return cone.RefRadius(); }

inline double gp_Cone_semi_angle(const gp_Cone &cone) { return cone.SemiAngle(); }

#include <gp_Sphere.hxx>

inline std::unique_ptr<gp_Pnt> gp_Sphere_location(const gp_Sphere &sphere) {
  return std::unique_ptr<gp_Pnt>(new gp_Pnt(sphere.Location()));
}

inline double gp_Sphere_radius(const gp_Sphere &sphere) { return sphere.Radius(); }
