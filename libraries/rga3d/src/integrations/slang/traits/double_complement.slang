implementing rga3d;

/// DoubleComplement
/// Repeatedly taking a Complement will eventually return the original object. In geometric algebras with an even number of dimensions, double_complement(x) = right_complement(right_complement(x)) = left_complement(left_complement(x)). In geometric algebras with an odd number of dimensions, double_complement(x) = complement(complement(x)). In all cases, x = double_complement(double_complement(x)).
public static const DoubleComplementPrefixOrPostfix double_complement = DoubleComplementPrefixOrPostfix();
public struct DoubleComplementPrefixOrPostfix {}
__include "./impls/double_complement";
