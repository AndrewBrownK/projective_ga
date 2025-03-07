implementing cga3d;
// Fancy postfix trick
public AntiFlatPoint operator /(AntiFlatPoint lhs, AntiConstraintValidPrefixOrPostfix rhs)  {
    return lhs.anti_constraint_valid();
    }
// Fancy prefix trick
public AntiFlatPoint operator /(AntiConstraintValidPrefixOrPostfix lhs, AntiFlatPoint rhs) {
    return rhs.anti_constraint_valid();
}
public extension AntiFlatPoint {
    public AntiFlatPoint anti_constraint_valid() {
        return this;
    }
}
// Fancy postfix trick
public AntiPlane operator /(AntiPlane lhs, AntiConstraintValidPrefixOrPostfix rhs)  {
    return lhs.anti_constraint_valid();
    }
// Fancy prefix trick
public AntiPlane operator /(AntiConstraintValidPrefixOrPostfix lhs, AntiPlane rhs) {
    return rhs.anti_constraint_valid();
}
public extension AntiPlane {
    public AntiPlane anti_constraint_valid() {
        return this;
    }
}
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
public FlatPoint operator /(FlatPoint lhs, AntiConstraintValidPrefixOrPostfix rhs)  {
    return lhs.anti_constraint_valid();
    }
// Fancy prefix trick
public FlatPoint operator /(AntiConstraintValidPrefixOrPostfix lhs, FlatPoint rhs) {
    return rhs.anti_constraint_valid();
}
public extension FlatPoint {
    public FlatPoint anti_constraint_valid() {
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
public RoundPoint operator /(RoundPoint lhs, AntiConstraintValidPrefixOrPostfix rhs)  {
    return lhs.anti_constraint_valid();
    }
// Fancy prefix trick
public RoundPoint operator /(AntiConstraintValidPrefixOrPostfix lhs, RoundPoint rhs) {
    return rhs.anti_constraint_valid();
}
public extension RoundPoint {
    public RoundPoint anti_constraint_valid() {
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
// Fancy postfix trick
public Sphere operator /(Sphere lhs, AntiConstraintValidPrefixOrPostfix rhs)  {
    return lhs.anti_constraint_valid();
    }
// Fancy prefix trick
public Sphere operator /(AntiConstraintValidPrefixOrPostfix lhs, Sphere rhs) {
    return rhs.anti_constraint_valid();
}
public extension Sphere {
    public Sphere anti_constraint_valid() {
        return this;
    }
}
