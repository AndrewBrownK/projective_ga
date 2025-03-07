implementing rga3d;

/// SquareRoot
/// Square root of geometry. Multiple different types of geometry might square to one type of geometry, so this is only defined for types that are closed with themselves under the geometric product.
public static const SquareRootPrefixOrPostfix square_root = SquareRootPrefixOrPostfix();
public struct SquareRootPrefixOrPostfix {}
__include "./impls/square_root";
