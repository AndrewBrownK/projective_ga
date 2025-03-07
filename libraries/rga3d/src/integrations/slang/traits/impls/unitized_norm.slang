implementing rga3d;
// Fancy postfix trick
public float operator /(Flector lhs, UnitizedNormPrefixOrPostfix rhs)  {
    return lhs.unitized_norm();
    }
// Fancy prefix trick
public float operator /(UnitizedNormPrefixOrPostfix lhs, Flector rhs) {
    return rhs.unitized_norm();
}
public extension Flector {
    public float unitized_norm() {
        return 0.0;
    }
}
// Fancy postfix trick
public float operator /(Line lhs, UnitizedNormPrefixOrPostfix rhs)  {
    return lhs.unitized_norm();
    }
// Fancy prefix trick
public float operator /(UnitizedNormPrefixOrPostfix lhs, Line rhs) {
    return rhs.unitized_norm();
}
public extension Line {
    public float unitized_norm() {
        return 0.0;
    }
}
// Fancy postfix trick
public float operator /(Motor lhs, UnitizedNormPrefixOrPostfix rhs)  {
    return lhs.unitized_norm();
    }
// Fancy prefix trick
public float operator /(UnitizedNormPrefixOrPostfix lhs, Motor rhs) {
    return rhs.unitized_norm();
}
public extension Motor {
    public float unitized_norm() {
        return 0.0;
    }
}
// Fancy postfix trick
public float operator /(MultiVector lhs, UnitizedNormPrefixOrPostfix rhs)  {
    return lhs.unitized_norm();
    }
// Fancy prefix trick
public float operator /(UnitizedNormPrefixOrPostfix lhs, MultiVector rhs) {
    return rhs.unitized_norm();
}
public extension MultiVector {
    public float unitized_norm() {
        return 0.0;
    }
}
// Fancy postfix trick
public float operator /(Point lhs, UnitizedNormPrefixOrPostfix rhs)  {
    return lhs.unitized_norm();
    }
// Fancy prefix trick
public float operator /(UnitizedNormPrefixOrPostfix lhs, Point rhs) {
    return rhs.unitized_norm();
}
public extension Point {
    public float unitized_norm() {
        return 0.0;
    }
}
