implementing cga3d;

/// Circle
public struct Circle {
    // e423, e431, e412, 0
    internal float4 group0;
    // e415, e425, e435, e321
    internal float4 group1;
    // e235, e315, e125, 0
    internal float4 group2;

    public __init(
        float e423, float e431, float e412, float e415, float e425, float e435, float e321, float e235, float e315, float e125
    ) {
        group0 = float4(e423, e431, e412, 0.0);
        group1 = float4(e415, e425, e435, e321);
        group2 = float4(e235, e315, e125, 0.0);
    }
    internal __init(
        float3 g0, float4 g1, float3 g2
    ) {
        group0 = float4(g0, 0.0);
        group1 = g1;
        group2 = float4(g2, 0.0);
    }
}
public extension Circle {
    public property e423: float {
        get { return group0[0]; }
        set { group0[0] = newValue; }
    }
    public property e431: float {
        get { return group0[1]; }
        set { group0[1] = newValue; }
    }
    public property e412: float {
        get { return group0[2]; }
        set { group0[2] = newValue; }
    }
    public property e415: float {
        get { return group1[0]; }
        set { group1[0] = newValue; }
    }
    public property e425: float {
        get { return group1[1]; }
        set { group1[1] = newValue; }
    }
    public property e435: float {
        get { return group1[2]; }
        set { group1[2] = newValue; }
    }
    public property e321: float {
        get { return group1[3]; }
        set { group1[3] = newValue; }
    }
    public property e235: float {
        get { return group2[0]; }
        set { group2[0] = newValue; }
    }
    public property e315: float {
        get { return group2[1]; }
        set { group2[1] = newValue; }
    }
    public property e125: float {
        get { return group2[2]; }
        set { group2[2] = newValue; }
    }
}
public extension Circle: IComparable {
    public bool equals(Circle other) {
        return equalsHelper(this.group0, other.group0) && equalsHelper(this.group1, other.group1) && equalsHelper(this.group2, other.group2);
    }
    public bool lessThan(Circle other) {
        if (!equalsHelper(this.group0, other.group0))
            return lessThanHelper(this.group0, other.group0);
        if (!equalsHelper(this.group1, other.group1))
            return lessThanHelper(this.group1, other.group1);
        return lessThanHelper(this.group2, other.group2);
    }
    public bool lessThanOrEquals(Circle other) {
        if (!equalsHelper(this.group0, other.group0))
            return lessThanOrEqualsHelper(this.group0, other.group0);
        if (!equalsHelper(this.group1, other.group1))
            return lessThanOrEqualsHelper(this.group1, other.group1);
        return lessThanOrEqualsHelper(this.group2, other.group2);
    }
}
__include "impls/circle";
