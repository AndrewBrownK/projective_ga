implementing cga3d;

/// WeightExpansion
/// This is an interior product (contrast with inner product and exterior product). The interior products are derived by Wedging (or AntiWedging) one object with the Dual (or AntiDual) of another object.
public static const WeightExpansionInfix weight_expansion = WeightExpansionInfix();
public struct WeightExpansionInfix {}
public struct WeightExpansionInfixPartial<A> { internal A a; }
__include "./impls/weight_expansion";
