implementing rga3d;

/// Plane
public struct Plane {
    // e423, e431, e412, e321
    internal float4 group0;

    public __init(
        float e423, float e431, float e412, float e321
    ) {
        group0 = float4(e423, e431, e412, e321);
    }
    internal __init(
        float4 g0
    ) {
        group0 = g0;
    }
}
public extension Plane {
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
    public property e321: float {
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
