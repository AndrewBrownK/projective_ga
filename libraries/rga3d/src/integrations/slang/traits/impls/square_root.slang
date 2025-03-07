implementing rga3d;
// Fancy postfix trick
public Scalar operator /(Scalar lhs, SquareRootPrefixOrPostfix rhs)  {
    return lhs.square_root();
    }
// Fancy prefix trick
public Scalar operator /(SquareRootPrefixOrPostfix lhs, Scalar rhs) {
    return rhs.square_root();
}
public extension Scalar {
    public Scalar square_root() {
        return Scalar(
            /* scalar */
            pow(this.scalar, 0.5)
        );
    }
}
