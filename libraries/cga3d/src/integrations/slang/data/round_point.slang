implementing cga3d;

/// RoundPoint
public struct RoundPoint {
    // e1, e2, e3, e4
    internal float4 group0;
    // e5, 0, 0, 0
    internal float4 group1;

    public __init(
        float e1, float e2, float e3, float e4, float e5
    ) {
        group0 = float4(e1, e2, e3, e4);
        group1 = float4(e5, 0.0, 0.0, 0.0);
    }
    internal __init(
        float4 g0, float g1
    ) {
        group0 = g0;
        group1 = float4(g1, 0.0, 0.0, 0.0);
    }
}
public extension RoundPoint {
    public property e1: float {
        get { return group0[0]; }
        set { group0[0] = newValue; }
    }
    public property e2: float {
        get { return group0[1]; }
        set { group0[1] = newValue; }
    }
    public property e3: float {
        get { return group0[2]; }
        set { group0[2] = newValue; }
    }
    public property e4: float {
        get { return group0[3]; }
        set { group0[3] = newValue; }
    }
    public property e5: float {
        get { return group1[0]; }
        set { group1[0] = newValue; }
    }
}
public extension RoundPoint: IComparable {
    public bool equals(RoundPoint other) {
        return equalsHelper(this.group0, other.group0) && equalsHelper(this.group1, other.group1);
    }
    public bool lessThan(RoundPoint other) {
        if (!equalsHelper(this.group0, other.group0))
            return lessThanHelper(this.group0, other.group0);
        return lessThanHelper(this.group1, other.group1);
    }
    public bool lessThanOrEquals(RoundPoint other) {
        if (!equalsHelper(this.group0, other.group0))
            return lessThanOrEqualsHelper(this.group0, other.group0);
        return lessThanOrEqualsHelper(this.group1, other.group1);
    }
}
__include "impls/round_point";
