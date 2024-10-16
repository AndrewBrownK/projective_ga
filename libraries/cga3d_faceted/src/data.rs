pub mod base {
    pub use crate::data::anti_plane::AntiPlane;
    pub use crate::data::anti_plane_on_origin::AntiPlaneOnOrigin;
    pub use crate::data::anti_sphere_on_origin::AntiSphereOnOrigin;
    pub use crate::data::infinity::Infinity;
    pub use crate::data::origin::Origin;
    pub use crate::data::round_point::RoundPoint;
    pub use crate::data::round_point_at_origin::RoundPointAtOrigin;
}
pub mod join_0 {
    pub use crate::data::anti_plane::AntiPlane;
    pub use crate::data::anti_plane_on_origin::AntiPlaneOnOrigin;
    pub use crate::data::anti_sphere_on_origin::AntiSphereOnOrigin;
    pub use crate::data::infinity::Infinity;
    pub use crate::data::origin::Origin;
    pub use crate::data::round_point::RoundPoint;
    pub use crate::data::round_point_at_origin::RoundPointAtOrigin;
}
pub mod join_1 {
    pub use crate::data::anti_circle_on_origin::AntiCircleOnOrigin;
    pub use crate::data::anti_line::AntiLine;
    pub use crate::data::anti_line_on_origin::AntiLineOnOrigin;
    pub use crate::data::dipole::Dipole;
    pub use crate::data::dipole_aligning_origin::DipoleAligningOrigin;
    pub use crate::data::dipole_at_infinity::DipoleAtInfinity;
    pub use crate::data::dipole_at_origin::DipoleAtOrigin;
    pub use crate::data::dipole_on_origin::DipoleOnOrigin;
    pub use crate::data::dipole_orthogonal_origin::DipoleOrthogonalOrigin;
    pub use crate::data::flat_origin::FlatOrigin;
    pub use crate::data::flat_point::FlatPoint;
    pub use crate::data::flat_point_at_infinity::FlatPointAtInfinity;
    pub use crate::data::mystery_dipole::MysteryDipole;
    pub use crate::data::null_dipole_at_origin::NullDipoleAtOrigin;
}
pub mod join_2 {
    pub use crate::data::anti_dipole_on_origin::AntiDipoleOnOrigin;
    pub use crate::data::anti_flat_origin::AntiFlatOrigin;
    pub use crate::data::anti_flat_point::AntiFlatPoint;
    pub use crate::data::circle::Circle;
    pub use crate::data::circle_aligning_origin::CircleAligningOrigin;
    pub use crate::data::circle_at_infinity::CircleAtInfinity;
    pub use crate::data::circle_at_origin::CircleAtOrigin;
    pub use crate::data::circle_on_origin::CircleOnOrigin;
    pub use crate::data::circle_orthogonal_origin::CircleOrthogonalOrigin;
    pub use crate::data::line::Line;
    pub use crate::data::line_at_infinity::LineAtInfinity;
    pub use crate::data::line_on_origin::LineOnOrigin;
    pub use crate::data::mystery_circle::MysteryCircle;
    pub use crate::data::null_circle_at_origin::NullCircleAtOrigin;
}
pub mod join_3 {
    pub use crate::data::horizon::Horizon;
    pub use crate::data::null_sphere_at_origin::NullSphereAtOrigin;
    pub use crate::data::plane::Plane;
    pub use crate::data::plane_on_origin::PlaneOnOrigin;
    pub use crate::data::sphere::Sphere;
    pub use crate::data::sphere_at_origin::SphereAtOrigin;
    pub use crate::data::sphere_on_origin::SphereOnOrigin;
}
pub mod meet_0 {
    pub use crate::data::horizon::Horizon;
    pub use crate::data::null_sphere_at_origin::NullSphereAtOrigin;
    pub use crate::data::plane::Plane;
    pub use crate::data::plane_on_origin::PlaneOnOrigin;
    pub use crate::data::sphere::Sphere;
    pub use crate::data::sphere_at_origin::SphereAtOrigin;
    pub use crate::data::sphere_on_origin::SphereOnOrigin;
}
pub mod meet_1 {
    pub use crate::data::anti_dipole_on_origin::AntiDipoleOnOrigin;
    pub use crate::data::anti_flat_origin::AntiFlatOrigin;
    pub use crate::data::anti_flat_point::AntiFlatPoint;
    pub use crate::data::circle::Circle;
    pub use crate::data::circle_aligning_origin::CircleAligningOrigin;
    pub use crate::data::circle_at_infinity::CircleAtInfinity;
    pub use crate::data::circle_at_origin::CircleAtOrigin;
    pub use crate::data::circle_on_origin::CircleOnOrigin;
    pub use crate::data::circle_orthogonal_origin::CircleOrthogonalOrigin;
    pub use crate::data::line::Line;
    pub use crate::data::line_at_infinity::LineAtInfinity;
    pub use crate::data::line_on_origin::LineOnOrigin;
    pub use crate::data::mystery_circle::MysteryCircle;
    pub use crate::data::null_circle_at_origin::NullCircleAtOrigin;
}
pub mod meet_2 {
    pub use crate::data::anti_circle_on_origin::AntiCircleOnOrigin;
    pub use crate::data::anti_line::AntiLine;
    pub use crate::data::anti_line_on_origin::AntiLineOnOrigin;
    pub use crate::data::dipole::Dipole;
    pub use crate::data::dipole_aligning_origin::DipoleAligningOrigin;
    pub use crate::data::dipole_at_infinity::DipoleAtInfinity;
    pub use crate::data::dipole_at_origin::DipoleAtOrigin;
    pub use crate::data::dipole_on_origin::DipoleOnOrigin;
    pub use crate::data::dipole_orthogonal_origin::DipoleOrthogonalOrigin;
    pub use crate::data::flat_origin::FlatOrigin;
    pub use crate::data::flat_point::FlatPoint;
    pub use crate::data::flat_point_at_infinity::FlatPointAtInfinity;
    pub use crate::data::mystery_dipole::MysteryDipole;
    pub use crate::data::null_dipole_at_origin::NullDipoleAtOrigin;
}
pub mod meet_3 {
    pub use crate::data::anti_plane::AntiPlane;
    pub use crate::data::anti_plane_on_origin::AntiPlaneOnOrigin;
    pub use crate::data::anti_sphere_on_origin::AntiSphereOnOrigin;
    pub use crate::data::infinity::Infinity;
    pub use crate::data::origin::Origin;
    pub use crate::data::round_point::RoundPoint;
    pub use crate::data::round_point_at_origin::RoundPointAtOrigin;
}
pub mod reflection_0 {
    pub use crate::data::anti_scalar::AntiScalar;
}
pub mod reflection_1 {
    pub use crate::data::horizon::Horizon;
    pub use crate::data::null_sphere_at_origin::NullSphereAtOrigin;
    pub use crate::data::plane::Plane;
    pub use crate::data::plane_on_origin::PlaneOnOrigin;
    pub use crate::data::sphere::Sphere;
    pub use crate::data::sphere_at_origin::SphereAtOrigin;
    pub use crate::data::sphere_on_origin::SphereOnOrigin;
}
pub mod reflection_2 {
    pub use crate::data::anti_dipole_on_origin::AntiDipoleOnOrigin;
    pub use crate::data::anti_flat_origin::AntiFlatOrigin;
    pub use crate::data::anti_flat_point::AntiFlatPoint;
    pub use crate::data::circle::Circle;
    pub use crate::data::circle_aligning_origin::CircleAligningOrigin;
    pub use crate::data::circle_at_infinity::CircleAtInfinity;
    pub use crate::data::circle_at_origin::CircleAtOrigin;
    pub use crate::data::circle_on_origin::CircleOnOrigin;
    pub use crate::data::circle_orthogonal_origin::CircleOrthogonalOrigin;
    pub use crate::data::circle_rotor::CircleRotor;
    pub use crate::data::circle_rotor_aligning_origin::CircleRotorAligningOrigin;
    pub use crate::data::circle_rotor_aligning_origin_at_infinity::CircleRotorAligningOriginAtInfinity;
    pub use crate::data::circle_rotor_at_infinity::CircleRotorAtInfinity;
    pub use crate::data::circle_rotor_on_origin::CircleRotorOnOrigin;
    pub use crate::data::line::Line;
    pub use crate::data::line_at_infinity::LineAtInfinity;
    pub use crate::data::line_on_origin::LineOnOrigin;
    pub use crate::data::motor_on_origin::MotorOnOrigin;
    pub use crate::data::mystery_circle::MysteryCircle;
    pub use crate::data::mystery_circle_rotor::MysteryCircleRotor;
    pub use crate::data::null_circle_at_origin::NullCircleAtOrigin;
}
pub mod reflection_3 {
    pub use crate::data::anti_circle_on_origin::AntiCircleOnOrigin;
    pub use crate::data::anti_line::AntiLine;
    pub use crate::data::anti_line_on_origin::AntiLineOnOrigin;
    pub use crate::data::dipole::Dipole;
    pub use crate::data::dipole_aligning_origin::DipoleAligningOrigin;
    pub use crate::data::dipole_at_infinity::DipoleAtInfinity;
    pub use crate::data::dipole_at_origin::DipoleAtOrigin;
    pub use crate::data::dipole_inversion::DipoleInversion;
    pub use crate::data::dipole_inversion_aligning_origin::DipoleInversionAligningOrigin;
    pub use crate::data::dipole_inversion_at_infinity::DipoleInversionAtInfinity;
    pub use crate::data::dipole_inversion_at_origin::DipoleInversionAtOrigin;
    pub use crate::data::dipole_inversion_on_origin::DipoleInversionOnOrigin;
    pub use crate::data::dipole_inversion_orthogonal_origin::DipoleInversionOrthogonalOrigin;
    pub use crate::data::dipole_on_origin::DipoleOnOrigin;
    pub use crate::data::dipole_orthogonal_origin::DipoleOrthogonalOrigin;
    pub use crate::data::flat_origin::FlatOrigin;
    pub use crate::data::flat_point::FlatPoint;
    pub use crate::data::flat_point_at_infinity::FlatPointAtInfinity;
    pub use crate::data::flector::Flector;
    pub use crate::data::flector_at_infinity::FlectorAtInfinity;
    pub use crate::data::flector_on_origin::FlectorOnOrigin;
    pub use crate::data::mystery_dipole::MysteryDipole;
    pub use crate::data::mystery_dipole_inversion::MysteryDipoleInversion;
    pub use crate::data::null_dipole_at_origin::NullDipoleAtOrigin;
    pub use crate::data::null_dipole_inversion_at_origin::NullDipoleInversionAtOrigin;
}
pub mod reflection_4 {
    pub use crate::data::anti_dipole_inversion::AntiDipoleInversion;
    pub use crate::data::anti_dipole_inversion_at_infinity::AntiDipoleInversionAtInfinity;
    pub use crate::data::anti_dipole_inversion_on_origin::AntiDipoleInversionOnOrigin;
    pub use crate::data::anti_dipole_inversion_orthogonal_origin::AntiDipoleInversionOrthogonalOrigin;
    pub use crate::data::anti_flector::AntiFlector;
    pub use crate::data::anti_flector_on_origin::AntiFlectorOnOrigin;
    pub use crate::data::anti_mystery_dipole_inversion::AntiMysteryDipoleInversion;
    pub use crate::data::anti_plane::AntiPlane;
    pub use crate::data::anti_plane_on_origin::AntiPlaneOnOrigin;
    pub use crate::data::anti_sphere_on_origin::AntiSphereOnOrigin;
    pub use crate::data::dual_num::DualNum;
    pub use crate::data::infinity::Infinity;
    pub use crate::data::motor::Motor;
    pub use crate::data::motor_at_infinity::MotorAtInfinity;
    pub use crate::data::mystery_versor_even::MysteryVersorEven;
    pub use crate::data::null_versor_even_at_origin::NullVersorEvenAtOrigin;
    pub use crate::data::origin::Origin;
    pub use crate::data::round_point::RoundPoint;
    pub use crate::data::round_point_at_origin::RoundPointAtOrigin;
    pub use crate::data::versor_even::VersorEven;
    pub use crate::data::versor_even_aligning_origin::VersorEvenAligningOrigin;
    pub use crate::data::versor_even_at_infinity::VersorEvenAtInfinity;
    pub use crate::data::versor_even_at_origin::VersorEvenAtOrigin;
    pub use crate::data::versor_even_on_origin::VersorEvenOnOrigin;
    pub use crate::data::versor_even_orthogonal_origin::VersorEvenOrthogonalOrigin;
}
pub mod reflection_5 {
    pub use crate::data::anti_circle_rotor::AntiCircleRotor;
    pub use crate::data::anti_circle_rotor_aligning_origin::AntiCircleRotorAligningOrigin;
    pub use crate::data::anti_circle_rotor_aligning_origin_at_infinity::AntiCircleRotorAligningOriginAtInfinity;
    pub use crate::data::anti_circle_rotor_at_infinity::AntiCircleRotorAtInfinity;
    pub use crate::data::anti_circle_rotor_on_origin::AntiCircleRotorOnOrigin;
    pub use crate::data::anti_dual_num::AntiDualNum;
    pub use crate::data::anti_motor::AntiMotor;
    pub use crate::data::anti_motor_on_origin::AntiMotorOnOrigin;
    pub use crate::data::anti_mystery_circle_rotor::AntiMysteryCircleRotor;
    pub use crate::data::anti_versor_even_on_origin::AntiVersorEvenOnOrigin;
    pub use crate::data::mystery_versor_odd::MysteryVersorOdd;
    pub use crate::data::scalar::Scalar;
    pub use crate::data::versor_odd::VersorOdd;
    pub use crate::data::versor_odd_at_infinity::VersorOddAtInfinity;
    pub use crate::data::versor_odd_orthogonal_origin::VersorOddOrthogonalOrigin;
}
pub mod vector_0 {
    pub use crate::data::scalar::Scalar;
}
pub mod vector_1 {
    pub use crate::data::anti_plane::AntiPlane;
    pub use crate::data::anti_plane_on_origin::AntiPlaneOnOrigin;
    pub use crate::data::anti_sphere_on_origin::AntiSphereOnOrigin;
    pub use crate::data::infinity::Infinity;
    pub use crate::data::origin::Origin;
    pub use crate::data::round_point::RoundPoint;
    pub use crate::data::round_point_at_origin::RoundPointAtOrigin;
}
pub mod vector_2 {
    pub use crate::data::anti_circle_on_origin::AntiCircleOnOrigin;
    pub use crate::data::anti_line::AntiLine;
    pub use crate::data::anti_line_on_origin::AntiLineOnOrigin;
    pub use crate::data::dipole::Dipole;
    pub use crate::data::dipole_aligning_origin::DipoleAligningOrigin;
    pub use crate::data::dipole_at_infinity::DipoleAtInfinity;
    pub use crate::data::dipole_at_origin::DipoleAtOrigin;
    pub use crate::data::dipole_on_origin::DipoleOnOrigin;
    pub use crate::data::dipole_orthogonal_origin::DipoleOrthogonalOrigin;
    pub use crate::data::flat_origin::FlatOrigin;
    pub use crate::data::flat_point::FlatPoint;
    pub use crate::data::flat_point_at_infinity::FlatPointAtInfinity;
    pub use crate::data::mystery_dipole::MysteryDipole;
    pub use crate::data::null_dipole_at_origin::NullDipoleAtOrigin;
}
pub mod vector_3 {
    pub use crate::data::anti_dipole_on_origin::AntiDipoleOnOrigin;
    pub use crate::data::anti_flat_origin::AntiFlatOrigin;
    pub use crate::data::anti_flat_point::AntiFlatPoint;
    pub use crate::data::circle::Circle;
    pub use crate::data::circle_aligning_origin::CircleAligningOrigin;
    pub use crate::data::circle_at_infinity::CircleAtInfinity;
    pub use crate::data::circle_at_origin::CircleAtOrigin;
    pub use crate::data::circle_on_origin::CircleOnOrigin;
    pub use crate::data::circle_orthogonal_origin::CircleOrthogonalOrigin;
    pub use crate::data::line::Line;
    pub use crate::data::line_at_infinity::LineAtInfinity;
    pub use crate::data::line_on_origin::LineOnOrigin;
    pub use crate::data::mystery_circle::MysteryCircle;
    pub use crate::data::null_circle_at_origin::NullCircleAtOrigin;
}
pub mod vector_4 {
    pub use crate::data::horizon::Horizon;
    pub use crate::data::null_sphere_at_origin::NullSphereAtOrigin;
    pub use crate::data::plane::Plane;
    pub use crate::data::plane_on_origin::PlaneOnOrigin;
    pub use crate::data::sphere::Sphere;
    pub use crate::data::sphere_at_origin::SphereAtOrigin;
    pub use crate::data::sphere_on_origin::SphereOnOrigin;
}
pub mod vector_5 {
    pub use crate::data::anti_scalar::AntiScalar;
}
pub mod vector_mixed {
    pub use crate::data::anti_circle_rotor::AntiCircleRotor;
    pub use crate::data::anti_circle_rotor_aligning_origin::AntiCircleRotorAligningOrigin;
    pub use crate::data::anti_circle_rotor_aligning_origin_at_infinity::AntiCircleRotorAligningOriginAtInfinity;
    pub use crate::data::anti_circle_rotor_at_infinity::AntiCircleRotorAtInfinity;
    pub use crate::data::anti_circle_rotor_on_origin::AntiCircleRotorOnOrigin;
    pub use crate::data::anti_dipole_inversion::AntiDipoleInversion;
    pub use crate::data::anti_dipole_inversion_at_infinity::AntiDipoleInversionAtInfinity;
    pub use crate::data::anti_dipole_inversion_on_origin::AntiDipoleInversionOnOrigin;
    pub use crate::data::anti_dipole_inversion_orthogonal_origin::AntiDipoleInversionOrthogonalOrigin;
    pub use crate::data::anti_dual_num::AntiDualNum;
    pub use crate::data::anti_flector::AntiFlector;
    pub use crate::data::anti_flector_on_origin::AntiFlectorOnOrigin;
    pub use crate::data::anti_motor::AntiMotor;
    pub use crate::data::anti_motor_on_origin::AntiMotorOnOrigin;
    pub use crate::data::anti_mystery_circle_rotor::AntiMysteryCircleRotor;
    pub use crate::data::anti_mystery_dipole_inversion::AntiMysteryDipoleInversion;
    pub use crate::data::anti_versor_even_on_origin::AntiVersorEvenOnOrigin;
    pub use crate::data::circle_rotor::CircleRotor;
    pub use crate::data::circle_rotor_aligning_origin::CircleRotorAligningOrigin;
    pub use crate::data::circle_rotor_aligning_origin_at_infinity::CircleRotorAligningOriginAtInfinity;
    pub use crate::data::circle_rotor_at_infinity::CircleRotorAtInfinity;
    pub use crate::data::circle_rotor_on_origin::CircleRotorOnOrigin;
    pub use crate::data::dipole_inversion::DipoleInversion;
    pub use crate::data::dipole_inversion_aligning_origin::DipoleInversionAligningOrigin;
    pub use crate::data::dipole_inversion_at_infinity::DipoleInversionAtInfinity;
    pub use crate::data::dipole_inversion_at_origin::DipoleInversionAtOrigin;
    pub use crate::data::dipole_inversion_on_origin::DipoleInversionOnOrigin;
    pub use crate::data::dipole_inversion_orthogonal_origin::DipoleInversionOrthogonalOrigin;
    pub use crate::data::dual_num::DualNum;
    pub use crate::data::flector::Flector;
    pub use crate::data::flector_at_infinity::FlectorAtInfinity;
    pub use crate::data::flector_on_origin::FlectorOnOrigin;
    pub use crate::data::motor::Motor;
    pub use crate::data::motor_at_infinity::MotorAtInfinity;
    pub use crate::data::motor_on_origin::MotorOnOrigin;
    pub use crate::data::multi_vector::MultiVector;
    pub use crate::data::mystery_circle_rotor::MysteryCircleRotor;
    pub use crate::data::mystery_dipole_inversion::MysteryDipoleInversion;
    pub use crate::data::mystery_versor_even::MysteryVersorEven;
    pub use crate::data::mystery_versor_odd::MysteryVersorOdd;
    pub use crate::data::null_dipole_inversion_at_origin::NullDipoleInversionAtOrigin;
    pub use crate::data::null_versor_even_at_origin::NullVersorEvenAtOrigin;
    pub use crate::data::versor_even::VersorEven;
    pub use crate::data::versor_even_aligning_origin::VersorEvenAligningOrigin;
    pub use crate::data::versor_even_at_infinity::VersorEvenAtInfinity;
    pub use crate::data::versor_even_at_origin::VersorEvenAtOrigin;
    pub use crate::data::versor_even_on_origin::VersorEvenOnOrigin;
    pub use crate::data::versor_even_orthogonal_origin::VersorEvenOrthogonalOrigin;
    pub use crate::data::versor_odd::VersorOdd;
    pub use crate::data::versor_odd_at_infinity::VersorOddAtInfinity;
    pub use crate::data::versor_odd_orthogonal_origin::VersorOddOrthogonalOrigin;
}
mod anti_circle_on_origin;
pub use anti_circle_on_origin::AntiCircleOnOrigin;
mod anti_circle_rotor;
pub use anti_circle_rotor::AntiCircleRotor;
mod anti_circle_rotor_aligning_origin;
pub use anti_circle_rotor_aligning_origin::AntiCircleRotorAligningOrigin;
mod anti_circle_rotor_aligning_origin_at_infinity;
pub use anti_circle_rotor_aligning_origin_at_infinity::AntiCircleRotorAligningOriginAtInfinity;
mod anti_circle_rotor_at_infinity;
pub use anti_circle_rotor_at_infinity::AntiCircleRotorAtInfinity;
mod anti_circle_rotor_on_origin;
pub use anti_circle_rotor_on_origin::AntiCircleRotorOnOrigin;
mod anti_dipole_inversion;
pub use anti_dipole_inversion::AntiDipoleInversion;
mod anti_dipole_inversion_at_infinity;
pub use anti_dipole_inversion_at_infinity::AntiDipoleInversionAtInfinity;
mod anti_dipole_inversion_on_origin;
pub use anti_dipole_inversion_on_origin::AntiDipoleInversionOnOrigin;
mod anti_dipole_inversion_orthogonal_origin;
pub use anti_dipole_inversion_orthogonal_origin::AntiDipoleInversionOrthogonalOrigin;
mod anti_dipole_on_origin;
pub use anti_dipole_on_origin::AntiDipoleOnOrigin;
mod anti_dual_num;
pub use anti_dual_num::AntiDualNum;
mod anti_flat_origin;
pub use anti_flat_origin::AntiFlatOrigin;
mod anti_flat_point;
pub use anti_flat_point::AntiFlatPoint;
mod anti_flector;
pub use anti_flector::AntiFlector;
mod anti_flector_on_origin;
pub use anti_flector_on_origin::AntiFlectorOnOrigin;
mod anti_line;
pub use anti_line::AntiLine;
mod anti_line_on_origin;
pub use anti_line_on_origin::AntiLineOnOrigin;
mod anti_motor;
pub use anti_motor::AntiMotor;
mod anti_motor_on_origin;
pub use anti_motor_on_origin::AntiMotorOnOrigin;
mod anti_mystery_circle_rotor;
pub use anti_mystery_circle_rotor::AntiMysteryCircleRotor;
mod anti_mystery_dipole_inversion;
pub use anti_mystery_dipole_inversion::AntiMysteryDipoleInversion;
mod anti_plane;
pub use anti_plane::AntiPlane;
mod anti_plane_on_origin;
pub use anti_plane_on_origin::AntiPlaneOnOrigin;
mod anti_scalar;
pub use anti_scalar::AntiScalar;
mod anti_sphere_on_origin;
pub use anti_sphere_on_origin::AntiSphereOnOrigin;
mod anti_versor_even_on_origin;
pub use anti_versor_even_on_origin::AntiVersorEvenOnOrigin;
mod circle;
pub use circle::Circle;
mod circle_aligning_origin;
pub use circle_aligning_origin::CircleAligningOrigin;
mod circle_at_infinity;
pub use circle_at_infinity::CircleAtInfinity;
mod circle_at_origin;
pub use circle_at_origin::CircleAtOrigin;
mod circle_on_origin;
pub use circle_on_origin::CircleOnOrigin;
mod circle_orthogonal_origin;
pub use circle_orthogonal_origin::CircleOrthogonalOrigin;
mod circle_rotor;
pub use circle_rotor::CircleRotor;
mod circle_rotor_aligning_origin;
pub use circle_rotor_aligning_origin::CircleRotorAligningOrigin;
mod circle_rotor_aligning_origin_at_infinity;
pub use circle_rotor_aligning_origin_at_infinity::CircleRotorAligningOriginAtInfinity;
mod circle_rotor_at_infinity;
pub use circle_rotor_at_infinity::CircleRotorAtInfinity;
mod circle_rotor_on_origin;
pub use circle_rotor_on_origin::CircleRotorOnOrigin;
mod dipole;
pub use dipole::Dipole;
mod dipole_aligning_origin;
pub use dipole_aligning_origin::DipoleAligningOrigin;
mod dipole_at_infinity;
pub use dipole_at_infinity::DipoleAtInfinity;
mod dipole_at_origin;
pub use dipole_at_origin::DipoleAtOrigin;
mod dipole_inversion;
pub use dipole_inversion::DipoleInversion;
mod dipole_inversion_aligning_origin;
pub use dipole_inversion_aligning_origin::DipoleInversionAligningOrigin;
mod dipole_inversion_at_infinity;
pub use dipole_inversion_at_infinity::DipoleInversionAtInfinity;
mod dipole_inversion_at_origin;
pub use dipole_inversion_at_origin::DipoleInversionAtOrigin;
mod dipole_inversion_on_origin;
pub use dipole_inversion_on_origin::DipoleInversionOnOrigin;
mod dipole_inversion_orthogonal_origin;
pub use dipole_inversion_orthogonal_origin::DipoleInversionOrthogonalOrigin;
mod dipole_on_origin;
pub use dipole_on_origin::DipoleOnOrigin;
mod dipole_orthogonal_origin;
pub use dipole_orthogonal_origin::DipoleOrthogonalOrigin;
mod dual_num;
pub use dual_num::DualNum;
mod flat_origin;
pub use flat_origin::FlatOrigin;
mod flat_point;
pub use flat_point::FlatPoint;
mod flat_point_at_infinity;
pub use flat_point_at_infinity::FlatPointAtInfinity;
mod flector;
pub use flector::Flector;
mod flector_at_infinity;
pub use flector_at_infinity::FlectorAtInfinity;
mod flector_on_origin;
pub use flector_on_origin::FlectorOnOrigin;
mod horizon;
pub use horizon::Horizon;
mod infinity;
pub use infinity::Infinity;
mod line;
pub use line::Line;
mod line_at_infinity;
pub use line_at_infinity::LineAtInfinity;
mod line_on_origin;
pub use line_on_origin::LineOnOrigin;
mod motor;
pub use motor::Motor;
mod motor_at_infinity;
pub use motor_at_infinity::MotorAtInfinity;
mod motor_on_origin;
pub use motor_on_origin::MotorOnOrigin;
mod multi_vector;
pub use multi_vector::MultiVector;
mod mystery_circle;
pub use mystery_circle::MysteryCircle;
mod mystery_circle_rotor;
pub use mystery_circle_rotor::MysteryCircleRotor;
mod mystery_dipole;
pub use mystery_dipole::MysteryDipole;
mod mystery_dipole_inversion;
pub use mystery_dipole_inversion::MysteryDipoleInversion;
mod mystery_versor_even;
pub use mystery_versor_even::MysteryVersorEven;
mod mystery_versor_odd;
pub use mystery_versor_odd::MysteryVersorOdd;
mod null_circle_at_origin;
pub use null_circle_at_origin::NullCircleAtOrigin;
mod null_dipole_at_origin;
pub use null_dipole_at_origin::NullDipoleAtOrigin;
mod null_dipole_inversion_at_origin;
pub use null_dipole_inversion_at_origin::NullDipoleInversionAtOrigin;
mod null_sphere_at_origin;
pub use null_sphere_at_origin::NullSphereAtOrigin;
mod null_versor_even_at_origin;
pub use null_versor_even_at_origin::NullVersorEvenAtOrigin;
mod origin;
pub use origin::Origin;
mod plane;
pub use plane::Plane;
mod plane_on_origin;
pub use plane_on_origin::PlaneOnOrigin;
mod round_point;
pub use round_point::RoundPoint;
mod round_point_at_origin;
pub use round_point_at_origin::RoundPointAtOrigin;
mod scalar;
pub use scalar::Scalar;
mod sphere;
pub use sphere::Sphere;
mod sphere_at_origin;
pub use sphere_at_origin::SphereAtOrigin;
mod sphere_on_origin;
pub use sphere_on_origin::SphereOnOrigin;
mod versor_even;
pub use versor_even::VersorEven;
mod versor_even_aligning_origin;
pub use versor_even_aligning_origin::VersorEvenAligningOrigin;
mod versor_even_at_infinity;
pub use versor_even_at_infinity::VersorEvenAtInfinity;
mod versor_even_at_origin;
pub use versor_even_at_origin::VersorEvenAtOrigin;
mod versor_even_on_origin;
pub use versor_even_on_origin::VersorEvenOnOrigin;
mod versor_even_orthogonal_origin;
pub use versor_even_orthogonal_origin::VersorEvenOrthogonalOrigin;
mod versor_odd;
pub use versor_odd::VersorOdd;
mod versor_odd_at_infinity;
pub use versor_odd_at_infinity::VersorOddAtInfinity;
mod versor_odd_orthogonal_origin;
pub use versor_odd_orthogonal_origin::VersorOddOrthogonalOrigin;
