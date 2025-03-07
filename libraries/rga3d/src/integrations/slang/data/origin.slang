implementing rga3d;

/// Origin.
/// The Origin is the RoundPoint where x, y, z, and radius are all zero.
/// It is the base element e4.
/// Not to be confused with FlatOrigin, which is a Dipole connecting Origin and Infinity.
public struct Origin {
    // e4, 0, 0, 0
    internal float4 group0;

    public __init(
        float e4
    ) {
        group0 = float4(e4, 0.0, 0.0, 0.0);
    }
}
public extension Origin {
    public property e4: float {
        get { return group0[0]; }
        set { group0[0] = newValue; }
    }
}
public extension Origin: IComparable {
    public bool equals(Origin other) {
        return equalsHelper(this.group0, other.group0);
    }
    public bool lessThan(Origin other) {
        return lessThanHelper(this.group0, other.group0);
    }
    public bool lessThanOrEquals(Origin other) {
        return lessThanOrEqualsHelper(this.group0, other.group0);
    }
}
__include "impls/origin";
