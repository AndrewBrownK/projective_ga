implementing cga3d;

/// WeightContraction
/// This is an interior product (contrast with inner product and exterior product). The interior products are derived by Wedging (or AntiWedging) one object with the Dual (or AntiDual) of another object.
public static const WeightContractionInfix weight_contraction = WeightContractionInfix();
public struct WeightContractionInfix {}
public struct WeightContractionInfixPartial<A> { internal A a; }
__include "./impls/weight_contraction";
