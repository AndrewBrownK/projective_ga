implementing cga3d;

/// Motor
public struct Motor {
    // e415, e425, e435, e12345
    internal float4 group0;
    // e235, e315, e125, e5
    internal float4 group1;

    public __init(
        float e415, float e425, float e435, float e12345, float e235, float e315, float e125, float e5
    ) {
        group0 = float4(e415, e425, e435, e12345);
        group1 = float4(e235, e315, e125, e5);
    }
    internal __init(
        float4 g0, float4 g1
    ) {
        group0 = g0;
        group1 = g1;
    }
}
public extension Motor {
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
    public property e12345: float {
        get { return group0[3]; }
        set { group0[3] = newValue; }
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
    public property e5: float {
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
