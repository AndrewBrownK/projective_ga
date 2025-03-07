implementing rga3d;

/// DualNum
public struct DualNum {
    // scalar, e1234, 0, 0
    internal float4 group0;

    public __init(
        float scalar, float e1234
    ) {
        group0 = float4(scalar, e1234, 0.0, 0.0);
    }
    internal __init(
        float2 g0
    ) {
        group0 = float4(g0, 0.0, 0.0);
    }
}
public extension DualNum {
    public property scalar: float {
        get { return group0[0]; }
        set { group0[0] = newValue; }
    }
    public property e1234: float {
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
