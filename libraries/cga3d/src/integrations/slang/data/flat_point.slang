implementing cga3d;

/// FlatPoint
public struct FlatPoint {
    // e15, e25, e35, e45
    internal float4 group0;

    public __init(
        float e15, float e25, float e35, float e45
    ) {
        group0 = float4(e15, e25, e35, e45);
    }
    internal __init(
        float4 g0
    ) {
        group0 = g0;
    }
}
public extension FlatPoint {
    public property e15: float {
        get { return group0[0]; }
        set { group0[0] = newValue; }
    }
    public property e25: float {
        get { return group0[1]; }
        set { group0[1] = newValue; }
    }
    public property e35: float {
        get { return group0[2]; }
        set { group0[2] = newValue; }
    }
    public property e45: float {
        get { return group0[3]; }
        set { group0[3] = newValue; }
    }
}
public extension FlatPoint: IComparable {
    public bool equals(FlatPoint other) {
        return equalsHelper(this.group0, other.group0);
    }
    public bool lessThan(FlatPoint other) {
        return lessThanHelper(this.group0, other.group0);
    }
    public bool lessThanOrEquals(FlatPoint other) {
        return lessThanOrEqualsHelper(this.group0, other.group0);
    }
}
__include "impls/flat_point";
