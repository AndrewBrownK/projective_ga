implementing cga3d;
// Fancy postfix trick
public AntiFlatPoint operator /(AntiFlatPoint lhs, ConstraintValidPrefixOrPostfix rhs)  {
    return lhs.constraint_valid();
    }
// Fancy prefix trick
public AntiFlatPoint operator /(ConstraintValidPrefixOrPostfix lhs, AntiFlatPoint rhs) {
    return rhs.constraint_valid();
}
public extension AntiFlatPoint {
    public AntiFlatPoint constraint_valid() {
        return this;
    }
}
// Fancy postfix trick
public AntiFlector operator /(AntiFlector lhs, ConstraintValidPrefixOrPostfix rhs)  {
    return lhs.constraint_valid();
    }
// Fancy prefix trick
public AntiFlector operator /(ConstraintValidPrefixOrPostfix lhs, AntiFlector rhs) {
    return rhs.constraint_valid();
}
public extension AntiFlector {
    public AntiFlector constraint_valid() {
        return this;
    }
}
// Fancy postfix trick
public AntiPlane operator /(AntiPlane lhs, ConstraintValidPrefixOrPostfix rhs)  {
    return lhs.constraint_valid();
    }
// Fancy prefix trick
public AntiPlane operator /(ConstraintValidPrefixOrPostfix lhs, AntiPlane rhs) {
    return rhs.constraint_valid();
}
public extension AntiPlane {
    public AntiPlane constraint_valid() {
        return this;
    }
}
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
public FlatPoint operator /(FlatPoint lhs, ConstraintValidPrefixOrPostfix rhs)  {
    return lhs.constraint_valid();
    }
// Fancy prefix trick
public FlatPoint operator /(ConstraintValidPrefixOrPostfix lhs, FlatPoint rhs) {
    return rhs.constraint_valid();
}
public extension FlatPoint {
    public FlatPoint constraint_valid() {
        return this;
    }
}
// Fancy postfix trick
public Flector operator /(Flector lhs, ConstraintValidPrefixOrPostfix rhs)  {
    return lhs.constraint_valid();
    }
// Fancy prefix trick
public Flector operator /(ConstraintValidPrefixOrPostfix lhs, Flector rhs) {
    return rhs.constraint_valid();
}
public extension Flector {
    public Flector constraint_valid() {
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
public RoundPoint operator /(RoundPoint lhs, ConstraintValidPrefixOrPostfix rhs)  {
    return lhs.constraint_valid();
    }
// Fancy prefix trick
public RoundPoint operator /(ConstraintValidPrefixOrPostfix lhs, RoundPoint rhs) {
    return rhs.constraint_valid();
}
public extension RoundPoint {
    public RoundPoint constraint_valid() {
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
// Fancy postfix trick
public Sphere operator /(Sphere lhs, ConstraintValidPrefixOrPostfix rhs)  {
    return lhs.constraint_valid();
    }
// Fancy prefix trick
public Sphere operator /(ConstraintValidPrefixOrPostfix lhs, Sphere rhs) {
    return rhs.constraint_valid();
}
public extension Sphere {
    public Sphere constraint_valid() {
        return this;
    }
}
