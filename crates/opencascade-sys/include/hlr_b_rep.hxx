#include <HLRAlgo_Projector.hxx>
#include <HLRBRep_Algo.hxx>
#include <HLRBRep_HLRToShape.hxx>
#include <bindings_common.hxx>
#include <gp_Ax2.hxx>
#include <gp_Dir.hxx>
#include <gp_Pnt.hxx>

// Fasad kecil di atas HLRBRep. `HLRBRep_Algo` dipakai lewat
// `Handle(...)` (Standard_Transient), dan `HLRBRep_HLRToShape` menuntut
// handle itu — bukan pointer mentah. Membungkusnya di sini jauh lebih
// sederhana daripada mengekspos tipe Handle-nya ke cxx.
class DucadHlrSession {
public:
  DucadHlrSession() : algo(new HLRBRep_Algo()) {}

  void add_shape(const TopoDS_Shape &shape) { algo->Add(shape); }

  // `dir` adalah arah PANDANG (dari mata ke objek); `up` menentukan
  // orientasi sumbu vertikal hasil proyeksi. Keduanya tidak boleh sejajar —
  // pemanggil di sisi Rust yang menjamin itu.
  bool set_projection(double dx, double dy, double dz, double ux, double uy, double uz) {
    try {
      gp_Ax2 cs(gp_Pnt(0.0, 0.0, 0.0), gp_Dir(dx, dy, dz), gp_Dir(ux, uy, uz));
      algo->Projector(HLRAlgo_Projector(cs));
      return true;
    } catch (const Standard_Failure &) {
      return false;
    }
  }

  // HLR eksak mahal dan bisa melempar untuk geometri patologis; kegagalan
  // dilaporkan sebagai `false`, bukan membatalkan proses.
  bool compute() {
    try {
      CoutSilencer silencer;
      algo->Update();
      algo->Hide();
      computed = true;
      return true;
    } catch (const Standard_Failure &) {
      return false;
    }
  }

  std::unique_ptr<TopoDS_Shape> visible_sharp() const { return extract(0); }
  std::unique_ptr<TopoDS_Shape> visible_outline() const { return extract(1); }
  std::unique_ptr<TopoDS_Shape> hidden_sharp() const { return extract(2); }
  std::unique_ptr<TopoDS_Shape> hidden_outline() const { return extract(3); }

private:
  std::unique_ptr<TopoDS_Shape> extract(int which) const {
    if (!computed) {
      return std::unique_ptr<TopoDS_Shape>(new TopoDS_Shape());
    }
    try {
      CoutSilencer silencer;
      HLRBRep_HLRToShape to_shape(algo);
      switch (which) {
      case 0:
        return std::unique_ptr<TopoDS_Shape>(new TopoDS_Shape(to_shape.VCompound()));
      case 1:
        return std::unique_ptr<TopoDS_Shape>(new TopoDS_Shape(to_shape.OutLineVCompound()));
      case 2:
        return std::unique_ptr<TopoDS_Shape>(new TopoDS_Shape(to_shape.HCompound()));
      default:
        return std::unique_ptr<TopoDS_Shape>(new TopoDS_Shape(to_shape.OutLineHCompound()));
      }
    } catch (const Standard_Failure &) {
      return std::unique_ptr<TopoDS_Shape>(new TopoDS_Shape());
    }
  }

  Handle(HLRBRep_Algo) algo;
  bool computed = false;
};

inline std::unique_ptr<DucadHlrSession> DucadHlrSession_ctor() {
  return std::unique_ptr<DucadHlrSession>(new DucadHlrSession());
}
