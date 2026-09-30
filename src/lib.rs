//! 3D math primitives built on top of [`glam`].
//!
//! * [`BoundingBox`] — a 3D axis-aligned bounding box.
//! * [`IsoTransform`] — a rigid transform: rotation followed by translation.
//! * [`Conformal3`] — a uniform scale, rotation and translation.
//! * [`DualQuat`] — a dual quaternion, for blending rigid transforms.
//! * [`Plane3`] and [`Ray3`] — planes and rays, with intersection helpers.
//! * [`MeshGen`] — positions, normals and indices for cubes, spheres and capsules.
//! * [`ColorRgba8`], [`UNorm8`] and [`UNorm16`] — compact color and fixed-point types.
//! * Extension traits for `f32`, [`Vec2`], [`Vec3`], [`Vec4`] and [`Quat`], all in [`prelude`].
//!
//! The commonly used `glam` types are re-exported, so `re_math::Vec3` is `glam::Vec3`.
//!
//! ## Example
//! ```
//! use re_math::{BoundingBox, IsoTransform, Plane3, Ray3, Vec3};
//!
//! // The bounding box of some points:
//! let points = [Vec3::new(1.0, 2.0, 3.0), Vec3::new(-1.0, 0.0, 1.0)];
//! let bbox = BoundingBox::from_points(points.into_iter());
//! assert_eq!(bbox.center(), Vec3::new(0.0, 1.0, 2.0));
//!
//! // A camera at +Z looking at the origin:
//! let view_from_world =
//!     IsoTransform::look_at_rh(Vec3::new(0.0, 0.0, 5.0), Vec3::ZERO, Vec3::Y).unwrap();
//! assert!(
//!     view_from_world
//!         .transform_point3(Vec3::ZERO)
//!         .abs_diff_eq(Vec3::new(0.0, 0.0, -5.0), 1e-6)
//! );
//!
//! // Shoot a ray down onto the ground plane:
//! let ray = Ray3::from_origin_dir(Vec3::new(0.0, 10.0, 0.0), Vec3::NEG_Y);
//! assert_eq!(ray.intersects_plane(Plane3::ZX), Vec3::ZERO);
//! ```
//!
//! `re_math` is a fork of [`macaw`](https://crates.io/crates/macaw) by [Embark Studios](https://www.embark-studios.com/).
//!
//! ## Feature flags
#![cfg_attr(feature = "document-features", doc = document_features::document_features!())]
//!

#![cfg_attr(target_arch = "spirv", feature(repr_simd, core_intrinsics))]
#![cfg_attr(target_arch = "spirv", no_std)]

mod affine3;
mod bounding_box;
#[cfg(not(target_arch = "spirv"))]
mod color_rgba8;
mod conformal;
mod dual_quat;
mod fixed;
mod float_ext;
mod iso_transform;
mod mat3_ext;
#[cfg(not(target_arch = "spirv"))]
mod mesh_gen;
mod plane3;
#[cfg(not(target_arch = "spirv"))]
mod quat_ext;
mod ray3;
mod utils;
mod vec2_ext;
mod vec3_ext;
mod vec4_ext;

pub use self::affine3::*;
pub use self::bounding_box::*;
pub use self::conformal::*;
pub use self::dual_quat::*;
pub use self::fixed::*;
pub use self::float_ext::*;
pub use self::iso_transform::*;
pub use self::mat3_ext::*;
pub use self::plane3::*;
pub use self::ray3::*;
pub use self::utils::*;
pub use self::vec2_ext::*;
pub use self::vec3_ext::*;
pub use self::vec4_ext::*;

#[cfg(not(target_arch = "spirv"))]
pub use color_rgba8::*;
#[cfg(not(target_arch = "spirv"))]
pub use mesh_gen::*;
#[cfg(not(target_arch = "spirv"))]
pub use quat_ext::*;

/// Prelude module with extension traits
pub mod prelude {
    pub use crate::FloatExt;
    pub use crate::Vec2Ext;
    pub use crate::Vec2Swizzles;
    pub use crate::Vec3Ext;
    pub use crate::Vec3Swizzles;
    pub use crate::Vec4Ext;
    pub use crate::Vec4Swizzles;

    #[cfg(not(target_arch = "spirv"))]
    pub use crate::QuatExt;
}

// Re-export main glam types.
// i32
pub use glam::IVec2;
pub use glam::IVec3;
pub use glam::IVec4;
pub use glam::ivec2;
pub use glam::ivec3;
// u32
pub use glam::UVec2;
pub use glam::UVec3;
pub use glam::UVec4;
pub use glam::uvec2;
pub use glam::uvec3;
pub use glam::uvec4;
// f32
pub use glam::Affine3A;
pub use glam::Mat2;
pub use glam::Mat3;
pub use glam::Mat3A;
pub use glam::Mat4;
pub use glam::Quat;
pub use glam::Vec2;
pub use glam::Vec3;
pub use glam::Vec3A;
pub use glam::Vec4;
pub use glam::mat2;
pub use glam::mat3;
pub use glam::mat3a;
pub use glam::mat4;
pub use glam::quat;
pub use glam::vec2;
pub use glam::vec3;
pub use glam::vec3a;
pub use glam::vec4;
// f64
pub use glam::DAffine2;
pub use glam::DAffine3;
pub use glam::DMat2;
pub use glam::DMat3;
pub use glam::DMat4;
pub use glam::DQuat;
pub use glam::DVec2;
pub use glam::DVec3;
pub use glam::DVec4;
pub use glam::dmat2;
pub use glam::dmat3;
pub use glam::dmat4;
pub use glam::dquat;
pub use glam::dvec2;
pub use glam::dvec3;
pub use glam::dvec4;
// other
pub use glam::EulerRot;
pub use glam::Vec2Swizzles;
pub use glam::Vec3Swizzles;
pub use glam::Vec4Swizzles;
