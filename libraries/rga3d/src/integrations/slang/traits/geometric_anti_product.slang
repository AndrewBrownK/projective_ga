implementing rga3d;

/// GeometricAntiProduct
/// The GeometricAntiProduct or sometimes called AntiProduct is the dual to the GeometricProduct. It depends on a specified AntiScalar. Anti-Multiplying uniform grade geometry may result in mixed grade anti-products, bottoming out at the Scalar. See also AntiSandwich.
public static const GeometricAntiProductInfix geometric_anti_product = GeometricAntiProductInfix();
public struct GeometricAntiProductInfix {}
public struct GeometricAntiProductInfixPartial<A> { internal A a; }
__include "./impls/geometric_anti_product";
