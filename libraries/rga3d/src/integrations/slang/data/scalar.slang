implementing rga3d;

/// Scalar
public struct Scalar {
    // scalar, 0, 0, 0
    internal float4 group0;

    public __init(
        float scalar
    ) {
        group0 = float4(scalar, 0.0, 0.0, 0.0);
    }
}
public extension Scalar {
    public property scalar: float {
        get { return group0[0]; }
        set { group0[0] = newValue; }
    }
}
public extension Scalar: IComparable {
    public bool equals(Scalar other) {
        return equalsHelper(this.group0, other.group0);
    }
    public bool lessThan(Scalar other) {
        return lessThanHelper(this.group0, other.group0);
    }
    public bool lessThanOrEquals(Scalar other) {
        return lessThanOrEqualsHelper(this.group0, other.group0);
    }
}
__include "impls/scalar";
