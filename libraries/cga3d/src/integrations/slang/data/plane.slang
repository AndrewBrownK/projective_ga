implementing cga3d;

/// Plane
public struct Plane {
    // e4235, e4315, e4125, e3215
    internal float4 group0;

    public __init(
        float e4235, float e4315, float e4125, float e3215
    ) {
        group0 = float4(e4235, e4315, e4125, e3215);
    }
    internal __init(
        float4 g0
    ) {
        group0 = g0;
    }
}
public extension Plane {
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
}
public extension Plane: IComparable {
    public bool equals(Plane other) {
        return equalsHelper(this.group0, other.group0);
    }
    public bool lessThan(Plane other) {
        return lessThanHelper(this.group0, other.group0);
    }
    public bool lessThanOrEquals(Plane other) {
        return lessThanOrEqualsHelper(this.group0, other.group0);
    }
}
__include "impls/plane";
