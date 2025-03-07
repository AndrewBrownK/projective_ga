implementing cga3d;

/// BulkExpansion
/// This is an interior product (contrast with inner product and exterior product). The interior products are derived by Wedging (or AntiWedging) one object with the Dual (or AntiDual) of another object.
public static const BulkExpansionInfix bulk_expansion = BulkExpansionInfix();
public struct BulkExpansionInfix {}
public struct BulkExpansionInfixPartial<A> { internal A a; }
__include "./impls/bulk_expansion";
