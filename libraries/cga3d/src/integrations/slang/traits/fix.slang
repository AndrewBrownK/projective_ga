implementing cga3d;

/// Fix
/// Automatically fix the geometric constraint by adjusting the weight to comply with the bulk, and then bulk normalize the result.
public static const FixPrefixOrPostfix fix = FixPrefixOrPostfix();
public struct FixPrefixOrPostfix {}
__include "./impls/fix";
