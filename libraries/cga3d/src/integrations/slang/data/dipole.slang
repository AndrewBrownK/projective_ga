implementing cga3d;

/// Dipole
public struct Dipole {
    // e41, e42, e43, 0
    internal float4 group0;
    // e23, e31, e12, e45
    internal float4 group1;
    // e15, e25, e35, 0
    internal float4 group2;

    public __init(
        float e41, float e42, float e43, float e23, float e31, float e12, float e45, float e15, float e25, float e35
    ) {
        group0 = float4(e41, e42, e43, 0.0);
        group1 = float4(e23, e31, e12, e45);
        group2 = float4(e15, e25, e35, 0.0);
    }
    internal __init(
        float3 g0, float4 g1, float3 g2
    ) {
        group0 = float4(g0, 0.0);
        group1 = g1;
        group2 = float4(g2, 0.0);
    }
}
public extension Dipole {
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
}
public extension Dipole: IComparable {
    public bool equals(Dipole other) {
        return equalsHelper(this.group0, other.group0) && equalsHelper(this.group1, other.group1) && equalsHelper(this.group2, other.group2);
    }
    public bool lessThan(Dipole other) {
        if (!equalsHelper(this.group0, other.group0))
            return lessThanHelper(this.group0, other.group0);
        if (!equalsHelper(this.group1, other.group1))
            return lessThanHelper(this.group1, other.group1);
        return lessThanHelper(this.group2, other.group2);
    }
    public bool lessThanOrEquals(Dipole other) {
        if (!equalsHelper(this.group0, other.group0))
            return lessThanOrEqualsHelper(this.group0, other.group0);
        if (!equalsHelper(this.group1, other.group1))
            return lessThanOrEqualsHelper(this.group1, other.group1);
        return lessThanOrEqualsHelper(this.group2, other.group2);
    }
}
__include "impls/dipole";
