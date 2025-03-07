implementing cga3d;

/// CenterNorm
/// Distance between origin and center (not yet unitized, still requires division by round weight).
public static const CenterNormPrefixOrPostfix center_norm = CenterNormPrefixOrPostfix();
public struct CenterNormPrefixOrPostfix {}
__include "./impls/center_norm";
