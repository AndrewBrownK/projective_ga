implementing cga3d;

/// Sphere
public struct Sphere {
    // e4235, e4315, e4125, e3215
    internal float4 group0;
    // e1234, 0, 0, 0
    internal float4 group1;

    public __init(
        float e4235, float e4315, float e4125, float e3215, float e1234
    ) {
        group0 = float4(e4235, e4315, e4125, e3215);
        group1 = float4(e1234, 0.0, 0.0, 0.0);
    }
    internal __init(
        float4 g0, float g1
    ) {
        group0 = g0;
        group1 = float4(g1, 0.0, 0.0, 0.0);
    }
}
public extension Sphere {
    public property e4235: float {
        get { return group0[0]; }
        set { group0[0] = newValue; }
    }
    public property e4315: float {
        get { return group0[1]; }
        set { group0[1] = newValue; }
    }
    public property e4125: float {
        get { return group0[2]; }
        set { group0[2] = newValue; }
    }
    public property e3215: float {
        get { return group0[3]; }
        set { group0[3] = newValue; }
    }
    public property e1234: float {
        get { return group1[0]; }
        set { group1[0] = newValue; }
    }
}
public extension Sphere: IComparable {
    public bool equals(Sphere other) {
        return equalsHelper(this.group0, other.group0) && equalsHelper(this.group1, other.group1);
    }
    public bool lessThan(Sphere other) {
        if (!equalsHelper(this.group0, other.group0))
            return lessThanHelper(this.group0, other.group0);
        return lessThanHelper(this.group1, other.group1);
    }
    public bool lessThanOrEquals(Sphere other) {
        if (!equalsHelper(this.group0, other.group0))
            return lessThanOrEqualsHelper(this.group0, other.group0);
        return lessThanOrEqualsHelper(this.group1, other.group1);
    }
}
__include "impls/sphere";
