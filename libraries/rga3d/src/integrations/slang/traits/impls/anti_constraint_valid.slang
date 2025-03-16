implementing rga3d;
// Fancy postfix trick
public AntiScalar operator /(AntiScalar lhs, AntiConstraintValidPrefixOrPostfix rhs)  {
    return lhs.anti_constraint_valid();
    }
// Fancy prefix trick
public AntiScalar operator /(AntiConstraintValidPrefixOrPostfix lhs, AntiScalar rhs) {
    return rhs.anti_constraint_valid();
}
public extension AntiScalar {
    public AntiScalar anti_constraint_valid() {
        return this;
    }
}
// Fancy postfix trick
public Flector operator /(Flector lhs, AntiConstraintValidPrefixOrPostfix rhs)  {
    return lhs.anti_constraint_valid();
    }
// Fancy prefix trick
public Flector operator /(AntiConstraintValidPrefixOrPostfix lhs, Flector rhs) {
    return rhs.anti_constraint_valid();
}
public extension Flector {
    public Flector anti_constraint_valid() {
        return this;
    }
}
// Fancy postfix trick
public Horizon operator /(Horizon lhs, AntiConstraintValidPrefixOrPostfix rhs)  {
    return lhs.anti_constraint_valid();
    }
// Fancy prefix trick
public Horizon operator /(AntiConstraintValidPrefixOrPostfix lhs, Horizon rhs) {
    return rhs.anti_constraint_valid();
}
public extension Horizon {
    public Horizon anti_constraint_valid() {
        return this;
    }
}
// Fancy postfix trick
public Origin operator /(Origin lhs, AntiConstraintValidPrefixOrPostfix rhs)  {
    return lhs.anti_constraint_valid();
    }
// Fancy prefix trick
public Origin operator /(AntiConstraintValidPrefixOrPostfix lhs, Origin rhs) {
    return rhs.anti_constraint_valid();
}
public extension Origin {
    public Origin anti_constraint_valid() {
        return this;
    }
}
// Fancy postfix trick
public Plane operator /(Plane lhs, AntiConstraintValidPrefixOrPostfix rhs)  {
    return lhs.anti_constraint_valid();
    }
// Fancy prefix trick
public Plane operator /(AntiConstraintValidPrefixOrPostfix lhs, Plane rhs) {
    return rhs.anti_constraint_valid();
}
public extension Plane {
    public Plane anti_constraint_valid() {
        return this;
    }
}
// Fancy postfix trick
public Point operator /(Point lhs, AntiConstraintValidPrefixOrPostfix rhs)  {
    return lhs.anti_constraint_valid();
    }
// Fancy prefix trick
public Point operator /(AntiConstraintValidPrefixOrPostfix lhs, Point rhs) {
    return rhs.anti_constraint_valid();
}
public extension Point {
    public Point anti_constraint_valid() {
        return this;
    }
}
// Fancy postfix trick
public Scalar operator /(Scalar lhs, AntiConstraintValidPrefixOrPostfix rhs)  {
    return lhs.anti_constraint_valid();
    }
// Fancy prefix trick
public Scalar operator /(AntiConstraintValidPrefixOrPostfix lhs, Scalar rhs) {
    return rhs.anti_constraint_valid();
}
public extension Scalar {
    public Scalar anti_constraint_valid() {
        return this;
    }
}
