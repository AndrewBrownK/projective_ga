implementing cga3d;

/// AntiMotor
public struct AntiMotor {
    // e23, e31, e12, scalar
    internal float4 group0;
    // e15, e25, e35, e3215
    internal float4 group1;

    public __init(
        float e23, float e31, float e12, float scalar, float e15, float e25, float e35, float e3215
    ) {
        group0 = float4(e23, e31, e12, scalar);
        group1 = float4(e15, e25, e35, e3215);
    }
    internal __init(
        float4 g0, float4 g1
    ) {
        group0 = g0;
        group1 = g1;
    }
}
public extension AntiMotor {
    public property e23: float {
        get { return group0[0]; }
        set { group0[0] = newValue; }
    }
    public property e31: float {
        get { return group0[1]; }
        set { group0[1] = newValue; }
    }
    public property e12: float {
        get { return group0[2]; }
        set { group0[2] = newValue; }
    }
    public property scalar: float {
        get { return group0[3]; }
        set { group0[3] = newValue; }
    }
    public property e15: float {
        get { return group1[0]; }
        set { group1[0] = newValue; }
    }
    public property e25: float {
        get { return group1[1]; }
        set { group1[1] = newValue; }
    }
    public property e35: float {
        get { return group1[2]; }
        set { group1[2] = newValue; }
    }
    public property e3215: float {
        get { return group1[3]; }
        set { group1[3] = newValue; }
    }
}
public extension AntiMotor: IComparable {
    public bool equals(AntiMotor other) {
        return equalsHelper(this.group0, other.group0) && equalsHelper(this.group1, other.group1);
    }
    public bool lessThan(AntiMotor other) {
        if (!equalsHelper(this.group0, other.group0))
            return lessThanHelper(this.group0, other.group0);
        return lessThanHelper(this.group1, other.group1);
    }
    public bool lessThanOrEquals(AntiMotor other) {
        if (!equalsHelper(this.group0, other.group0))
            return lessThanOrEqualsHelper(this.group0, other.group0);
        return lessThanOrEqualsHelper(this.group1, other.group1);
    }
}
__include "impls/anti_motor";
