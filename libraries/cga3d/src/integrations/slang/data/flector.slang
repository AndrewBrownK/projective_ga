implementing cga3d;

/// Flector
public struct Flector {
    // e15, e25, e35, e45
    internal float4 group0;
    // e4235, e4315, e4125, e3215
    internal float4 group1;

    public __init(
        float e15, float e25, float e35, float e45, float e4235, float e4315, float e4125, float e3215
    ) {
        group0 = float4(e15, e25, e35, e45);
        group1 = float4(e4235, e4315, e4125, e3215);
    }
    internal __init(
        float4 g0, float4 g1
    ) {
        group0 = g0;
        group1 = g1;
    }
}
public extension Flector {
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
    public property e4235: float {
        get { return group1[0]; }
        set { group1[0] = newValue; }
    }
    public property e4315: float {
        get { return group1[1]; }
        set { group1[1] = newValue; }
    }
    public property e4125: float {
        get { return group1[2]; }
        set { group1[2] = newValue; }
    }
    public property e3215: float {
        get { return group1[3]; }
        set { group1[3] = newValue; }
    }
}
public extension Flector: IComparable {
    public bool equals(Flector other) {
        return equalsHelper(this.group0, other.group0) && equalsHelper(this.group1, other.group1);
    }
    public bool lessThan(Flector other) {
        if (!equalsHelper(this.group0, other.group0))
            return lessThanHelper(this.group0, other.group0);
        return lessThanHelper(this.group1, other.group1);
    }
    public bool lessThanOrEquals(Flector other) {
        if (!equalsHelper(this.group0, other.group0))
            return lessThanOrEqualsHelper(this.group0, other.group0);
        return lessThanOrEqualsHelper(this.group1, other.group1);
    }
}
__include "impls/flector";
