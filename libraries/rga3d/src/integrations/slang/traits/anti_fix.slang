implementing rga3d;

/// AntiFix
/// Automatically fix the anti geometric constraint by adjusting the bulk to comply with the weight, and then weight normalize the result.
public static const AntiFixPrefixOrPostfix anti_fix = AntiFixPrefixOrPostfix();
public struct AntiFixPrefixOrPostfix {}
__include "./impls/anti_fix";
