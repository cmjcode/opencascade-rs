use crate::{
    primitives::{FaceOrientation, Shape},
    Error,
};
use cxx::UniquePtr;
use glam::{dvec2, dvec3, DVec2, DVec3};
use opencascade_sys as ffi;

#[derive(Debug)]
pub struct Mesh {
    pub vertices: Vec<DVec3>,
    pub uvs: Vec<DVec2>,
    pub normals: Vec<DVec3>,
    pub indices: Vec<usize>,
}

pub struct Mesher {
    pub(crate) inner: UniquePtr<ffi::b_rep_mesh::BRepMesh_IncrementalMesh>,
}

impl Mesher {
    pub fn try_new(shape: &Shape, triangulation_tolerance: f64) -> Result<Self, Error> {
        Self::try_new_with_angular(shape, triangulation_tolerance, 0.10)
    }

    pub fn try_new_with_angular(
        shape: &Shape,
        triangulation_tolerance: f64,
        angular_deflection: f64,
    ) -> Result<Self, Error> {
        let inner = ffi::b_rep_mesh::IncrementalMesh_new_with_angular(
            &shape.inner,
            triangulation_tolerance,
            false,
            angular_deflection,
            false,
        );

        if inner.IsDone() {
            Ok(Self { inner })
        } else {
            Err(Error::TriangulationFailed)
        }
    }

    pub fn mesh(mut self) -> Result<Mesh, Error> {
        let mut vertices = vec![];
        let mut uvs = vec![];
        let mut normals = vec![];
        let mut indices = vec![];

        let triangulated_shape = Shape::from_shape(self.inner.pin_mut().Shape());

        for face in triangulated_shape.faces() {
            let mut location = ffi::top_loc::Location_new();

            let triangulation_handle =
                ffi::b_rep::BRep_Tool_Triangulation(&face.inner, location.pin_mut());

            let triangulation = ffi::poly::Handle_Poly_Triangulation_Get(&triangulation_handle)
                .map_err(|_| Error::UntriangulatedFace)?;

            let index_offset = vertices.len();
            let face_point_count = triangulation.NbNodes();

            let trsf = ffi::top_loc::TopLoc_Location_Transformation(&location);

            for i in 1..=face_point_count {
                let mut point = ffi::poly::Poly_Triangulation_Node(triangulation, i);
                point.pin_mut().Transform(&trsf);
                vertices.push(dvec3(point.X(), point.Y(), point.Z()));
            }

            let mut u_min = f64::INFINITY;
            let mut v_min = f64::INFINITY;

            let mut u_max = f64::NEG_INFINITY;
            let mut v_max = f64::NEG_INFINITY;

            for i in 1..=(face_point_count) {
                let uv = ffi::poly::Poly_Triangulation_UV(triangulation, i);
                let (u, v) = (uv.X(), uv.Y());

                u_min = u_min.min(u);
                v_min = v_min.min(v);

                u_max = u_max.max(u);
                v_max = v_max.max(v);

                uvs.push(dvec2(u, v));
            }

            // Normalize the newly added UV coordinates.
            for uv in &mut uvs[index_offset..(index_offset + face_point_count as usize)] {
                uv.x = (uv.x - u_min) / (u_max - u_min);
                uv.y = (uv.y - v_min) / (v_max - v_min);

                if face.orientation() != FaceOrientation::Forward {
                    uv.x = 1.0 - uv.x;
                }
            }

            // Compute per-vertex normals from the triangulation surface geometry.
            // ComputeNormals populates the normal array stored inside `triangulation`.
            ffi::b_rep_lib::BRepLib_ToolTriangulatedShape::ComputeNormals(
                &face.inner,
                &triangulation_handle,
            );

            // CRITICAL: loop range must be `1..=face_point_count` (inclusive) to match
            // the vertex loop above exactly. Using `1..(array.Length())` was off-by-one:
            // it produced 1..face_point_count (exclusive), missing the last normal per
            // face and causing every normal after index 0 to map to the wrong vertex.
            for i in 1..=face_point_count {
                let mut normal = ffi::poly::Poly_Triangulation_Normal(triangulation, i);
                normal.pin_mut().Transform(&trsf);
                normals.push(dvec3(normal.X(), normal.Y(), normal.Z()));
            }

            for i in 1..=triangulation.NbTriangles() {
                let triangle = triangulation.Triangle(i);

                if face.orientation() == FaceOrientation::Forward {
                    indices.push(index_offset + triangle.Value(1) as usize - 1);
                    indices.push(index_offset + triangle.Value(2) as usize - 1);
                    indices.push(index_offset + triangle.Value(3) as usize - 1);
                } else {
                    indices.push(index_offset + triangle.Value(3) as usize - 1);
                    indices.push(index_offset + triangle.Value(2) as usize - 1);
                    indices.push(index_offset + triangle.Value(1) as usize - 1);
                }
            }
        }

        Ok(Mesh { vertices, uvs, normals, indices })
    }
}
