implementing rga3d;

/// Flector
public struct Flector {
    // e1, e2, e3, e4
    internal float4 group0;
    // e423, e431, e412, e321
    internal float4 group1;

    public __init(
        float e1, float e2, float e3, float e4, float e423, float e431, float e412, float e321
    ) {
        group0 = float4(e1, e2, e3, e4);
        group1 = float4(e423, e431, e412, e321);
    }
    internal __init(
        float4 g0, float4 g1
    ) {
        group0 = g0;
        group1 = g1;
    }
}
public extension Flector {
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
    public property e423: float {
        get { return group1[0]; }
        set { group1[0] = newValue; }
    }
    public property e431: float {
        get { return group1[1]; }
        set { group1[1] = newValue; }
    }
    public property e412: float {
        get { return group1[2]; }
        set { group1[2] = newValue; }
    }
    public property e321: float {
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
