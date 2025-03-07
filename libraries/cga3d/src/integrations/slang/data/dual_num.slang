implementing cga3d;

/// DualNum
public struct DualNum {
    // e5, e12345, 0, 0
    internal float4 group0;

    public __init(
        float e5, float e12345
    ) {
        group0 = float4(e5, e12345, 0.0, 0.0);
    }
    internal __init(
        float2 g0
    ) {
        group0 = float4(g0, 0.0, 0.0);
    }
}
public extension DualNum {
    public property e5: float {
        get { return group0[0]; }
        set { group0[0] = newValue; }
    }
    public property e12345: float {
        get { return group0[1]; }
        set { group0[1] = newValue; }
    }
}
public extension DualNum: IComparable {
    public bool equals(DualNum other) {
        return equalsHelper(this.group0, other.group0);
    }
    public bool lessThan(DualNum other) {
        return lessThanHelper(this.group0, other.group0);
    }
    public bool lessThanOrEquals(DualNum other) {
        return lessThanOrEqualsHelper(this.group0, other.group0);
    }
}
__include "impls/dual_num";
