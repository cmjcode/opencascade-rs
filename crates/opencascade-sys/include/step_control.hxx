#include <STEPControl_Reader.hxx>
#include <STEPControl_Writer.hxx>
#include <STEPControl_Controller.hxx>
#include <Interface_Static.hxx>
#include <XSControl_WorkSession.hxx>
#include <bindings_common.hxx>

inline void silence_step() {
  static bool done = false;
  if (!done) {
    STEPControl_Controller::Init();
    Interface_Static::SetIVal("trace.level", 0);
    Interface_Static::SetIVal("read.step.trace", 0);
    Interface_Static::SetIVal("write.step.trace", 0);
    done = true;
  }
}

inline IFSelect_ReturnStatus read_step(STEPControl_Reader &reader, rust::String theFileName) {
  CoutSilencer silencer;
  silence_step();
  return reader.ReadFile(theFileName.c_str());
}

inline std::unique_ptr<TopoDS_Shape> one_shape_step(const STEPControl_Reader &reader) {
  return std::unique_ptr<TopoDS_Shape>(new TopoDS_Shape(reader.OneShape()));
}

inline IFSelect_ReturnStatus transfer_shape(STEPControl_Writer &writer, const TopoDS_Shape &theShape) {
  CoutSilencer silencer;
  silence_step();
  return writer.Transfer(theShape, STEPControl_AsIs);
}

inline IFSelect_ReturnStatus write_step(STEPControl_Writer &writer, rust::String theFileName) {
  CoutSilencer silencer;
  silence_step();
  return writer.Write(theFileName.c_str());
}

