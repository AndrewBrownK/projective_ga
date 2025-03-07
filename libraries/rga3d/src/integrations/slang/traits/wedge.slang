implementing rga3d;

/// Wedge
/// The Wedge product (also known as "Exterior Product" or Grassmann's "Progressive Combinatorial Product") combines BasisElements into higher grade BasisElements. For example, wedge(e1, e2) = e12, and wedge(e1, e23) = e123. The Wedge product is anti-commutative, so wedge(a, b) = -wedge(b, a). A non-scalar element wedged with itself is zero. This behaves something like a union of the subscripts in the BasisElements.
public static const WedgeInfix wedge = WedgeInfix();
public struct WedgeInfix {}
public struct WedgeInfixPartial<A> { internal A a; }
__include "./impls/wedge";
