implementing rga3d;

/// AntiSquareRoot
/// Square root of geometry with respect to the AntiProduct. Multiple different types of geometry might anti-square to one type of geometry, so this is only defined for types that are closed with themselves under the geometric anti-product.
public static const AntiSquareRootPrefixOrPostfix anti_square_root = AntiSquareRootPrefixOrPostfix();
public struct AntiSquareRootPrefixOrPostfix {}
__include "./impls/anti_square_root";
