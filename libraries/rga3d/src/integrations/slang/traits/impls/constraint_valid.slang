implementing rga3d;
// Fancy postfix trick
public AntiScalar operator /(AntiScalar lhs, ConstraintValidPrefixOrPostfix rhs)  {
    return lhs.constraint_valid();
    }
// Fancy prefix trick
public AntiScalar operator /(ConstraintValidPrefixOrPostfix lhs, AntiScalar rhs) {
    return rhs.constraint_valid();
}
public extension AntiScalar {
    public AntiScalar constraint_valid() {
        return this;
    }
}
// Fancy postfix trick
public Horizon operator /(Horizon lhs, ConstraintValidPrefixOrPostfix rhs)  {
    return lhs.constraint_valid();
    }
// Fancy prefix trick
public Horizon operator /(ConstraintValidPrefixOrPostfix lhs, Horizon rhs) {
    return rhs.constraint_valid();
}
public extension Horizon {
    public Horizon constraint_valid() {
        return this;
    }
}
// Fancy postfix trick
public Origin operator /(Origin lhs, ConstraintValidPrefixOrPostfix rhs)  {
    return lhs.constraint_valid();
    }
// Fancy prefix trick
public Origin operator /(ConstraintValidPrefixOrPostfix lhs, Origin rhs) {
    return rhs.constraint_valid();
}
public extension Origin {
    public Origin constraint_valid() {
        return this;
    }
}
// Fancy postfix trick
public Plane operator /(Plane lhs, ConstraintValidPrefixOrPostfix rhs)  {
    return lhs.constraint_valid();
    }
// Fancy prefix trick
public Plane operator /(ConstraintValidPrefixOrPostfix lhs, Plane rhs) {
    return rhs.constraint_valid();
}
public extension Plane {
    public Plane constraint_valid() {
        return this;
    }
}
// Fancy postfix trick
public Point operator /(Point lhs, ConstraintValidPrefixOrPostfix rhs)  {
    return lhs.constraint_valid();
    }
// Fancy prefix trick
public Point operator /(ConstraintValidPrefixOrPostfix lhs, Point rhs) {
    return rhs.constraint_valid();
}
public extension Point {
    public Point constraint_valid() {
        return this;
    }
}
// Fancy postfix trick
public Scalar operator /(Scalar lhs, ConstraintValidPrefixOrPostfix rhs)  {
    return lhs.constraint_valid();
    }
// Fancy prefix trick
public Scalar operator /(ConstraintValidPrefixOrPostfix lhs, Scalar rhs) {
    return rhs.constraint_valid();
}
public extension Scalar {
    public Scalar constraint_valid() {
        return this;
    }
}
