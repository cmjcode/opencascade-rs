pub use inner::*;

#[cxx::bridge]
mod inner {
    unsafe extern "C++" {
        include!("opencascade-sys/include/shape_analysis.hxx");

        type TopoDS_Shape = crate::topo_ds::TopoDS_Shape;
        type Handle_TopTools_HSequenceOfShape = crate::top_tools::Handle_TopTools_HSequenceOfShape;

        type ShapeAnalysis_FreeBounds;
        pub fn ShapeAnalysis_FreeBounds_ConnectEdgesToWires(
            edges: Pin<&mut Handle_TopTools_HSequenceOfShape>,
            tolerance: f64,
            shared: bool,
            wires: Pin<&mut Handle_TopTools_HSequenceOfShape>,
        );
    }
}

#[allow(non_snake_case)]
impl ShapeAnalysis_FreeBounds {
    pub fn ConnectEdgesToWires(
        edges: std::pin::Pin<&mut Handle_TopTools_HSequenceOfShape>,
        tolerance: f64,
        shared: bool,
        wires: std::pin::Pin<&mut Handle_TopTools_HSequenceOfShape>,
    ) {
        ShapeAnalysis_FreeBounds_ConnectEdgesToWires(edges, tolerance, shared, wires);
    }
}
