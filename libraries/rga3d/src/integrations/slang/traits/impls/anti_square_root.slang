implementing rga3d;
// Fancy postfix trick
public AntiScalar operator /(AntiScalar lhs, AntiSquareRootPrefixOrPostfix rhs)  {
    return lhs.anti_square_root();
    }
// Fancy prefix trick
public AntiScalar operator /(AntiSquareRootPrefixOrPostfix lhs, AntiScalar rhs) {
    return rhs.anti_square_root();
}
public extension AntiScalar {
    public AntiScalar anti_square_root() {
        return AntiScalar(
            /* e1234 */
            pow(this.e1234, 0.5)
        );
    }
}
