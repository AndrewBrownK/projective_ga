implementing cga3d;

/// AntiConstraintValid
/// Implementors of this trait cannot violate the anti geometric constraint. They always represent valid geometry. This trait does not exist to perform any calculation, it just exists to serve as contrasting information side-by-side with AntiConstraintViolation. See also AntiConstraintViolation and AntiFix.
public static const AntiConstraintValidPrefixOrPostfix anti_constraint_valid = AntiConstraintValidPrefixOrPostfix();
public struct AntiConstraintValidPrefixOrPostfix {}
__include "./impls/anti_constraint_valid";
