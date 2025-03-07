implementing rga3d;

/// RejectViaOriginFrom
/// Counterpart to ProjectViaOriginOnto.
public static const RejectViaOriginFromInfix reject_via_origin_from = RejectViaOriginFromInfix();
public struct RejectViaOriginFromInfix {}
public struct RejectViaOriginFromInfixPartial<A> { internal A a; }
__include "./impls/reject_via_origin_from";
