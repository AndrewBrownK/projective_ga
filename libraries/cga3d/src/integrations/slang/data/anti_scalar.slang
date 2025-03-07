implementing cga3d;

/// AntiScalar
public struct AntiScalar {
    // e12345, 0, 0, 0
    internal float4 group0;

    public __init(
        float e12345
    ) {
        group0 = float4(e12345, 0.0, 0.0, 0.0);
    }
}
public extension AntiScalar {
    public property e12345: float {
        get { return group0[0]; }
        set { group0[0] = newValue; }
    }
}
public extension AntiScalar: IComparable {
    public bool equals(AntiScalar other) {
        return equalsHelper(this.group0, other.group0);
    }
    public bool lessThan(AntiScalar other) {
        return lessThanHelper(this.group0, other.group0);
    }
    public bool lessThanOrEquals(AntiScalar other) {
        return lessThanOrEqualsHelper(this.group0, other.group0);
    }
}
__include "impls/anti_scalar";
