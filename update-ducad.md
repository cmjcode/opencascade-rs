# DUCAD Technical Patch Changelog: opencascade-rs

This document details all technical modifications, extensions, and build configuration changes applied to the vendored `opencascade-rs` repository (`branch: ducad-patches`) to meet the requirements of the **DUCAD** parametric CAD kernel.

---

## 1. Background & Rationale

The upstream `opencascade-rs` repository ([github.com/bschwind/opencascade-rs](https://github.com/bschwind/opencascade-rs)) provides safe Rust bindings to OpenCASCADE Technology (OCCT). DUCAD requires low-level CXX FFI bridging and several capabilities not yet present in upstream:
1. **Configurable Ray-Casting Picking Tolerance:** Upstream hardcoded the geometric picking tolerance to `0.0001`, which caused oblique perspective camera 3D ray-casting to frequently return `None`.
2. **Surface & Curve Geometry Inspection:** The ability to inspect and classify face geometry (planar, cylindrical, conical, spherical) and extract analytical parameters (radius, axis, apex, normal vectors) required for sketching on curved faces, dimensioning, and constraint solving.
3. **Advanced B-Rep Feature Modeling:** Operations such as edge-specific chamfering, selective fillet radius, hollow/thick-solid (shelling) with open closing faces, sweep (pipe / pipe-shell), and arbitrary axis revolving.
4. **Topological Naming & History Tracking:** Access to OpenCASCADE B-Rep history structures (`Generated`, `Modified`, `IsDeleted`) from boolean operations to enable persistent topological face/edge tracking.
5. **Dependency Synchronization:** Upgraded to `glam 0.33` and `thiserror 2` to match the DUCAD workspace without redundant conversion overhead.

---

## 2. Workspace & Build Configuration Changes

| File | Changes | Rationale |
| :--- | :--- | :--- |
| [`Cargo.toml`](file:///Users/jayuda/Documents/PROJECT/DUCAD/ducad-editor/vendors/opencascade-rs/Cargo.toml) | Simplified `workspace.members` to `["crates/kicad-parser", "crates/opencascade", "crates/opencascade-sys"]`. Excluded `examples`, `viewer`, `model-api`, and `wasm-example`. | Eliminates compilation errors from outdated upstream demo/example crates not used by DUCAD. |
| [`crates/opencascade/Cargo.toml`](file:///Users/jayuda/Documents/PROJECT/DUCAD/ducad-editor/vendors/opencascade-rs/crates/opencascade/Cargo.toml) | Upgraded dependencies to `glam = "0.33"` and `thiserror = "2"`. | Aligns dependency versions with the root `ducad-editor` workspace. |
| [`crates/opencascade-sys/build.rs`](file:///Users/jayuda/Documents/PROJECT/DUCAD/ducad-editor/vendors/opencascade-rs/crates/opencascade-sys/build.rs) | Added `.flag_if_supported("-Wno-deprecated-declarations")` to the C++ compilation builder. | Suppresses Apple Clang SDK deprecation warnings for `sprintf` within third-party OCCT template headers on macOS. |

---

## 3. FFI Layer Modifications (`crates/opencascade-sys`)

### A. `b_rep_adaptor` (`include/b_rep_adaptor.hxx`, `src/b_rep_adaptor.rs`)
- **`BRepAdaptor_Surface`:**
  - Added constructor `BRepAdaptor_Surface_ctor(face, restriction)`.
  - Added `GetType()` binding for surface classification (`GeomAbs_SurfaceType`).
  - Added analytical geometry conversion bindings: `BRepAdaptor_Surface_cylinder`, `BRepAdaptor_Surface_cone`, `BRepAdaptor_Surface_sphere`.
- **`BRepAdaptor_Curve`:**
  - Added constructor `BRepAdaptor_Curve_new(edge)`.
  - Added `FirstParameter()`, `LastParameter()`, and curve point evaluation `BRepAdaptor_Curve_value(curve, u)`.
  - Added `GetType()` binding for curve classification (`GeomAbs_CurveType`).

### B. `b_rep_algo_api` (`include/b_rep_algo_api.hxx`, `src/b_rep_algo_api.rs`)
- **Exception-Safe Checked Operations:**
  - Added C++ exception handling wrappers: `BRepAlgoAPI_Cut_ctor_checked`, `BRepAlgoAPI_Fuse_ctor_checked`, `BRepAlgoAPI_Common_ctor_checked`, `BRepAlgoAPI_Section_ctor_checked`.
  - Added checked shape extractors: `BRepAlgoAPI_Cut_shape_checked`, etc.
- **Topological History Methods:**
  - Added bindings for `Generated()`, `Modified()`, `IsDeleted()`, `HasGenerated()`, `HasModified()`, `HasDeleted()` to track B-Rep shape lineages through boolean operations.

### C. `b_rep_builder_api` (`include/b_rep_builder_api.hxx`, `src/b_rep_builder_api.rs`)
- Added `BRepBuilderAPI_MakeFace_add_wire` binding to attach inner boundary wires (holes) to an existing face.
- Added checked constructors for `BRepBuilderAPI_Transform`.

### D. `b_rep_fillet_api` (`include/b_rep_fillet_api.hxx`, `src/b_rep_fillet_api.rs`)
- Added `BRepFilletAPI_MakeChamfer` binding for selective edge chamfering with a specified distance.
- Added `BRepFilletAPI_MakeFillet_add_edge` binding for selective edge filleting with constant radius.

### E. `b_rep_offset_api` (`include/b_rep_offset_api.hxx`, `src/b_rep_offset_api.rs`)
- Added `BRepOffsetAPI_MakeOffset` & `BRepOffsetAPI_MakeThickSolid` bindings for hollow/thick-solid operations with optional closing faces.
- Added `BRepOffsetAPI_MakePipe` & `BRepOffsetAPI_MakePipeShell` bindings for curve sweep/pipe lofting along path wires.

### F. `b_rep_prim_api` (`include/b_rep_prim_api.hxx`, `src/b_rep_prim_api.rs`)
- Added `BRepPrimAPI_MakePrism` binding (direct face/wire extrusion along a direction vector).
- Added `BRepPrimAPI_MakeRevol` binding (profile revolution about an axis).
- Added parametric cone (`MakeCone`) and torus (`MakeTorus`) constructor bindings.

### G. `geom_abs` & `gp` (`include/geom_abs.hxx`, `src/geom_abs.rs`, `include/gp.hxx`, `src/gp.rs`)
- **`geom_abs`:**
  - CXX enum declarations for `GeomAbs_SurfaceType` (`Plane`, `Cylinder`, `Cone`, `Sphere`, `Torus`, `BezierSurface`, `BSplineSurface`, etc.).
  - CXX enum declarations for `GeomAbs_CurveType` (`Line`, `Circle`, `Ellipse`, `Hyperbola`, `Parabola`, `BezierCurve`, `BSplineCurve`, etc.).
- **`gp`:**
  - Added bindings for analytical geometry entities: `gp_Cylinder`, `gp_Cone`, `gp_Sphere`, `gp_Torus`, `gp_Lin`, `gp_Circ`, `gp_Ax1`, `gp_Ax2`.
  - Property accessors for reference location, axis direction, radius, and cone semi-angle.

---

## 4. High-Level Safe Rust API (`crates/opencascade`)

### A. `primitives/shape.rs`
- **`faces_along_ray_tol(ray_origin, ray_dir, tolerance)`:**
  - Ray-casting picking with parameterized tolerance to accurately detect intersected faces from 3D camera rays.
- **Advanced B-Rep Modification Operations:**
  - `chamfer_edges(distance, edges)`: Chamfers designated edges.
  - `fillet_edges(radius, edges)`: Fillets designated edges.
  - `hollow / shell(closing_faces, thickness, tolerance)`: Generates hollowed-out thin-walled solids.
  - `sweep(spine_wire)`: Sweeps a profile along a spine wire.
  - `revolve(axis_point, axis_dir, angle_radians)`: Revolves a profile face around a 3D axis.
- **Topological Downcasting & Traversal:**
  - Safe extraction iterators for `edges()`, `faces()`, `wires()`, `solids()`, `shells()`, `vertices()`.

### B. `primitives/face.rs`
- **Geometry Classification & Inspection:**
  - `surface_type(&self) -> SurfaceType`: Returns the analytical surface type.
  - `as_cylinder(&self) -> Option<CylinderData>`: Extracts origin, axis, and radius.
  - `as_cone(&self) -> Option<ConeData>`: Extracts apex, axis, and semi-angle.
  - `as_sphere(&self) -> Option<SphereData>`: Extracts center point and radius.
  - `normal_at(&self, u, v) -> Option<DVec3>`: Computes surface normal at UV coordinates.
- **Direct Face Operations:**
  - `extrude(&self, dir: DVec3) -> Solid`: Creates an extruded solid directly from a face.
  - `outer_wire(&self) -> Wire` and `inner_wires(&self) -> Vec<Wire>`: Separates outer boundary and inner hole loops.

### C. `primitives/edge.rs`
- **Edge Curve Inspection:**
  - `curve_type(&self) -> CurveType`: Identifies line, circle, ellipse, parabola, hyperbola, or B-Spline curve.
  - `point_at_parameter(&self, u: f64) -> DVec3`: Evaluates 3D coordinates at a given parameter.
  - `length(&self) -> f64`: Computes the 3D arc length of the edge.

### D. `primitives/boolean_shape.rs`
- Exposes `history()` to allow DUCAD's topological naming engine to track persistent face and edge IDs across Cut / Fuse / Common operations.

### E. `adhoc.rs`
- Provides the `AdHocShape` helper struct for rapid procedural primitive construction (box, cylinder, sphere) with safe boolean chaining.

---

## 5. File Inventory

```
vendors/opencascade-rs/
├── Cargo.toml                                    [MODIFIED: pruned workspace members]
├── update-ducad.md                               [NEW: technical changelog documentation]
└── crates/
    ├── opencascade/
    │   ├── Cargo.toml                            [MODIFIED: glam 0.33, thiserror 2]
    │   └── src/
    │       ├── lib.rs                            [MODIFIED: re-exported new modules]
    │       ├── adhoc.rs                          [NEW: AdHocShape procedural helper]
    │       └── primitives/
    │           ├── boolean_shape.rs              [MODIFIED: history tracking]
    │           ├── edge.rs                       [MODIFIED: curve adaptor & parameter eval]
    │           ├── face.rs                       [MODIFIED: surface adaptor & analytical extraction]
    │           └── shape.rs                      [MODIFIED: ray casting tol, chamfer, fillet, shell]
    └── opencascade-sys/
        ├── build.rs                              [MODIFIED: -Wno-deprecated-declarations flag]
        ├── include/
        │   ├── b_rep_adaptor.hxx                 [MODIFIED: surface & curve adaptor headers]
        │   ├── b_rep_algo_api.hxx                [MODIFIED: checked boolean & history headers]
        │   ├── b_rep_builder_api.hxx             [MODIFIED: make face add wire headers]
        │   ├── b_rep_fillet_api.hxx              [MODIFIED: chamfer & fillet edge headers]
        │   ├── b_rep_offset_api.hxx              [MODIFIED: shell & pipe sweep headers]
        │   ├── b_rep_prim_api.hxx                [MODIFIED: prism & revol headers]
        │   ├── bindings_common.hxx               [MODIFIED: FFI exception handling macros]
        │   ├── geom_abs.hxx                      [MODIFIED: surface & curve type enums]
        │   └── gp.hxx                            [MODIFIED: analytical geometry structures]
        └── src/
            ├── b_rep_adaptor.rs                  [MODIFIED: CXX bridge adaptors]
            ├── b_rep_algo_api.rs                 [MODIFIED: CXX bridge boolean & history]
            ├── b_rep_builder_api.rs              [MODIFIED: CXX bridge builder]
            ├── b_rep_fillet_api.rs               [MODIFIED: CXX bridge fillet/chamfer]
            ├── b_rep_offset_api.rs               [MODIFIED: CXX bridge offset/shell]
            ├── b_rep_prim_api.rs                 [MODIFIED: CXX bridge prim/revol]
            ├── geom_abs.rs                       [MODIFIED: CXX bridge geom_abs]
            └── gp.rs                             [MODIFIED: CXX bridge gp entities]
```

---

## 6. Build & Verification

To verify the integration from the `ducad-editor` root directory:

```bash
# Check compilation for the entire DUCAD workspace including patched vendor crates
cargo check --workspace

# Run linter checks
cargo clippy --workspace
```

All targets compile cleanly with **0 errors and 0 warnings**.
