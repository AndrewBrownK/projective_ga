implementing rga3d;

/// MultiVector
public struct MultiVector {
    // scalar, e1234, 0, 0
    internal float4 group0;
    // e1, e2, e3, e4
    internal float4 group1;
    // e41, e42, e43, 0
    internal float4 group2;
    // e23, e31, e12, 0
    internal float4 group3;
    // e423, e431, e412, e321
    internal float4 group4;

    public __init(
        float scalar, float e1234, float e1, float e2, float e3, float e4, float e41, float e42, float e43, float e23, float e31, float e12, float e423, float e431, float e412, float e321
    ) {
        group0 = float4(scalar, e1234, 0.0, 0.0);
        group1 = float4(e1, e2, e3, e4);
        group2 = float4(e41, e42, e43, 0.0);
        group3 = float4(e23, e31, e12, 0.0);
        group4 = float4(e423, e431, e412, e321);
    }
    internal __init(
        float2 g0, float4 g1, float3 g2, float3 g3, float4 g4
    ) {
        group0 = float4(g0, 0.0, 0.0);
        group1 = g1;
        group2 = float4(g2, 0.0);
        group3 = float4(g3, 0.0);
        group4 = g4;
    }
}
public extension MultiVector {
    public property scalar: float {
        get { return group0[0]; }
        set { group0[0] = newValue; }
    }
    public property e1234: float {
        get { return group0[1]; }
        set { group0[1] = newValue; }
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
    public property e4: float {
        get { return group1[3]; }
        set { group1[3] = newValue; }
    }
    public property e41: float {
        get { return group2[0]; }
        set { group2[0] = newValue; }
    }
    public property e42: float {
        get { return group2[1]; }
        set { group2[1] = newValue; }
    }
    public property e43: float {
        get { return group2[2]; }
        set { group2[2] = newValue; }
    }
    public property e23: float {
        get { return group3[0]; }
        set { group3[0] = newValue; }
    }
    public property e31: float {
        get { return group3[1]; }
        set { group3[1] = newValue; }
    }
    public property e12: float {
        get { return group3[2]; }
        set { group3[2] = newValue; }
    }
    public property e423: float {
        get { return group4[0]; }
        set { group4[0] = newValue; }
    }
    public property e431: float {
        get { return group4[1]; }
        set { group4[1] = newValue; }
    }
    public property e412: float {
        get { return group4[2]; }
        set { group4[2] = newValue; }
    }
    public property e321: float {
        get { return group4[3]; }
        set { group4[3] = newValue; }
    }
}
public extension MultiVector: IComparable {
    public bool equals(MultiVector other) {
        return equalsHelper(this.group0, other.group0) && equalsHelper(this.group1, other.group1) && equalsHelper(this.group2, other.group2) && equalsHelper(this.group3, other.group3) && equalsHelper(this.group4, other.group4);
    }
    public bool lessThan(MultiVector other) {
        if (!equalsHelper(this.group0, other.group0))
            return lessThanHelper(this.group0, other.group0);
        if (!equalsHelper(this.group1, other.group1))
            return lessThanHelper(this.group1, other.group1);
        if (!equalsHelper(this.group2, other.group2))
            return lessThanHelper(this.group2, other.group2);
        if (!equalsHelper(this.group3, other.group3))
            return lessThanHelper(this.group3, other.group3);
        return lessThanHelper(this.group4, other.group4);
    }
    public bool lessThanOrEquals(MultiVector other) {
        if (!equalsHelper(this.group0, other.group0))
            return lessThanOrEqualsHelper(this.group0, other.group0);
        if (!equalsHelper(this.group1, other.group1))
            return lessThanOrEqualsHelper(this.group1, other.group1);
        if (!equalsHelper(this.group2, other.group2))
            return lessThanOrEqualsHelper(this.group2, other.group2);
        if (!equalsHelper(this.group3, other.group3))
            return lessThanOrEqualsHelper(this.group3, other.group3);
        return lessThanOrEqualsHelper(this.group4, other.group4);
    }
}
__include "impls/multi_vector";
