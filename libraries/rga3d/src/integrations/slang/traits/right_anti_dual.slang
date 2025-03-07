implementing rga3d;

/// RightAntiDual
/// This dual is the "AntiMetric Right Dual". To take this dual of an object, the object is multiplied by the anti-metric, and then we take the right complement. This will turn a Scalar into an AntiScalar, a Vector into an AntiVector, so on, and vice versa. The use of the anti-metric may give distinct results from a simple right complement, typically by changing the coefficient on some terms (1, -1, or 0)
public static const RightAntiDualPrefixOrPostfix right_anti_dual = RightAntiDualPrefixOrPostfix();
public struct RightAntiDualPrefixOrPostfix {}
__include "./impls/right_anti_dual";
