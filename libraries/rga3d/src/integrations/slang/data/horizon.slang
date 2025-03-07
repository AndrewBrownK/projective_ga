implementing rga3d;

/// Horizon
public struct Horizon {
    // e321, 0, 0, 0
    internal float4 group0;

    public __init(
        float e321
    ) {
        group0 = float4(e321, 0.0, 0.0, 0.0);
    }
}
public extension Horizon {
    public property e321: float {
        get { return group0[0]; }
        set { group0[0] = newValue; }
    }
}
public extension Horizon: IComparable {
    public bool equals(Horizon other) {
        return equalsHelper(this.group0, other.group0);
    }
    public bool lessThan(Horizon other) {
        return lessThanHelper(this.group0, other.group0);
    }
    public bool lessThanOrEquals(Horizon other) {
        return lessThanOrEqualsHelper(this.group0, other.group0);
    }
}
__include "impls/horizon";
