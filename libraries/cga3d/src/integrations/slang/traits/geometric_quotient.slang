implementing cga3d;

/// GeometricQuotient
/// Product of A with Inverse of B.
public static const GeometricQuotientInfix geometric_quotient = GeometricQuotientInfix();
public struct GeometricQuotientInfix {}
public struct GeometricQuotientInfixPartial<A> { internal A a; }
__include "./impls/geometric_quotient";
