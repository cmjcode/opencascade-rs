use crate::{
    mesh::{Mesh, Mesher},
    primitives::{
        make_axis_1, make_axis_2, make_dir, make_point, make_point2d, make_vec, BooleanShape,
        Compound, Edge, EdgeIterator, Face, FaceIterator, ShapeType, Shell, Solid, Vertex, Wire,
    },
    Error,
};
use cxx::UniquePtr;
use glam::{dvec2, dvec3, DVec3};
use opencascade_sys as ffi;
use std::{path::Path, pin::Pin};

pub struct Shape {
    pub(crate) inner: UniquePtr<ffi::topo_ds::TopoDS_Shape>,
}

impl Clone for Shape {
    fn clone(&self) -> Self {
        Self::from_shape(&self.inner)
    }
}

unsafe impl Send for Shape {}

impl AsRef<Shape> for Shape {
    fn as_ref(&self) -> &Shape {
        self
    }
}

impl From<Vertex> for Shape {
    fn from(vertex: Vertex) -> Self {
        let shape = ffi::topo_ds::cast_vertex_to_shape(&vertex.inner);

        Self::from_shape(shape)
    }
}

impl From<&Vertex> for Shape {
    fn from(vertex: &Vertex) -> Self {
        let shape = ffi::topo_ds::cast_vertex_to_shape(&vertex.inner);

        Self::from_shape(shape)
    }
}

impl From<Edge> for Shape {
    fn from(edge: Edge) -> Self {
        let shape = ffi::topo_ds::cast_edge_to_shape(&edge.inner);

        Self::from_shape(shape)
    }
}

impl From<&Edge> for Shape {
    fn from(edge: &Edge) -> Self {
        let shape = ffi::topo_ds::cast_edge_to_shape(&edge.inner);

        Self::from_shape(shape)
    }
}

impl From<Wire> for Shape {
    fn from(wire: Wire) -> Self {
        let shape = ffi::topo_ds::cast_wire_to_shape(&wire.inner);

        Self::from_shape(shape)
    }
}

impl From<&Wire> for Shape {
    fn from(wire: &Wire) -> Self {
        let shape = ffi::topo_ds::cast_wire_to_shape(&wire.inner);

        Self::from_shape(shape)
    }
}

impl From<Face> for Shape {
    fn from(face: Face) -> Self {
        let shape = ffi::topo_ds::cast_face_to_shape(&face.inner);

        Self::from_shape(shape)
    }
}

impl From<&Face> for Shape {
    fn from(face: &Face) -> Self {
        let shape = ffi::topo_ds::cast_face_to_shape(&face.inner);

        Self::from_shape(shape)
    }
}

impl From<Shell> for Shape {
    fn from(shell: Shell) -> Self {
        let shape = ffi::topo_ds::cast_shell_to_shape(&shell.inner);

        Self::from_shape(shape)
    }
}

impl From<&Shell> for Shape {
    fn from(shell: &Shell) -> Self {
        let shape = ffi::topo_ds::cast_shell_to_shape(&shell.inner);

        Self::from_shape(shape)
    }
}

impl From<Solid> for Shape {
    fn from(solid: Solid) -> Self {
        let shape = ffi::topo_ds::cast_solid_to_shape(&solid.inner);

        Self::from_shape(shape)
    }
}

impl From<&Solid> for Shape {
    fn from(solid: &Solid) -> Self {
        let shape = ffi::topo_ds::cast_solid_to_shape(&solid.inner);

        Self::from_shape(shape)
    }
}

impl From<Compound> for Shape {
    fn from(compound: Compound) -> Self {
        let shape = ffi::topo_ds::cast_compound_to_shape(&compound.inner);

        Self::from_shape(shape)
    }
}

impl From<&Compound> for Shape {
    fn from(compound: &Compound) -> Self {
        let shape = ffi::topo_ds::cast_compound_to_shape(&compound.inner);

        Self::from_shape(shape)
    }
}

impl From<BooleanShape> for Shape {
    fn from(boolean_shape: BooleanShape) -> Self {
        boolean_shape.shape
    }
}

pub struct SphereBuilder {
    center: DVec3,
    radius: f64,
    z_angle: f64,
}

impl SphereBuilder {
    pub fn build(self) -> Shape {
        let axis = make_axis_2(self.center, DVec3::Z);
        let mut make_shere =
            ffi::b_rep_prim_api::BRepPrimAPI_MakeSphere_new(&axis, self.radius, self.z_angle);

        Shape::from_shape(make_shere.pin_mut().Shape())
    }

    pub fn at(mut self, center: DVec3) -> Self {
        self.center = center;
        self
    }

    pub fn z_angle(mut self, z_angle: f64) -> Self {
        self.z_angle = z_angle;
        self
    }
}

pub struct ConeBuilder {
    pos: DVec3,
    height: f64,
    bottom_radius: f64,
    top_radius: f64,
    z_angle: f64,
}

impl ConeBuilder {
    pub fn build(self) -> Shape {
        let axis = make_axis_2(self.pos, DVec3::Z);
        let mut make_cone = ffi::b_rep_prim_api::BRepPrimAPI_MakeCone_new(
            &axis,
            self.bottom_radius,
            self.top_radius,
            self.height,
            self.z_angle,
        );

        Shape::from_shape(make_cone.pin_mut().Shape())
    }

    pub fn at(mut self, pos: DVec3) -> Self {
        self.pos = pos;
        self
    }

    pub fn bottom_radius(mut self, bottom_radius: f64) -> Self {
        self.bottom_radius = bottom_radius;
        self
    }

    pub fn top_radius(mut self, top_radius: f64) -> Self {
        self.top_radius = top_radius;
        self
    }

    pub fn height(mut self, height: f64) -> Self {
        self.height = height;
        self
    }

    pub fn z_angle(mut self, z_angle: f64) -> Self {
        self.z_angle = z_angle;
        self
    }
}

pub struct TorusBuilder {
    pos: DVec3,
    z_axis: DVec3,
    radius_1: f64,
    radius_2: f64,
    angle_1: f64,
    angle_2: f64,
    z_angle: f64,
}

impl TorusBuilder {
    pub fn build(self) -> Shape {
        let axis = make_axis_2(self.pos, self.z_axis);
        let mut make_torus = ffi::b_rep_prim_api::BRepPrimAPI_MakeTorus_new(
            &axis,
            self.radius_1,
            self.radius_2,
            self.angle_1,
            self.angle_2,
            self.z_angle,
        );

        Shape::from_shape(make_torus.pin_mut().Shape())
    }

    pub fn at(mut self, pos: DVec3) -> Self {
        self.pos = pos;
        self
    }

    pub fn z_axis(mut self, z_axis: DVec3) -> Self {
        self.z_axis = z_axis;
        self
    }

    pub fn radius_1(mut self, radius_1: f64) -> Self {
        self.radius_1 = radius_1;
        self
    }

    pub fn radius_2(mut self, radius_2: f64) -> Self {
        self.radius_2 = radius_2;
        self
    }

    pub fn angle_1(mut self, angle_1: f64) -> Self {
        self.angle_1 = angle_1;
        self
    }

    pub fn angle_2(mut self, angle_2: f64) -> Self {
        self.angle_2 = angle_2;
        self
    }

    pub fn z_angle(mut self, z_angle: f64) -> Self {
        self.z_angle = z_angle;
        self
    }
}

impl Shape {
    #[must_use]
    pub fn as_wire(&self) -> Option<Wire> {
        if self.shape_type() == ShapeType::Wire {
            let inner = ffi::topo_ds::TopoDS::Wire(&self.inner);
            Some(Wire::from_wire(inner))
        } else {
            None
        }
    }

    #[must_use]
    pub fn expect_wire(&self) -> Wire {
        self.as_wire().unwrap_or_else(|| panic!("expected Wire, got {:?}", self.shape_type()))
    }

    #[must_use]
    pub fn as_face(&self) -> Option<Face> {
        if self.shape_type() == ShapeType::Face {
            let inner = ffi::topo_ds::TopoDS::Face(&self.inner);
            Some(Face::from_face(inner))
        } else {
            None
        }
    }

    #[must_use]
    pub fn expect_face(&self) -> Face {
        self.as_face().unwrap_or_else(|| panic!("expected Face, got {:?}", self.shape_type()))
    }

    #[must_use]
    pub fn as_solid(&self) -> Option<Solid> {
        if self.shape_type() == ShapeType::Solid {
            let inner = ffi::topo_ds::TopoDS::Solid(&self.inner);
            Some(Solid::from_solid(inner))
        } else {
            None
        }
    }

    #[must_use]
    pub fn expect_solid(&self) -> Solid {
        self.as_solid().unwrap_or_else(|| panic!("expected Solid, got {:?}", self.shape_type()))
    }

    pub(crate) fn from_shape(shape: &ffi::topo_ds::TopoDS_Shape) -> Self {
        let inner = ffi::topo_ds::TopoDS_Shape_to_owned(shape);

        Self { inner }
    }

    /// Make a shape that models empty space.
    pub fn empty() -> Self {
        // NOTE: It may seem like using `TopoDS_Shape()` directly should work,
        //       but shape operations such as union fail on actual "null shapes".

        // Construct an empty compound
        let mut compound = ffi::topo_ds::TopoDS_Compound_new();
        let builder = ffi::b_rep::BRep_Builder_new();
        let topods_builder = ffi::b_rep::BRep_Builder_upcast_to_topods_builder(&builder);
        topods_builder.MakeCompound(compound.pin_mut());

        let inner = ffi::topo_ds::TopoDS_Compound_as_shape(compound);

        Self { inner }
    }

    /// Make a box with one corner at corner_1, and the opposite corner
    /// at corner_2.
    pub fn box_from_corners(corner_1: DVec3, corner_2: DVec3) -> Self {
        let min_corner = corner_1.min(corner_2);
        let max_corner = corner_1.max(corner_2);

        let point = ffi::gp::new_point(min_corner.x, min_corner.y, min_corner.z);
        let diff = max_corner - min_corner;
        let mut my_box =
            ffi::b_rep_prim_api::BRepPrimAPI_MakeBox_new(&point, diff.x, diff.y, diff.z);

        Self::from_shape(my_box.pin_mut().Shape())
    }

    /// Make a box with `width` (x), `depth` (y), and `height` (z)
    /// centered around the origin.
    pub fn box_centered(width: f64, depth: f64, height: f64) -> Self {
        let half_width = width / 2.0;
        let half_depth = depth / 2.0;
        let half_height = height / 2.0;

        let corner_1 = dvec3(-half_width, -half_depth, -half_height);
        let corner_2 = dvec3(half_width, half_depth, half_height);
        Self::box_from_corners(corner_1, corner_2)
    }

    /// Make a box with `width` (x), `depth` (y), and `height` (z)
    /// extending into the positive axes
    pub fn box_with_dimensions(width: f64, depth: f64, height: f64) -> Self {
        let corner_1 = DVec3::ZERO;
        let corner_2 = dvec3(width, depth, height);
        Self::box_from_corners(corner_1, corner_2)
    }

    /// Make a cube with side length of `size`
    /// extending into the positive axes
    pub fn cube(size: f64) -> Self {
        Self::box_with_dimensions(size, size, size)
    }

    /// Make a centered cube with side length of `size`
    pub fn cube_centered(size: f64) -> Self {
        Self::box_centered(size, size, size)
    }

    /// Make a cylinder with base at point `p`, radius `r`, and height `h`.
    /// Extends from `p` along axis `dir`.
    pub fn cylinder(p: DVec3, r: f64, dir: DVec3, h: f64) -> Self {
        let cylinder_coord_system = make_axis_2(p, dir);
        let mut cylinder =
            ffi::b_rep_prim_api::BRepPrimAPI_MakeCylinder_new(&cylinder_coord_system, r, h);

        Self::from_shape(cylinder.pin_mut().Shape())
    }

    /// Make a "default" cylinder with radius `r` and height `h`.
    /// The base is at the coordinate origin, and extends along the Z axis.
    pub fn cylinder_radius_height(r: f64, h: f64) -> Self {
        Self::cylinder(DVec3::ZERO, r, DVec3::Z, h)
    }

    /// Make a cylinder from start point `p1` and end point `p2`,
    /// with radius `r`.
    pub fn cylinder_from_points(p1: DVec3, p2: DVec3, r: f64) -> Self {
        let dir = p2 - p1;
        Self::cylinder(p1, r, dir, dir.length())
    }

    /// Make a cylinder centered at point `p`, with radius `r`, and height `h`.
    /// Extends along axis `dir`.
    pub fn cylinder_centered(p: DVec3, r: f64, dir: DVec3, h: f64) -> Self {
        let p = p - (dir.normalize() * (h / 2.0));
        Self::cylinder(p, r, dir, h)
    }

    pub fn sphere(radius: f64) -> SphereBuilder {
        SphereBuilder { center: DVec3::ZERO, radius, z_angle: std::f64::consts::TAU }
    }

    pub fn cone() -> ConeBuilder {
        ConeBuilder {
            pos: DVec3::ZERO,
            height: 1.0,
            bottom_radius: 1.0,
            top_radius: 0.0,
            z_angle: std::f64::consts::TAU,
        }
    }

    pub fn torus() -> TorusBuilder {
        TorusBuilder {
            pos: DVec3::ZERO,
            z_axis: DVec3::Z,
            radius_1: 20.0,
            radius_2: 10.0,
            angle_1: -std::f64::consts::PI,
            angle_2: std::f64::consts::PI,
            z_angle: std::f64::consts::TAU,
        }
    }

    pub fn shape_type(&self) -> ShapeType {
        self.inner.ShapeType().into()
    }

    pub fn fillet_edge(&mut self, radius: f64, edge: &Edge) -> Result<(), crate::Error> {
        let mut make_fillet = ffi::b_rep_fillet_api::BRepFilletAPI_MakeFillet_new(&self.inner);
        make_fillet.pin_mut().add_edge(radius, &edge.inner);

        let filleted_shape = ffi::b_rep_fillet_api::BRepFilletAPI_MakeFillet_shape_checked(make_fillet.pin_mut())
            .map_err(|e| crate::Error::FilletFailed(e.to_string()))?;

        self.inner = ffi::topo_ds::TopoDS_Shape_to_owned(filleted_shape);
        Ok(())
    }

    #[must_use]
    pub fn variable_fillet_edge(
        &self,
        radius_values: impl IntoIterator<Item = (f64, f64)>,
        edge: &Edge,
    ) -> Self {
        self.variable_fillet_edges(radius_values, [edge])
    }

    pub fn chamfer_edge(&mut self, distance: f64, edge: &Edge) -> Result<(), crate::Error> {
        let mut make_chamfer = ffi::b_rep_fillet_api::BRepFilletAPI_MakeChamfer_new(&self.inner);
        make_chamfer.pin_mut().add_edge(distance, &edge.inner);

        let chamfered_shape = ffi::b_rep_fillet_api::BRepFilletAPI_MakeChamfer_shape_checked(make_chamfer.pin_mut())
            .map_err(|e| crate::Error::FilletFailed(e.to_string()))?;

        self.inner = ffi::topo_ds::TopoDS_Shape_to_owned(chamfered_shape);
        Ok(())
    }

    pub fn fillet_edges<T: AsRef<Edge>>(
        &mut self,
        radius: f64,
        edges: impl IntoIterator<Item = T>,
    ) -> Result<(), crate::Error> {
        let mut make_fillet = ffi::b_rep_fillet_api::BRepFilletAPI_MakeFillet_new(&self.inner);

        for edge in edges.into_iter() {
            make_fillet.pin_mut().add_edge(radius, &edge.as_ref().inner);
        }

        let filleted_shape = ffi::b_rep_fillet_api::BRepFilletAPI_MakeFillet_shape_checked(make_fillet.pin_mut())
            .map_err(|e| crate::Error::FilletFailed(e.to_string()))?;

        self.inner = ffi::topo_ds::TopoDS_Shape_to_owned(filleted_shape);
        Ok(())
    }

    pub fn fillet_edges_variable<T: AsRef<Edge>>(
        &mut self,
        radius_start: f64,
        radius_end: f64,
        edges: impl IntoIterator<Item = T>,
    ) -> Result<(), crate::Error> {
        let mut array = ffi::t_col_gp::TColgp_Array1OfPnt2d_new(1, 2);
        array.pin_mut().SetValue(1, &make_point2d(dvec2(0.0, radius_start)));
        array.pin_mut().SetValue(2, &make_point2d(dvec2(1.0, radius_end)));

        let mut make_fillet = ffi::b_rep_fillet_api::BRepFilletAPI_MakeFillet_new(&self.inner);

        for edge in edges.into_iter() {
            make_fillet
                .pin_mut()
                .variable_add_edge(&array, &edge.as_ref().inner);
        }

        let filleted_shape = ffi::b_rep_fillet_api::BRepFilletAPI_MakeFillet_shape_checked(make_fillet.pin_mut())
            .map_err(|e| crate::Error::FilletFailed(e.to_string()))?;

        self.inner = ffi::topo_ds::TopoDS_Shape_to_owned(filleted_shape);
        Ok(())
    }

    #[must_use]
    pub fn variable_fillet_edges<T: AsRef<Edge>>(
        &self,
        radius_values: impl IntoIterator<Item = (f64, f64)>,
        edges: impl IntoIterator<Item = T>,
    ) -> Self {
        let radius_values: Vec<_> = radius_values.into_iter().collect();
        let mut array = ffi::t_col_gp::TColgp_Array1OfPnt2d_new(1, radius_values.len() as i32);

        for (index, (t, radius)) in radius_values.into_iter().enumerate() {
            array.pin_mut().SetValue(index as i32 + 1, &make_point2d(dvec2(t, radius)));
        }

        let mut make_fillet = ffi::b_rep_fillet_api::BRepFilletAPI_MakeFillet_new(&self.inner);

        for edge in edges.into_iter() {
            make_fillet.pin_mut().variable_add_edge(&array, &edge.as_ref().inner);
        }

        Self::from_shape(make_fillet.pin_mut().Shape())
    }

    pub fn chamfer_edges<T: AsRef<Edge>>(
        &mut self,
        distance: f64,
        edges: impl IntoIterator<Item = T>,
    ) -> Result<(), crate::Error> {
        let mut make_chamfer = ffi::b_rep_fillet_api::BRepFilletAPI_MakeChamfer_new(&self.inner);

        for edge in edges.into_iter() {
            make_chamfer.pin_mut().add_edge(distance, &edge.as_ref().inner);
        }

        let chamfered_shape = ffi::b_rep_fillet_api::BRepFilletAPI_MakeChamfer_shape_checked(make_chamfer.pin_mut())
            .map_err(|e| crate::Error::FilletFailed(e.to_string()))?;

        self.inner = ffi::topo_ds::TopoDS_Shape_to_owned(chamfered_shape);
        Ok(())
    }

    /// Performs fillet of `radius` on all edges of the shape
    pub fn fillet(&mut self, radius: f64) -> Result<(), crate::Error> {
        self.fillet_edges(radius, self.edges())
    }

    /// Performs chamfer of `distance` on all edges of the shape
    pub fn chamfer(&mut self, distance: f64) -> Result<(), crate::Error> {
        self.chamfer_edges(distance, self.edges())
    }

    pub fn subtract(&self, other: &Shape) -> Result<BooleanShape, crate::Error> {
        let mut cut_operation =
            ffi::b_rep_algo_api::BRepAlgoAPI_Cut_ctor_checked(&self.inner, &other.inner)
                .map_err(|e| crate::Error::BooleanOpFailed(e.to_string()))?;

        let edge_list = cut_operation.pin_mut().SectionEdges();
        let vec = ffi::topo_ds::shape_list_to_vector(edge_list);

        let mut new_edges = vec![];
        for shape in vec.iter() {
            let edge = ffi::topo_ds::TopoDS::Edge(shape);
            new_edges.push(Edge::from_edge(edge));
        }

        let cut_shape = ffi::b_rep_algo_api::BRepAlgoAPI_Cut_shape_checked(cut_operation.pin_mut())
            .map_err(|e| crate::Error::BooleanOpFailed(e.to_string()))?;
        let shape = Self::from_shape(cut_shape);

        Ok(BooleanShape { shape, new_edges })
    }

    pub fn read_step(path: impl AsRef<Path>) -> Result<Self, Error> {
        let mut reader = ffi::step_control::STEPControl_Reader_new();

        let status = ffi::step_control::read_step(
            reader.pin_mut(),
            path.as_ref().to_string_lossy().to_string(),
        );

        if status != ffi::if_select::IFSelect_ReturnStatus::IFSelect_RetDone {
            return Err(Error::StepReadFailed);
        }

        reader.pin_mut().TransferRoots(&ffi::message::Message_ProgressRange_new());

        let inner = ffi::step_control::one_shape_step(&reader);

        Ok(Self { inner })
    }

    pub fn write_step(&self, path: impl AsRef<Path>) -> Result<(), Error> {
        Self::write_all_step(std::iter::once(self), path)
    }

    pub fn write_all_step<T: AsRef<Shape>>(
        shapes: impl IntoIterator<Item = T>,
        path: impl AsRef<Path>,
    ) -> Result<(), Error> {
        let mut writer = ffi::step_control::STEPControl_Writer_new();
        let mut count = 0;

        for shape in shapes {
            let status = ffi::step_control::transfer_shape(writer.pin_mut(), &shape.as_ref().inner);

            if status != ffi::if_select::IFSelect_ReturnStatus::IFSelect_RetDone {
                return Err(Error::StepWriteTransferFailed);
            }

            count += 1;
        }

        if count == 0 {
            return Err(Error::StepWriteNoShapes);
        }

        let status = ffi::step_control::write_step(
            writer.pin_mut(),
            path.as_ref().to_string_lossy().to_string(),
        );

        if status != ffi::if_select::IFSelect_ReturnStatus::IFSelect_RetDone {
            return Err(Error::StepWriteFailed);
        }

        Ok(())
    }

    pub fn read_iges(path: impl AsRef<Path>) -> Result<Self, Error> {
        let mut reader = ffi::iges_control::IGESControl_Reader_new();

        let status = ffi::iges_control::read_iges(
            reader.pin_mut(),
            path.as_ref().to_string_lossy().to_string(),
        );

        reader.pin_mut().TransferRoots(&ffi::message::Message_ProgressRange_new());

        if status != ffi::if_select::IFSelect_ReturnStatus::IFSelect_RetDone {
            return Err(Error::IgesReadFailed);
        }

        let inner = ffi::iges_control::one_shape_iges(&reader);

        Ok(Self { inner })
    }

    pub fn write_iges(&self, path: impl AsRef<Path>) -> Result<(), Error> {
        let mut writer = ffi::iges_control::IGESControl_Writer_new();

        let success =
            writer.pin_mut().AddShape(&self.inner, &ffi::message::Message_ProgressRange_new());

        if !success {
            return Err(Error::IgesWriteFailed);
        }

        writer.pin_mut().ComputeModel();
        let success = ffi::iges_control::write_iges(
            writer.pin_mut(),
            path.as_ref().to_string_lossy().to_string(),
        );

        if success {
            Ok(())
        } else {
            Err(Error::IgesWriteFailed)
        }
    }

    pub fn write_brep_text(&self, path: impl AsRef<Path>) -> Result<(), Error> {
        let success =
            ffi::b_rep_tools::write(&self.inner, path.as_ref().to_string_lossy().to_string());

        if success {
            Ok(())
        } else {
            Err(Error::BrepWriteFailed)
        }
    }

    pub fn read_brep_text(path: impl AsRef<Path>) -> Result<Self, Error> {
        let inner = ffi::b_rep_tools::read(path.as_ref().to_string_lossy().to_string());

        if inner.is_null() {
            Err(Error::BrepReadFailed)
        } else {
            Ok(Self { inner })
        }
    }

    pub fn write_brep_bin(&self, path: impl AsRef<Path>) -> Result<(), Error> {
        let success =
            ffi::bin_tools::write(&self.inner, path.as_ref().to_string_lossy().to_string());

        if success {
            Ok(())
        } else {
            Err(Error::BrepWriteFailed)
        }
    }

    pub fn read_brep_bin(path: impl AsRef<Path>) -> Result<Self, Error> {
        let inner = ffi::bin_tools::read(path.as_ref().to_string_lossy().to_string());

        if inner.is_null() {
            Err(Error::BrepReadFailed)
        } else {
            Ok(Self { inner })
        }
    }

    pub fn union(&self, other: &Shape) -> Result<BooleanShape, crate::Error> {
        let mut fuse_operation =
            ffi::b_rep_algo_api::BRepAlgoAPI_Fuse_ctor_checked(&self.inner, &other.inner)
                .map_err(|e| crate::Error::BooleanOpFailed(e.to_string()))?;
        let edge_list = fuse_operation.pin_mut().SectionEdges();
        let vec = ffi::topo_ds::shape_list_to_vector(edge_list);

        let mut new_edges = vec![];
        for shape in vec.iter() {
            let edge = ffi::topo_ds::TopoDS::Edge(shape);
            new_edges.push(Edge::from_edge(edge));
        }

        let fuse_shape = ffi::b_rep_algo_api::BRepAlgoAPI_Fuse_shape_checked(fuse_operation.pin_mut())
            .map_err(|e| crate::Error::BooleanOpFailed(e.to_string()))?;
        let shape = Self::from_shape(fuse_shape);

        Ok(BooleanShape { shape, new_edges })
    }

    pub fn intersect(&self, other: &Shape) -> Result<BooleanShape, crate::Error> {
        let mut common_operation =
            ffi::b_rep_algo_api::BRepAlgoAPI_Common_ctor_checked(&self.inner, &other.inner)
                .map_err(|e| crate::Error::BooleanOpFailed(e.to_string()))?;
        let edge_list = common_operation.pin_mut().SectionEdges();
        let vec = ffi::topo_ds::shape_list_to_vector(edge_list);

        let mut new_edges = vec![];
        for shape in vec.iter() {
            let edge = ffi::topo_ds::TopoDS::Edge(shape);
            new_edges.push(Edge::from_edge(edge));
        }

        let common_shape = ffi::b_rep_algo_api::BRepAlgoAPI_Common_shape_checked(common_operation.pin_mut())
            .map_err(|e| crate::Error::BooleanOpFailed(e.to_string()))?;
        let shape = Self::from_shape(common_shape);

        Ok(BooleanShape { shape, new_edges })
    }

    pub fn write_stl<P: AsRef<Path>>(&self, path: P) -> Result<(), Error> {
        self.write_stl_with_tolerance(path, 0.001)
    }

    pub fn write_stl_with_tolerance<P: AsRef<Path>>(
        &self,
        path: P,
        triangulation_tolerance: f64,
    ) -> Result<(), Error> {
        let mut stl_writer = ffi::stl_api::StlAPI_Writer_new();
        let mesher = Mesher::try_new(self, triangulation_tolerance)?;
        let success = ffi::stl_api::write_stl(
            stl_writer.pin_mut(),
            mesher.inner.Shape(),
            path.as_ref().to_string_lossy().to_string(),
        );

        if success {
            Ok(())
        } else {
            Err(Error::StlWriteFailed)
        }
    }

    pub fn read_stl<P: AsRef<Path>>(path: P) -> Result<Self, Error> {
        let mut shape = ffi::topo_ds::TopoDS_Compound_as_shape(ffi::topo_ds::TopoDS_Compound_new());
        let success = ffi::stl_api::read_stl(
            shape.pin_mut(),
            path.as_ref().to_string_lossy().to_string(),
        );

        if success {
            Ok(Self { inner: shape })
        } else {
            Err(Error::StlReadFailed)
        }
    }

    #[must_use]
    pub fn clean(&self) -> Self {
        let mut upgrader = ffi::shape_upgrade::UnifySameDomain_new(&self.inner, true, true, true);
        upgrader.pin_mut().allow_internal_edges(false);
        upgrader.pin_mut().build();

        Self::from_shape(upgrader.shape())
    }

    pub fn set_global_translation(&mut self, translation: DVec3) {
        let mut transform = ffi::gp::new_transform();
        let translation_vec = make_vec(translation);
        transform.pin_mut().set_translation_vec(&translation_vec);

        let location = ffi::top_loc::Location_from_transform(&transform);

        self.inner.pin_mut().set_global_translation(&location, false);
    }

    pub fn scale(&mut self, pivot: DVec3, factor: f64) {
        let point = make_point(pivot);
        let mut transform = ffi::gp::new_transform();
        transform.pin_mut().SetScale(&point, factor);

        let mut transform_builder =
            ffi::b_rep_builder_api::BRepBuilderAPI_Transform_new(&self.inner, &transform, true);
        transform_builder.pin_mut().Build(&ffi::message::Message_ProgressRange_new());
        let transformed_shape = transform_builder.pin_mut().Shape();
        self.inner = ffi::topo_ds::TopoDS_Shape_to_owned(transformed_shape);
    }

    pub fn rotate(&mut self, pivot: DVec3, axis: DVec3, angle_rad: f64) {
        let axis_1 = make_axis_1(pivot, axis);
        let mut transform = ffi::gp::new_transform();
        transform.pin_mut().SetRotation(&axis_1, angle_rad);

        let mut transform_builder =
            ffi::b_rep_builder_api::BRepBuilderAPI_Transform_new(&self.inner, &transform, true);
        transform_builder.pin_mut().Build(&ffi::message::Message_ProgressRange_new());
        let transformed_shape = transform_builder.pin_mut().Shape();
        self.inner = ffi::topo_ds::TopoDS_Shape_to_owned(transformed_shape);
    }

    pub fn volume(&self) -> f64 {
        let mut props = ffi::g_prop::GProps_new();
        let only_closed = false;
        let skip_shared = false;
        let use_triangulation = false;
        ffi::b_rep_g_prop::BRepGProp::VolumeProperties(
            &self.inner,
            props.pin_mut(),
            only_closed,
            skip_shared,
            use_triangulation,
        );
        props.Mass()
    }

    pub fn mesh(&self) -> Result<Mesh, Error> {
        let bb = crate::bounding_box::aabb(self);
        let diag = if bb.is_void() {
            50.0
        } else {
            (bb.max() - bb.min()).length().max(1.0)
        };
        let tol = (diag * 0.001).clamp(0.05, 0.5);
        self.mesh_with_tolerance(tol)
    }

    pub fn mesh_with_tolerance(&self, triangulation_tolerance: f64) -> Result<Mesh, Error> {
        let mesher = Mesher::try_new(self, triangulation_tolerance)?;
        mesher.mesh()
    }

    pub fn edges(&self) -> EdgeIterator {
        let explorer = ffi::top_exp::TopExp_Explorer_new(
            &self.inner,
            ffi::top_abs::TopAbs_ShapeEnum::TopAbs_EDGE,
        );
        EdgeIterator { explorer }
    }

    pub fn faces(&self) -> FaceIterator {
        let explorer = ffi::top_exp::TopExp_Explorer_new(
            &self.inner,
            ffi::top_abs::TopAbs_ShapeEnum::TopAbs_FACE,
        );
        FaceIterator { explorer }
    }

    pub fn faces_along_ray(&self, ray_start: DVec3, ray_dir: DVec3) -> Vec<(Face, DVec3)> {
        self.faces_along_ray_with_tolerance(ray_start, ray_dir, 0.0001)
    }

    pub fn faces_along_ray_with_tolerance(
        &self,
        ray_start: DVec3,
        ray_dir: DVec3,
        tolerance: f64,
    ) -> Vec<(Face, DVec3)> {
        let mut intersector = ffi::b_rep_int_curve_surface::BRepIntCurveSurface_Inter_new();
        intersector.pin_mut().Init(
            &self.inner,
            &ffi::gp::gp_Lin_new(&make_point(ray_start), &make_dir(ray_dir)),
            tolerance,
        );

        let mut results = vec![];

        while intersector.More() {
            let face = ffi::b_rep_int_curve_surface::BRepIntCurveSurface_Inter_face(&intersector);
            let face = Face::from_face(&face);
            let point = ffi::b_rep_int_curve_surface::BRepIntCurveSurface_Inter_point(&intersector);

            results.push((face, dvec3(point.X(), point.Y(), point.Z())));

            intersector.pin_mut().Next();
        }

        results
    }

    pub fn faces_along_line(&self, line_origin: DVec3, line_dir: DVec3) -> Vec<LineFaceHitPoint> {
        self.faces_along_line_with_tolerance(line_origin, line_dir, 0.0001)
    }

    pub fn faces_along_line_with_tolerance(
        &self,
        line_origin: DVec3,
        line_dir: DVec3,
        tolerance: f64,
    ) -> Vec<LineFaceHitPoint> {
        let mut intersector = ffi::b_rep_int_curve_surface::BRepIntCurveSurface_Inter_new();
        intersector.pin_mut().Init(
            &self.inner,
            &ffi::gp::gp_Lin_new(&make_point(line_origin), &make_dir(line_dir)),
            tolerance,
        );

        let mut results = vec![];

        while intersector.More() {
            let face = ffi::b_rep_int_curve_surface::BRepIntCurveSurface_Inter_face(&intersector);
            let face = Face::from_face(&face);
            let point = ffi::b_rep_int_curve_surface::BRepIntCurveSurface_Inter_point(&intersector);

            results.push(LineFaceHitPoint {
                face,
                t: intersector.W(),
                u: intersector.U(),
                v: intersector.V(),
                point: dvec3(point.X(), point.Y(), point.Z()),
            });

            intersector.pin_mut().Next();
        }

        results
    }

    /// Create a transformed copy of this shape using a `gp_Trsf` configured by `configure`.
    fn with_transform(&self, configure: impl FnOnce(Pin<&mut ffi::gp::gp_Trsf>)) -> Self {
        let mut transform = ffi::gp::new_transform();
        configure(transform.pin_mut());
        let mut brep =
            ffi::b_rep_builder_api::BRepBuilderAPI_Transform_new(&self.inner, &transform, true);
        Self::from_shape(brep.pin_mut().Shape())
    }

    /// Create a translated copy of this shape.
    #[must_use]
    pub fn translated(&self, offset: DVec3) -> Self {
        self.with_transform(|trsf| {
            let translation_vec = make_vec(offset);
            trsf.set_translation_vec(&translation_vec);
        })
    }

    /// Create a rotated copy of this shape about an axis through the origin.
    #[must_use]
    pub fn rotated(&self, axis: DVec3, angle: f64) -> Self {
        self.with_transform(|trsf| {
            let axis_1 = make_axis_1(DVec3::ZERO, axis);
            trsf.SetRotation(&axis_1, angle);
        })
    }

    /// Create a scaled copy of this shape about a point.
    #[must_use]
    pub fn scaled(&self, point: DVec3, factor: f64) -> Self {
        self.with_transform(|trsf| {
            let pnt = make_point(point);
            trsf.SetScale(&pnt, factor);
        })
    }

    /// Create a mirrored copy of this shape about an axis.
    #[must_use]
    pub fn mirrored(&self, origin: DVec3, dir: DVec3) -> Self {
        self.with_transform(|trsf| {
            let axis_1 = make_axis_1(origin, dir);
            trsf.set_mirror_axis(&axis_1);
        })
    }

    pub fn try_hollow<T: AsRef<Face>>(
        &self,
        offset: f64,
        faces_to_remove: impl IntoIterator<Item = T>,
    ) -> Result<Self, Error> {
        let mut faces_list = ffi::top_tools::new_list_of_shape();

        for face in faces_to_remove.into_iter() {
            let shape = ffi::topo_ds::cast_face_to_shape(&face.as_ref().inner);
            faces_list.pin_mut().Append(shape);
        }

        let mut solid_maker = ffi::b_rep_offset_api::BRepOffsetAPI_MakeThickSolid_new();

        let offset_mode = ffi::b_rep_offset_api::BRepOffset_Mode::BRepOffset_Skin;
        let intersection = false;
        let self_intersection = false;
        let join_type = ffi::geom_abs::GeomAbs_JoinType::GeomAbs_Arc;
        let remove_intersecting_edges = false;

        solid_maker.pin_mut().MakeThickSolidByJoin(
            &self.inner,
            &faces_list,
            offset,
            0.001,
            offset_mode,
            intersection,
            self_intersection,
            join_type,
            remove_intersecting_edges,
            &ffi::message::Message_ProgressRange_new(),
        );
        solid_maker.pin_mut().Build(&ffi::message::Message_ProgressRange_new());

        let hollowed_shape =
            ffi::b_rep_offset_api::BRepOffsetAPI_MakeThickSolid_shape_checked(solid_maker.pin_mut())
                .map_err(|e| Error::HollowFailed(e.to_string()))?;
        let res = Self::from_shape(hollowed_shape);
        if res.faces().count() == 0 {
            return Err(Error::HollowFailed("Operasi hollow menghasilkan bentuk kosong (tidak ada face tersisa)".to_string()));
        }
        Ok(res)
    }

    pub fn try_hollow_variable<T: AsRef<Face>, U: AsRef<Face>>(
        &self,
        default_offset: f64,
        faces_to_remove: impl IntoIterator<Item = T>,
        variable_faces: impl IntoIterator<Item = (U, f64)>,
    ) -> Result<Self, Error> {
        const OFFSET_TOLERANCE: f64 = 1e-3;
        let to_error = |e: cxx::Exception| Error::HollowFailed(e.what().to_string());

        let mut make_offset = ffi::b_rep_offset_api::BRepOffset_MakeOffset_ctor();
        ffi::b_rep_offset_api::BRepOffset_MakeOffset_Initialize(
            make_offset.pin_mut(),
            &self.inner,
            default_offset,
            OFFSET_TOLERANCE,
            ffi::b_rep_offset_api::BRepOffset_Mode::BRepOffset_Skin,
            true,
            false,
            ffi::geom_abs::GeomAbs_JoinType::GeomAbs_Arc,
            true,
            false,
        )
        .map_err(to_error)?;

        for face in faces_to_remove {
            ffi::b_rep_offset_api::BRepOffset_MakeOffset_AddFace(
                make_offset.pin_mut(),
                &face.as_ref().inner,
            )
            .map_err(to_error)?;
        }

        for (face, offset) in variable_faces {
            ffi::b_rep_offset_api::BRepOffset_MakeOffset_SetOffsetOnFace(
                make_offset.pin_mut(),
                &face.as_ref().inner,
                offset,
            )
            .map_err(to_error)?;
        }

        ffi::b_rep_offset_api::BRepOffset_MakeOffset_MakeOffsetShape(make_offset.pin_mut())
            .map_err(to_error)?;

        let result_shape =
            ffi::b_rep_offset_api::BRepOffset_MakeOffset_Shape(&make_offset).map_err(to_error)?;
        Ok(Self::from_shape(result_shape))
    }

    #[must_use]
    pub fn hollow<T: AsRef<Face>>(
        &self,
        offset: f64,
        faces_to_remove: impl IntoIterator<Item = T>,
    ) -> Self {
        self.try_hollow(offset, faces_to_remove)
            .unwrap_or_else(|e| panic!("Failed to hollow shape: {e}"))
    }

    #[must_use]
    pub fn offset_surface(&self, offset: f64) -> Self {
        let faces_to_remove: [Face; 0] = [];
        self.hollow(offset, faces_to_remove)
    }

    pub fn offset_on_face(&self, face: &Face, offset: f64) -> Result<Self, crate::Error> {
        const OFFSET_TOLERANCE: f64 = 1e-4;

        let to_error = |e: cxx::Exception| crate::Error::OffsetOnFaceFailed(e.what().to_string());

        let mut make_offset = ffi::b_rep_offset_api::BRepOffset_MakeOffset_ctor();
        ffi::b_rep_offset_api::BRepOffset_MakeOffset_Initialize(
            make_offset.pin_mut(),
            &self.inner,
            0.0,
            OFFSET_TOLERANCE,
            ffi::b_rep_offset_api::BRepOffset_Mode::BRepOffset_Skin,
            true,
            false,
            ffi::geom_abs::GeomAbs_JoinType::GeomAbs_Intersection,
            false,
            false,
        )
        .map_err(to_error)?;

        ffi::b_rep_offset_api::BRepOffset_MakeOffset_SetOffsetOnFace(
            make_offset.pin_mut(),
            &face.inner,
            offset,
        )
        .map_err(to_error)?;

        ffi::b_rep_offset_api::BRepOffset_MakeOffset_MakeOffsetShape(make_offset.pin_mut())
            .map_err(to_error)?;

        let result_shape =
            ffi::b_rep_offset_api::BRepOffset_MakeOffset_Shape(&make_offset).map_err(to_error)?;
        Ok(Self::from_shape(result_shape))
    }

    pub fn pipe(spine: &Wire, profile: &Shape) -> Result<Self, crate::Error> {
        let mut make_pipe =
            ffi::b_rep_offset_api::BRepOffsetAPI_MakePipe_ctor_checked(&spine.inner, &profile.inner)
                .map_err(|e| crate::Error::PipeFailed(e.what().to_string()))?;

        let result_shape =
            ffi::b_rep_offset_api::BRepOffsetAPI_MakePipe_shape_checked(make_pipe.pin_mut())
                .map_err(|e| crate::Error::PipeFailed(e.what().to_string()))?;

        Ok(Self::from_shape(result_shape))
    }

    pub fn draft_angle(
        &self,
        neutral_plane_point: DVec3,
        neutral_plane_normal: DVec3,
        pull_direction: DVec3,
        angle_deg: f64,
        faces: &[&Face],
    ) -> Result<Self, crate::Error> {
        let to_error = |e: cxx::Exception| crate::Error::DraftAngleFailed(e.what().to_string());

        let angle_rad = angle_deg.to_radians();

        let pull_dir = make_dir(pull_direction);
        let np_point = make_point(neutral_plane_point);
        let np_normal = make_dir(neutral_plane_normal);
        let neutral_plane = ffi::gp::gp_Pln_ctor_point_and_dir(&np_point, &np_normal);

        let mut draft =
            ffi::b_rep_offset_api::BRepOffsetAPI_DraftAngle_ctor(&self.inner).map_err(to_error)?;

        for face in faces {
            ffi::b_rep_offset_api::BRepOffsetAPI_DraftAngle_Add(
                draft.pin_mut(),
                &face.inner,
                &pull_dir,
                angle_rad,
                &neutral_plane,
            )
            .map_err(to_error)?;
        }

        ffi::b_rep_offset_api::BRepOffsetAPI_DraftAngle_Build(draft.pin_mut()).map_err(to_error)?;

        if !ffi::b_rep_offset_api::BRepOffsetAPI_DraftAngle_IsDone(&draft) {
            return Err(crate::Error::DraftAngleFailed(
                "Build() selesai tanpa error tapi IsDone() == false (sudut atau face tidak kompatibel)"
                    .to_string(),
            ));
        }

        let result_shape =
            ffi::b_rep_offset_api::BRepOffsetAPI_DraftAngle_shape_checked(draft.pin_mut())
                .map_err(to_error)?;
        Ok(Self::from_shape(result_shape))
    }

    pub fn split_with_plane(
        &self,
        plane_point: DVec3,
        plane_normal: DVec3,
    ) -> Result<Vec<Self>, crate::Error> {
        let to_error = |e: cxx::Exception| crate::Error::SplitFailed(e.what().to_string());

        let res_vec = ffi::b_rep_algo_api::split_shape_with_plane(
            &self.inner,
            plane_point.x,
            plane_point.y,
            plane_point.z,
            plane_normal.x,
            plane_normal.y,
            plane_normal.z,
        )
        .map_err(to_error)?;

        let mut shapes = Vec::new();
        if let Some(vec) = res_vec.as_ref() {
            for shape_ref in vec.iter() {
                shapes.push(Self::from_shape(shape_ref));
            }
        }

        Ok(shapes)
    }

    pub fn split_with_tool(&self, tool: &Shape) -> Result<Vec<Self>, crate::Error> {
        let to_error = |e: cxx::Exception| crate::Error::SplitFailed(e.what().to_string());

        let res_vec = ffi::b_rep_algo_api::split_shape_with_tool(&self.inner, &tool.inner)
            .map_err(to_error)?;

        let mut shapes = Vec::new();
        if let Some(vec) = res_vec.as_ref() {
            for shape_ref in vec.iter() {
                shapes.push(Self::from_shape(shape_ref));
            }
        }

        Ok(shapes)
    }

    pub fn split_faces_with_plane(
        &self,
        plane_point: DVec3,
        plane_normal: DVec3,
    ) -> Result<Self, crate::Error> {
        let to_error = |e: cxx::Exception| crate::Error::SplitFailed(e.what().to_string());

        let res_shape = ffi::b_rep_algo_api::split_faces_with_plane(
            &self.inner,
            plane_point.x,
            plane_point.y,
            plane_point.z,
            plane_normal.x,
            plane_normal.y,
            plane_normal.z,
        )
        .map_err(to_error)?;

        Ok(Self::from_shape(&res_shape))
    }

    pub fn section_with_plane(
        &self,
        plane_point: DVec3,
        plane_normal: DVec3,
    ) -> Result<Vec<Self>, crate::Error> {
        let to_error = |e: cxx::Exception| crate::Error::SplitFailed(e.what().to_string());

        let res_vec = ffi::b_rep_algo_api::section_shape_with_plane(
            &self.inner,
            plane_point.x,
            plane_point.y,
            plane_point.z,
            plane_normal.x,
            plane_normal.y,
            plane_normal.z,
        )
        .map_err(to_error)?;

        let mut shapes = Vec::new();
        if let Some(vec) = res_vec.as_ref() {
            for shape_ref in vec.iter() {
                shapes.push(Self::from_shape(shape_ref));
            }
        }

        Ok(shapes)
    }

    /// Drill a cylindrical hole along the line defined by point `p`
    /// and direction `dir`, with `radius`.
    #[must_use]
    pub fn drill_hole(&self, p: DVec3, dir: DVec3, radius: f64) -> Self {
        let hole_axis = make_axis_1(p, dir);

        let mut make_hole = ffi::b_rep_feat::BRepFeat_MakeCylindricalHole_new();
        make_hole.pin_mut().Init(&self.inner, &hole_axis);

        make_hole.pin_mut().Perform(radius);
        make_hole.pin_mut().Build();

        Self::from_shape(make_hole.pin_mut().Shape())
    }
}

/// Information about a point where a line hits (i.e. intersects) a face
pub struct LineFaceHitPoint {
    /// The face that is hit
    pub face: Face,
    /// The T parameter along the line
    pub t: f64,
    /// The U parameter on the face
    pub u: f64,
    /// The V parameter on the face
    pub v: f64,
    /// The intersection point
    pub point: DVec3,
}

pub struct ChamferMaker {
    inner: UniquePtr<ffi::b_rep_fillet_api::BRepFilletAPI_MakeChamfer>,
}

impl ChamferMaker {
    pub fn new(shape: &Shape) -> Self {
        let make_chamfer = ffi::b_rep_fillet_api::BRepFilletAPI_MakeChamfer_new(&shape.inner);

        Self { inner: make_chamfer }
    }

    pub fn add_edge(&mut self, distance: f64, edge: &Edge) {
        self.inner.pin_mut().add_edge(distance, &edge.inner);
    }

    pub fn build(mut self) -> Shape {
        Shape::from_shape(self.inner.pin_mut().Shape())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn face_shape() -> Shape {
        Shape::from(&Face::from_wire(&Wire::rect(10.0, 10.0)))
    }

    fn wire_shape() -> Shape {
        Shape::from(&Wire::rect(10.0, 10.0))
    }

    fn solid_shape() -> Shape {
        Shape::box_centered(10.0, 10.0, 10.0)
    }

    #[test]
    fn test_as_wire() {
        let shape = wire_shape();
        assert!(shape.as_wire().is_some());
        assert!(shape.as_face().is_none());
        assert!(shape.as_solid().is_none());
    }

    #[test]
    fn test_as_face() {
        let shape = face_shape();
        assert!(shape.as_face().is_some());
        assert!(shape.as_wire().is_none());
        assert!(shape.as_solid().is_none());
    }

    #[test]
    fn test_as_solid() {
        let shape = solid_shape();
        assert!(shape.as_solid().is_some());
        assert!(shape.as_wire().is_none());
        assert!(shape.as_face().is_none());
    }

    #[test]
    fn test_empty_shape() {
        let shape = Shape::empty();
        assert!(shape.as_wire().is_none());
        assert!(shape.as_face().is_none());
        assert!(shape.as_solid().is_none());
    }

    #[test]
    fn test_expect_wire() {
        let shape = wire_shape();
        let _wire = shape.expect_wire();
    }

    #[test]
    #[should_panic(expected = "expected Wire, got Face")]
    fn test_expect_wire_panics_on_face() {
        let shape = face_shape();
        let _wire = shape.expect_wire();
    }

    #[test]
    #[should_panic(expected = "expected Face, got Solid")]
    fn test_expect_face_panics_on_solid() {
        let shape = solid_shape();
        let _face = shape.expect_face();
    }

    #[test]
    #[should_panic(expected = "expected Solid, got Face")]
    fn test_expect_solid_panics_on_face() {
        let shape = face_shape();
        let _solid = shape.expect_solid();
    }

    #[test]
    fn test_write_step() {
        let shape = solid_shape();
        let path = std::env::temp_dir().join("test_write_step.step");
        let result = shape.write_step(&path);
        assert!(result.is_ok());
        assert!(path.exists());
        assert!(path.metadata().unwrap().len() > 0);
        let _ = std::fs::remove_file(&path);
    }

    #[test]
    fn test_write_all_step_one_shape() {
        let shape = solid_shape();
        let path = std::env::temp_dir().join("test_write_all_step_one.step");
        let result = Shape::write_all_step([&shape], &path);
        assert!(result.is_ok());
        assert!(path.exists());
        assert!(path.metadata().unwrap().len() > 0);
        let _ = std::fs::remove_file(&path);
    }

    #[test]
    fn test_write_all_step_multiple_shapes() {
        let s1 = Shape::box_centered(10.0, 10.0, 10.0);
        let s2 = Shape::sphere(5.0).at(glam::DVec3::new(20.0, 0.0, 0.0)).build();
        let s3 = Shape::cylinder_radius_height(3.0, 15.0);
        let path = std::env::temp_dir().join("test_write_all_step_multi.step");
        let result = Shape::write_all_step([&s1, &s2, &s3], &path);
        assert!(result.is_ok());
        assert!(path.exists());
        assert!(path.metadata().unwrap().len() > 0);
        let _ = std::fs::remove_file(&path);
    }

    #[test]
    fn test_write_all_step_empty() {
        let path = std::env::temp_dir().join("test_write_all_step_empty.step");
        let result = Shape::write_all_step(std::iter::empty::<&Shape>(), &path);
        assert!(result.is_err());
        assert!(!path.exists());
    }
}
