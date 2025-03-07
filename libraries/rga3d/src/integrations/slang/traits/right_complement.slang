implementing rga3d;

/// RightComplement
/// The RightComplement of a BasisElement is the missing BasisElement that when wedged together will create the AntiScalar. For example, with an AntiScalar of e1234, the right_complement(e1) = e234, because wedge(e1, e234) = e1234. In this example, the right_complement(e234) = -e1, because wedge(e234, -e1) = e1234. See also LeftComplement and DoubleComplement. The LeftComplement can be used to undo a RightComplement.
public static const RightComplementPrefixOrPostfix right_complement = RightComplementPrefixOrPostfix();
public struct RightComplementPrefixOrPostfix {}
__include "./impls/right_complement";
