implementing cga3d;
public extension AntiFlatPoint {
    public uint anti_grade() {
        return 2;
    }
}
public extension AntiLine {
    public uint anti_grade() {
        return 3;
    }
}
public extension AntiPlane {
    public uint anti_grade() {
        return 4;
    }
}
public extension AntiScalar {
    public uint anti_grade() {
        return 0;
    }
}
public extension Circle {
    public uint anti_grade() {
        return 2;
    }
}
public extension Dipole {
    public uint anti_grade() {
        return 3;
    }
}
public extension FlatPoint {
    public uint anti_grade() {
        return 3;
    }
}
public extension Line {
    public uint anti_grade() {
        return 2;
    }
}
public extension Plane {
    public uint anti_grade() {
        return 1;
    }
}
public extension RoundPoint {
    public uint anti_grade() {
        return 4;
    }
}
public extension Scalar {
    public uint anti_grade() {
        return 5;
    }
}
public extension Sphere {
    public uint anti_grade() {
        return 1;
    }
}
