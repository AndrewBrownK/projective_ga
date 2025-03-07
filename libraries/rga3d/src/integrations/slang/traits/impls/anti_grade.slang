implementing rga3d;
public extension AntiScalar {
    public uint anti_grade() {
        return 0;
    }
}
public extension Horizon {
    public uint anti_grade() {
        return 1;
    }
}
public extension Line {
    public uint anti_grade() {
        return 2;
    }
}
public extension Origin {
    public uint anti_grade() {
        return 3;
    }
}
public extension Plane {
    public uint anti_grade() {
        return 1;
    }
}
public extension Point {
    public uint anti_grade() {
        return 3;
    }
}
public extension Scalar {
    public uint anti_grade() {
        return 4;
    }
}
