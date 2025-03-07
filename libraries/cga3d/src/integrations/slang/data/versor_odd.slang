implementing cga3d;

/// VersorOdd
public struct VersorOdd {
    // e41, e42, e43, scalar
    internal float4 group0;
    // e23, e31, e12, e45
    internal float4 group1;
    // e15, e25, e35, e1234
    internal float4 group2;
    // e4235, e4315, e4125, e3215
    internal float4 group3;

    public __init(
        float e41, float e42, float e43, float scalar, float e23, float e31, float e12, float e45, float e15, float e25, float e35, float e1234, float e4235, float e4315, float e4125, float e3215
    ) {
        group0 = float4(e41, e42, e43, scalar);
        group1 = float4(e23, e31, e12, e45);
        group2 = float4(e15, e25, e35, e1234);
        group3 = float4(e4235, e4315, e4125, e3215);
    }
    internal __init(
        float4 g0, float4 g1, float4 g2, float4 g3
    ) {
        group0 = g0;
        group1 = g1;
        group2 = g2;
        group3 = g3;
    }
}
public extension VersorOdd {
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
    public property scalar: float {
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
    public property e45: float {
        get { return group1[3]; }
        set { group1[3] = newValue; }
    }
    public property e15: float {
        get { return group2[0]; }
        set { group2[0] = newValue; }
    }
    public property e25: float {
        get { return group2[1]; }
        set { group2[1] = newValue; }
    }
    public property e35: float {
        get { return group2[2]; }
        set { group2[2] = newValue; }
    }
    public property e1234: float {
        get { return group2[3]; }
        set { group2[3] = newValue; }
    }
    public property e4235: float {
        get { return group3[0]; }
        set { group3[0] = newValue; }
    }
    public property e4315: float {
        get { return group3[1]; }
        set { group3[1] = newValue; }
    }
    public property e4125: float {
        get { return group3[2]; }
        set { group3[2] = newValue; }
    }
    public property e3215: float {
        get { return group3[3]; }
        set { group3[3] = newValue; }
    }
}
public extension VersorOdd: IComparable {
    public bool equals(VersorOdd other) {
        return equalsHelper(this.group0, other.group0) && equalsHelper(this.group1, other.group1) && equalsHelper(this.group2, other.group2) && equalsHelper(this.group3, other.group3);
    }
    public bool lessThan(VersorOdd other) {
        if (!equalsHelper(this.group0, other.group0))
            return lessThanHelper(this.group0, other.group0);
        if (!equalsHelper(this.group1, other.group1))
            return lessThanHelper(this.group1, other.group1);
        if (!equalsHelper(this.group2, other.group2))
            return lessThanHelper(this.group2, other.group2);
        return lessThanHelper(this.group3, other.group3);
    }
    public bool lessThanOrEquals(VersorOdd other) {
        if (!equalsHelper(this.group0, other.group0))
            return lessThanOrEqualsHelper(this.group0, other.group0);
        if (!equalsHelper(this.group1, other.group1))
            return lessThanOrEqualsHelper(this.group1, other.group1);
        if (!equalsHelper(this.group2, other.group2))
            return lessThanOrEqualsHelper(this.group2, other.group2);
        return lessThanOrEqualsHelper(this.group3, other.group3);
    }
}
__include "impls/versor_odd";
