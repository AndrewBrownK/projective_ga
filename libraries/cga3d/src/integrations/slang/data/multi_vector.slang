implementing cga3d;

/// MultiVector
public struct MultiVector {
    // scalar, e12345, 0, 0
    internal float4 group0;
    // e1, e2, e3, e4
    internal float4 group1;
    // e5, 0, 0, 0
    internal float4 group2;
    // e15, e25, e35, e45
    internal float4 group3;
    // e41, e42, e43, 0
    internal float4 group4;
    // e23, e31, e12, 0
    internal float4 group5;
    // e415, e425, e435, e321
    internal float4 group6;
    // e423, e431, e412, 0
    internal float4 group7;
    // e235, e315, e125, 0
    internal float4 group8;
    // e4235, e4315, e4125, e3215
    internal float4 group9;
    // e1234, 0, 0, 0
    internal float4 group10;

    public __init(
        float scalar, float e12345, float e1, float e2, float e3, float e4, float e5, float e15, float e25, float e35, float e45, float e41, float e42, float e43, float e23, float e31, float e12, float e415, float e425, float e435, float e321, float e423, float e431, float e412, float e235, float e315, float e125, float e4235, float e4315, float e4125, float e3215, float e1234
    ) {
        group0 = float4(scalar, e12345, 0.0, 0.0);
        group1 = float4(e1, e2, e3, e4);
        group2 = float4(e5, 0.0, 0.0, 0.0);
        group3 = float4(e15, e25, e35, e45);
        group4 = float4(e41, e42, e43, 0.0);
        group5 = float4(e23, e31, e12, 0.0);
        group6 = float4(e415, e425, e435, e321);
        group7 = float4(e423, e431, e412, 0.0);
        group8 = float4(e235, e315, e125, 0.0);
        group9 = float4(e4235, e4315, e4125, e3215);
        group10 = float4(e1234, 0.0, 0.0, 0.0);
    }
    internal __init(
        float2 g0, float4 g1, float g2, float4 g3, float3 g4, float3 g5, float4 g6, float3 g7, float3 g8, float4 g9, float g10
    ) {
        group0 = float4(g0, 0.0, 0.0);
        group1 = g1;
        group2 = float4(g2, 0.0, 0.0, 0.0);
        group3 = g3;
        group4 = float4(g4, 0.0);
        group5 = float4(g5, 0.0);
        group6 = g6;
        group7 = float4(g7, 0.0);
        group8 = float4(g8, 0.0);
        group9 = g9;
        group10 = float4(g10, 0.0, 0.0, 0.0);
    }
}
public extension MultiVector {
    public property scalar: float {
        get { return group0[0]; }
        set { group0[0] = newValue; }
    }
    public property e12345: float {
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
    public property e5: float {
        get { return group2[0]; }
        set { group2[0] = newValue; }
    }
    public property e15: float {
        get { return group3[0]; }
        set { group3[0] = newValue; }
    }
    public property e25: float {
        get { return group3[1]; }
        set { group3[1] = newValue; }
    }
    public property e35: float {
        get { return group3[2]; }
        set { group3[2] = newValue; }
    }
    public property e45: float {
        get { return group3[3]; }
        set { group3[3] = newValue; }
    }
    public property e41: float {
        get { return group4[0]; }
        set { group4[0] = newValue; }
    }
    public property e42: float {
        get { return group4[1]; }
        set { group4[1] = newValue; }
    }
    public property e43: float {
        get { return group4[2]; }
        set { group4[2] = newValue; }
    }
    public property e23: float {
        get { return group5[0]; }
        set { group5[0] = newValue; }
    }
    public property e31: float {
        get { return group5[1]; }
        set { group5[1] = newValue; }
    }
    public property e12: float {
        get { return group5[2]; }
        set { group5[2] = newValue; }
    }
    public property e415: float {
        get { return group6[0]; }
        set { group6[0] = newValue; }
    }
    public property e425: float {
        get { return group6[1]; }
        set { group6[1] = newValue; }
    }
    public property e435: float {
        get { return group6[2]; }
        set { group6[2] = newValue; }
    }
    public property e321: float {
        get { return group6[3]; }
        set { group6[3] = newValue; }
    }
    public property e423: float {
        get { return group7[0]; }
        set { group7[0] = newValue; }
    }
    public property e431: float {
        get { return group7[1]; }
        set { group7[1] = newValue; }
    }
    public property e412: float {
        get { return group7[2]; }
        set { group7[2] = newValue; }
    }
    public property e235: float {
        get { return group8[0]; }
        set { group8[0] = newValue; }
    }
    public property e315: float {
        get { return group8[1]; }
        set { group8[1] = newValue; }
    }
    public property e125: float {
        get { return group8[2]; }
        set { group8[2] = newValue; }
    }
    public property e4235: float {
        get { return group9[0]; }
        set { group9[0] = newValue; }
    }
    public property e4315: float {
        get { return group9[1]; }
        set { group9[1] = newValue; }
    }
    public property e4125: float {
        get { return group9[2]; }
        set { group9[2] = newValue; }
    }
    public property e3215: float {
        get { return group9[3]; }
        set { group9[3] = newValue; }
    }
    public property e1234: float {
        get { return group10[0]; }
        set { group10[0] = newValue; }
    }
}
public extension MultiVector: IComparable {
    public bool equals(MultiVector other) {
        return equalsHelper(this.group0, other.group0) && equalsHelper(this.group1, other.group1) && equalsHelper(this.group2, other.group2) && equalsHelper(this.group3, other.group3) && equalsHelper(this.group4, other.group4) && equalsHelper(this.group5, other.group5) && equalsHelper(this.group6, other.group6) && equalsHelper(this.group7, other.group7) && equalsHelper(this.group8, other.group8) && equalsHelper(this.group9, other.group9) && equalsHelper(this.group10, other.group10);
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
        if (!equalsHelper(this.group4, other.group4))
            return lessThanHelper(this.group4, other.group4);
        if (!equalsHelper(this.group5, other.group5))
            return lessThanHelper(this.group5, other.group5);
        if (!equalsHelper(this.group6, other.group6))
            return lessThanHelper(this.group6, other.group6);
        if (!equalsHelper(this.group7, other.group7))
            return lessThanHelper(this.group7, other.group7);
        if (!equalsHelper(this.group8, other.group8))
            return lessThanHelper(this.group8, other.group8);
        if (!equalsHelper(this.group9, other.group9))
            return lessThanHelper(this.group9, other.group9);
        return lessThanHelper(this.group10, other.group10);
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
        if (!equalsHelper(this.group4, other.group4))
            return lessThanOrEqualsHelper(this.group4, other.group4);
        if (!equalsHelper(this.group5, other.group5))
            return lessThanOrEqualsHelper(this.group5, other.group5);
        if (!equalsHelper(this.group6, other.group6))
            return lessThanOrEqualsHelper(this.group6, other.group6);
        if (!equalsHelper(this.group7, other.group7))
            return lessThanOrEqualsHelper(this.group7, other.group7);
        if (!equalsHelper(this.group8, other.group8))
            return lessThanOrEqualsHelper(this.group8, other.group8);
        if (!equalsHelper(this.group9, other.group9))
            return lessThanOrEqualsHelper(this.group9, other.group9);
        return lessThanOrEqualsHelper(this.group10, other.group10);
    }
}
__include "impls/multi_vector";
