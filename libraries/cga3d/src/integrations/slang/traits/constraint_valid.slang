implementing cga3d;

/// ConstraintValid
/// Implementors of this trait cannot violate the geometric constraint. They always represent valid geometry. This trait does not exist to perform any calculation, it just exists to serve as contrasting information side-by-side with ConstraintViolation. See also ConstraintViolation and Fix.
public static const ConstraintValidPrefixOrPostfix constraint_valid = ConstraintValidPrefixOrPostfix();
public struct ConstraintValidPrefixOrPostfix {}
__include "./impls/constraint_valid";
