implementing rga3d;

/// GeometricAntiQuotient
/// AntiProduct of A with AntiInverse of B.
public static const GeometricAntiQuotientInfix geometric_anti_quotient = GeometricAntiQuotientInfix();
public struct GeometricAntiQuotientInfix {}
public struct GeometricAntiQuotientInfixPartial<A> { internal A a; }
__include "./impls/geometric_anti_quotient";
