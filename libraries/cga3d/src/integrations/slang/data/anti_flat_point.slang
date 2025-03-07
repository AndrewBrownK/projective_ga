implementing cga3d;

/// AntiFlatPoint
public struct AntiFlatPoint {
    // e235, e315, e125, e321
    internal float4 group0;

    public __init(
        float e235, float e315, float e125, float e321
    ) {
        group0 = float4(e235, e315, e125, e321);
    }
    internal __init(
        float4 g0
    ) {
        group0 = g0;
    }
}
public extension AntiFlatPoint {
    public property e235: float {
        get { return group0[0]; }
        set { group0[0] = newValue; }
    }
    public property e315: float {
        get { return group0[1]; }
        set { group0[1] = newValue; }
    }
    public property e125: float {
        get { return group0[2]; }
        set { group0[2] = newValue; }
    }
    public property e321: float {
        get { return group0[3]; }
        set { group0[3] = newValue; }
    }
}
public extension AntiFlatPoint: IComparable {
    public bool equals(AntiFlatPoint other) {
        return equalsHelper(this.group0, other.group0);
    }
    public bool lessThan(AntiFlatPoint other) {
        return lessThanHelper(this.group0, other.group0);
    }
    public bool lessThanOrEquals(AntiFlatPoint other) {
        return lessThanOrEqualsHelper(this.group0, other.group0);
    }
}
__include "impls/anti_flat_point";
