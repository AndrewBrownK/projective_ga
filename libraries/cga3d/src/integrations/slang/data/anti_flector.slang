implementing cga3d;

/// AntiFlector
public struct AntiFlector {
    // e235, e315, e125, e321
    internal float4 group0;
    // e1, e2, e3, e5
    internal float4 group1;

    public __init(
        float e235, float e315, float e125, float e321, float e1, float e2, float e3, float e5
    ) {
        group0 = float4(e235, e315, e125, e321);
        group1 = float4(e1, e2, e3, e5);
    }
    internal __init(
        float4 g0, float4 g1
    ) {
        group0 = g0;
        group1 = g1;
    }
}
public extension AntiFlector {
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
    public property e1: float {
        get { return group1[0]; }
        set { group1[0] = newValue; }
    }
    public property e2: float {
        get { return group1[1]; }
        set { group1[1] = newValue; }
    }
    public property e3: float {
        get { return group1[2]; }
        set { group1[2] = newValue; }
    }
    public property e5: float {
        get { return group1[3]; }
        set { group1[3] = newValue; }
    }
}
public extension AntiFlector: IComparable {
    public bool equals(AntiFlector other) {
        return equalsHelper(this.group0, other.group0) && equalsHelper(this.group1, other.group1);
    }
    public bool lessThan(AntiFlector other) {
        if (!equalsHelper(this.group0, other.group0))
            return lessThanHelper(this.group0, other.group0);
        return lessThanHelper(this.group1, other.group1);
    }
    public bool lessThanOrEquals(AntiFlector other) {
        if (!equalsHelper(this.group0, other.group0))
            return lessThanOrEqualsHelper(this.group0, other.group0);
        return lessThanOrEqualsHelper(this.group1, other.group1);
    }
}
__include "impls/anti_flector";
