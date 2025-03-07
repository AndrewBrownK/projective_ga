implementing cga3d;

/// AntiDualNum
public struct AntiDualNum {
    // e3215, scalar, 0, 0
    internal float4 group0;

    public __init(
        float e3215, float scalar
    ) {
        group0 = float4(e3215, scalar, 0.0, 0.0);
    }
    internal __init(
        float2 g0
    ) {
        group0 = float4(g0, 0.0, 0.0);
    }
}
public extension AntiDualNum {
    public property e3215: float {
        get { return group0[0]; }
        set { group0[0] = newValue; }
    }
    public property scalar: float {
        get { return group0[1]; }
        set { group0[1] = newValue; }
    }
}
public extension AntiDualNum: IComparable {
    public bool equals(AntiDualNum other) {
        return equalsHelper(this.group0, other.group0);
    }
    public bool lessThan(AntiDualNum other) {
        return lessThanHelper(this.group0, other.group0);
    }
    public bool lessThanOrEquals(AntiDualNum other) {
        return lessThanOrEqualsHelper(this.group0, other.group0);
    }
}
__include "impls/anti_dual_num";
