implementing rga3d;

/// ConstraintViolation
/// Not every combinations of floats is valid geometry. Some types of objects are required to fulfill a constraint in order to be valid geometry. We call this the geometric constraint. If a type of object may possibly violate this constraint, then it will implement this trait. The constraint is violated if a non-zero value is returned. See also ConstraintValid and Fix.
public static const ConstraintViolationPrefixOrPostfix constraint_violation = ConstraintViolationPrefixOrPostfix();
public struct ConstraintViolationPrefixOrPostfix {}
__include "./impls/constraint_violation";
