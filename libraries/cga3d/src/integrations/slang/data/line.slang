implementing cga3d;

/// Line
public struct Line {
    // e415, e425, e435, 0
    internal float4 group0;
    // e235, e315, e125, 0
    internal float4 group1;

    public __init(
        float e415, float e425, float e435, float e235, float e315, float e125
    ) {
        group0 = float4(e415, e425, e435, 0.0);
        group1 = float4(e235, e315, e125, 0.0);
    }
    internal __init(
        float3 g0, float3 g1
    ) {
        group0 = float4(g0, 0.0);
        group1 = float4(g1, 0.0);
    }
}
public extension Line {
    public property e415: float {
        get { return group0[0]; }
        set { group0[0] = newValue; }
    }
    public property e425: float {
        get { return group0[1]; }
        set { group0[1] = newValue; }
    }
    public property e435: float {
        get { return group0[2]; }
        set { group0[2] = newValue; }
    }
    public property e235: float {
        get { return group1[0]; }
        set { group1[0] = newValue; }
    }
    public property e315: float {
        get { return group1[1]; }
        set { group1[1] = newValue; }
    }
    public property e125: float {
        get { return group1[2]; }
        set { group1[2] = newValue; }
    }
}
public extension Line: IComparable {
    public bool equals(Line other) {
        return equalsHelper(this.group0, other.group0) && equalsHelper(this.group1, other.group1);
    }
    public bool lessThan(Line other) {
        if (!equalsHelper(this.group0, other.group0))
            return lessThanHelper(this.group0, other.group0);
        return lessThanHelper(this.group1, other.group1);
    }
    public bool lessThanOrEquals(Line other) {
        if (!equalsHelper(this.group0, other.group0))
            return lessThanOrEqualsHelper(this.group0, other.group0);
        return lessThanOrEqualsHelper(this.group1, other.group1);
    }
}
__include "impls/line";
