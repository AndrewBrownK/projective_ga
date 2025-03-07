implementing cga3d;
public extension AntiFlatPoint {
    public uint grade() {
        return 3;
    }
}
public extension AntiLine {
    public uint grade() {
        return 2;
    }
}
public extension AntiPlane {
    public uint grade() {
        return 1;
    }
}
public extension AntiScalar {
    public uint grade() {
        return 5;
    }
}
public extension Circle {
    public uint grade() {
        return 3;
    }
}
public extension Dipole {
    public uint grade() {
        return 2;
    }
}
public extension FlatPoint {
    public uint grade() {
        return 2;
    }
}
public extension Line {
    public uint grade() {
        return 3;
    }
}
public extension Plane {
    public uint grade() {
        return 4;
    }
}
public extension RoundPoint {
    public uint grade() {
        return 1;
    }
}
public extension Scalar {
    public uint grade() {
        return 0;
    }
}
public extension Sphere {
    public uint grade() {
        return 4;
    }
}
