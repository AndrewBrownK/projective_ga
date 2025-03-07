implementing rga3d;

/// Sandwich
/// The so-called "sandwich product" squeezes some factor A between another factor B and the reversal of B. This is frequently used to represent geometric transformations, for example reflecting across a plane or rotating around a line.
public static const SandwichInfix sandwich = SandwichInfix();
public struct SandwichInfix {}
public struct SandwichInfixPartial<A> { internal A a; }
__include "./impls/sandwich";
