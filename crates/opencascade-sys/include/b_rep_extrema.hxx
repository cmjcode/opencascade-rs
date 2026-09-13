#include <BRepExtrema_DistShapeShape.hxx>
#include <bindings_common.hxx>
#include <gp_Pnt.hxx>

// Fasad tipis: konstruktor OCCT langsung menghitung dan bisa melempar
// untuk geometri patologis. Kegagalan dilaporkan lewat `is_done`, tidak
// pernah menembus batas FFI.
class DucadDistance {
public:
  DucadDistance(const TopoDS_Shape &a, const TopoDS_Shape &b) : done(false), min_distance(0.0) {
    try {
      CoutSilencer silencer;
      BRepExtrema_DistShapeShape calc(a, b);
      if (calc.IsDone() && calc.NbSolution() > 0) {
        done = true;
        min_distance = calc.Value();
        p1 = calc.PointOnShape1(1);
        p2 = calc.PointOnShape2(1);
      }
    } catch (const Standard_Failure &) {
      done = false;
    }
  }

  bool is_done() const { return done; }
  double value() const { return min_distance; }
  double p1x() const { return p1.X(); }
  double p1y() const { return p1.Y(); }
  double p1z() const { return p1.Z(); }
  double p2x() const { return p2.X(); }
  double p2y() const { return p2.Y(); }
  double p2z() const { return p2.Z(); }

private:
  bool done;
  double min_distance;
  gp_Pnt p1;
  gp_Pnt p2;
};

inline std::unique_ptr<DucadDistance> DucadDistance_ctor(const TopoDS_Shape &a,
                                                          const TopoDS_Shape &b) {
  return std::unique_ptr<DucadDistance>(new DucadDistance(a, b));
}
