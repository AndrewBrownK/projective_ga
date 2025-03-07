implementing rga3d;

/// Motor
public struct Motor {
    // e41, e42, e43, e1234
    internal float4 group0;
    // e23, e31, e12, scalar
    internal float4 group1;

    public __init(
        float e41, float e42, float e43, float e1234, float e23, float e31, float e12, float scalar
    ) {
        group0 = float4(e41, e42, e43, e1234);
        group1 = float4(e23, e31, e12, scalar);
    }
    internal __init(
        float4 g0, float4 g1
    ) {
        group0 = g0;
        group1 = g1;
    }
}
public extension Motor {
    public property e41: float {
        get { return group0[0]; }
        set { group0[0] = newValue; }
    }
    public property e42: float {
        get { return group0[1]; }
        set { group0[1] = newValue; }
    }
    public property e43: float {
        get { return group0[2]; }
        set { group0[2] = newValue; }
    }
    public property e1234: float {
        get { return group0[3]; }
        set { group0[3] = newValue; }
    }
    public property e23: float {
        get { return group1[0]; }
        set { group1[0] = newValue; }
    }
    public property e31: float {
        get { return group1[1]; }
        set { group1[1] = newValue; }
    }
    public property e12: float {
        get { return group1[2]; }
        set { group1[2] = newValue; }
    }
    public property scalar: float {
        get { return group1[3]; }
        set { group1[3] = newValue; }
    }
}
public extension Motor: IComparable {
    public bool equals(Motor other) {
        return equalsHelper(this.group0, other.group0) && equalsHelper(this.group1, other.group1);
    }
    public bool lessThan(Motor other) {
        if (!equalsHelper(this.group0, other.group0))
            return lessThanHelper(this.group0, other.group0);
        return lessThanHelper(this.group1, other.group1);
    }
    public bool lessThanOrEquals(Motor other) {
        if (!equalsHelper(this.group0, other.group0))
            return lessThanOrEqualsHelper(this.group0, other.group0);
        return lessThanOrEqualsHelper(this.group1, other.group1);
    }
}
__include "impls/motor";
