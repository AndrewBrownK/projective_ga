implementing cga3d;

/// BulkContraction
/// This is an interior product (contrast with inner product and exterior product). The interior products are derived by Wedging (or AntiWedging) one object with the Dual (or AntiDual) of another object.
public static const BulkContractionInfix bulk_contraction = BulkContractionInfix();
public struct BulkContractionInfix {}
public struct BulkContractionInfixPartial<A> { internal A a; }
__include "./impls/bulk_contraction";
