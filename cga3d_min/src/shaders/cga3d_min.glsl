#define_import_path cga3d_min

//
// AUTO-GENERATED - DO NOT MODIFY
//
// To contribute to this file, see the adjacent codegen package.
// https://github.com/AndrewBrownK/projective_ga/
//

struct Scalar {
    // 1
    float g0;
};

struct AntiScalar {
    // e12345
    float g0;
};

struct DualNum {
    // 1, e12345
    vec2 g0;
};

struct FlatPoint {
    // e15, e25, e35, e45
    vec4 g0;
};

struct Line {
    // -e145, -e245, -e345
    vec3 g0;
    // e235, -e135, e125
    vec3 g1;
};

struct Plane {
    // e2345, -e1345, e1245, -e1235
    vec4 g0;
};

struct RoundPoint {
    // e1, e2, e3
    vec3 g0;
    // e4, e5
    vec2 g1;
};

struct Dipole {
    // -e14, -e24, -e34
    vec3 g0;
    // e23, -e13, e12
    vec3 g1;
    // e15, e25, e35, e45
    vec4 g2;
};

struct Circle {
    // e234, -e134, e124, -e123
    vec4 g0;
    // -e145, -e245, -e345
    vec3 g1;
    // e235, -e135, e125
    vec3 g2;
};

struct Sphere {
    // e2345, -e1345, e1245
    vec3 g0;
    // e1234, -e1235
    vec2 g1;
};

struct Motor {
    // -e145, -e245, -e345, e12345
    vec4 g0;
    // e235, -e135, e125
    vec3 g1;
};

struct Flector {
    // e15, e25, e35, e45
    vec4 g0;
    // e2345, -e1345, e1245, -e1235
    vec4 g1;
};

struct MultiVector {
    // 1, e12345
    vec2 g0;
    // e1, e2, e3
    vec3 g1;
    // e4, e5
    vec2 g2;
    // -e14, -e24, -e34
    vec3 g3;
    // e23, -e13, e12
    vec3 g4;
    // e15, e25, e35, e45
    vec4 g5;
    // e234, -e134, e124, -e123
    vec4 g6;
    // -e145, -e245, -e345
    vec3 g7;
    // e235, -e135, e125
    vec3 g8;
    // e2345, -e1345, e1245
    vec3 g9;
    // e1234, -e1235
    vec2 g10;
};

AntiScalar anti_scalar__one() {
    return AntiScalar(0.0);
}

Circle circle__one() {
    return Circle(vec4(0.0), vec3(0.0), vec3(0.0));
}

Dipole dipole__one() {
    return Dipole(vec3(0.0), vec3(0.0), vec4(0.0));
}

DualNum dual_num__one() {
    return DualNum(vec2(1.0, 0.0));
}

FlatPoint flat_point__one() {
    return FlatPoint(vec4(0.0));
}

Flector flector__one() {
    return Flector(vec4(0.0), vec4(0.0));
}

Line line__one() {
    return Line(vec3(0.0), vec3(0.0));
}

Motor motor__one() {
    return Motor(vec4(0.0), vec3(0.0));
}

MultiVector multi_vector__one() {
    return MultiVector(vec2(1.0, 0.0), vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0), vec4(0.0), vec4(0.0), vec3(0.0), vec3(0.0), vec3(0.0), vec2(0.0));
}

Plane plane__one() {
    return Plane(vec4(0.0));
}

RoundPoint round_point__one() {
    return RoundPoint(vec3(0.0), vec2(0.0));
}

Scalar scalar__one() {
    return Scalar(1.0);
}

Sphere sphere__one() {
    return Sphere(vec3(0.0), vec2(0.0));
}

AntiScalar anti_scalar__unit() {
    return AntiScalar(1.0);
}

Circle circle__unit() {
    return Circle(vec4(1.0), vec3(1.0), vec3(1.0));
}

Dipole dipole__unit() {
    return Dipole(vec3(1.0), vec3(1.0), vec4(1.0));
}

DualNum dual_num__unit() {
    return DualNum(vec2(1.0));
}

FlatPoint flat_point__unit() {
    return FlatPoint(vec4(1.0));
}

Flector flector__unit() {
    return Flector(vec4(1.0), vec4(1.0));
}

Line line__unit() {
    return Line(vec3(1.0), vec3(1.0));
}

Motor motor__unit() {
    return Motor(vec4(1.0), vec3(1.0));
}

MultiVector multi_vector__unit() {
    return MultiVector(vec2(1.0), vec3(1.0), vec2(1.0), vec3(1.0), vec3(1.0), vec4(1.0), vec4(1.0), vec3(1.0), vec3(1.0), vec3(1.0), vec2(1.0));
}

Plane plane__unit() {
    return Plane(vec4(1.0));
}

RoundPoint round_point__unit() {
    return RoundPoint(vec3(1.0), vec2(1.0));
}

Scalar scalar__unit() {
    return Scalar(1.0);
}

Sphere sphere__unit() {
    return Sphere(vec3(1.0), vec2(1.0));
}

AntiScalar anti_scalar__zero() {
    return AntiScalar(0.0);
}

Circle circle__zero() {
    return Circle(vec4(0.0), vec3(0.0), vec3(0.0));
}

Dipole dipole__zero() {
    return Dipole(vec3(0.0), vec3(0.0), vec4(0.0));
}

DualNum dual_num__zero() {
    return DualNum(vec2(0.0));
}

FlatPoint flat_point__zero() {
    return FlatPoint(vec4(0.0));
}

Flector flector__zero() {
    return Flector(vec4(0.0), vec4(0.0));
}

Line line__zero() {
    return Line(vec3(0.0), vec3(0.0));
}

Motor motor__zero() {
    return Motor(vec4(0.0), vec3(0.0));
}

MultiVector multi_vector__zero() {
    return MultiVector(vec2(0.0), vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0), vec4(0.0), vec4(0.0), vec3(0.0), vec3(0.0), vec3(0.0), vec2(0.0));
}

Plane plane__zero() {
    return Plane(vec4(0.0));
}

RoundPoint round_point__zero() {
    return RoundPoint(vec3(0.0), vec2(0.0));
}

Scalar scalar__zero() {
    return Scalar(0.0);
}

Sphere sphere__zero() {
    return Sphere(vec3(0.0), vec2(0.0));
}

AntiScalar anti_scalar__neg(AntiScalar self) {
    return AntiScalar(-self.g0);
}

Circle circle__neg(Circle self) {
    return Circle(self.g0 * vec4(-1.0), self.g1 * vec3(-1.0), self.g2 * vec3(-1.0));
}

Dipole dipole__neg(Dipole self) {
    return Dipole(self.g0 * vec3(-1.0), self.g1 * vec3(-1.0), self.g2 * vec4(-1.0));
}

DualNum dual_num__neg(DualNum self) {
    return DualNum(self.g0 * vec2(-1.0));
}

FlatPoint flat_point__neg(FlatPoint self) {
    return FlatPoint(self.g0 * vec4(-1.0));
}

Flector flector__neg(Flector self) {
    return Flector(self.g0 * vec4(-1.0), self.g1 * vec4(-1.0));
}

Line line__neg(Line self) {
    return Line(self.g0 * vec3(-1.0), self.g1 * vec3(-1.0));
}

Motor motor__neg(Motor self) {
    return Motor(self.g0 * vec4(-1.0), self.g1 * vec3(-1.0));
}

MultiVector multi_vector__neg(MultiVector self) {
    return MultiVector(self.g0 * vec2(-1.0), self.g1 * vec3(-1.0), self.g2 * vec2(-1.0), self.g3 * vec3(-1.0), self.g4 * vec3(-1.0), self.g5 * vec4(-1.0), self.g6 * vec4(-1.0), self.g7 * vec3(-1.0), self.g8 * vec3(-1.0), self.g9 * vec3(-1.0), self.g10 * vec2(-1.0));
}

Plane plane__neg(Plane self) {
    return Plane(self.g0 * vec4(-1.0));
}

RoundPoint round_point__neg(RoundPoint self) {
    return RoundPoint(self.g0 * vec3(-1.0), self.g1 * vec2(-1.0));
}

Scalar scalar__neg(Scalar self) {
    return Scalar(-self.g0);
}

Sphere sphere__neg(Sphere self) {
    return Sphere(self.g0 * vec3(-1.0), self.g1 * vec2(-1.0));
}

AntiScalar anti_scalar__add__anti_scalar(AntiScalar self, AntiScalar other) {
    return AntiScalar(self.g0 + other.g0);
}

MultiVector anti_scalar__add__circle(AntiScalar self, Circle other) {
    return MultiVector(vec2(0.0, self.g0), vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0), vec4(0.0), other.g0, other.g1, other.g2, vec3(0.0), vec2(0.0));
}

MultiVector anti_scalar__add__dipole(AntiScalar self, Dipole other) {
    return MultiVector(vec2(0.0, self.g0), vec3(0.0), vec2(0.0), other.g0, other.g1, other.g2, vec4(0.0), vec3(0.0), vec3(0.0), vec3(0.0), vec2(0.0));
}

DualNum anti_scalar__add__dual_num(AntiScalar self, DualNum other) {
    return DualNum(vec2(0.0, self.g0) + other.g0);
}

MultiVector anti_scalar__add__flat_point(AntiScalar self, FlatPoint other) {
    return MultiVector(vec2(0.0, self.g0), vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0), other.g0, vec4(0.0), vec3(0.0), vec3(0.0), vec3(0.0), vec2(0.0));
}

MultiVector anti_scalar__add__flector(AntiScalar self, Flector other) {
    return MultiVector(vec2(0.0, self.g0), vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0), other.g0, vec4(0.0), vec3(0.0), vec3(0.0), vec3(other.g1.x, other.g1.y, other.g1.z), vec2(0.0, other.g1.w));
}

Motor anti_scalar__add__line(AntiScalar self, Line other) {
    return Motor(vec4(0.0, 0.0, 0.0, self.g0) + vec4(other.g0.x, other.g0.y, other.g0.z, 0.0), other.g1);
}

Motor anti_scalar__add__motor(AntiScalar self, Motor other) {
    return Motor(vec4(0.0, 0.0, 0.0, self.g0) + other.g0, other.g1);
}

MultiVector anti_scalar__add__multi_vector(AntiScalar self, MultiVector other) {
    return MultiVector(vec2(0.0, self.g0) + other.g0, other.g1, other.g2, other.g3, other.g4, other.g5, other.g6, other.g7, other.g8, other.g9, other.g10);
}

MultiVector anti_scalar__add__plane(AntiScalar self, Plane other) {
    return MultiVector(vec2(0.0, self.g0), vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0), vec4(0.0), vec4(0.0), vec3(0.0), vec3(0.0), vec3(other.g0.x, other.g0.y, other.g0.z), vec2(0.0, other.g0.w));
}

MultiVector anti_scalar__add__round_point(AntiScalar self, RoundPoint other) {
    return MultiVector(vec2(0.0, self.g0), other.g0, other.g1, vec3(0.0), vec3(0.0), vec4(0.0), vec4(0.0), vec3(0.0), vec3(0.0), vec3(0.0), vec2(0.0));
}

DualNum anti_scalar__add__scalar(AntiScalar self, Scalar other) {
    return DualNum(vec2(0.0, self.g0) + vec2(other.g0, 0.0));
}

MultiVector anti_scalar__add__sphere(AntiScalar self, Sphere other) {
    return MultiVector(vec2(0.0, self.g0), vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0), vec4(0.0), vec4(0.0), vec3(0.0), vec3(0.0), other.g0, other.g1);
}

MultiVector circle__add__anti_scalar(Circle self, AntiScalar other) {
    return MultiVector(vec2(0.0, other.g0), vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0), vec4(0.0), self.g0, self.g1, self.g2, vec3(0.0), vec2(0.0));
}

Circle circle__add__circle(Circle self, Circle other) {
    return Circle(self.g0 + other.g0, self.g1 + other.g1, self.g2 + other.g2);
}

MultiVector circle__add__dipole(Circle self, Dipole other) {
    return MultiVector(vec2(0.0), vec3(0.0), vec2(0.0), other.g0, other.g1, other.g2, self.g0, self.g1, self.g2, vec3(0.0), vec2(0.0));
}

MultiVector circle__add__dual_num(Circle self, DualNum other) {
    return MultiVector(other.g0, vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0), vec4(0.0), self.g0, self.g1, self.g2, vec3(0.0), vec2(0.0));
}

MultiVector circle__add__flat_point(Circle self, FlatPoint other) {
    return MultiVector(vec2(0.0), vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0), other.g0, self.g0, self.g1, self.g2, vec3(0.0), vec2(0.0));
}

MultiVector circle__add__flector(Circle self, Flector other) {
    return MultiVector(vec2(0.0), vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0), other.g0, self.g0, self.g1, self.g2, vec3(other.g1.x, other.g1.y, other.g1.z), vec2(0.0, other.g1.w));
}

Circle circle__add__line(Circle self, Line other) {
    return Circle(self.g0, self.g1 + other.g0, self.g2 + other.g1);
}

MultiVector circle__add__motor(Circle self, Motor other) {
    return MultiVector(vec2(0.0, other.g0.w), vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0), vec4(0.0), self.g0, self.g1 + vec3(other.g0.x, other.g0.y, other.g0.z), self.g2 + other.g1, vec3(0.0), vec2(0.0));
}

MultiVector circle__add__multi_vector(Circle self, MultiVector other) {
    return MultiVector(other.g0, other.g1, other.g2, other.g3, other.g4, other.g5, self.g0 + other.g6, self.g1 + other.g7, self.g2 + other.g8, other.g9, other.g10);
}

MultiVector circle__add__plane(Circle self, Plane other) {
    return MultiVector(vec2(0.0), vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0), vec4(0.0), self.g0, self.g1, self.g2, vec3(other.g0.x, other.g0.y, other.g0.z), vec2(0.0, other.g0.w));
}

MultiVector circle__add__round_point(Circle self, RoundPoint other) {
    return MultiVector(vec2(0.0), other.g0, other.g1, vec3(0.0), vec3(0.0), vec4(0.0), self.g0, self.g1, self.g2, vec3(0.0), vec2(0.0));
}

MultiVector circle__add__scalar(Circle self, Scalar other) {
    return MultiVector(vec2(other.g0, 0.0), vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0), vec4(0.0), self.g0, self.g1, self.g2, vec3(0.0), vec2(0.0));
}

MultiVector circle__add__sphere(Circle self, Sphere other) {
    return MultiVector(vec2(0.0), vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0), vec4(0.0), self.g0, self.g1, self.g2, other.g0, other.g1);
}

MultiVector dipole__add__anti_scalar(Dipole self, AntiScalar other) {
    return MultiVector(vec2(0.0, other.g0), vec3(0.0), vec2(0.0), self.g0, self.g1, self.g2, vec4(0.0), vec3(0.0), vec3(0.0), vec3(0.0), vec2(0.0));
}

MultiVector dipole__add__circle(Dipole self, Circle other) {
    return MultiVector(vec2(0.0), vec3(0.0), vec2(0.0), self.g0, self.g1, self.g2, other.g0, other.g1, other.g2, vec3(0.0), vec2(0.0));
}

Dipole dipole__add__dipole(Dipole self, Dipole other) {
    return Dipole(self.g0 + other.g0, self.g1 + other.g1, self.g2 + other.g2);
}

MultiVector dipole__add__dual_num(Dipole self, DualNum other) {
    return MultiVector(other.g0, vec3(0.0), vec2(0.0), self.g0, self.g1, self.g2, vec4(0.0), vec3(0.0), vec3(0.0), vec3(0.0), vec2(0.0));
}

Dipole dipole__add__flat_point(Dipole self, FlatPoint other) {
    return Dipole(self.g0, self.g1, self.g2 + other.g0);
}

MultiVector dipole__add__flector(Dipole self, Flector other) {
    return MultiVector(vec2(0.0), vec3(0.0), vec2(0.0), self.g0, self.g1, self.g2 + other.g0, vec4(0.0), vec3(0.0), vec3(0.0), vec3(other.g1.x, other.g1.y, other.g1.z), vec2(0.0, other.g1.w));
}

MultiVector dipole__add__line(Dipole self, Line other) {
    return MultiVector(vec2(0.0), vec3(0.0), vec2(0.0), self.g0, self.g1, self.g2, vec4(0.0), other.g0, other.g1, vec3(0.0), vec2(0.0));
}

MultiVector dipole__add__motor(Dipole self, Motor other) {
    return MultiVector(vec2(0.0, other.g0.w), vec3(0.0), vec2(0.0), self.g0, self.g1, self.g2, vec4(0.0), vec3(other.g0.x, other.g0.y, other.g0.z), other.g1, vec3(0.0), vec2(0.0));
}

MultiVector dipole__add__multi_vector(Dipole self, MultiVector other) {
    return MultiVector(other.g0, other.g1, other.g2, self.g0 + other.g3, self.g1 + other.g4, self.g2 + other.g5, other.g6, other.g7, other.g8, other.g9, other.g10);
}

MultiVector dipole__add__plane(Dipole self, Plane other) {
    return MultiVector(vec2(0.0), vec3(0.0), vec2(0.0), self.g0, self.g1, self.g2, vec4(0.0), vec3(0.0), vec3(0.0), vec3(other.g0.x, other.g0.y, other.g0.z), vec2(0.0, other.g0.w));
}

MultiVector dipole__add__round_point(Dipole self, RoundPoint other) {
    return MultiVector(vec2(0.0), other.g0, other.g1, self.g0, self.g1, self.g2, vec4(0.0), vec3(0.0), vec3(0.0), vec3(0.0), vec2(0.0));
}

MultiVector dipole__add__scalar(Dipole self, Scalar other) {
    return MultiVector(vec2(other.g0, 0.0), vec3(0.0), vec2(0.0), self.g0, self.g1, self.g2, vec4(0.0), vec3(0.0), vec3(0.0), vec3(0.0), vec2(0.0));
}

MultiVector dipole__add__sphere(Dipole self, Sphere other) {
    return MultiVector(vec2(0.0), vec3(0.0), vec2(0.0), self.g0, self.g1, self.g2, vec4(0.0), vec3(0.0), vec3(0.0), other.g0, other.g1);
}

DualNum dual_num__add__anti_scalar(DualNum self, AntiScalar other) {
    return DualNum(self.g0 + vec2(0.0, other.g0));
}

MultiVector dual_num__add__circle(DualNum self, Circle other) {
    return MultiVector(self.g0, vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0), vec4(0.0), other.g0, other.g1, other.g2, vec3(0.0), vec2(0.0));
}

MultiVector dual_num__add__dipole(DualNum self, Dipole other) {
    return MultiVector(self.g0, vec3(0.0), vec2(0.0), other.g0, other.g1, other.g2, vec4(0.0), vec3(0.0), vec3(0.0), vec3(0.0), vec2(0.0));
}

DualNum dual_num__add__dual_num(DualNum self, DualNum other) {
    return DualNum(self.g0 + other.g0);
}

MultiVector dual_num__add__flat_point(DualNum self, FlatPoint other) {
    return MultiVector(self.g0, vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0), other.g0, vec4(0.0), vec3(0.0), vec3(0.0), vec3(0.0), vec2(0.0));
}

MultiVector dual_num__add__flector(DualNum self, Flector other) {
    return MultiVector(self.g0, vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0), other.g0, vec4(0.0), vec3(0.0), vec3(0.0), vec3(other.g1.x, other.g1.y, other.g1.z), vec2(0.0, other.g1.w));
}

MultiVector dual_num__add__line(DualNum self, Line other) {
    return MultiVector(self.g0, vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0), vec4(0.0), vec4(0.0), other.g0, other.g1, vec3(0.0), vec2(0.0));
}

MultiVector dual_num__add__motor(DualNum self, Motor other) {
    return MultiVector(self.g0 + vec2(0.0, other.g0.w), vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0), vec4(0.0), vec4(0.0), vec3(other.g0.x, other.g0.y, other.g0.z), other.g1, vec3(0.0), vec2(0.0));
}

MultiVector dual_num__add__multi_vector(DualNum self, MultiVector other) {
    return MultiVector(self.g0 + other.g0, other.g1, other.g2, other.g3, other.g4, other.g5, other.g6, other.g7, other.g8, other.g9, other.g10);
}

MultiVector dual_num__add__plane(DualNum self, Plane other) {
    return MultiVector(self.g0, vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0), vec4(0.0), vec4(0.0), vec3(0.0), vec3(0.0), vec3(other.g0.x, other.g0.y, other.g0.z), vec2(0.0, other.g0.w));
}

MultiVector dual_num__add__round_point(DualNum self, RoundPoint other) {
    return MultiVector(self.g0, other.g0, other.g1, vec3(0.0), vec3(0.0), vec4(0.0), vec4(0.0), vec3(0.0), vec3(0.0), vec3(0.0), vec2(0.0));
}

DualNum dual_num__add__scalar(DualNum self, Scalar other) {
    return DualNum(self.g0 + vec2(other.g0, 0.0));
}

MultiVector dual_num__add__sphere(DualNum self, Sphere other) {
    return MultiVector(self.g0, vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0), vec4(0.0), vec4(0.0), vec3(0.0), vec3(0.0), other.g0, other.g1);
}

MultiVector flat_point__add__anti_scalar(FlatPoint self, AntiScalar other) {
    return MultiVector(vec2(0.0, other.g0), vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0), self.g0, vec4(0.0), vec3(0.0), vec3(0.0), vec3(0.0), vec2(0.0));
}

MultiVector flat_point__add__circle(FlatPoint self, Circle other) {
    return MultiVector(vec2(0.0), vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0), self.g0, other.g0, other.g1, other.g2, vec3(0.0), vec2(0.0));
}

Dipole flat_point__add__dipole(FlatPoint self, Dipole other) {
    return Dipole(other.g0, other.g1, self.g0 + other.g2);
}

MultiVector flat_point__add__dual_num(FlatPoint self, DualNum other) {
    return MultiVector(other.g0, vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0), self.g0, vec4(0.0), vec3(0.0), vec3(0.0), vec3(0.0), vec2(0.0));
}

FlatPoint flat_point__add__flat_point(FlatPoint self, FlatPoint other) {
    return FlatPoint(self.g0 + other.g0);
}

Flector flat_point__add__flector(FlatPoint self, Flector other) {
    return Flector(self.g0 + other.g0, other.g1);
}

MultiVector flat_point__add__line(FlatPoint self, Line other) {
    return MultiVector(vec2(0.0), vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0), self.g0, vec4(0.0), other.g0, other.g1, vec3(0.0), vec2(0.0));
}

MultiVector flat_point__add__motor(FlatPoint self, Motor other) {
    return MultiVector(vec2(0.0, other.g0.w), vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0), self.g0, vec4(0.0), vec3(other.g0.x, other.g0.y, other.g0.z), other.g1, vec3(0.0), vec2(0.0));
}

MultiVector flat_point__add__multi_vector(FlatPoint self, MultiVector other) {
    return MultiVector(other.g0, other.g1, other.g2, other.g3, other.g4, self.g0 + other.g5, other.g6, other.g7, other.g8, other.g9, other.g10);
}

Flector flat_point__add__plane(FlatPoint self, Plane other) {
    return Flector(self.g0, other.g0);
}

MultiVector flat_point__add__round_point(FlatPoint self, RoundPoint other) {
    return MultiVector(vec2(0.0), other.g0, other.g1, vec3(0.0), vec3(0.0), self.g0, vec4(0.0), vec3(0.0), vec3(0.0), vec3(0.0), vec2(0.0));
}

MultiVector flat_point__add__scalar(FlatPoint self, Scalar other) {
    return MultiVector(vec2(other.g0, 0.0), vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0), self.g0, vec4(0.0), vec3(0.0), vec3(0.0), vec3(0.0), vec2(0.0));
}

MultiVector flat_point__add__sphere(FlatPoint self, Sphere other) {
    return MultiVector(vec2(0.0), vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0), self.g0, vec4(0.0), vec3(0.0), vec3(0.0), other.g0, other.g1);
}

MultiVector flector__add__anti_scalar(Flector self, AntiScalar other) {
    return MultiVector(vec2(0.0, other.g0), vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0), self.g0, vec4(0.0), vec3(0.0), vec3(0.0), vec3(self.g1.x, self.g1.y, self.g1.z), vec2(0.0, self.g1.w));
}

MultiVector flector__add__circle(Flector self, Circle other) {
    return MultiVector(vec2(0.0), vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0), self.g0, other.g0, other.g1, other.g2, vec3(self.g1.x, self.g1.y, self.g1.z), vec2(0.0, self.g1.w));
}

MultiVector flector__add__dipole(Flector self, Dipole other) {
    return MultiVector(vec2(0.0), vec3(0.0), vec2(0.0), other.g0, other.g1, self.g0 + other.g2, vec4(0.0), vec3(0.0), vec3(0.0), vec3(self.g1.x, self.g1.y, self.g1.z), vec2(0.0, self.g1.w));
}

MultiVector flector__add__dual_num(Flector self, DualNum other) {
    return MultiVector(other.g0, vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0), self.g0, vec4(0.0), vec3(0.0), vec3(0.0), vec3(self.g1.x, self.g1.y, self.g1.z), vec2(0.0, self.g1.w));
}

Flector flector__add__flat_point(Flector self, FlatPoint other) {
    return Flector(self.g0 + other.g0, self.g1);
}

Flector flector__add__flector(Flector self, Flector other) {
    return Flector(self.g0 + other.g0, self.g1 + other.g1);
}

MultiVector flector__add__line(Flector self, Line other) {
    return MultiVector(vec2(0.0), vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0), self.g0, vec4(0.0), other.g0, other.g1, vec3(self.g1.x, self.g1.y, self.g1.z), vec2(0.0, self.g1.w));
}

MultiVector flector__add__motor(Flector self, Motor other) {
    return MultiVector(vec2(0.0, other.g0.w), vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0), self.g0, vec4(0.0), vec3(other.g0.x, other.g0.y, other.g0.z), other.g1, vec3(self.g1.x, self.g1.y, self.g1.z), vec2(0.0, self.g1.w));
}

MultiVector flector__add__multi_vector(Flector self, MultiVector other) {
    return MultiVector(other.g0, other.g1, other.g2, other.g3, other.g4, self.g0 + other.g5, other.g6, other.g7, other.g8, vec3(self.g1.x, self.g1.y, self.g1.z) + other.g9, vec2(0.0, self.g1.w) + other.g10);
}

Flector flector__add__plane(Flector self, Plane other) {
    return Flector(self.g0, self.g1 + other.g0);
}

MultiVector flector__add__round_point(Flector self, RoundPoint other) {
    return MultiVector(vec2(0.0), other.g0, other.g1, vec3(0.0), vec3(0.0), self.g0, vec4(0.0), vec3(0.0), vec3(0.0), vec3(self.g1.x, self.g1.y, self.g1.z), vec2(0.0, self.g1.w));
}

MultiVector flector__add__scalar(Flector self, Scalar other) {
    return MultiVector(vec2(other.g0, 0.0), vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0), self.g0, vec4(0.0), vec3(0.0), vec3(0.0), vec3(self.g1.x, self.g1.y, self.g1.z), vec2(0.0, self.g1.w));
}

MultiVector flector__add__sphere(Flector self, Sphere other) {
    return MultiVector(vec2(0.0), vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0), self.g0, vec4(0.0), vec3(0.0), vec3(0.0), vec3(self.g1.x, self.g1.y, self.g1.z) + other.g0, vec2(0.0, self.g1.w) + other.g1);
}

Motor line__add__anti_scalar(Line self, AntiScalar other) {
    return Motor(vec4(self.g0.x, self.g0.y, self.g0.z, 0.0) + vec4(0.0, 0.0, 0.0, other.g0), self.g1);
}

Circle line__add__circle(Line self, Circle other) {
    return Circle(other.g0, self.g0 + other.g1, self.g1 + other.g2);
}

MultiVector line__add__dipole(Line self, Dipole other) {
    return MultiVector(vec2(0.0), vec3(0.0), vec2(0.0), other.g0, other.g1, other.g2, vec4(0.0), self.g0, self.g1, vec3(0.0), vec2(0.0));
}

MultiVector line__add__dual_num(Line self, DualNum other) {
    return MultiVector(other.g0, vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0), vec4(0.0), vec4(0.0), self.g0, self.g1, vec3(0.0), vec2(0.0));
}

MultiVector line__add__flat_point(Line self, FlatPoint other) {
    return MultiVector(vec2(0.0), vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0), other.g0, vec4(0.0), self.g0, self.g1, vec3(0.0), vec2(0.0));
}

MultiVector line__add__flector(Line self, Flector other) {
    return MultiVector(vec2(0.0), vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0), other.g0, vec4(0.0), self.g0, self.g1, vec3(other.g1.x, other.g1.y, other.g1.z), vec2(0.0, other.g1.w));
}

Line line__add__line(Line self, Line other) {
    return Line(self.g0 + other.g0, self.g1 + other.g1);
}

Motor line__add__motor(Line self, Motor other) {
    return Motor(vec4(self.g0.x, self.g0.y, self.g0.z, 0.0) + other.g0, self.g1 + other.g1);
}

MultiVector line__add__multi_vector(Line self, MultiVector other) {
    return MultiVector(other.g0, other.g1, other.g2, other.g3, other.g4, other.g5, other.g6, self.g0 + other.g7, self.g1 + other.g8, other.g9, other.g10);
}

MultiVector line__add__plane(Line self, Plane other) {
    return MultiVector(vec2(0.0), vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0), vec4(0.0), vec4(0.0), self.g0, self.g1, vec3(other.g0.x, other.g0.y, other.g0.z), vec2(0.0, other.g0.w));
}

MultiVector line__add__round_point(Line self, RoundPoint other) {
    return MultiVector(vec2(0.0), other.g0, other.g1, vec3(0.0), vec3(0.0), vec4(0.0), vec4(0.0), self.g0, self.g1, vec3(0.0), vec2(0.0));
}

MultiVector line__add__scalar(Line self, Scalar other) {
    return MultiVector(vec2(other.g0, 0.0), vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0), vec4(0.0), vec4(0.0), self.g0, self.g1, vec3(0.0), vec2(0.0));
}

MultiVector line__add__sphere(Line self, Sphere other) {
    return MultiVector(vec2(0.0), vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0), vec4(0.0), vec4(0.0), self.g0, self.g1, other.g0, other.g1);
}

Motor motor__add__anti_scalar(Motor self, AntiScalar other) {
    return Motor(self.g0 + vec4(0.0, 0.0, 0.0, other.g0), self.g1);
}

MultiVector motor__add__circle(Motor self, Circle other) {
    return MultiVector(vec2(0.0, self.g0.w), vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0), vec4(0.0), other.g0, vec3(self.g0.x, self.g0.y, self.g0.z) + other.g1, self.g1 + other.g2, vec3(0.0), vec2(0.0));
}

MultiVector motor__add__dipole(Motor self, Dipole other) {
    return MultiVector(vec2(0.0, self.g0.w), vec3(0.0), vec2(0.0), other.g0, other.g1, other.g2, vec4(0.0), vec3(self.g0.x, self.g0.y, self.g0.z), self.g1, vec3(0.0), vec2(0.0));
}

MultiVector motor__add__dual_num(Motor self, DualNum other) {
    return MultiVector(vec2(0.0, self.g0.w) + other.g0, vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0), vec4(0.0), vec4(0.0), vec3(self.g0.x, self.g0.y, self.g0.z), self.g1, vec3(0.0), vec2(0.0));
}

MultiVector motor__add__flat_point(Motor self, FlatPoint other) {
    return MultiVector(vec2(0.0, self.g0.w), vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0), other.g0, vec4(0.0), vec3(self.g0.x, self.g0.y, self.g0.z), self.g1, vec3(0.0), vec2(0.0));
}

MultiVector motor__add__flector(Motor self, Flector other) {
    return MultiVector(vec2(0.0, self.g0.w), vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0), other.g0, vec4(0.0), vec3(self.g0.x, self.g0.y, self.g0.z), self.g1, vec3(other.g1.x, other.g1.y, other.g1.z), vec2(0.0, other.g1.w));
}

Motor motor__add__line(Motor self, Line other) {
    return Motor(self.g0 + vec4(other.g0.x, other.g0.y, other.g0.z, 0.0), self.g1 + other.g1);
}

Motor motor__add__motor(Motor self, Motor other) {
    return Motor(self.g0 + other.g0, self.g1 + other.g1);
}

MultiVector motor__add__multi_vector(Motor self, MultiVector other) {
    return MultiVector(vec2(0.0, self.g0.w) + other.g0, other.g1, other.g2, other.g3, other.g4, other.g5, other.g6, vec3(self.g0.x, self.g0.y, self.g0.z) + other.g7, self.g1 + other.g8, other.g9, other.g10);
}

MultiVector motor__add__plane(Motor self, Plane other) {
    return MultiVector(vec2(0.0, self.g0.w), vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0), vec4(0.0), vec4(0.0), vec3(self.g0.x, self.g0.y, self.g0.z), self.g1, vec3(other.g0.x, other.g0.y, other.g0.z), vec2(0.0, other.g0.w));
}

MultiVector motor__add__round_point(Motor self, RoundPoint other) {
    return MultiVector(vec2(0.0, self.g0.w), other.g0, other.g1, vec3(0.0), vec3(0.0), vec4(0.0), vec4(0.0), vec3(self.g0.x, self.g0.y, self.g0.z), self.g1, vec3(0.0), vec2(0.0));
}

MultiVector motor__add__scalar(Motor self, Scalar other) {
    return MultiVector(vec2(0.0, self.g0.w) + vec2(other.g0, 0.0), vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0), vec4(0.0), vec4(0.0), vec3(self.g0.x, self.g0.y, self.g0.z), self.g1, vec3(0.0), vec2(0.0));
}

MultiVector motor__add__sphere(Motor self, Sphere other) {
    return MultiVector(vec2(0.0, self.g0.w), vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0), vec4(0.0), vec4(0.0), vec3(self.g0.x, self.g0.y, self.g0.z), self.g1, other.g0, other.g1);
}

MultiVector multi_vector__add__anti_scalar(MultiVector self, AntiScalar other) {
    return MultiVector(self.g0 + vec2(0.0, other.g0), self.g1, self.g2, self.g3, self.g4, self.g5, self.g6, self.g7, self.g8, self.g9, self.g10);
}

MultiVector multi_vector__add__circle(MultiVector self, Circle other) {
    return MultiVector(self.g0, self.g1, self.g2, self.g3, self.g4, self.g5, self.g6 + other.g0, self.g7 + other.g1, self.g8 + other.g2, self.g9, self.g10);
}

MultiVector multi_vector__add__dipole(MultiVector self, Dipole other) {
    return MultiVector(self.g0, self.g1, self.g2, self.g3 + other.g0, self.g4 + other.g1, self.g5 + other.g2, self.g6, self.g7, self.g8, self.g9, self.g10);
}

MultiVector multi_vector__add__dual_num(MultiVector self, DualNum other) {
    return MultiVector(self.g0 + other.g0, self.g1, self.g2, self.g3, self.g4, self.g5, self.g6, self.g7, self.g8, self.g9, self.g10);
}

MultiVector multi_vector__add__flat_point(MultiVector self, FlatPoint other) {
    return MultiVector(self.g0, self.g1, self.g2, self.g3, self.g4, self.g5 + other.g0, self.g6, self.g7, self.g8, self.g9, self.g10);
}

MultiVector multi_vector__add__flector(MultiVector self, Flector other) {
    return MultiVector(self.g0, self.g1, self.g2, self.g3, self.g4, self.g5 + other.g0, self.g6, self.g7, self.g8, self.g9 + vec3(other.g1.x, other.g1.y, other.g1.z), self.g10 + vec2(0.0, other.g1.w));
}

MultiVector multi_vector__add__line(MultiVector self, Line other) {
    return MultiVector(self.g0, self.g1, self.g2, self.g3, self.g4, self.g5, self.g6, self.g7 + other.g0, self.g8 + other.g1, self.g9, self.g10);
}

MultiVector multi_vector__add__motor(MultiVector self, Motor other) {
    return MultiVector(self.g0 + vec2(0.0, other.g0.w), self.g1, self.g2, self.g3, self.g4, self.g5, self.g6, self.g7 + vec3(other.g0.x, other.g0.y, other.g0.z), self.g8 + other.g1, self.g9, self.g10);
}

MultiVector multi_vector__add__multi_vector(MultiVector self, MultiVector other) {
    return MultiVector(self.g0 + other.g0, self.g1 + other.g1, self.g2 + other.g2, self.g3 + other.g3, self.g4 + other.g4, self.g5 + other.g5, self.g6 + other.g6, self.g7 + other.g7, self.g8 + other.g8, self.g9 + other.g9, self.g10 + other.g10);
}

MultiVector multi_vector__add__plane(MultiVector self, Plane other) {
    return MultiVector(self.g0, self.g1, self.g2, self.g3, self.g4, self.g5, self.g6, self.g7, self.g8, self.g9 + vec3(other.g0.x, other.g0.y, other.g0.z), self.g10 + vec2(0.0, other.g0.w));
}

MultiVector multi_vector__add__round_point(MultiVector self, RoundPoint other) {
    return MultiVector(self.g0, self.g1 + other.g0, self.g2 + other.g1, self.g3, self.g4, self.g5, self.g6, self.g7, self.g8, self.g9, self.g10);
}

MultiVector multi_vector__add__scalar(MultiVector self, Scalar other) {
    return MultiVector(self.g0 + vec2(other.g0, 0.0), self.g1, self.g2, self.g3, self.g4, self.g5, self.g6, self.g7, self.g8, self.g9, self.g10);
}

MultiVector multi_vector__add__sphere(MultiVector self, Sphere other) {
    return MultiVector(self.g0, self.g1, self.g2, self.g3, self.g4, self.g5, self.g6, self.g7, self.g8, self.g9 + other.g0, self.g10 + other.g1);
}

MultiVector plane__add__anti_scalar(Plane self, AntiScalar other) {
    return MultiVector(vec2(0.0, other.g0), vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0), vec4(0.0), vec4(0.0), vec3(0.0), vec3(0.0), vec3(self.g0.x, self.g0.y, self.g0.z), vec2(0.0, self.g0.w));
}

MultiVector plane__add__circle(Plane self, Circle other) {
    return MultiVector(vec2(0.0), vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0), vec4(0.0), other.g0, other.g1, other.g2, vec3(self.g0.x, self.g0.y, self.g0.z), vec2(0.0, self.g0.w));
}

MultiVector plane__add__dipole(Plane self, Dipole other) {
    return MultiVector(vec2(0.0), vec3(0.0), vec2(0.0), other.g0, other.g1, other.g2, vec4(0.0), vec3(0.0), vec3(0.0), vec3(self.g0.x, self.g0.y, self.g0.z), vec2(0.0, self.g0.w));
}

MultiVector plane__add__dual_num(Plane self, DualNum other) {
    return MultiVector(other.g0, vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0), vec4(0.0), vec4(0.0), vec3(0.0), vec3(0.0), vec3(self.g0.x, self.g0.y, self.g0.z), vec2(0.0, self.g0.w));
}

Flector plane__add__flat_point(Plane self, FlatPoint other) {
    return Flector(other.g0, self.g0);
}

Flector plane__add__flector(Plane self, Flector other) {
    return Flector(other.g0, self.g0 + other.g1);
}

MultiVector plane__add__line(Plane self, Line other) {
    return MultiVector(vec2(0.0), vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0), vec4(0.0), vec4(0.0), other.g0, other.g1, vec3(self.g0.x, self.g0.y, self.g0.z), vec2(0.0, self.g0.w));
}

MultiVector plane__add__motor(Plane self, Motor other) {
    return MultiVector(vec2(0.0, other.g0.w), vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0), vec4(0.0), vec4(0.0), vec3(other.g0.x, other.g0.y, other.g0.z), other.g1, vec3(self.g0.x, self.g0.y, self.g0.z), vec2(0.0, self.g0.w));
}

MultiVector plane__add__multi_vector(Plane self, MultiVector other) {
    return MultiVector(other.g0, other.g1, other.g2, other.g3, other.g4, other.g5, other.g6, other.g7, other.g8, vec3(self.g0.x, self.g0.y, self.g0.z) + other.g9, vec2(0.0, self.g0.w) + other.g10);
}

Plane plane__add__plane(Plane self, Plane other) {
    return Plane(self.g0 + other.g0);
}

MultiVector plane__add__round_point(Plane self, RoundPoint other) {
    return MultiVector(vec2(0.0), other.g0, other.g1, vec3(0.0), vec3(0.0), vec4(0.0), vec4(0.0), vec3(0.0), vec3(0.0), vec3(self.g0.x, self.g0.y, self.g0.z), vec2(0.0, self.g0.w));
}

MultiVector plane__add__scalar(Plane self, Scalar other) {
    return MultiVector(vec2(other.g0, 0.0), vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0), vec4(0.0), vec4(0.0), vec3(0.0), vec3(0.0), vec3(self.g0.x, self.g0.y, self.g0.z), vec2(0.0, self.g0.w));
}

Sphere plane__add__sphere(Plane self, Sphere other) {
    return Sphere(vec3(self.g0.x, self.g0.y, self.g0.z) + other.g0, vec2(0.0, self.g0.w) + other.g1);
}

MultiVector round_point__add__anti_scalar(RoundPoint self, AntiScalar other) {
    return MultiVector(vec2(0.0, other.g0), self.g0, self.g1, vec3(0.0), vec3(0.0), vec4(0.0), vec4(0.0), vec3(0.0), vec3(0.0), vec3(0.0), vec2(0.0));
}

MultiVector round_point__add__circle(RoundPoint self, Circle other) {
    return MultiVector(vec2(0.0), self.g0, self.g1, vec3(0.0), vec3(0.0), vec4(0.0), other.g0, other.g1, other.g2, vec3(0.0), vec2(0.0));
}

MultiVector round_point__add__dipole(RoundPoint self, Dipole other) {
    return MultiVector(vec2(0.0), self.g0, self.g1, other.g0, other.g1, other.g2, vec4(0.0), vec3(0.0), vec3(0.0), vec3(0.0), vec2(0.0));
}

MultiVector round_point__add__dual_num(RoundPoint self, DualNum other) {
    return MultiVector(other.g0, self.g0, self.g1, vec3(0.0), vec3(0.0), vec4(0.0), vec4(0.0), vec3(0.0), vec3(0.0), vec3(0.0), vec2(0.0));
}

MultiVector round_point__add__flat_point(RoundPoint self, FlatPoint other) {
    return MultiVector(vec2(0.0), self.g0, self.g1, vec3(0.0), vec3(0.0), other.g0, vec4(0.0), vec3(0.0), vec3(0.0), vec3(0.0), vec2(0.0));
}

MultiVector round_point__add__flector(RoundPoint self, Flector other) {
    return MultiVector(vec2(0.0), self.g0, self.g1, vec3(0.0), vec3(0.0), other.g0, vec4(0.0), vec3(0.0), vec3(0.0), vec3(other.g1.x, other.g1.y, other.g1.z), vec2(0.0, other.g1.w));
}

MultiVector round_point__add__line(RoundPoint self, Line other) {
    return MultiVector(vec2(0.0), self.g0, self.g1, vec3(0.0), vec3(0.0), vec4(0.0), vec4(0.0), other.g0, other.g1, vec3(0.0), vec2(0.0));
}

MultiVector round_point__add__motor(RoundPoint self, Motor other) {
    return MultiVector(vec2(0.0, other.g0.w), self.g0, self.g1, vec3(0.0), vec3(0.0), vec4(0.0), vec4(0.0), vec3(other.g0.x, other.g0.y, other.g0.z), other.g1, vec3(0.0), vec2(0.0));
}

MultiVector round_point__add__multi_vector(RoundPoint self, MultiVector other) {
    return MultiVector(other.g0, self.g0 + other.g1, self.g1 + other.g2, other.g3, other.g4, other.g5, other.g6, other.g7, other.g8, other.g9, other.g10);
}

MultiVector round_point__add__plane(RoundPoint self, Plane other) {
    return MultiVector(vec2(0.0), self.g0, self.g1, vec3(0.0), vec3(0.0), vec4(0.0), vec4(0.0), vec3(0.0), vec3(0.0), vec3(other.g0.x, other.g0.y, other.g0.z), vec2(0.0, other.g0.w));
}

RoundPoint round_point__add__round_point(RoundPoint self, RoundPoint other) {
    return RoundPoint(self.g0 + other.g0, self.g1 + other.g1);
}

MultiVector round_point__add__scalar(RoundPoint self, Scalar other) {
    return MultiVector(vec2(other.g0, 0.0), self.g0, self.g1, vec3(0.0), vec3(0.0), vec4(0.0), vec4(0.0), vec3(0.0), vec3(0.0), vec3(0.0), vec2(0.0));
}

MultiVector round_point__add__sphere(RoundPoint self, Sphere other) {
    return MultiVector(vec2(0.0), self.g0, self.g1, vec3(0.0), vec3(0.0), vec4(0.0), vec4(0.0), vec3(0.0), vec3(0.0), other.g0, other.g1);
}

DualNum scalar__add__anti_scalar(Scalar self, AntiScalar other) {
    return DualNum(vec2(self.g0, 0.0) + vec2(0.0, other.g0));
}

MultiVector scalar__add__circle(Scalar self, Circle other) {
    return MultiVector(vec2(self.g0, 0.0), vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0), vec4(0.0), other.g0, other.g1, other.g2, vec3(0.0), vec2(0.0));
}

MultiVector scalar__add__dipole(Scalar self, Dipole other) {
    return MultiVector(vec2(self.g0, 0.0), vec3(0.0), vec2(0.0), other.g0, other.g1, other.g2, vec4(0.0), vec3(0.0), vec3(0.0), vec3(0.0), vec2(0.0));
}

DualNum scalar__add__dual_num(Scalar self, DualNum other) {
    return DualNum(vec2(self.g0, 0.0) + other.g0);
}

MultiVector scalar__add__flat_point(Scalar self, FlatPoint other) {
    return MultiVector(vec2(self.g0, 0.0), vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0), other.g0, vec4(0.0), vec3(0.0), vec3(0.0), vec3(0.0), vec2(0.0));
}

MultiVector scalar__add__flector(Scalar self, Flector other) {
    return MultiVector(vec2(self.g0, 0.0), vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0), other.g0, vec4(0.0), vec3(0.0), vec3(0.0), vec3(other.g1.x, other.g1.y, other.g1.z), vec2(0.0, other.g1.w));
}

MultiVector scalar__add__line(Scalar self, Line other) {
    return MultiVector(vec2(self.g0, 0.0), vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0), vec4(0.0), vec4(0.0), other.g0, other.g1, vec3(0.0), vec2(0.0));
}

MultiVector scalar__add__motor(Scalar self, Motor other) {
    return MultiVector(vec2(self.g0, 0.0) + vec2(0.0, other.g0.w), vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0), vec4(0.0), vec4(0.0), vec3(other.g0.x, other.g0.y, other.g0.z), other.g1, vec3(0.0), vec2(0.0));
}

MultiVector scalar__add__multi_vector(Scalar self, MultiVector other) {
    return MultiVector(vec2(self.g0, 0.0) + other.g0, other.g1, other.g2, other.g3, other.g4, other.g5, other.g6, other.g7, other.g8, other.g9, other.g10);
}

MultiVector scalar__add__plane(Scalar self, Plane other) {
    return MultiVector(vec2(self.g0, 0.0), vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0), vec4(0.0), vec4(0.0), vec3(0.0), vec3(0.0), vec3(other.g0.x, other.g0.y, other.g0.z), vec2(0.0, other.g0.w));
}

MultiVector scalar__add__round_point(Scalar self, RoundPoint other) {
    return MultiVector(vec2(self.g0, 0.0), other.g0, other.g1, vec3(0.0), vec3(0.0), vec4(0.0), vec4(0.0), vec3(0.0), vec3(0.0), vec3(0.0), vec2(0.0));
}

Scalar scalar__add__scalar(Scalar self, Scalar other) {
    return Scalar(self.g0 + other.g0);
}

MultiVector scalar__add__sphere(Scalar self, Sphere other) {
    return MultiVector(vec2(self.g0, 0.0), vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0), vec4(0.0), vec4(0.0), vec3(0.0), vec3(0.0), other.g0, other.g1);
}

MultiVector sphere__add__anti_scalar(Sphere self, AntiScalar other) {
    return MultiVector(vec2(0.0, other.g0), vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0), vec4(0.0), vec4(0.0), vec3(0.0), vec3(0.0), self.g0, self.g1);
}

MultiVector sphere__add__circle(Sphere self, Circle other) {
    return MultiVector(vec2(0.0), vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0), vec4(0.0), other.g0, other.g1, other.g2, self.g0, self.g1);
}

MultiVector sphere__add__dipole(Sphere self, Dipole other) {
    return MultiVector(vec2(0.0), vec3(0.0), vec2(0.0), other.g0, other.g1, other.g2, vec4(0.0), vec3(0.0), vec3(0.0), self.g0, self.g1);
}

MultiVector sphere__add__dual_num(Sphere self, DualNum other) {
    return MultiVector(other.g0, vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0), vec4(0.0), vec4(0.0), vec3(0.0), vec3(0.0), self.g0, self.g1);
}

MultiVector sphere__add__flat_point(Sphere self, FlatPoint other) {
    return MultiVector(vec2(0.0), vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0), other.g0, vec4(0.0), vec3(0.0), vec3(0.0), self.g0, self.g1);
}

MultiVector sphere__add__flector(Sphere self, Flector other) {
    return MultiVector(vec2(0.0), vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0), other.g0, vec4(0.0), vec3(0.0), vec3(0.0), self.g0 + vec3(other.g1.x, other.g1.y, other.g1.z), self.g1 + vec2(0.0, other.g1.w));
}

MultiVector sphere__add__line(Sphere self, Line other) {
    return MultiVector(vec2(0.0), vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0), vec4(0.0), vec4(0.0), other.g0, other.g1, self.g0, self.g1);
}

MultiVector sphere__add__motor(Sphere self, Motor other) {
    return MultiVector(vec2(0.0, other.g0.w), vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0), vec4(0.0), vec4(0.0), vec3(other.g0.x, other.g0.y, other.g0.z), other.g1, self.g0, self.g1);
}

MultiVector sphere__add__multi_vector(Sphere self, MultiVector other) {
    return MultiVector(other.g0, other.g1, other.g2, other.g3, other.g4, other.g5, other.g6, other.g7, other.g8, self.g0 + other.g9, self.g1 + other.g10);
}

Sphere sphere__add__plane(Sphere self, Plane other) {
    return Sphere(self.g0 + vec3(other.g0.x, other.g0.y, other.g0.z), self.g1 + vec2(0.0, other.g0.w));
}

MultiVector sphere__add__round_point(Sphere self, RoundPoint other) {
    return MultiVector(vec2(0.0), other.g0, other.g1, vec3(0.0), vec3(0.0), vec4(0.0), vec4(0.0), vec3(0.0), vec3(0.0), self.g0, self.g1);
}

MultiVector sphere__add__scalar(Sphere self, Scalar other) {
    return MultiVector(vec2(other.g0, 0.0), vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0), vec4(0.0), vec4(0.0), vec3(0.0), vec3(0.0), self.g0, self.g1);
}

Sphere sphere__add__sphere(Sphere self, Sphere other) {
    return Sphere(self.g0 + other.g0, self.g1 + other.g1);
}

AntiScalar anti_scalar__div__anti_scalar(AntiScalar self, AntiScalar other) {
    return AntiScalar(self.g0 / other.g0);
}

Circle circle__div__circle(Circle self, Circle other) {
    return Circle(self.g0 / other.g0, self.g1 / other.g1, self.g2 / other.g2);
}

Dipole dipole__div__dipole(Dipole self, Dipole other) {
    return Dipole(self.g0 / other.g0, self.g1 / other.g1, self.g2 / other.g2);
}

DualNum dual_num__div__dual_num(DualNum self, DualNum other) {
    return DualNum(self.g0 / other.g0);
}

FlatPoint flat_point__div__flat_point(FlatPoint self, FlatPoint other) {
    return FlatPoint(self.g0 / other.g0);
}

Flector flector__div__flector(Flector self, Flector other) {
    return Flector(self.g0 / other.g0, self.g1 / other.g1);
}

Line line__div__line(Line self, Line other) {
    return Line(self.g0 / other.g0, self.g1 / other.g1);
}

Motor motor__div__motor(Motor self, Motor other) {
    return Motor(self.g0 / other.g0, self.g1 / other.g1);
}

MultiVector multi_vector__div__multi_vector(MultiVector self, MultiVector other) {
    return MultiVector(self.g0 / other.g0, self.g1 / other.g1, self.g2 / other.g2, self.g3 / other.g3, self.g4 / other.g4, self.g5 / other.g5, self.g6 / other.g6, self.g7 / other.g7, self.g8 / other.g8, self.g9 / other.g9, self.g10 / other.g10);
}

Plane plane__div__plane(Plane self, Plane other) {
    return Plane(self.g0 / other.g0);
}

RoundPoint round_point__div__round_point(RoundPoint self, RoundPoint other) {
    return RoundPoint(self.g0 / other.g0, self.g1 / other.g1);
}

Scalar scalar__div__scalar(Scalar self, Scalar other) {
    return Scalar(self.g0 / other.g0);
}

Sphere sphere__div__sphere(Sphere self, Sphere other) {
    return Sphere(self.g0 / other.g0, self.g1 / other.g1);
}

Line circle__into__line(Circle self) {
    return Line(self.g1, self.g2);
}

FlatPoint dipole__into__flat_point(Dipole self) {
    return FlatPoint(self.g2);
}

AntiScalar dual_num__into__anti_scalar(DualNum self) {
    return AntiScalar(self.g0.y);
}

Scalar dual_num__into__scalar(DualNum self) {
    return Scalar(self.g0.x);
}

FlatPoint flector__into__flat_point(Flector self) {
    return FlatPoint(self.g0);
}

Plane flector__into__plane(Flector self) {
    return Plane(self.g1);
}

AntiScalar motor__into__anti_scalar(Motor self) {
    return AntiScalar(self.g0.w);
}

Line motor__into__line(Motor self) {
    return Line(vec3(self.g0.x, self.g0.y, self.g0.z), self.g1);
}

AntiScalar multi_vector__into__anti_scalar(MultiVector self) {
    return AntiScalar(self.g0.y);
}

Circle multi_vector__into__circle(MultiVector self) {
    return Circle(self.g6, self.g7, self.g8);
}

Dipole multi_vector__into__dipole(MultiVector self) {
    return Dipole(self.g3, self.g4, self.g5);
}

DualNum multi_vector__into__dual_num(MultiVector self) {
    return DualNum(self.g0);
}

FlatPoint multi_vector__into__flat_point(MultiVector self) {
    return FlatPoint(self.g5);
}

Flector multi_vector__into__flector(MultiVector self) {
    return Flector(self.g5, vec4(self.g9.x, self.g9.y, self.g9.z, self.g10.y));
}

Line multi_vector__into__line(MultiVector self) {
    return Line(self.g7, self.g8);
}

Motor multi_vector__into__motor(MultiVector self) {
    return Motor(vec4(self.g7.x, self.g7.y, self.g7.z, self.g0.y), self.g8);
}

Plane multi_vector__into__plane(MultiVector self) {
    return Plane(vec4(self.g9.x, self.g9.y, self.g9.z, self.g10.y));
}

RoundPoint multi_vector__into__round_point(MultiVector self) {
    return RoundPoint(self.g1, self.g2);
}

Scalar multi_vector__into__scalar(MultiVector self) {
    return Scalar(self.g0.x);
}

Sphere multi_vector__into__sphere(MultiVector self) {
    return Sphere(self.g9, self.g10);
}

Plane sphere__into__plane(Sphere self) {
    return Plane(vec4(self.g0.x, self.g0.y, self.g0.z, self.g1.y));
}

AntiScalar anti_scalar__mul__anti_scalar(AntiScalar self, AntiScalar other) {
    return AntiScalar(self.g0 * other.g0);
}

Circle circle__mul__circle(Circle self, Circle other) {
    return Circle(self.g0 * other.g0, self.g1 * other.g1, self.g2 * other.g2);
}

Dipole dipole__mul__dipole(Dipole self, Dipole other) {
    return Dipole(self.g0 * other.g0, self.g1 * other.g1, self.g2 * other.g2);
}

DualNum dual_num__mul__dual_num(DualNum self, DualNum other) {
    return DualNum(self.g0 * other.g0);
}

FlatPoint flat_point__mul__flat_point(FlatPoint self, FlatPoint other) {
    return FlatPoint(self.g0 * other.g0);
}

Flector flector__mul__flector(Flector self, Flector other) {
    return Flector(self.g0 * other.g0, self.g1 * other.g1);
}

Line line__mul__line(Line self, Line other) {
    return Line(self.g0 * other.g0, self.g1 * other.g1);
}

Motor motor__mul__motor(Motor self, Motor other) {
    return Motor(self.g0 * other.g0, self.g1 * other.g1);
}

MultiVector multi_vector__mul__multi_vector(MultiVector self, MultiVector other) {
    return MultiVector(self.g0 * other.g0, self.g1 * other.g1, self.g2 * other.g2, self.g3 * other.g3, self.g4 * other.g4, self.g5 * other.g5, self.g6 * other.g6, self.g7 * other.g7, self.g8 * other.g8, self.g9 * other.g9, self.g10 * other.g10);
}

Plane plane__mul__plane(Plane self, Plane other) {
    return Plane(self.g0 * other.g0);
}

RoundPoint round_point__mul__round_point(RoundPoint self, RoundPoint other) {
    return RoundPoint(self.g0 * other.g0, self.g1 * other.g1);
}

Scalar scalar__mul__scalar(Scalar self, Scalar other) {
    return Scalar(self.g0 * other.g0);
}

Sphere sphere__mul__sphere(Sphere self, Sphere other) {
    return Sphere(self.g0 * other.g0, self.g1 * other.g1);
}

AntiScalar anti_scalar__sub__anti_scalar(AntiScalar self, AntiScalar other) {
    return AntiScalar(self.g0 - other.g0);
}

MultiVector anti_scalar__sub__circle(AntiScalar self, Circle other) {
    return MultiVector(vec2(0.0, self.g0), vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0), vec4(0.0), vec4(0.0) - other.g0, vec3(0.0) - other.g1, vec3(0.0) - other.g2, vec3(0.0), vec2(0.0));
}

MultiVector anti_scalar__sub__dipole(AntiScalar self, Dipole other) {
    return MultiVector(vec2(0.0, self.g0), vec3(0.0), vec2(0.0), vec3(0.0) - other.g0, vec3(0.0) - other.g1, vec4(0.0) - other.g2, vec4(0.0), vec3(0.0), vec3(0.0), vec3(0.0), vec2(0.0));
}

DualNum anti_scalar__sub__dual_num(AntiScalar self, DualNum other) {
    return DualNum(vec2(0.0, self.g0) - other.g0);
}

MultiVector anti_scalar__sub__flat_point(AntiScalar self, FlatPoint other) {
    return MultiVector(vec2(0.0, self.g0), vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0), vec4(0.0) - other.g0, vec4(0.0), vec3(0.0), vec3(0.0), vec3(0.0), vec2(0.0));
}

MultiVector anti_scalar__sub__flector(AntiScalar self, Flector other) {
    return MultiVector(vec2(0.0, self.g0), vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0), vec4(0.0) - other.g0, vec4(0.0), vec3(0.0), vec3(0.0), vec3(0.0) - vec3(other.g1.x, other.g1.y, other.g1.z), vec2(0.0) - vec2(0.0, other.g1.w));
}

Motor anti_scalar__sub__line(AntiScalar self, Line other) {
    return Motor(vec4(0.0, 0.0, 0.0, self.g0) - vec4(other.g0.x, other.g0.y, other.g0.z, 0.0), vec3(0.0) - other.g1);
}

Motor anti_scalar__sub__motor(AntiScalar self, Motor other) {
    return Motor(vec4(0.0, 0.0, 0.0, self.g0) - other.g0, vec3(0.0) - other.g1);
}

MultiVector anti_scalar__sub__multi_vector(AntiScalar self, MultiVector other) {
    return MultiVector(vec2(0.0, self.g0) - other.g0, vec3(0.0) - other.g1, vec2(0.0) - other.g2, vec3(0.0) - other.g3, vec3(0.0) - other.g4, vec4(0.0) - other.g5, vec4(0.0) - other.g6, vec3(0.0) - other.g7, vec3(0.0) - other.g8, vec3(0.0) - other.g9, vec2(0.0) - other.g10);
}

MultiVector anti_scalar__sub__plane(AntiScalar self, Plane other) {
    return MultiVector(vec2(0.0, self.g0), vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0), vec4(0.0), vec4(0.0), vec3(0.0), vec3(0.0), vec3(0.0) - vec3(other.g0.x, other.g0.y, other.g0.z), vec2(0.0) - vec2(0.0, other.g0.w));
}

MultiVector anti_scalar__sub__round_point(AntiScalar self, RoundPoint other) {
    return MultiVector(vec2(0.0, self.g0), vec3(0.0) - other.g0, vec2(0.0) - other.g1, vec3(0.0), vec3(0.0), vec4(0.0), vec4(0.0), vec3(0.0), vec3(0.0), vec3(0.0), vec2(0.0));
}

DualNum anti_scalar__sub__scalar(AntiScalar self, Scalar other) {
    return DualNum(vec2(0.0, self.g0) - vec2(other.g0, 0.0));
}

MultiVector anti_scalar__sub__sphere(AntiScalar self, Sphere other) {
    return MultiVector(vec2(0.0, self.g0), vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0), vec4(0.0), vec4(0.0), vec3(0.0), vec3(0.0), vec3(0.0) - other.g0, vec2(0.0) - other.g1);
}

MultiVector circle__sub__anti_scalar(Circle self, AntiScalar other) {
    return MultiVector(vec2(0.0) - vec2(0.0, other.g0), vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0), vec4(0.0), self.g0, self.g1, self.g2, vec3(0.0), vec2(0.0));
}

Circle circle__sub__circle(Circle self, Circle other) {
    return Circle(self.g0 - other.g0, self.g1 - other.g1, self.g2 - other.g2);
}

MultiVector circle__sub__dipole(Circle self, Dipole other) {
    return MultiVector(vec2(0.0), vec3(0.0), vec2(0.0), vec3(0.0) - other.g0, vec3(0.0) - other.g1, vec4(0.0) - other.g2, self.g0, self.g1, self.g2, vec3(0.0), vec2(0.0));
}

MultiVector circle__sub__dual_num(Circle self, DualNum other) {
    return MultiVector(vec2(0.0) - other.g0, vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0), vec4(0.0), self.g0, self.g1, self.g2, vec3(0.0), vec2(0.0));
}

MultiVector circle__sub__flat_point(Circle self, FlatPoint other) {
    return MultiVector(vec2(0.0), vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0), vec4(0.0) - other.g0, self.g0, self.g1, self.g2, vec3(0.0), vec2(0.0));
}

MultiVector circle__sub__flector(Circle self, Flector other) {
    return MultiVector(vec2(0.0), vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0), vec4(0.0) - other.g0, self.g0, self.g1, self.g2, vec3(0.0) - vec3(other.g1.x, other.g1.y, other.g1.z), vec2(0.0) - vec2(0.0, other.g1.w));
}

Circle circle__sub__line(Circle self, Line other) {
    return Circle(self.g0, self.g1 - other.g0, self.g2 - other.g1);
}

MultiVector circle__sub__motor(Circle self, Motor other) {
    return MultiVector(vec2(0.0) - vec2(0.0, other.g0.w), vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0), vec4(0.0), self.g0, self.g1 - vec3(other.g0.x, other.g0.y, other.g0.z), self.g2 - other.g1, vec3(0.0), vec2(0.0));
}

MultiVector circle__sub__multi_vector(Circle self, MultiVector other) {
    return MultiVector(vec2(0.0) - other.g0, vec3(0.0) - other.g1, vec2(0.0) - other.g2, vec3(0.0) - other.g3, vec3(0.0) - other.g4, vec4(0.0) - other.g5, self.g0 - other.g6, self.g1 - other.g7, self.g2 - other.g8, vec3(0.0) - other.g9, vec2(0.0) - other.g10);
}

MultiVector circle__sub__plane(Circle self, Plane other) {
    return MultiVector(vec2(0.0), vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0), vec4(0.0), self.g0, self.g1, self.g2, vec3(0.0) - vec3(other.g0.x, other.g0.y, other.g0.z), vec2(0.0) - vec2(0.0, other.g0.w));
}

MultiVector circle__sub__round_point(Circle self, RoundPoint other) {
    return MultiVector(vec2(0.0), vec3(0.0) - other.g0, vec2(0.0) - other.g1, vec3(0.0), vec3(0.0), vec4(0.0), self.g0, self.g1, self.g2, vec3(0.0), vec2(0.0));
}

MultiVector circle__sub__scalar(Circle self, Scalar other) {
    return MultiVector(vec2(0.0) - vec2(other.g0, 0.0), vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0), vec4(0.0), self.g0, self.g1, self.g2, vec3(0.0), vec2(0.0));
}

MultiVector circle__sub__sphere(Circle self, Sphere other) {
    return MultiVector(vec2(0.0), vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0), vec4(0.0), self.g0, self.g1, self.g2, vec3(0.0) - other.g0, vec2(0.0) - other.g1);
}

MultiVector dipole__sub__anti_scalar(Dipole self, AntiScalar other) {
    return MultiVector(vec2(0.0) - vec2(0.0, other.g0), vec3(0.0), vec2(0.0), self.g0, self.g1, self.g2, vec4(0.0), vec3(0.0), vec3(0.0), vec3(0.0), vec2(0.0));
}

MultiVector dipole__sub__circle(Dipole self, Circle other) {
    return MultiVector(vec2(0.0), vec3(0.0), vec2(0.0), self.g0, self.g1, self.g2, vec4(0.0) - other.g0, vec3(0.0) - other.g1, vec3(0.0) - other.g2, vec3(0.0), vec2(0.0));
}

Dipole dipole__sub__dipole(Dipole self, Dipole other) {
    return Dipole(self.g0 - other.g0, self.g1 - other.g1, self.g2 - other.g2);
}

MultiVector dipole__sub__dual_num(Dipole self, DualNum other) {
    return MultiVector(vec2(0.0) - other.g0, vec3(0.0), vec2(0.0), self.g0, self.g1, self.g2, vec4(0.0), vec3(0.0), vec3(0.0), vec3(0.0), vec2(0.0));
}

Dipole dipole__sub__flat_point(Dipole self, FlatPoint other) {
    return Dipole(self.g0, self.g1, self.g2 - other.g0);
}

MultiVector dipole__sub__flector(Dipole self, Flector other) {
    return MultiVector(vec2(0.0), vec3(0.0), vec2(0.0), self.g0, self.g1, self.g2 - other.g0, vec4(0.0), vec3(0.0), vec3(0.0), vec3(0.0) - vec3(other.g1.x, other.g1.y, other.g1.z), vec2(0.0) - vec2(0.0, other.g1.w));
}

MultiVector dipole__sub__line(Dipole self, Line other) {
    return MultiVector(vec2(0.0), vec3(0.0), vec2(0.0), self.g0, self.g1, self.g2, vec4(0.0), vec3(0.0) - other.g0, vec3(0.0) - other.g1, vec3(0.0), vec2(0.0));
}

MultiVector dipole__sub__motor(Dipole self, Motor other) {
    return MultiVector(vec2(0.0) - vec2(0.0, other.g0.w), vec3(0.0), vec2(0.0), self.g0, self.g1, self.g2, vec4(0.0), vec3(0.0) - vec3(other.g0.x, other.g0.y, other.g0.z), vec3(0.0) - other.g1, vec3(0.0), vec2(0.0));
}

MultiVector dipole__sub__multi_vector(Dipole self, MultiVector other) {
    return MultiVector(vec2(0.0) - other.g0, vec3(0.0) - other.g1, vec2(0.0) - other.g2, self.g0 - other.g3, self.g1 - other.g4, self.g2 - other.g5, vec4(0.0) - other.g6, vec3(0.0) - other.g7, vec3(0.0) - other.g8, vec3(0.0) - other.g9, vec2(0.0) - other.g10);
}

MultiVector dipole__sub__plane(Dipole self, Plane other) {
    return MultiVector(vec2(0.0), vec3(0.0), vec2(0.0), self.g0, self.g1, self.g2, vec4(0.0), vec3(0.0), vec3(0.0), vec3(0.0) - vec3(other.g0.x, other.g0.y, other.g0.z), vec2(0.0) - vec2(0.0, other.g0.w));
}

MultiVector dipole__sub__round_point(Dipole self, RoundPoint other) {
    return MultiVector(vec2(0.0), vec3(0.0) - other.g0, vec2(0.0) - other.g1, self.g0, self.g1, self.g2, vec4(0.0), vec3(0.0), vec3(0.0), vec3(0.0), vec2(0.0));
}

MultiVector dipole__sub__scalar(Dipole self, Scalar other) {
    return MultiVector(vec2(0.0) - vec2(other.g0, 0.0), vec3(0.0), vec2(0.0), self.g0, self.g1, self.g2, vec4(0.0), vec3(0.0), vec3(0.0), vec3(0.0), vec2(0.0));
}

MultiVector dipole__sub__sphere(Dipole self, Sphere other) {
    return MultiVector(vec2(0.0), vec3(0.0), vec2(0.0), self.g0, self.g1, self.g2, vec4(0.0), vec3(0.0), vec3(0.0), vec3(0.0) - other.g0, vec2(0.0) - other.g1);
}

DualNum dual_num__sub__anti_scalar(DualNum self, AntiScalar other) {
    return DualNum(self.g0 - vec2(0.0, other.g0));
}

MultiVector dual_num__sub__circle(DualNum self, Circle other) {
    return MultiVector(self.g0, vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0), vec4(0.0), vec4(0.0) - other.g0, vec3(0.0) - other.g1, vec3(0.0) - other.g2, vec3(0.0), vec2(0.0));
}

MultiVector dual_num__sub__dipole(DualNum self, Dipole other) {
    return MultiVector(self.g0, vec3(0.0), vec2(0.0), vec3(0.0) - other.g0, vec3(0.0) - other.g1, vec4(0.0) - other.g2, vec4(0.0), vec3(0.0), vec3(0.0), vec3(0.0), vec2(0.0));
}

DualNum dual_num__sub__dual_num(DualNum self, DualNum other) {
    return DualNum(self.g0 - other.g0);
}

MultiVector dual_num__sub__flat_point(DualNum self, FlatPoint other) {
    return MultiVector(self.g0, vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0), vec4(0.0) - other.g0, vec4(0.0), vec3(0.0), vec3(0.0), vec3(0.0), vec2(0.0));
}

MultiVector dual_num__sub__flector(DualNum self, Flector other) {
    return MultiVector(self.g0, vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0), vec4(0.0) - other.g0, vec4(0.0), vec3(0.0), vec3(0.0), vec3(0.0) - vec3(other.g1.x, other.g1.y, other.g1.z), vec2(0.0) - vec2(0.0, other.g1.w));
}

MultiVector dual_num__sub__line(DualNum self, Line other) {
    return MultiVector(self.g0, vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0), vec4(0.0), vec4(0.0), vec3(0.0) - other.g0, vec3(0.0) - other.g1, vec3(0.0), vec2(0.0));
}

MultiVector dual_num__sub__motor(DualNum self, Motor other) {
    return MultiVector(self.g0 - vec2(0.0, other.g0.w), vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0), vec4(0.0), vec4(0.0), vec3(0.0) - vec3(other.g0.x, other.g0.y, other.g0.z), vec3(0.0) - other.g1, vec3(0.0), vec2(0.0));
}

MultiVector dual_num__sub__multi_vector(DualNum self, MultiVector other) {
    return MultiVector(self.g0 - other.g0, vec3(0.0) - other.g1, vec2(0.0) - other.g2, vec3(0.0) - other.g3, vec3(0.0) - other.g4, vec4(0.0) - other.g5, vec4(0.0) - other.g6, vec3(0.0) - other.g7, vec3(0.0) - other.g8, vec3(0.0) - other.g9, vec2(0.0) - other.g10);
}

MultiVector dual_num__sub__plane(DualNum self, Plane other) {
    return MultiVector(self.g0, vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0), vec4(0.0), vec4(0.0), vec3(0.0), vec3(0.0), vec3(0.0) - vec3(other.g0.x, other.g0.y, other.g0.z), vec2(0.0) - vec2(0.0, other.g0.w));
}

MultiVector dual_num__sub__round_point(DualNum self, RoundPoint other) {
    return MultiVector(self.g0, vec3(0.0) - other.g0, vec2(0.0) - other.g1, vec3(0.0), vec3(0.0), vec4(0.0), vec4(0.0), vec3(0.0), vec3(0.0), vec3(0.0), vec2(0.0));
}

DualNum dual_num__sub__scalar(DualNum self, Scalar other) {
    return DualNum(self.g0 - vec2(other.g0, 0.0));
}

MultiVector dual_num__sub__sphere(DualNum self, Sphere other) {
    return MultiVector(self.g0, vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0), vec4(0.0), vec4(0.0), vec3(0.0), vec3(0.0), vec3(0.0) - other.g0, vec2(0.0) - other.g1);
}

MultiVector flat_point__sub__anti_scalar(FlatPoint self, AntiScalar other) {
    return MultiVector(vec2(0.0) - vec2(0.0, other.g0), vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0), self.g0, vec4(0.0), vec3(0.0), vec3(0.0), vec3(0.0), vec2(0.0));
}

MultiVector flat_point__sub__circle(FlatPoint self, Circle other) {
    return MultiVector(vec2(0.0), vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0), self.g0, vec4(0.0) - other.g0, vec3(0.0) - other.g1, vec3(0.0) - other.g2, vec3(0.0), vec2(0.0));
}

Dipole flat_point__sub__dipole(FlatPoint self, Dipole other) {
    return Dipole(vec3(0.0) - other.g0, vec3(0.0) - other.g1, self.g0 - other.g2);
}

MultiVector flat_point__sub__dual_num(FlatPoint self, DualNum other) {
    return MultiVector(vec2(0.0) - other.g0, vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0), self.g0, vec4(0.0), vec3(0.0), vec3(0.0), vec3(0.0), vec2(0.0));
}

FlatPoint flat_point__sub__flat_point(FlatPoint self, FlatPoint other) {
    return FlatPoint(self.g0 - other.g0);
}

Flector flat_point__sub__flector(FlatPoint self, Flector other) {
    return Flector(self.g0 - other.g0, vec4(0.0) - other.g1);
}

MultiVector flat_point__sub__line(FlatPoint self, Line other) {
    return MultiVector(vec2(0.0), vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0), self.g0, vec4(0.0), vec3(0.0) - other.g0, vec3(0.0) - other.g1, vec3(0.0), vec2(0.0));
}

MultiVector flat_point__sub__motor(FlatPoint self, Motor other) {
    return MultiVector(vec2(0.0) - vec2(0.0, other.g0.w), vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0), self.g0, vec4(0.0), vec3(0.0) - vec3(other.g0.x, other.g0.y, other.g0.z), vec3(0.0) - other.g1, vec3(0.0), vec2(0.0));
}

MultiVector flat_point__sub__multi_vector(FlatPoint self, MultiVector other) {
    return MultiVector(vec2(0.0) - other.g0, vec3(0.0) - other.g1, vec2(0.0) - other.g2, vec3(0.0) - other.g3, vec3(0.0) - other.g4, self.g0 - other.g5, vec4(0.0) - other.g6, vec3(0.0) - other.g7, vec3(0.0) - other.g8, vec3(0.0) - other.g9, vec2(0.0) - other.g10);
}

Flector flat_point__sub__plane(FlatPoint self, Plane other) {
    return Flector(self.g0, vec4(0.0) - other.g0);
}

MultiVector flat_point__sub__round_point(FlatPoint self, RoundPoint other) {
    return MultiVector(vec2(0.0), vec3(0.0) - other.g0, vec2(0.0) - other.g1, vec3(0.0), vec3(0.0), self.g0, vec4(0.0), vec3(0.0), vec3(0.0), vec3(0.0), vec2(0.0));
}

MultiVector flat_point__sub__scalar(FlatPoint self, Scalar other) {
    return MultiVector(vec2(0.0) - vec2(other.g0, 0.0), vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0), self.g0, vec4(0.0), vec3(0.0), vec3(0.0), vec3(0.0), vec2(0.0));
}

MultiVector flat_point__sub__sphere(FlatPoint self, Sphere other) {
    return MultiVector(vec2(0.0), vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0), self.g0, vec4(0.0), vec3(0.0), vec3(0.0), vec3(0.0) - other.g0, vec2(0.0) - other.g1);
}

MultiVector flector__sub__anti_scalar(Flector self, AntiScalar other) {
    return MultiVector(vec2(0.0) - vec2(0.0, other.g0), vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0), self.g0, vec4(0.0), vec3(0.0), vec3(0.0), vec3(self.g1.x, self.g1.y, self.g1.z), vec2(0.0, self.g1.w));
}

MultiVector flector__sub__circle(Flector self, Circle other) {
    return MultiVector(vec2(0.0), vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0), self.g0, vec4(0.0) - other.g0, vec3(0.0) - other.g1, vec3(0.0) - other.g2, vec3(self.g1.x, self.g1.y, self.g1.z), vec2(0.0, self.g1.w));
}

MultiVector flector__sub__dipole(Flector self, Dipole other) {
    return MultiVector(vec2(0.0), vec3(0.0), vec2(0.0), vec3(0.0) - other.g0, vec3(0.0) - other.g1, self.g0 - other.g2, vec4(0.0), vec3(0.0), vec3(0.0), vec3(self.g1.x, self.g1.y, self.g1.z), vec2(0.0, self.g1.w));
}

MultiVector flector__sub__dual_num(Flector self, DualNum other) {
    return MultiVector(vec2(0.0) - other.g0, vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0), self.g0, vec4(0.0), vec3(0.0), vec3(0.0), vec3(self.g1.x, self.g1.y, self.g1.z), vec2(0.0, self.g1.w));
}

Flector flector__sub__flat_point(Flector self, FlatPoint other) {
    return Flector(self.g0 - other.g0, self.g1);
}

Flector flector__sub__flector(Flector self, Flector other) {
    return Flector(self.g0 - other.g0, self.g1 - other.g1);
}

MultiVector flector__sub__line(Flector self, Line other) {
    return MultiVector(vec2(0.0), vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0), self.g0, vec4(0.0), vec3(0.0) - other.g0, vec3(0.0) - other.g1, vec3(self.g1.x, self.g1.y, self.g1.z), vec2(0.0, self.g1.w));
}

MultiVector flector__sub__motor(Flector self, Motor other) {
    return MultiVector(vec2(0.0) - vec2(0.0, other.g0.w), vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0), self.g0, vec4(0.0), vec3(0.0) - vec3(other.g0.x, other.g0.y, other.g0.z), vec3(0.0) - other.g1, vec3(self.g1.x, self.g1.y, self.g1.z), vec2(0.0, self.g1.w));
}

MultiVector flector__sub__multi_vector(Flector self, MultiVector other) {
    return MultiVector(vec2(0.0) - other.g0, vec3(0.0) - other.g1, vec2(0.0) - other.g2, vec3(0.0) - other.g3, vec3(0.0) - other.g4, self.g0 - other.g5, vec4(0.0) - other.g6, vec3(0.0) - other.g7, vec3(0.0) - other.g8, vec3(self.g1.x, self.g1.y, self.g1.z) - other.g9, vec2(0.0, self.g1.w) - other.g10);
}

Flector flector__sub__plane(Flector self, Plane other) {
    return Flector(self.g0, self.g1 - other.g0);
}

MultiVector flector__sub__round_point(Flector self, RoundPoint other) {
    return MultiVector(vec2(0.0), vec3(0.0) - other.g0, vec2(0.0) - other.g1, vec3(0.0), vec3(0.0), self.g0, vec4(0.0), vec3(0.0), vec3(0.0), vec3(self.g1.x, self.g1.y, self.g1.z), vec2(0.0, self.g1.w));
}

MultiVector flector__sub__scalar(Flector self, Scalar other) {
    return MultiVector(vec2(0.0) - vec2(other.g0, 0.0), vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0), self.g0, vec4(0.0), vec3(0.0), vec3(0.0), vec3(self.g1.x, self.g1.y, self.g1.z), vec2(0.0, self.g1.w));
}

MultiVector flector__sub__sphere(Flector self, Sphere other) {
    return MultiVector(vec2(0.0), vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0), self.g0, vec4(0.0), vec3(0.0), vec3(0.0), vec3(self.g1.x, self.g1.y, self.g1.z) - other.g0, vec2(0.0, self.g1.w) - other.g1);
}

Motor line__sub__anti_scalar(Line self, AntiScalar other) {
    return Motor(vec4(self.g0.x, self.g0.y, self.g0.z, 0.0) - vec4(0.0, 0.0, 0.0, other.g0), self.g1);
}

Circle line__sub__circle(Line self, Circle other) {
    return Circle(vec4(0.0) - other.g0, self.g0 - other.g1, self.g1 - other.g2);
}

MultiVector line__sub__dipole(Line self, Dipole other) {
    return MultiVector(vec2(0.0), vec3(0.0), vec2(0.0), vec3(0.0) - other.g0, vec3(0.0) - other.g1, vec4(0.0) - other.g2, vec4(0.0), self.g0, self.g1, vec3(0.0), vec2(0.0));
}

MultiVector line__sub__dual_num(Line self, DualNum other) {
    return MultiVector(vec2(0.0) - other.g0, vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0), vec4(0.0), vec4(0.0), self.g0, self.g1, vec3(0.0), vec2(0.0));
}

MultiVector line__sub__flat_point(Line self, FlatPoint other) {
    return MultiVector(vec2(0.0), vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0), vec4(0.0) - other.g0, vec4(0.0), self.g0, self.g1, vec3(0.0), vec2(0.0));
}

MultiVector line__sub__flector(Line self, Flector other) {
    return MultiVector(vec2(0.0), vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0), vec4(0.0) - other.g0, vec4(0.0), self.g0, self.g1, vec3(0.0) - vec3(other.g1.x, other.g1.y, other.g1.z), vec2(0.0) - vec2(0.0, other.g1.w));
}

Line line__sub__line(Line self, Line other) {
    return Line(self.g0 - other.g0, self.g1 - other.g1);
}

Motor line__sub__motor(Line self, Motor other) {
    return Motor(vec4(self.g0.x, self.g0.y, self.g0.z, 0.0) - other.g0, self.g1 - other.g1);
}

MultiVector line__sub__multi_vector(Line self, MultiVector other) {
    return MultiVector(vec2(0.0) - other.g0, vec3(0.0) - other.g1, vec2(0.0) - other.g2, vec3(0.0) - other.g3, vec3(0.0) - other.g4, vec4(0.0) - other.g5, vec4(0.0) - other.g6, self.g0 - other.g7, self.g1 - other.g8, vec3(0.0) - other.g9, vec2(0.0) - other.g10);
}

MultiVector line__sub__plane(Line self, Plane other) {
    return MultiVector(vec2(0.0), vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0), vec4(0.0), vec4(0.0), self.g0, self.g1, vec3(0.0) - vec3(other.g0.x, other.g0.y, other.g0.z), vec2(0.0) - vec2(0.0, other.g0.w));
}

MultiVector line__sub__round_point(Line self, RoundPoint other) {
    return MultiVector(vec2(0.0), vec3(0.0) - other.g0, vec2(0.0) - other.g1, vec3(0.0), vec3(0.0), vec4(0.0), vec4(0.0), self.g0, self.g1, vec3(0.0), vec2(0.0));
}

MultiVector line__sub__scalar(Line self, Scalar other) {
    return MultiVector(vec2(0.0) - vec2(other.g0, 0.0), vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0), vec4(0.0), vec4(0.0), self.g0, self.g1, vec3(0.0), vec2(0.0));
}

MultiVector line__sub__sphere(Line self, Sphere other) {
    return MultiVector(vec2(0.0), vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0), vec4(0.0), vec4(0.0), self.g0, self.g1, vec3(0.0) - other.g0, vec2(0.0) - other.g1);
}

Motor motor__sub__anti_scalar(Motor self, AntiScalar other) {
    return Motor(self.g0 - vec4(0.0, 0.0, 0.0, other.g0), self.g1);
}

MultiVector motor__sub__circle(Motor self, Circle other) {
    return MultiVector(vec2(0.0, self.g0.w), vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0), vec4(0.0), vec4(0.0) - other.g0, vec3(self.g0.x, self.g0.y, self.g0.z) - other.g1, self.g1 - other.g2, vec3(0.0), vec2(0.0));
}

MultiVector motor__sub__dipole(Motor self, Dipole other) {
    return MultiVector(vec2(0.0, self.g0.w), vec3(0.0), vec2(0.0), vec3(0.0) - other.g0, vec3(0.0) - other.g1, vec4(0.0) - other.g2, vec4(0.0), vec3(self.g0.x, self.g0.y, self.g0.z), self.g1, vec3(0.0), vec2(0.0));
}

MultiVector motor__sub__dual_num(Motor self, DualNum other) {
    return MultiVector(vec2(0.0, self.g0.w) - other.g0, vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0), vec4(0.0), vec4(0.0), vec3(self.g0.x, self.g0.y, self.g0.z), self.g1, vec3(0.0), vec2(0.0));
}

MultiVector motor__sub__flat_point(Motor self, FlatPoint other) {
    return MultiVector(vec2(0.0, self.g0.w), vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0), vec4(0.0) - other.g0, vec4(0.0), vec3(self.g0.x, self.g0.y, self.g0.z), self.g1, vec3(0.0), vec2(0.0));
}

MultiVector motor__sub__flector(Motor self, Flector other) {
    return MultiVector(vec2(0.0, self.g0.w), vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0), vec4(0.0) - other.g0, vec4(0.0), vec3(self.g0.x, self.g0.y, self.g0.z), self.g1, vec3(0.0) - vec3(other.g1.x, other.g1.y, other.g1.z), vec2(0.0) - vec2(0.0, other.g1.w));
}

Motor motor__sub__line(Motor self, Line other) {
    return Motor(self.g0 - vec4(other.g0.x, other.g0.y, other.g0.z, 0.0), self.g1 - other.g1);
}

Motor motor__sub__motor(Motor self, Motor other) {
    return Motor(self.g0 - other.g0, self.g1 - other.g1);
}

MultiVector motor__sub__multi_vector(Motor self, MultiVector other) {
    return MultiVector(vec2(0.0, self.g0.w) - other.g0, vec3(0.0) - other.g1, vec2(0.0) - other.g2, vec3(0.0) - other.g3, vec3(0.0) - other.g4, vec4(0.0) - other.g5, vec4(0.0) - other.g6, vec3(self.g0.x, self.g0.y, self.g0.z) - other.g7, self.g1 - other.g8, vec3(0.0) - other.g9, vec2(0.0) - other.g10);
}

MultiVector motor__sub__plane(Motor self, Plane other) {
    return MultiVector(vec2(0.0, self.g0.w), vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0), vec4(0.0), vec4(0.0), vec3(self.g0.x, self.g0.y, self.g0.z), self.g1, vec3(0.0) - vec3(other.g0.x, other.g0.y, other.g0.z), vec2(0.0) - vec2(0.0, other.g0.w));
}

MultiVector motor__sub__round_point(Motor self, RoundPoint other) {
    return MultiVector(vec2(0.0, self.g0.w), vec3(0.0) - other.g0, vec2(0.0) - other.g1, vec3(0.0), vec3(0.0), vec4(0.0), vec4(0.0), vec3(self.g0.x, self.g0.y, self.g0.z), self.g1, vec3(0.0), vec2(0.0));
}

MultiVector motor__sub__scalar(Motor self, Scalar other) {
    return MultiVector(vec2(0.0, self.g0.w) - vec2(other.g0, 0.0), vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0), vec4(0.0), vec4(0.0), vec3(self.g0.x, self.g0.y, self.g0.z), self.g1, vec3(0.0), vec2(0.0));
}

MultiVector motor__sub__sphere(Motor self, Sphere other) {
    return MultiVector(vec2(0.0, self.g0.w), vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0), vec4(0.0), vec4(0.0), vec3(self.g0.x, self.g0.y, self.g0.z), self.g1, vec3(0.0) - other.g0, vec2(0.0) - other.g1);
}

MultiVector multi_vector__sub__anti_scalar(MultiVector self, AntiScalar other) {
    return MultiVector(self.g0 - vec2(0.0, other.g0), self.g1, self.g2, self.g3, self.g4, self.g5, self.g6, self.g7, self.g8, self.g9, self.g10);
}

MultiVector multi_vector__sub__circle(MultiVector self, Circle other) {
    return MultiVector(self.g0, self.g1, self.g2, self.g3, self.g4, self.g5, self.g6 - other.g0, self.g7 - other.g1, self.g8 - other.g2, self.g9, self.g10);
}

MultiVector multi_vector__sub__dipole(MultiVector self, Dipole other) {
    return MultiVector(self.g0, self.g1, self.g2, self.g3 - other.g0, self.g4 - other.g1, self.g5 - other.g2, self.g6, self.g7, self.g8, self.g9, self.g10);
}

MultiVector multi_vector__sub__dual_num(MultiVector self, DualNum other) {
    return MultiVector(self.g0 - other.g0, self.g1, self.g2, self.g3, self.g4, self.g5, self.g6, self.g7, self.g8, self.g9, self.g10);
}

MultiVector multi_vector__sub__flat_point(MultiVector self, FlatPoint other) {
    return MultiVector(self.g0, self.g1, self.g2, self.g3, self.g4, self.g5 - other.g0, self.g6, self.g7, self.g8, self.g9, self.g10);
}

MultiVector multi_vector__sub__flector(MultiVector self, Flector other) {
    return MultiVector(self.g0, self.g1, self.g2, self.g3, self.g4, self.g5 - other.g0, self.g6, self.g7, self.g8, self.g9 - vec3(other.g1.x, other.g1.y, other.g1.z), self.g10 - vec2(0.0, other.g1.w));
}

MultiVector multi_vector__sub__line(MultiVector self, Line other) {
    return MultiVector(self.g0, self.g1, self.g2, self.g3, self.g4, self.g5, self.g6, self.g7 - other.g0, self.g8 - other.g1, self.g9, self.g10);
}

MultiVector multi_vector__sub__motor(MultiVector self, Motor other) {
    return MultiVector(self.g0 - vec2(0.0, other.g0.w), self.g1, self.g2, self.g3, self.g4, self.g5, self.g6, self.g7 - vec3(other.g0.x, other.g0.y, other.g0.z), self.g8 - other.g1, self.g9, self.g10);
}

MultiVector multi_vector__sub__multi_vector(MultiVector self, MultiVector other) {
    return MultiVector(self.g0 - other.g0, self.g1 - other.g1, self.g2 - other.g2, self.g3 - other.g3, self.g4 - other.g4, self.g5 - other.g5, self.g6 - other.g6, self.g7 - other.g7, self.g8 - other.g8, self.g9 - other.g9, self.g10 - other.g10);
}

MultiVector multi_vector__sub__plane(MultiVector self, Plane other) {
    return MultiVector(self.g0, self.g1, self.g2, self.g3, self.g4, self.g5, self.g6, self.g7, self.g8, self.g9 - vec3(other.g0.x, other.g0.y, other.g0.z), self.g10 - vec2(0.0, other.g0.w));
}

MultiVector multi_vector__sub__round_point(MultiVector self, RoundPoint other) {
    return MultiVector(self.g0, self.g1 - other.g0, self.g2 - other.g1, self.g3, self.g4, self.g5, self.g6, self.g7, self.g8, self.g9, self.g10);
}

MultiVector multi_vector__sub__scalar(MultiVector self, Scalar other) {
    return MultiVector(self.g0 - vec2(other.g0, 0.0), self.g1, self.g2, self.g3, self.g4, self.g5, self.g6, self.g7, self.g8, self.g9, self.g10);
}

MultiVector multi_vector__sub__sphere(MultiVector self, Sphere other) {
    return MultiVector(self.g0, self.g1, self.g2, self.g3, self.g4, self.g5, self.g6, self.g7, self.g8, self.g9 - other.g0, self.g10 - other.g1);
}

MultiVector plane__sub__anti_scalar(Plane self, AntiScalar other) {
    return MultiVector(vec2(0.0) - vec2(0.0, other.g0), vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0), vec4(0.0), vec4(0.0), vec3(0.0), vec3(0.0), vec3(self.g0.x, self.g0.y, self.g0.z), vec2(0.0, self.g0.w));
}

MultiVector plane__sub__circle(Plane self, Circle other) {
    return MultiVector(vec2(0.0), vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0), vec4(0.0), vec4(0.0) - other.g0, vec3(0.0) - other.g1, vec3(0.0) - other.g2, vec3(self.g0.x, self.g0.y, self.g0.z), vec2(0.0, self.g0.w));
}

MultiVector plane__sub__dipole(Plane self, Dipole other) {
    return MultiVector(vec2(0.0), vec3(0.0), vec2(0.0), vec3(0.0) - other.g0, vec3(0.0) - other.g1, vec4(0.0) - other.g2, vec4(0.0), vec3(0.0), vec3(0.0), vec3(self.g0.x, self.g0.y, self.g0.z), vec2(0.0, self.g0.w));
}

MultiVector plane__sub__dual_num(Plane self, DualNum other) {
    return MultiVector(vec2(0.0) - other.g0, vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0), vec4(0.0), vec4(0.0), vec3(0.0), vec3(0.0), vec3(self.g0.x, self.g0.y, self.g0.z), vec2(0.0, self.g0.w));
}

Flector plane__sub__flat_point(Plane self, FlatPoint other) {
    return Flector(vec4(0.0) - other.g0, self.g0);
}

Flector plane__sub__flector(Plane self, Flector other) {
    return Flector(vec4(0.0) - other.g0, self.g0 - other.g1);
}

MultiVector plane__sub__line(Plane self, Line other) {
    return MultiVector(vec2(0.0), vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0), vec4(0.0), vec4(0.0), vec3(0.0) - other.g0, vec3(0.0) - other.g1, vec3(self.g0.x, self.g0.y, self.g0.z), vec2(0.0, self.g0.w));
}

MultiVector plane__sub__motor(Plane self, Motor other) {
    return MultiVector(vec2(0.0) - vec2(0.0, other.g0.w), vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0), vec4(0.0), vec4(0.0), vec3(0.0) - vec3(other.g0.x, other.g0.y, other.g0.z), vec3(0.0) - other.g1, vec3(self.g0.x, self.g0.y, self.g0.z), vec2(0.0, self.g0.w));
}

MultiVector plane__sub__multi_vector(Plane self, MultiVector other) {
    return MultiVector(vec2(0.0) - other.g0, vec3(0.0) - other.g1, vec2(0.0) - other.g2, vec3(0.0) - other.g3, vec3(0.0) - other.g4, vec4(0.0) - other.g5, vec4(0.0) - other.g6, vec3(0.0) - other.g7, vec3(0.0) - other.g8, vec3(self.g0.x, self.g0.y, self.g0.z) - other.g9, vec2(0.0, self.g0.w) - other.g10);
}

Plane plane__sub__plane(Plane self, Plane other) {
    return Plane(self.g0 - other.g0);
}

MultiVector plane__sub__round_point(Plane self, RoundPoint other) {
    return MultiVector(vec2(0.0), vec3(0.0) - other.g0, vec2(0.0) - other.g1, vec3(0.0), vec3(0.0), vec4(0.0), vec4(0.0), vec3(0.0), vec3(0.0), vec3(self.g0.x, self.g0.y, self.g0.z), vec2(0.0, self.g0.w));
}

MultiVector plane__sub__scalar(Plane self, Scalar other) {
    return MultiVector(vec2(0.0) - vec2(other.g0, 0.0), vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0), vec4(0.0), vec4(0.0), vec3(0.0), vec3(0.0), vec3(self.g0.x, self.g0.y, self.g0.z), vec2(0.0, self.g0.w));
}

Sphere plane__sub__sphere(Plane self, Sphere other) {
    return Sphere(vec3(self.g0.x, self.g0.y, self.g0.z) - other.g0, vec2(0.0, self.g0.w) - other.g1);
}

MultiVector round_point__sub__anti_scalar(RoundPoint self, AntiScalar other) {
    return MultiVector(vec2(0.0) - vec2(0.0, other.g0), self.g0, self.g1, vec3(0.0), vec3(0.0), vec4(0.0), vec4(0.0), vec3(0.0), vec3(0.0), vec3(0.0), vec2(0.0));
}

MultiVector round_point__sub__circle(RoundPoint self, Circle other) {
    return MultiVector(vec2(0.0), self.g0, self.g1, vec3(0.0), vec3(0.0), vec4(0.0), vec4(0.0) - other.g0, vec3(0.0) - other.g1, vec3(0.0) - other.g2, vec3(0.0), vec2(0.0));
}

MultiVector round_point__sub__dipole(RoundPoint self, Dipole other) {
    return MultiVector(vec2(0.0), self.g0, self.g1, vec3(0.0) - other.g0, vec3(0.0) - other.g1, vec4(0.0) - other.g2, vec4(0.0), vec3(0.0), vec3(0.0), vec3(0.0), vec2(0.0));
}

MultiVector round_point__sub__dual_num(RoundPoint self, DualNum other) {
    return MultiVector(vec2(0.0) - other.g0, self.g0, self.g1, vec3(0.0), vec3(0.0), vec4(0.0), vec4(0.0), vec3(0.0), vec3(0.0), vec3(0.0), vec2(0.0));
}

MultiVector round_point__sub__flat_point(RoundPoint self, FlatPoint other) {
    return MultiVector(vec2(0.0), self.g0, self.g1, vec3(0.0), vec3(0.0), vec4(0.0) - other.g0, vec4(0.0), vec3(0.0), vec3(0.0), vec3(0.0), vec2(0.0));
}

MultiVector round_point__sub__flector(RoundPoint self, Flector other) {
    return MultiVector(vec2(0.0), self.g0, self.g1, vec3(0.0), vec3(0.0), vec4(0.0) - other.g0, vec4(0.0), vec3(0.0), vec3(0.0), vec3(0.0) - vec3(other.g1.x, other.g1.y, other.g1.z), vec2(0.0) - vec2(0.0, other.g1.w));
}

MultiVector round_point__sub__line(RoundPoint self, Line other) {
    return MultiVector(vec2(0.0), self.g0, self.g1, vec3(0.0), vec3(0.0), vec4(0.0), vec4(0.0), vec3(0.0) - other.g0, vec3(0.0) - other.g1, vec3(0.0), vec2(0.0));
}

MultiVector round_point__sub__motor(RoundPoint self, Motor other) {
    return MultiVector(vec2(0.0) - vec2(0.0, other.g0.w), self.g0, self.g1, vec3(0.0), vec3(0.0), vec4(0.0), vec4(0.0), vec3(0.0) - vec3(other.g0.x, other.g0.y, other.g0.z), vec3(0.0) - other.g1, vec3(0.0), vec2(0.0));
}

MultiVector round_point__sub__multi_vector(RoundPoint self, MultiVector other) {
    return MultiVector(vec2(0.0) - other.g0, self.g0 - other.g1, self.g1 - other.g2, vec3(0.0) - other.g3, vec3(0.0) - other.g4, vec4(0.0) - other.g5, vec4(0.0) - other.g6, vec3(0.0) - other.g7, vec3(0.0) - other.g8, vec3(0.0) - other.g9, vec2(0.0) - other.g10);
}

MultiVector round_point__sub__plane(RoundPoint self, Plane other) {
    return MultiVector(vec2(0.0), self.g0, self.g1, vec3(0.0), vec3(0.0), vec4(0.0), vec4(0.0), vec3(0.0), vec3(0.0), vec3(0.0) - vec3(other.g0.x, other.g0.y, other.g0.z), vec2(0.0) - vec2(0.0, other.g0.w));
}

RoundPoint round_point__sub__round_point(RoundPoint self, RoundPoint other) {
    return RoundPoint(self.g0 - other.g0, self.g1 - other.g1);
}

MultiVector round_point__sub__scalar(RoundPoint self, Scalar other) {
    return MultiVector(vec2(0.0) - vec2(other.g0, 0.0), self.g0, self.g1, vec3(0.0), vec3(0.0), vec4(0.0), vec4(0.0), vec3(0.0), vec3(0.0), vec3(0.0), vec2(0.0));
}

MultiVector round_point__sub__sphere(RoundPoint self, Sphere other) {
    return MultiVector(vec2(0.0), self.g0, self.g1, vec3(0.0), vec3(0.0), vec4(0.0), vec4(0.0), vec3(0.0), vec3(0.0), vec3(0.0) - other.g0, vec2(0.0) - other.g1);
}

DualNum scalar__sub__anti_scalar(Scalar self, AntiScalar other) {
    return DualNum(vec2(self.g0, 0.0) - vec2(0.0, other.g0));
}

MultiVector scalar__sub__circle(Scalar self, Circle other) {
    return MultiVector(vec2(self.g0, 0.0), vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0), vec4(0.0), vec4(0.0) - other.g0, vec3(0.0) - other.g1, vec3(0.0) - other.g2, vec3(0.0), vec2(0.0));
}

MultiVector scalar__sub__dipole(Scalar self, Dipole other) {
    return MultiVector(vec2(self.g0, 0.0), vec3(0.0), vec2(0.0), vec3(0.0) - other.g0, vec3(0.0) - other.g1, vec4(0.0) - other.g2, vec4(0.0), vec3(0.0), vec3(0.0), vec3(0.0), vec2(0.0));
}

DualNum scalar__sub__dual_num(Scalar self, DualNum other) {
    return DualNum(vec2(self.g0, 0.0) - other.g0);
}

MultiVector scalar__sub__flat_point(Scalar self, FlatPoint other) {
    return MultiVector(vec2(self.g0, 0.0), vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0), vec4(0.0) - other.g0, vec4(0.0), vec3(0.0), vec3(0.0), vec3(0.0), vec2(0.0));
}

MultiVector scalar__sub__flector(Scalar self, Flector other) {
    return MultiVector(vec2(self.g0, 0.0), vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0), vec4(0.0) - other.g0, vec4(0.0), vec3(0.0), vec3(0.0), vec3(0.0) - vec3(other.g1.x, other.g1.y, other.g1.z), vec2(0.0) - vec2(0.0, other.g1.w));
}

MultiVector scalar__sub__line(Scalar self, Line other) {
    return MultiVector(vec2(self.g0, 0.0), vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0), vec4(0.0), vec4(0.0), vec3(0.0) - other.g0, vec3(0.0) - other.g1, vec3(0.0), vec2(0.0));
}

MultiVector scalar__sub__motor(Scalar self, Motor other) {
    return MultiVector(vec2(self.g0, 0.0) - vec2(0.0, other.g0.w), vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0), vec4(0.0), vec4(0.0), vec3(0.0) - vec3(other.g0.x, other.g0.y, other.g0.z), vec3(0.0) - other.g1, vec3(0.0), vec2(0.0));
}

MultiVector scalar__sub__multi_vector(Scalar self, MultiVector other) {
    return MultiVector(vec2(self.g0, 0.0) - other.g0, vec3(0.0) - other.g1, vec2(0.0) - other.g2, vec3(0.0) - other.g3, vec3(0.0) - other.g4, vec4(0.0) - other.g5, vec4(0.0) - other.g6, vec3(0.0) - other.g7, vec3(0.0) - other.g8, vec3(0.0) - other.g9, vec2(0.0) - other.g10);
}

MultiVector scalar__sub__plane(Scalar self, Plane other) {
    return MultiVector(vec2(self.g0, 0.0), vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0), vec4(0.0), vec4(0.0), vec3(0.0), vec3(0.0), vec3(0.0) - vec3(other.g0.x, other.g0.y, other.g0.z), vec2(0.0) - vec2(0.0, other.g0.w));
}

MultiVector scalar__sub__round_point(Scalar self, RoundPoint other) {
    return MultiVector(vec2(self.g0, 0.0), vec3(0.0) - other.g0, vec2(0.0) - other.g1, vec3(0.0), vec3(0.0), vec4(0.0), vec4(0.0), vec3(0.0), vec3(0.0), vec3(0.0), vec2(0.0));
}

Scalar scalar__sub__scalar(Scalar self, Scalar other) {
    return Scalar(self.g0 - other.g0);
}

MultiVector scalar__sub__sphere(Scalar self, Sphere other) {
    return MultiVector(vec2(self.g0, 0.0), vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0), vec4(0.0), vec4(0.0), vec3(0.0), vec3(0.0), vec3(0.0) - other.g0, vec2(0.0) - other.g1);
}

MultiVector sphere__sub__anti_scalar(Sphere self, AntiScalar other) {
    return MultiVector(vec2(0.0) - vec2(0.0, other.g0), vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0), vec4(0.0), vec4(0.0), vec3(0.0), vec3(0.0), self.g0, self.g1);
}

MultiVector sphere__sub__circle(Sphere self, Circle other) {
    return MultiVector(vec2(0.0), vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0), vec4(0.0), vec4(0.0) - other.g0, vec3(0.0) - other.g1, vec3(0.0) - other.g2, self.g0, self.g1);
}

MultiVector sphere__sub__dipole(Sphere self, Dipole other) {
    return MultiVector(vec2(0.0), vec3(0.0), vec2(0.0), vec3(0.0) - other.g0, vec3(0.0) - other.g1, vec4(0.0) - other.g2, vec4(0.0), vec3(0.0), vec3(0.0), self.g0, self.g1);
}

MultiVector sphere__sub__dual_num(Sphere self, DualNum other) {
    return MultiVector(vec2(0.0) - other.g0, vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0), vec4(0.0), vec4(0.0), vec3(0.0), vec3(0.0), self.g0, self.g1);
}

MultiVector sphere__sub__flat_point(Sphere self, FlatPoint other) {
    return MultiVector(vec2(0.0), vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0), vec4(0.0) - other.g0, vec4(0.0), vec3(0.0), vec3(0.0), self.g0, self.g1);
}

MultiVector sphere__sub__flector(Sphere self, Flector other) {
    return MultiVector(vec2(0.0), vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0), vec4(0.0) - other.g0, vec4(0.0), vec3(0.0), vec3(0.0), self.g0 - vec3(other.g1.x, other.g1.y, other.g1.z), self.g1 - vec2(0.0, other.g1.w));
}

MultiVector sphere__sub__line(Sphere self, Line other) {
    return MultiVector(vec2(0.0), vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0), vec4(0.0), vec4(0.0), vec3(0.0) - other.g0, vec3(0.0) - other.g1, self.g0, self.g1);
}

MultiVector sphere__sub__motor(Sphere self, Motor other) {
    return MultiVector(vec2(0.0) - vec2(0.0, other.g0.w), vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0), vec4(0.0), vec4(0.0), vec3(0.0) - vec3(other.g0.x, other.g0.y, other.g0.z), vec3(0.0) - other.g1, self.g0, self.g1);
}

MultiVector sphere__sub__multi_vector(Sphere self, MultiVector other) {
    return MultiVector(vec2(0.0) - other.g0, vec3(0.0) - other.g1, vec2(0.0) - other.g2, vec3(0.0) - other.g3, vec3(0.0) - other.g4, vec4(0.0) - other.g5, vec4(0.0) - other.g6, vec3(0.0) - other.g7, vec3(0.0) - other.g8, self.g0 - other.g9, self.g1 - other.g10);
}

Sphere sphere__sub__plane(Sphere self, Plane other) {
    return Sphere(self.g0 - vec3(other.g0.x, other.g0.y, other.g0.z), self.g1 - vec2(0.0, other.g0.w));
}

MultiVector sphere__sub__round_point(Sphere self, RoundPoint other) {
    return MultiVector(vec2(0.0), vec3(0.0) - other.g0, vec2(0.0) - other.g1, vec3(0.0), vec3(0.0), vec4(0.0), vec4(0.0), vec3(0.0), vec3(0.0), self.g0, self.g1);
}

MultiVector sphere__sub__scalar(Sphere self, Scalar other) {
    return MultiVector(vec2(0.0) - vec2(other.g0, 0.0), vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0), vec4(0.0), vec4(0.0), vec3(0.0), vec3(0.0), self.g0, self.g1);
}

Sphere sphere__sub__sphere(Sphere self, Sphere other) {
    return Sphere(self.g0 - other.g0, self.g1 - other.g1);
}

AntiScalar anti_scalar__anti_wedge_dot__anti_scalar(AntiScalar self, AntiScalar other) {
    return AntiScalar(self.g0 * other.g0);
}

Circle anti_scalar__anti_wedge_dot__circle(AntiScalar self, Circle other) {
    return Circle(vec4(self.g0) * other.g0, vec3(self.g0) * other.g1, vec3(self.g0) * other.g2);
}

Dipole anti_scalar__anti_wedge_dot__dipole(AntiScalar self, Dipole other) {
    return Dipole(vec3(self.g0) * other.g0, vec3(self.g0) * other.g1, vec4(self.g0) * other.g2);
}

DualNum anti_scalar__anti_wedge_dot__dual_num(AntiScalar self, DualNum other) {
    return DualNum(vec2(self.g0) * other.g0);
}

FlatPoint anti_scalar__anti_wedge_dot__flat_point(AntiScalar self, FlatPoint other) {
    return FlatPoint(vec4(self.g0) * other.g0);
}

Flector anti_scalar__anti_wedge_dot__flector(AntiScalar self, Flector other) {
    return Flector(vec4(self.g0) * other.g0, vec4(self.g0) * other.g1);
}

Line anti_scalar__anti_wedge_dot__line(AntiScalar self, Line other) {
    return Line(vec3(self.g0) * other.g0, vec3(self.g0) * other.g1);
}

Motor anti_scalar__anti_wedge_dot__motor(AntiScalar self, Motor other) {
    return Motor(vec4(self.g0) * other.g0, vec3(self.g0) * other.g1);
}

MultiVector anti_scalar__anti_wedge_dot__multi_vector(AntiScalar self, MultiVector other) {
    return MultiVector(vec2(self.g0) * other.g0, vec3(self.g0) * other.g1, vec2(self.g0) * other.g2, vec3(self.g0) * other.g3, vec3(self.g0) * other.g4, vec4(self.g0) * other.g5, vec4(self.g0) * other.g6, vec3(self.g0) * other.g7, vec3(self.g0) * other.g8, vec3(self.g0) * other.g9, vec2(self.g0) * other.g10);
}

Plane anti_scalar__anti_wedge_dot__plane(AntiScalar self, Plane other) {
    return Plane(vec4(self.g0) * other.g0);
}

RoundPoint anti_scalar__anti_wedge_dot__round_point(AntiScalar self, RoundPoint other) {
    return RoundPoint(vec3(self.g0) * other.g0, vec2(self.g0) * other.g1);
}

Scalar anti_scalar__anti_wedge_dot__scalar(AntiScalar self, Scalar other) {
    return Scalar(self.g0 * other.g0);
}

Sphere anti_scalar__anti_wedge_dot__sphere(AntiScalar self, Sphere other) {
    return Sphere(vec3(self.g0) * other.g0, vec2(self.g0) * other.g1);
}

Circle circle__anti_wedge_dot__anti_scalar(Circle self, AntiScalar other) {
    return Circle(self.g0 * vec4(other.g0), self.g1 * vec3(other.g0), self.g2 * vec3(other.g0));
}

MultiVector circle__anti_wedge_dot__circle(Circle self, Circle other) {
    return MultiVector(vec2(self.g0.x) * vec2(0.0, -other.g2.x) + vec2(self.g0.y) * vec2(0.0, -other.g2.y) + vec2(self.g0.z) * vec2(0.0, -other.g2.z) + vec2(self.g0.w) * vec2(0.0, other.g0.w) + vec2(self.g1.x) * vec2(0.0, -other.g1.x) + vec2(self.g1.y) * vec2(0.0, -other.g1.y) + vec2(self.g1.z) * vec2(0.0, -other.g1.z) + vec2(self.g2.x) * vec2(0.0, -other.g0.x) + vec2(self.g2.y) * vec2(0.0, -other.g0.y) + vec2(self.g2.z) * vec2(0.0, -other.g0.z), vec3(self.g0.x) * vec3(0.0, other.g2.z, -other.g2.y) + vec3(self.g0.y) * vec3(-other.g2.z, 0.0, other.g2.x) + vec3(self.g0.z) * vec3(other.g2.y, -other.g2.x, 0.0) + vec3(self.g0.w) * other.g1 + self.g1 * vec3(other.g0.w) + vec3(self.g2.x) * vec3(0.0, -other.g0.z, other.g0.y) + vec3(self.g2.y) * vec3(other.g0.z, 0.0, -other.g0.x) + vec3(self.g2.z) * vec3(-other.g0.y, other.g0.x, 0.0), vec2(self.g0.x) * vec2(-other.g1.x, 0.0) + vec2(self.g0.y) * vec2(-other.g1.y, 0.0) + vec2(self.g0.z) * vec2(-other.g1.z, 0.0) - vec2(self.g1.x) * vec2(other.g0.x, other.g2.x) - vec2(self.g1.y) * vec2(other.g0.y, other.g2.y) - vec2(self.g1.z) * vec2(other.g0.z, other.g2.z) + vec2(self.g2.x) * vec2(0.0, -other.g1.x) + vec2(self.g2.y) * vec2(0.0, -other.g1.y) + vec2(self.g2.z) * vec2(0.0, -other.g1.z), vec3(0.0), vec3(0.0), vec4(0.0), vec4(self.g0.x) * vec4(other.g0.w, -other.g1.z, other.g1.y, other.g2.x) + vec4(self.g0.y) * vec4(other.g1.z, other.g0.w, -other.g1.x, other.g2.y) + vec4(self.g0.z) * vec4(-other.g1.y, other.g1.x, other.g0.w, other.g2.z) + vec4(self.g0.w) * vec4(-other.g0.x, -other.g0.y, -other.g0.z, 0.0) + vec4(self.g1.x) * vec4(0.0, -other.g0.z, other.g0.y, 0.0) + vec4(self.g1.y) * vec4(other.g0.z, 0.0, -other.g0.x, 0.0) + vec4(self.g1.z) * vec4(-other.g0.y, other.g0.x, 0.0, 0.0) + vec4(self.g2.x) * vec4(0.0, 0.0, 0.0, -other.g0.x) + vec4(self.g2.y) * vec4(0.0, 0.0, 0.0, -other.g0.y) + vec4(self.g2.z) * vec4(0.0, 0.0, 0.0, -other.g0.z), vec3(self.g0.x) * vec3(0.0, -other.g2.z, other.g2.y) + vec3(self.g0.y) * vec3(other.g2.z, 0.0, -other.g2.x) + vec3(self.g0.z) * vec3(-other.g2.y, other.g2.x, 0.0) + vec3(self.g1.x) * vec3(0.0, -other.g1.z, other.g1.y) + vec3(self.g1.y) * vec3(other.g1.z, 0.0, -other.g1.x) + vec3(self.g1.z) * vec3(-other.g1.y, other.g1.x, 0.0) + vec3(self.g2.x) * vec3(0.0, -other.g0.z, other.g0.y) + vec3(self.g2.y) * vec3(other.g0.z, 0.0, -other.g0.x) + vec3(self.g2.z) * vec3(-other.g0.y, other.g0.x, 0.0), vec3(self.g0.w) * other.g2 + vec3(self.g1.x) * vec3(0.0, -other.g2.z, other.g2.y) + vec3(self.g1.y) * vec3(other.g2.z, 0.0, -other.g2.x) + vec3(self.g1.z) * vec3(-other.g2.y, other.g2.x, 0.0) + vec3(self.g2.x) * vec3(-other.g0.w, -other.g1.z, other.g1.y) + vec3(self.g2.y) * vec3(other.g1.z, -other.g0.w, -other.g1.x) + vec3(self.g2.z) * vec3(-other.g1.y, other.g1.x, -other.g0.w), vec3(0.0), vec2(0.0));
}

MultiVector circle__anti_wedge_dot__dipole(Circle self, Dipole other) {
    return MultiVector(vec2(self.g0.x) * vec2(-other.g2.x, 0.0) + vec2(self.g0.y) * vec2(-other.g2.y, 0.0) + vec2(self.g0.z) * vec2(-other.g2.z, 0.0) + vec2(self.g0.w) * vec2(-other.g2.w, 0.0) + vec2(self.g1.x) * vec2(-other.g1.x, 0.0) + vec2(self.g1.y) * vec2(-other.g1.y, 0.0) + vec2(self.g1.z) * vec2(-other.g1.z, 0.0) + vec2(self.g2.x) * vec2(-other.g0.x, 0.0) + vec2(self.g2.y) * vec2(-other.g0.y, 0.0) + vec2(self.g2.z) * vec2(-other.g0.z, 0.0), vec3(0.0), vec2(0.0), vec3(self.g0.x) * vec3(-other.g2.w, -other.g1.z, other.g1.y) + vec3(self.g0.y) * vec3(other.g1.z, -other.g2.w, -other.g1.x) + vec3(self.g0.z) * vec3(-other.g1.y, other.g1.x, -other.g2.w) - vec3(self.g0.w) * other.g0 + vec3(self.g1.x) * vec3(0.0, -other.g0.z, other.g0.y) + vec3(self.g1.y) * vec3(other.g0.z, 0.0, -other.g0.x) + vec3(self.g1.z) * vec3(-other.g0.y, other.g0.x, 0.0), vec3(self.g0.x) * vec3(0.0, -other.g2.z, other.g2.y) + vec3(self.g0.y) * vec3(other.g2.z, 0.0, -other.g2.x) + vec3(self.g0.z) * vec3(-other.g2.y, other.g2.x, 0.0) + vec3(self.g1.x) * vec3(0.0, -other.g1.z, other.g1.y) + vec3(self.g1.y) * vec3(other.g1.z, 0.0, -other.g1.x) + vec3(self.g1.z) * vec3(-other.g1.y, other.g1.x, 0.0) + vec3(self.g2.x) * vec3(0.0, -other.g0.z, other.g0.y) + vec3(self.g2.y) * vec3(other.g0.z, 0.0, -other.g0.x) + vec3(self.g2.z) * vec3(-other.g0.y, other.g0.x, 0.0), vec4(self.g0.x) * vec4(0.0, 0.0, 0.0, -other.g2.x) + vec4(self.g0.y) * vec4(0.0, 0.0, 0.0, -other.g2.y) + self.g0.wwwz * other.g2.xyzz * vec4(1.0, 1.0, 1.0, -1.0) + vec4(self.g1.x) * vec4(0.0, -other.g2.z, other.g2.y, 0.0) + vec4(self.g1.y) * vec4(other.g2.z, 0.0, -other.g2.x, 0.0) + vec4(self.g1.z) * vec4(-other.g2.y, other.g2.x, 0.0, 0.0) + vec4(self.g2.x) * vec4(other.g2.w, -other.g1.z, other.g1.y, other.g0.x) + vec4(self.g2.y) * vec4(other.g1.z, other.g2.w, -other.g1.x, other.g0.y) + vec4(self.g2.z) * vec4(-other.g1.y, other.g1.x, other.g2.w, other.g0.z), vec4(0.0), vec3(0.0), vec3(0.0), vec3(self.g0.x) * vec3(0.0, -other.g2.z, other.g2.y) + vec3(self.g0.y) * vec3(other.g2.z, 0.0, -other.g2.x) + vec3(self.g0.z) * vec3(-other.g2.y, other.g2.x, 0.0) - vec3(self.g0.w) * other.g1 + self.g1 * vec3(other.g2.w) + vec3(self.g2.x) * vec3(0.0, other.g0.z, -other.g0.y) + vec3(self.g2.y) * vec3(-other.g0.z, 0.0, other.g0.x) + vec3(self.g2.z) * vec3(other.g0.y, -other.g0.x, 0.0), vec2(self.g0.x) * vec2(-other.g1.x, 0.0) + vec2(self.g0.y) * vec2(-other.g1.y, 0.0) + vec2(self.g0.z) * vec2(-other.g1.z, 0.0) - vec2(self.g1.x) * vec2(other.g0.x, other.g2.x) - vec2(self.g1.y) * vec2(other.g0.y, other.g2.y) - vec2(self.g1.z) * vec2(other.g0.z, other.g2.z) + vec2(self.g2.x) * vec2(0.0, -other.g1.x) + vec2(self.g2.y) * vec2(0.0, -other.g1.y) + vec2(self.g2.z) * vec2(0.0, -other.g1.z));
}

MultiVector circle__anti_wedge_dot__dual_num(Circle self, DualNum other) {
    return MultiVector(vec2(0.0), vec3(0.0), vec2(0.0), vec3(self.g0.x, self.g0.y, self.g0.z) * vec3(other.g0.x), self.g1 * vec3(other.g0.x), vec4(self.g0.w) * vec4(0.0, 0.0, 0.0, -other.g0.x) + vec4(self.g2.x, self.g2.y, self.g2.z, self.g2.x) * vec4(other.g0.x, other.g0.x, other.g0.x, 0.0), self.g0 * vec4(other.g0.y), self.g1 * vec3(other.g0.y), self.g2 * vec3(other.g0.y), vec3(0.0), vec2(0.0));
}

MultiVector circle__anti_wedge_dot__flat_point(Circle self, FlatPoint other) {
    return MultiVector(vec2(self.g0.x) * vec2(-other.g0.x, 0.0) + vec2(self.g0.y) * vec2(-other.g0.y, 0.0) + vec2(self.g0.z) * vec2(-other.g0.z, 0.0) + vec2(self.g0.w) * vec2(-other.g0.w, 0.0), vec3(0.0), vec2(0.0), vec3(0.0) - vec3(self.g0.x, self.g0.y, self.g0.z) * vec3(other.g0.w), vec3(self.g0.x) * vec3(0.0, -other.g0.z, other.g0.y) + vec3(self.g0.y) * vec3(other.g0.z, 0.0, -other.g0.x) + vec3(self.g0.z) * vec3(-other.g0.y, other.g0.x, 0.0), vec4(self.g0.x) * vec4(0.0, 0.0, 0.0, -other.g0.x) + vec4(self.g0.y) * vec4(0.0, 0.0, 0.0, -other.g0.y) + self.g0.wwwz * other.g0.xyzz * vec4(1.0, 1.0, 1.0, -1.0) + vec4(self.g1.x) * vec4(0.0, -other.g0.z, other.g0.y, 0.0) + vec4(self.g1.y) * vec4(other.g0.z, 0.0, -other.g0.x, 0.0) + vec4(self.g1.z) * vec4(-other.g0.y, other.g0.x, 0.0, 0.0) + vec4(self.g2.x, self.g2.y, self.g2.z, self.g2.x) * vec4(other.g0.w, other.g0.w, other.g0.w, 0.0), vec4(0.0), vec3(0.0), vec3(0.0), vec3(self.g0.x) * vec3(0.0, -other.g0.z, other.g0.y) + vec3(self.g0.y) * vec3(other.g0.z, 0.0, -other.g0.x) + vec3(self.g0.z) * vec3(-other.g0.y, other.g0.x, 0.0) + self.g1 * vec3(other.g0.w), vec2(self.g1.x) * vec2(0.0, -other.g0.x) + vec2(self.g1.y) * vec2(0.0, -other.g0.y) + vec2(self.g1.z) * vec2(0.0, -other.g0.z));
}

MultiVector circle__anti_wedge_dot__flector(Circle self, Flector other) {
    return MultiVector(vec2(self.g0.x) * vec2(-other.g0.x, 0.0) + vec2(self.g0.y) * vec2(-other.g0.y, 0.0) + vec2(self.g0.z) * vec2(-other.g0.z, 0.0) + vec2(self.g0.w) * vec2(-other.g0.w, 0.0), vec3(0.0), vec2(0.0), vec3(self.g0.x) * vec3(-other.g0.w, other.g1.z, -other.g1.y) + vec3(self.g0.y) * vec3(-other.g1.z, -other.g0.w, other.g1.x) + vec3(self.g0.z) * vec3(other.g1.y, -other.g1.x, -other.g0.w), vec3(self.g0.x) * vec3(other.g1.w, -other.g0.z, other.g0.y) + vec3(self.g0.y) * vec3(other.g0.z, other.g1.w, -other.g0.x) + vec3(self.g0.z) * vec3(-other.g0.y, other.g0.x, other.g1.w) - vec3(self.g0.w) * vec3(other.g1.x, other.g1.y, other.g1.z), vec4(self.g0.x) * vec4(0.0, 0.0, 0.0, -other.g0.x) + vec4(self.g0.y) * vec4(0.0, 0.0, 0.0, -other.g0.y) + self.g0.wwwz * other.g0.xyzz * vec4(1.0, 1.0, 1.0, -1.0) + vec4(self.g1.x) * vec4(other.g1.w, -other.g0.z, other.g0.y, -other.g1.x) + vec4(self.g1.y) * vec4(other.g0.z, other.g1.w, -other.g0.x, -other.g1.y) + vec4(self.g1.z) * vec4(-other.g0.y, other.g0.x, other.g1.w, -other.g1.z) + vec4(self.g2.x) * vec4(other.g0.w, -other.g1.z, other.g1.y, 0.0) + vec4(self.g2.y) * vec4(other.g1.z, other.g0.w, -other.g1.x, 0.0) + vec4(self.g2.z) * vec4(-other.g1.y, other.g1.x, other.g0.w, 0.0), vec4(0.0), vec3(0.0), vec3(0.0), vec3(self.g0.x) * vec3(other.g1.w, -other.g0.z, other.g0.y) + vec3(self.g0.y) * vec3(other.g0.z, other.g1.w, -other.g0.x) + vec3(self.g0.z) * vec3(-other.g0.y, other.g0.x, other.g1.w) + vec3(self.g1.x) * vec3(other.g0.w, -other.g1.z, other.g1.y) + vec3(self.g1.y) * vec3(other.g1.z, other.g0.w, -other.g1.x) + vec3(self.g1.z) * vec3(-other.g1.y, other.g1.x, other.g0.w), vec2(self.g0.x) * vec2(other.g1.x, 0.0) + vec2(self.g0.y) * vec2(other.g1.y, 0.0) + vec2(self.g0.z, self.g0.w) * vec2(other.g1.z, other.g1.w) + vec2(self.g1.x) * vec2(0.0, -other.g0.x) + vec2(self.g1.y) * vec2(0.0, -other.g0.y) + vec2(self.g1.z) * vec2(0.0, -other.g0.z) + vec2(self.g2.x) * vec2(0.0, -other.g1.x) + vec2(self.g2.y) * vec2(0.0, -other.g1.y) + vec2(self.g2.z) * vec2(0.0, -other.g1.z));
}

MultiVector circle__anti_wedge_dot__line(Circle self, Line other) {
    return MultiVector(vec2(self.g0.x) * vec2(0.0, -other.g1.x) + vec2(self.g0.y) * vec2(0.0, -other.g1.y) + vec2(self.g0.z) * vec2(0.0, -other.g1.z) + vec2(self.g1.x) * vec2(0.0, -other.g0.x) + vec2(self.g1.y) * vec2(0.0, -other.g0.y) + vec2(self.g1.z) * vec2(0.0, -other.g0.z), vec3(self.g0.x) * vec3(0.0, other.g1.z, -other.g1.y) + vec3(self.g0.y) * vec3(-other.g1.z, 0.0, other.g1.x) + vec3(self.g0.z) * vec3(other.g1.y, -other.g1.x, 0.0) + vec3(self.g0.w) * other.g0, vec2(self.g0.x) * vec2(-other.g0.x, 0.0) + vec2(self.g0.y) * vec2(-other.g0.y, 0.0) + vec2(self.g0.z) * vec2(-other.g0.z, 0.0) + vec2(self.g1.x) * vec2(0.0, -other.g1.x) + vec2(self.g1.y) * vec2(0.0, -other.g1.y) + vec2(self.g1.z) * vec2(0.0, -other.g1.z) + vec2(self.g2.x) * vec2(0.0, -other.g0.x) + vec2(self.g2.y) * vec2(0.0, -other.g0.y) + vec2(self.g2.z) * vec2(0.0, -other.g0.z), vec3(0.0), vec3(0.0), vec4(0.0), vec4(self.g0.x) * vec4(0.0, -other.g0.z, other.g0.y, other.g1.x) + vec4(self.g0.y) * vec4(other.g0.z, 0.0, -other.g0.x, other.g1.y) + vec4(self.g0.z) * vec4(-other.g0.y, other.g0.x, 0.0, other.g1.z), vec3(self.g0.x) * vec3(0.0, -other.g1.z, other.g1.y) + vec3(self.g0.y) * vec3(other.g1.z, 0.0, -other.g1.x) + vec3(self.g0.z) * vec3(-other.g1.y, other.g1.x, 0.0) + vec3(self.g1.x) * vec3(0.0, -other.g0.z, other.g0.y) + vec3(self.g1.y) * vec3(other.g0.z, 0.0, -other.g0.x) + vec3(self.g1.z) * vec3(-other.g0.y, other.g0.x, 0.0), vec3(self.g0.w) * other.g1 + vec3(self.g1.x) * vec3(0.0, -other.g1.z, other.g1.y) + vec3(self.g1.y) * vec3(other.g1.z, 0.0, -other.g1.x) + vec3(self.g1.z) * vec3(-other.g1.y, other.g1.x, 0.0) + vec3(self.g2.x) * vec3(0.0, -other.g0.z, other.g0.y) + vec3(self.g2.y) * vec3(other.g0.z, 0.0, -other.g0.x) + vec3(self.g2.z) * vec3(-other.g0.y, other.g0.x, 0.0), vec3(0.0), vec2(0.0));
}

MultiVector circle__anti_wedge_dot__motor(Circle self, Motor other) {
    return MultiVector(vec2(self.g0.x) * vec2(0.0, -other.g1.x) + vec2(self.g0.y) * vec2(0.0, -other.g1.y) + vec2(self.g0.z) * vec2(0.0, -other.g1.z) + vec2(self.g1.x) * vec2(0.0, -other.g0.x) + vec2(self.g1.y) * vec2(0.0, -other.g0.y) + vec2(self.g1.z) * vec2(0.0, -other.g0.z), vec3(self.g0.x) * vec3(0.0, other.g1.z, -other.g1.y) + vec3(self.g0.y) * vec3(-other.g1.z, 0.0, other.g1.x) + vec3(self.g0.z) * vec3(other.g1.y, -other.g1.x, 0.0) + vec3(self.g0.w) * vec3(other.g0.x, other.g0.y, other.g0.z), vec2(self.g0.x) * vec2(-other.g0.x, 0.0) + vec2(self.g0.y) * vec2(-other.g0.y, 0.0) + vec2(self.g0.z) * vec2(-other.g0.z, 0.0) + vec2(self.g1.x) * vec2(0.0, -other.g1.x) + vec2(self.g1.y) * vec2(0.0, -other.g1.y) + vec2(self.g1.z) * vec2(0.0, -other.g1.z) + vec2(self.g2.x) * vec2(0.0, -other.g0.x) + vec2(self.g2.y) * vec2(0.0, -other.g0.y) + vec2(self.g2.z) * vec2(0.0, -other.g0.z), vec3(0.0), vec3(0.0), vec4(0.0), vec4(self.g0.x) * vec4(other.g0.w, -other.g0.z, other.g0.y, other.g1.x) + vec4(self.g0.y) * vec4(other.g0.z, other.g0.w, -other.g0.x, other.g1.y) + vec4(self.g0.z) * vec4(-other.g0.y, other.g0.x, other.g0.w, other.g1.z) + vec4(self.g0.w) * vec4(0.0, 0.0, 0.0, other.g0.w), vec3(self.g0.x) * vec3(0.0, -other.g1.z, other.g1.y) + vec3(self.g0.y) * vec3(other.g1.z, 0.0, -other.g1.x) + vec3(self.g0.z) * vec3(-other.g1.y, other.g1.x, 0.0) + vec3(self.g1.x) * vec3(other.g0.w, -other.g0.z, other.g0.y) + vec3(self.g1.y) * vec3(other.g0.z, other.g0.w, -other.g0.x) + vec3(self.g1.z) * vec3(-other.g0.y, other.g0.x, other.g0.w), vec3(self.g0.w) * other.g1 + vec3(self.g1.x) * vec3(0.0, -other.g1.z, other.g1.y) + vec3(self.g1.y) * vec3(other.g1.z, 0.0, -other.g1.x) + vec3(self.g1.z) * vec3(-other.g1.y, other.g1.x, 0.0) + vec3(self.g2.x) * vec3(other.g0.w, -other.g0.z, other.g0.y) + vec3(self.g2.y) * vec3(other.g0.z, other.g0.w, -other.g0.x) + vec3(self.g2.z) * vec3(-other.g0.y, other.g0.x, other.g0.w), vec3(0.0), vec2(0.0));
}

MultiVector circle__anti_wedge_dot__multi_vector(Circle self, MultiVector other) {
    return MultiVector(vec2(0.0) - vec2(self.g0.x) * vec2(other.g5.x, other.g8.x) - vec2(self.g0.y) * vec2(other.g5.y, other.g8.y) - vec2(self.g0.z) * vec2(other.g5.z, other.g8.z) + vec2(self.g0.w) * vec2(-other.g5.w, other.g6.w) - vec2(self.g1.x) * vec2(other.g4.x, other.g7.x) - vec2(self.g1.y) * vec2(other.g4.y, other.g7.y) - vec2(self.g1.z) * vec2(other.g4.z, other.g7.z) - vec2(self.g2.x) * vec2(other.g3.x, other.g6.x) - vec2(self.g2.y) * vec2(other.g3.y, other.g6.y) - vec2(self.g2.z) * vec2(other.g3.z, other.g6.z), vec3(self.g0.x) * vec3(-other.g2.y, other.g8.z, -other.g8.y) + vec3(self.g0.y) * vec3(-other.g8.z, -other.g2.y, other.g8.x) + vec3(self.g0.z) * vec3(other.g8.y, -other.g8.x, -other.g2.y) + vec3(self.g0.w) * other.g7 + vec3(self.g1.x) * vec3(other.g6.w, -other.g1.z, other.g1.y) + vec3(self.g1.y) * vec3(other.g1.z, other.g6.w, -other.g1.x) + vec3(self.g1.z) * vec3(-other.g1.y, other.g1.x, other.g6.w) + vec3(self.g2.x) * vec3(other.g2.x, -other.g6.z, other.g6.y) + vec3(self.g2.y) * vec3(other.g6.z, other.g2.x, -other.g6.x) + vec3(self.g2.z) * vec3(-other.g6.y, other.g6.x, other.g2.x), vec2(self.g0.x) * vec2(-other.g7.x, 0.0) + vec2(self.g0.x) * vec2(-other.g1.x, 0.0) + vec2(self.g0.y) * vec2(-other.g7.y, 0.0) + vec2(self.g0.y) * vec2(-other.g1.y, 0.0) + vec2(self.g0.z) * vec2(-other.g7.z, 0.0) + vec2(self.g0.z) * vec2(-other.g1.z, 0.0) + vec2(self.g0.w) * other.g2 * vec2(-1.0, 1.0) - vec2(self.g1.x) * vec2(other.g6.x, other.g8.x) - vec2(self.g1.y) * vec2(other.g6.y, other.g8.y) - vec2(self.g1.z) * vec2(other.g6.z, other.g8.z) + vec2(self.g2.x) * vec2(0.0, -other.g7.x) + vec2(self.g2.x) * vec2(0.0, other.g1.x) + vec2(self.g2.y) * vec2(0.0, -other.g7.y) + vec2(self.g2.y) * vec2(0.0, other.g1.y) + vec2(self.g2.z) * vec2(0.0, -other.g7.z) + vec2(self.g2.z) * vec2(0.0, other.g1.z), vec3(self.g0.x) * vec3(-other.g5.w, other.g9.z, -other.g9.y) + vec3(self.g0.x) * vec3(other.g0.x, -other.g4.z, other.g4.y) + vec3(self.g0.y) * vec3(-other.g9.z, -other.g5.w, other.g9.x) + vec3(self.g0.y) * vec3(other.g4.z, other.g0.x, -other.g4.x) + vec3(self.g0.z) * vec3(other.g9.y, -other.g9.x, -other.g5.w) + vec3(self.g0.z) * vec3(-other.g4.y, other.g4.x, other.g0.x) - vec3(self.g0.w) * other.g3 + vec3(self.g1.x) * vec3(other.g10.x, -other.g3.z, other.g3.y) + vec3(self.g1.y) * vec3(other.g3.z, other.g10.x, -other.g3.x) + vec3(self.g1.z) * vec3(-other.g3.y, other.g3.x, other.g10.x), vec3(self.g0.x) * vec3(other.g10.y, -other.g5.z, other.g5.y) + vec3(self.g0.y) * vec3(other.g5.z, other.g10.y, -other.g5.x) + vec3(self.g0.z) * vec3(-other.g5.y, other.g5.x, other.g10.y) - vec3(self.g0.w) * other.g9 + vec3(self.g1.x) * vec3(other.g0.x, -other.g4.z, other.g4.y) + vec3(self.g1.y) * vec3(other.g4.z, other.g0.x, -other.g4.x) + vec3(self.g1.z) * vec3(-other.g4.y, other.g4.x, other.g0.x) + vec3(self.g2.x) * vec3(other.g10.x, -other.g3.z, other.g3.y) + vec3(self.g2.y) * vec3(other.g3.z, other.g10.x, -other.g3.x) + vec3(self.g2.z) * vec3(-other.g3.y, other.g3.x, other.g10.x), vec4(self.g0.x) * vec4(0.0, 0.0, 0.0, -other.g5.x) + vec4(self.g0.y) * vec4(0.0, 0.0, 0.0, -other.g5.y) + vec4(self.g0.z) * vec4(0.0, 0.0, 0.0, -other.g5.z) + vec4(self.g0.w) * vec4(other.g5.x, other.g5.y, other.g5.z, -other.g0.x) + vec4(self.g1.x) * vec4(other.g10.y, -other.g5.z, other.g5.y, -other.g9.x) + vec4(self.g1.y) * vec4(other.g5.z, other.g10.y, -other.g5.x, -other.g9.y) + vec4(self.g1.z) * vec4(-other.g5.y, other.g5.x, other.g10.y, -other.g9.z) + vec4(self.g2.x) * vec4(other.g5.w, -other.g9.z, other.g9.y, other.g3.x) + vec4(self.g2.x) * vec4(other.g0.x, -other.g4.z, other.g4.y, 0.0) + vec4(self.g2.y) * vec4(other.g9.z, other.g5.w, -other.g9.x, other.g3.y) + vec4(self.g2.y) * vec4(other.g4.z, other.g0.x, -other.g4.x, 0.0) + vec4(self.g2.z) * vec4(-other.g9.y, other.g9.x, other.g5.w, other.g3.z) + vec4(self.g2.z) * vec4(-other.g4.y, other.g4.x, other.g0.x, 0.0), vec4(self.g0.x) * vec4(other.g6.w, -other.g7.z, other.g7.y, other.g8.x) + vec4(self.g0.x) * vec4(other.g0.y, -other.g1.z, other.g1.y, 0.0) + vec4(self.g0.y) * vec4(other.g7.z, other.g6.w, -other.g7.x, other.g8.y) + vec4(self.g0.y) * vec4(other.g1.z, other.g0.y, -other.g1.x, 0.0) + vec4(self.g0.z) * vec4(-other.g7.y, other.g7.x, other.g6.w, other.g8.z) + vec4(self.g0.z) * vec4(-other.g1.y, other.g1.x, other.g0.y, 0.0) + vec4(self.g0.w) * vec4(-other.g6.x, -other.g6.y, -other.g6.z, other.g0.y) + vec4(self.g1.x) * vec4(other.g2.x, -other.g6.z, other.g6.y, -other.g1.x) + vec4(self.g1.y) * vec4(other.g6.z, other.g2.x, -other.g6.x, -other.g1.y) + vec4(self.g1.z) * vec4(-other.g6.y, other.g6.x, other.g2.x, -other.g1.z) + vec4(self.g2.x) * vec4(0.0, 0.0, 0.0, -other.g6.x) + vec4(self.g2.y) * vec4(0.0, 0.0, 0.0, -other.g6.y) + vec4(self.g2.z) * vec4(0.0, 0.0, 0.0, -other.g6.z), vec3(self.g0.x) * vec3(other.g2.y, -other.g8.z, other.g8.y) + vec3(self.g0.y) * vec3(other.g8.z, other.g2.y, -other.g8.x) + vec3(self.g0.z) * vec3(-other.g8.y, other.g8.x, other.g2.y) + vec3(self.g0.w) * other.g1 + vec3(self.g1.x) * vec3(other.g0.y, -other.g7.z, other.g7.y) + vec3(self.g1.y) * vec3(other.g7.z, other.g0.y, -other.g7.x) + vec3(self.g1.z) * vec3(-other.g7.y, other.g7.x, other.g0.y) + vec3(self.g2.x) * vec3(other.g2.x, -other.g6.z, other.g6.y) + vec3(self.g2.y) * vec3(other.g6.z, other.g2.x, -other.g6.x) + vec3(self.g2.z) * vec3(-other.g6.y, other.g6.x, other.g2.x), vec3(self.g0.w) * other.g8 + vec3(self.g1.x) * vec3(other.g2.y, -other.g8.z, other.g8.y) + vec3(self.g1.y) * vec3(other.g8.z, other.g2.y, -other.g8.x) + vec3(self.g1.z) * vec3(-other.g8.y, other.g8.x, other.g2.y) + vec3(self.g2.x) * vec3(-other.g6.w, -other.g7.z, other.g7.y) + vec3(self.g2.x) * vec3(other.g0.y, other.g1.z, -other.g1.y) + vec3(self.g2.y) * vec3(other.g7.z, -other.g6.w, -other.g7.x) + vec3(self.g2.y) * vec3(-other.g1.z, other.g0.y, other.g1.x) + vec3(self.g2.z) * vec3(-other.g7.y, other.g7.x, -other.g6.w) + vec3(self.g2.z) * vec3(other.g1.y, -other.g1.x, other.g0.y), vec3(self.g0.x) * vec3(other.g10.y, -other.g5.z, other.g5.y) + vec3(self.g0.y) * vec3(other.g5.z, other.g10.y, -other.g5.x) + vec3(self.g0.z) * vec3(-other.g5.y, other.g5.x, other.g10.y) - vec3(self.g0.w) * other.g4 + vec3(self.g1.x) * vec3(other.g5.w, -other.g9.z, other.g9.y) + vec3(self.g1.y) * vec3(other.g9.z, other.g5.w, -other.g9.x) + vec3(self.g1.z) * vec3(-other.g9.y, other.g9.x, other.g5.w) + vec3(self.g2.x) * vec3(-other.g10.x, other.g3.z, -other.g3.y) + vec3(self.g2.y) * vec3(-other.g3.z, -other.g10.x, other.g3.x) + vec3(self.g2.z) * vec3(other.g3.y, -other.g3.x, -other.g10.x), vec2(self.g0.x) * vec2(other.g9.x, 0.0) + vec2(self.g0.x) * vec2(-other.g4.x, 0.0) + vec2(self.g0.y) * vec2(other.g9.y, 0.0) + vec2(self.g0.y) * vec2(-other.g4.y, 0.0) + vec2(self.g0.z) * vec2(other.g9.z, 0.0) + vec2(self.g0.z) * vec2(-other.g4.z, 0.0) + vec2(self.g0.w) * other.g10 * vec2(-1.0, 1.0) - vec2(self.g1.x) * vec2(other.g3.x, other.g5.x) - vec2(self.g1.y) * vec2(other.g3.y, other.g5.y) - vec2(self.g1.z) * vec2(other.g3.z, other.g5.z) + vec2(self.g2.x) * vec2(0.0, -other.g9.x) + vec2(self.g2.x) * vec2(0.0, -other.g4.x) + vec2(self.g2.y) * vec2(0.0, -other.g9.y) + vec2(self.g2.y) * vec2(0.0, -other.g4.y) + vec2(self.g2.z) * vec2(0.0, -other.g9.z) + vec2(self.g2.z) * vec2(0.0, -other.g4.z));
}

MultiVector circle__anti_wedge_dot__plane(Circle self, Plane other) {
    return MultiVector(vec2(0.0), vec3(0.0), vec2(0.0), vec3(self.g0.x) * vec3(0.0, other.g0.z, -other.g0.y) + vec3(self.g0.y) * vec3(-other.g0.z, 0.0, other.g0.x) + vec3(self.g0.z) * vec3(other.g0.y, -other.g0.x, 0.0), vec3(self.g0.x, self.g0.y, self.g0.z) * vec3(other.g0.w) - vec3(self.g0.w) * vec3(other.g0.x, other.g0.y, other.g0.z), vec4(self.g1.x) * vec4(other.g0.w, 0.0, 0.0, -other.g0.x) + vec4(self.g1.y) * vec4(0.0, other.g0.w, 0.0, -other.g0.y) + vec4(self.g1.z) * vec4(0.0, 0.0, other.g0.w, -other.g0.z) + vec4(self.g2.x) * vec4(0.0, -other.g0.z, other.g0.y, 0.0) + vec4(self.g2.y) * vec4(other.g0.z, 0.0, -other.g0.x, 0.0) + vec4(self.g2.z) * vec4(-other.g0.y, other.g0.x, 0.0, 0.0), vec4(0.0), vec3(0.0), vec3(0.0), vec3(self.g0.x, self.g0.y, self.g0.z) * vec3(other.g0.w) + vec3(self.g1.x) * vec3(0.0, -other.g0.z, other.g0.y) + vec3(self.g1.y) * vec3(other.g0.z, 0.0, -other.g0.x) + vec3(self.g1.z) * vec3(-other.g0.y, other.g0.x, 0.0), vec2(self.g0.x) * vec2(other.g0.x, 0.0) + vec2(self.g0.y) * vec2(other.g0.y, 0.0) + vec2(self.g0.z, self.g0.w) * vec2(other.g0.z, other.g0.w) + vec2(self.g2.x) * vec2(0.0, -other.g0.x) + vec2(self.g2.y) * vec2(0.0, -other.g0.y) + vec2(self.g2.z) * vec2(0.0, -other.g0.z));
}

MultiVector circle__anti_wedge_dot__round_point(Circle self, RoundPoint other) {
    return MultiVector(vec2(0.0), vec3(0.0) - vec3(self.g0.x, self.g0.y, self.g0.z) * vec3(other.g1.y) + vec3(self.g1.x) * vec3(0.0, -other.g0.z, other.g0.y) + vec3(self.g1.y) * vec3(other.g0.z, 0.0, -other.g0.x) + vec3(self.g1.z) * vec3(-other.g0.y, other.g0.x, 0.0) + self.g2 * vec3(other.g1.x), vec2(self.g0.x) * vec2(-other.g0.x, 0.0) + vec2(self.g0.y) * vec2(-other.g0.y, 0.0) + vec2(self.g0.z) * vec2(-other.g0.z, 0.0) + vec2(self.g0.w) * other.g1 * vec2(-1.0, 1.0) + vec2(self.g2.x) * vec2(0.0, other.g0.x) + vec2(self.g2.y) * vec2(0.0, other.g0.y) + vec2(self.g2.z) * vec2(0.0, other.g0.z), vec3(0.0), vec3(0.0), vec4(0.0), vec4(self.g0.x) * vec4(0.0, -other.g0.z, other.g0.y, 0.0) + vec4(self.g0.y) * vec4(other.g0.z, 0.0, -other.g0.x, 0.0) + vec4(self.g0.z) * vec4(-other.g0.y, other.g0.x, 0.0, 0.0) + vec4(self.g1.x) * vec4(other.g1.x, 0.0, 0.0, -other.g0.x) + vec4(self.g1.y) * vec4(0.0, other.g1.x, 0.0, -other.g0.y) + vec4(self.g1.z) * vec4(0.0, 0.0, other.g1.x, -other.g0.z), vec3(self.g0.x, self.g0.y, self.g0.z) * vec3(other.g1.y) + vec3(self.g0.w) * other.g0 + self.g2 * vec3(other.g1.x), self.g1 * vec3(other.g1.y) + vec3(self.g2.x) * vec3(0.0, other.g0.z, -other.g0.y) + vec3(self.g2.y) * vec3(-other.g0.z, 0.0, other.g0.x) + vec3(self.g2.z) * vec3(other.g0.y, -other.g0.x, 0.0), vec3(0.0), vec2(0.0));
}

Dipole circle__anti_wedge_dot__scalar(Circle self, Scalar other) {
    return Dipole(vec3(self.g0.x, self.g0.y, self.g0.z) * vec3(other.g0), self.g1 * vec3(other.g0), vec4(self.g0.w) * vec4(0.0, 0.0, 0.0, -other.g0) + vec4(self.g2.x, self.g2.y, self.g2.z, self.g2.x) * vec4(other.g0, other.g0, other.g0, 0.0));
}

MultiVector circle__anti_wedge_dot__sphere(Circle self, Sphere other) {
    return MultiVector(vec2(0.0), vec3(0.0), vec2(0.0), vec3(self.g0.x) * vec3(0.0, other.g0.z, -other.g0.y) + vec3(self.g0.y) * vec3(-other.g0.z, 0.0, other.g0.x) + vec3(self.g0.z) * vec3(other.g0.y, -other.g0.x, 0.0) + self.g1 * vec3(other.g1.x), vec3(self.g0.x, self.g0.y, self.g0.z) * vec3(other.g1.y) - vec3(self.g0.w) * other.g0 + self.g2 * vec3(other.g1.x), vec4(self.g1.x) * vec4(other.g1.y, 0.0, 0.0, -other.g0.x) + vec4(self.g1.y) * vec4(0.0, other.g1.y, 0.0, -other.g0.y) + vec4(self.g1.z) * vec4(0.0, 0.0, other.g1.y, -other.g0.z) + vec4(self.g2.x) * vec4(0.0, -other.g0.z, other.g0.y, 0.0) + vec4(self.g2.y) * vec4(other.g0.z, 0.0, -other.g0.x, 0.0) + vec4(self.g2.z) * vec4(-other.g0.y, other.g0.x, 0.0, 0.0), vec4(0.0), vec3(0.0), vec3(0.0), vec3(self.g0.x, self.g0.y, self.g0.z) * vec3(other.g1.y) + vec3(self.g1.x) * vec3(0.0, -other.g0.z, other.g0.y) + vec3(self.g1.y) * vec3(other.g0.z, 0.0, -other.g0.x) + vec3(self.g1.z) * vec3(-other.g0.y, other.g0.x, 0.0) - self.g2 * vec3(other.g1.x), vec2(self.g0.x) * vec2(other.g0.x, 0.0) + vec2(self.g0.y) * vec2(other.g0.y, 0.0) + vec2(self.g0.z) * vec2(other.g0.z, 0.0) + vec2(self.g0.w) * other.g1 * vec2(-1.0, 1.0) + vec2(self.g2.x) * vec2(0.0, -other.g0.x) + vec2(self.g2.y) * vec2(0.0, -other.g0.y) + vec2(self.g2.z) * vec2(0.0, -other.g0.z));
}

Dipole dipole__anti_wedge_dot__anti_scalar(Dipole self, AntiScalar other) {
    return Dipole(self.g0 * vec3(other.g0), self.g1 * vec3(other.g0), self.g2 * vec4(other.g0));
}

MultiVector dipole__anti_wedge_dot__circle(Dipole self, Circle other) {
    return MultiVector(vec2(self.g0.x) * vec2(-other.g2.x, 0.0) + vec2(self.g0.y) * vec2(-other.g2.y, 0.0) + vec2(self.g0.z) * vec2(-other.g2.z, 0.0) + vec2(self.g1.x) * vec2(-other.g1.x, 0.0) + vec2(self.g1.y) * vec2(-other.g1.y, 0.0) + vec2(self.g1.z) * vec2(-other.g1.z, 0.0) + vec2(self.g2.x) * vec2(-other.g0.x, 0.0) + vec2(self.g2.y) * vec2(-other.g0.y, 0.0) + vec2(self.g2.z) * vec2(-other.g0.z, 0.0) + vec2(self.g2.w) * vec2(-other.g0.w, 0.0), vec3(0.0), vec2(0.0), vec3(self.g0.x) * vec3(other.g0.w, -other.g1.z, other.g1.y) + vec3(self.g0.y) * vec3(other.g1.z, other.g0.w, -other.g1.x) + vec3(self.g0.z) * vec3(-other.g1.y, other.g1.x, other.g0.w) + vec3(self.g1.x) * vec3(0.0, -other.g0.z, other.g0.y) + vec3(self.g1.y) * vec3(other.g0.z, 0.0, -other.g0.x) + vec3(self.g1.z) * vec3(-other.g0.y, other.g0.x, 0.0) + vec3(self.g2.w) * vec3(other.g0.x, other.g0.y, other.g0.z), vec3(self.g0.x) * vec3(0.0, -other.g2.z, other.g2.y) + vec3(self.g0.y) * vec3(other.g2.z, 0.0, -other.g2.x) + vec3(self.g0.z) * vec3(-other.g2.y, other.g2.x, 0.0) + vec3(self.g1.x) * vec3(0.0, -other.g1.z, other.g1.y) + vec3(self.g1.y) * vec3(other.g1.z, 0.0, -other.g1.x) + vec3(self.g1.z) * vec3(-other.g1.y, other.g1.x, 0.0) + vec3(self.g2.x) * vec3(0.0, -other.g0.z, other.g0.y) + vec3(self.g2.y) * vec3(other.g0.z, 0.0, -other.g0.x) + vec3(self.g2.z) * vec3(-other.g0.y, other.g0.x, 0.0), vec4(self.g0.x) * vec4(0.0, 0.0, 0.0, -other.g2.x) + vec4(self.g0.y) * vec4(0.0, 0.0, 0.0, -other.g2.y) + vec4(self.g0.z) * vec4(0.0, 0.0, 0.0, -other.g2.z) + vec4(self.g1.x) * vec4(0.0, -other.g2.z, other.g2.y, 0.0) + vec4(self.g1.y) * vec4(other.g2.z, 0.0, -other.g2.x, 0.0) + vec4(self.g1.z) * vec4(-other.g2.y, other.g2.x, 0.0, 0.0) + vec4(self.g2.x) * vec4(-other.g0.w, -other.g1.z, other.g1.y, other.g0.x) + vec4(self.g2.y) * vec4(other.g1.z, -other.g0.w, -other.g1.x, other.g0.y) + vec4(self.g2.z) * vec4(-other.g1.y, other.g1.x, -other.g0.w, other.g0.z) + vec4(self.g2.w) * vec4(-other.g2.x, -other.g2.y, -other.g2.z, 0.0), vec4(0.0), vec3(0.0), vec3(0.0), vec3(self.g0.x) * vec3(0.0, -other.g2.z, other.g2.y) + vec3(self.g0.y) * vec3(other.g2.z, 0.0, -other.g2.x) + vec3(self.g0.z) * vec3(-other.g2.y, other.g2.x, 0.0) - self.g1 * vec3(other.g0.w) + vec3(self.g2.x) * vec3(0.0, other.g0.z, -other.g0.y) + vec3(self.g2.y) * vec3(-other.g0.z, 0.0, other.g0.x) + vec3(self.g2.z) * vec3(other.g0.y, -other.g0.x, 0.0) + vec3(self.g2.w) * other.g1, vec2(self.g0.x) * vec2(-other.g1.x, 0.0) + vec2(self.g0.y) * vec2(-other.g1.y, 0.0) + vec2(self.g0.z) * vec2(-other.g1.z, 0.0) - vec2(self.g1.x) * vec2(other.g0.x, other.g2.x) - vec2(self.g1.y) * vec2(other.g0.y, other.g2.y) - vec2(self.g1.z) * vec2(other.g0.z, other.g2.z) + vec2(self.g2.x) * vec2(0.0, -other.g1.x) + vec2(self.g2.y) * vec2(0.0, -other.g1.y) + vec2(self.g2.z) * vec2(0.0, -other.g1.z));
}

MultiVector dipole__anti_wedge_dot__dipole(Dipole self, Dipole other) {
    return MultiVector(vec2(self.g0.x) * vec2(0.0, other.g2.x) + vec2(self.g0.y) * vec2(0.0, other.g2.y) + vec2(self.g0.z) * vec2(0.0, other.g2.z) + vec2(self.g1.x) * vec2(0.0, other.g1.x) + vec2(self.g1.y) * vec2(0.0, other.g1.y) + vec2(self.g1.z) * vec2(0.0, other.g1.z) + vec2(self.g2.x) * vec2(0.0, other.g0.x) + vec2(self.g2.y) * vec2(0.0, other.g0.y) + vec2(self.g2.z) * vec2(0.0, other.g0.z) + vec2(self.g2.w) * vec2(0.0, -other.g2.w), vec3(self.g0.x) * vec3(0.0, -other.g2.z, other.g2.y) + vec3(self.g0.y) * vec3(other.g2.z, 0.0, -other.g2.x) + vec3(self.g0.z) * vec3(-other.g2.y, other.g2.x, 0.0) + self.g1 * vec3(other.g2.w) + vec3(self.g2.x) * vec3(0.0, other.g0.z, -other.g0.y) + vec3(self.g2.y) * vec3(-other.g0.z, 0.0, other.g0.x) + vec3(self.g2.z) * vec3(other.g0.y, -other.g0.x, 0.0) + vec3(self.g2.w) * other.g1, vec2(self.g0.x) * vec2(other.g1.x, 0.0) + vec2(self.g0.y) * vec2(other.g1.y, 0.0) + vec2(self.g0.z) * vec2(other.g1.z, 0.0) + vec2(self.g1.x) * vec2(other.g0.x, other.g2.x) + vec2(self.g1.y) * vec2(other.g0.y, other.g2.y) + vec2(self.g1.z) * vec2(other.g0.z, other.g2.z) + vec2(self.g2.x) * vec2(0.0, other.g1.x) + vec2(self.g2.y) * vec2(0.0, other.g1.y) + vec2(self.g2.z) * vec2(0.0, other.g1.z), vec3(0.0), vec3(0.0), vec4(0.0), vec4(self.g0.x) * vec4(other.g2.w, other.g1.z, -other.g1.y, -other.g2.x) + vec4(self.g0.y) * vec4(-other.g1.z, other.g2.w, other.g1.x, -other.g2.y) + vec4(self.g0.z) * vec4(other.g1.y, -other.g1.x, other.g2.w, -other.g2.z) + vec4(self.g1.x) * vec4(0.0, other.g0.z, -other.g0.y, 0.0) + vec4(self.g1.y) * vec4(-other.g0.z, 0.0, other.g0.x, 0.0) + vec4(self.g1.z) * vec4(other.g0.y, -other.g0.x, 0.0, 0.0) + vec4(self.g2.x) * vec4(0.0, 0.0, 0.0, other.g0.x) + vec4(self.g2.y) * vec4(0.0, 0.0, 0.0, other.g0.y) + self.g2.wwwz * vec4(-other.g0.x, -other.g0.y, -other.g0.z, other.g0.z), vec3(self.g0.x) * vec3(0.0, other.g2.z, -other.g2.y) + vec3(self.g0.y) * vec3(-other.g2.z, 0.0, other.g2.x) + vec3(self.g0.z) * vec3(other.g2.y, -other.g2.x, 0.0) + vec3(self.g1.x) * vec3(0.0, other.g1.z, -other.g1.y) + vec3(self.g1.y) * vec3(-other.g1.z, 0.0, other.g1.x) + vec3(self.g1.z) * vec3(other.g1.y, -other.g1.x, 0.0) + vec3(self.g2.x) * vec3(0.0, other.g0.z, -other.g0.y) + vec3(self.g2.y) * vec3(-other.g0.z, 0.0, other.g0.x) + vec3(self.g2.z) * vec3(other.g0.y, -other.g0.x, 0.0), vec3(self.g1.x) * vec3(0.0, other.g2.z, -other.g2.y) + vec3(self.g1.y) * vec3(-other.g2.z, 0.0, other.g2.x) + vec3(self.g1.z) * vec3(other.g2.y, -other.g2.x, 0.0) + vec3(self.g2.x) * vec3(-other.g2.w, other.g1.z, -other.g1.y) + vec3(self.g2.y) * vec3(-other.g1.z, -other.g2.w, other.g1.x) + vec3(self.g2.z) * vec3(other.g1.y, -other.g1.x, -other.g2.w) + vec3(self.g2.w) * vec3(other.g2.x, other.g2.y, other.g2.z), vec3(0.0), vec2(0.0));
}

MultiVector dipole__anti_wedge_dot__dual_num(Dipole self, DualNum other) {
    return MultiVector(vec2(0.0), vec3(0.0), vec2(0.0), self.g0 * vec3(other.g0.y), self.g1 * vec3(other.g0.y), self.g2 * vec4(other.g0.y), vec4(self.g0.x, self.g0.y, self.g0.z, self.g0.x) * vec4(-other.g0.x, -other.g0.x, -other.g0.x, 0.0) + vec4(self.g2.w) * vec4(0.0, 0.0, 0.0, other.g0.x), vec3(0.0) - self.g1 * vec3(other.g0.x), vec3(0.0) - vec3(self.g2.x, self.g2.y, self.g2.z) * vec3(other.g0.x), vec3(0.0), vec2(0.0));
}

MultiVector dipole__anti_wedge_dot__flat_point(Dipole self, FlatPoint other) {
    return MultiVector(vec2(self.g0.x) * vec2(0.0, other.g0.x) + vec2(self.g0.y) * vec2(0.0, other.g0.y) + vec2(self.g0.z) * vec2(0.0, other.g0.z) + vec2(self.g2.w) * vec2(0.0, -other.g0.w), vec3(self.g0.x) * vec3(0.0, -other.g0.z, other.g0.y) + vec3(self.g0.y) * vec3(other.g0.z, 0.0, -other.g0.x) + vec3(self.g0.z) * vec3(-other.g0.y, other.g0.x, 0.0) + self.g1 * vec3(other.g0.w), vec2(self.g1.x) * vec2(0.0, other.g0.x) + vec2(self.g1.y) * vec2(0.0, other.g0.y) + vec2(self.g1.z) * vec2(0.0, other.g0.z), vec3(0.0), vec3(0.0), vec4(0.0), vec4(self.g0.x) * vec4(other.g0.w, 0.0, 0.0, -other.g0.x) + vec4(self.g0.y) * vec4(0.0, other.g0.w, 0.0, -other.g0.y) + vec4(self.g0.z) * vec4(0.0, 0.0, other.g0.w, -other.g0.z), vec3(self.g0.x) * vec3(0.0, other.g0.z, -other.g0.y) + vec3(self.g0.y) * vec3(-other.g0.z, 0.0, other.g0.x) + vec3(self.g0.z) * vec3(other.g0.y, -other.g0.x, 0.0), vec3(self.g1.x) * vec3(0.0, other.g0.z, -other.g0.y) + vec3(self.g1.y) * vec3(-other.g0.z, 0.0, other.g0.x) + vec3(self.g1.z) * vec3(other.g0.y, -other.g0.x, 0.0) - vec3(self.g2.x, self.g2.y, self.g2.z) * vec3(other.g0.w) + vec3(self.g2.w) * vec3(other.g0.x, other.g0.y, other.g0.z), vec3(0.0), vec2(0.0));
}

MultiVector dipole__anti_wedge_dot__flector(Dipole self, Flector other) {
    return MultiVector(vec2(self.g0.x) * vec2(0.0, other.g0.x) + vec2(self.g0.y) * vec2(0.0, other.g0.y) + vec2(self.g0.z) * vec2(0.0, other.g0.z) + vec2(self.g2.w) * vec2(0.0, -other.g0.w), vec3(self.g0.x) * vec3(other.g1.w, -other.g0.z, other.g0.y) + vec3(self.g0.y) * vec3(other.g0.z, other.g1.w, -other.g0.x) + vec3(self.g0.z) * vec3(-other.g0.y, other.g0.x, other.g1.w) + vec3(self.g1.x) * vec3(other.g0.w, -other.g1.z, other.g1.y) + vec3(self.g1.y) * vec3(other.g1.z, other.g0.w, -other.g1.x) + vec3(self.g1.z) * vec3(-other.g1.y, other.g1.x, other.g0.w), vec2(self.g0.x) * vec2(-other.g1.x, 0.0) + vec2(self.g0.y) * vec2(-other.g1.y, 0.0) + vec2(self.g0.z) * vec2(-other.g1.z, 0.0) + vec2(self.g1.x) * vec2(0.0, other.g0.x) + vec2(self.g1.y) * vec2(0.0, other.g0.y) + vec2(self.g1.z) * vec2(0.0, other.g0.z) + vec2(self.g2.x) * vec2(0.0, other.g1.x) + vec2(self.g2.y) * vec2(0.0, other.g1.y) + vec2(self.g2.z) * vec2(0.0, other.g1.z) + vec2(self.g2.w) * vec2(0.0, other.g1.w), vec3(0.0), vec3(0.0), vec4(0.0), vec4(self.g0.x) * vec4(other.g0.w, -other.g1.z, other.g1.y, -other.g0.x) + vec4(self.g0.y) * vec4(other.g1.z, other.g0.w, -other.g1.x, -other.g0.y) + vec4(self.g0.z) * vec4(-other.g1.y, other.g1.x, other.g0.w, -other.g0.z) + vec4(self.g1.x) * vec4(0.0, 0.0, 0.0, -other.g1.x) + vec4(self.g1.y) * vec4(0.0, 0.0, 0.0, -other.g1.y) + vec4(self.g1.z) * vec4(0.0, 0.0, 0.0, -other.g1.z), vec3(self.g0.x) * vec3(-other.g1.w, other.g0.z, -other.g0.y) + vec3(self.g0.y) * vec3(-other.g0.z, -other.g1.w, other.g0.x) + vec3(self.g0.z) * vec3(other.g0.y, -other.g0.x, -other.g1.w) - vec3(self.g2.w) * vec3(other.g1.x, other.g1.y, other.g1.z), vec3(self.g1.x) * vec3(-other.g1.w, other.g0.z, -other.g0.y) + vec3(self.g1.y) * vec3(-other.g0.z, -other.g1.w, other.g0.x) + vec3(self.g1.z) * vec3(other.g0.y, -other.g0.x, -other.g1.w) + vec3(self.g2.x) * vec3(-other.g0.w, other.g1.z, -other.g1.y) + vec3(self.g2.y) * vec3(-other.g1.z, -other.g0.w, other.g1.x) + vec3(self.g2.z) * vec3(other.g1.y, -other.g1.x, -other.g0.w) + vec3(self.g2.w) * vec3(other.g0.x, other.g0.y, other.g0.z), vec3(0.0), vec2(0.0));
}

MultiVector dipole__anti_wedge_dot__line(Dipole self, Line other) {
    return MultiVector(vec2(self.g0.x) * vec2(-other.g1.x, 0.0) + vec2(self.g0.y) * vec2(-other.g1.y, 0.0) + vec2(self.g0.z) * vec2(-other.g1.z, 0.0) + vec2(self.g1.x) * vec2(-other.g0.x, 0.0) + vec2(self.g1.y) * vec2(-other.g0.y, 0.0) + vec2(self.g1.z) * vec2(-other.g0.z, 0.0), vec3(0.0), vec2(0.0), vec3(self.g0.x) * vec3(0.0, -other.g0.z, other.g0.y) + vec3(self.g0.y) * vec3(other.g0.z, 0.0, -other.g0.x) + vec3(self.g0.z) * vec3(-other.g0.y, other.g0.x, 0.0), vec3(self.g0.x) * vec3(0.0, -other.g1.z, other.g1.y) + vec3(self.g0.y) * vec3(other.g1.z, 0.0, -other.g1.x) + vec3(self.g0.z) * vec3(-other.g1.y, other.g1.x, 0.0) + vec3(self.g1.x) * vec3(0.0, -other.g0.z, other.g0.y) + vec3(self.g1.y) * vec3(other.g0.z, 0.0, -other.g0.x) + vec3(self.g1.z) * vec3(-other.g0.y, other.g0.x, 0.0), vec4(self.g0.x) * vec4(0.0, 0.0, 0.0, -other.g1.x) + vec4(self.g0.y) * vec4(0.0, 0.0, 0.0, -other.g1.y) + vec4(self.g0.z) * vec4(0.0, 0.0, 0.0, -other.g1.z) + vec4(self.g1.x) * vec4(0.0, -other.g1.z, other.g1.y, 0.0) + vec4(self.g1.y) * vec4(other.g1.z, 0.0, -other.g1.x, 0.0) + vec4(self.g1.z) * vec4(-other.g1.y, other.g1.x, 0.0, 0.0) + vec4(self.g2.x) * vec4(0.0, -other.g0.z, other.g0.y, 0.0) + vec4(self.g2.y) * vec4(other.g0.z, 0.0, -other.g0.x, 0.0) + vec4(self.g2.z) * vec4(-other.g0.y, other.g0.x, 0.0, 0.0) + vec4(self.g2.w) * vec4(-other.g1.x, -other.g1.y, -other.g1.z, 0.0), vec4(0.0), vec3(0.0), vec3(0.0), vec3(self.g0.x) * vec3(0.0, -other.g1.z, other.g1.y) + vec3(self.g0.y) * vec3(other.g1.z, 0.0, -other.g1.x) + vec3(self.g0.z) * vec3(-other.g1.y, other.g1.x, 0.0) + vec3(self.g2.w) * other.g0, vec2(self.g0.x) * vec2(-other.g0.x, 0.0) + vec2(self.g0.y) * vec2(-other.g0.y, 0.0) + vec2(self.g0.z) * vec2(-other.g0.z, 0.0) + vec2(self.g1.x) * vec2(0.0, -other.g1.x) + vec2(self.g1.y) * vec2(0.0, -other.g1.y) + vec2(self.g1.z) * vec2(0.0, -other.g1.z) + vec2(self.g2.x) * vec2(0.0, -other.g0.x) + vec2(self.g2.y) * vec2(0.0, -other.g0.y) + vec2(self.g2.z) * vec2(0.0, -other.g0.z));
}

MultiVector dipole__anti_wedge_dot__motor(Dipole self, Motor other) {
    return MultiVector(vec2(self.g0.x) * vec2(-other.g1.x, 0.0) + vec2(self.g0.y) * vec2(-other.g1.y, 0.0) + vec2(self.g0.z) * vec2(-other.g1.z, 0.0) + vec2(self.g1.x) * vec2(-other.g0.x, 0.0) + vec2(self.g1.y) * vec2(-other.g0.y, 0.0) + vec2(self.g1.z) * vec2(-other.g0.z, 0.0), vec3(0.0), vec2(0.0), vec3(self.g0.x) * vec3(other.g0.w, -other.g0.z, other.g0.y) + vec3(self.g0.y) * vec3(other.g0.z, other.g0.w, -other.g0.x) + vec3(self.g0.z) * vec3(-other.g0.y, other.g0.x, other.g0.w), vec3(self.g0.x) * vec3(0.0, -other.g1.z, other.g1.y) + vec3(self.g0.y) * vec3(other.g1.z, 0.0, -other.g1.x) + vec3(self.g0.z) * vec3(-other.g1.y, other.g1.x, 0.0) + vec3(self.g1.x) * vec3(other.g0.w, -other.g0.z, other.g0.y) + vec3(self.g1.y) * vec3(other.g0.z, other.g0.w, -other.g0.x) + vec3(self.g1.z) * vec3(-other.g0.y, other.g0.x, other.g0.w), vec4(self.g0.x) * vec4(0.0, 0.0, 0.0, -other.g1.x) + vec4(self.g0.y) * vec4(0.0, 0.0, 0.0, -other.g1.y) + vec4(self.g0.z) * vec4(0.0, 0.0, 0.0, -other.g1.z) + vec4(self.g1.x) * vec4(0.0, -other.g1.z, other.g1.y, 0.0) + vec4(self.g1.y) * vec4(other.g1.z, 0.0, -other.g1.x, 0.0) + vec4(self.g1.z) * vec4(-other.g1.y, other.g1.x, 0.0, 0.0) + vec4(self.g2.x) * vec4(other.g0.w, -other.g0.z, other.g0.y, 0.0) + vec4(self.g2.y) * vec4(other.g0.z, other.g0.w, -other.g0.x, 0.0) + vec4(self.g2.z) * vec4(-other.g0.y, other.g0.x, other.g0.w, 0.0) + vec4(self.g2.w) * vec4(-other.g1.x, -other.g1.y, -other.g1.z, other.g0.w), vec4(0.0), vec3(0.0), vec3(0.0), vec3(self.g0.x) * vec3(0.0, -other.g1.z, other.g1.y) + vec3(self.g0.y) * vec3(other.g1.z, 0.0, -other.g1.x) + vec3(self.g0.z) * vec3(-other.g1.y, other.g1.x, 0.0) + vec3(self.g2.w) * vec3(other.g0.x, other.g0.y, other.g0.z), vec2(self.g0.x) * vec2(-other.g0.x, 0.0) + vec2(self.g0.y) * vec2(-other.g0.y, 0.0) + vec2(self.g0.z) * vec2(-other.g0.z, 0.0) + vec2(self.g1.x) * vec2(0.0, -other.g1.x) + vec2(self.g1.y) * vec2(0.0, -other.g1.y) + vec2(self.g1.z) * vec2(0.0, -other.g1.z) + vec2(self.g2.x) * vec2(0.0, -other.g0.x) + vec2(self.g2.y) * vec2(0.0, -other.g0.y) + vec2(self.g2.z) * vec2(0.0, -other.g0.z));
}

MultiVector dipole__anti_wedge_dot__multi_vector(Dipole self, MultiVector other) {
    return MultiVector(vec2(self.g0.x) * vec2(-other.g8.x, other.g5.x) + vec2(self.g0.y) * vec2(-other.g8.y, other.g5.y) + vec2(self.g0.z) * vec2(-other.g8.z, other.g5.z) + vec2(self.g1.x) * vec2(-other.g7.x, other.g4.x) + vec2(self.g1.y) * vec2(-other.g7.y, other.g4.y) + vec2(self.g1.z) * vec2(-other.g7.z, other.g4.z) + vec2(self.g2.x) * vec2(-other.g6.x, other.g3.x) + vec2(self.g2.y) * vec2(-other.g6.y, other.g3.y) + vec2(self.g2.z) * vec2(-other.g6.z, other.g3.z) - vec2(self.g2.w) * vec2(other.g6.w, other.g5.w), vec3(self.g0.x) * vec3(other.g10.y, -other.g5.z, other.g5.y) + vec3(self.g0.y) * vec3(other.g5.z, other.g10.y, -other.g5.x) + vec3(self.g0.z) * vec3(-other.g5.y, other.g5.x, other.g10.y) + vec3(self.g1.x) * vec3(other.g5.w, -other.g9.z, other.g9.y) + vec3(self.g1.y) * vec3(other.g9.z, other.g5.w, -other.g9.x) + vec3(self.g1.z) * vec3(-other.g9.y, other.g9.x, other.g5.w) + vec3(self.g2.x) * vec3(-other.g10.x, other.g3.z, -other.g3.y) + vec3(self.g2.y) * vec3(-other.g3.z, -other.g10.x, other.g3.x) + vec3(self.g2.z) * vec3(other.g3.y, -other.g3.x, -other.g10.x) + vec3(self.g2.w) * other.g4, vec2(self.g0.x) * vec2(-other.g9.x, 0.0) + vec2(self.g0.x) * vec2(other.g4.x, 0.0) + vec2(self.g0.y) * vec2(-other.g9.y, 0.0) + vec2(self.g0.y) * vec2(other.g4.y, 0.0) + vec2(self.g0.z) * vec2(-other.g9.z, 0.0) + vec2(self.g0.z) * vec2(other.g4.z, 0.0) + vec2(self.g1.x) * vec2(other.g3.x, other.g5.x) + vec2(self.g1.y) * vec2(other.g3.y, other.g5.y) + vec2(self.g1.z) * vec2(other.g3.z, other.g5.z) + vec2(self.g2.x) * vec2(0.0, other.g9.x) + vec2(self.g2.x) * vec2(0.0, other.g4.x) + vec2(self.g2.y) * vec2(0.0, other.g9.y) + vec2(self.g2.y) * vec2(0.0, other.g4.y) + vec2(self.g2.z) * vec2(0.0, other.g9.z) + vec2(self.g2.z) * vec2(0.0, other.g4.z) + vec2(self.g2.w) * other.g10 * vec2(-1.0, 1.0), vec3(self.g0.x) * vec3(other.g6.w, -other.g7.z, other.g7.y) + vec3(self.g0.x) * vec3(other.g0.y, -other.g1.z, other.g1.y) + vec3(self.g0.y) * vec3(other.g7.z, other.g6.w, -other.g7.x) + vec3(self.g0.y) * vec3(other.g1.z, other.g0.y, -other.g1.x) + vec3(self.g0.z) * vec3(-other.g7.y, other.g7.x, other.g6.w) + vec3(self.g0.z) * vec3(-other.g1.y, other.g1.x, other.g0.y) + vec3(self.g1.x) * vec3(other.g2.x, -other.g6.z, other.g6.y) + vec3(self.g1.y) * vec3(other.g6.z, other.g2.x, -other.g6.x) + vec3(self.g1.z) * vec3(-other.g6.y, other.g6.x, other.g2.x) + vec3(self.g2.w) * vec3(other.g6.x, other.g6.y, other.g6.z), vec3(self.g0.x) * vec3(other.g2.y, -other.g8.z, other.g8.y) + vec3(self.g0.y) * vec3(other.g8.z, other.g2.y, -other.g8.x) + vec3(self.g0.z) * vec3(-other.g8.y, other.g8.x, other.g2.y) + vec3(self.g1.x) * vec3(other.g0.y, -other.g7.z, other.g7.y) + vec3(self.g1.y) * vec3(other.g7.z, other.g0.y, -other.g7.x) + vec3(self.g1.z) * vec3(-other.g7.y, other.g7.x, other.g0.y) + vec3(self.g2.x) * vec3(other.g2.x, -other.g6.z, other.g6.y) + vec3(self.g2.y) * vec3(other.g6.z, other.g2.x, -other.g6.x) + vec3(self.g2.z) * vec3(-other.g6.y, other.g6.x, other.g2.x) - vec3(self.g2.w) * other.g1, vec4(self.g0.x) * vec4(0.0, 0.0, 0.0, -other.g8.x) + vec4(self.g0.y) * vec4(0.0, 0.0, 0.0, -other.g8.y) + vec4(self.g0.z) * vec4(0.0, 0.0, 0.0, -other.g8.z) + vec4(self.g1.x) * vec4(other.g2.y, -other.g8.z, other.g8.y, other.g1.x) + vec4(self.g1.y) * vec4(other.g8.z, other.g2.y, -other.g8.x, other.g1.y) + vec4(self.g1.z) * vec4(-other.g8.y, other.g8.x, other.g2.y, other.g1.z) + vec4(self.g2.x) * vec4(-other.g6.w, -other.g7.z, other.g7.y, other.g6.x) + vec4(self.g2.x) * vec4(other.g0.y, other.g1.z, -other.g1.y, 0.0) + vec4(self.g2.y) * vec4(other.g7.z, -other.g6.w, -other.g7.x, other.g6.y) + vec4(self.g2.y) * vec4(-other.g1.z, other.g0.y, other.g1.x, 0.0) + vec4(self.g2.z) * vec4(-other.g7.y, other.g7.x, -other.g6.w, other.g6.z) + vec4(self.g2.z) * vec4(other.g1.y, -other.g1.x, other.g0.y, 0.0) + vec4(self.g2.w) * vec4(-other.g8.x, -other.g8.y, -other.g8.z, other.g0.y), vec4(self.g0.x) * vec4(other.g5.w, -other.g9.z, other.g9.y, -other.g5.x) + vec4(self.g0.x) * vec4(-other.g0.x, other.g4.z, -other.g4.y, 0.0) + vec4(self.g0.y) * vec4(other.g9.z, other.g5.w, -other.g9.x, -other.g5.y) + vec4(self.g0.y) * vec4(-other.g4.z, -other.g0.x, other.g4.x, 0.0) + vec4(self.g0.z) * vec4(-other.g9.y, other.g9.x, other.g5.w, -other.g5.z) + vec4(self.g0.z) * vec4(other.g4.y, -other.g4.x, -other.g0.x, 0.0) + vec4(self.g1.x) * vec4(-other.g10.x, other.g3.z, -other.g3.y, -other.g9.x) + vec4(self.g1.y) * vec4(-other.g3.z, -other.g10.x, other.g3.x, -other.g9.y) + vec4(self.g1.z) * vec4(other.g3.y, -other.g3.x, -other.g10.x, -other.g9.z) + vec4(self.g2.x) * vec4(0.0, 0.0, 0.0, other.g3.x) + vec4(self.g2.y) * vec4(0.0, 0.0, 0.0, other.g3.y) + vec4(self.g2.z) * vec4(0.0, 0.0, 0.0, other.g3.z) + vec4(self.g2.w) * vec4(-other.g3.x, -other.g3.y, -other.g3.z, other.g0.x), vec3(self.g0.x) * vec3(-other.g10.y, other.g5.z, -other.g5.y) + vec3(self.g0.y) * vec3(-other.g5.z, -other.g10.y, other.g5.x) + vec3(self.g0.z) * vec3(other.g5.y, -other.g5.x, -other.g10.y) + vec3(self.g1.x) * vec3(-other.g0.x, other.g4.z, -other.g4.y) + vec3(self.g1.y) * vec3(-other.g4.z, -other.g0.x, other.g4.x) + vec3(self.g1.z) * vec3(other.g4.y, -other.g4.x, -other.g0.x) + vec3(self.g2.x) * vec3(-other.g10.x, other.g3.z, -other.g3.y) + vec3(self.g2.y) * vec3(-other.g3.z, -other.g10.x, other.g3.x) + vec3(self.g2.z) * vec3(other.g3.y, -other.g3.x, -other.g10.x) - vec3(self.g2.w) * other.g9, vec3(self.g1.x) * vec3(-other.g10.y, other.g5.z, -other.g5.y) + vec3(self.g1.y) * vec3(-other.g5.z, -other.g10.y, other.g5.x) + vec3(self.g1.z) * vec3(other.g5.y, -other.g5.x, -other.g10.y) + vec3(self.g2.x) * vec3(-other.g5.w, other.g9.z, -other.g9.y) + vec3(self.g2.x) * vec3(-other.g0.x, other.g4.z, -other.g4.y) + vec3(self.g2.y) * vec3(-other.g9.z, -other.g5.w, other.g9.x) + vec3(self.g2.y) * vec3(-other.g4.z, -other.g0.x, other.g4.x) + vec3(self.g2.z) * vec3(other.g9.y, -other.g9.x, -other.g5.w) + vec3(self.g2.z) * vec3(other.g4.y, -other.g4.x, -other.g0.x) + vec3(self.g2.w) * vec3(other.g5.x, other.g5.y, other.g5.z), vec3(self.g0.x) * vec3(other.g2.y, -other.g8.z, other.g8.y) + vec3(self.g0.y) * vec3(other.g8.z, other.g2.y, -other.g8.x) + vec3(self.g0.z) * vec3(-other.g8.y, other.g8.x, other.g2.y) + vec3(self.g1.x) * vec3(-other.g6.w, other.g1.z, -other.g1.y) + vec3(self.g1.y) * vec3(-other.g1.z, -other.g6.w, other.g1.x) + vec3(self.g1.z) * vec3(other.g1.y, -other.g1.x, -other.g6.w) + vec3(self.g2.x) * vec3(-other.g2.x, other.g6.z, -other.g6.y) + vec3(self.g2.y) * vec3(-other.g6.z, -other.g2.x, other.g6.x) + vec3(self.g2.z) * vec3(other.g6.y, -other.g6.x, -other.g2.x) + vec3(self.g2.w) * other.g7, vec2(self.g0.x) * vec2(-other.g7.x, 0.0) + vec2(self.g0.x) * vec2(-other.g1.x, 0.0) + vec2(self.g0.y) * vec2(-other.g7.y, 0.0) + vec2(self.g0.y) * vec2(-other.g1.y, 0.0) + vec2(self.g0.z) * vec2(-other.g7.z, 0.0) + vec2(self.g0.z) * vec2(-other.g1.z, 0.0) - vec2(self.g1.x) * vec2(other.g6.x, other.g8.x) - vec2(self.g1.y) * vec2(other.g6.y, other.g8.y) - vec2(self.g1.z) * vec2(other.g6.z, other.g8.z) + vec2(self.g2.x) * vec2(0.0, -other.g7.x) + vec2(self.g2.x) * vec2(0.0, other.g1.x) + vec2(self.g2.y) * vec2(0.0, -other.g7.y) + vec2(self.g2.y) * vec2(0.0, other.g1.y) + vec2(self.g2.z) * vec2(0.0, -other.g7.z) + vec2(self.g2.z) * vec2(0.0, other.g1.z) + vec2(self.g2.w) * other.g2 * vec2(1.0, -1.0));
}

MultiVector dipole__anti_wedge_dot__plane(Dipole self, Plane other) {
    return MultiVector(vec2(0.0), self.g0 * vec3(other.g0.w) + vec3(self.g1.x) * vec3(0.0, -other.g0.z, other.g0.y) + vec3(self.g1.y) * vec3(other.g0.z, 0.0, -other.g0.x) + vec3(self.g1.z) * vec3(-other.g0.y, other.g0.x, 0.0), vec2(self.g0.x) * vec2(-other.g0.x, 0.0) + vec2(self.g0.y) * vec2(-other.g0.y, 0.0) + vec2(self.g0.z) * vec2(-other.g0.z, 0.0) + vec2(self.g2.x) * vec2(0.0, other.g0.x) + vec2(self.g2.y) * vec2(0.0, other.g0.y) + vec2(self.g2.z) * vec2(0.0, other.g0.z) + vec2(self.g2.w) * vec2(0.0, other.g0.w), vec3(0.0), vec3(0.0), vec4(0.0), vec4(self.g0.x) * vec4(0.0, -other.g0.z, other.g0.y, 0.0) + vec4(self.g0.y) * vec4(other.g0.z, 0.0, -other.g0.x, 0.0) + vec4(self.g0.z) * vec4(-other.g0.y, other.g0.x, 0.0, 0.0) + vec4(self.g1.x) * vec4(0.0, 0.0, 0.0, -other.g0.x) + vec4(self.g1.y) * vec4(0.0, 0.0, 0.0, -other.g0.y) + vec4(self.g1.z) * vec4(0.0, 0.0, 0.0, -other.g0.z), vec3(0.0) - self.g0 * vec3(other.g0.w) - vec3(self.g2.w) * vec3(other.g0.x, other.g0.y, other.g0.z), vec3(0.0) - self.g1 * vec3(other.g0.w) + vec3(self.g2.x) * vec3(0.0, other.g0.z, -other.g0.y) + vec3(self.g2.y) * vec3(-other.g0.z, 0.0, other.g0.x) + vec3(self.g2.z) * vec3(other.g0.y, -other.g0.x, 0.0), vec3(0.0), vec2(0.0));
}

MultiVector dipole__anti_wedge_dot__round_point(Dipole self, RoundPoint other) {
    return MultiVector(vec2(0.0), vec3(0.0), vec2(0.0), vec3(self.g0.x) * vec3(0.0, -other.g0.z, other.g0.y) + vec3(self.g0.y) * vec3(other.g0.z, 0.0, -other.g0.x) + vec3(self.g0.z) * vec3(-other.g0.y, other.g0.x, 0.0) + self.g1 * vec3(other.g1.x), self.g0 * vec3(other.g1.y) + vec3(self.g2.x, self.g2.y, self.g2.z) * vec3(other.g1.x) - vec3(self.g2.w) * other.g0, vec4(self.g1.x) * vec4(other.g1.y, 0.0, 0.0, other.g0.x) + vec4(self.g1.y) * vec4(0.0, other.g1.y, 0.0, other.g0.y) + vec4(self.g1.z) * vec4(0.0, 0.0, other.g1.y, other.g0.z) + vec4(self.g2.x) * vec4(0.0, other.g0.z, -other.g0.y, 0.0) + vec4(self.g2.y) * vec4(-other.g0.z, 0.0, other.g0.x, 0.0) + vec4(self.g2.z) * vec4(other.g0.y, -other.g0.x, 0.0, 0.0), vec4(0.0), vec3(0.0), vec3(0.0), self.g0 * vec3(other.g1.y) + vec3(self.g1.x) * vec3(0.0, other.g0.z, -other.g0.y) + vec3(self.g1.y) * vec3(-other.g0.z, 0.0, other.g0.x) + vec3(self.g1.z) * vec3(other.g0.y, -other.g0.x, 0.0) - vec3(self.g2.x, self.g2.y, self.g2.z) * vec3(other.g1.x), vec2(self.g0.x) * vec2(-other.g0.x, 0.0) + vec2(self.g0.y) * vec2(-other.g0.y, 0.0) + vec2(self.g0.z) * vec2(-other.g0.z, 0.0) + vec2(self.g2.x) * vec2(0.0, other.g0.x) + vec2(self.g2.y) * vec2(0.0, other.g0.y) + vec2(self.g2.z) * vec2(0.0, other.g0.z) + vec2(self.g2.w) * other.g1 * vec2(1.0, -1.0));
}

Circle dipole__anti_wedge_dot__scalar(Dipole self, Scalar other) {
    return Circle(vec4(self.g0.x, self.g0.y, self.g0.z, self.g0.x) * vec4(-other.g0, -other.g0, -other.g0, 0.0) + vec4(self.g2.w) * vec4(0.0, 0.0, 0.0, other.g0), vec3(0.0) - self.g1 * vec3(other.g0), vec3(0.0) - vec3(self.g2.x, self.g2.y, self.g2.z) * vec3(other.g0));
}

MultiVector dipole__anti_wedge_dot__sphere(Dipole self, Sphere other) {
    return MultiVector(vec2(0.0), self.g0 * vec3(other.g1.y) + vec3(self.g1.x) * vec3(0.0, -other.g0.z, other.g0.y) + vec3(self.g1.y) * vec3(other.g0.z, 0.0, -other.g0.x) + vec3(self.g1.z) * vec3(-other.g0.y, other.g0.x, 0.0) - vec3(self.g2.x, self.g2.y, self.g2.z) * vec3(other.g1.x), vec2(self.g0.x) * vec2(-other.g0.x, 0.0) + vec2(self.g0.y) * vec2(-other.g0.y, 0.0) + vec2(self.g0.z) * vec2(-other.g0.z, 0.0) + vec2(self.g2.x) * vec2(0.0, other.g0.x) + vec2(self.g2.y) * vec2(0.0, other.g0.y) + vec2(self.g2.z) * vec2(0.0, other.g0.z) + vec2(self.g2.w) * other.g1 * vec2(-1.0, 1.0), vec3(0.0), vec3(0.0), vec4(0.0), vec4(self.g0.x) * vec4(0.0, -other.g0.z, other.g0.y, 0.0) + vec4(self.g0.y) * vec4(other.g0.z, 0.0, -other.g0.x, 0.0) + vec4(self.g0.z) * vec4(-other.g0.y, other.g0.x, 0.0, 0.0) + vec4(self.g1.x) * vec4(-other.g1.x, 0.0, 0.0, -other.g0.x) + vec4(self.g1.y) * vec4(0.0, -other.g1.x, 0.0, -other.g0.y) + vec4(self.g1.z) * vec4(0.0, 0.0, -other.g1.x, -other.g0.z), vec3(0.0) - self.g0 * vec3(other.g1.y) - vec3(self.g2.x, self.g2.y, self.g2.z) * vec3(other.g1.x) - vec3(self.g2.w) * other.g0, vec3(0.0) - self.g1 * vec3(other.g1.y) + vec3(self.g2.x) * vec3(0.0, other.g0.z, -other.g0.y) + vec3(self.g2.y) * vec3(-other.g0.z, 0.0, other.g0.x) + vec3(self.g2.z) * vec3(other.g0.y, -other.g0.x, 0.0), vec3(0.0), vec2(0.0));
}

DualNum dual_num__anti_wedge_dot__anti_scalar(DualNum self, AntiScalar other) {
    return DualNum(self.g0 * vec2(other.g0));
}

MultiVector dual_num__anti_wedge_dot__circle(DualNum self, Circle other) {
    return MultiVector(vec2(0.0), vec3(0.0), vec2(0.0), vec3(self.g0.x) * vec3(other.g0.x, other.g0.y, other.g0.z), vec3(self.g0.x) * other.g1, vec4(self.g0.x) * vec4(other.g2.x, other.g2.y, other.g2.z, -other.g0.w), vec4(self.g0.y) * other.g0, vec3(self.g0.y) * other.g1, vec3(self.g0.y) * other.g2, vec3(0.0), vec2(0.0));
}

MultiVector dual_num__anti_wedge_dot__dipole(DualNum self, Dipole other) {
    return MultiVector(vec2(0.0), vec3(0.0), vec2(0.0), vec3(self.g0.y) * other.g0, vec3(self.g0.y) * other.g1, vec4(self.g0.y) * other.g2, vec4(self.g0.x) * vec4(-other.g0.x, -other.g0.y, -other.g0.z, other.g2.w), vec3(0.0) - vec3(self.g0.x) * other.g1, vec3(0.0) - vec3(self.g0.x) * vec3(other.g2.x, other.g2.y, other.g2.z), vec3(0.0), vec2(0.0));
}

DualNum dual_num__anti_wedge_dot__dual_num(DualNum self, DualNum other) {
    return DualNum(vec2(self.g0.x) * other.g0.yx * vec2(1.0, -1.0) + vec2(self.g0.y) * other.g0);
}

MultiVector dual_num__anti_wedge_dot__flat_point(DualNum self, FlatPoint other) {
    return MultiVector(vec2(0.0), vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0), vec4(self.g0.y) * other.g0, vec4(self.g0.x) * vec4(0.0, 0.0, 0.0, other.g0.w), vec3(0.0), vec3(0.0) - vec3(self.g0.x) * vec3(other.g0.x, other.g0.y, other.g0.z), vec3(0.0), vec2(0.0));
}

MultiVector dual_num__anti_wedge_dot__flector(DualNum self, Flector other) {
    return MultiVector(vec2(0.0), vec3(self.g0.x) * vec3(other.g1.x, other.g1.y, other.g1.z), vec2(self.g0.x) * vec2(0.0, -other.g1.w), vec3(0.0), vec3(0.0), vec4(self.g0.y) * other.g0, vec4(self.g0.x) * vec4(0.0, 0.0, 0.0, other.g0.w), vec3(0.0), vec3(0.0) - vec3(self.g0.x) * vec3(other.g0.x, other.g0.y, other.g0.z), vec3(self.g0.y) * vec3(other.g1.x, other.g1.y, other.g1.z), vec2(self.g0.y) * vec2(0.0, other.g1.w));
}

MultiVector dual_num__anti_wedge_dot__line(DualNum self, Line other) {
    return MultiVector(vec2(0.0), vec3(0.0), vec2(0.0), vec3(0.0), vec3(self.g0.x) * other.g0, vec4(self.g0.x) * vec4(other.g1.x, other.g1.y, other.g1.z, 0.0), vec4(0.0), vec3(self.g0.y) * other.g0, vec3(self.g0.y) * other.g1, vec3(0.0), vec2(0.0));
}

MultiVector dual_num__anti_wedge_dot__motor(DualNum self, Motor other) {
    return MultiVector(self.g0 * vec2(other.g0.w), vec3(0.0), vec2(0.0), vec3(0.0), vec3(self.g0.x) * vec3(other.g0.x, other.g0.y, other.g0.z), vec4(self.g0.x) * vec4(other.g1.x, other.g1.y, other.g1.z, 0.0), vec4(0.0), vec3(self.g0.y) * vec3(other.g0.x, other.g0.y, other.g0.z), vec3(self.g0.y) * other.g1, vec3(0.0), vec2(0.0));
}

MultiVector dual_num__anti_wedge_dot__multi_vector(DualNum self, MultiVector other) {
    return MultiVector(vec2(self.g0.x) * other.g0.yx * vec2(1.0, -1.0) + vec2(self.g0.y) * other.g0, vec3(self.g0.x) * other.g9 + vec3(self.g0.y) * other.g1, vec2(0.0) - vec2(self.g0.x) * other.g10 + vec2(self.g0.y) * other.g2, vec3(self.g0.x) * vec3(other.g6.x, other.g6.y, other.g6.z) + vec3(self.g0.y) * other.g3, vec3(self.g0.x) * other.g7 + vec3(self.g0.y) * other.g4, vec4(self.g0.x) * vec4(other.g8.x, other.g8.y, other.g8.z, -other.g6.w) + vec4(self.g0.y) * other.g5, vec4(self.g0.x) * vec4(-other.g3.x, -other.g3.y, -other.g3.z, other.g5.w) + vec4(self.g0.y) * other.g6, vec3(0.0) - vec3(self.g0.x) * other.g4 + vec3(self.g0.y) * other.g7, vec3(0.0) - vec3(self.g0.x) * vec3(other.g5.x, other.g5.y, other.g5.z) + vec3(self.g0.y) * other.g8, vec3(0.0) - vec3(self.g0.x) * other.g1 + vec3(self.g0.y) * other.g9, vec2(self.g0.x) * other.g2 + vec2(self.g0.y) * other.g10);
}

MultiVector dual_num__anti_wedge_dot__plane(DualNum self, Plane other) {
    return MultiVector(vec2(0.0), vec3(self.g0.x) * vec3(other.g0.x, other.g0.y, other.g0.z), vec2(self.g0.x) * vec2(0.0, -other.g0.w), vec3(0.0), vec3(0.0), vec4(0.0), vec4(0.0), vec3(0.0), vec3(0.0), vec3(self.g0.y) * vec3(other.g0.x, other.g0.y, other.g0.z), vec2(self.g0.y) * vec2(0.0, other.g0.w));
}

MultiVector dual_num__anti_wedge_dot__round_point(DualNum self, RoundPoint other) {
    return MultiVector(vec2(0.0), vec3(self.g0.y) * other.g0, vec2(self.g0.y) * other.g1, vec3(0.0), vec3(0.0), vec4(0.0), vec4(0.0), vec3(0.0), vec3(0.0), vec3(0.0) - vec3(self.g0.x) * other.g0, vec2(self.g0.x) * other.g1);
}

DualNum dual_num__anti_wedge_dot__scalar(DualNum self, Scalar other) {
    return DualNum(self.g0.yx * vec2(other.g0));
}

MultiVector dual_num__anti_wedge_dot__sphere(DualNum self, Sphere other) {
    return MultiVector(vec2(0.0), vec3(self.g0.x) * other.g0, vec2(0.0) - vec2(self.g0.x) * other.g1, vec3(0.0), vec3(0.0), vec4(0.0), vec4(0.0), vec3(0.0), vec3(0.0), vec3(self.g0.y) * other.g0, vec2(self.g0.y) * other.g1);
}

FlatPoint flat_point__anti_wedge_dot__anti_scalar(FlatPoint self, AntiScalar other) {
    return FlatPoint(self.g0 * vec4(other.g0));
}

MultiVector flat_point__anti_wedge_dot__circle(FlatPoint self, Circle other) {
    return MultiVector(vec2(self.g0.x) * vec2(-other.g0.x, 0.0) + vec2(self.g0.y) * vec2(-other.g0.y, 0.0) + vec2(self.g0.z) * vec2(-other.g0.z, 0.0) + vec2(self.g0.w) * vec2(-other.g0.w, 0.0), vec3(0.0), vec2(0.0), vec3(self.g0.w) * vec3(other.g0.x, other.g0.y, other.g0.z), vec3(self.g0.x) * vec3(0.0, -other.g0.z, other.g0.y) + vec3(self.g0.y) * vec3(other.g0.z, 0.0, -other.g0.x) + vec3(self.g0.z) * vec3(-other.g0.y, other.g0.x, 0.0), vec4(self.g0.x) * vec4(-other.g0.w, -other.g1.z, other.g1.y, other.g0.x) + vec4(self.g0.y) * vec4(other.g1.z, -other.g0.w, -other.g1.x, other.g0.y) + vec4(self.g0.z) * vec4(-other.g1.y, other.g1.x, -other.g0.w, other.g0.z) + vec4(self.g0.w) * vec4(-other.g2.x, -other.g2.y, -other.g2.z, 0.0), vec4(0.0), vec3(0.0), vec3(0.0), vec3(self.g0.x) * vec3(0.0, other.g0.z, -other.g0.y) + vec3(self.g0.y) * vec3(-other.g0.z, 0.0, other.g0.x) + vec3(self.g0.z) * vec3(other.g0.y, -other.g0.x, 0.0) + vec3(self.g0.w) * other.g1, vec2(self.g0.x) * vec2(0.0, -other.g1.x) + vec2(self.g0.y) * vec2(0.0, -other.g1.y) + vec2(self.g0.z) * vec2(0.0, -other.g1.z));
}

MultiVector flat_point__anti_wedge_dot__dipole(FlatPoint self, Dipole other) {
    return MultiVector(vec2(self.g0.x) * vec2(0.0, other.g0.x) + vec2(self.g0.y) * vec2(0.0, other.g0.y) + vec2(self.g0.z) * vec2(0.0, other.g0.z) + vec2(self.g0.w) * vec2(0.0, -other.g2.w), vec3(self.g0.x) * vec3(0.0, other.g0.z, -other.g0.y) + vec3(self.g0.y) * vec3(-other.g0.z, 0.0, other.g0.x) + vec3(self.g0.z) * vec3(other.g0.y, -other.g0.x, 0.0) + vec3(self.g0.w) * other.g1, vec2(self.g0.x) * vec2(0.0, other.g1.x) + vec2(self.g0.y) * vec2(0.0, other.g1.y) + vec2(self.g0.z) * vec2(0.0, other.g1.z), vec3(0.0), vec3(0.0), vec4(0.0), vec4(self.g0.x) * vec4(0.0, 0.0, 0.0, other.g0.x) + vec4(self.g0.y) * vec4(0.0, 0.0, 0.0, other.g0.y) + self.g0.wwwz * vec4(-other.g0.x, -other.g0.y, -other.g0.z, other.g0.z), vec3(self.g0.x) * vec3(0.0, other.g0.z, -other.g0.y) + vec3(self.g0.y) * vec3(-other.g0.z, 0.0, other.g0.x) + vec3(self.g0.z) * vec3(other.g0.y, -other.g0.x, 0.0), vec3(self.g0.x) * vec3(-other.g2.w, other.g1.z, -other.g1.y) + vec3(self.g0.y) * vec3(-other.g1.z, -other.g2.w, other.g1.x) + vec3(self.g0.z) * vec3(other.g1.y, -other.g1.x, -other.g2.w) + vec3(self.g0.w) * vec3(other.g2.x, other.g2.y, other.g2.z), vec3(0.0), vec2(0.0));
}

MultiVector flat_point__anti_wedge_dot__dual_num(FlatPoint self, DualNum other) {
    return MultiVector(vec2(0.0), vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0), self.g0 * vec4(other.g0.y), vec4(self.g0.w) * vec4(0.0, 0.0, 0.0, other.g0.x), vec3(0.0), vec3(0.0) - vec3(self.g0.x, self.g0.y, self.g0.z) * vec3(other.g0.x), vec3(0.0), vec2(0.0));
}

Motor flat_point__anti_wedge_dot__flat_point(FlatPoint self, FlatPoint other) {
    return Motor(vec4(self.g0.w) * vec4(0.0, 0.0, 0.0, -other.g0.w), vec3(0.0) - vec3(self.g0.x, self.g0.y, self.g0.z) * vec3(other.g0.w) + vec3(self.g0.w) * vec3(other.g0.x, other.g0.y, other.g0.z));
}

MultiVector flat_point__anti_wedge_dot__flector(FlatPoint self, Flector other) {
    return MultiVector(vec2(self.g0.w) * vec2(0.0, -other.g0.w), vec3(0.0), vec2(self.g0.x) * vec2(0.0, other.g1.x) + vec2(self.g0.y) * vec2(0.0, other.g1.y) + vec2(self.g0.z) * vec2(0.0, other.g1.z) + vec2(self.g0.w) * vec2(0.0, other.g1.w), vec3(0.0), vec3(0.0), vec4(0.0), vec4(0.0), vec3(0.0) - vec3(self.g0.w) * vec3(other.g1.x, other.g1.y, other.g1.z), vec3(self.g0.x) * vec3(-other.g0.w, other.g1.z, -other.g1.y) + vec3(self.g0.y) * vec3(-other.g1.z, -other.g0.w, other.g1.x) + vec3(self.g0.z) * vec3(other.g1.y, -other.g1.x, -other.g0.w) + vec3(self.g0.w) * vec3(other.g0.x, other.g0.y, other.g0.z), vec3(0.0), vec2(0.0));
}

Flector flat_point__anti_wedge_dot__line(FlatPoint self, Line other) {
    return Flector(vec4(self.g0.x) * vec4(0.0, -other.g0.z, other.g0.y, 0.0) + vec4(self.g0.y) * vec4(other.g0.z, 0.0, -other.g0.x, 0.0) + vec4(self.g0.z) * vec4(-other.g0.y, other.g0.x, 0.0, 0.0) + vec4(self.g0.w) * vec4(-other.g1.x, -other.g1.y, -other.g1.z, 0.0), vec4(self.g0.x) * vec4(0.0, 0.0, 0.0, -other.g0.x) + vec4(self.g0.y) * vec4(0.0, 0.0, 0.0, -other.g0.y) + self.g0.wwwz * vec4(other.g0.x, other.g0.y, other.g0.z, -other.g0.z));
}

Flector flat_point__anti_wedge_dot__motor(FlatPoint self, Motor other) {
    return Flector(vec4(self.g0.x) * vec4(other.g0.w, -other.g0.z, other.g0.y, 0.0) + vec4(self.g0.y) * vec4(other.g0.z, other.g0.w, -other.g0.x, 0.0) + vec4(self.g0.z) * vec4(-other.g0.y, other.g0.x, other.g0.w, 0.0) + vec4(self.g0.w) * vec4(-other.g1.x, -other.g1.y, -other.g1.z, other.g0.w), vec4(self.g0.x) * vec4(0.0, 0.0, 0.0, -other.g0.x) + vec4(self.g0.y) * vec4(0.0, 0.0, 0.0, -other.g0.y) + self.g0.wwwz * other.g0.xyzz * vec4(1.0, 1.0, 1.0, -1.0));
}

MultiVector flat_point__anti_wedge_dot__multi_vector(FlatPoint self, MultiVector other) {
    return MultiVector(vec2(self.g0.x) * vec2(-other.g6.x, other.g3.x) + vec2(self.g0.y) * vec2(-other.g6.y, other.g3.y) + vec2(self.g0.z) * vec2(-other.g6.z, other.g3.z) - vec2(self.g0.w) * vec2(other.g6.w, other.g5.w), vec3(self.g0.x) * vec3(-other.g10.x, other.g3.z, -other.g3.y) + vec3(self.g0.y) * vec3(-other.g3.z, -other.g10.x, other.g3.x) + vec3(self.g0.z) * vec3(other.g3.y, -other.g3.x, -other.g10.x) + vec3(self.g0.w) * other.g4, vec2(self.g0.x) * vec2(0.0, other.g9.x) + vec2(self.g0.x) * vec2(0.0, other.g4.x) + vec2(self.g0.y) * vec2(0.0, other.g9.y) + vec2(self.g0.y) * vec2(0.0, other.g4.y) + vec2(self.g0.z) * vec2(0.0, other.g9.z) + vec2(self.g0.z) * vec2(0.0, other.g4.z) + vec2(self.g0.w) * other.g10 * vec2(-1.0, 1.0), vec3(self.g0.w) * vec3(other.g6.x, other.g6.y, other.g6.z), vec3(self.g0.x) * vec3(other.g2.x, -other.g6.z, other.g6.y) + vec3(self.g0.y) * vec3(other.g6.z, other.g2.x, -other.g6.x) + vec3(self.g0.z) * vec3(-other.g6.y, other.g6.x, other.g2.x) - vec3(self.g0.w) * other.g1, vec4(self.g0.x) * vec4(-other.g6.w, -other.g7.z, other.g7.y, other.g6.x) + vec4(self.g0.x) * vec4(other.g0.y, other.g1.z, -other.g1.y, 0.0) + vec4(self.g0.y) * vec4(other.g7.z, -other.g6.w, -other.g7.x, other.g6.y) + vec4(self.g0.y) * vec4(-other.g1.z, other.g0.y, other.g1.x, 0.0) + vec4(self.g0.z) * vec4(-other.g7.y, other.g7.x, -other.g6.w, other.g6.z) + vec4(self.g0.z) * vec4(other.g1.y, -other.g1.x, other.g0.y, 0.0) + vec4(self.g0.w) * vec4(-other.g8.x, -other.g8.y, -other.g8.z, other.g0.y), vec4(self.g0.x) * vec4(0.0, 0.0, 0.0, other.g3.x) + vec4(self.g0.y) * vec4(0.0, 0.0, 0.0, other.g3.y) + vec4(self.g0.z) * vec4(0.0, 0.0, 0.0, other.g3.z) + vec4(self.g0.w) * vec4(-other.g3.x, -other.g3.y, -other.g3.z, other.g0.x), vec3(self.g0.x) * vec3(-other.g10.x, other.g3.z, -other.g3.y) + vec3(self.g0.y) * vec3(-other.g3.z, -other.g10.x, other.g3.x) + vec3(self.g0.z) * vec3(other.g3.y, -other.g3.x, -other.g10.x) - vec3(self.g0.w) * other.g9, vec3(self.g0.x) * vec3(-other.g5.w, other.g9.z, -other.g9.y) + vec3(self.g0.x) * vec3(-other.g0.x, other.g4.z, -other.g4.y) + vec3(self.g0.y) * vec3(-other.g9.z, -other.g5.w, other.g9.x) + vec3(self.g0.y) * vec3(-other.g4.z, -other.g0.x, other.g4.x) + vec3(self.g0.z) * vec3(other.g9.y, -other.g9.x, -other.g5.w) + vec3(self.g0.z) * vec3(other.g4.y, -other.g4.x, -other.g0.x) + vec3(self.g0.w) * vec3(other.g5.x, other.g5.y, other.g5.z), vec3(self.g0.x) * vec3(-other.g2.x, other.g6.z, -other.g6.y) + vec3(self.g0.y) * vec3(-other.g6.z, -other.g2.x, other.g6.x) + vec3(self.g0.z) * vec3(other.g6.y, -other.g6.x, -other.g2.x) + vec3(self.g0.w) * other.g7, vec2(self.g0.x) * vec2(0.0, -other.g7.x) + vec2(self.g0.x) * vec2(0.0, other.g1.x) + vec2(self.g0.y) * vec2(0.0, -other.g7.y) + vec2(self.g0.y) * vec2(0.0, other.g1.y) + vec2(self.g0.z) * vec2(0.0, -other.g7.z) + vec2(self.g0.z) * vec2(0.0, other.g1.z) + vec2(self.g0.w) * other.g2 * vec2(1.0, -1.0));
}

MultiVector flat_point__anti_wedge_dot__plane(FlatPoint self, Plane other) {
    return MultiVector(vec2(0.0), vec3(0.0), vec2(self.g0.x) * vec2(0.0, other.g0.x) + vec2(self.g0.y) * vec2(0.0, other.g0.y) + vec2(self.g0.z) * vec2(0.0, other.g0.z) + vec2(self.g0.w) * vec2(0.0, other.g0.w), vec3(0.0), vec3(0.0), vec4(0.0), vec4(0.0), vec3(0.0) - vec3(self.g0.w) * vec3(other.g0.x, other.g0.y, other.g0.z), vec3(self.g0.x) * vec3(0.0, other.g0.z, -other.g0.y) + vec3(self.g0.y) * vec3(-other.g0.z, 0.0, other.g0.x) + vec3(self.g0.z) * vec3(other.g0.y, -other.g0.x, 0.0), vec3(0.0), vec2(0.0));
}

MultiVector flat_point__anti_wedge_dot__round_point(FlatPoint self, RoundPoint other) {
    return MultiVector(vec2(0.0), vec3(0.0), vec2(0.0), vec3(0.0), vec3(self.g0.x, self.g0.y, self.g0.z) * vec3(other.g1.x) - vec3(self.g0.w) * other.g0, vec4(self.g0.x) * vec4(0.0, other.g0.z, -other.g0.y, 0.0) + vec4(self.g0.y) * vec4(-other.g0.z, 0.0, other.g0.x, 0.0) + vec4(self.g0.z) * vec4(other.g0.y, -other.g0.x, 0.0, 0.0), vec4(0.0), vec3(0.0), vec3(0.0), vec3(0.0) - vec3(self.g0.x, self.g0.y, self.g0.z) * vec3(other.g1.x), vec2(self.g0.x) * vec2(0.0, other.g0.x) + vec2(self.g0.y) * vec2(0.0, other.g0.y) + vec2(self.g0.z) * vec2(0.0, other.g0.z) + vec2(self.g0.w) * other.g1 * vec2(1.0, -1.0));
}

Circle flat_point__anti_wedge_dot__scalar(FlatPoint self, Scalar other) {
    return Circle(vec4(self.g0.w) * vec4(0.0, 0.0, 0.0, other.g0), vec3(0.0), vec3(0.0) - vec3(self.g0.x, self.g0.y, self.g0.z) * vec3(other.g0));
}

MultiVector flat_point__anti_wedge_dot__sphere(FlatPoint self, Sphere other) {
    return MultiVector(vec2(0.0), vec3(0.0) - vec3(self.g0.x, self.g0.y, self.g0.z) * vec3(other.g1.x), vec2(self.g0.x) * vec2(0.0, other.g0.x) + vec2(self.g0.y) * vec2(0.0, other.g0.y) + vec2(self.g0.z) * vec2(0.0, other.g0.z) + vec2(self.g0.w) * other.g1 * vec2(-1.0, 1.0), vec3(0.0), vec3(0.0), vec4(0.0), vec4(0.0), vec3(0.0) - vec3(self.g0.x, self.g0.y, self.g0.z) * vec3(other.g1.x) - vec3(self.g0.w) * other.g0, vec3(self.g0.x) * vec3(0.0, other.g0.z, -other.g0.y) + vec3(self.g0.y) * vec3(-other.g0.z, 0.0, other.g0.x) + vec3(self.g0.z) * vec3(other.g0.y, -other.g0.x, 0.0), vec3(0.0), vec2(0.0));
}

Flector flector__anti_wedge_dot__anti_scalar(Flector self, AntiScalar other) {
    return Flector(self.g0 * vec4(other.g0), self.g1 * vec4(other.g0));
}

MultiVector flector__anti_wedge_dot__circle(Flector self, Circle other) {
    return MultiVector(vec2(self.g0.x) * vec2(-other.g0.x, 0.0) + vec2(self.g0.y) * vec2(-other.g0.y, 0.0) + vec2(self.g0.z) * vec2(-other.g0.z, 0.0) + vec2(self.g0.w) * vec2(-other.g0.w, 0.0), vec3(0.0), vec2(0.0), vec3(self.g0.w) * vec3(other.g0.x, other.g0.y, other.g0.z) + vec3(self.g1.x) * vec3(0.0, -other.g0.z, other.g0.y) + vec3(self.g1.y) * vec3(other.g0.z, 0.0, -other.g0.x) + vec3(self.g1.z) * vec3(-other.g0.y, other.g0.x, 0.0), vec3(self.g0.x) * vec3(0.0, -other.g0.z, other.g0.y) + vec3(self.g0.y) * vec3(other.g0.z, 0.0, -other.g0.x) + vec3(self.g0.z) * vec3(-other.g0.y, other.g0.x, 0.0) - vec3(self.g1.x, self.g1.y, self.g1.z) * vec3(other.g0.w) + vec3(self.g1.w) * vec3(other.g0.x, other.g0.y, other.g0.z), vec4(self.g0.x) * vec4(-other.g0.w, -other.g1.z, other.g1.y, other.g0.x) + vec4(self.g0.y) * vec4(other.g1.z, -other.g0.w, -other.g1.x, other.g0.y) + vec4(self.g0.z) * vec4(-other.g1.y, other.g1.x, -other.g0.w, other.g0.z) + vec4(self.g0.w) * vec4(-other.g2.x, -other.g2.y, -other.g2.z, 0.0) + vec4(self.g1.x) * vec4(0.0, other.g2.z, -other.g2.y, -other.g1.x) + vec4(self.g1.y) * vec4(-other.g2.z, 0.0, other.g2.x, -other.g1.y) + vec4(self.g1.z) * vec4(other.g2.y, -other.g2.x, 0.0, -other.g1.z) + vec4(self.g1.w) * vec4(other.g1.x, other.g1.y, other.g1.z, 0.0), vec4(0.0), vec3(0.0), vec3(0.0), vec3(self.g0.x) * vec3(0.0, other.g0.z, -other.g0.y) + vec3(self.g0.y) * vec3(-other.g0.z, 0.0, other.g0.x) + vec3(self.g0.z) * vec3(other.g0.y, -other.g0.x, 0.0) + vec3(self.g0.w) * other.g1 + vec3(self.g1.x) * vec3(0.0, -other.g1.z, other.g1.y) + vec3(self.g1.y) * vec3(other.g1.z, 0.0, -other.g1.x) + vec3(self.g1.z) * vec3(-other.g1.y, other.g1.x, 0.0) - vec3(self.g1.w) * vec3(other.g0.x, other.g0.y, other.g0.z), vec2(self.g0.x) * vec2(0.0, -other.g1.x) + vec2(self.g0.y) * vec2(0.0, -other.g1.y) + vec2(self.g0.z) * vec2(0.0, -other.g1.z) + vec2(self.g1.x) * vec2(-other.g0.x, other.g2.x) + vec2(self.g1.y) * vec2(-other.g0.y, other.g2.y) + vec2(self.g1.z) * vec2(-other.g0.z, other.g2.z) + vec2(self.g1.w) * vec2(0.0, -other.g0.w));
}

MultiVector flector__anti_wedge_dot__dipole(Flector self, Dipole other) {
    return MultiVector(vec2(self.g0.x) * vec2(0.0, other.g0.x) + vec2(self.g0.y) * vec2(0.0, other.g0.y) + vec2(self.g0.z) * vec2(0.0, other.g0.z) + vec2(self.g0.w) * vec2(0.0, -other.g2.w), vec3(self.g0.x) * vec3(0.0, other.g0.z, -other.g0.y) + vec3(self.g0.y) * vec3(-other.g0.z, 0.0, other.g0.x) + vec3(self.g0.z) * vec3(other.g0.y, -other.g0.x, 0.0) + vec3(self.g0.w) * other.g1 + vec3(self.g1.x) * vec3(0.0, -other.g1.z, other.g1.y) + vec3(self.g1.y) * vec3(other.g1.z, 0.0, -other.g1.x) + vec3(self.g1.z) * vec3(-other.g1.y, other.g1.x, 0.0) - vec3(self.g1.w) * other.g0, vec2(self.g0.x) * vec2(0.0, other.g1.x) + vec2(self.g0.y) * vec2(0.0, other.g1.y) + vec2(self.g0.z) * vec2(0.0, other.g1.z) + vec2(self.g1.x) * vec2(other.g0.x, -other.g2.x) + vec2(self.g1.y) * vec2(other.g0.y, -other.g2.y) + vec2(self.g1.z) * vec2(other.g0.z, -other.g2.z) + vec2(self.g1.w) * vec2(0.0, -other.g2.w), vec3(0.0), vec3(0.0), vec4(0.0), vec4(self.g0.x) * vec4(0.0, 0.0, 0.0, other.g0.x) + vec4(self.g0.y) * vec4(0.0, 0.0, 0.0, other.g0.y) + self.g0.wwwz * vec4(-other.g0.x, -other.g0.y, -other.g0.z, other.g0.z) + vec4(self.g1.x) * vec4(0.0, other.g0.z, -other.g0.y, -other.g1.x) + vec4(self.g1.y) * vec4(-other.g0.z, 0.0, other.g0.x, -other.g1.y) + vec4(self.g1.z) * vec4(other.g0.y, -other.g0.x, 0.0, -other.g1.z), vec3(self.g0.x) * vec3(0.0, other.g0.z, -other.g0.y) + vec3(self.g0.y) * vec3(-other.g0.z, 0.0, other.g0.x) + vec3(self.g0.z) * vec3(other.g0.y, -other.g0.x, 0.0) - vec3(self.g1.x, self.g1.y, self.g1.z) * vec3(other.g2.w) - vec3(self.g1.w) * other.g0, vec3(self.g0.x) * vec3(-other.g2.w, other.g1.z, -other.g1.y) + vec3(self.g0.y) * vec3(-other.g1.z, -other.g2.w, other.g1.x) + vec3(self.g0.z) * vec3(other.g1.y, -other.g1.x, -other.g2.w) + vec3(self.g0.w) * vec3(other.g2.x, other.g2.y, other.g2.z) + vec3(self.g1.x) * vec3(0.0, -other.g2.z, other.g2.y) + vec3(self.g1.y) * vec3(other.g2.z, 0.0, -other.g2.x) + vec3(self.g1.z) * vec3(-other.g2.y, other.g2.x, 0.0) - vec3(self.g1.w) * other.g1, vec3(0.0), vec2(0.0));
}

MultiVector flector__anti_wedge_dot__dual_num(Flector self, DualNum other) {
    return MultiVector(vec2(0.0), vec3(self.g1.x, self.g1.y, self.g1.z) * vec3(other.g0.x), vec2(self.g1.w) * vec2(0.0, -other.g0.x), vec3(0.0), vec3(0.0), self.g0 * vec4(other.g0.y), vec4(self.g0.w) * vec4(0.0, 0.0, 0.0, other.g0.x), vec3(0.0), vec3(0.0) - vec3(self.g0.x, self.g0.y, self.g0.z) * vec3(other.g0.x), vec3(self.g1.x, self.g1.y, self.g1.z) * vec3(other.g0.y), vec2(self.g1.w) * vec2(0.0, other.g0.y));
}

MultiVector flector__anti_wedge_dot__flat_point(Flector self, FlatPoint other) {
    return MultiVector(vec2(self.g0.w) * vec2(0.0, -other.g0.w), vec3(0.0), vec2(self.g1.x) * vec2(0.0, -other.g0.x) + vec2(self.g1.y) * vec2(0.0, -other.g0.y) + vec2(self.g1.z) * vec2(0.0, -other.g0.z) + vec2(self.g1.w) * vec2(0.0, -other.g0.w), vec3(0.0), vec3(0.0), vec4(0.0), vec4(0.0), vec3(0.0) - vec3(self.g1.x, self.g1.y, self.g1.z) * vec3(other.g0.w), vec3(0.0) - vec3(self.g0.x, self.g0.y, self.g0.z) * vec3(other.g0.w) + vec3(self.g0.w) * vec3(other.g0.x, other.g0.y, other.g0.z) + vec3(self.g1.x) * vec3(0.0, -other.g0.z, other.g0.y) + vec3(self.g1.y) * vec3(other.g0.z, 0.0, -other.g0.x) + vec3(self.g1.z) * vec3(-other.g0.y, other.g0.x, 0.0), vec3(0.0), vec2(0.0));
}

MultiVector flector__anti_wedge_dot__flector(Flector self, Flector other) {
    return MultiVector(vec2(self.g0.w) * vec2(0.0, -other.g0.w) + vec2(self.g1.x) * vec2(0.0, other.g1.x) + vec2(self.g1.y) * vec2(0.0, other.g1.y) + vec2(self.g1.z) * vec2(0.0, other.g1.z), vec3(0.0), vec2(self.g0.x) * vec2(0.0, other.g1.x) + vec2(self.g0.y) * vec2(0.0, other.g1.y) + vec2(self.g0.z) * vec2(0.0, other.g1.z) + vec2(self.g0.w) * vec2(0.0, other.g1.w) + vec2(self.g1.x) * vec2(0.0, -other.g0.x) + vec2(self.g1.y) * vec2(0.0, -other.g0.y) + vec2(self.g1.z) * vec2(0.0, -other.g0.z) + vec2(self.g1.w) * vec2(0.0, -other.g0.w), vec3(0.0), vec3(0.0), vec4(0.0), vec4(0.0), vec3(0.0) - vec3(self.g0.w) * vec3(other.g1.x, other.g1.y, other.g1.z) + vec3(self.g1.x) * vec3(-other.g0.w, other.g1.z, -other.g1.y) + vec3(self.g1.y) * vec3(-other.g1.z, -other.g0.w, other.g1.x) + vec3(self.g1.z) * vec3(other.g1.y, -other.g1.x, -other.g0.w), vec3(self.g0.x) * vec3(-other.g0.w, other.g1.z, -other.g1.y) + vec3(self.g0.y) * vec3(-other.g1.z, -other.g0.w, other.g1.x) + vec3(self.g0.z) * vec3(other.g1.y, -other.g1.x, -other.g0.w) + vec3(self.g0.w) * vec3(other.g0.x, other.g0.y, other.g0.z) + vec3(self.g1.x) * vec3(other.g1.w, -other.g0.z, other.g0.y) + vec3(self.g1.y) * vec3(other.g0.z, other.g1.w, -other.g0.x) + vec3(self.g1.z) * vec3(-other.g0.y, other.g0.x, other.g1.w) - vec3(self.g1.w) * vec3(other.g1.x, other.g1.y, other.g1.z), vec3(0.0), vec2(0.0));
}

Flector flector__anti_wedge_dot__line(Flector self, Line other) {
    return Flector(vec4(self.g0.x) * vec4(0.0, -other.g0.z, other.g0.y, 0.0) + vec4(self.g0.y) * vec4(other.g0.z, 0.0, -other.g0.x, 0.0) + vec4(self.g0.z) * vec4(-other.g0.y, other.g0.x, 0.0, 0.0) + vec4(self.g0.w) * vec4(-other.g1.x, -other.g1.y, -other.g1.z, 0.0) + vec4(self.g1.x) * vec4(0.0, other.g1.z, -other.g1.y, -other.g0.x) + vec4(self.g1.y) * vec4(-other.g1.z, 0.0, other.g1.x, -other.g0.y) + vec4(self.g1.z) * vec4(other.g1.y, -other.g1.x, 0.0, -other.g0.z) + vec4(self.g1.w) * vec4(other.g0.x, other.g0.y, other.g0.z, 0.0), vec4(self.g0.x) * vec4(0.0, 0.0, 0.0, -other.g0.x) + vec4(self.g0.y) * vec4(0.0, 0.0, 0.0, -other.g0.y) + self.g0.wwwz * vec4(other.g0.x, other.g0.y, other.g0.z, -other.g0.z) + vec4(self.g1.x) * vec4(0.0, -other.g0.z, other.g0.y, other.g1.x) + vec4(self.g1.y) * vec4(other.g0.z, 0.0, -other.g0.x, other.g1.y) + vec4(self.g1.z) * vec4(-other.g0.y, other.g0.x, 0.0, other.g1.z));
}

Flector flector__anti_wedge_dot__motor(Flector self, Motor other) {
    return Flector(vec4(self.g0.x) * vec4(other.g0.w, -other.g0.z, other.g0.y, 0.0) + vec4(self.g0.y) * vec4(other.g0.z, other.g0.w, -other.g0.x, 0.0) + vec4(self.g0.z) * vec4(-other.g0.y, other.g0.x, other.g0.w, 0.0) + vec4(self.g0.w) * vec4(-other.g1.x, -other.g1.y, -other.g1.z, other.g0.w) + vec4(self.g1.x) * vec4(0.0, other.g1.z, -other.g1.y, -other.g0.x) + vec4(self.g1.y) * vec4(-other.g1.z, 0.0, other.g1.x, -other.g0.y) + vec4(self.g1.z) * vec4(other.g1.y, -other.g1.x, 0.0, -other.g0.z) + vec4(self.g1.w) * vec4(other.g0.x, other.g0.y, other.g0.z, 0.0), vec4(self.g0.x) * vec4(0.0, 0.0, 0.0, -other.g0.x) + vec4(self.g0.y) * vec4(0.0, 0.0, 0.0, -other.g0.y) + self.g0.wwwz * other.g0.xyzz * vec4(1.0, 1.0, 1.0, -1.0) + vec4(self.g1.x) * vec4(other.g0.w, -other.g0.z, other.g0.y, other.g1.x) + vec4(self.g1.y) * vec4(other.g0.z, other.g0.w, -other.g0.x, other.g1.y) + vec4(self.g1.z) * vec4(-other.g0.y, other.g0.x, other.g0.w, other.g1.z) + vec4(self.g1.w) * vec4(0.0, 0.0, 0.0, other.g0.w));
}

MultiVector flector__anti_wedge_dot__multi_vector(Flector self, MultiVector other) {
    return MultiVector(vec2(self.g0.x) * vec2(-other.g6.x, other.g3.x) + vec2(self.g0.y) * vec2(-other.g6.y, other.g3.y) + vec2(self.g0.z) * vec2(-other.g6.z, other.g3.z) - vec2(self.g0.w) * vec2(other.g6.w, other.g5.w) + vec2(self.g1.x) * vec2(other.g1.x, other.g9.x) + vec2(self.g1.y) * vec2(other.g1.y, other.g9.y) + vec2(self.g1.z) * vec2(other.g1.z, other.g9.z) + vec2(self.g1.w) * vec2(other.g2.x, -other.g10.x), vec3(self.g0.x) * vec3(-other.g10.x, other.g3.z, -other.g3.y) + vec3(self.g0.y) * vec3(-other.g3.z, -other.g10.x, other.g3.x) + vec3(self.g0.z) * vec3(other.g3.y, -other.g3.x, -other.g10.x) + vec3(self.g0.w) * other.g4 + vec3(self.g1.x) * vec3(other.g0.x, -other.g4.z, other.g4.y) + vec3(self.g1.y) * vec3(other.g4.z, other.g0.x, -other.g4.x) + vec3(self.g1.z) * vec3(-other.g4.y, other.g4.x, other.g0.x) - vec3(self.g1.w) * other.g3, vec2(self.g0.x) * vec2(0.0, other.g9.x) + vec2(self.g0.x) * vec2(0.0, other.g4.x) + vec2(self.g0.y) * vec2(0.0, other.g9.y) + vec2(self.g0.y) * vec2(0.0, other.g4.y) + vec2(self.g0.z) * vec2(0.0, other.g9.z) + vec2(self.g0.z) * vec2(0.0, other.g4.z) + vec2(self.g0.w) * other.g10 * vec2(-1.0, 1.0) + vec2(self.g1.x) * vec2(other.g3.x, -other.g5.x) + vec2(self.g1.y) * vec2(other.g3.y, -other.g5.y) + vec2(self.g1.z) * vec2(other.g3.z, -other.g5.z) + vec2(self.g1.w) * vec2(0.0, -other.g5.w) + vec2(self.g1.w) * vec2(0.0, -other.g0.x), vec3(self.g0.w) * vec3(other.g6.x, other.g6.y, other.g6.z) + vec3(self.g1.x) * vec3(other.g2.x, -other.g6.z, other.g6.y) + vec3(self.g1.y) * vec3(other.g6.z, other.g2.x, -other.g6.x) + vec3(self.g1.z) * vec3(-other.g6.y, other.g6.x, other.g2.x), vec3(self.g0.x) * vec3(other.g2.x, -other.g6.z, other.g6.y) + vec3(self.g0.y) * vec3(other.g6.z, other.g2.x, -other.g6.x) + vec3(self.g0.z) * vec3(-other.g6.y, other.g6.x, other.g2.x) - vec3(self.g0.w) * other.g1 + vec3(self.g1.x) * vec3(-other.g6.w, other.g1.z, -other.g1.y) + vec3(self.g1.y) * vec3(-other.g1.z, -other.g6.w, other.g1.x) + vec3(self.g1.z) * vec3(other.g1.y, -other.g1.x, -other.g6.w) + vec3(self.g1.w) * vec3(other.g6.x, other.g6.y, other.g6.z), vec4(self.g0.x) * vec4(-other.g6.w, -other.g7.z, other.g7.y, other.g6.x) + vec4(self.g0.x) * vec4(other.g0.y, other.g1.z, -other.g1.y, 0.0) + vec4(self.g0.y) * vec4(other.g7.z, -other.g6.w, -other.g7.x, other.g6.y) + vec4(self.g0.y) * vec4(-other.g1.z, other.g0.y, other.g1.x, 0.0) + vec4(self.g0.z) * vec4(-other.g7.y, other.g7.x, -other.g6.w, other.g6.z) + vec4(self.g0.z) * vec4(other.g1.y, -other.g1.x, other.g0.y, 0.0) + vec4(self.g0.w) * vec4(-other.g8.x, -other.g8.y, -other.g8.z, other.g0.y) + vec4(self.g1.x) * vec4(-other.g2.y, other.g8.z, -other.g8.y, -other.g7.x) + vec4(self.g1.y) * vec4(-other.g8.z, -other.g2.y, other.g8.x, -other.g7.y) + vec4(self.g1.z) * vec4(other.g8.y, -other.g8.x, -other.g2.y, -other.g7.z) + vec4(self.g1.w) * vec4(other.g7.x, other.g7.y, other.g7.z, -other.g2.x) + vec4(self.g1.w) * vec4(-other.g1.x, -other.g1.y, -other.g1.z, 0.0), vec4(self.g0.x) * vec4(0.0, 0.0, 0.0, other.g3.x) + vec4(self.g0.y) * vec4(0.0, 0.0, 0.0, other.g3.y) + vec4(self.g0.z) * vec4(0.0, 0.0, 0.0, other.g3.z) + vec4(self.g0.w) * vec4(-other.g3.x, -other.g3.y, -other.g3.z, other.g0.x) + vec4(self.g1.x) * vec4(-other.g10.x, other.g3.z, -other.g3.y, -other.g4.x) + vec4(self.g1.y) * vec4(-other.g3.z, -other.g10.x, other.g3.x, -other.g4.y) + vec4(self.g1.z) * vec4(other.g3.y, -other.g3.x, -other.g10.x, -other.g4.z) + vec4(self.g1.w) * vec4(0.0, 0.0, 0.0, -other.g10.x), vec3(self.g0.x) * vec3(-other.g10.x, other.g3.z, -other.g3.y) + vec3(self.g0.y) * vec3(-other.g3.z, -other.g10.x, other.g3.x) + vec3(self.g0.z) * vec3(other.g3.y, -other.g3.x, -other.g10.x) - vec3(self.g0.w) * other.g9 + vec3(self.g1.x) * vec3(-other.g5.w, other.g9.z, -other.g9.y) + vec3(self.g1.y) * vec3(-other.g9.z, -other.g5.w, other.g9.x) + vec3(self.g1.z) * vec3(other.g9.y, -other.g9.x, -other.g5.w) - vec3(self.g1.w) * other.g3, vec3(self.g0.x) * vec3(-other.g5.w, other.g9.z, -other.g9.y) + vec3(self.g0.x) * vec3(-other.g0.x, other.g4.z, -other.g4.y) + vec3(self.g0.y) * vec3(-other.g9.z, -other.g5.w, other.g9.x) + vec3(self.g0.y) * vec3(-other.g4.z, -other.g0.x, other.g4.x) + vec3(self.g0.z) * vec3(other.g9.y, -other.g9.x, -other.g5.w) + vec3(self.g0.z) * vec3(other.g4.y, -other.g4.x, -other.g0.x) + vec3(self.g0.w) * vec3(other.g5.x, other.g5.y, other.g5.z) + vec3(self.g1.x) * vec3(other.g10.y, -other.g5.z, other.g5.y) + vec3(self.g1.y) * vec3(other.g5.z, other.g10.y, -other.g5.x) + vec3(self.g1.z) * vec3(-other.g5.y, other.g5.x, other.g10.y) - vec3(self.g1.w) * other.g9 - vec3(self.g1.w) * other.g4, vec3(self.g0.x) * vec3(-other.g2.x, other.g6.z, -other.g6.y) + vec3(self.g0.y) * vec3(-other.g6.z, -other.g2.x, other.g6.x) + vec3(self.g0.z) * vec3(other.g6.y, -other.g6.x, -other.g2.x) + vec3(self.g0.w) * other.g7 + vec3(self.g1.x) * vec3(other.g0.y, -other.g7.z, other.g7.y) + vec3(self.g1.y) * vec3(other.g7.z, other.g0.y, -other.g7.x) + vec3(self.g1.z) * vec3(-other.g7.y, other.g7.x, other.g0.y) - vec3(self.g1.w) * vec3(other.g6.x, other.g6.y, other.g6.z), vec2(self.g0.x) * vec2(0.0, -other.g7.x) + vec2(self.g0.x) * vec2(0.0, other.g1.x) + vec2(self.g0.y) * vec2(0.0, -other.g7.y) + vec2(self.g0.y) * vec2(0.0, other.g1.y) + vec2(self.g0.z) * vec2(0.0, -other.g7.z) + vec2(self.g0.z) * vec2(0.0, other.g1.z) + vec2(self.g0.w) * other.g2 * vec2(1.0, -1.0) + vec2(self.g1.x) * vec2(-other.g6.x, other.g8.x) + vec2(self.g1.y) * vec2(-other.g6.y, other.g8.y) + vec2(self.g1.z) * vec2(-other.g6.z, other.g8.z) + vec2(self.g1.w) * vec2(0.0, -other.g6.w) + vec2(self.g1.w) * vec2(0.0, other.g0.y));
}

MultiVector flector__anti_wedge_dot__plane(Flector self, Plane other) {
    return MultiVector(vec2(self.g1.x) * vec2(0.0, other.g0.x) + vec2(self.g1.y) * vec2(0.0, other.g0.y) + vec2(self.g1.z) * vec2(0.0, other.g0.z), vec3(0.0), vec2(self.g0.x) * vec2(0.0, other.g0.x) + vec2(self.g0.y) * vec2(0.0, other.g0.y) + vec2(self.g0.z) * vec2(0.0, other.g0.z) + vec2(self.g0.w) * vec2(0.0, other.g0.w), vec3(0.0), vec3(0.0), vec4(0.0), vec4(0.0), vec3(0.0) - vec3(self.g0.w) * vec3(other.g0.x, other.g0.y, other.g0.z) + vec3(self.g1.x) * vec3(0.0, other.g0.z, -other.g0.y) + vec3(self.g1.y) * vec3(-other.g0.z, 0.0, other.g0.x) + vec3(self.g1.z) * vec3(other.g0.y, -other.g0.x, 0.0), vec3(self.g0.x) * vec3(0.0, other.g0.z, -other.g0.y) + vec3(self.g0.y) * vec3(-other.g0.z, 0.0, other.g0.x) + vec3(self.g0.z) * vec3(other.g0.y, -other.g0.x, 0.0) + vec3(self.g1.x, self.g1.y, self.g1.z) * vec3(other.g0.w) - vec3(self.g1.w) * vec3(other.g0.x, other.g0.y, other.g0.z), vec3(0.0), vec2(0.0));
}

MultiVector flector__anti_wedge_dot__round_point(Flector self, RoundPoint other) {
    return MultiVector(vec2(self.g1.x) * vec2(other.g0.x, 0.0) + vec2(self.g1.y) * vec2(other.g0.y, 0.0) + vec2(self.g1.z) * vec2(other.g0.z, 0.0) + vec2(self.g1.w) * vec2(other.g1.x, 0.0), vec3(0.0), vec2(0.0), vec3(self.g1.x, self.g1.y, self.g1.z) * vec3(other.g1.x), vec3(self.g0.x, self.g0.y, self.g0.z) * vec3(other.g1.x) - vec3(self.g0.w) * other.g0 + vec3(self.g1.x) * vec3(0.0, other.g0.z, -other.g0.y) + vec3(self.g1.y) * vec3(-other.g0.z, 0.0, other.g0.x) + vec3(self.g1.z) * vec3(other.g0.y, -other.g0.x, 0.0), vec4(self.g0.x) * vec4(0.0, other.g0.z, -other.g0.y, 0.0) + vec4(self.g0.y) * vec4(-other.g0.z, 0.0, other.g0.x, 0.0) + vec4(self.g0.z) * vec4(other.g0.y, -other.g0.x, 0.0, 0.0) + self.g1.xyzx * vec4(-other.g1.y, -other.g1.y, -other.g1.y, 0.0) - vec4(self.g1.w) * vec4(other.g0.x, other.g0.y, other.g0.z, other.g1.x), vec4(0.0), vec3(0.0), vec3(0.0), vec3(0.0) - vec3(self.g0.x, self.g0.y, self.g0.z) * vec3(other.g1.x), vec2(self.g0.x) * vec2(0.0, other.g0.x) + vec2(self.g0.y) * vec2(0.0, other.g0.y) + vec2(self.g0.z) * vec2(0.0, other.g0.z) + vec2(self.g0.w) * other.g1 * vec2(1.0, -1.0));
}

MultiVector flector__anti_wedge_dot__scalar(Flector self, Scalar other) {
    return MultiVector(vec2(0.0), vec3(self.g1.x, self.g1.y, self.g1.z) * vec3(other.g0), vec2(self.g1.w) * vec2(0.0, -other.g0), vec3(0.0), vec3(0.0), vec4(0.0), vec4(self.g0.w) * vec4(0.0, 0.0, 0.0, other.g0), vec3(0.0), vec3(0.0) - vec3(self.g0.x, self.g0.y, self.g0.z) * vec3(other.g0), vec3(0.0), vec2(0.0));
}

MultiVector flector__anti_wedge_dot__sphere(Flector self, Sphere other) {
    return MultiVector(vec2(self.g1.x) * vec2(0.0, other.g0.x) + vec2(self.g1.y) * vec2(0.0, other.g0.y) + vec2(self.g1.z) * vec2(0.0, other.g0.z) + vec2(self.g1.w) * vec2(0.0, -other.g1.x), vec3(0.0) - vec3(self.g0.x, self.g0.y, self.g0.z) * vec3(other.g1.x), vec2(self.g0.x) * vec2(0.0, other.g0.x) + vec2(self.g0.y) * vec2(0.0, other.g0.y) + vec2(self.g0.z) * vec2(0.0, other.g0.z) + vec2(self.g0.w) * other.g1 * vec2(-1.0, 1.0), vec3(0.0), vec3(0.0), vec4(0.0), vec4(0.0) - self.g1 * vec4(other.g1.x), vec3(0.0) - vec3(self.g0.x, self.g0.y, self.g0.z) * vec3(other.g1.x) - vec3(self.g0.w) * other.g0 + vec3(self.g1.x) * vec3(0.0, other.g0.z, -other.g0.y) + vec3(self.g1.y) * vec3(-other.g0.z, 0.0, other.g0.x) + vec3(self.g1.z) * vec3(other.g0.y, -other.g0.x, 0.0), vec3(self.g0.x) * vec3(0.0, other.g0.z, -other.g0.y) + vec3(self.g0.y) * vec3(-other.g0.z, 0.0, other.g0.x) + vec3(self.g0.z) * vec3(other.g0.y, -other.g0.x, 0.0) + vec3(self.g1.x, self.g1.y, self.g1.z) * vec3(other.g1.y) - vec3(self.g1.w) * other.g0, vec3(0.0), vec2(0.0));
}

Line line__anti_wedge_dot__anti_scalar(Line self, AntiScalar other) {
    return Line(self.g0 * vec3(other.g0), self.g1 * vec3(other.g0));
}

MultiVector line__anti_wedge_dot__circle(Line self, Circle other) {
    return MultiVector(vec2(self.g0.x) * vec2(0.0, -other.g1.x) + vec2(self.g0.y) * vec2(0.0, -other.g1.y) + vec2(self.g0.z) * vec2(0.0, -other.g1.z) + vec2(self.g1.x) * vec2(0.0, -other.g0.x) + vec2(self.g1.y) * vec2(0.0, -other.g0.y) + vec2(self.g1.z) * vec2(0.0, -other.g0.z), self.g0 * vec3(other.g0.w) + vec3(self.g1.x) * vec3(0.0, -other.g0.z, other.g0.y) + vec3(self.g1.y) * vec3(other.g0.z, 0.0, -other.g0.x) + vec3(self.g1.z) * vec3(-other.g0.y, other.g0.x, 0.0), vec2(0.0) - vec2(self.g0.x) * vec2(other.g0.x, other.g2.x) - vec2(self.g0.y) * vec2(other.g0.y, other.g2.y) - vec2(self.g0.z) * vec2(other.g0.z, other.g2.z) + vec2(self.g1.x) * vec2(0.0, -other.g1.x) + vec2(self.g1.y) * vec2(0.0, -other.g1.y) + vec2(self.g1.z) * vec2(0.0, -other.g1.z), vec3(0.0), vec3(0.0), vec4(0.0), vec4(self.g0.x) * vec4(0.0, -other.g0.z, other.g0.y, 0.0) + vec4(self.g0.y) * vec4(other.g0.z, 0.0, -other.g0.x, 0.0) + vec4(self.g0.z) * vec4(-other.g0.y, other.g0.x, 0.0, 0.0) + vec4(self.g1.x) * vec4(0.0, 0.0, 0.0, -other.g0.x) + vec4(self.g1.y) * vec4(0.0, 0.0, 0.0, -other.g0.y) + vec4(self.g1.z) * vec4(0.0, 0.0, 0.0, -other.g0.z), vec3(self.g0.x) * vec3(0.0, -other.g1.z, other.g1.y) + vec3(self.g0.y) * vec3(other.g1.z, 0.0, -other.g1.x) + vec3(self.g0.z) * vec3(-other.g1.y, other.g1.x, 0.0) + vec3(self.g1.x) * vec3(0.0, -other.g0.z, other.g0.y) + vec3(self.g1.y) * vec3(other.g0.z, 0.0, -other.g0.x) + vec3(self.g1.z) * vec3(-other.g0.y, other.g0.x, 0.0), vec3(self.g0.x) * vec3(0.0, -other.g2.z, other.g2.y) + vec3(self.g0.y) * vec3(other.g2.z, 0.0, -other.g2.x) + vec3(self.g0.z) * vec3(-other.g2.y, other.g2.x, 0.0) + vec3(self.g1.x) * vec3(-other.g0.w, -other.g1.z, other.g1.y) + vec3(self.g1.y) * vec3(other.g1.z, -other.g0.w, -other.g1.x) + vec3(self.g1.z) * vec3(-other.g1.y, other.g1.x, -other.g0.w), vec3(0.0), vec2(0.0));
}

MultiVector line__anti_wedge_dot__dipole(Line self, Dipole other) {
    return MultiVector(vec2(self.g0.x) * vec2(-other.g1.x, 0.0) + vec2(self.g0.y) * vec2(-other.g1.y, 0.0) + vec2(self.g0.z) * vec2(-other.g1.z, 0.0) + vec2(self.g1.x) * vec2(-other.g0.x, 0.0) + vec2(self.g1.y) * vec2(-other.g0.y, 0.0) + vec2(self.g1.z) * vec2(-other.g0.z, 0.0), vec3(0.0), vec2(0.0), vec3(self.g0.x) * vec3(0.0, -other.g0.z, other.g0.y) + vec3(self.g0.y) * vec3(other.g0.z, 0.0, -other.g0.x) + vec3(self.g0.z) * vec3(-other.g0.y, other.g0.x, 0.0), vec3(self.g0.x) * vec3(0.0, -other.g1.z, other.g1.y) + vec3(self.g0.y) * vec3(other.g1.z, 0.0, -other.g1.x) + vec3(self.g0.z) * vec3(-other.g1.y, other.g1.x, 0.0) + vec3(self.g1.x) * vec3(0.0, -other.g0.z, other.g0.y) + vec3(self.g1.y) * vec3(other.g0.z, 0.0, -other.g0.x) + vec3(self.g1.z) * vec3(-other.g0.y, other.g0.x, 0.0), vec4(self.g0.x) * vec4(0.0, -other.g2.z, other.g2.y, 0.0) + vec4(self.g0.y) * vec4(other.g2.z, 0.0, -other.g2.x, 0.0) + vec4(self.g0.z) * vec4(-other.g2.y, other.g2.x, 0.0, 0.0) + vec4(self.g1.x) * vec4(other.g2.w, -other.g1.z, other.g1.y, other.g0.x) + vec4(self.g1.y) * vec4(other.g1.z, other.g2.w, -other.g1.x, other.g0.y) + vec4(self.g1.z) * vec4(-other.g1.y, other.g1.x, other.g2.w, other.g0.z), vec4(0.0), vec3(0.0), vec3(0.0), self.g0 * vec3(other.g2.w) + vec3(self.g1.x) * vec3(0.0, other.g0.z, -other.g0.y) + vec3(self.g1.y) * vec3(-other.g0.z, 0.0, other.g0.x) + vec3(self.g1.z) * vec3(other.g0.y, -other.g0.x, 0.0), vec2(0.0) - vec2(self.g0.x) * vec2(other.g0.x, other.g2.x) - vec2(self.g0.y) * vec2(other.g0.y, other.g2.y) - vec2(self.g0.z) * vec2(other.g0.z, other.g2.z) + vec2(self.g1.x) * vec2(0.0, -other.g1.x) + vec2(self.g1.y) * vec2(0.0, -other.g1.y) + vec2(self.g1.z) * vec2(0.0, -other.g1.z));
}

MultiVector line__anti_wedge_dot__dual_num(Line self, DualNum other) {
    return MultiVector(vec2(0.0), vec3(0.0), vec2(0.0), vec3(0.0), self.g0 * vec3(other.g0.x), vec4(self.g1.x, self.g1.y, self.g1.z, self.g1.x) * vec4(other.g0.x, other.g0.x, other.g0.x, 0.0), vec4(0.0), self.g0 * vec3(other.g0.y), self.g1 * vec3(other.g0.y), vec3(0.0), vec2(0.0));
}

Flector line__anti_wedge_dot__flat_point(Line self, FlatPoint other) {
    return Flector(vec4(self.g0.x) * vec4(0.0, -other.g0.z, other.g0.y, 0.0) + vec4(self.g0.y) * vec4(other.g0.z, 0.0, -other.g0.x, 0.0) + vec4(self.g0.z) * vec4(-other.g0.y, other.g0.x, 0.0, 0.0) + vec4(self.g1.x, self.g1.y, self.g1.z, self.g1.x) * vec4(other.g0.w, other.g0.w, other.g0.w, 0.0), vec4(self.g0.x) * vec4(other.g0.w, 0.0, 0.0, -other.g0.x) + vec4(self.g0.y) * vec4(0.0, other.g0.w, 0.0, -other.g0.y) + vec4(self.g0.z) * vec4(0.0, 0.0, other.g0.w, -other.g0.z));
}

Flector line__anti_wedge_dot__flector(Line self, Flector other) {
    return Flector(vec4(self.g0.x) * vec4(other.g1.w, -other.g0.z, other.g0.y, -other.g1.x) + vec4(self.g0.y) * vec4(other.g0.z, other.g1.w, -other.g0.x, -other.g1.y) + vec4(self.g0.z) * vec4(-other.g0.y, other.g0.x, other.g1.w, -other.g1.z) + vec4(self.g1.x) * vec4(other.g0.w, -other.g1.z, other.g1.y, 0.0) + vec4(self.g1.y) * vec4(other.g1.z, other.g0.w, -other.g1.x, 0.0) + vec4(self.g1.z) * vec4(-other.g1.y, other.g1.x, other.g0.w, 0.0), vec4(self.g0.x) * vec4(other.g0.w, -other.g1.z, other.g1.y, -other.g0.x) + vec4(self.g0.y) * vec4(other.g1.z, other.g0.w, -other.g1.x, -other.g0.y) + vec4(self.g0.z) * vec4(-other.g1.y, other.g1.x, other.g0.w, -other.g0.z) + vec4(self.g1.x) * vec4(0.0, 0.0, 0.0, -other.g1.x) + vec4(self.g1.y) * vec4(0.0, 0.0, 0.0, -other.g1.y) + vec4(self.g1.z) * vec4(0.0, 0.0, 0.0, -other.g1.z));
}

MultiVector line__anti_wedge_dot__line(Line self, Line other) {
    return MultiVector(vec2(self.g0.x) * vec2(0.0, -other.g0.x) + vec2(self.g0.y) * vec2(0.0, -other.g0.y) + vec2(self.g0.z) * vec2(0.0, -other.g0.z), vec3(0.0), vec2(self.g0.x) * vec2(0.0, -other.g1.x) + vec2(self.g0.y) * vec2(0.0, -other.g1.y) + vec2(self.g0.z) * vec2(0.0, -other.g1.z) + vec2(self.g1.x) * vec2(0.0, -other.g0.x) + vec2(self.g1.y) * vec2(0.0, -other.g0.y) + vec2(self.g1.z) * vec2(0.0, -other.g0.z), vec3(0.0), vec3(0.0), vec4(0.0), vec4(0.0), vec3(self.g0.x) * vec3(0.0, -other.g0.z, other.g0.y) + vec3(self.g0.y) * vec3(other.g0.z, 0.0, -other.g0.x) + vec3(self.g0.z) * vec3(-other.g0.y, other.g0.x, 0.0), vec3(self.g0.x) * vec3(0.0, -other.g1.z, other.g1.y) + vec3(self.g0.y) * vec3(other.g1.z, 0.0, -other.g1.x) + vec3(self.g0.z) * vec3(-other.g1.y, other.g1.x, 0.0) + vec3(self.g1.x) * vec3(0.0, -other.g0.z, other.g0.y) + vec3(self.g1.y) * vec3(other.g0.z, 0.0, -other.g0.x) + vec3(self.g1.z) * vec3(-other.g0.y, other.g0.x, 0.0), vec3(0.0), vec2(0.0));
}

MultiVector line__anti_wedge_dot__motor(Line self, Motor other) {
    return MultiVector(vec2(self.g0.x) * vec2(0.0, -other.g0.x) + vec2(self.g0.y) * vec2(0.0, -other.g0.y) + vec2(self.g0.z) * vec2(0.0, -other.g0.z), vec3(0.0), vec2(self.g0.x) * vec2(0.0, -other.g1.x) + vec2(self.g0.y) * vec2(0.0, -other.g1.y) + vec2(self.g0.z) * vec2(0.0, -other.g1.z) + vec2(self.g1.x) * vec2(0.0, -other.g0.x) + vec2(self.g1.y) * vec2(0.0, -other.g0.y) + vec2(self.g1.z) * vec2(0.0, -other.g0.z), vec3(0.0), vec3(0.0), vec4(0.0), vec4(0.0), vec3(self.g0.x) * vec3(other.g0.w, -other.g0.z, other.g0.y) + vec3(self.g0.y) * vec3(other.g0.z, other.g0.w, -other.g0.x) + vec3(self.g0.z) * vec3(-other.g0.y, other.g0.x, other.g0.w), vec3(self.g0.x) * vec3(0.0, -other.g1.z, other.g1.y) + vec3(self.g0.y) * vec3(other.g1.z, 0.0, -other.g1.x) + vec3(self.g0.z) * vec3(-other.g1.y, other.g1.x, 0.0) + vec3(self.g1.x) * vec3(other.g0.w, -other.g0.z, other.g0.y) + vec3(self.g1.y) * vec3(other.g0.z, other.g0.w, -other.g0.x) + vec3(self.g1.z) * vec3(-other.g0.y, other.g0.x, other.g0.w), vec3(0.0), vec2(0.0));
}

MultiVector line__anti_wedge_dot__multi_vector(Line self, MultiVector other) {
    return MultiVector(vec2(0.0) - vec2(self.g0.x) * vec2(other.g4.x, other.g7.x) - vec2(self.g0.y) * vec2(other.g4.y, other.g7.y) - vec2(self.g0.z) * vec2(other.g4.z, other.g7.z) - vec2(self.g1.x) * vec2(other.g3.x, other.g6.x) - vec2(self.g1.y) * vec2(other.g3.y, other.g6.y) - vec2(self.g1.z) * vec2(other.g3.z, other.g6.z), vec3(self.g0.x) * vec3(other.g6.w, -other.g1.z, other.g1.y) + vec3(self.g0.y) * vec3(other.g1.z, other.g6.w, -other.g1.x) + vec3(self.g0.z) * vec3(-other.g1.y, other.g1.x, other.g6.w) + vec3(self.g1.x) * vec3(other.g2.x, -other.g6.z, other.g6.y) + vec3(self.g1.y) * vec3(other.g6.z, other.g2.x, -other.g6.x) + vec3(self.g1.z) * vec3(-other.g6.y, other.g6.x, other.g2.x), vec2(0.0) - vec2(self.g0.x) * vec2(other.g6.x, other.g8.x) - vec2(self.g0.y) * vec2(other.g6.y, other.g8.y) - vec2(self.g0.z) * vec2(other.g6.z, other.g8.z) + vec2(self.g1.x) * vec2(0.0, -other.g7.x) + vec2(self.g1.x) * vec2(0.0, other.g1.x) + vec2(self.g1.y) * vec2(0.0, -other.g7.y) + vec2(self.g1.y) * vec2(0.0, other.g1.y) + vec2(self.g1.z) * vec2(0.0, -other.g7.z) + vec2(self.g1.z) * vec2(0.0, other.g1.z), vec3(self.g0.x) * vec3(other.g10.x, -other.g3.z, other.g3.y) + vec3(self.g0.y) * vec3(other.g3.z, other.g10.x, -other.g3.x) + vec3(self.g0.z) * vec3(-other.g3.y, other.g3.x, other.g10.x), vec3(self.g0.x) * vec3(other.g0.x, -other.g4.z, other.g4.y) + vec3(self.g0.y) * vec3(other.g4.z, other.g0.x, -other.g4.x) + vec3(self.g0.z) * vec3(-other.g4.y, other.g4.x, other.g0.x) + vec3(self.g1.x) * vec3(other.g10.x, -other.g3.z, other.g3.y) + vec3(self.g1.y) * vec3(other.g3.z, other.g10.x, -other.g3.x) + vec3(self.g1.z) * vec3(-other.g3.y, other.g3.x, other.g10.x), vec4(self.g0.x) * vec4(other.g10.y, -other.g5.z, other.g5.y, -other.g9.x) + vec4(self.g0.y) * vec4(other.g5.z, other.g10.y, -other.g5.x, -other.g9.y) + vec4(self.g0.z) * vec4(-other.g5.y, other.g5.x, other.g10.y, -other.g9.z) + vec4(self.g1.x) * vec4(other.g5.w, -other.g9.z, other.g9.y, other.g3.x) + vec4(self.g1.x) * vec4(other.g0.x, -other.g4.z, other.g4.y, 0.0) + vec4(self.g1.y) * vec4(other.g9.z, other.g5.w, -other.g9.x, other.g3.y) + vec4(self.g1.y) * vec4(other.g4.z, other.g0.x, -other.g4.x, 0.0) + vec4(self.g1.z) * vec4(-other.g9.y, other.g9.x, other.g5.w, other.g3.z) + vec4(self.g1.z) * vec4(-other.g4.y, other.g4.x, other.g0.x, 0.0), vec4(self.g0.x) * vec4(other.g2.x, -other.g6.z, other.g6.y, -other.g1.x) + vec4(self.g0.y) * vec4(other.g6.z, other.g2.x, -other.g6.x, -other.g1.y) + vec4(self.g0.z) * vec4(-other.g6.y, other.g6.x, other.g2.x, -other.g1.z) + vec4(self.g1.x) * vec4(0.0, 0.0, 0.0, -other.g6.x) + vec4(self.g1.y) * vec4(0.0, 0.0, 0.0, -other.g6.y) + vec4(self.g1.z) * vec4(0.0, 0.0, 0.0, -other.g6.z), vec3(self.g0.x) * vec3(other.g0.y, -other.g7.z, other.g7.y) + vec3(self.g0.y) * vec3(other.g7.z, other.g0.y, -other.g7.x) + vec3(self.g0.z) * vec3(-other.g7.y, other.g7.x, other.g0.y) + vec3(self.g1.x) * vec3(other.g2.x, -other.g6.z, other.g6.y) + vec3(self.g1.y) * vec3(other.g6.z, other.g2.x, -other.g6.x) + vec3(self.g1.z) * vec3(-other.g6.y, other.g6.x, other.g2.x), vec3(self.g0.x) * vec3(other.g2.y, -other.g8.z, other.g8.y) + vec3(self.g0.y) * vec3(other.g8.z, other.g2.y, -other.g8.x) + vec3(self.g0.z) * vec3(-other.g8.y, other.g8.x, other.g2.y) + vec3(self.g1.x) * vec3(-other.g6.w, -other.g7.z, other.g7.y) + vec3(self.g1.x) * vec3(other.g0.y, other.g1.z, -other.g1.y) + vec3(self.g1.y) * vec3(other.g7.z, -other.g6.w, -other.g7.x) + vec3(self.g1.y) * vec3(-other.g1.z, other.g0.y, other.g1.x) + vec3(self.g1.z) * vec3(-other.g7.y, other.g7.x, -other.g6.w) + vec3(self.g1.z) * vec3(other.g1.y, -other.g1.x, other.g0.y), vec3(self.g0.x) * vec3(other.g5.w, -other.g9.z, other.g9.y) + vec3(self.g0.y) * vec3(other.g9.z, other.g5.w, -other.g9.x) + vec3(self.g0.z) * vec3(-other.g9.y, other.g9.x, other.g5.w) + vec3(self.g1.x) * vec3(-other.g10.x, other.g3.z, -other.g3.y) + vec3(self.g1.y) * vec3(-other.g3.z, -other.g10.x, other.g3.x) + vec3(self.g1.z) * vec3(other.g3.y, -other.g3.x, -other.g10.x), vec2(0.0) - vec2(self.g0.x) * vec2(other.g3.x, other.g5.x) - vec2(self.g0.y) * vec2(other.g3.y, other.g5.y) - vec2(self.g0.z) * vec2(other.g3.z, other.g5.z) + vec2(self.g1.x) * vec2(0.0, -other.g9.x) + vec2(self.g1.x) * vec2(0.0, -other.g4.x) + vec2(self.g1.y) * vec2(0.0, -other.g9.y) + vec2(self.g1.y) * vec2(0.0, -other.g4.y) + vec2(self.g1.z) * vec2(0.0, -other.g9.z) + vec2(self.g1.z) * vec2(0.0, -other.g4.z));
}

Flector line__anti_wedge_dot__plane(Line self, Plane other) {
    return Flector(vec4(self.g0.x) * vec4(other.g0.w, 0.0, 0.0, -other.g0.x) + vec4(self.g0.y) * vec4(0.0, other.g0.w, 0.0, -other.g0.y) + vec4(self.g0.z) * vec4(0.0, 0.0, other.g0.w, -other.g0.z) + vec4(self.g1.x) * vec4(0.0, -other.g0.z, other.g0.y, 0.0) + vec4(self.g1.y) * vec4(other.g0.z, 0.0, -other.g0.x, 0.0) + vec4(self.g1.z) * vec4(-other.g0.y, other.g0.x, 0.0, 0.0), vec4(self.g0.x) * vec4(0.0, -other.g0.z, other.g0.y, 0.0) + vec4(self.g0.y) * vec4(other.g0.z, 0.0, -other.g0.x, 0.0) + vec4(self.g0.z) * vec4(-other.g0.y, other.g0.x, 0.0, 0.0) + vec4(self.g1.x) * vec4(0.0, 0.0, 0.0, -other.g0.x) + vec4(self.g1.y) * vec4(0.0, 0.0, 0.0, -other.g0.y) + vec4(self.g1.z) * vec4(0.0, 0.0, 0.0, -other.g0.z));
}

MultiVector line__anti_wedge_dot__round_point(Line self, RoundPoint other) {
    return MultiVector(vec2(0.0), vec3(self.g0.x) * vec3(0.0, -other.g0.z, other.g0.y) + vec3(self.g0.y) * vec3(other.g0.z, 0.0, -other.g0.x) + vec3(self.g0.z) * vec3(-other.g0.y, other.g0.x, 0.0) + self.g1 * vec3(other.g1.x), vec2(self.g1.x) * vec2(0.0, other.g0.x) + vec2(self.g1.y) * vec2(0.0, other.g0.y) + vec2(self.g1.z) * vec2(0.0, other.g0.z), vec3(0.0), vec3(0.0), vec4(0.0), vec4(self.g0.x) * vec4(other.g1.x, 0.0, 0.0, -other.g0.x) + vec4(self.g0.y) * vec4(0.0, other.g1.x, 0.0, -other.g0.y) + vec4(self.g0.z) * vec4(0.0, 0.0, other.g1.x, -other.g0.z), self.g1 * vec3(other.g1.x), self.g0 * vec3(other.g1.y) + vec3(self.g1.x) * vec3(0.0, other.g0.z, -other.g0.y) + vec3(self.g1.y) * vec3(-other.g0.z, 0.0, other.g0.x) + vec3(self.g1.z) * vec3(other.g0.y, -other.g0.x, 0.0), vec3(0.0), vec2(0.0));
}

Dipole line__anti_wedge_dot__scalar(Line self, Scalar other) {
    return Dipole(vec3(0.0), self.g0 * vec3(other.g0), vec4(self.g1.x, self.g1.y, self.g1.z, self.g1.x) * vec4(other.g0, other.g0, other.g0, 0.0));
}

MultiVector line__anti_wedge_dot__sphere(Line self, Sphere other) {
    return MultiVector(vec2(0.0), vec3(0.0), vec2(0.0), self.g0 * vec3(other.g1.x), self.g1 * vec3(other.g1.x), vec4(self.g0.x) * vec4(other.g1.y, 0.0, 0.0, -other.g0.x) + vec4(self.g0.y) * vec4(0.0, other.g1.y, 0.0, -other.g0.y) + vec4(self.g0.z) * vec4(0.0, 0.0, other.g1.y, -other.g0.z) + vec4(self.g1.x) * vec4(0.0, -other.g0.z, other.g0.y, 0.0) + vec4(self.g1.y) * vec4(other.g0.z, 0.0, -other.g0.x, 0.0) + vec4(self.g1.z) * vec4(-other.g0.y, other.g0.x, 0.0, 0.0), vec4(0.0), vec3(0.0), vec3(0.0), vec3(self.g0.x) * vec3(0.0, -other.g0.z, other.g0.y) + vec3(self.g0.y) * vec3(other.g0.z, 0.0, -other.g0.x) + vec3(self.g0.z) * vec3(-other.g0.y, other.g0.x, 0.0) - self.g1 * vec3(other.g1.x), vec2(self.g1.x) * vec2(0.0, -other.g0.x) + vec2(self.g1.y) * vec2(0.0, -other.g0.y) + vec2(self.g1.z) * vec2(0.0, -other.g0.z));
}

Motor motor__anti_wedge_dot__anti_scalar(Motor self, AntiScalar other) {
    return Motor(self.g0 * vec4(other.g0), self.g1 * vec3(other.g0));
}

MultiVector motor__anti_wedge_dot__circle(Motor self, Circle other) {
    return MultiVector(vec2(self.g0.x) * vec2(0.0, -other.g1.x) + vec2(self.g0.y) * vec2(0.0, -other.g1.y) + vec2(self.g0.z) * vec2(0.0, -other.g1.z) + vec2(self.g1.x) * vec2(0.0, -other.g0.x) + vec2(self.g1.y) * vec2(0.0, -other.g0.y) + vec2(self.g1.z) * vec2(0.0, -other.g0.z), vec3(self.g0.x, self.g0.y, self.g0.z) * vec3(other.g0.w) + vec3(self.g1.x) * vec3(0.0, -other.g0.z, other.g0.y) + vec3(self.g1.y) * vec3(other.g0.z, 0.0, -other.g0.x) + vec3(self.g1.z) * vec3(-other.g0.y, other.g0.x, 0.0), vec2(0.0) - vec2(self.g0.x) * vec2(other.g0.x, other.g2.x) - vec2(self.g0.y) * vec2(other.g0.y, other.g2.y) - vec2(self.g0.z) * vec2(other.g0.z, other.g2.z) + vec2(self.g1.x) * vec2(0.0, -other.g1.x) + vec2(self.g1.y) * vec2(0.0, -other.g1.y) + vec2(self.g1.z) * vec2(0.0, -other.g1.z), vec3(0.0), vec3(0.0), vec4(0.0), vec4(self.g0.x) * vec4(0.0, -other.g0.z, other.g0.y, 0.0) + vec4(self.g0.y) * vec4(other.g0.z, 0.0, -other.g0.x, 0.0) + vec4(self.g0.z) * vec4(-other.g0.y, other.g0.x, 0.0, 0.0) + vec4(self.g0.w) * other.g0 + vec4(self.g1.x) * vec4(0.0, 0.0, 0.0, -other.g0.x) + vec4(self.g1.y) * vec4(0.0, 0.0, 0.0, -other.g0.y) + vec4(self.g1.z) * vec4(0.0, 0.0, 0.0, -other.g0.z), vec3(self.g0.x) * vec3(0.0, -other.g1.z, other.g1.y) + vec3(self.g0.y) * vec3(other.g1.z, 0.0, -other.g1.x) + vec3(self.g0.z) * vec3(-other.g1.y, other.g1.x, 0.0) + vec3(self.g0.w) * other.g1 + vec3(self.g1.x) * vec3(0.0, -other.g0.z, other.g0.y) + vec3(self.g1.y) * vec3(other.g0.z, 0.0, -other.g0.x) + vec3(self.g1.z) * vec3(-other.g0.y, other.g0.x, 0.0), vec3(self.g0.x) * vec3(0.0, -other.g2.z, other.g2.y) + vec3(self.g0.y) * vec3(other.g2.z, 0.0, -other.g2.x) + vec3(self.g0.z) * vec3(-other.g2.y, other.g2.x, 0.0) + vec3(self.g0.w) * other.g2 + vec3(self.g1.x) * vec3(-other.g0.w, -other.g1.z, other.g1.y) + vec3(self.g1.y) * vec3(other.g1.z, -other.g0.w, -other.g1.x) + vec3(self.g1.z) * vec3(-other.g1.y, other.g1.x, -other.g0.w), vec3(0.0), vec2(0.0));
}

MultiVector motor__anti_wedge_dot__dipole(Motor self, Dipole other) {
    return MultiVector(vec2(self.g0.x) * vec2(-other.g1.x, 0.0) + vec2(self.g0.y) * vec2(-other.g1.y, 0.0) + vec2(self.g0.z) * vec2(-other.g1.z, 0.0) + vec2(self.g1.x) * vec2(-other.g0.x, 0.0) + vec2(self.g1.y) * vec2(-other.g0.y, 0.0) + vec2(self.g1.z) * vec2(-other.g0.z, 0.0), vec3(0.0), vec2(0.0), vec3(self.g0.x) * vec3(0.0, -other.g0.z, other.g0.y) + vec3(self.g0.y) * vec3(other.g0.z, 0.0, -other.g0.x) + vec3(self.g0.z) * vec3(-other.g0.y, other.g0.x, 0.0) + vec3(self.g0.w) * other.g0, vec3(self.g0.x) * vec3(0.0, -other.g1.z, other.g1.y) + vec3(self.g0.y) * vec3(other.g1.z, 0.0, -other.g1.x) + vec3(self.g0.z) * vec3(-other.g1.y, other.g1.x, 0.0) + vec3(self.g0.w) * other.g1 + vec3(self.g1.x) * vec3(0.0, -other.g0.z, other.g0.y) + vec3(self.g1.y) * vec3(other.g0.z, 0.0, -other.g0.x) + vec3(self.g1.z) * vec3(-other.g0.y, other.g0.x, 0.0), vec4(self.g0.x) * vec4(0.0, -other.g2.z, other.g2.y, 0.0) + vec4(self.g0.y) * vec4(other.g2.z, 0.0, -other.g2.x, 0.0) + vec4(self.g0.z) * vec4(-other.g2.y, other.g2.x, 0.0, 0.0) + vec4(self.g0.w) * other.g2 + vec4(self.g1.x) * vec4(other.g2.w, -other.g1.z, other.g1.y, other.g0.x) + vec4(self.g1.y) * vec4(other.g1.z, other.g2.w, -other.g1.x, other.g0.y) + vec4(self.g1.z) * vec4(-other.g1.y, other.g1.x, other.g2.w, other.g0.z), vec4(0.0), vec3(0.0), vec3(0.0), vec3(self.g0.x, self.g0.y, self.g0.z) * vec3(other.g2.w) + vec3(self.g1.x) * vec3(0.0, other.g0.z, -other.g0.y) + vec3(self.g1.y) * vec3(-other.g0.z, 0.0, other.g0.x) + vec3(self.g1.z) * vec3(other.g0.y, -other.g0.x, 0.0), vec2(0.0) - vec2(self.g0.x) * vec2(other.g0.x, other.g2.x) - vec2(self.g0.y) * vec2(other.g0.y, other.g2.y) - vec2(self.g0.z) * vec2(other.g0.z, other.g2.z) + vec2(self.g1.x) * vec2(0.0, -other.g1.x) + vec2(self.g1.y) * vec2(0.0, -other.g1.y) + vec2(self.g1.z) * vec2(0.0, -other.g1.z));
}

MultiVector motor__anti_wedge_dot__dual_num(Motor self, DualNum other) {
    return MultiVector(vec2(self.g0.w) * other.g0, vec3(0.0), vec2(0.0), vec3(0.0), vec3(self.g0.x, self.g0.y, self.g0.z) * vec3(other.g0.x), vec4(self.g1.x, self.g1.y, self.g1.z, self.g1.x) * vec4(other.g0.x, other.g0.x, other.g0.x, 0.0), vec4(0.0), vec3(self.g0.x, self.g0.y, self.g0.z) * vec3(other.g0.y), self.g1 * vec3(other.g0.y), vec3(0.0), vec2(0.0));
}

Flector motor__anti_wedge_dot__flat_point(Motor self, FlatPoint other) {
    return Flector(vec4(self.g0.x) * vec4(0.0, -other.g0.z, other.g0.y, 0.0) + vec4(self.g0.y) * vec4(other.g0.z, 0.0, -other.g0.x, 0.0) + vec4(self.g0.z) * vec4(-other.g0.y, other.g0.x, 0.0, 0.0) + vec4(self.g0.w) * other.g0 + vec4(self.g1.x, self.g1.y, self.g1.z, self.g1.x) * vec4(other.g0.w, other.g0.w, other.g0.w, 0.0), vec4(self.g0.x) * vec4(other.g0.w, 0.0, 0.0, -other.g0.x) + vec4(self.g0.y) * vec4(0.0, other.g0.w, 0.0, -other.g0.y) + vec4(self.g0.z) * vec4(0.0, 0.0, other.g0.w, -other.g0.z));
}

Flector motor__anti_wedge_dot__flector(Motor self, Flector other) {
    return Flector(vec4(self.g0.x) * vec4(other.g1.w, -other.g0.z, other.g0.y, -other.g1.x) + vec4(self.g0.y) * vec4(other.g0.z, other.g1.w, -other.g0.x, -other.g1.y) + vec4(self.g0.z) * vec4(-other.g0.y, other.g0.x, other.g1.w, -other.g1.z) + vec4(self.g0.w) * other.g0 + vec4(self.g1.x) * vec4(other.g0.w, -other.g1.z, other.g1.y, 0.0) + vec4(self.g1.y) * vec4(other.g1.z, other.g0.w, -other.g1.x, 0.0) + vec4(self.g1.z) * vec4(-other.g1.y, other.g1.x, other.g0.w, 0.0), vec4(self.g0.x) * vec4(other.g0.w, -other.g1.z, other.g1.y, -other.g0.x) + vec4(self.g0.y) * vec4(other.g1.z, other.g0.w, -other.g1.x, -other.g0.y) + vec4(self.g0.z) * vec4(-other.g1.y, other.g1.x, other.g0.w, -other.g0.z) + vec4(self.g0.w) * other.g1 + vec4(self.g1.x) * vec4(0.0, 0.0, 0.0, -other.g1.x) + vec4(self.g1.y) * vec4(0.0, 0.0, 0.0, -other.g1.y) + vec4(self.g1.z) * vec4(0.0, 0.0, 0.0, -other.g1.z));
}

MultiVector motor__anti_wedge_dot__line(Motor self, Line other) {
    return MultiVector(vec2(self.g0.x) * vec2(0.0, -other.g0.x) + vec2(self.g0.y) * vec2(0.0, -other.g0.y) + vec2(self.g0.z) * vec2(0.0, -other.g0.z), vec3(0.0), vec2(self.g0.x) * vec2(0.0, -other.g1.x) + vec2(self.g0.y) * vec2(0.0, -other.g1.y) + vec2(self.g0.z) * vec2(0.0, -other.g1.z) + vec2(self.g1.x) * vec2(0.0, -other.g0.x) + vec2(self.g1.y) * vec2(0.0, -other.g0.y) + vec2(self.g1.z) * vec2(0.0, -other.g0.z), vec3(0.0), vec3(0.0), vec4(0.0), vec4(0.0), vec3(self.g0.x) * vec3(0.0, -other.g0.z, other.g0.y) + vec3(self.g0.y) * vec3(other.g0.z, 0.0, -other.g0.x) + vec3(self.g0.z) * vec3(-other.g0.y, other.g0.x, 0.0) + vec3(self.g0.w) * other.g0, vec3(self.g0.x) * vec3(0.0, -other.g1.z, other.g1.y) + vec3(self.g0.y) * vec3(other.g1.z, 0.0, -other.g1.x) + vec3(self.g0.z) * vec3(-other.g1.y, other.g1.x, 0.0) + vec3(self.g0.w) * other.g1 + vec3(self.g1.x) * vec3(0.0, -other.g0.z, other.g0.y) + vec3(self.g1.y) * vec3(other.g0.z, 0.0, -other.g0.x) + vec3(self.g1.z) * vec3(-other.g0.y, other.g0.x, 0.0), vec3(0.0), vec2(0.0));
}

MultiVector motor__anti_wedge_dot__motor(Motor self, Motor other) {
    return MultiVector(vec2(self.g0.x) * vec2(0.0, -other.g0.x) + vec2(self.g0.y) * vec2(0.0, -other.g0.y) + vec2(self.g0.z) * vec2(0.0, -other.g0.z) + vec2(self.g0.w) * vec2(0.0, other.g0.w), vec3(0.0), vec2(self.g0.x) * vec2(0.0, -other.g1.x) + vec2(self.g0.y) * vec2(0.0, -other.g1.y) + vec2(self.g0.z) * vec2(0.0, -other.g1.z) + vec2(self.g1.x) * vec2(0.0, -other.g0.x) + vec2(self.g1.y) * vec2(0.0, -other.g0.y) + vec2(self.g1.z) * vec2(0.0, -other.g0.z), vec3(0.0), vec3(0.0), vec4(0.0), vec4(0.0), vec3(self.g0.x) * vec3(other.g0.w, -other.g0.z, other.g0.y) + vec3(self.g0.y) * vec3(other.g0.z, other.g0.w, -other.g0.x) + vec3(self.g0.z) * vec3(-other.g0.y, other.g0.x, other.g0.w) + vec3(self.g0.w) * vec3(other.g0.x, other.g0.y, other.g0.z), vec3(self.g0.x) * vec3(0.0, -other.g1.z, other.g1.y) + vec3(self.g0.y) * vec3(other.g1.z, 0.0, -other.g1.x) + vec3(self.g0.z) * vec3(-other.g1.y, other.g1.x, 0.0) + vec3(self.g0.w) * other.g1 + vec3(self.g1.x) * vec3(other.g0.w, -other.g0.z, other.g0.y) + vec3(self.g1.y) * vec3(other.g0.z, other.g0.w, -other.g0.x) + vec3(self.g1.z) * vec3(-other.g0.y, other.g0.x, other.g0.w), vec3(0.0), vec2(0.0));
}

MultiVector motor__anti_wedge_dot__multi_vector(Motor self, MultiVector other) {
    return MultiVector(vec2(0.0) - vec2(self.g0.x) * vec2(other.g4.x, other.g7.x) - vec2(self.g0.y) * vec2(other.g4.y, other.g7.y) - vec2(self.g0.z) * vec2(other.g4.z, other.g7.z) + vec2(self.g0.w) * other.g0 - vec2(self.g1.x) * vec2(other.g3.x, other.g6.x) - vec2(self.g1.y) * vec2(other.g3.y, other.g6.y) - vec2(self.g1.z) * vec2(other.g3.z, other.g6.z), vec3(self.g0.x) * vec3(other.g6.w, -other.g1.z, other.g1.y) + vec3(self.g0.y) * vec3(other.g1.z, other.g6.w, -other.g1.x) + vec3(self.g0.z) * vec3(-other.g1.y, other.g1.x, other.g6.w) + vec3(self.g0.w) * other.g1 + vec3(self.g1.x) * vec3(other.g2.x, -other.g6.z, other.g6.y) + vec3(self.g1.y) * vec3(other.g6.z, other.g2.x, -other.g6.x) + vec3(self.g1.z) * vec3(-other.g6.y, other.g6.x, other.g2.x), vec2(0.0) - vec2(self.g0.x) * vec2(other.g6.x, other.g8.x) - vec2(self.g0.y) * vec2(other.g6.y, other.g8.y) - vec2(self.g0.z) * vec2(other.g6.z, other.g8.z) + vec2(self.g0.w) * other.g2 + vec2(self.g1.x) * vec2(0.0, -other.g7.x) + vec2(self.g1.x) * vec2(0.0, other.g1.x) + vec2(self.g1.y) * vec2(0.0, -other.g7.y) + vec2(self.g1.y) * vec2(0.0, other.g1.y) + vec2(self.g1.z) * vec2(0.0, -other.g7.z) + vec2(self.g1.z) * vec2(0.0, other.g1.z), vec3(self.g0.x) * vec3(other.g10.x, -other.g3.z, other.g3.y) + vec3(self.g0.y) * vec3(other.g3.z, other.g10.x, -other.g3.x) + vec3(self.g0.z) * vec3(-other.g3.y, other.g3.x, other.g10.x) + vec3(self.g0.w) * other.g3, vec3(self.g0.x) * vec3(other.g0.x, -other.g4.z, other.g4.y) + vec3(self.g0.y) * vec3(other.g4.z, other.g0.x, -other.g4.x) + vec3(self.g0.z) * vec3(-other.g4.y, other.g4.x, other.g0.x) + vec3(self.g0.w) * other.g4 + vec3(self.g1.x) * vec3(other.g10.x, -other.g3.z, other.g3.y) + vec3(self.g1.y) * vec3(other.g3.z, other.g10.x, -other.g3.x) + vec3(self.g1.z) * vec3(-other.g3.y, other.g3.x, other.g10.x), vec4(self.g0.x) * vec4(other.g10.y, -other.g5.z, other.g5.y, -other.g9.x) + vec4(self.g0.y) * vec4(other.g5.z, other.g10.y, -other.g5.x, -other.g9.y) + vec4(self.g0.z) * vec4(-other.g5.y, other.g5.x, other.g10.y, -other.g9.z) + vec4(self.g0.w) * other.g5 + vec4(self.g1.x) * vec4(other.g5.w, -other.g9.z, other.g9.y, other.g3.x) + vec4(self.g1.x) * vec4(other.g0.x, -other.g4.z, other.g4.y, 0.0) + vec4(self.g1.y) * vec4(other.g9.z, other.g5.w, -other.g9.x, other.g3.y) + vec4(self.g1.y) * vec4(other.g4.z, other.g0.x, -other.g4.x, 0.0) + vec4(self.g1.z) * vec4(-other.g9.y, other.g9.x, other.g5.w, other.g3.z) + vec4(self.g1.z) * vec4(-other.g4.y, other.g4.x, other.g0.x, 0.0), vec4(self.g0.x) * vec4(other.g2.x, -other.g6.z, other.g6.y, -other.g1.x) + vec4(self.g0.y) * vec4(other.g6.z, other.g2.x, -other.g6.x, -other.g1.y) + vec4(self.g0.z) * vec4(-other.g6.y, other.g6.x, other.g2.x, -other.g1.z) + vec4(self.g0.w) * other.g6 + vec4(self.g1.x) * vec4(0.0, 0.0, 0.0, -other.g6.x) + vec4(self.g1.y) * vec4(0.0, 0.0, 0.0, -other.g6.y) + vec4(self.g1.z) * vec4(0.0, 0.0, 0.0, -other.g6.z), vec3(self.g0.x) * vec3(other.g0.y, -other.g7.z, other.g7.y) + vec3(self.g0.y) * vec3(other.g7.z, other.g0.y, -other.g7.x) + vec3(self.g0.z) * vec3(-other.g7.y, other.g7.x, other.g0.y) + vec3(self.g0.w) * other.g7 + vec3(self.g1.x) * vec3(other.g2.x, -other.g6.z, other.g6.y) + vec3(self.g1.y) * vec3(other.g6.z, other.g2.x, -other.g6.x) + vec3(self.g1.z) * vec3(-other.g6.y, other.g6.x, other.g2.x), vec3(self.g0.x) * vec3(other.g2.y, -other.g8.z, other.g8.y) + vec3(self.g0.y) * vec3(other.g8.z, other.g2.y, -other.g8.x) + vec3(self.g0.z) * vec3(-other.g8.y, other.g8.x, other.g2.y) + vec3(self.g0.w) * other.g8 + vec3(self.g1.x) * vec3(-other.g6.w, -other.g7.z, other.g7.y) + vec3(self.g1.x) * vec3(other.g0.y, other.g1.z, -other.g1.y) + vec3(self.g1.y) * vec3(other.g7.z, -other.g6.w, -other.g7.x) + vec3(self.g1.y) * vec3(-other.g1.z, other.g0.y, other.g1.x) + vec3(self.g1.z) * vec3(-other.g7.y, other.g7.x, -other.g6.w) + vec3(self.g1.z) * vec3(other.g1.y, -other.g1.x, other.g0.y), vec3(self.g0.x) * vec3(other.g5.w, -other.g9.z, other.g9.y) + vec3(self.g0.y) * vec3(other.g9.z, other.g5.w, -other.g9.x) + vec3(self.g0.z) * vec3(-other.g9.y, other.g9.x, other.g5.w) + vec3(self.g0.w) * other.g9 + vec3(self.g1.x) * vec3(-other.g10.x, other.g3.z, -other.g3.y) + vec3(self.g1.y) * vec3(-other.g3.z, -other.g10.x, other.g3.x) + vec3(self.g1.z) * vec3(other.g3.y, -other.g3.x, -other.g10.x), vec2(0.0) - vec2(self.g0.x) * vec2(other.g3.x, other.g5.x) - vec2(self.g0.y) * vec2(other.g3.y, other.g5.y) - vec2(self.g0.z) * vec2(other.g3.z, other.g5.z) + vec2(self.g0.w) * other.g10 + vec2(self.g1.x) * vec2(0.0, -other.g9.x) + vec2(self.g1.x) * vec2(0.0, -other.g4.x) + vec2(self.g1.y) * vec2(0.0, -other.g9.y) + vec2(self.g1.y) * vec2(0.0, -other.g4.y) + vec2(self.g1.z) * vec2(0.0, -other.g9.z) + vec2(self.g1.z) * vec2(0.0, -other.g4.z));
}

Flector motor__anti_wedge_dot__plane(Motor self, Plane other) {
    return Flector(vec4(self.g0.x) * vec4(other.g0.w, 0.0, 0.0, -other.g0.x) + vec4(self.g0.y) * vec4(0.0, other.g0.w, 0.0, -other.g0.y) + vec4(self.g0.z) * vec4(0.0, 0.0, other.g0.w, -other.g0.z) + vec4(self.g1.x) * vec4(0.0, -other.g0.z, other.g0.y, 0.0) + vec4(self.g1.y) * vec4(other.g0.z, 0.0, -other.g0.x, 0.0) + vec4(self.g1.z) * vec4(-other.g0.y, other.g0.x, 0.0, 0.0), vec4(self.g0.x) * vec4(0.0, -other.g0.z, other.g0.y, 0.0) + vec4(self.g0.y) * vec4(other.g0.z, 0.0, -other.g0.x, 0.0) + vec4(self.g0.z) * vec4(-other.g0.y, other.g0.x, 0.0, 0.0) + vec4(self.g0.w) * other.g0 + vec4(self.g1.x) * vec4(0.0, 0.0, 0.0, -other.g0.x) + vec4(self.g1.y) * vec4(0.0, 0.0, 0.0, -other.g0.y) + vec4(self.g1.z) * vec4(0.0, 0.0, 0.0, -other.g0.z));
}

MultiVector motor__anti_wedge_dot__round_point(Motor self, RoundPoint other) {
    return MultiVector(vec2(0.0), vec3(self.g0.x) * vec3(0.0, -other.g0.z, other.g0.y) + vec3(self.g0.y) * vec3(other.g0.z, 0.0, -other.g0.x) + vec3(self.g0.z) * vec3(-other.g0.y, other.g0.x, 0.0) + vec3(self.g0.w) * other.g0 + self.g1 * vec3(other.g1.x), vec2(self.g0.w) * other.g1 + vec2(self.g1.x) * vec2(0.0, other.g0.x) + vec2(self.g1.y) * vec2(0.0, other.g0.y) + vec2(self.g1.z) * vec2(0.0, other.g0.z), vec3(0.0), vec3(0.0), vec4(0.0), vec4(self.g0.x) * vec4(other.g1.x, 0.0, 0.0, -other.g0.x) + vec4(self.g0.y) * vec4(0.0, other.g1.x, 0.0, -other.g0.y) + vec4(self.g0.z) * vec4(0.0, 0.0, other.g1.x, -other.g0.z), self.g1 * vec3(other.g1.x), vec3(self.g0.x, self.g0.y, self.g0.z) * vec3(other.g1.y) + vec3(self.g1.x) * vec3(0.0, other.g0.z, -other.g0.y) + vec3(self.g1.y) * vec3(-other.g0.z, 0.0, other.g0.x) + vec3(self.g1.z) * vec3(other.g0.y, -other.g0.x, 0.0), vec3(0.0), vec2(0.0));
}

MultiVector motor__anti_wedge_dot__scalar(Motor self, Scalar other) {
    return MultiVector(vec2(self.g0.w) * vec2(other.g0, 0.0), vec3(0.0), vec2(0.0), vec3(0.0), vec3(self.g0.x, self.g0.y, self.g0.z) * vec3(other.g0), vec4(self.g1.x, self.g1.y, self.g1.z, self.g1.x) * vec4(other.g0, other.g0, other.g0, 0.0), vec4(0.0), vec3(0.0), vec3(0.0), vec3(0.0), vec2(0.0));
}

MultiVector motor__anti_wedge_dot__sphere(Motor self, Sphere other) {
    return MultiVector(vec2(0.0), vec3(0.0), vec2(0.0), vec3(self.g0.x, self.g0.y, self.g0.z) * vec3(other.g1.x), self.g1 * vec3(other.g1.x), vec4(self.g0.x) * vec4(other.g1.y, 0.0, 0.0, -other.g0.x) + vec4(self.g0.y) * vec4(0.0, other.g1.y, 0.0, -other.g0.y) + vec4(self.g0.z) * vec4(0.0, 0.0, other.g1.y, -other.g0.z) + vec4(self.g1.x) * vec4(0.0, -other.g0.z, other.g0.y, 0.0) + vec4(self.g1.y) * vec4(other.g0.z, 0.0, -other.g0.x, 0.0) + vec4(self.g1.z) * vec4(-other.g0.y, other.g0.x, 0.0, 0.0), vec4(0.0), vec3(0.0), vec3(0.0), vec3(self.g0.x) * vec3(0.0, -other.g0.z, other.g0.y) + vec3(self.g0.y) * vec3(other.g0.z, 0.0, -other.g0.x) + vec3(self.g0.z) * vec3(-other.g0.y, other.g0.x, 0.0) + vec3(self.g0.w) * other.g0 - self.g1 * vec3(other.g1.x), vec2(self.g0.w) * other.g1 + vec2(self.g1.x) * vec2(0.0, -other.g0.x) + vec2(self.g1.y) * vec2(0.0, -other.g0.y) + vec2(self.g1.z) * vec2(0.0, -other.g0.z));
}

MultiVector multi_vector__anti_wedge_dot__anti_scalar(MultiVector self, AntiScalar other) {
    return MultiVector(self.g0 * vec2(other.g0), self.g1 * vec3(other.g0), self.g2 * vec2(other.g0), self.g3 * vec3(other.g0), self.g4 * vec3(other.g0), self.g5 * vec4(other.g0), self.g6 * vec4(other.g0), self.g7 * vec3(other.g0), self.g8 * vec3(other.g0), self.g9 * vec3(other.g0), self.g10 * vec2(other.g0));
}

MultiVector multi_vector__anti_wedge_dot__circle(MultiVector self, Circle other) {
    return MultiVector(vec2(self.g3.x) * vec2(-other.g2.x, 0.0) + vec2(self.g3.y) * vec2(-other.g2.y, 0.0) + vec2(self.g3.z) * vec2(-other.g2.z, 0.0) + vec2(self.g4.x) * vec2(-other.g1.x, 0.0) + vec2(self.g4.y) * vec2(-other.g1.y, 0.0) + vec2(self.g4.z) * vec2(-other.g1.z, 0.0) + vec2(self.g5.x) * vec2(-other.g0.x, 0.0) + vec2(self.g5.y) * vec2(-other.g0.y, 0.0) + vec2(self.g5.z) * vec2(-other.g0.z, 0.0) + vec2(self.g5.w) * vec2(-other.g0.w, 0.0) + vec2(self.g6.x) * vec2(0.0, -other.g2.x) + vec2(self.g6.y) * vec2(0.0, -other.g2.y) + vec2(self.g6.z) * vec2(0.0, -other.g2.z) + vec2(self.g6.w) * vec2(0.0, other.g0.w) + vec2(self.g7.x) * vec2(0.0, -other.g1.x) + vec2(self.g7.y) * vec2(0.0, -other.g1.y) + vec2(self.g7.z) * vec2(0.0, -other.g1.z) + vec2(self.g8.x) * vec2(0.0, -other.g0.x) + vec2(self.g8.y) * vec2(0.0, -other.g0.y) + vec2(self.g8.z) * vec2(0.0, -other.g0.z), vec3(self.g1.x) * vec3(0.0, -other.g1.z, other.g1.y) + vec3(self.g1.y) * vec3(other.g1.z, 0.0, -other.g1.x) + vec3(self.g1.z) * vec3(-other.g1.y, other.g1.x, 0.0) - vec3(self.g2.x) * other.g2 + vec3(self.g2.y) * vec3(other.g0.x, other.g0.y, other.g0.z) + vec3(self.g6.x) * vec3(0.0, other.g2.z, -other.g2.y) + vec3(self.g6.y) * vec3(-other.g2.z, 0.0, other.g2.x) + vec3(self.g6.z) * vec3(other.g2.y, -other.g2.x, 0.0) + vec3(self.g6.w) * other.g1 + self.g7 * vec3(other.g0.w) + vec3(self.g8.x) * vec3(0.0, -other.g0.z, other.g0.y) + vec3(self.g8.y) * vec3(other.g0.z, 0.0, -other.g0.x) + vec3(self.g8.z) * vec3(-other.g0.y, other.g0.x, 0.0), vec2(self.g1.x) * vec2(other.g0.x, -other.g2.x) + vec2(self.g1.y) * vec2(other.g0.y, -other.g2.y) + vec2(self.g1.z) * vec2(other.g0.z, -other.g2.z) + self.g2 * vec2(other.g0.w) + vec2(self.g6.x) * vec2(-other.g1.x, 0.0) + vec2(self.g6.y) * vec2(-other.g1.y, 0.0) + vec2(self.g6.z) * vec2(-other.g1.z, 0.0) - vec2(self.g7.x) * vec2(other.g0.x, other.g2.x) - vec2(self.g7.y) * vec2(other.g0.y, other.g2.y) - vec2(self.g7.z) * vec2(other.g0.z, other.g2.z) + vec2(self.g8.x) * vec2(0.0, -other.g1.x) + vec2(self.g8.y) * vec2(0.0, -other.g1.y) + vec2(self.g8.z) * vec2(0.0, -other.g1.z), vec3(self.g0.x) * vec3(other.g0.x, other.g0.y, other.g0.z) + vec3(self.g3.x) * vec3(other.g0.w, -other.g1.z, other.g1.y) + vec3(self.g3.y) * vec3(other.g1.z, other.g0.w, -other.g1.x) + vec3(self.g3.z) * vec3(-other.g1.y, other.g1.x, other.g0.w) + vec3(self.g4.x) * vec3(0.0, -other.g0.z, other.g0.y) + vec3(self.g4.y) * vec3(other.g0.z, 0.0, -other.g0.x) + vec3(self.g4.z) * vec3(-other.g0.y, other.g0.x, 0.0) + vec3(self.g5.w) * vec3(other.g0.x, other.g0.y, other.g0.z) + vec3(self.g9.x) * vec3(0.0, -other.g0.z, other.g0.y) + vec3(self.g9.y) * vec3(other.g0.z, 0.0, -other.g0.x) + vec3(self.g9.z) * vec3(-other.g0.y, other.g0.x, 0.0) + vec3(self.g10.x) * other.g1, vec3(self.g0.x) * other.g1 + vec3(self.g3.x) * vec3(0.0, -other.g2.z, other.g2.y) + vec3(self.g3.y) * vec3(other.g2.z, 0.0, -other.g2.x) + vec3(self.g3.z) * vec3(-other.g2.y, other.g2.x, 0.0) + vec3(self.g4.x) * vec3(0.0, -other.g1.z, other.g1.y) + vec3(self.g4.y) * vec3(other.g1.z, 0.0, -other.g1.x) + vec3(self.g4.z) * vec3(-other.g1.y, other.g1.x, 0.0) + vec3(self.g5.x) * vec3(0.0, -other.g0.z, other.g0.y) + vec3(self.g5.y) * vec3(other.g0.z, 0.0, -other.g0.x) + vec3(self.g5.z) * vec3(-other.g0.y, other.g0.x, 0.0) - self.g9 * vec3(other.g0.w) + vec3(self.g10.x) * other.g2 + vec3(self.g10.y) * vec3(other.g0.x, other.g0.y, other.g0.z), vec4(self.g0.x) * vec4(other.g2.x, other.g2.y, other.g2.z, -other.g0.w) + vec4(self.g3.x) * vec4(0.0, 0.0, 0.0, -other.g2.x) + vec4(self.g3.y) * vec4(0.0, 0.0, 0.0, -other.g2.y) + vec4(self.g3.z) * vec4(0.0, 0.0, 0.0, -other.g2.z) + vec4(self.g4.x) * vec4(0.0, -other.g2.z, other.g2.y, 0.0) + vec4(self.g4.y) * vec4(other.g2.z, 0.0, -other.g2.x, 0.0) + vec4(self.g4.z) * vec4(-other.g2.y, other.g2.x, 0.0, 0.0) + vec4(self.g5.x) * vec4(-other.g0.w, -other.g1.z, other.g1.y, other.g0.x) + vec4(self.g5.y) * vec4(other.g1.z, -other.g0.w, -other.g1.x, other.g0.y) + vec4(self.g5.z) * vec4(-other.g1.y, other.g1.x, -other.g0.w, other.g0.z) + vec4(self.g5.w) * vec4(-other.g2.x, -other.g2.y, -other.g2.z, 0.0) + vec4(self.g9.x) * vec4(0.0, other.g2.z, -other.g2.y, -other.g1.x) + vec4(self.g9.y) * vec4(-other.g2.z, 0.0, other.g2.x, -other.g1.y) + vec4(self.g9.z) * vec4(other.g2.y, -other.g2.x, 0.0, -other.g1.z) + vec4(self.g10.y) * vec4(other.g1.x, other.g1.y, other.g1.z, 0.0), vec4(self.g0.y) * other.g0 + vec4(self.g1.x) * vec4(0.0, other.g0.z, -other.g0.y, -other.g1.x) + vec4(self.g1.y) * vec4(-other.g0.z, 0.0, other.g0.x, -other.g1.y) + vec4(self.g1.z) * vec4(other.g0.y, -other.g0.x, 0.0, -other.g1.z) + vec4(self.g2.x) * vec4(other.g1.x, other.g1.y, other.g1.z, 0.0) + vec4(self.g6.x) * vec4(other.g0.w, -other.g1.z, other.g1.y, other.g2.x) + vec4(self.g6.y) * vec4(other.g1.z, other.g0.w, -other.g1.x, other.g2.y) + vec4(self.g6.z) * vec4(-other.g1.y, other.g1.x, other.g0.w, other.g2.z) + vec4(self.g6.w) * vec4(-other.g0.x, -other.g0.y, -other.g0.z, 0.0) + vec4(self.g7.x) * vec4(0.0, -other.g0.z, other.g0.y, 0.0) + vec4(self.g7.y) * vec4(other.g0.z, 0.0, -other.g0.x, 0.0) + vec4(self.g7.z) * vec4(-other.g0.y, other.g0.x, 0.0, 0.0) + vec4(self.g8.x) * vec4(0.0, 0.0, 0.0, -other.g0.x) + vec4(self.g8.y) * vec4(0.0, 0.0, 0.0, -other.g0.y) + vec4(self.g8.z) * vec4(0.0, 0.0, 0.0, -other.g0.z), vec3(self.g0.y) * other.g1 + self.g1 * vec3(other.g0.w) + vec3(self.g2.x) * other.g2 + vec3(self.g2.y) * vec3(other.g0.x, other.g0.y, other.g0.z) + vec3(self.g6.x) * vec3(0.0, -other.g2.z, other.g2.y) + vec3(self.g6.y) * vec3(other.g2.z, 0.0, -other.g2.x) + vec3(self.g6.z) * vec3(-other.g2.y, other.g2.x, 0.0) + vec3(self.g7.x) * vec3(0.0, -other.g1.z, other.g1.y) + vec3(self.g7.y) * vec3(other.g1.z, 0.0, -other.g1.x) + vec3(self.g7.z) * vec3(-other.g1.y, other.g1.x, 0.0) + vec3(self.g8.x) * vec3(0.0, -other.g0.z, other.g0.y) + vec3(self.g8.y) * vec3(other.g0.z, 0.0, -other.g0.x) + vec3(self.g8.z) * vec3(-other.g0.y, other.g0.x, 0.0), vec3(self.g0.y) * other.g2 + vec3(self.g1.x) * vec3(0.0, -other.g2.z, other.g2.y) + vec3(self.g1.y) * vec3(other.g2.z, 0.0, -other.g2.x) + vec3(self.g1.z) * vec3(-other.g2.y, other.g2.x, 0.0) + vec3(self.g2.y) * other.g1 + vec3(self.g6.w) * other.g2 + vec3(self.g7.x) * vec3(0.0, -other.g2.z, other.g2.y) + vec3(self.g7.y) * vec3(other.g2.z, 0.0, -other.g2.x) + vec3(self.g7.z) * vec3(-other.g2.y, other.g2.x, 0.0) + vec3(self.g8.x) * vec3(-other.g0.w, -other.g1.z, other.g1.y) + vec3(self.g8.y) * vec3(other.g1.z, -other.g0.w, -other.g1.x) + vec3(self.g8.z) * vec3(-other.g1.y, other.g1.x, -other.g0.w), vec3(self.g3.x) * vec3(0.0, -other.g2.z, other.g2.y) + vec3(self.g3.y) * vec3(other.g2.z, 0.0, -other.g2.x) + vec3(self.g3.z) * vec3(-other.g2.y, other.g2.x, 0.0) - self.g4 * vec3(other.g0.w) + vec3(self.g5.x) * vec3(0.0, other.g0.z, -other.g0.y) + vec3(self.g5.y) * vec3(-other.g0.z, 0.0, other.g0.x) + vec3(self.g5.z) * vec3(other.g0.y, -other.g0.x, 0.0) + vec3(self.g5.w) * other.g1 + vec3(self.g9.x) * vec3(0.0, -other.g1.z, other.g1.y) + vec3(self.g9.y) * vec3(other.g1.z, 0.0, -other.g1.x) + vec3(self.g9.z) * vec3(-other.g1.y, other.g1.x, 0.0) + vec3(self.g10.x) * other.g2 - vec3(self.g10.y) * vec3(other.g0.x, other.g0.y, other.g0.z), vec2(self.g3.x) * vec2(-other.g1.x, 0.0) + vec2(self.g3.y) * vec2(-other.g1.y, 0.0) + vec2(self.g3.z) * vec2(-other.g1.z, 0.0) - vec2(self.g4.x) * vec2(other.g0.x, other.g2.x) - vec2(self.g4.y) * vec2(other.g0.y, other.g2.y) - vec2(self.g4.z) * vec2(other.g0.z, other.g2.z) + vec2(self.g5.x) * vec2(0.0, -other.g1.x) + vec2(self.g5.y) * vec2(0.0, -other.g1.y) + vec2(self.g5.z) * vec2(0.0, -other.g1.z) + vec2(self.g9.x) * vec2(-other.g0.x, other.g2.x) + vec2(self.g9.y) * vec2(-other.g0.y, other.g2.y) + vec2(self.g9.z) * vec2(-other.g0.z, other.g2.z) + self.g10 * vec2(other.g0.w));
}

MultiVector multi_vector__anti_wedge_dot__dipole(MultiVector self, Dipole other) {
    return MultiVector(vec2(self.g3.x) * vec2(0.0, other.g2.x) + vec2(self.g3.y) * vec2(0.0, other.g2.y) + vec2(self.g3.z) * vec2(0.0, other.g2.z) + vec2(self.g4.x) * vec2(0.0, other.g1.x) + vec2(self.g4.y) * vec2(0.0, other.g1.y) + vec2(self.g4.z) * vec2(0.0, other.g1.z) + vec2(self.g5.x) * vec2(0.0, other.g0.x) + vec2(self.g5.y) * vec2(0.0, other.g0.y) + vec2(self.g5.z) * vec2(0.0, other.g0.z) + vec2(self.g5.w) * vec2(0.0, -other.g2.w) + vec2(self.g6.x) * vec2(-other.g2.x, 0.0) + vec2(self.g6.y) * vec2(-other.g2.y, 0.0) + vec2(self.g6.z) * vec2(-other.g2.z, 0.0) + vec2(self.g6.w) * vec2(-other.g2.w, 0.0) + vec2(self.g7.x) * vec2(-other.g1.x, 0.0) + vec2(self.g7.y) * vec2(-other.g1.y, 0.0) + vec2(self.g7.z) * vec2(-other.g1.z, 0.0) + vec2(self.g8.x) * vec2(-other.g0.x, 0.0) + vec2(self.g8.y) * vec2(-other.g0.y, 0.0) + vec2(self.g8.z) * vec2(-other.g0.z, 0.0), vec3(self.g3.x) * vec3(0.0, -other.g2.z, other.g2.y) + vec3(self.g3.y) * vec3(other.g2.z, 0.0, -other.g2.x) + vec3(self.g3.z) * vec3(-other.g2.y, other.g2.x, 0.0) + self.g4 * vec3(other.g2.w) + vec3(self.g5.x) * vec3(0.0, other.g0.z, -other.g0.y) + vec3(self.g5.y) * vec3(-other.g0.z, 0.0, other.g0.x) + vec3(self.g5.z) * vec3(other.g0.y, -other.g0.x, 0.0) + vec3(self.g5.w) * other.g1 + vec3(self.g9.x) * vec3(0.0, -other.g1.z, other.g1.y) + vec3(self.g9.y) * vec3(other.g1.z, 0.0, -other.g1.x) + vec3(self.g9.z) * vec3(-other.g1.y, other.g1.x, 0.0) + vec3(self.g10.x) * vec3(other.g2.x, other.g2.y, other.g2.z) - vec3(self.g10.y) * other.g0, vec2(self.g3.x) * vec2(other.g1.x, 0.0) + vec2(self.g3.y) * vec2(other.g1.y, 0.0) + vec2(self.g3.z) * vec2(other.g1.z, 0.0) + vec2(self.g4.x) * vec2(other.g0.x, other.g2.x) + vec2(self.g4.y) * vec2(other.g0.y, other.g2.y) + vec2(self.g4.z) * vec2(other.g0.z, other.g2.z) + vec2(self.g5.x) * vec2(0.0, other.g1.x) + vec2(self.g5.y) * vec2(0.0, other.g1.y) + vec2(self.g5.z) * vec2(0.0, other.g1.z) + vec2(self.g9.x) * vec2(other.g0.x, -other.g2.x) + vec2(self.g9.y) * vec2(other.g0.y, -other.g2.y) + vec2(self.g9.z) * vec2(other.g0.z, -other.g2.z) + self.g10 * vec2(other.g2.w), vec3(self.g0.y) * other.g0 + vec3(self.g1.x) * vec3(0.0, other.g0.z, -other.g0.y) + vec3(self.g1.y) * vec3(-other.g0.z, 0.0, other.g0.x) + vec3(self.g1.z) * vec3(other.g0.y, -other.g0.x, 0.0) + vec3(self.g2.x) * other.g1 + vec3(self.g6.x) * vec3(-other.g2.w, -other.g1.z, other.g1.y) + vec3(self.g6.y) * vec3(other.g1.z, -other.g2.w, -other.g1.x) + vec3(self.g6.z) * vec3(-other.g1.y, other.g1.x, -other.g2.w) - vec3(self.g6.w) * other.g0 + vec3(self.g7.x) * vec3(0.0, -other.g0.z, other.g0.y) + vec3(self.g7.y) * vec3(other.g0.z, 0.0, -other.g0.x) + vec3(self.g7.z) * vec3(-other.g0.y, other.g0.x, 0.0), vec3(self.g0.y) * other.g1 - self.g1 * vec3(other.g2.w) + vec3(self.g2.x) * vec3(other.g2.x, other.g2.y, other.g2.z) + vec3(self.g2.y) * other.g0 + vec3(self.g6.x) * vec3(0.0, -other.g2.z, other.g2.y) + vec3(self.g6.y) * vec3(other.g2.z, 0.0, -other.g2.x) + vec3(self.g6.z) * vec3(-other.g2.y, other.g2.x, 0.0) + vec3(self.g7.x) * vec3(0.0, -other.g1.z, other.g1.y) + vec3(self.g7.y) * vec3(other.g1.z, 0.0, -other.g1.x) + vec3(self.g7.z) * vec3(-other.g1.y, other.g1.x, 0.0) + vec3(self.g8.x) * vec3(0.0, -other.g0.z, other.g0.y) + vec3(self.g8.y) * vec3(other.g0.z, 0.0, -other.g0.x) + vec3(self.g8.z) * vec3(-other.g0.y, other.g0.x, 0.0), vec4(self.g0.y) * other.g2 + vec4(self.g1.x) * vec4(0.0, -other.g2.z, other.g2.y, other.g1.x) + vec4(self.g1.y) * vec4(other.g2.z, 0.0, -other.g2.x, other.g1.y) + vec4(self.g1.z) * vec4(-other.g2.y, other.g2.x, 0.0, other.g1.z) + vec4(self.g2.y) * vec4(other.g1.x, other.g1.y, other.g1.z, 0.0) + vec4(self.g6.x) * vec4(0.0, 0.0, 0.0, -other.g2.x) + vec4(self.g6.y) * vec4(0.0, 0.0, 0.0, -other.g2.y) + self.g6.wwwz * other.g2.xyzz * vec4(1.0, 1.0, 1.0, -1.0) + vec4(self.g7.x) * vec4(0.0, -other.g2.z, other.g2.y, 0.0) + vec4(self.g7.y) * vec4(other.g2.z, 0.0, -other.g2.x, 0.0) + vec4(self.g7.z) * vec4(-other.g2.y, other.g2.x, 0.0, 0.0) + vec4(self.g8.x) * vec4(other.g2.w, -other.g1.z, other.g1.y, other.g0.x) + vec4(self.g8.y) * vec4(other.g1.z, other.g2.w, -other.g1.x, other.g0.y) + vec4(self.g8.z) * vec4(-other.g1.y, other.g1.x, other.g2.w, other.g0.z), vec4(self.g0.x) * vec4(-other.g0.x, -other.g0.y, -other.g0.z, other.g2.w) + vec4(self.g3.x) * vec4(other.g2.w, other.g1.z, -other.g1.y, -other.g2.x) + vec4(self.g3.y) * vec4(-other.g1.z, other.g2.w, other.g1.x, -other.g2.y) + vec4(self.g3.z) * vec4(other.g1.y, -other.g1.x, other.g2.w, -other.g2.z) + vec4(self.g4.x) * vec4(0.0, other.g0.z, -other.g0.y, 0.0) + vec4(self.g4.y) * vec4(-other.g0.z, 0.0, other.g0.x, 0.0) + vec4(self.g4.z) * vec4(other.g0.y, -other.g0.x, 0.0, 0.0) + vec4(self.g5.x) * vec4(0.0, 0.0, 0.0, other.g0.x) + vec4(self.g5.y) * vec4(0.0, 0.0, 0.0, other.g0.y) + self.g5.wwwz * vec4(-other.g0.x, -other.g0.y, -other.g0.z, other.g0.z) + vec4(self.g9.x) * vec4(0.0, other.g0.z, -other.g0.y, -other.g1.x) + vec4(self.g9.y) * vec4(-other.g0.z, 0.0, other.g0.x, -other.g1.y) + vec4(self.g9.z) * vec4(other.g0.y, -other.g0.x, 0.0, -other.g1.z) + vec4(self.g10.x) * vec4(-other.g1.x, -other.g1.y, -other.g1.z, 0.0), vec3(0.0) - vec3(self.g0.x) * other.g1 + vec3(self.g3.x) * vec3(0.0, other.g2.z, -other.g2.y) + vec3(self.g3.y) * vec3(-other.g2.z, 0.0, other.g2.x) + vec3(self.g3.z) * vec3(other.g2.y, -other.g2.x, 0.0) + vec3(self.g4.x) * vec3(0.0, other.g1.z, -other.g1.y) + vec3(self.g4.y) * vec3(-other.g1.z, 0.0, other.g1.x) + vec3(self.g4.z) * vec3(other.g1.y, -other.g1.x, 0.0) + vec3(self.g5.x) * vec3(0.0, other.g0.z, -other.g0.y) + vec3(self.g5.y) * vec3(-other.g0.z, 0.0, other.g0.x) + vec3(self.g5.z) * vec3(other.g0.y, -other.g0.x, 0.0) - self.g9 * vec3(other.g2.w) - vec3(self.g10.x) * vec3(other.g2.x, other.g2.y, other.g2.z) - vec3(self.g10.y) * other.g0, vec3(0.0) - vec3(self.g0.x) * vec3(other.g2.x, other.g2.y, other.g2.z) + vec3(self.g4.x) * vec3(0.0, other.g2.z, -other.g2.y) + vec3(self.g4.y) * vec3(-other.g2.z, 0.0, other.g2.x) + vec3(self.g4.z) * vec3(other.g2.y, -other.g2.x, 0.0) + vec3(self.g5.x) * vec3(-other.g2.w, other.g1.z, -other.g1.y) + vec3(self.g5.y) * vec3(-other.g1.z, -other.g2.w, other.g1.x) + vec3(self.g5.z) * vec3(other.g1.y, -other.g1.x, -other.g2.w) + vec3(self.g5.w) * vec3(other.g2.x, other.g2.y, other.g2.z) + vec3(self.g9.x) * vec3(0.0, -other.g2.z, other.g2.y) + vec3(self.g9.y) * vec3(other.g2.z, 0.0, -other.g2.x) + vec3(self.g9.z) * vec3(-other.g2.y, other.g2.x, 0.0) - vec3(self.g10.y) * other.g1, vec3(self.g1.x) * vec3(0.0, other.g1.z, -other.g1.y) + vec3(self.g1.y) * vec3(-other.g1.z, 0.0, other.g1.x) + vec3(self.g1.z) * vec3(other.g1.y, -other.g1.x, 0.0) + vec3(self.g2.x) * vec3(other.g2.x, other.g2.y, other.g2.z) - vec3(self.g2.y) * other.g0 + vec3(self.g6.x) * vec3(0.0, -other.g2.z, other.g2.y) + vec3(self.g6.y) * vec3(other.g2.z, 0.0, -other.g2.x) + vec3(self.g6.z) * vec3(-other.g2.y, other.g2.x, 0.0) - vec3(self.g6.w) * other.g1 + self.g7 * vec3(other.g2.w) + vec3(self.g8.x) * vec3(0.0, other.g0.z, -other.g0.y) + vec3(self.g8.y) * vec3(-other.g0.z, 0.0, other.g0.x) + vec3(self.g8.z) * vec3(other.g0.y, -other.g0.x, 0.0), vec2(self.g1.x) * vec2(other.g0.x, -other.g2.x) + vec2(self.g1.y) * vec2(other.g0.y, -other.g2.y) + vec2(self.g1.z) * vec2(other.g0.z, -other.g2.z) + self.g2 * vec2(-other.g2.w) + vec2(self.g6.x) * vec2(-other.g1.x, 0.0) + vec2(self.g6.y) * vec2(-other.g1.y, 0.0) + vec2(self.g6.z) * vec2(-other.g1.z, 0.0) - vec2(self.g7.x) * vec2(other.g0.x, other.g2.x) - vec2(self.g7.y) * vec2(other.g0.y, other.g2.y) - vec2(self.g7.z) * vec2(other.g0.z, other.g2.z) + vec2(self.g8.x) * vec2(0.0, -other.g1.x) + vec2(self.g8.y) * vec2(0.0, -other.g1.y) + vec2(self.g8.z) * vec2(0.0, -other.g1.z));
}

MultiVector multi_vector__anti_wedge_dot__dual_num(MultiVector self, DualNum other) {
    return MultiVector(vec2(self.g0.x) * other.g0.yx * vec2(1.0, -1.0) + vec2(self.g0.y) * other.g0, self.g1 * vec3(other.g0.y) + self.g9 * vec3(other.g0.x), self.g2 * vec2(other.g0.y) - self.g10 * vec2(other.g0.x), self.g3 * vec3(other.g0.y) + vec3(self.g6.x, self.g6.y, self.g6.z) * vec3(other.g0.x), self.g4 * vec3(other.g0.y) + self.g7 * vec3(other.g0.x), self.g5 * vec4(other.g0.y) + vec4(self.g6.w) * vec4(0.0, 0.0, 0.0, -other.g0.x) + vec4(self.g8.x, self.g8.y, self.g8.z, self.g8.x) * vec4(other.g0.x, other.g0.x, other.g0.x, 0.0), vec4(self.g3.x, self.g3.y, self.g3.z, self.g3.x) * vec4(-other.g0.x, -other.g0.x, -other.g0.x, 0.0) + vec4(self.g5.w) * vec4(0.0, 0.0, 0.0, other.g0.x) + self.g6 * vec4(other.g0.y), vec3(0.0) - self.g4 * vec3(other.g0.x) + self.g7 * vec3(other.g0.y), vec3(0.0) - vec3(self.g5.x, self.g5.y, self.g5.z) * vec3(other.g0.x) + self.g8 * vec3(other.g0.y), vec3(0.0) - self.g1 * vec3(other.g0.x) + self.g9 * vec3(other.g0.y), self.g2 * vec2(other.g0.x) + self.g10 * vec2(other.g0.y));
}

MultiVector multi_vector__anti_wedge_dot__flat_point(MultiVector self, FlatPoint other) {
    return MultiVector(vec2(self.g3.x) * vec2(0.0, other.g0.x) + vec2(self.g3.y) * vec2(0.0, other.g0.y) + vec2(self.g3.z) * vec2(0.0, other.g0.z) + vec2(self.g5.w) * vec2(0.0, -other.g0.w) + vec2(self.g6.x) * vec2(-other.g0.x, 0.0) + vec2(self.g6.y) * vec2(-other.g0.y, 0.0) + vec2(self.g6.z) * vec2(-other.g0.z, 0.0) + vec2(self.g6.w) * vec2(-other.g0.w, 0.0), vec3(self.g3.x) * vec3(0.0, -other.g0.z, other.g0.y) + vec3(self.g3.y) * vec3(other.g0.z, 0.0, -other.g0.x) + vec3(self.g3.z) * vec3(-other.g0.y, other.g0.x, 0.0) + self.g4 * vec3(other.g0.w) + vec3(self.g10.x) * vec3(other.g0.x, other.g0.y, other.g0.z), vec2(self.g4.x) * vec2(0.0, other.g0.x) + vec2(self.g4.y) * vec2(0.0, other.g0.y) + vec2(self.g4.z) * vec2(0.0, other.g0.z) + vec2(self.g9.x) * vec2(0.0, -other.g0.x) + vec2(self.g9.y) * vec2(0.0, -other.g0.y) + vec2(self.g9.z) * vec2(0.0, -other.g0.z) + self.g10 * vec2(other.g0.w), vec3(0.0) - vec3(self.g6.x, self.g6.y, self.g6.z) * vec3(other.g0.w), vec3(0.0) - self.g1 * vec3(other.g0.w) + vec3(self.g2.x) * vec3(other.g0.x, other.g0.y, other.g0.z) + vec3(self.g6.x) * vec3(0.0, -other.g0.z, other.g0.y) + vec3(self.g6.y) * vec3(other.g0.z, 0.0, -other.g0.x) + vec3(self.g6.z) * vec3(-other.g0.y, other.g0.x, 0.0), vec4(self.g0.y) * other.g0 + vec4(self.g1.x) * vec4(0.0, -other.g0.z, other.g0.y, 0.0) + vec4(self.g1.y) * vec4(other.g0.z, 0.0, -other.g0.x, 0.0) + vec4(self.g1.z) * vec4(-other.g0.y, other.g0.x, 0.0, 0.0) + vec4(self.g6.x) * vec4(0.0, 0.0, 0.0, -other.g0.x) + vec4(self.g6.y) * vec4(0.0, 0.0, 0.0, -other.g0.y) + self.g6.wwwz * other.g0.xyzz * vec4(1.0, 1.0, 1.0, -1.0) + vec4(self.g7.x) * vec4(0.0, -other.g0.z, other.g0.y, 0.0) + vec4(self.g7.y) * vec4(other.g0.z, 0.0, -other.g0.x, 0.0) + vec4(self.g7.z) * vec4(-other.g0.y, other.g0.x, 0.0, 0.0) + vec4(self.g8.x, self.g8.y, self.g8.z, self.g8.x) * vec4(other.g0.w, other.g0.w, other.g0.w, 0.0), vec4(self.g0.x) * vec4(0.0, 0.0, 0.0, other.g0.w) + vec4(self.g3.x) * vec4(other.g0.w, 0.0, 0.0, -other.g0.x) + vec4(self.g3.y) * vec4(0.0, other.g0.w, 0.0, -other.g0.y) + vec4(self.g3.z) * vec4(0.0, 0.0, other.g0.w, -other.g0.z), vec3(self.g3.x) * vec3(0.0, other.g0.z, -other.g0.y) + vec3(self.g3.y) * vec3(-other.g0.z, 0.0, other.g0.x) + vec3(self.g3.z) * vec3(other.g0.y, -other.g0.x, 0.0) - self.g9 * vec3(other.g0.w) - vec3(self.g10.x) * vec3(other.g0.x, other.g0.y, other.g0.z), vec3(0.0) - vec3(self.g0.x) * vec3(other.g0.x, other.g0.y, other.g0.z) + vec3(self.g4.x) * vec3(0.0, other.g0.z, -other.g0.y) + vec3(self.g4.y) * vec3(-other.g0.z, 0.0, other.g0.x) + vec3(self.g4.z) * vec3(other.g0.y, -other.g0.x, 0.0) - vec3(self.g5.x, self.g5.y, self.g5.z) * vec3(other.g0.w) + vec3(self.g5.w) * vec3(other.g0.x, other.g0.y, other.g0.z) + vec3(self.g9.x) * vec3(0.0, -other.g0.z, other.g0.y) + vec3(self.g9.y) * vec3(other.g0.z, 0.0, -other.g0.x) + vec3(self.g9.z) * vec3(-other.g0.y, other.g0.x, 0.0), vec3(self.g2.x) * vec3(other.g0.x, other.g0.y, other.g0.z) + vec3(self.g6.x) * vec3(0.0, -other.g0.z, other.g0.y) + vec3(self.g6.y) * vec3(other.g0.z, 0.0, -other.g0.x) + vec3(self.g6.z) * vec3(-other.g0.y, other.g0.x, 0.0) + self.g7 * vec3(other.g0.w), vec2(self.g1.x) * vec2(0.0, -other.g0.x) + vec2(self.g1.y) * vec2(0.0, -other.g0.y) + vec2(self.g1.z) * vec2(0.0, -other.g0.z) + self.g2 * vec2(-other.g0.w) + vec2(self.g7.x) * vec2(0.0, -other.g0.x) + vec2(self.g7.y) * vec2(0.0, -other.g0.y) + vec2(self.g7.z) * vec2(0.0, -other.g0.z));
}

MultiVector multi_vector__anti_wedge_dot__flector(MultiVector self, Flector other) {
    return MultiVector(vec2(self.g1.x) * vec2(other.g1.x, 0.0) + vec2(self.g1.y) * vec2(other.g1.y, 0.0) + vec2(self.g1.z) * vec2(other.g1.z, 0.0) + vec2(self.g2.x) * vec2(other.g1.w, 0.0) + vec2(self.g3.x) * vec2(0.0, other.g0.x) + vec2(self.g3.y) * vec2(0.0, other.g0.y) + vec2(self.g3.z) * vec2(0.0, other.g0.z) + vec2(self.g5.w) * vec2(0.0, -other.g0.w) + vec2(self.g6.x) * vec2(-other.g0.x, 0.0) + vec2(self.g6.y) * vec2(-other.g0.y, 0.0) + vec2(self.g6.z) * vec2(-other.g0.z, 0.0) + vec2(self.g6.w) * vec2(-other.g0.w, 0.0) + vec2(self.g9.x) * vec2(0.0, other.g1.x) + vec2(self.g9.y) * vec2(0.0, other.g1.y) + vec2(self.g9.z) * vec2(0.0, other.g1.z) + vec2(self.g10.x) * vec2(0.0, -other.g1.w), vec3(self.g0.x) * vec3(other.g1.x, other.g1.y, other.g1.z) + vec3(self.g3.x) * vec3(other.g1.w, -other.g0.z, other.g0.y) + vec3(self.g3.y) * vec3(other.g0.z, other.g1.w, -other.g0.x) + vec3(self.g3.z) * vec3(-other.g0.y, other.g0.x, other.g1.w) + vec3(self.g4.x) * vec3(other.g0.w, -other.g1.z, other.g1.y) + vec3(self.g4.y) * vec3(other.g1.z, other.g0.w, -other.g1.x) + vec3(self.g4.z) * vec3(-other.g1.y, other.g1.x, other.g0.w) + vec3(self.g10.x) * vec3(other.g0.x, other.g0.y, other.g0.z), vec2(self.g0.x) * vec2(0.0, -other.g1.w) + vec2(self.g3.x) * vec2(-other.g1.x, 0.0) + vec2(self.g3.y) * vec2(-other.g1.y, 0.0) + vec2(self.g3.z) * vec2(-other.g1.z, 0.0) + vec2(self.g4.x) * vec2(0.0, other.g0.x) + vec2(self.g4.y) * vec2(0.0, other.g0.y) + vec2(self.g4.z) * vec2(0.0, other.g0.z) + vec2(self.g5.x) * vec2(0.0, other.g1.x) + vec2(self.g5.y) * vec2(0.0, other.g1.y) + vec2(self.g5.z) * vec2(0.0, other.g1.z) + vec2(self.g5.w) * vec2(0.0, other.g1.w) + vec2(self.g9.x) * vec2(0.0, -other.g0.x) + vec2(self.g9.y) * vec2(0.0, -other.g0.y) + vec2(self.g9.z) * vec2(0.0, -other.g0.z) + self.g10 * vec2(other.g0.w), vec3(0.0) - vec3(self.g2.x) * vec3(other.g1.x, other.g1.y, other.g1.z) + vec3(self.g6.x) * vec3(-other.g0.w, other.g1.z, -other.g1.y) + vec3(self.g6.y) * vec3(-other.g1.z, -other.g0.w, other.g1.x) + vec3(self.g6.z) * vec3(other.g1.y, -other.g1.x, -other.g0.w), vec3(self.g1.x) * vec3(-other.g0.w, other.g1.z, -other.g1.y) + vec3(self.g1.y) * vec3(-other.g1.z, -other.g0.w, other.g1.x) + vec3(self.g1.z) * vec3(other.g1.y, -other.g1.x, -other.g0.w) + vec3(self.g2.x) * vec3(other.g0.x, other.g0.y, other.g0.z) + vec3(self.g6.x) * vec3(other.g1.w, -other.g0.z, other.g0.y) + vec3(self.g6.y) * vec3(other.g0.z, other.g1.w, -other.g0.x) + vec3(self.g6.z) * vec3(-other.g0.y, other.g0.x, other.g1.w) - vec3(self.g6.w) * vec3(other.g1.x, other.g1.y, other.g1.z), vec4(self.g0.y) * other.g0 + vec4(self.g1.x) * vec4(other.g1.w, -other.g0.z, other.g0.y, 0.0) + vec4(self.g1.y) * vec4(other.g0.z, other.g1.w, -other.g0.x, 0.0) + vec4(self.g1.z) * vec4(-other.g0.y, other.g0.x, other.g1.w, 0.0) + vec4(self.g2.y, self.g2.y, self.g2.y, self.g2.x) * other.g1 + vec4(self.g6.x) * vec4(0.0, 0.0, 0.0, -other.g0.x) + vec4(self.g6.y) * vec4(0.0, 0.0, 0.0, -other.g0.y) + self.g6.wwwz * other.g0.xyzz * vec4(1.0, 1.0, 1.0, -1.0) + vec4(self.g7.x) * vec4(other.g1.w, -other.g0.z, other.g0.y, -other.g1.x) + vec4(self.g7.y) * vec4(other.g0.z, other.g1.w, -other.g0.x, -other.g1.y) + vec4(self.g7.z) * vec4(-other.g0.y, other.g0.x, other.g1.w, -other.g1.z) + vec4(self.g8.x) * vec4(other.g0.w, -other.g1.z, other.g1.y, 0.0) + vec4(self.g8.y) * vec4(other.g1.z, other.g0.w, -other.g1.x, 0.0) + vec4(self.g8.z) * vec4(-other.g1.y, other.g1.x, other.g0.w, 0.0), vec4(self.g0.x) * vec4(0.0, 0.0, 0.0, other.g0.w) + vec4(self.g3.x) * vec4(other.g0.w, -other.g1.z, other.g1.y, -other.g0.x) + vec4(self.g3.y) * vec4(other.g1.z, other.g0.w, -other.g1.x, -other.g0.y) + vec4(self.g3.z) * vec4(-other.g1.y, other.g1.x, other.g0.w, -other.g0.z) + vec4(self.g4.x) * vec4(0.0, 0.0, 0.0, -other.g1.x) + vec4(self.g4.y) * vec4(0.0, 0.0, 0.0, -other.g1.y) + vec4(self.g4.z) * vec4(0.0, 0.0, 0.0, -other.g1.z) + vec4(self.g10.x) * other.g1, vec3(self.g3.x) * vec3(-other.g1.w, other.g0.z, -other.g0.y) + vec3(self.g3.y) * vec3(-other.g0.z, -other.g1.w, other.g0.x) + vec3(self.g3.z) * vec3(other.g0.y, -other.g0.x, -other.g1.w) - vec3(self.g5.w) * vec3(other.g1.x, other.g1.y, other.g1.z) + vec3(self.g9.x) * vec3(-other.g0.w, other.g1.z, -other.g1.y) + vec3(self.g9.y) * vec3(-other.g1.z, -other.g0.w, other.g1.x) + vec3(self.g9.z) * vec3(other.g1.y, -other.g1.x, -other.g0.w) - vec3(self.g10.x) * vec3(other.g0.x, other.g0.y, other.g0.z), vec3(0.0) - vec3(self.g0.x) * vec3(other.g0.x, other.g0.y, other.g0.z) + vec3(self.g4.x) * vec3(-other.g1.w, other.g0.z, -other.g0.y) + vec3(self.g4.y) * vec3(-other.g0.z, -other.g1.w, other.g0.x) + vec3(self.g4.z) * vec3(other.g0.y, -other.g0.x, -other.g1.w) + vec3(self.g5.x) * vec3(-other.g0.w, other.g1.z, -other.g1.y) + vec3(self.g5.y) * vec3(-other.g1.z, -other.g0.w, other.g1.x) + vec3(self.g5.z) * vec3(other.g1.y, -other.g1.x, -other.g0.w) + vec3(self.g5.w) * vec3(other.g0.x, other.g0.y, other.g0.z) + vec3(self.g9.x) * vec3(other.g1.w, -other.g0.z, other.g0.y) + vec3(self.g9.y) * vec3(other.g0.z, other.g1.w, -other.g0.x) + vec3(self.g9.z) * vec3(-other.g0.y, other.g0.x, other.g1.w) - vec3(self.g10.y) * vec3(other.g1.x, other.g1.y, other.g1.z), vec3(self.g0.y) * vec3(other.g1.x, other.g1.y, other.g1.z) + vec3(self.g2.x) * vec3(other.g0.x, other.g0.y, other.g0.z) + vec3(self.g6.x) * vec3(other.g1.w, -other.g0.z, other.g0.y) + vec3(self.g6.y) * vec3(other.g0.z, other.g1.w, -other.g0.x) + vec3(self.g6.z) * vec3(-other.g0.y, other.g0.x, other.g1.w) + vec3(self.g7.x) * vec3(other.g0.w, -other.g1.z, other.g1.y) + vec3(self.g7.y) * vec3(other.g1.z, other.g0.w, -other.g1.x) + vec3(self.g7.z) * vec3(-other.g1.y, other.g1.x, other.g0.w), vec2(self.g0.y) * vec2(0.0, other.g1.w) + vec2(self.g1.x) * vec2(0.0, -other.g0.x) + vec2(self.g1.y) * vec2(0.0, -other.g0.y) + vec2(self.g1.z) * vec2(0.0, -other.g0.z) + self.g2 * vec2(-other.g0.w) + vec2(self.g6.x) * vec2(other.g1.x, 0.0) + vec2(self.g6.y) * vec2(other.g1.y, 0.0) + vec2(self.g6.z, self.g6.w) * vec2(other.g1.z, other.g1.w) + vec2(self.g7.x) * vec2(0.0, -other.g0.x) + vec2(self.g7.y) * vec2(0.0, -other.g0.y) + vec2(self.g7.z) * vec2(0.0, -other.g0.z) + vec2(self.g8.x) * vec2(0.0, -other.g1.x) + vec2(self.g8.y) * vec2(0.0, -other.g1.y) + vec2(self.g8.z) * vec2(0.0, -other.g1.z));
}

MultiVector multi_vector__anti_wedge_dot__line(MultiVector self, Line other) {
    return MultiVector(vec2(self.g3.x) * vec2(-other.g1.x, 0.0) + vec2(self.g3.y) * vec2(-other.g1.y, 0.0) + vec2(self.g3.z) * vec2(-other.g1.z, 0.0) + vec2(self.g4.x) * vec2(-other.g0.x, 0.0) + vec2(self.g4.y) * vec2(-other.g0.y, 0.0) + vec2(self.g4.z) * vec2(-other.g0.z, 0.0) + vec2(self.g6.x) * vec2(0.0, -other.g1.x) + vec2(self.g6.y) * vec2(0.0, -other.g1.y) + vec2(self.g6.z) * vec2(0.0, -other.g1.z) + vec2(self.g7.x) * vec2(0.0, -other.g0.x) + vec2(self.g7.y) * vec2(0.0, -other.g0.y) + vec2(self.g7.z) * vec2(0.0, -other.g0.z), vec3(self.g1.x) * vec3(0.0, -other.g0.z, other.g0.y) + vec3(self.g1.y) * vec3(other.g0.z, 0.0, -other.g0.x) + vec3(self.g1.z) * vec3(-other.g0.y, other.g0.x, 0.0) - vec3(self.g2.x) * other.g1 + vec3(self.g6.x) * vec3(0.0, other.g1.z, -other.g1.y) + vec3(self.g6.y) * vec3(-other.g1.z, 0.0, other.g1.x) + vec3(self.g6.z) * vec3(other.g1.y, -other.g1.x, 0.0) + vec3(self.g6.w) * other.g0, vec2(self.g1.x) * vec2(0.0, -other.g1.x) + vec2(self.g1.y) * vec2(0.0, -other.g1.y) + vec2(self.g1.z) * vec2(0.0, -other.g1.z) + vec2(self.g6.x) * vec2(-other.g0.x, 0.0) + vec2(self.g6.y) * vec2(-other.g0.y, 0.0) + vec2(self.g6.z) * vec2(-other.g0.z, 0.0) + vec2(self.g7.x) * vec2(0.0, -other.g1.x) + vec2(self.g7.y) * vec2(0.0, -other.g1.y) + vec2(self.g7.z) * vec2(0.0, -other.g1.z) + vec2(self.g8.x) * vec2(0.0, -other.g0.x) + vec2(self.g8.y) * vec2(0.0, -other.g0.y) + vec2(self.g8.z) * vec2(0.0, -other.g0.z), vec3(self.g3.x) * vec3(0.0, -other.g0.z, other.g0.y) + vec3(self.g3.y) * vec3(other.g0.z, 0.0, -other.g0.x) + vec3(self.g3.z) * vec3(-other.g0.y, other.g0.x, 0.0) + vec3(self.g10.x) * other.g0, vec3(self.g0.x) * other.g0 + vec3(self.g3.x) * vec3(0.0, -other.g1.z, other.g1.y) + vec3(self.g3.y) * vec3(other.g1.z, 0.0, -other.g1.x) + vec3(self.g3.z) * vec3(-other.g1.y, other.g1.x, 0.0) + vec3(self.g4.x) * vec3(0.0, -other.g0.z, other.g0.y) + vec3(self.g4.y) * vec3(other.g0.z, 0.0, -other.g0.x) + vec3(self.g4.z) * vec3(-other.g0.y, other.g0.x, 0.0) + vec3(self.g10.x) * other.g1, vec4(self.g0.x) * vec4(other.g1.x, other.g1.y, other.g1.z, 0.0) + vec4(self.g3.x) * vec4(0.0, 0.0, 0.0, -other.g1.x) + vec4(self.g3.y) * vec4(0.0, 0.0, 0.0, -other.g1.y) + vec4(self.g3.z) * vec4(0.0, 0.0, 0.0, -other.g1.z) + vec4(self.g4.x) * vec4(0.0, -other.g1.z, other.g1.y, 0.0) + vec4(self.g4.y) * vec4(other.g1.z, 0.0, -other.g1.x, 0.0) + vec4(self.g4.z) * vec4(-other.g1.y, other.g1.x, 0.0, 0.0) + vec4(self.g5.x) * vec4(0.0, -other.g0.z, other.g0.y, 0.0) + vec4(self.g5.y) * vec4(other.g0.z, 0.0, -other.g0.x, 0.0) + vec4(self.g5.z) * vec4(-other.g0.y, other.g0.x, 0.0, 0.0) + vec4(self.g5.w) * vec4(-other.g1.x, -other.g1.y, -other.g1.z, 0.0) + vec4(self.g9.x) * vec4(0.0, other.g1.z, -other.g1.y, -other.g0.x) + vec4(self.g9.y) * vec4(-other.g1.z, 0.0, other.g1.x, -other.g0.y) + vec4(self.g9.z) * vec4(other.g1.y, -other.g1.x, 0.0, -other.g0.z) + vec4(self.g10.y) * vec4(other.g0.x, other.g0.y, other.g0.z, 0.0), vec4(self.g1.x) * vec4(0.0, 0.0, 0.0, -other.g0.x) + vec4(self.g1.y) * vec4(0.0, 0.0, 0.0, -other.g0.y) + vec4(self.g1.z) * vec4(0.0, 0.0, 0.0, -other.g0.z) + vec4(self.g2.x) * vec4(other.g0.x, other.g0.y, other.g0.z, 0.0) + vec4(self.g6.x) * vec4(0.0, -other.g0.z, other.g0.y, other.g1.x) + vec4(self.g6.y) * vec4(other.g0.z, 0.0, -other.g0.x, other.g1.y) + vec4(self.g6.z) * vec4(-other.g0.y, other.g0.x, 0.0, other.g1.z), vec3(self.g0.y) * other.g0 + vec3(self.g2.x) * other.g1 + vec3(self.g6.x) * vec3(0.0, -other.g1.z, other.g1.y) + vec3(self.g6.y) * vec3(other.g1.z, 0.0, -other.g1.x) + vec3(self.g6.z) * vec3(-other.g1.y, other.g1.x, 0.0) + vec3(self.g7.x) * vec3(0.0, -other.g0.z, other.g0.y) + vec3(self.g7.y) * vec3(other.g0.z, 0.0, -other.g0.x) + vec3(self.g7.z) * vec3(-other.g0.y, other.g0.x, 0.0), vec3(self.g0.y) * other.g1 + vec3(self.g1.x) * vec3(0.0, -other.g1.z, other.g1.y) + vec3(self.g1.y) * vec3(other.g1.z, 0.0, -other.g1.x) + vec3(self.g1.z) * vec3(-other.g1.y, other.g1.x, 0.0) + vec3(self.g2.y) * other.g0 + vec3(self.g6.w) * other.g1 + vec3(self.g7.x) * vec3(0.0, -other.g1.z, other.g1.y) + vec3(self.g7.y) * vec3(other.g1.z, 0.0, -other.g1.x) + vec3(self.g7.z) * vec3(-other.g1.y, other.g1.x, 0.0) + vec3(self.g8.x) * vec3(0.0, -other.g0.z, other.g0.y) + vec3(self.g8.y) * vec3(other.g0.z, 0.0, -other.g0.x) + vec3(self.g8.z) * vec3(-other.g0.y, other.g0.x, 0.0), vec3(self.g3.x) * vec3(0.0, -other.g1.z, other.g1.y) + vec3(self.g3.y) * vec3(other.g1.z, 0.0, -other.g1.x) + vec3(self.g3.z) * vec3(-other.g1.y, other.g1.x, 0.0) + vec3(self.g5.w) * other.g0 + vec3(self.g9.x) * vec3(0.0, -other.g0.z, other.g0.y) + vec3(self.g9.y) * vec3(other.g0.z, 0.0, -other.g0.x) + vec3(self.g9.z) * vec3(-other.g0.y, other.g0.x, 0.0) + vec3(self.g10.x) * other.g1, vec2(self.g3.x) * vec2(-other.g0.x, 0.0) + vec2(self.g3.y) * vec2(-other.g0.y, 0.0) + vec2(self.g3.z) * vec2(-other.g0.z, 0.0) + vec2(self.g4.x) * vec2(0.0, -other.g1.x) + vec2(self.g4.y) * vec2(0.0, -other.g1.y) + vec2(self.g4.z) * vec2(0.0, -other.g1.z) + vec2(self.g5.x) * vec2(0.0, -other.g0.x) + vec2(self.g5.y) * vec2(0.0, -other.g0.y) + vec2(self.g5.z) * vec2(0.0, -other.g0.z) + vec2(self.g9.x) * vec2(0.0, other.g1.x) + vec2(self.g9.y) * vec2(0.0, other.g1.y) + vec2(self.g9.z) * vec2(0.0, other.g1.z));
}

MultiVector multi_vector__anti_wedge_dot__motor(MultiVector self, Motor other) {
    return MultiVector(self.g0 * vec2(other.g0.w) + vec2(self.g3.x) * vec2(-other.g1.x, 0.0) + vec2(self.g3.y) * vec2(-other.g1.y, 0.0) + vec2(self.g3.z) * vec2(-other.g1.z, 0.0) + vec2(self.g4.x) * vec2(-other.g0.x, 0.0) + vec2(self.g4.y) * vec2(-other.g0.y, 0.0) + vec2(self.g4.z) * vec2(-other.g0.z, 0.0) + vec2(self.g6.x) * vec2(0.0, -other.g1.x) + vec2(self.g6.y) * vec2(0.0, -other.g1.y) + vec2(self.g6.z) * vec2(0.0, -other.g1.z) + vec2(self.g7.x) * vec2(0.0, -other.g0.x) + vec2(self.g7.y) * vec2(0.0, -other.g0.y) + vec2(self.g7.z) * vec2(0.0, -other.g0.z), vec3(self.g1.x) * vec3(other.g0.w, -other.g0.z, other.g0.y) + vec3(self.g1.y) * vec3(other.g0.z, other.g0.w, -other.g0.x) + vec3(self.g1.z) * vec3(-other.g0.y, other.g0.x, other.g0.w) - vec3(self.g2.x) * other.g1 + vec3(self.g6.x) * vec3(0.0, other.g1.z, -other.g1.y) + vec3(self.g6.y) * vec3(-other.g1.z, 0.0, other.g1.x) + vec3(self.g6.z) * vec3(other.g1.y, -other.g1.x, 0.0) + vec3(self.g6.w) * vec3(other.g0.x, other.g0.y, other.g0.z), vec2(self.g1.x) * vec2(0.0, -other.g1.x) + vec2(self.g1.y) * vec2(0.0, -other.g1.y) + vec2(self.g1.z) * vec2(0.0, -other.g1.z) + self.g2 * vec2(other.g0.w) + vec2(self.g6.x) * vec2(-other.g0.x, 0.0) + vec2(self.g6.y) * vec2(-other.g0.y, 0.0) + vec2(self.g6.z) * vec2(-other.g0.z, 0.0) + vec2(self.g7.x) * vec2(0.0, -other.g1.x) + vec2(self.g7.y) * vec2(0.0, -other.g1.y) + vec2(self.g7.z) * vec2(0.0, -other.g1.z) + vec2(self.g8.x) * vec2(0.0, -other.g0.x) + vec2(self.g8.y) * vec2(0.0, -other.g0.y) + vec2(self.g8.z) * vec2(0.0, -other.g0.z), vec3(self.g3.x) * vec3(other.g0.w, -other.g0.z, other.g0.y) + vec3(self.g3.y) * vec3(other.g0.z, other.g0.w, -other.g0.x) + vec3(self.g3.z) * vec3(-other.g0.y, other.g0.x, other.g0.w) + vec3(self.g10.x) * vec3(other.g0.x, other.g0.y, other.g0.z), vec3(self.g0.x) * vec3(other.g0.x, other.g0.y, other.g0.z) + vec3(self.g3.x) * vec3(0.0, -other.g1.z, other.g1.y) + vec3(self.g3.y) * vec3(other.g1.z, 0.0, -other.g1.x) + vec3(self.g3.z) * vec3(-other.g1.y, other.g1.x, 0.0) + vec3(self.g4.x) * vec3(other.g0.w, -other.g0.z, other.g0.y) + vec3(self.g4.y) * vec3(other.g0.z, other.g0.w, -other.g0.x) + vec3(self.g4.z) * vec3(-other.g0.y, other.g0.x, other.g0.w) + vec3(self.g10.x) * other.g1, vec4(self.g0.x) * vec4(other.g1.x, other.g1.y, other.g1.z, 0.0) + vec4(self.g3.x) * vec4(0.0, 0.0, 0.0, -other.g1.x) + vec4(self.g3.y) * vec4(0.0, 0.0, 0.0, -other.g1.y) + vec4(self.g3.z) * vec4(0.0, 0.0, 0.0, -other.g1.z) + vec4(self.g4.x) * vec4(0.0, -other.g1.z, other.g1.y, 0.0) + vec4(self.g4.y) * vec4(other.g1.z, 0.0, -other.g1.x, 0.0) + vec4(self.g4.z) * vec4(-other.g1.y, other.g1.x, 0.0, 0.0) + vec4(self.g5.x) * vec4(other.g0.w, -other.g0.z, other.g0.y, 0.0) + vec4(self.g5.y) * vec4(other.g0.z, other.g0.w, -other.g0.x, 0.0) + vec4(self.g5.z) * vec4(-other.g0.y, other.g0.x, other.g0.w, 0.0) + vec4(self.g5.w) * vec4(-other.g1.x, -other.g1.y, -other.g1.z, other.g0.w) + vec4(self.g9.x) * vec4(0.0, other.g1.z, -other.g1.y, -other.g0.x) + vec4(self.g9.y) * vec4(-other.g1.z, 0.0, other.g1.x, -other.g0.y) + vec4(self.g9.z) * vec4(other.g1.y, -other.g1.x, 0.0, -other.g0.z) + vec4(self.g10.y) * vec4(other.g0.x, other.g0.y, other.g0.z, 0.0), vec4(self.g1.x) * vec4(0.0, 0.0, 0.0, -other.g0.x) + vec4(self.g1.y) * vec4(0.0, 0.0, 0.0, -other.g0.y) + vec4(self.g1.z) * vec4(0.0, 0.0, 0.0, -other.g0.z) + vec4(self.g2.x) * vec4(other.g0.x, other.g0.y, other.g0.z, 0.0) + vec4(self.g6.x) * vec4(other.g0.w, -other.g0.z, other.g0.y, other.g1.x) + vec4(self.g6.y) * vec4(other.g0.z, other.g0.w, -other.g0.x, other.g1.y) + vec4(self.g6.z) * vec4(-other.g0.y, other.g0.x, other.g0.w, other.g1.z) + vec4(self.g6.w) * vec4(0.0, 0.0, 0.0, other.g0.w), vec3(self.g0.y) * vec3(other.g0.x, other.g0.y, other.g0.z) + vec3(self.g2.x) * other.g1 + vec3(self.g6.x) * vec3(0.0, -other.g1.z, other.g1.y) + vec3(self.g6.y) * vec3(other.g1.z, 0.0, -other.g1.x) + vec3(self.g6.z) * vec3(-other.g1.y, other.g1.x, 0.0) + vec3(self.g7.x) * vec3(other.g0.w, -other.g0.z, other.g0.y) + vec3(self.g7.y) * vec3(other.g0.z, other.g0.w, -other.g0.x) + vec3(self.g7.z) * vec3(-other.g0.y, other.g0.x, other.g0.w), vec3(self.g0.y) * other.g1 + vec3(self.g1.x) * vec3(0.0, -other.g1.z, other.g1.y) + vec3(self.g1.y) * vec3(other.g1.z, 0.0, -other.g1.x) + vec3(self.g1.z) * vec3(-other.g1.y, other.g1.x, 0.0) + vec3(self.g2.y) * vec3(other.g0.x, other.g0.y, other.g0.z) + vec3(self.g6.w) * other.g1 + vec3(self.g7.x) * vec3(0.0, -other.g1.z, other.g1.y) + vec3(self.g7.y) * vec3(other.g1.z, 0.0, -other.g1.x) + vec3(self.g7.z) * vec3(-other.g1.y, other.g1.x, 0.0) + vec3(self.g8.x) * vec3(other.g0.w, -other.g0.z, other.g0.y) + vec3(self.g8.y) * vec3(other.g0.z, other.g0.w, -other.g0.x) + vec3(self.g8.z) * vec3(-other.g0.y, other.g0.x, other.g0.w), vec3(self.g3.x) * vec3(0.0, -other.g1.z, other.g1.y) + vec3(self.g3.y) * vec3(other.g1.z, 0.0, -other.g1.x) + vec3(self.g3.z) * vec3(-other.g1.y, other.g1.x, 0.0) + vec3(self.g5.w) * vec3(other.g0.x, other.g0.y, other.g0.z) + vec3(self.g9.x) * vec3(other.g0.w, -other.g0.z, other.g0.y) + vec3(self.g9.y) * vec3(other.g0.z, other.g0.w, -other.g0.x) + vec3(self.g9.z) * vec3(-other.g0.y, other.g0.x, other.g0.w) + vec3(self.g10.x) * other.g1, vec2(self.g3.x) * vec2(-other.g0.x, 0.0) + vec2(self.g3.y) * vec2(-other.g0.y, 0.0) + vec2(self.g3.z) * vec2(-other.g0.z, 0.0) + vec2(self.g4.x) * vec2(0.0, -other.g1.x) + vec2(self.g4.y) * vec2(0.0, -other.g1.y) + vec2(self.g4.z) * vec2(0.0, -other.g1.z) + vec2(self.g5.x) * vec2(0.0, -other.g0.x) + vec2(self.g5.y) * vec2(0.0, -other.g0.y) + vec2(self.g5.z) * vec2(0.0, -other.g0.z) + vec2(self.g9.x) * vec2(0.0, other.g1.x) + vec2(self.g9.y) * vec2(0.0, other.g1.y) + vec2(self.g9.z) * vec2(0.0, other.g1.z) + self.g10 * vec2(other.g0.w));
}

MultiVector multi_vector__anti_wedge_dot__multi_vector(MultiVector self, MultiVector other) {
    return MultiVector(vec2(self.g0.x) * other.g0.yx * vec2(1.0, -1.0) + vec2(self.g0.y) * other.g0 + vec2(self.g1.x) * vec2(other.g9.x, -other.g1.x) + vec2(self.g1.y) * vec2(other.g9.y, -other.g1.y) + vec2(self.g1.z) * vec2(other.g9.z, -other.g1.z) + vec2(self.g2.x) * vec2(other.g10.y, other.g2.y) + vec2(self.g2.y) * vec2(other.g10.x, other.g2.x) + vec2(self.g3.x) * vec2(-other.g8.x, other.g5.x) + vec2(self.g3.y) * vec2(-other.g8.y, other.g5.y) + vec2(self.g3.z) * vec2(-other.g8.z, other.g5.z) + vec2(self.g4.x) * vec2(-other.g7.x, other.g4.x) + vec2(self.g4.y) * vec2(-other.g7.y, other.g4.y) + vec2(self.g4.z) * vec2(-other.g7.z, other.g4.z) + vec2(self.g5.x) * vec2(-other.g6.x, other.g3.x) + vec2(self.g5.y) * vec2(-other.g6.y, other.g3.y) + vec2(self.g5.z) * vec2(-other.g6.z, other.g3.z) - vec2(self.g5.w) * vec2(other.g6.w, other.g5.w) - vec2(self.g6.x) * vec2(other.g5.x, other.g8.x) - vec2(self.g6.y) * vec2(other.g5.y, other.g8.y) - vec2(self.g6.z) * vec2(other.g5.z, other.g8.z) + vec2(self.g6.w) * vec2(-other.g5.w, other.g6.w) - vec2(self.g7.x) * vec2(other.g4.x, other.g7.x) - vec2(self.g7.y) * vec2(other.g4.y, other.g7.y) - vec2(self.g7.z) * vec2(other.g4.z, other.g7.z) - vec2(self.g8.x) * vec2(other.g3.x, other.g6.x) - vec2(self.g8.y) * vec2(other.g3.y, other.g6.y) - vec2(self.g8.z) * vec2(other.g3.z, other.g6.z) + vec2(self.g9.x) * vec2(other.g1.x, other.g9.x) + vec2(self.g9.y) * vec2(other.g1.y, other.g9.y) + vec2(self.g9.z) * vec2(other.g1.z, other.g9.z) + vec2(self.g10.x) * vec2(other.g2.y, -other.g10.y) + vec2(self.g10.y) * vec2(other.g2.x, -other.g10.x), vec3(self.g0.x) * other.g9 + vec3(self.g0.y) * other.g1 + vec3(self.g1.x) * vec3(other.g0.y, -other.g7.z, other.g7.y) + vec3(self.g1.y) * vec3(other.g7.z, other.g0.y, -other.g7.x) + vec3(self.g1.z) * vec3(-other.g7.y, other.g7.x, other.g0.y) - vec3(self.g2.x) * other.g8 + vec3(self.g2.y) * vec3(other.g6.x, other.g6.y, other.g6.z) + vec3(self.g3.x) * vec3(other.g10.y, -other.g5.z, other.g5.y) + vec3(self.g3.y) * vec3(other.g5.z, other.g10.y, -other.g5.x) + vec3(self.g3.z) * vec3(-other.g5.y, other.g5.x, other.g10.y) + vec3(self.g4.x) * vec3(other.g5.w, -other.g9.z, other.g9.y) + vec3(self.g4.y) * vec3(other.g9.z, other.g5.w, -other.g9.x) + vec3(self.g4.z) * vec3(-other.g9.y, other.g9.x, other.g5.w) + vec3(self.g5.x) * vec3(-other.g10.x, other.g3.z, -other.g3.y) + vec3(self.g5.y) * vec3(-other.g3.z, -other.g10.x, other.g3.x) + vec3(self.g5.z) * vec3(other.g3.y, -other.g3.x, -other.g10.x) + vec3(self.g5.w) * other.g4 + vec3(self.g6.x) * vec3(-other.g2.y, other.g8.z, -other.g8.y) + vec3(self.g6.y) * vec3(-other.g8.z, -other.g2.y, other.g8.x) + vec3(self.g6.z) * vec3(other.g8.y, -other.g8.x, -other.g2.y) + vec3(self.g6.w) * other.g7 + vec3(self.g7.x) * vec3(other.g6.w, -other.g1.z, other.g1.y) + vec3(self.g7.y) * vec3(other.g1.z, other.g6.w, -other.g1.x) + vec3(self.g7.z) * vec3(-other.g1.y, other.g1.x, other.g6.w) + vec3(self.g8.x) * vec3(other.g2.x, -other.g6.z, other.g6.y) + vec3(self.g8.y) * vec3(other.g6.z, other.g2.x, -other.g6.x) + vec3(self.g8.z) * vec3(-other.g6.y, other.g6.x, other.g2.x) + vec3(self.g9.x) * vec3(other.g0.x, -other.g4.z, other.g4.y) + vec3(self.g9.y) * vec3(other.g4.z, other.g0.x, -other.g4.x) + vec3(self.g9.z) * vec3(-other.g4.y, other.g4.x, other.g0.x) + vec3(self.g10.x) * vec3(other.g5.x, other.g5.y, other.g5.z) - vec3(self.g10.y) * other.g3, vec2(0.0) - vec2(self.g0.x) * other.g10 + vec2(self.g0.y) * other.g2 + vec2(self.g1.x) * vec2(other.g6.x, -other.g8.x) + vec2(self.g1.y) * vec2(other.g6.y, -other.g8.y) + vec2(self.g1.z) * vec2(other.g6.z, -other.g8.z) + self.g2 * vec2(other.g6.w) + self.g2 * vec2(other.g0.y) + vec2(self.g3.x) * vec2(-other.g9.x, 0.0) + vec2(self.g3.x) * vec2(other.g4.x, 0.0) + vec2(self.g3.y) * vec2(-other.g9.y, 0.0) + vec2(self.g3.y) * vec2(other.g4.y, 0.0) + vec2(self.g3.z) * vec2(-other.g9.z, 0.0) + vec2(self.g3.z) * vec2(other.g4.z, 0.0) + vec2(self.g4.x) * vec2(other.g3.x, other.g5.x) + vec2(self.g4.y) * vec2(other.g3.y, other.g5.y) + vec2(self.g4.z) * vec2(other.g3.z, other.g5.z) + vec2(self.g5.x) * vec2(0.0, other.g9.x) + vec2(self.g5.x) * vec2(0.0, other.g4.x) + vec2(self.g5.y) * vec2(0.0, other.g9.y) + vec2(self.g5.y) * vec2(0.0, other.g4.y) + vec2(self.g5.z) * vec2(0.0, other.g9.z) + vec2(self.g5.z) * vec2(0.0, other.g4.z) + vec2(self.g5.w) * other.g10 * vec2(-1.0, 1.0) + vec2(self.g6.x) * vec2(-other.g7.x, 0.0) + vec2(self.g6.x) * vec2(-other.g1.x, 0.0) + vec2(self.g6.y) * vec2(-other.g7.y, 0.0) + vec2(self.g6.y) * vec2(-other.g1.y, 0.0) + vec2(self.g6.z) * vec2(-other.g7.z, 0.0) + vec2(self.g6.z) * vec2(-other.g1.z, 0.0) + vec2(self.g6.w) * other.g2 * vec2(-1.0, 1.0) - vec2(self.g7.x) * vec2(other.g6.x, other.g8.x) - vec2(self.g7.y) * vec2(other.g6.y, other.g8.y) - vec2(self.g7.z) * vec2(other.g6.z, other.g8.z) + vec2(self.g8.x) * vec2(0.0, -other.g7.x) + vec2(self.g8.x) * vec2(0.0, other.g1.x) + vec2(self.g8.y) * vec2(0.0, -other.g7.y) + vec2(self.g8.y) * vec2(0.0, other.g1.y) + vec2(self.g8.z) * vec2(0.0, -other.g7.z) + vec2(self.g8.z) * vec2(0.0, other.g1.z) + vec2(self.g9.x) * vec2(other.g3.x, -other.g5.x) + vec2(self.g9.y) * vec2(other.g3.y, -other.g5.y) + vec2(self.g9.z) * vec2(other.g3.z, -other.g5.z) + self.g10 * vec2(other.g5.w) - self.g10 * vec2(other.g0.x), vec3(self.g0.x) * vec3(other.g6.x, other.g6.y, other.g6.z) + vec3(self.g0.y) * other.g3 + vec3(self.g1.x) * vec3(-other.g10.x, other.g3.z, -other.g3.y) + vec3(self.g1.y) * vec3(-other.g3.z, -other.g10.x, other.g3.x) + vec3(self.g1.z) * vec3(other.g3.y, -other.g3.x, -other.g10.x) - vec3(self.g2.x) * other.g9 + vec3(self.g2.x) * other.g4 + vec3(self.g3.x) * vec3(other.g6.w, -other.g7.z, other.g7.y) + vec3(self.g3.x) * vec3(other.g0.y, -other.g1.z, other.g1.y) + vec3(self.g3.y) * vec3(other.g7.z, other.g6.w, -other.g7.x) + vec3(self.g3.y) * vec3(other.g1.z, other.g0.y, -other.g1.x) + vec3(self.g3.z) * vec3(-other.g7.y, other.g7.x, other.g6.w) + vec3(self.g3.z) * vec3(-other.g1.y, other.g1.x, other.g0.y) + vec3(self.g4.x) * vec3(other.g2.x, -other.g6.z, other.g6.y) + vec3(self.g4.y) * vec3(other.g6.z, other.g2.x, -other.g6.x) + vec3(self.g4.z) * vec3(-other.g6.y, other.g6.x, other.g2.x) + vec3(self.g5.w) * vec3(other.g6.x, other.g6.y, other.g6.z) + vec3(self.g6.x) * vec3(-other.g5.w, other.g9.z, -other.g9.y) + vec3(self.g6.x) * vec3(other.g0.x, -other.g4.z, other.g4.y) + vec3(self.g6.y) * vec3(-other.g9.z, -other.g5.w, other.g9.x) + vec3(self.g6.y) * vec3(other.g4.z, other.g0.x, -other.g4.x) + vec3(self.g6.z) * vec3(other.g9.y, -other.g9.x, -other.g5.w) + vec3(self.g6.z) * vec3(-other.g4.y, other.g4.x, other.g0.x) - vec3(self.g6.w) * other.g3 + vec3(self.g7.x) * vec3(other.g10.x, -other.g3.z, other.g3.y) + vec3(self.g7.y) * vec3(other.g3.z, other.g10.x, -other.g3.x) + vec3(self.g7.z) * vec3(-other.g3.y, other.g3.x, other.g10.x) + vec3(self.g9.x) * vec3(other.g2.x, -other.g6.z, other.g6.y) + vec3(self.g9.y) * vec3(other.g6.z, other.g2.x, -other.g6.x) + vec3(self.g9.z) * vec3(-other.g6.y, other.g6.x, other.g2.x) + vec3(self.g10.x) * other.g7 + vec3(self.g10.x) * other.g1, vec3(self.g0.x) * other.g7 + vec3(self.g0.y) * other.g4 + vec3(self.g1.x) * vec3(-other.g5.w, other.g9.z, -other.g9.y) + vec3(self.g1.y) * vec3(-other.g9.z, -other.g5.w, other.g9.x) + vec3(self.g1.z) * vec3(other.g9.y, -other.g9.x, -other.g5.w) + vec3(self.g2.x) * vec3(other.g5.x, other.g5.y, other.g5.z) + vec3(self.g2.y) * other.g3 + vec3(self.g3.x) * vec3(other.g2.y, -other.g8.z, other.g8.y) + vec3(self.g3.y) * vec3(other.g8.z, other.g2.y, -other.g8.x) + vec3(self.g3.z) * vec3(-other.g8.y, other.g8.x, other.g2.y) + vec3(self.g4.x) * vec3(other.g0.y, -other.g7.z, other.g7.y) + vec3(self.g4.y) * vec3(other.g7.z, other.g0.y, -other.g7.x) + vec3(self.g4.z) * vec3(-other.g7.y, other.g7.x, other.g0.y) + vec3(self.g5.x) * vec3(other.g2.x, -other.g6.z, other.g6.y) + vec3(self.g5.y) * vec3(other.g6.z, other.g2.x, -other.g6.x) + vec3(self.g5.z) * vec3(-other.g6.y, other.g6.x, other.g2.x) - vec3(self.g5.w) * other.g1 + vec3(self.g6.x) * vec3(other.g10.y, -other.g5.z, other.g5.y) + vec3(self.g6.y) * vec3(other.g5.z, other.g10.y, -other.g5.x) + vec3(self.g6.z) * vec3(-other.g5.y, other.g5.x, other.g10.y) - vec3(self.g6.w) * other.g9 + vec3(self.g7.x) * vec3(other.g0.x, -other.g4.z, other.g4.y) + vec3(self.g7.y) * vec3(other.g4.z, other.g0.x, -other.g4.x) + vec3(self.g7.z) * vec3(-other.g4.y, other.g4.x, other.g0.x) + vec3(self.g8.x) * vec3(other.g10.x, -other.g3.z, other.g3.y) + vec3(self.g8.y) * vec3(other.g3.z, other.g10.x, -other.g3.x) + vec3(self.g8.z) * vec3(-other.g3.y, other.g3.x, other.g10.x) + vec3(self.g9.x) * vec3(-other.g6.w, other.g1.z, -other.g1.y) + vec3(self.g9.y) * vec3(-other.g1.z, -other.g6.w, other.g1.x) + vec3(self.g9.z) * vec3(other.g1.y, -other.g1.x, -other.g6.w) + vec3(self.g10.x) * other.g8 + vec3(self.g10.y) * vec3(other.g6.x, other.g6.y, other.g6.z), vec4(self.g0.x) * vec4(other.g8.x, other.g8.y, other.g8.z, -other.g6.w) + vec4(self.g0.y) * other.g5 + vec4(self.g1.x) * vec4(other.g10.y, -other.g5.z, other.g5.y, other.g4.x) + vec4(self.g1.y) * vec4(other.g5.z, other.g10.y, -other.g5.x, other.g4.y) + vec4(self.g1.z) * vec4(-other.g5.y, other.g5.x, other.g10.y, other.g4.z) + vec4(self.g2.x) * vec4(0.0, 0.0, 0.0, other.g10.y) + vec4(self.g2.y) * vec4(other.g9.x, other.g9.y, other.g9.z, -other.g10.x) + vec4(self.g2.y) * vec4(other.g4.x, other.g4.y, other.g4.z, 0.0) + vec4(self.g3.x) * vec4(0.0, 0.0, 0.0, -other.g8.x) + vec4(self.g3.y) * vec4(0.0, 0.0, 0.0, -other.g8.y) + vec4(self.g3.z) * vec4(0.0, 0.0, 0.0, -other.g8.z) + vec4(self.g4.x) * vec4(other.g2.y, -other.g8.z, other.g8.y, other.g1.x) + vec4(self.g4.y) * vec4(other.g8.z, other.g2.y, -other.g8.x, other.g1.y) + vec4(self.g4.z) * vec4(-other.g8.y, other.g8.x, other.g2.y, other.g1.z) + vec4(self.g5.x) * vec4(-other.g6.w, -other.g7.z, other.g7.y, other.g6.x) + vec4(self.g5.x) * vec4(other.g0.y, other.g1.z, -other.g1.y, 0.0) + vec4(self.g5.y) * vec4(other.g7.z, -other.g6.w, -other.g7.x, other.g6.y) + vec4(self.g5.y) * vec4(-other.g1.z, other.g0.y, other.g1.x, 0.0) + vec4(self.g5.z) * vec4(-other.g7.y, other.g7.x, -other.g6.w, other.g6.z) + vec4(self.g5.z) * vec4(other.g1.y, -other.g1.x, other.g0.y, 0.0) + vec4(self.g5.w) * vec4(-other.g8.x, -other.g8.y, -other.g8.z, other.g0.y) + vec4(self.g6.x) * vec4(0.0, 0.0, 0.0, -other.g5.x) + vec4(self.g6.y) * vec4(0.0, 0.0, 0.0, -other.g5.y) + vec4(self.g6.z) * vec4(0.0, 0.0, 0.0, -other.g5.z) + vec4(self.g6.w) * vec4(other.g5.x, other.g5.y, other.g5.z, -other.g0.x) + vec4(self.g7.x) * vec4(other.g10.y, -other.g5.z, other.g5.y, -other.g9.x) + vec4(self.g7.y) * vec4(other.g5.z, other.g10.y, -other.g5.x, -other.g9.y) + vec4(self.g7.z) * vec4(-other.g5.y, other.g5.x, other.g10.y, -other.g9.z) + vec4(self.g8.x) * vec4(other.g5.w, -other.g9.z, other.g9.y, other.g3.x) + vec4(self.g8.x) * vec4(other.g0.x, -other.g4.z, other.g4.y, 0.0) + vec4(self.g8.y) * vec4(other.g9.z, other.g5.w, -other.g9.x, other.g3.y) + vec4(self.g8.y) * vec4(other.g4.z, other.g0.x, -other.g4.x, 0.0) + vec4(self.g8.z) * vec4(-other.g9.y, other.g9.x, other.g5.w, other.g3.z) + vec4(self.g8.z) * vec4(-other.g4.y, other.g4.x, other.g0.x, 0.0) + vec4(self.g9.x) * vec4(-other.g2.y, other.g8.z, -other.g8.y, -other.g7.x) + vec4(self.g9.y) * vec4(-other.g8.z, -other.g2.y, other.g8.x, -other.g7.y) + vec4(self.g9.z) * vec4(other.g8.y, -other.g8.x, -other.g2.y, -other.g7.z) + vec4(self.g10.x) * vec4(0.0, 0.0, 0.0, other.g2.y) + vec4(self.g10.y) * vec4(other.g7.x, other.g7.y, other.g7.z, -other.g2.x) + vec4(self.g10.y) * vec4(-other.g1.x, -other.g1.y, -other.g1.z, 0.0), vec4(self.g0.x) * vec4(-other.g3.x, -other.g3.y, -other.g3.z, other.g5.w) + vec4(self.g0.y) * other.g6 + vec4(self.g1.x) * vec4(-other.g2.x, other.g6.z, -other.g6.y, -other.g7.x) + vec4(self.g1.y) * vec4(-other.g6.z, -other.g2.x, other.g6.x, -other.g7.y) + vec4(self.g1.z) * vec4(other.g6.y, -other.g6.x, -other.g2.x, -other.g7.z) + vec4(self.g2.x) * vec4(other.g7.x, other.g7.y, other.g7.z, -other.g2.y) + vec4(self.g2.x) * vec4(other.g1.x, other.g1.y, other.g1.z, 0.0) + vec4(self.g2.y) * vec4(0.0, 0.0, 0.0, other.g2.x) + vec4(self.g3.x) * vec4(other.g5.w, -other.g9.z, other.g9.y, -other.g5.x) + vec4(self.g3.x) * vec4(-other.g0.x, other.g4.z, -other.g4.y, 0.0) + vec4(self.g3.y) * vec4(other.g9.z, other.g5.w, -other.g9.x, -other.g5.y) + vec4(self.g3.y) * vec4(-other.g4.z, -other.g0.x, other.g4.x, 0.0) + vec4(self.g3.z) * vec4(-other.g9.y, other.g9.x, other.g5.w, -other.g5.z) + vec4(self.g3.z) * vec4(other.g4.y, -other.g4.x, -other.g0.x, 0.0) + vec4(self.g4.x) * vec4(-other.g10.x, other.g3.z, -other.g3.y, -other.g9.x) + vec4(self.g4.y) * vec4(-other.g3.z, -other.g10.x, other.g3.x, -other.g9.y) + vec4(self.g4.z) * vec4(other.g3.y, -other.g3.x, -other.g10.x, -other.g9.z) + vec4(self.g5.x) * vec4(0.0, 0.0, 0.0, other.g3.x) + vec4(self.g5.y) * vec4(0.0, 0.0, 0.0, other.g3.y) + vec4(self.g5.z) * vec4(0.0, 0.0, 0.0, other.g3.z) + vec4(self.g5.w) * vec4(-other.g3.x, -other.g3.y, -other.g3.z, other.g0.x) + vec4(self.g6.x) * vec4(other.g6.w, -other.g7.z, other.g7.y, other.g8.x) + vec4(self.g6.x) * vec4(other.g0.y, -other.g1.z, other.g1.y, 0.0) + vec4(self.g6.y) * vec4(other.g7.z, other.g6.w, -other.g7.x, other.g8.y) + vec4(self.g6.y) * vec4(other.g1.z, other.g0.y, -other.g1.x, 0.0) + vec4(self.g6.z) * vec4(-other.g7.y, other.g7.x, other.g6.w, other.g8.z) + vec4(self.g6.z) * vec4(-other.g1.y, other.g1.x, other.g0.y, 0.0) + vec4(self.g6.w) * vec4(-other.g6.x, -other.g6.y, -other.g6.z, other.g0.y) + vec4(self.g7.x) * vec4(other.g2.x, -other.g6.z, other.g6.y, -other.g1.x) + vec4(self.g7.y) * vec4(other.g6.z, other.g2.x, -other.g6.x, -other.g1.y) + vec4(self.g7.z) * vec4(-other.g6.y, other.g6.x, other.g2.x, -other.g1.z) + vec4(self.g8.x) * vec4(0.0, 0.0, 0.0, -other.g6.x) + vec4(self.g8.y) * vec4(0.0, 0.0, 0.0, -other.g6.y) + vec4(self.g8.z) * vec4(0.0, 0.0, 0.0, -other.g6.z) + vec4(self.g9.x) * vec4(-other.g10.x, other.g3.z, -other.g3.y, -other.g4.x) + vec4(self.g9.y) * vec4(-other.g3.z, -other.g10.x, other.g3.x, -other.g4.y) + vec4(self.g9.z) * vec4(other.g3.y, -other.g3.x, -other.g10.x, -other.g4.z) + vec4(self.g10.x) * vec4(other.g9.x, other.g9.y, other.g9.z, other.g10.y) + vec4(self.g10.x) * vec4(-other.g4.x, -other.g4.y, -other.g4.z, 0.0) + vec4(self.g10.y) * vec4(0.0, 0.0, 0.0, -other.g10.x), vec3(0.0) - vec3(self.g0.x) * other.g4 + vec3(self.g0.y) * other.g7 + vec3(self.g1.x) * vec3(other.g6.w, -other.g1.z, other.g1.y) + vec3(self.g1.y) * vec3(other.g1.z, other.g6.w, -other.g1.x) + vec3(self.g1.z) * vec3(-other.g1.y, other.g1.x, other.g6.w) + vec3(self.g2.x) * other.g8 + vec3(self.g2.y) * vec3(other.g6.x, other.g6.y, other.g6.z) + vec3(self.g3.x) * vec3(-other.g10.y, other.g5.z, -other.g5.y) + vec3(self.g3.y) * vec3(-other.g5.z, -other.g10.y, other.g5.x) + vec3(self.g3.z) * vec3(other.g5.y, -other.g5.x, -other.g10.y) + vec3(self.g4.x) * vec3(-other.g0.x, other.g4.z, -other.g4.y) + vec3(self.g4.y) * vec3(-other.g4.z, -other.g0.x, other.g4.x) + vec3(self.g4.z) * vec3(other.g4.y, -other.g4.x, -other.g0.x) + vec3(self.g5.x) * vec3(-other.g10.x, other.g3.z, -other.g3.y) + vec3(self.g5.y) * vec3(-other.g3.z, -other.g10.x, other.g3.x) + vec3(self.g5.z) * vec3(other.g3.y, -other.g3.x, -other.g10.x) - vec3(self.g5.w) * other.g9 + vec3(self.g6.x) * vec3(other.g2.y, -other.g8.z, other.g8.y) + vec3(self.g6.y) * vec3(other.g8.z, other.g2.y, -other.g8.x) + vec3(self.g6.z) * vec3(-other.g8.y, other.g8.x, other.g2.y) + vec3(self.g6.w) * other.g1 + vec3(self.g7.x) * vec3(other.g0.y, -other.g7.z, other.g7.y) + vec3(self.g7.y) * vec3(other.g7.z, other.g0.y, -other.g7.x) + vec3(self.g7.z) * vec3(-other.g7.y, other.g7.x, other.g0.y) + vec3(self.g8.x) * vec3(other.g2.x, -other.g6.z, other.g6.y) + vec3(self.g8.y) * vec3(other.g6.z, other.g2.x, -other.g6.x) + vec3(self.g8.z) * vec3(-other.g6.y, other.g6.x, other.g2.x) + vec3(self.g9.x) * vec3(-other.g5.w, other.g9.z, -other.g9.y) + vec3(self.g9.y) * vec3(-other.g9.z, -other.g5.w, other.g9.x) + vec3(self.g9.z) * vec3(other.g9.y, -other.g9.x, -other.g5.w) - vec3(self.g10.x) * vec3(other.g5.x, other.g5.y, other.g5.z) - vec3(self.g10.y) * other.g3, vec3(0.0) - vec3(self.g0.x) * vec3(other.g5.x, other.g5.y, other.g5.z) + vec3(self.g0.y) * other.g8 + vec3(self.g1.x) * vec3(other.g2.y, -other.g8.z, other.g8.y) + vec3(self.g1.y) * vec3(other.g8.z, other.g2.y, -other.g8.x) + vec3(self.g1.z) * vec3(-other.g8.y, other.g8.x, other.g2.y) + vec3(self.g2.y) * other.g7 - vec3(self.g2.y) * other.g1 + vec3(self.g4.x) * vec3(-other.g10.y, other.g5.z, -other.g5.y) + vec3(self.g4.y) * vec3(-other.g5.z, -other.g10.y, other.g5.x) + vec3(self.g4.z) * vec3(other.g5.y, -other.g5.x, -other.g10.y) + vec3(self.g5.x) * vec3(-other.g5.w, other.g9.z, -other.g9.y) + vec3(self.g5.x) * vec3(-other.g0.x, other.g4.z, -other.g4.y) + vec3(self.g5.y) * vec3(-other.g9.z, -other.g5.w, other.g9.x) + vec3(self.g5.y) * vec3(-other.g4.z, -other.g0.x, other.g4.x) + vec3(self.g5.z) * vec3(other.g9.y, -other.g9.x, -other.g5.w) + vec3(self.g5.z) * vec3(other.g4.y, -other.g4.x, -other.g0.x) + vec3(self.g5.w) * vec3(other.g5.x, other.g5.y, other.g5.z) + vec3(self.g6.w) * other.g8 + vec3(self.g7.x) * vec3(other.g2.y, -other.g8.z, other.g8.y) + vec3(self.g7.y) * vec3(other.g8.z, other.g2.y, -other.g8.x) + vec3(self.g7.z) * vec3(-other.g8.y, other.g8.x, other.g2.y) + vec3(self.g8.x) * vec3(-other.g6.w, -other.g7.z, other.g7.y) + vec3(self.g8.x) * vec3(other.g0.y, other.g1.z, -other.g1.y) + vec3(self.g8.y) * vec3(other.g7.z, -other.g6.w, -other.g7.x) + vec3(self.g8.y) * vec3(-other.g1.z, other.g0.y, other.g1.x) + vec3(self.g8.z) * vec3(-other.g7.y, other.g7.x, -other.g6.w) + vec3(self.g8.z) * vec3(other.g1.y, -other.g1.x, other.g0.y) + vec3(self.g9.x) * vec3(other.g10.y, -other.g5.z, other.g5.y) + vec3(self.g9.y) * vec3(other.g5.z, other.g10.y, -other.g5.x) + vec3(self.g9.z) * vec3(-other.g5.y, other.g5.x, other.g10.y) - vec3(self.g10.y) * other.g9 - vec3(self.g10.y) * other.g4, vec3(0.0) - vec3(self.g0.x) * other.g1 + vec3(self.g0.y) * other.g9 + vec3(self.g1.x) * vec3(-other.g0.x, other.g4.z, -other.g4.y) + vec3(self.g1.y) * vec3(-other.g4.z, -other.g0.x, other.g4.x) + vec3(self.g1.z) * vec3(other.g4.y, -other.g4.x, -other.g0.x) + vec3(self.g2.x) * vec3(other.g5.x, other.g5.y, other.g5.z) - vec3(self.g2.y) * other.g3 + vec3(self.g3.x) * vec3(other.g2.y, -other.g8.z, other.g8.y) + vec3(self.g3.y) * vec3(other.g8.z, other.g2.y, -other.g8.x) + vec3(self.g3.z) * vec3(-other.g8.y, other.g8.x, other.g2.y) + vec3(self.g4.x) * vec3(-other.g6.w, other.g1.z, -other.g1.y) + vec3(self.g4.y) * vec3(-other.g1.z, -other.g6.w, other.g1.x) + vec3(self.g4.z) * vec3(other.g1.y, -other.g1.x, -other.g6.w) + vec3(self.g5.x) * vec3(-other.g2.x, other.g6.z, -other.g6.y) + vec3(self.g5.y) * vec3(-other.g6.z, -other.g2.x, other.g6.x) + vec3(self.g5.z) * vec3(other.g6.y, -other.g6.x, -other.g2.x) + vec3(self.g5.w) * other.g7 + vec3(self.g6.x) * vec3(other.g10.y, -other.g5.z, other.g5.y) + vec3(self.g6.y) * vec3(other.g5.z, other.g10.y, -other.g5.x) + vec3(self.g6.z) * vec3(-other.g5.y, other.g5.x, other.g10.y) - vec3(self.g6.w) * other.g4 + vec3(self.g7.x) * vec3(other.g5.w, -other.g9.z, other.g9.y) + vec3(self.g7.y) * vec3(other.g9.z, other.g5.w, -other.g9.x) + vec3(self.g7.z) * vec3(-other.g9.y, other.g9.x, other.g5.w) + vec3(self.g8.x) * vec3(-other.g10.x, other.g3.z, -other.g3.y) + vec3(self.g8.y) * vec3(-other.g3.z, -other.g10.x, other.g3.x) + vec3(self.g8.z) * vec3(other.g3.y, -other.g3.x, -other.g10.x) + vec3(self.g9.x) * vec3(other.g0.y, -other.g7.z, other.g7.y) + vec3(self.g9.y) * vec3(other.g7.z, other.g0.y, -other.g7.x) + vec3(self.g9.z) * vec3(-other.g7.y, other.g7.x, other.g0.y) + vec3(self.g10.x) * other.g8 - vec3(self.g10.y) * vec3(other.g6.x, other.g6.y, other.g6.z), vec2(self.g0.x) * other.g2 + vec2(self.g0.y) * other.g10 + vec2(self.g1.x) * vec2(other.g3.x, -other.g5.x) + vec2(self.g1.y) * vec2(other.g3.y, -other.g5.y) + vec2(self.g1.z) * vec2(other.g3.z, -other.g5.z) + self.g2 * vec2(-other.g5.w) + self.g2 * vec2(other.g0.x) + vec2(self.g3.x) * vec2(-other.g7.x, 0.0) + vec2(self.g3.x) * vec2(-other.g1.x, 0.0) + vec2(self.g3.y) * vec2(-other.g7.y, 0.0) + vec2(self.g3.y) * vec2(-other.g1.y, 0.0) + vec2(self.g3.z) * vec2(-other.g7.z, 0.0) + vec2(self.g3.z) * vec2(-other.g1.z, 0.0) - vec2(self.g4.x) * vec2(other.g6.x, other.g8.x) - vec2(self.g4.y) * vec2(other.g6.y, other.g8.y) - vec2(self.g4.z) * vec2(other.g6.z, other.g8.z) + vec2(self.g5.x) * vec2(0.0, -other.g7.x) + vec2(self.g5.x) * vec2(0.0, other.g1.x) + vec2(self.g5.y) * vec2(0.0, -other.g7.y) + vec2(self.g5.y) * vec2(0.0, other.g1.y) + vec2(self.g5.z) * vec2(0.0, -other.g7.z) + vec2(self.g5.z) * vec2(0.0, other.g1.z) + vec2(self.g5.w) * other.g2 * vec2(1.0, -1.0) + vec2(self.g6.x) * vec2(other.g9.x, 0.0) + vec2(self.g6.x) * vec2(-other.g4.x, 0.0) + vec2(self.g6.y) * vec2(other.g9.y, 0.0) + vec2(self.g6.y) * vec2(-other.g4.y, 0.0) + vec2(self.g6.z) * vec2(other.g9.z, 0.0) + vec2(self.g6.z) * vec2(-other.g4.z, 0.0) + vec2(self.g6.w) * other.g10 * vec2(-1.0, 1.0) - vec2(self.g7.x) * vec2(other.g3.x, other.g5.x) - vec2(self.g7.y) * vec2(other.g3.y, other.g5.y) - vec2(self.g7.z) * vec2(other.g3.z, other.g5.z) + vec2(self.g8.x) * vec2(0.0, -other.g9.x) + vec2(self.g8.x) * vec2(0.0, -other.g4.x) + vec2(self.g8.y) * vec2(0.0, -other.g9.y) + vec2(self.g8.y) * vec2(0.0, -other.g4.y) + vec2(self.g8.z) * vec2(0.0, -other.g9.z) + vec2(self.g8.z) * vec2(0.0, -other.g4.z) + vec2(self.g9.x) * vec2(-other.g6.x, other.g8.x) + vec2(self.g9.y) * vec2(-other.g6.y, other.g8.y) + vec2(self.g9.z) * vec2(-other.g6.z, other.g8.z) + self.g10 * vec2(other.g6.w) + self.g10 * vec2(other.g0.y));
}

MultiVector multi_vector__anti_wedge_dot__plane(MultiVector self, Plane other) {
    return MultiVector(vec2(self.g1.x) * vec2(other.g0.x, 0.0) + vec2(self.g1.y) * vec2(other.g0.y, 0.0) + vec2(self.g1.z) * vec2(other.g0.z, 0.0) + vec2(self.g2.x) * vec2(other.g0.w, 0.0) + vec2(self.g9.x) * vec2(0.0, other.g0.x) + vec2(self.g9.y) * vec2(0.0, other.g0.y) + vec2(self.g9.z) * vec2(0.0, other.g0.z) + vec2(self.g10.x) * vec2(0.0, -other.g0.w), vec3(self.g0.x) * vec3(other.g0.x, other.g0.y, other.g0.z) + self.g3 * vec3(other.g0.w) + vec3(self.g4.x) * vec3(0.0, -other.g0.z, other.g0.y) + vec3(self.g4.y) * vec3(other.g0.z, 0.0, -other.g0.x) + vec3(self.g4.z) * vec3(-other.g0.y, other.g0.x, 0.0), vec2(self.g0.x) * vec2(0.0, -other.g0.w) + vec2(self.g3.x) * vec2(-other.g0.x, 0.0) + vec2(self.g3.y) * vec2(-other.g0.y, 0.0) + vec2(self.g3.z) * vec2(-other.g0.z, 0.0) + vec2(self.g5.x) * vec2(0.0, other.g0.x) + vec2(self.g5.y) * vec2(0.0, other.g0.y) + vec2(self.g5.z) * vec2(0.0, other.g0.z) + vec2(self.g5.w) * vec2(0.0, other.g0.w), vec3(0.0) - vec3(self.g2.x) * vec3(other.g0.x, other.g0.y, other.g0.z) + vec3(self.g6.x) * vec3(0.0, other.g0.z, -other.g0.y) + vec3(self.g6.y) * vec3(-other.g0.z, 0.0, other.g0.x) + vec3(self.g6.z) * vec3(other.g0.y, -other.g0.x, 0.0), vec3(self.g1.x) * vec3(0.0, other.g0.z, -other.g0.y) + vec3(self.g1.y) * vec3(-other.g0.z, 0.0, other.g0.x) + vec3(self.g1.z) * vec3(other.g0.y, -other.g0.x, 0.0) + vec3(self.g6.x, self.g6.y, self.g6.z) * vec3(other.g0.w) - vec3(self.g6.w) * vec3(other.g0.x, other.g0.y, other.g0.z), vec4(self.g1.x, self.g1.y, self.g1.z, self.g1.x) * vec4(other.g0.w, other.g0.w, other.g0.w, 0.0) + vec4(self.g2.y, self.g2.y, self.g2.y, self.g2.x) * other.g0 + vec4(self.g7.x) * vec4(other.g0.w, 0.0, 0.0, -other.g0.x) + vec4(self.g7.y) * vec4(0.0, other.g0.w, 0.0, -other.g0.y) + vec4(self.g7.z) * vec4(0.0, 0.0, other.g0.w, -other.g0.z) + vec4(self.g8.x) * vec4(0.0, -other.g0.z, other.g0.y, 0.0) + vec4(self.g8.y) * vec4(other.g0.z, 0.0, -other.g0.x, 0.0) + vec4(self.g8.z) * vec4(-other.g0.y, other.g0.x, 0.0, 0.0), vec4(self.g3.x) * vec4(0.0, -other.g0.z, other.g0.y, 0.0) + vec4(self.g3.y) * vec4(other.g0.z, 0.0, -other.g0.x, 0.0) + vec4(self.g3.z) * vec4(-other.g0.y, other.g0.x, 0.0, 0.0) + vec4(self.g4.x) * vec4(0.0, 0.0, 0.0, -other.g0.x) + vec4(self.g4.y) * vec4(0.0, 0.0, 0.0, -other.g0.y) + vec4(self.g4.z) * vec4(0.0, 0.0, 0.0, -other.g0.z) + vec4(self.g10.x) * other.g0, vec3(0.0) - self.g3 * vec3(other.g0.w) - vec3(self.g5.w) * vec3(other.g0.x, other.g0.y, other.g0.z) + vec3(self.g9.x) * vec3(0.0, other.g0.z, -other.g0.y) + vec3(self.g9.y) * vec3(-other.g0.z, 0.0, other.g0.x) + vec3(self.g9.z) * vec3(other.g0.y, -other.g0.x, 0.0), vec3(0.0) - self.g4 * vec3(other.g0.w) + vec3(self.g5.x) * vec3(0.0, other.g0.z, -other.g0.y) + vec3(self.g5.y) * vec3(-other.g0.z, 0.0, other.g0.x) + vec3(self.g5.z) * vec3(other.g0.y, -other.g0.x, 0.0) + self.g9 * vec3(other.g0.w) - vec3(self.g10.y) * vec3(other.g0.x, other.g0.y, other.g0.z), vec3(self.g0.y) * vec3(other.g0.x, other.g0.y, other.g0.z) + vec3(self.g6.x, self.g6.y, self.g6.z) * vec3(other.g0.w) + vec3(self.g7.x) * vec3(0.0, -other.g0.z, other.g0.y) + vec3(self.g7.y) * vec3(other.g0.z, 0.0, -other.g0.x) + vec3(self.g7.z) * vec3(-other.g0.y, other.g0.x, 0.0), vec2(self.g0.y) * vec2(0.0, other.g0.w) + vec2(self.g6.x) * vec2(other.g0.x, 0.0) + vec2(self.g6.y) * vec2(other.g0.y, 0.0) + vec2(self.g6.z, self.g6.w) * vec2(other.g0.z, other.g0.w) + vec2(self.g8.x) * vec2(0.0, -other.g0.x) + vec2(self.g8.y) * vec2(0.0, -other.g0.y) + vec2(self.g8.z) * vec2(0.0, -other.g0.z));
}

MultiVector multi_vector__anti_wedge_dot__round_point(MultiVector self, RoundPoint other) {
    return MultiVector(vec2(self.g1.x) * vec2(0.0, -other.g0.x) + vec2(self.g1.y) * vec2(0.0, -other.g0.y) + vec2(self.g1.z) * vec2(0.0, -other.g0.z) + vec2(self.g2.x) * vec2(0.0, other.g1.y) + vec2(self.g2.y) * vec2(0.0, other.g1.x) + vec2(self.g9.x) * vec2(other.g0.x, 0.0) + vec2(self.g9.y) * vec2(other.g0.y, 0.0) + vec2(self.g9.z) * vec2(other.g0.z, 0.0) + vec2(self.g10.x) * vec2(other.g1.y, 0.0) + vec2(self.g10.y) * vec2(other.g1.x, 0.0), vec3(self.g0.y) * other.g0 - vec3(self.g6.x, self.g6.y, self.g6.z) * vec3(other.g1.y) + vec3(self.g7.x) * vec3(0.0, -other.g0.z, other.g0.y) + vec3(self.g7.y) * vec3(other.g0.z, 0.0, -other.g0.x) + vec3(self.g7.z) * vec3(-other.g0.y, other.g0.x, 0.0) + self.g8 * vec3(other.g1.x), vec2(self.g0.y) * other.g1 + vec2(self.g6.x) * vec2(-other.g0.x, 0.0) + vec2(self.g6.y) * vec2(-other.g0.y, 0.0) + vec2(self.g6.z) * vec2(-other.g0.z, 0.0) + vec2(self.g6.w) * other.g1 * vec2(-1.0, 1.0) + vec2(self.g8.x) * vec2(0.0, other.g0.x) + vec2(self.g8.y) * vec2(0.0, other.g0.y) + vec2(self.g8.z) * vec2(0.0, other.g0.z), vec3(self.g3.x) * vec3(0.0, -other.g0.z, other.g0.y) + vec3(self.g3.y) * vec3(other.g0.z, 0.0, -other.g0.x) + vec3(self.g3.z) * vec3(-other.g0.y, other.g0.x, 0.0) + self.g4 * vec3(other.g1.x) + self.g9 * vec3(other.g1.x) + vec3(self.g10.x) * other.g0, self.g3 * vec3(other.g1.y) + vec3(self.g5.x, self.g5.y, self.g5.z) * vec3(other.g1.x) - vec3(self.g5.w) * other.g0 + vec3(self.g9.x) * vec3(0.0, other.g0.z, -other.g0.y) + vec3(self.g9.y) * vec3(-other.g0.z, 0.0, other.g0.x) + vec3(self.g9.z) * vec3(other.g0.y, -other.g0.x, 0.0), vec4(self.g4.x) * vec4(other.g1.y, 0.0, 0.0, other.g0.x) + vec4(self.g4.y) * vec4(0.0, other.g1.y, 0.0, other.g0.y) + vec4(self.g4.z) * vec4(0.0, 0.0, other.g1.y, other.g0.z) + vec4(self.g5.x) * vec4(0.0, other.g0.z, -other.g0.y, 0.0) + vec4(self.g5.y) * vec4(-other.g0.z, 0.0, other.g0.x, 0.0) + vec4(self.g5.z) * vec4(other.g0.y, -other.g0.x, 0.0, 0.0) + vec4(self.g9.x, self.g9.y, self.g9.z, self.g9.x) * vec4(-other.g1.y, -other.g1.y, -other.g1.y, 0.0) + vec4(self.g10.x) * vec4(0.0, 0.0, 0.0, other.g1.y) - vec4(self.g10.y) * vec4(other.g0.x, other.g0.y, other.g0.z, other.g1.x), vec4(self.g1.x, self.g1.y, self.g1.z, self.g1.x) * vec4(-other.g1.x, -other.g1.x, -other.g1.x, 0.0) + vec4(self.g2.x) * vec4(other.g0.x, other.g0.y, other.g0.z, -other.g1.y) + vec4(self.g2.y) * vec4(0.0, 0.0, 0.0, other.g1.x) + vec4(self.g6.x) * vec4(0.0, -other.g0.z, other.g0.y, 0.0) + vec4(self.g6.y) * vec4(other.g0.z, 0.0, -other.g0.x, 0.0) + vec4(self.g6.z) * vec4(-other.g0.y, other.g0.x, 0.0, 0.0) + vec4(self.g7.x) * vec4(other.g1.x, 0.0, 0.0, -other.g0.x) + vec4(self.g7.y) * vec4(0.0, other.g1.x, 0.0, -other.g0.y) + vec4(self.g7.z) * vec4(0.0, 0.0, other.g1.x, -other.g0.z), vec3(self.g1.x) * vec3(0.0, -other.g0.z, other.g0.y) + vec3(self.g1.y) * vec3(other.g0.z, 0.0, -other.g0.x) + vec3(self.g1.z) * vec3(-other.g0.y, other.g0.x, 0.0) + vec3(self.g6.x, self.g6.y, self.g6.z) * vec3(other.g1.y) + vec3(self.g6.w) * other.g0 + self.g8 * vec3(other.g1.x), self.g1 * vec3(other.g1.y) - vec3(self.g2.y) * other.g0 + self.g7 * vec3(other.g1.y) + vec3(self.g8.x) * vec3(0.0, other.g0.z, -other.g0.y) + vec3(self.g8.y) * vec3(-other.g0.z, 0.0, other.g0.x) + vec3(self.g8.z) * vec3(other.g0.y, -other.g0.x, 0.0), vec3(0.0) - vec3(self.g0.x) * other.g0 + self.g3 * vec3(other.g1.y) + vec3(self.g4.x) * vec3(0.0, other.g0.z, -other.g0.y) + vec3(self.g4.y) * vec3(-other.g0.z, 0.0, other.g0.x) + vec3(self.g4.z) * vec3(other.g0.y, -other.g0.x, 0.0) - vec3(self.g5.x, self.g5.y, self.g5.z) * vec3(other.g1.x), vec2(self.g0.x) * other.g1 + vec2(self.g3.x) * vec2(-other.g0.x, 0.0) + vec2(self.g3.y) * vec2(-other.g0.y, 0.0) + vec2(self.g3.z) * vec2(-other.g0.z, 0.0) + vec2(self.g5.x) * vec2(0.0, other.g0.x) + vec2(self.g5.y) * vec2(0.0, other.g0.y) + vec2(self.g5.z) * vec2(0.0, other.g0.z) + vec2(self.g5.w) * other.g1 * vec2(1.0, -1.0));
}

MultiVector multi_vector__anti_wedge_dot__scalar(MultiVector self, Scalar other) {
    return MultiVector(self.g0.yx * vec2(other.g0), self.g9 * vec3(other.g0), vec2(0.0) - self.g10 * vec2(other.g0), vec3(self.g6.x, self.g6.y, self.g6.z) * vec3(other.g0), self.g7 * vec3(other.g0), vec4(self.g6.w) * vec4(0.0, 0.0, 0.0, -other.g0) + vec4(self.g8.x, self.g8.y, self.g8.z, self.g8.x) * vec4(other.g0, other.g0, other.g0, 0.0), vec4(self.g3.x, self.g3.y, self.g3.z, self.g3.x) * vec4(-other.g0, -other.g0, -other.g0, 0.0) + vec4(self.g5.w) * vec4(0.0, 0.0, 0.0, other.g0), vec3(0.0) - self.g4 * vec3(other.g0), vec3(0.0) - vec3(self.g5.x, self.g5.y, self.g5.z) * vec3(other.g0), vec3(0.0) - self.g1 * vec3(other.g0), self.g2 * vec2(other.g0));
}

MultiVector multi_vector__anti_wedge_dot__sphere(MultiVector self, Sphere other) {
    return MultiVector(vec2(self.g1.x) * vec2(other.g0.x, 0.0) + vec2(self.g1.y) * vec2(other.g0.y, 0.0) + vec2(self.g1.z) * vec2(other.g0.z, 0.0) + vec2(self.g2.x) * vec2(other.g1.y, 0.0) + vec2(self.g2.y) * vec2(other.g1.x, 0.0) + vec2(self.g9.x) * vec2(0.0, other.g0.x) + vec2(self.g9.y) * vec2(0.0, other.g0.y) + vec2(self.g9.z) * vec2(0.0, other.g0.z) + vec2(self.g10.x) * vec2(0.0, -other.g1.y) + vec2(self.g10.y) * vec2(0.0, -other.g1.x), vec3(self.g0.x) * other.g0 + self.g3 * vec3(other.g1.y) + vec3(self.g4.x) * vec3(0.0, -other.g0.z, other.g0.y) + vec3(self.g4.y) * vec3(other.g0.z, 0.0, -other.g0.x) + vec3(self.g4.z) * vec3(-other.g0.y, other.g0.x, 0.0) - vec3(self.g5.x, self.g5.y, self.g5.z) * vec3(other.g1.x), vec2(0.0) - vec2(self.g0.x) * other.g1 + vec2(self.g3.x) * vec2(-other.g0.x, 0.0) + vec2(self.g3.y) * vec2(-other.g0.y, 0.0) + vec2(self.g3.z) * vec2(-other.g0.z, 0.0) + vec2(self.g5.x) * vec2(0.0, other.g0.x) + vec2(self.g5.y) * vec2(0.0, other.g0.y) + vec2(self.g5.z) * vec2(0.0, other.g0.z) + vec2(self.g5.w) * other.g1 * vec2(-1.0, 1.0), vec3(0.0) - self.g1 * vec3(other.g1.x) - vec3(self.g2.x) * other.g0 + vec3(self.g6.x) * vec3(0.0, other.g0.z, -other.g0.y) + vec3(self.g6.y) * vec3(-other.g0.z, 0.0, other.g0.x) + vec3(self.g6.z) * vec3(other.g0.y, -other.g0.x, 0.0) + self.g7 * vec3(other.g1.x), vec3(self.g1.x) * vec3(0.0, other.g0.z, -other.g0.y) + vec3(self.g1.y) * vec3(-other.g0.z, 0.0, other.g0.x) + vec3(self.g1.z) * vec3(other.g0.y, -other.g0.x, 0.0) + vec3(self.g6.x, self.g6.y, self.g6.z) * vec3(other.g1.y) - vec3(self.g6.w) * other.g0 + self.g8 * vec3(other.g1.x), vec4(self.g1.x, self.g1.y, self.g1.z, self.g1.x) * vec4(other.g1.y, other.g1.y, other.g1.y, 0.0) + vec4(self.g2.x) * vec4(0.0, 0.0, 0.0, other.g1.y) + vec4(self.g2.y) * vec4(other.g0.x, other.g0.y, other.g0.z, -other.g1.x) + vec4(self.g7.x) * vec4(other.g1.y, 0.0, 0.0, -other.g0.x) + vec4(self.g7.y) * vec4(0.0, other.g1.y, 0.0, -other.g0.y) + vec4(self.g7.z) * vec4(0.0, 0.0, other.g1.y, -other.g0.z) + vec4(self.g8.x) * vec4(0.0, -other.g0.z, other.g0.y, 0.0) + vec4(self.g8.y) * vec4(other.g0.z, 0.0, -other.g0.x, 0.0) + vec4(self.g8.z) * vec4(-other.g0.y, other.g0.x, 0.0, 0.0), vec4(self.g3.x) * vec4(0.0, -other.g0.z, other.g0.y, 0.0) + vec4(self.g3.y) * vec4(other.g0.z, 0.0, -other.g0.x, 0.0) + vec4(self.g3.z) * vec4(-other.g0.y, other.g0.x, 0.0, 0.0) + vec4(self.g4.x) * vec4(-other.g1.x, 0.0, 0.0, -other.g0.x) + vec4(self.g4.y) * vec4(0.0, -other.g1.x, 0.0, -other.g0.y) + vec4(self.g4.z) * vec4(0.0, 0.0, -other.g1.x, -other.g0.z) + vec4(self.g9.x, self.g9.y, self.g9.z, self.g9.x) * vec4(-other.g1.x, -other.g1.x, -other.g1.x, 0.0) + vec4(self.g10.x) * vec4(other.g0.x, other.g0.y, other.g0.z, other.g1.y) + vec4(self.g10.y) * vec4(0.0, 0.0, 0.0, -other.g1.x), vec3(0.0) - self.g3 * vec3(other.g1.y) - vec3(self.g5.x, self.g5.y, self.g5.z) * vec3(other.g1.x) - vec3(self.g5.w) * other.g0 + vec3(self.g9.x) * vec3(0.0, other.g0.z, -other.g0.y) + vec3(self.g9.y) * vec3(-other.g0.z, 0.0, other.g0.x) + vec3(self.g9.z) * vec3(other.g0.y, -other.g0.x, 0.0), vec3(0.0) - self.g4 * vec3(other.g1.y) + vec3(self.g5.x) * vec3(0.0, other.g0.z, -other.g0.y) + vec3(self.g5.y) * vec3(-other.g0.z, 0.0, other.g0.x) + vec3(self.g5.z) * vec3(other.g0.y, -other.g0.x, 0.0) + self.g9 * vec3(other.g1.y) - vec3(self.g10.y) * other.g0, vec3(self.g0.y) * other.g0 + vec3(self.g6.x, self.g6.y, self.g6.z) * vec3(other.g1.y) + vec3(self.g7.x) * vec3(0.0, -other.g0.z, other.g0.y) + vec3(self.g7.y) * vec3(other.g0.z, 0.0, -other.g0.x) + vec3(self.g7.z) * vec3(-other.g0.y, other.g0.x, 0.0) - self.g8 * vec3(other.g1.x), vec2(self.g0.y) * other.g1 + vec2(self.g6.x) * vec2(other.g0.x, 0.0) + vec2(self.g6.y) * vec2(other.g0.y, 0.0) + vec2(self.g6.z) * vec2(other.g0.z, 0.0) + vec2(self.g6.w) * other.g1 * vec2(-1.0, 1.0) + vec2(self.g8.x) * vec2(0.0, -other.g0.x) + vec2(self.g8.y) * vec2(0.0, -other.g0.y) + vec2(self.g8.z) * vec2(0.0, -other.g0.z));
}

Plane plane__anti_wedge_dot__anti_scalar(Plane self, AntiScalar other) {
    return Plane(self.g0 * vec4(other.g0));
}

MultiVector plane__anti_wedge_dot__circle(Plane self, Circle other) {
    return MultiVector(vec2(0.0), vec3(0.0), vec2(0.0), vec3(self.g0.x) * vec3(0.0, -other.g0.z, other.g0.y) + vec3(self.g0.y) * vec3(other.g0.z, 0.0, -other.g0.x) + vec3(self.g0.z) * vec3(-other.g0.y, other.g0.x, 0.0), vec3(0.0) - vec3(self.g0.x, self.g0.y, self.g0.z) * vec3(other.g0.w) + vec3(self.g0.w) * vec3(other.g0.x, other.g0.y, other.g0.z), vec4(self.g0.x) * vec4(0.0, other.g2.z, -other.g2.y, -other.g1.x) + vec4(self.g0.y) * vec4(-other.g2.z, 0.0, other.g2.x, -other.g1.y) + vec4(self.g0.z) * vec4(other.g2.y, -other.g2.x, 0.0, -other.g1.z) + vec4(self.g0.w) * vec4(other.g1.x, other.g1.y, other.g1.z, 0.0), vec4(0.0), vec3(0.0), vec3(0.0), vec3(self.g0.x) * vec3(0.0, -other.g1.z, other.g1.y) + vec3(self.g0.y) * vec3(other.g1.z, 0.0, -other.g1.x) + vec3(self.g0.z) * vec3(-other.g1.y, other.g1.x, 0.0) - vec3(self.g0.w) * vec3(other.g0.x, other.g0.y, other.g0.z), vec2(self.g0.x) * vec2(-other.g0.x, other.g2.x) + vec2(self.g0.y) * vec2(-other.g0.y, other.g2.y) + vec2(self.g0.z) * vec2(-other.g0.z, other.g2.z) + vec2(self.g0.w) * vec2(0.0, -other.g0.w));
}

MultiVector plane__anti_wedge_dot__dipole(Plane self, Dipole other) {
    return MultiVector(vec2(0.0), vec3(self.g0.x) * vec3(0.0, -other.g1.z, other.g1.y) + vec3(self.g0.y) * vec3(other.g1.z, 0.0, -other.g1.x) + vec3(self.g0.z) * vec3(-other.g1.y, other.g1.x, 0.0) - vec3(self.g0.w) * other.g0, vec2(self.g0.x) * vec2(other.g0.x, -other.g2.x) + vec2(self.g0.y) * vec2(other.g0.y, -other.g2.y) + vec2(self.g0.z) * vec2(other.g0.z, -other.g2.z) + vec2(self.g0.w) * vec2(0.0, -other.g2.w), vec3(0.0), vec3(0.0), vec4(0.0), vec4(self.g0.x) * vec4(0.0, other.g0.z, -other.g0.y, -other.g1.x) + vec4(self.g0.y) * vec4(-other.g0.z, 0.0, other.g0.x, -other.g1.y) + vec4(self.g0.z) * vec4(other.g0.y, -other.g0.x, 0.0, -other.g1.z), vec3(0.0) - vec3(self.g0.x, self.g0.y, self.g0.z) * vec3(other.g2.w) - vec3(self.g0.w) * other.g0, vec3(self.g0.x) * vec3(0.0, -other.g2.z, other.g2.y) + vec3(self.g0.y) * vec3(other.g2.z, 0.0, -other.g2.x) + vec3(self.g0.z) * vec3(-other.g2.y, other.g2.x, 0.0) - vec3(self.g0.w) * other.g1, vec3(0.0), vec2(0.0));
}

MultiVector plane__anti_wedge_dot__dual_num(Plane self, DualNum other) {
    return MultiVector(vec2(0.0), vec3(self.g0.x, self.g0.y, self.g0.z) * vec3(other.g0.x), vec2(self.g0.w) * vec2(0.0, -other.g0.x), vec3(0.0), vec3(0.0), vec4(0.0), vec4(0.0), vec3(0.0), vec3(0.0), vec3(self.g0.x, self.g0.y, self.g0.z) * vec3(other.g0.y), vec2(self.g0.w) * vec2(0.0, other.g0.y));
}

MultiVector plane__anti_wedge_dot__flat_point(Plane self, FlatPoint other) {
    return MultiVector(vec2(0.0), vec3(0.0), vec2(self.g0.x) * vec2(0.0, -other.g0.x) + vec2(self.g0.y) * vec2(0.0, -other.g0.y) + vec2(self.g0.z) * vec2(0.0, -other.g0.z) + vec2(self.g0.w) * vec2(0.0, -other.g0.w), vec3(0.0), vec3(0.0), vec4(0.0), vec4(0.0), vec3(0.0) - vec3(self.g0.x, self.g0.y, self.g0.z) * vec3(other.g0.w), vec3(self.g0.x) * vec3(0.0, -other.g0.z, other.g0.y) + vec3(self.g0.y) * vec3(other.g0.z, 0.0, -other.g0.x) + vec3(self.g0.z) * vec3(-other.g0.y, other.g0.x, 0.0), vec3(0.0), vec2(0.0));
}

MultiVector plane__anti_wedge_dot__flector(Plane self, Flector other) {
    return MultiVector(vec2(self.g0.x) * vec2(0.0, other.g1.x) + vec2(self.g0.y) * vec2(0.0, other.g1.y) + vec2(self.g0.z) * vec2(0.0, other.g1.z), vec3(0.0), vec2(self.g0.x) * vec2(0.0, -other.g0.x) + vec2(self.g0.y) * vec2(0.0, -other.g0.y) + vec2(self.g0.z) * vec2(0.0, -other.g0.z) + vec2(self.g0.w) * vec2(0.0, -other.g0.w), vec3(0.0), vec3(0.0), vec4(0.0), vec4(0.0), vec3(self.g0.x) * vec3(-other.g0.w, other.g1.z, -other.g1.y) + vec3(self.g0.y) * vec3(-other.g1.z, -other.g0.w, other.g1.x) + vec3(self.g0.z) * vec3(other.g1.y, -other.g1.x, -other.g0.w), vec3(self.g0.x) * vec3(other.g1.w, -other.g0.z, other.g0.y) + vec3(self.g0.y) * vec3(other.g0.z, other.g1.w, -other.g0.x) + vec3(self.g0.z) * vec3(-other.g0.y, other.g0.x, other.g1.w) - vec3(self.g0.w) * vec3(other.g1.x, other.g1.y, other.g1.z), vec3(0.0), vec2(0.0));
}

Flector plane__anti_wedge_dot__line(Plane self, Line other) {
    return Flector(vec4(self.g0.x) * vec4(0.0, other.g1.z, -other.g1.y, -other.g0.x) + vec4(self.g0.y) * vec4(-other.g1.z, 0.0, other.g1.x, -other.g0.y) + vec4(self.g0.z) * vec4(other.g1.y, -other.g1.x, 0.0, -other.g0.z) + vec4(self.g0.w) * vec4(other.g0.x, other.g0.y, other.g0.z, 0.0), vec4(self.g0.x) * vec4(0.0, -other.g0.z, other.g0.y, other.g1.x) + vec4(self.g0.y) * vec4(other.g0.z, 0.0, -other.g0.x, other.g1.y) + vec4(self.g0.z) * vec4(-other.g0.y, other.g0.x, 0.0, other.g1.z));
}

Flector plane__anti_wedge_dot__motor(Plane self, Motor other) {
    return Flector(vec4(self.g0.x) * vec4(0.0, other.g1.z, -other.g1.y, -other.g0.x) + vec4(self.g0.y) * vec4(-other.g1.z, 0.0, other.g1.x, -other.g0.y) + vec4(self.g0.z) * vec4(other.g1.y, -other.g1.x, 0.0, -other.g0.z) + vec4(self.g0.w) * vec4(other.g0.x, other.g0.y, other.g0.z, 0.0), vec4(self.g0.x) * vec4(other.g0.w, -other.g0.z, other.g0.y, other.g1.x) + vec4(self.g0.y) * vec4(other.g0.z, other.g0.w, -other.g0.x, other.g1.y) + vec4(self.g0.z) * vec4(-other.g0.y, other.g0.x, other.g0.w, other.g1.z) + vec4(self.g0.w) * vec4(0.0, 0.0, 0.0, other.g0.w));
}

MultiVector plane__anti_wedge_dot__multi_vector(Plane self, MultiVector other) {
    return MultiVector(vec2(self.g0.x) * vec2(other.g1.x, other.g9.x) + vec2(self.g0.y) * vec2(other.g1.y, other.g9.y) + vec2(self.g0.z) * vec2(other.g1.z, other.g9.z) + vec2(self.g0.w) * vec2(other.g2.x, -other.g10.x), vec3(self.g0.x) * vec3(other.g0.x, -other.g4.z, other.g4.y) + vec3(self.g0.y) * vec3(other.g4.z, other.g0.x, -other.g4.x) + vec3(self.g0.z) * vec3(-other.g4.y, other.g4.x, other.g0.x) - vec3(self.g0.w) * other.g3, vec2(self.g0.x) * vec2(other.g3.x, -other.g5.x) + vec2(self.g0.y) * vec2(other.g3.y, -other.g5.y) + vec2(self.g0.z) * vec2(other.g3.z, -other.g5.z) + vec2(self.g0.w) * vec2(0.0, -other.g5.w) + vec2(self.g0.w) * vec2(0.0, -other.g0.x), vec3(self.g0.x) * vec3(other.g2.x, -other.g6.z, other.g6.y) + vec3(self.g0.y) * vec3(other.g6.z, other.g2.x, -other.g6.x) + vec3(self.g0.z) * vec3(-other.g6.y, other.g6.x, other.g2.x), vec3(self.g0.x) * vec3(-other.g6.w, other.g1.z, -other.g1.y) + vec3(self.g0.y) * vec3(-other.g1.z, -other.g6.w, other.g1.x) + vec3(self.g0.z) * vec3(other.g1.y, -other.g1.x, -other.g6.w) + vec3(self.g0.w) * vec3(other.g6.x, other.g6.y, other.g6.z), vec4(self.g0.x) * vec4(-other.g2.y, other.g8.z, -other.g8.y, -other.g7.x) + vec4(self.g0.y) * vec4(-other.g8.z, -other.g2.y, other.g8.x, -other.g7.y) + vec4(self.g0.z) * vec4(other.g8.y, -other.g8.x, -other.g2.y, -other.g7.z) + vec4(self.g0.w) * vec4(other.g7.x, other.g7.y, other.g7.z, -other.g2.x) + vec4(self.g0.w) * vec4(-other.g1.x, -other.g1.y, -other.g1.z, 0.0), vec4(self.g0.x) * vec4(-other.g10.x, other.g3.z, -other.g3.y, -other.g4.x) + vec4(self.g0.y) * vec4(-other.g3.z, -other.g10.x, other.g3.x, -other.g4.y) + vec4(self.g0.z) * vec4(other.g3.y, -other.g3.x, -other.g10.x, -other.g4.z) + vec4(self.g0.w) * vec4(0.0, 0.0, 0.0, -other.g10.x), vec3(self.g0.x) * vec3(-other.g5.w, other.g9.z, -other.g9.y) + vec3(self.g0.y) * vec3(-other.g9.z, -other.g5.w, other.g9.x) + vec3(self.g0.z) * vec3(other.g9.y, -other.g9.x, -other.g5.w) - vec3(self.g0.w) * other.g3, vec3(self.g0.x) * vec3(other.g10.y, -other.g5.z, other.g5.y) + vec3(self.g0.y) * vec3(other.g5.z, other.g10.y, -other.g5.x) + vec3(self.g0.z) * vec3(-other.g5.y, other.g5.x, other.g10.y) - vec3(self.g0.w) * other.g9 - vec3(self.g0.w) * other.g4, vec3(self.g0.x) * vec3(other.g0.y, -other.g7.z, other.g7.y) + vec3(self.g0.y) * vec3(other.g7.z, other.g0.y, -other.g7.x) + vec3(self.g0.z) * vec3(-other.g7.y, other.g7.x, other.g0.y) - vec3(self.g0.w) * vec3(other.g6.x, other.g6.y, other.g6.z), vec2(self.g0.x) * vec2(-other.g6.x, other.g8.x) + vec2(self.g0.y) * vec2(-other.g6.y, other.g8.y) + vec2(self.g0.z) * vec2(-other.g6.z, other.g8.z) + vec2(self.g0.w) * vec2(0.0, -other.g6.w) + vec2(self.g0.w) * vec2(0.0, other.g0.y));
}

Motor plane__anti_wedge_dot__plane(Plane self, Plane other) {
    return Motor(vec4(self.g0.x) * vec4(0.0, other.g0.z, -other.g0.y, other.g0.x) + vec4(self.g0.y) * vec4(-other.g0.z, 0.0, other.g0.x, other.g0.y) + vec4(self.g0.z) * vec4(other.g0.y, -other.g0.x, 0.0, other.g0.z), vec3(self.g0.x, self.g0.y, self.g0.z) * vec3(other.g0.w) - vec3(self.g0.w) * vec3(other.g0.x, other.g0.y, other.g0.z));
}

MultiVector plane__anti_wedge_dot__round_point(Plane self, RoundPoint other) {
    return MultiVector(vec2(self.g0.x) * vec2(other.g0.x, 0.0) + vec2(self.g0.y) * vec2(other.g0.y, 0.0) + vec2(self.g0.z) * vec2(other.g0.z, 0.0) + vec2(self.g0.w) * vec2(other.g1.x, 0.0), vec3(0.0), vec2(0.0), vec3(self.g0.x, self.g0.y, self.g0.z) * vec3(other.g1.x), vec3(self.g0.x) * vec3(0.0, other.g0.z, -other.g0.y) + vec3(self.g0.y) * vec3(-other.g0.z, 0.0, other.g0.x) + vec3(self.g0.z) * vec3(other.g0.y, -other.g0.x, 0.0), self.g0.xyzx * vec4(-other.g1.y, -other.g1.y, -other.g1.y, 0.0) - vec4(self.g0.w) * vec4(other.g0.x, other.g0.y, other.g0.z, other.g1.x), vec4(0.0), vec3(0.0), vec3(0.0), vec3(0.0), vec2(0.0));
}

RoundPoint plane__anti_wedge_dot__scalar(Plane self, Scalar other) {
    return RoundPoint(vec3(self.g0.x, self.g0.y, self.g0.z) * vec3(other.g0), vec2(self.g0.w) * vec2(0.0, -other.g0));
}

MultiVector plane__anti_wedge_dot__sphere(Plane self, Sphere other) {
    return MultiVector(vec2(self.g0.x) * vec2(0.0, other.g0.x) + vec2(self.g0.y) * vec2(0.0, other.g0.y) + vec2(self.g0.z) * vec2(0.0, other.g0.z) + vec2(self.g0.w) * vec2(0.0, -other.g1.x), vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0), vec4(0.0), vec4(0.0) - self.g0 * vec4(other.g1.x), vec3(self.g0.x) * vec3(0.0, other.g0.z, -other.g0.y) + vec3(self.g0.y) * vec3(-other.g0.z, 0.0, other.g0.x) + vec3(self.g0.z) * vec3(other.g0.y, -other.g0.x, 0.0), vec3(self.g0.x, self.g0.y, self.g0.z) * vec3(other.g1.y) - vec3(self.g0.w) * other.g0, vec3(0.0), vec2(0.0));
}

RoundPoint round_point__anti_wedge_dot__anti_scalar(RoundPoint self, AntiScalar other) {
    return RoundPoint(self.g0 * vec3(other.g0), self.g1 * vec2(other.g0));
}

MultiVector round_point__anti_wedge_dot__circle(RoundPoint self, Circle other) {
    return MultiVector(vec2(0.0), vec3(self.g0.x) * vec3(0.0, -other.g1.z, other.g1.y) + vec3(self.g0.y) * vec3(other.g1.z, 0.0, -other.g1.x) + vec3(self.g0.z) * vec3(-other.g1.y, other.g1.x, 0.0) - vec3(self.g1.x) * other.g2 + vec3(self.g1.y) * vec3(other.g0.x, other.g0.y, other.g0.z), vec2(self.g0.x) * vec2(other.g0.x, -other.g2.x) + vec2(self.g0.y) * vec2(other.g0.y, -other.g2.y) + vec2(self.g0.z) * vec2(other.g0.z, -other.g2.z) + self.g1 * vec2(other.g0.w), vec3(0.0), vec3(0.0), vec4(0.0), vec4(self.g0.x) * vec4(0.0, other.g0.z, -other.g0.y, -other.g1.x) + vec4(self.g0.y) * vec4(-other.g0.z, 0.0, other.g0.x, -other.g1.y) + vec4(self.g0.z) * vec4(other.g0.y, -other.g0.x, 0.0, -other.g1.z) + vec4(self.g1.x) * vec4(other.g1.x, other.g1.y, other.g1.z, 0.0), self.g0 * vec3(other.g0.w) + vec3(self.g1.x) * other.g2 + vec3(self.g1.y) * vec3(other.g0.x, other.g0.y, other.g0.z), vec3(self.g0.x) * vec3(0.0, -other.g2.z, other.g2.y) + vec3(self.g0.y) * vec3(other.g2.z, 0.0, -other.g2.x) + vec3(self.g0.z) * vec3(-other.g2.y, other.g2.x, 0.0) + vec3(self.g1.y) * other.g1, vec3(0.0), vec2(0.0));
}

MultiVector round_point__anti_wedge_dot__dipole(RoundPoint self, Dipole other) {
    return MultiVector(vec2(0.0), vec3(0.0), vec2(0.0), vec3(self.g0.x) * vec3(0.0, other.g0.z, -other.g0.y) + vec3(self.g0.y) * vec3(-other.g0.z, 0.0, other.g0.x) + vec3(self.g0.z) * vec3(other.g0.y, -other.g0.x, 0.0) + vec3(self.g1.x) * other.g1, vec3(0.0) - self.g0 * vec3(other.g2.w) + vec3(self.g1.x) * vec3(other.g2.x, other.g2.y, other.g2.z) + vec3(self.g1.y) * other.g0, vec4(self.g0.x) * vec4(0.0, -other.g2.z, other.g2.y, other.g1.x) + vec4(self.g0.y) * vec4(other.g2.z, 0.0, -other.g2.x, other.g1.y) + vec4(self.g0.z) * vec4(-other.g2.y, other.g2.x, 0.0, other.g1.z) + vec4(self.g1.y) * vec4(other.g1.x, other.g1.y, other.g1.z, 0.0), vec4(0.0), vec3(0.0), vec3(0.0), vec3(self.g0.x) * vec3(0.0, other.g1.z, -other.g1.y) + vec3(self.g0.y) * vec3(-other.g1.z, 0.0, other.g1.x) + vec3(self.g0.z) * vec3(other.g1.y, -other.g1.x, 0.0) + vec3(self.g1.x) * vec3(other.g2.x, other.g2.y, other.g2.z) - vec3(self.g1.y) * other.g0, vec2(self.g0.x) * vec2(other.g0.x, -other.g2.x) + vec2(self.g0.y) * vec2(other.g0.y, -other.g2.y) + vec2(self.g0.z) * vec2(other.g0.z, -other.g2.z) + self.g1 * vec2(-other.g2.w));
}

MultiVector round_point__anti_wedge_dot__dual_num(RoundPoint self, DualNum other) {
    return MultiVector(vec2(0.0), self.g0 * vec3(other.g0.y), self.g1 * vec2(other.g0.y), vec3(0.0), vec3(0.0), vec4(0.0), vec4(0.0), vec3(0.0), vec3(0.0), vec3(0.0) - self.g0 * vec3(other.g0.x), self.g1 * vec2(other.g0.x));
}

MultiVector round_point__anti_wedge_dot__flat_point(RoundPoint self, FlatPoint other) {
    return MultiVector(vec2(0.0), vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0) - self.g0 * vec3(other.g0.w) + vec3(self.g1.x) * vec3(other.g0.x, other.g0.y, other.g0.z), vec4(self.g0.x) * vec4(0.0, -other.g0.z, other.g0.y, 0.0) + vec4(self.g0.y) * vec4(other.g0.z, 0.0, -other.g0.x, 0.0) + vec4(self.g0.z) * vec4(-other.g0.y, other.g0.x, 0.0, 0.0), vec4(0.0), vec3(0.0), vec3(0.0), vec3(self.g1.x) * vec3(other.g0.x, other.g0.y, other.g0.z), vec2(self.g0.x) * vec2(0.0, -other.g0.x) + vec2(self.g0.y) * vec2(0.0, -other.g0.y) + vec2(self.g0.z) * vec2(0.0, -other.g0.z) + self.g1 * vec2(-other.g0.w));
}

MultiVector round_point__anti_wedge_dot__flector(RoundPoint self, Flector other) {
    return MultiVector(vec2(self.g0.x) * vec2(other.g1.x, 0.0) + vec2(self.g0.y) * vec2(other.g1.y, 0.0) + vec2(self.g0.z) * vec2(other.g1.z, 0.0) + vec2(self.g1.x) * vec2(other.g1.w, 0.0), vec3(0.0), vec2(0.0), vec3(0.0) - vec3(self.g1.x) * vec3(other.g1.x, other.g1.y, other.g1.z), vec3(self.g0.x) * vec3(-other.g0.w, other.g1.z, -other.g1.y) + vec3(self.g0.y) * vec3(-other.g1.z, -other.g0.w, other.g1.x) + vec3(self.g0.z) * vec3(other.g1.y, -other.g1.x, -other.g0.w) + vec3(self.g1.x) * vec3(other.g0.x, other.g0.y, other.g0.z), vec4(self.g0.x) * vec4(other.g1.w, -other.g0.z, other.g0.y, 0.0) + vec4(self.g0.y) * vec4(other.g0.z, other.g1.w, -other.g0.x, 0.0) + vec4(self.g0.z) * vec4(-other.g0.y, other.g0.x, other.g1.w, 0.0) + vec4(self.g1.y, self.g1.y, self.g1.y, self.g1.x) * other.g1, vec4(0.0), vec3(0.0), vec3(0.0), vec3(self.g1.x) * vec3(other.g0.x, other.g0.y, other.g0.z), vec2(self.g0.x) * vec2(0.0, -other.g0.x) + vec2(self.g0.y) * vec2(0.0, -other.g0.y) + vec2(self.g0.z) * vec2(0.0, -other.g0.z) + self.g1 * vec2(-other.g0.w));
}

MultiVector round_point__anti_wedge_dot__line(RoundPoint self, Line other) {
    return MultiVector(vec2(0.0), vec3(self.g0.x) * vec3(0.0, -other.g0.z, other.g0.y) + vec3(self.g0.y) * vec3(other.g0.z, 0.0, -other.g0.x) + vec3(self.g0.z) * vec3(-other.g0.y, other.g0.x, 0.0) - vec3(self.g1.x) * other.g1, vec2(self.g0.x) * vec2(0.0, -other.g1.x) + vec2(self.g0.y) * vec2(0.0, -other.g1.y) + vec2(self.g0.z) * vec2(0.0, -other.g1.z), vec3(0.0), vec3(0.0), vec4(0.0), vec4(self.g0.x) * vec4(0.0, 0.0, 0.0, -other.g0.x) + vec4(self.g0.y) * vec4(0.0, 0.0, 0.0, -other.g0.y) + vec4(self.g0.z) * vec4(0.0, 0.0, 0.0, -other.g0.z) + vec4(self.g1.x) * vec4(other.g0.x, other.g0.y, other.g0.z, 0.0), vec3(self.g1.x) * other.g1, vec3(self.g0.x) * vec3(0.0, -other.g1.z, other.g1.y) + vec3(self.g0.y) * vec3(other.g1.z, 0.0, -other.g1.x) + vec3(self.g0.z) * vec3(-other.g1.y, other.g1.x, 0.0) + vec3(self.g1.y) * other.g0, vec3(0.0), vec2(0.0));
}

MultiVector round_point__anti_wedge_dot__motor(RoundPoint self, Motor other) {
    return MultiVector(vec2(0.0), vec3(self.g0.x) * vec3(other.g0.w, -other.g0.z, other.g0.y) + vec3(self.g0.y) * vec3(other.g0.z, other.g0.w, -other.g0.x) + vec3(self.g0.z) * vec3(-other.g0.y, other.g0.x, other.g0.w) - vec3(self.g1.x) * other.g1, vec2(self.g0.x) * vec2(0.0, -other.g1.x) + vec2(self.g0.y) * vec2(0.0, -other.g1.y) + vec2(self.g0.z) * vec2(0.0, -other.g1.z) + self.g1 * vec2(other.g0.w), vec3(0.0), vec3(0.0), vec4(0.0), vec4(self.g0.x) * vec4(0.0, 0.0, 0.0, -other.g0.x) + vec4(self.g0.y) * vec4(0.0, 0.0, 0.0, -other.g0.y) + vec4(self.g0.z) * vec4(0.0, 0.0, 0.0, -other.g0.z) + vec4(self.g1.x) * vec4(other.g0.x, other.g0.y, other.g0.z, 0.0), vec3(self.g1.x) * other.g1, vec3(self.g0.x) * vec3(0.0, -other.g1.z, other.g1.y) + vec3(self.g0.y) * vec3(other.g1.z, 0.0, -other.g1.x) + vec3(self.g0.z) * vec3(-other.g1.y, other.g1.x, 0.0) + vec3(self.g1.y) * vec3(other.g0.x, other.g0.y, other.g0.z), vec3(0.0), vec2(0.0));
}

MultiVector round_point__anti_wedge_dot__multi_vector(RoundPoint self, MultiVector other) {
    return MultiVector(vec2(self.g0.x) * vec2(other.g9.x, -other.g1.x) + vec2(self.g0.y) * vec2(other.g9.y, -other.g1.y) + vec2(self.g0.z) * vec2(other.g9.z, -other.g1.z) + vec2(self.g1.x) * vec2(other.g10.y, other.g2.y) + vec2(self.g1.y) * vec2(other.g10.x, other.g2.x), vec3(self.g0.x) * vec3(other.g0.y, -other.g7.z, other.g7.y) + vec3(self.g0.y) * vec3(other.g7.z, other.g0.y, -other.g7.x) + vec3(self.g0.z) * vec3(-other.g7.y, other.g7.x, other.g0.y) - vec3(self.g1.x) * other.g8 + vec3(self.g1.y) * vec3(other.g6.x, other.g6.y, other.g6.z), vec2(self.g0.x) * vec2(other.g6.x, -other.g8.x) + vec2(self.g0.y) * vec2(other.g6.y, -other.g8.y) + vec2(self.g0.z) * vec2(other.g6.z, -other.g8.z) + self.g1 * vec2(other.g6.w) + self.g1 * vec2(other.g0.y), vec3(self.g0.x) * vec3(-other.g10.x, other.g3.z, -other.g3.y) + vec3(self.g0.y) * vec3(-other.g3.z, -other.g10.x, other.g3.x) + vec3(self.g0.z) * vec3(other.g3.y, -other.g3.x, -other.g10.x) - vec3(self.g1.x) * other.g9 + vec3(self.g1.x) * other.g4, vec3(self.g0.x) * vec3(-other.g5.w, other.g9.z, -other.g9.y) + vec3(self.g0.y) * vec3(-other.g9.z, -other.g5.w, other.g9.x) + vec3(self.g0.z) * vec3(other.g9.y, -other.g9.x, -other.g5.w) + vec3(self.g1.x) * vec3(other.g5.x, other.g5.y, other.g5.z) + vec3(self.g1.y) * other.g3, vec4(self.g0.x) * vec4(other.g10.y, -other.g5.z, other.g5.y, other.g4.x) + vec4(self.g0.y) * vec4(other.g5.z, other.g10.y, -other.g5.x, other.g4.y) + vec4(self.g0.z) * vec4(-other.g5.y, other.g5.x, other.g10.y, other.g4.z) + vec4(self.g1.x) * vec4(0.0, 0.0, 0.0, other.g10.y) + vec4(self.g1.y) * vec4(other.g9.x, other.g9.y, other.g9.z, -other.g10.x) + vec4(self.g1.y) * vec4(other.g4.x, other.g4.y, other.g4.z, 0.0), vec4(self.g0.x) * vec4(-other.g2.x, other.g6.z, -other.g6.y, -other.g7.x) + vec4(self.g0.y) * vec4(-other.g6.z, -other.g2.x, other.g6.x, -other.g7.y) + vec4(self.g0.z) * vec4(other.g6.y, -other.g6.x, -other.g2.x, -other.g7.z) + vec4(self.g1.x) * vec4(other.g7.x, other.g7.y, other.g7.z, -other.g2.y) + vec4(self.g1.x) * vec4(other.g1.x, other.g1.y, other.g1.z, 0.0) + vec4(self.g1.y) * vec4(0.0, 0.0, 0.0, other.g2.x), vec3(self.g0.x) * vec3(other.g6.w, -other.g1.z, other.g1.y) + vec3(self.g0.y) * vec3(other.g1.z, other.g6.w, -other.g1.x) + vec3(self.g0.z) * vec3(-other.g1.y, other.g1.x, other.g6.w) + vec3(self.g1.x) * other.g8 + vec3(self.g1.y) * vec3(other.g6.x, other.g6.y, other.g6.z), vec3(self.g0.x) * vec3(other.g2.y, -other.g8.z, other.g8.y) + vec3(self.g0.y) * vec3(other.g8.z, other.g2.y, -other.g8.x) + vec3(self.g0.z) * vec3(-other.g8.y, other.g8.x, other.g2.y) + vec3(self.g1.y) * other.g7 - vec3(self.g1.y) * other.g1, vec3(self.g0.x) * vec3(-other.g0.x, other.g4.z, -other.g4.y) + vec3(self.g0.y) * vec3(-other.g4.z, -other.g0.x, other.g4.x) + vec3(self.g0.z) * vec3(other.g4.y, -other.g4.x, -other.g0.x) + vec3(self.g1.x) * vec3(other.g5.x, other.g5.y, other.g5.z) - vec3(self.g1.y) * other.g3, vec2(self.g0.x) * vec2(other.g3.x, -other.g5.x) + vec2(self.g0.y) * vec2(other.g3.y, -other.g5.y) + vec2(self.g0.z) * vec2(other.g3.z, -other.g5.z) + self.g1 * vec2(-other.g5.w) + self.g1 * vec2(other.g0.x));
}

MultiVector round_point__anti_wedge_dot__plane(RoundPoint self, Plane other) {
    return MultiVector(vec2(self.g0.x) * vec2(other.g0.x, 0.0) + vec2(self.g0.y) * vec2(other.g0.y, 0.0) + vec2(self.g0.z) * vec2(other.g0.z, 0.0) + vec2(self.g1.x) * vec2(other.g0.w, 0.0), vec3(0.0), vec2(0.0), vec3(0.0) - vec3(self.g1.x) * vec3(other.g0.x, other.g0.y, other.g0.z), vec3(self.g0.x) * vec3(0.0, other.g0.z, -other.g0.y) + vec3(self.g0.y) * vec3(-other.g0.z, 0.0, other.g0.x) + vec3(self.g0.z) * vec3(other.g0.y, -other.g0.x, 0.0), vec4(self.g0.x, self.g0.y, self.g0.z, self.g0.x) * vec4(other.g0.w, other.g0.w, other.g0.w, 0.0) + vec4(self.g1.y, self.g1.y, self.g1.y, self.g1.x) * other.g0, vec4(0.0), vec3(0.0), vec3(0.0), vec3(0.0), vec2(0.0));
}

MultiVector round_point__anti_wedge_dot__round_point(RoundPoint self, RoundPoint other) {
    return MultiVector(vec2(self.g0.x) * vec2(0.0, -other.g0.x) + vec2(self.g0.y) * vec2(0.0, -other.g0.y) + vec2(self.g0.z) * vec2(0.0, -other.g0.z) + vec2(self.g1.x) * vec2(0.0, other.g1.y) + vec2(self.g1.y) * vec2(0.0, other.g1.x), vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0), vec4(0.0), vec4(self.g0.x, self.g0.y, self.g0.z, self.g0.x) * vec4(-other.g1.x, -other.g1.x, -other.g1.x, 0.0) + vec4(self.g1.x) * vec4(other.g0.x, other.g0.y, other.g0.z, -other.g1.y) + vec4(self.g1.y) * vec4(0.0, 0.0, 0.0, other.g1.x), vec3(self.g0.x) * vec3(0.0, -other.g0.z, other.g0.y) + vec3(self.g0.y) * vec3(other.g0.z, 0.0, -other.g0.x) + vec3(self.g0.z) * vec3(-other.g0.y, other.g0.x, 0.0), self.g0 * vec3(other.g1.y) - vec3(self.g1.y) * other.g0, vec3(0.0), vec2(0.0));
}

Sphere round_point__anti_wedge_dot__scalar(RoundPoint self, Scalar other) {
    return Sphere(vec3(0.0) - self.g0 * vec3(other.g0), self.g1 * vec2(other.g0));
}

MultiVector round_point__anti_wedge_dot__sphere(RoundPoint self, Sphere other) {
    return MultiVector(vec2(self.g0.x) * vec2(other.g0.x, 0.0) + vec2(self.g0.y) * vec2(other.g0.y, 0.0) + vec2(self.g0.z) * vec2(other.g0.z, 0.0) + vec2(self.g1.x) * vec2(other.g1.y, 0.0) + vec2(self.g1.y) * vec2(other.g1.x, 0.0), vec3(0.0), vec2(0.0), vec3(0.0) - self.g0 * vec3(other.g1.x) - vec3(self.g1.x) * other.g0, vec3(self.g0.x) * vec3(0.0, other.g0.z, -other.g0.y) + vec3(self.g0.y) * vec3(-other.g0.z, 0.0, other.g0.x) + vec3(self.g0.z) * vec3(other.g0.y, -other.g0.x, 0.0), vec4(self.g0.x, self.g0.y, self.g0.z, self.g0.x) * vec4(other.g1.y, other.g1.y, other.g1.y, 0.0) + vec4(self.g1.x) * vec4(0.0, 0.0, 0.0, other.g1.y) + vec4(self.g1.y) * vec4(other.g0.x, other.g0.y, other.g0.z, -other.g1.x), vec4(0.0), vec3(0.0), vec3(0.0), vec3(0.0), vec2(0.0));
}

Scalar scalar__anti_wedge_dot__anti_scalar(Scalar self, AntiScalar other) {
    return Scalar(self.g0 * other.g0);
}

Dipole scalar__anti_wedge_dot__circle(Scalar self, Circle other) {
    return Dipole(vec3(self.g0) * vec3(other.g0.x, other.g0.y, other.g0.z), vec3(self.g0) * other.g1, vec4(self.g0) * vec4(other.g2.x, other.g2.y, other.g2.z, -other.g0.w));
}

Circle scalar__anti_wedge_dot__dipole(Scalar self, Dipole other) {
    return Circle(vec4(self.g0) * vec4(-other.g0.x, -other.g0.y, -other.g0.z, other.g2.w), vec3(0.0) - vec3(self.g0) * other.g1, vec3(0.0) - vec3(self.g0) * vec3(other.g2.x, other.g2.y, other.g2.z));
}

DualNum scalar__anti_wedge_dot__dual_num(Scalar self, DualNum other) {
    return DualNum(vec2(self.g0) * other.g0.yx * vec2(1.0, -1.0));
}

Circle scalar__anti_wedge_dot__flat_point(Scalar self, FlatPoint other) {
    return Circle(vec4(self.g0) * vec4(0.0, 0.0, 0.0, other.g0.w), vec3(0.0), vec3(0.0) - vec3(self.g0) * vec3(other.g0.x, other.g0.y, other.g0.z));
}

MultiVector scalar__anti_wedge_dot__flector(Scalar self, Flector other) {
    return MultiVector(vec2(0.0), vec3(self.g0) * vec3(other.g1.x, other.g1.y, other.g1.z), vec2(self.g0) * vec2(0.0, -other.g1.w), vec3(0.0), vec3(0.0), vec4(0.0), vec4(self.g0) * vec4(0.0, 0.0, 0.0, other.g0.w), vec3(0.0), vec3(0.0) - vec3(self.g0) * vec3(other.g0.x, other.g0.y, other.g0.z), vec3(0.0), vec2(0.0));
}

Dipole scalar__anti_wedge_dot__line(Scalar self, Line other) {
    return Dipole(vec3(0.0), vec3(self.g0) * other.g0, vec4(self.g0) * vec4(other.g1.x, other.g1.y, other.g1.z, 0.0));
}

MultiVector scalar__anti_wedge_dot__motor(Scalar self, Motor other) {
    return MultiVector(vec2(self.g0) * vec2(other.g0.w, 0.0), vec3(0.0), vec2(0.0), vec3(0.0), vec3(self.g0) * vec3(other.g0.x, other.g0.y, other.g0.z), vec4(self.g0) * vec4(other.g1.x, other.g1.y, other.g1.z, 0.0), vec4(0.0), vec3(0.0), vec3(0.0), vec3(0.0), vec2(0.0));
}

MultiVector scalar__anti_wedge_dot__multi_vector(Scalar self, MultiVector other) {
    return MultiVector(vec2(self.g0) * other.g0.yx * vec2(1.0, -1.0), vec3(self.g0) * other.g9, vec2(0.0) - vec2(self.g0) * other.g10, vec3(self.g0) * vec3(other.g6.x, other.g6.y, other.g6.z), vec3(self.g0) * other.g7, vec4(self.g0) * vec4(other.g8.x, other.g8.y, other.g8.z, -other.g6.w), vec4(self.g0) * vec4(-other.g3.x, -other.g3.y, -other.g3.z, other.g5.w), vec3(0.0) - vec3(self.g0) * other.g4, vec3(0.0) - vec3(self.g0) * vec3(other.g5.x, other.g5.y, other.g5.z), vec3(0.0) - vec3(self.g0) * other.g1, vec2(self.g0) * other.g2);
}

RoundPoint scalar__anti_wedge_dot__plane(Scalar self, Plane other) {
    return RoundPoint(vec3(self.g0) * vec3(other.g0.x, other.g0.y, other.g0.z), vec2(self.g0) * vec2(0.0, -other.g0.w));
}

Sphere scalar__anti_wedge_dot__round_point(Scalar self, RoundPoint other) {
    return Sphere(vec3(0.0) - vec3(self.g0) * other.g0, vec2(self.g0) * other.g1);
}

AntiScalar scalar__anti_wedge_dot__scalar(Scalar self, Scalar other) {
    return AntiScalar(0.0 - self.g0 * other.g0);
}

RoundPoint scalar__anti_wedge_dot__sphere(Scalar self, Sphere other) {
    return RoundPoint(vec3(self.g0) * other.g0, vec2(0.0) - vec2(self.g0) * other.g1);
}

Sphere sphere__anti_wedge_dot__anti_scalar(Sphere self, AntiScalar other) {
    return Sphere(self.g0 * vec3(other.g0), self.g1 * vec2(other.g0));
}

MultiVector sphere__anti_wedge_dot__circle(Sphere self, Circle other) {
    return MultiVector(vec2(0.0), vec3(0.0), vec2(0.0), vec3(self.g0.x) * vec3(0.0, -other.g0.z, other.g0.y) + vec3(self.g0.y) * vec3(other.g0.z, 0.0, -other.g0.x) + vec3(self.g0.z) * vec3(-other.g0.y, other.g0.x, 0.0) + vec3(self.g1.x) * other.g1, vec3(0.0) - self.g0 * vec3(other.g0.w) + vec3(self.g1.x) * other.g2 + vec3(self.g1.y) * vec3(other.g0.x, other.g0.y, other.g0.z), vec4(self.g0.x) * vec4(0.0, other.g2.z, -other.g2.y, -other.g1.x) + vec4(self.g0.y) * vec4(-other.g2.z, 0.0, other.g2.x, -other.g1.y) + vec4(self.g0.z) * vec4(other.g2.y, -other.g2.x, 0.0, -other.g1.z) + vec4(self.g1.y) * vec4(other.g1.x, other.g1.y, other.g1.z, 0.0), vec4(0.0), vec3(0.0), vec3(0.0), vec3(self.g0.x) * vec3(0.0, -other.g1.z, other.g1.y) + vec3(self.g0.y) * vec3(other.g1.z, 0.0, -other.g1.x) + vec3(self.g0.z) * vec3(-other.g1.y, other.g1.x, 0.0) + vec3(self.g1.x) * other.g2 - vec3(self.g1.y) * vec3(other.g0.x, other.g0.y, other.g0.z), vec2(self.g0.x) * vec2(-other.g0.x, other.g2.x) + vec2(self.g0.y) * vec2(-other.g0.y, other.g2.y) + vec2(self.g0.z) * vec2(-other.g0.z, other.g2.z) + self.g1 * vec2(other.g0.w));
}

MultiVector sphere__anti_wedge_dot__dipole(Sphere self, Dipole other) {
    return MultiVector(vec2(0.0), vec3(self.g0.x) * vec3(0.0, -other.g1.z, other.g1.y) + vec3(self.g0.y) * vec3(other.g1.z, 0.0, -other.g1.x) + vec3(self.g0.z) * vec3(-other.g1.y, other.g1.x, 0.0) + vec3(self.g1.x) * vec3(other.g2.x, other.g2.y, other.g2.z) - vec3(self.g1.y) * other.g0, vec2(self.g0.x) * vec2(other.g0.x, -other.g2.x) + vec2(self.g0.y) * vec2(other.g0.y, -other.g2.y) + vec2(self.g0.z) * vec2(other.g0.z, -other.g2.z) + self.g1 * vec2(other.g2.w), vec3(0.0), vec3(0.0), vec4(0.0), vec4(self.g0.x) * vec4(0.0, other.g0.z, -other.g0.y, -other.g1.x) + vec4(self.g0.y) * vec4(-other.g0.z, 0.0, other.g0.x, -other.g1.y) + vec4(self.g0.z) * vec4(other.g0.y, -other.g0.x, 0.0, -other.g1.z) + vec4(self.g1.x) * vec4(-other.g1.x, -other.g1.y, -other.g1.z, 0.0), vec3(0.0) - self.g0 * vec3(other.g2.w) - vec3(self.g1.x) * vec3(other.g2.x, other.g2.y, other.g2.z) - vec3(self.g1.y) * other.g0, vec3(self.g0.x) * vec3(0.0, -other.g2.z, other.g2.y) + vec3(self.g0.y) * vec3(other.g2.z, 0.0, -other.g2.x) + vec3(self.g0.z) * vec3(-other.g2.y, other.g2.x, 0.0) - vec3(self.g1.y) * other.g1, vec3(0.0), vec2(0.0));
}

MultiVector sphere__anti_wedge_dot__dual_num(Sphere self, DualNum other) {
    return MultiVector(vec2(0.0), self.g0 * vec3(other.g0.x), vec2(0.0) - self.g1 * vec2(other.g0.x), vec3(0.0), vec3(0.0), vec4(0.0), vec4(0.0), vec3(0.0), vec3(0.0), self.g0 * vec3(other.g0.y), self.g1 * vec2(other.g0.y));
}

MultiVector sphere__anti_wedge_dot__flat_point(Sphere self, FlatPoint other) {
    return MultiVector(vec2(0.0), vec3(self.g1.x) * vec3(other.g0.x, other.g0.y, other.g0.z), vec2(self.g0.x) * vec2(0.0, -other.g0.x) + vec2(self.g0.y) * vec2(0.0, -other.g0.y) + vec2(self.g0.z) * vec2(0.0, -other.g0.z) + self.g1 * vec2(other.g0.w), vec3(0.0), vec3(0.0), vec4(0.0), vec4(0.0), vec3(0.0) - self.g0 * vec3(other.g0.w) - vec3(self.g1.x) * vec3(other.g0.x, other.g0.y, other.g0.z), vec3(self.g0.x) * vec3(0.0, -other.g0.z, other.g0.y) + vec3(self.g0.y) * vec3(other.g0.z, 0.0, -other.g0.x) + vec3(self.g0.z) * vec3(-other.g0.y, other.g0.x, 0.0), vec3(0.0), vec2(0.0));
}

MultiVector sphere__anti_wedge_dot__flector(Sphere self, Flector other) {
    return MultiVector(vec2(self.g0.x) * vec2(0.0, other.g1.x) + vec2(self.g0.y) * vec2(0.0, other.g1.y) + vec2(self.g0.z) * vec2(0.0, other.g1.z) + vec2(self.g1.x) * vec2(0.0, -other.g1.w), vec3(self.g1.x) * vec3(other.g0.x, other.g0.y, other.g0.z), vec2(self.g0.x) * vec2(0.0, -other.g0.x) + vec2(self.g0.y) * vec2(0.0, -other.g0.y) + vec2(self.g0.z) * vec2(0.0, -other.g0.z) + self.g1 * vec2(other.g0.w), vec3(0.0), vec3(0.0), vec4(0.0), vec4(self.g1.x) * other.g1, vec3(self.g0.x) * vec3(-other.g0.w, other.g1.z, -other.g1.y) + vec3(self.g0.y) * vec3(-other.g1.z, -other.g0.w, other.g1.x) + vec3(self.g0.z) * vec3(other.g1.y, -other.g1.x, -other.g0.w) - vec3(self.g1.x) * vec3(other.g0.x, other.g0.y, other.g0.z), vec3(self.g0.x) * vec3(other.g1.w, -other.g0.z, other.g0.y) + vec3(self.g0.y) * vec3(other.g0.z, other.g1.w, -other.g0.x) + vec3(self.g0.z) * vec3(-other.g0.y, other.g0.x, other.g1.w) - vec3(self.g1.y) * vec3(other.g1.x, other.g1.y, other.g1.z), vec3(0.0), vec2(0.0));
}

MultiVector sphere__anti_wedge_dot__line(Sphere self, Line other) {
    return MultiVector(vec2(0.0), vec3(0.0), vec2(0.0), vec3(self.g1.x) * other.g0, vec3(self.g1.x) * other.g1, vec4(self.g0.x) * vec4(0.0, other.g1.z, -other.g1.y, -other.g0.x) + vec4(self.g0.y) * vec4(-other.g1.z, 0.0, other.g1.x, -other.g0.y) + vec4(self.g0.z) * vec4(other.g1.y, -other.g1.x, 0.0, -other.g0.z) + vec4(self.g1.y) * vec4(other.g0.x, other.g0.y, other.g0.z, 0.0), vec4(0.0), vec3(0.0), vec3(0.0), vec3(self.g0.x) * vec3(0.0, -other.g0.z, other.g0.y) + vec3(self.g0.y) * vec3(other.g0.z, 0.0, -other.g0.x) + vec3(self.g0.z) * vec3(-other.g0.y, other.g0.x, 0.0) + vec3(self.g1.x) * other.g1, vec2(self.g0.x) * vec2(0.0, other.g1.x) + vec2(self.g0.y) * vec2(0.0, other.g1.y) + vec2(self.g0.z) * vec2(0.0, other.g1.z));
}

MultiVector sphere__anti_wedge_dot__motor(Sphere self, Motor other) {
    return MultiVector(vec2(0.0), vec3(0.0), vec2(0.0), vec3(self.g1.x) * vec3(other.g0.x, other.g0.y, other.g0.z), vec3(self.g1.x) * other.g1, vec4(self.g0.x) * vec4(0.0, other.g1.z, -other.g1.y, -other.g0.x) + vec4(self.g0.y) * vec4(-other.g1.z, 0.0, other.g1.x, -other.g0.y) + vec4(self.g0.z) * vec4(other.g1.y, -other.g1.x, 0.0, -other.g0.z) + vec4(self.g1.y) * vec4(other.g0.x, other.g0.y, other.g0.z, 0.0), vec4(0.0), vec3(0.0), vec3(0.0), vec3(self.g0.x) * vec3(other.g0.w, -other.g0.z, other.g0.y) + vec3(self.g0.y) * vec3(other.g0.z, other.g0.w, -other.g0.x) + vec3(self.g0.z) * vec3(-other.g0.y, other.g0.x, other.g0.w) + vec3(self.g1.x) * other.g1, vec2(self.g0.x) * vec2(0.0, other.g1.x) + vec2(self.g0.y) * vec2(0.0, other.g1.y) + vec2(self.g0.z) * vec2(0.0, other.g1.z) + self.g1 * vec2(other.g0.w));
}

MultiVector sphere__anti_wedge_dot__multi_vector(Sphere self, MultiVector other) {
    return MultiVector(vec2(self.g0.x) * vec2(other.g1.x, other.g9.x) + vec2(self.g0.y) * vec2(other.g1.y, other.g9.y) + vec2(self.g0.z) * vec2(other.g1.z, other.g9.z) + vec2(self.g1.x) * vec2(other.g2.y, -other.g10.y) + vec2(self.g1.y) * vec2(other.g2.x, -other.g10.x), vec3(self.g0.x) * vec3(other.g0.x, -other.g4.z, other.g4.y) + vec3(self.g0.y) * vec3(other.g4.z, other.g0.x, -other.g4.x) + vec3(self.g0.z) * vec3(-other.g4.y, other.g4.x, other.g0.x) + vec3(self.g1.x) * vec3(other.g5.x, other.g5.y, other.g5.z) - vec3(self.g1.y) * other.g3, vec2(self.g0.x) * vec2(other.g3.x, -other.g5.x) + vec2(self.g0.y) * vec2(other.g3.y, -other.g5.y) + vec2(self.g0.z) * vec2(other.g3.z, -other.g5.z) + self.g1 * vec2(other.g5.w) - self.g1 * vec2(other.g0.x), vec3(self.g0.x) * vec3(other.g2.x, -other.g6.z, other.g6.y) + vec3(self.g0.y) * vec3(other.g6.z, other.g2.x, -other.g6.x) + vec3(self.g0.z) * vec3(-other.g6.y, other.g6.x, other.g2.x) + vec3(self.g1.x) * other.g7 + vec3(self.g1.x) * other.g1, vec3(self.g0.x) * vec3(-other.g6.w, other.g1.z, -other.g1.y) + vec3(self.g0.y) * vec3(-other.g1.z, -other.g6.w, other.g1.x) + vec3(self.g0.z) * vec3(other.g1.y, -other.g1.x, -other.g6.w) + vec3(self.g1.x) * other.g8 + vec3(self.g1.y) * vec3(other.g6.x, other.g6.y, other.g6.z), vec4(self.g0.x) * vec4(-other.g2.y, other.g8.z, -other.g8.y, -other.g7.x) + vec4(self.g0.y) * vec4(-other.g8.z, -other.g2.y, other.g8.x, -other.g7.y) + vec4(self.g0.z) * vec4(other.g8.y, -other.g8.x, -other.g2.y, -other.g7.z) + vec4(self.g1.x) * vec4(0.0, 0.0, 0.0, other.g2.y) + vec4(self.g1.y) * vec4(other.g7.x, other.g7.y, other.g7.z, -other.g2.x) + vec4(self.g1.y) * vec4(-other.g1.x, -other.g1.y, -other.g1.z, 0.0), vec4(self.g0.x) * vec4(-other.g10.x, other.g3.z, -other.g3.y, -other.g4.x) + vec4(self.g0.y) * vec4(-other.g3.z, -other.g10.x, other.g3.x, -other.g4.y) + vec4(self.g0.z) * vec4(other.g3.y, -other.g3.x, -other.g10.x, -other.g4.z) + vec4(self.g1.x) * vec4(other.g9.x, other.g9.y, other.g9.z, other.g10.y) + vec4(self.g1.x) * vec4(-other.g4.x, -other.g4.y, -other.g4.z, 0.0) + vec4(self.g1.y) * vec4(0.0, 0.0, 0.0, -other.g10.x), vec3(self.g0.x) * vec3(-other.g5.w, other.g9.z, -other.g9.y) + vec3(self.g0.y) * vec3(-other.g9.z, -other.g5.w, other.g9.x) + vec3(self.g0.z) * vec3(other.g9.y, -other.g9.x, -other.g5.w) - vec3(self.g1.x) * vec3(other.g5.x, other.g5.y, other.g5.z) - vec3(self.g1.y) * other.g3, vec3(self.g0.x) * vec3(other.g10.y, -other.g5.z, other.g5.y) + vec3(self.g0.y) * vec3(other.g5.z, other.g10.y, -other.g5.x) + vec3(self.g0.z) * vec3(-other.g5.y, other.g5.x, other.g10.y) - vec3(self.g1.y) * other.g9 - vec3(self.g1.y) * other.g4, vec3(self.g0.x) * vec3(other.g0.y, -other.g7.z, other.g7.y) + vec3(self.g0.y) * vec3(other.g7.z, other.g0.y, -other.g7.x) + vec3(self.g0.z) * vec3(-other.g7.y, other.g7.x, other.g0.y) + vec3(self.g1.x) * other.g8 - vec3(self.g1.y) * vec3(other.g6.x, other.g6.y, other.g6.z), vec2(self.g0.x) * vec2(-other.g6.x, other.g8.x) + vec2(self.g0.y) * vec2(-other.g6.y, other.g8.y) + vec2(self.g0.z) * vec2(-other.g6.z, other.g8.z) + self.g1 * vec2(other.g6.w) + self.g1 * vec2(other.g0.y));
}

MultiVector sphere__anti_wedge_dot__plane(Sphere self, Plane other) {
    return MultiVector(vec2(self.g0.x) * vec2(0.0, other.g0.x) + vec2(self.g0.y) * vec2(0.0, other.g0.y) + vec2(self.g0.z) * vec2(0.0, other.g0.z) + vec2(self.g1.x) * vec2(0.0, -other.g0.w), vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0), vec4(0.0), vec4(self.g1.x) * other.g0, vec3(self.g0.x) * vec3(0.0, other.g0.z, -other.g0.y) + vec3(self.g0.y) * vec3(-other.g0.z, 0.0, other.g0.x) + vec3(self.g0.z) * vec3(other.g0.y, -other.g0.x, 0.0), self.g0 * vec3(other.g0.w) - vec3(self.g1.y) * vec3(other.g0.x, other.g0.y, other.g0.z), vec3(0.0), vec2(0.0));
}

MultiVector sphere__anti_wedge_dot__round_point(Sphere self, RoundPoint other) {
    return MultiVector(vec2(self.g0.x) * vec2(other.g0.x, 0.0) + vec2(self.g0.y) * vec2(other.g0.y, 0.0) + vec2(self.g0.z) * vec2(other.g0.z, 0.0) + vec2(self.g1.x) * vec2(other.g1.y, 0.0) + vec2(self.g1.y) * vec2(other.g1.x, 0.0), vec3(0.0), vec2(0.0), self.g0 * vec3(other.g1.x) + vec3(self.g1.x) * other.g0, vec3(self.g0.x) * vec3(0.0, other.g0.z, -other.g0.y) + vec3(self.g0.y) * vec3(-other.g0.z, 0.0, other.g0.x) + vec3(self.g0.z) * vec3(other.g0.y, -other.g0.x, 0.0), vec4(self.g0.x, self.g0.y, self.g0.z, self.g0.x) * vec4(-other.g1.y, -other.g1.y, -other.g1.y, 0.0) + vec4(self.g1.x) * vec4(0.0, 0.0, 0.0, other.g1.y) - vec4(self.g1.y) * vec4(other.g0.x, other.g0.y, other.g0.z, other.g1.x), vec4(0.0), vec3(0.0), vec3(0.0), vec3(0.0), vec2(0.0));
}

RoundPoint sphere__anti_wedge_dot__scalar(Sphere self, Scalar other) {
    return RoundPoint(self.g0 * vec3(other.g0), vec2(0.0) - self.g1 * vec2(other.g0));
}

MultiVector sphere__anti_wedge_dot__sphere(Sphere self, Sphere other) {
    return MultiVector(vec2(self.g0.x) * vec2(0.0, other.g0.x) + vec2(self.g0.y) * vec2(0.0, other.g0.y) + vec2(self.g0.z) * vec2(0.0, other.g0.z) + vec2(self.g1.x) * vec2(0.0, -other.g1.y) + vec2(self.g1.y) * vec2(0.0, -other.g1.x), vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0), vec4(0.0), vec4(self.g0.x, self.g0.y, self.g0.z, self.g0.x) * vec4(-other.g1.x, -other.g1.x, -other.g1.x, 0.0) + vec4(self.g1.x) * vec4(other.g0.x, other.g0.y, other.g0.z, other.g1.y) + vec4(self.g1.y) * vec4(0.0, 0.0, 0.0, -other.g1.x), vec3(self.g0.x) * vec3(0.0, other.g0.z, -other.g0.y) + vec3(self.g0.y) * vec3(-other.g0.z, 0.0, other.g0.x) + vec3(self.g0.z) * vec3(other.g0.y, -other.g0.x, 0.0), self.g0 * vec3(other.g1.y) - vec3(self.g1.y) * other.g0, vec3(0.0), vec2(0.0));
}

Scalar anti_scalar__wedge_dot__anti_scalar(AntiScalar self, AntiScalar other) {
    return Scalar(0.0 - self.g0 * other.g0);
}

Dipole anti_scalar__wedge_dot__circle(AntiScalar self, Circle other) {
    return Dipole(vec3(0.0) - vec3(self.g0) * vec3(other.g0.x, other.g0.y, other.g0.z), vec3(0.0) - vec3(self.g0) * other.g1, vec4(self.g0) * vec4(-other.g2.x, -other.g2.y, -other.g2.z, other.g0.w));
}

Circle anti_scalar__wedge_dot__dipole(AntiScalar self, Dipole other) {
    return Circle(vec4(self.g0) * vec4(other.g0.x, other.g0.y, other.g0.z, -other.g2.w), vec3(self.g0) * other.g1, vec3(self.g0) * vec3(other.g2.x, other.g2.y, other.g2.z));
}

DualNum anti_scalar__wedge_dot__dual_num(AntiScalar self, DualNum other) {
    return DualNum(vec2(self.g0) * other.g0.yx * vec2(-1.0, 1.0));
}

Circle anti_scalar__wedge_dot__flat_point(AntiScalar self, FlatPoint other) {
    return Circle(vec4(self.g0) * vec4(0.0, 0.0, 0.0, -other.g0.w), vec3(0.0), vec3(self.g0) * vec3(other.g0.x, other.g0.y, other.g0.z));
}

MultiVector anti_scalar__wedge_dot__flector(AntiScalar self, Flector other) {
    return MultiVector(vec2(0.0), vec3(0.0) - vec3(self.g0) * vec3(other.g1.x, other.g1.y, other.g1.z), vec2(self.g0) * vec2(0.0, other.g1.w), vec3(0.0), vec3(0.0), vec4(0.0), vec4(self.g0) * vec4(0.0, 0.0, 0.0, -other.g0.w), vec3(0.0), vec3(self.g0) * vec3(other.g0.x, other.g0.y, other.g0.z), vec3(0.0), vec2(0.0));
}

Dipole anti_scalar__wedge_dot__line(AntiScalar self, Line other) {
    return Dipole(vec3(0.0), vec3(0.0) - vec3(self.g0) * other.g0, vec4(self.g0) * vec4(-other.g1.x, -other.g1.y, -other.g1.z, 0.0));
}

MultiVector anti_scalar__wedge_dot__motor(AntiScalar self, Motor other) {
    return MultiVector(vec2(self.g0) * vec2(-other.g0.w, 0.0), vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0) - vec3(self.g0) * vec3(other.g0.x, other.g0.y, other.g0.z), vec4(self.g0) * vec4(-other.g1.x, -other.g1.y, -other.g1.z, 0.0), vec4(0.0), vec3(0.0), vec3(0.0), vec3(0.0), vec2(0.0));
}

MultiVector anti_scalar__wedge_dot__multi_vector(AntiScalar self, MultiVector other) {
    return MultiVector(vec2(self.g0) * other.g0.yx * vec2(-1.0, 1.0), vec3(0.0) - vec3(self.g0) * other.g9, vec2(self.g0) * other.g10, vec3(0.0) - vec3(self.g0) * vec3(other.g6.x, other.g6.y, other.g6.z), vec3(0.0) - vec3(self.g0) * other.g7, vec4(self.g0) * vec4(-other.g8.x, -other.g8.y, -other.g8.z, other.g6.w), vec4(self.g0) * vec4(other.g3.x, other.g3.y, other.g3.z, -other.g5.w), vec3(self.g0) * other.g4, vec3(self.g0) * vec3(other.g5.x, other.g5.y, other.g5.z), vec3(self.g0) * other.g1, vec2(0.0) - vec2(self.g0) * other.g2);
}

RoundPoint anti_scalar__wedge_dot__plane(AntiScalar self, Plane other) {
    return RoundPoint(vec3(0.0) - vec3(self.g0) * vec3(other.g0.x, other.g0.y, other.g0.z), vec2(self.g0) * vec2(0.0, other.g0.w));
}

Sphere anti_scalar__wedge_dot__round_point(AntiScalar self, RoundPoint other) {
    return Sphere(vec3(self.g0) * other.g0, vec2(0.0) - vec2(self.g0) * other.g1);
}

AntiScalar anti_scalar__wedge_dot__scalar(AntiScalar self, Scalar other) {
    return AntiScalar(self.g0 * other.g0);
}

RoundPoint anti_scalar__wedge_dot__sphere(AntiScalar self, Sphere other) {
    return RoundPoint(vec3(0.0) - vec3(self.g0) * other.g0, vec2(self.g0) * other.g1);
}

Dipole circle__wedge_dot__anti_scalar(Circle self, AntiScalar other) {
    return Dipole(vec3(0.0) - vec3(self.g0.x, self.g0.y, self.g0.z) * vec3(other.g0), vec3(0.0) - self.g1 * vec3(other.g0), vec4(self.g0.w) * vec4(0.0, 0.0, 0.0, other.g0) + vec4(self.g2.x, self.g2.y, self.g2.z, self.g2.x) * vec4(-other.g0, -other.g0, -other.g0, 0.0));
}

MultiVector circle__wedge_dot__circle(Circle self, Circle other) {
    return MultiVector(vec2(self.g0.x) * vec2(other.g2.x, 0.0) + vec2(self.g0.y) * vec2(other.g2.y, 0.0) + vec2(self.g0.z) * vec2(other.g2.z, 0.0) + vec2(self.g0.w) * vec2(-other.g0.w, 0.0) + vec2(self.g1.x) * vec2(other.g1.x, 0.0) + vec2(self.g1.y) * vec2(other.g1.y, 0.0) + vec2(self.g1.z) * vec2(other.g1.z, 0.0) + vec2(self.g2.x) * vec2(other.g0.x, 0.0) + vec2(self.g2.y) * vec2(other.g0.y, 0.0) + vec2(self.g2.z) * vec2(other.g0.z, 0.0), vec3(0.0), vec2(0.0), vec3(self.g0.x) * vec3(other.g0.w, -other.g1.z, other.g1.y) + vec3(self.g0.y) * vec3(other.g1.z, other.g0.w, -other.g1.x) + vec3(self.g0.z) * vec3(-other.g1.y, other.g1.x, other.g0.w) - vec3(self.g0.w) * vec3(other.g0.x, other.g0.y, other.g0.z) + vec3(self.g1.x) * vec3(0.0, -other.g0.z, other.g0.y) + vec3(self.g1.y) * vec3(other.g0.z, 0.0, -other.g0.x) + vec3(self.g1.z) * vec3(-other.g0.y, other.g0.x, 0.0), vec3(self.g0.x) * vec3(0.0, -other.g2.z, other.g2.y) + vec3(self.g0.y) * vec3(other.g2.z, 0.0, -other.g2.x) + vec3(self.g0.z) * vec3(-other.g2.y, other.g2.x, 0.0) + vec3(self.g1.x) * vec3(0.0, -other.g1.z, other.g1.y) + vec3(self.g1.y) * vec3(other.g1.z, 0.0, -other.g1.x) + vec3(self.g1.z) * vec3(-other.g1.y, other.g1.x, 0.0) + vec3(self.g2.x) * vec3(0.0, -other.g0.z, other.g0.y) + vec3(self.g2.y) * vec3(other.g0.z, 0.0, -other.g0.x) + vec3(self.g2.z) * vec3(-other.g0.y, other.g0.x, 0.0), vec4(self.g0.x) * vec4(0.0, 0.0, 0.0, -other.g2.x) + vec4(self.g0.y) * vec4(0.0, 0.0, 0.0, -other.g2.y) + self.g0.wwwz * vec4(other.g2.x, other.g2.y, other.g2.z, -other.g2.z) + vec4(self.g1.x) * vec4(0.0, -other.g2.z, other.g2.y, 0.0) + vec4(self.g1.y) * vec4(other.g2.z, 0.0, -other.g2.x, 0.0) + vec4(self.g1.z) * vec4(-other.g2.y, other.g2.x, 0.0, 0.0) + vec4(self.g2.x) * vec4(-other.g0.w, -other.g1.z, other.g1.y, other.g0.x) + vec4(self.g2.y) * vec4(other.g1.z, -other.g0.w, -other.g1.x, other.g0.y) + vec4(self.g2.z) * vec4(-other.g1.y, other.g1.x, -other.g0.w, other.g0.z), vec4(0.0), vec3(0.0), vec3(0.0), vec3(self.g0.x) * vec3(0.0, other.g2.z, -other.g2.y) + vec3(self.g0.y) * vec3(-other.g2.z, 0.0, other.g2.x) + vec3(self.g0.z) * vec3(other.g2.y, -other.g2.x, 0.0) + vec3(self.g0.w) * other.g1 + self.g1 * vec3(other.g0.w) + vec3(self.g2.x) * vec3(0.0, -other.g0.z, other.g0.y) + vec3(self.g2.y) * vec3(other.g0.z, 0.0, -other.g0.x) + vec3(self.g2.z) * vec3(-other.g0.y, other.g0.x, 0.0), vec2(self.g0.x) * vec2(other.g1.x, 0.0) + vec2(self.g0.y) * vec2(other.g1.y, 0.0) + vec2(self.g0.z) * vec2(other.g1.z, 0.0) + vec2(self.g1.x) * vec2(other.g0.x, other.g2.x) + vec2(self.g1.y) * vec2(other.g0.y, other.g2.y) + vec2(self.g1.z) * vec2(other.g0.z, other.g2.z) + vec2(self.g2.x) * vec2(0.0, other.g1.x) + vec2(self.g2.y) * vec2(0.0, other.g1.y) + vec2(self.g2.z) * vec2(0.0, other.g1.z));
}

MultiVector circle__wedge_dot__dipole(Circle self, Dipole other) {
    return MultiVector(vec2(self.g0.x) * vec2(0.0, -other.g2.x) + vec2(self.g0.y) * vec2(0.0, -other.g2.y) + vec2(self.g0.z) * vec2(0.0, -other.g2.z) + vec2(self.g0.w) * vec2(0.0, -other.g2.w) + vec2(self.g1.x) * vec2(0.0, -other.g1.x) + vec2(self.g1.y) * vec2(0.0, -other.g1.y) + vec2(self.g1.z) * vec2(0.0, -other.g1.z) + vec2(self.g2.x) * vec2(0.0, -other.g0.x) + vec2(self.g2.y) * vec2(0.0, -other.g0.y) + vec2(self.g2.z) * vec2(0.0, -other.g0.z), vec3(self.g0.x) * vec3(0.0, other.g2.z, -other.g2.y) + vec3(self.g0.y) * vec3(-other.g2.z, 0.0, other.g2.x) + vec3(self.g0.z) * vec3(other.g2.y, -other.g2.x, 0.0) + vec3(self.g0.w) * other.g1 - self.g1 * vec3(other.g2.w) + vec3(self.g2.x) * vec3(0.0, -other.g0.z, other.g0.y) + vec3(self.g2.y) * vec3(other.g0.z, 0.0, -other.g0.x) + vec3(self.g2.z) * vec3(-other.g0.y, other.g0.x, 0.0), vec2(self.g0.x) * vec2(-other.g1.x, 0.0) + vec2(self.g0.y) * vec2(-other.g1.y, 0.0) + vec2(self.g0.z) * vec2(-other.g1.z, 0.0) - vec2(self.g1.x) * vec2(other.g0.x, other.g2.x) - vec2(self.g1.y) * vec2(other.g0.y, other.g2.y) - vec2(self.g1.z) * vec2(other.g0.z, other.g2.z) + vec2(self.g2.x) * vec2(0.0, -other.g1.x) + vec2(self.g2.y) * vec2(0.0, -other.g1.y) + vec2(self.g2.z) * vec2(0.0, -other.g1.z), vec3(0.0), vec3(0.0), vec4(0.0), vec4(self.g0.x) * vec4(other.g2.w, other.g1.z, -other.g1.y, -other.g2.x) + vec4(self.g0.y) * vec4(-other.g1.z, other.g2.w, other.g1.x, -other.g2.y) + vec4(self.g0.z) * vec4(other.g1.y, -other.g1.x, other.g2.w, -other.g2.z) + vec4(self.g0.w) * vec4(other.g0.x, other.g0.y, other.g0.z, 0.0) + vec4(self.g1.x) * vec4(0.0, other.g0.z, -other.g0.y, 0.0) + vec4(self.g1.y) * vec4(-other.g0.z, 0.0, other.g0.x, 0.0) + vec4(self.g1.z) * vec4(other.g0.y, -other.g0.x, 0.0, 0.0) + vec4(self.g2.x) * vec4(0.0, 0.0, 0.0, other.g0.x) + vec4(self.g2.y) * vec4(0.0, 0.0, 0.0, other.g0.y) + vec4(self.g2.z) * vec4(0.0, 0.0, 0.0, other.g0.z), vec3(self.g0.x) * vec3(0.0, other.g2.z, -other.g2.y) + vec3(self.g0.y) * vec3(-other.g2.z, 0.0, other.g2.x) + vec3(self.g0.z) * vec3(other.g2.y, -other.g2.x, 0.0) + vec3(self.g1.x) * vec3(0.0, other.g1.z, -other.g1.y) + vec3(self.g1.y) * vec3(-other.g1.z, 0.0, other.g1.x) + vec3(self.g1.z) * vec3(other.g1.y, -other.g1.x, 0.0) + vec3(self.g2.x) * vec3(0.0, other.g0.z, -other.g0.y) + vec3(self.g2.y) * vec3(-other.g0.z, 0.0, other.g0.x) + vec3(self.g2.z) * vec3(other.g0.y, -other.g0.x, 0.0), vec3(0.0) - vec3(self.g0.w) * vec3(other.g2.x, other.g2.y, other.g2.z) + vec3(self.g1.x) * vec3(0.0, other.g2.z, -other.g2.y) + vec3(self.g1.y) * vec3(-other.g2.z, 0.0, other.g2.x) + vec3(self.g1.z) * vec3(other.g2.y, -other.g2.x, 0.0) + vec3(self.g2.x) * vec3(-other.g2.w, other.g1.z, -other.g1.y) + vec3(self.g2.y) * vec3(-other.g1.z, -other.g2.w, other.g1.x) + vec3(self.g2.z) * vec3(other.g1.y, -other.g1.x, -other.g2.w), vec3(0.0), vec2(0.0));
}

MultiVector circle__wedge_dot__dual_num(Circle self, DualNum other) {
    return MultiVector(vec2(0.0), vec3(0.0), vec2(0.0), vec3(0.0) - vec3(self.g0.x, self.g0.y, self.g0.z) * vec3(other.g0.y), vec3(0.0) - self.g1 * vec3(other.g0.y), vec4(self.g0.w) * vec4(0.0, 0.0, 0.0, other.g0.y) + vec4(self.g2.x, self.g2.y, self.g2.z, self.g2.x) * vec4(-other.g0.y, -other.g0.y, -other.g0.y, 0.0), self.g0 * vec4(other.g0.x), self.g1 * vec3(other.g0.x), self.g2 * vec3(other.g0.x), vec3(0.0), vec2(0.0));
}

MultiVector circle__wedge_dot__flat_point(Circle self, FlatPoint other) {
    return MultiVector(vec2(self.g0.x) * vec2(0.0, -other.g0.x) + vec2(self.g0.y) * vec2(0.0, -other.g0.y) + vec2(self.g0.z) * vec2(0.0, -other.g0.z) + vec2(self.g0.w) * vec2(0.0, -other.g0.w), vec3(self.g0.x) * vec3(0.0, other.g0.z, -other.g0.y) + vec3(self.g0.y) * vec3(-other.g0.z, 0.0, other.g0.x) + vec3(self.g0.z) * vec3(other.g0.y, -other.g0.x, 0.0) - self.g1 * vec3(other.g0.w), vec2(self.g1.x) * vec2(0.0, -other.g0.x) + vec2(self.g1.y) * vec2(0.0, -other.g0.y) + vec2(self.g1.z) * vec2(0.0, -other.g0.z), vec3(0.0), vec3(0.0), vec4(0.0), vec4(self.g0.x) * vec4(other.g0.w, 0.0, 0.0, -other.g0.x) + vec4(self.g0.y) * vec4(0.0, other.g0.w, 0.0, -other.g0.y) + vec4(self.g0.z) * vec4(0.0, 0.0, other.g0.w, -other.g0.z), vec3(self.g0.x) * vec3(0.0, other.g0.z, -other.g0.y) + vec3(self.g0.y) * vec3(-other.g0.z, 0.0, other.g0.x) + vec3(self.g0.z) * vec3(other.g0.y, -other.g0.x, 0.0), vec3(0.0) - vec3(self.g0.w) * vec3(other.g0.x, other.g0.y, other.g0.z) + vec3(self.g1.x) * vec3(0.0, other.g0.z, -other.g0.y) + vec3(self.g1.y) * vec3(-other.g0.z, 0.0, other.g0.x) + vec3(self.g1.z) * vec3(other.g0.y, -other.g0.x, 0.0) - self.g2 * vec3(other.g0.w), vec3(0.0), vec2(0.0));
}

MultiVector circle__wedge_dot__flector(Circle self, Flector other) {
    return MultiVector(vec2(self.g0.x) * vec2(0.0, -other.g0.x) + vec2(self.g0.y) * vec2(0.0, -other.g0.y) + vec2(self.g0.z) * vec2(0.0, -other.g0.z) + vec2(self.g0.w) * vec2(0.0, -other.g0.w), vec3(self.g0.x) * vec3(other.g1.w, other.g0.z, -other.g0.y) + vec3(self.g0.y) * vec3(-other.g0.z, other.g1.w, other.g0.x) + vec3(self.g0.z) * vec3(other.g0.y, -other.g0.x, other.g1.w) + vec3(self.g1.x) * vec3(-other.g0.w, -other.g1.z, other.g1.y) + vec3(self.g1.y) * vec3(other.g1.z, -other.g0.w, -other.g1.x) + vec3(self.g1.z) * vec3(-other.g1.y, other.g1.x, -other.g0.w), vec2(self.g0.x) * vec2(-other.g1.x, 0.0) + vec2(self.g0.y) * vec2(-other.g1.y, 0.0) - vec2(self.g0.z, self.g0.w) * vec2(other.g1.z, other.g1.w) + vec2(self.g1.x) * vec2(0.0, -other.g0.x) + vec2(self.g1.y) * vec2(0.0, -other.g0.y) + vec2(self.g1.z) * vec2(0.0, -other.g0.z) + vec2(self.g2.x) * vec2(0.0, other.g1.x) + vec2(self.g2.y) * vec2(0.0, other.g1.y) + vec2(self.g2.z) * vec2(0.0, other.g1.z), vec3(0.0), vec3(0.0), vec4(0.0), vec4(self.g0.x) * vec4(other.g0.w, other.g1.z, -other.g1.y, -other.g0.x) + vec4(self.g0.y) * vec4(-other.g1.z, other.g0.w, other.g1.x, -other.g0.y) + vec4(self.g0.z) * vec4(other.g1.y, -other.g1.x, other.g0.w, -other.g0.z) + vec4(self.g1.x) * vec4(0.0, 0.0, 0.0, other.g1.x) + vec4(self.g1.y) * vec4(0.0, 0.0, 0.0, other.g1.y) + vec4(self.g1.z) * vec4(0.0, 0.0, 0.0, other.g1.z), vec3(self.g0.x) * vec3(other.g1.w, other.g0.z, -other.g0.y) + vec3(self.g0.y) * vec3(-other.g0.z, other.g1.w, other.g0.x) + vec3(self.g0.z) * vec3(other.g0.y, -other.g0.x, other.g1.w) - vec3(self.g0.w) * vec3(other.g1.x, other.g1.y, other.g1.z), vec3(0.0) - vec3(self.g0.w) * vec3(other.g0.x, other.g0.y, other.g0.z) + vec3(self.g1.x) * vec3(other.g1.w, other.g0.z, -other.g0.y) + vec3(self.g1.y) * vec3(-other.g0.z, other.g1.w, other.g0.x) + vec3(self.g1.z) * vec3(other.g0.y, -other.g0.x, other.g1.w) + vec3(self.g2.x) * vec3(-other.g0.w, -other.g1.z, other.g1.y) + vec3(self.g2.y) * vec3(other.g1.z, -other.g0.w, -other.g1.x) + vec3(self.g2.z) * vec3(-other.g1.y, other.g1.x, -other.g0.w), vec3(0.0), vec2(0.0));
}

MultiVector circle__wedge_dot__line(Circle self, Line other) {
    return MultiVector(vec2(self.g0.x) * vec2(other.g1.x, 0.0) + vec2(self.g0.y) * vec2(other.g1.y, 0.0) + vec2(self.g0.z) * vec2(other.g1.z, 0.0) + vec2(self.g1.x) * vec2(other.g0.x, 0.0) + vec2(self.g1.y) * vec2(other.g0.y, 0.0) + vec2(self.g1.z) * vec2(other.g0.z, 0.0), vec3(0.0), vec2(0.0), vec3(self.g0.x) * vec3(0.0, -other.g0.z, other.g0.y) + vec3(self.g0.y) * vec3(other.g0.z, 0.0, -other.g0.x) + vec3(self.g0.z) * vec3(-other.g0.y, other.g0.x, 0.0), vec3(self.g0.x) * vec3(0.0, -other.g1.z, other.g1.y) + vec3(self.g0.y) * vec3(other.g1.z, 0.0, -other.g1.x) + vec3(self.g0.z) * vec3(-other.g1.y, other.g1.x, 0.0) + vec3(self.g1.x) * vec3(0.0, -other.g0.z, other.g0.y) + vec3(self.g1.y) * vec3(other.g0.z, 0.0, -other.g0.x) + vec3(self.g1.z) * vec3(-other.g0.y, other.g0.x, 0.0), vec4(self.g0.x) * vec4(0.0, 0.0, 0.0, -other.g1.x) + vec4(self.g0.y) * vec4(0.0, 0.0, 0.0, -other.g1.y) + self.g0.wwwz * vec4(other.g1.x, other.g1.y, other.g1.z, -other.g1.z) + vec4(self.g1.x) * vec4(0.0, -other.g1.z, other.g1.y, 0.0) + vec4(self.g1.y) * vec4(other.g1.z, 0.0, -other.g1.x, 0.0) + vec4(self.g1.z) * vec4(-other.g1.y, other.g1.x, 0.0, 0.0) + vec4(self.g2.x) * vec4(0.0, -other.g0.z, other.g0.y, 0.0) + vec4(self.g2.y) * vec4(other.g0.z, 0.0, -other.g0.x, 0.0) + vec4(self.g2.z) * vec4(-other.g0.y, other.g0.x, 0.0, 0.0), vec4(0.0), vec3(0.0), vec3(0.0), vec3(self.g0.x) * vec3(0.0, other.g1.z, -other.g1.y) + vec3(self.g0.y) * vec3(-other.g1.z, 0.0, other.g1.x) + vec3(self.g0.z) * vec3(other.g1.y, -other.g1.x, 0.0) + vec3(self.g0.w) * other.g0, vec2(self.g0.x) * vec2(other.g0.x, 0.0) + vec2(self.g0.y) * vec2(other.g0.y, 0.0) + vec2(self.g0.z) * vec2(other.g0.z, 0.0) + vec2(self.g1.x) * vec2(0.0, other.g1.x) + vec2(self.g1.y) * vec2(0.0, other.g1.y) + vec2(self.g1.z) * vec2(0.0, other.g1.z) + vec2(self.g2.x) * vec2(0.0, other.g0.x) + vec2(self.g2.y) * vec2(0.0, other.g0.y) + vec2(self.g2.z) * vec2(0.0, other.g0.z));
}

MultiVector circle__wedge_dot__motor(Circle self, Motor other) {
    return MultiVector(vec2(self.g0.x) * vec2(other.g1.x, 0.0) + vec2(self.g0.y) * vec2(other.g1.y, 0.0) + vec2(self.g0.z) * vec2(other.g1.z, 0.0) + vec2(self.g1.x) * vec2(other.g0.x, 0.0) + vec2(self.g1.y) * vec2(other.g0.y, 0.0) + vec2(self.g1.z) * vec2(other.g0.z, 0.0), vec3(0.0), vec2(0.0), vec3(self.g0.x) * vec3(-other.g0.w, -other.g0.z, other.g0.y) + vec3(self.g0.y) * vec3(other.g0.z, -other.g0.w, -other.g0.x) + vec3(self.g0.z) * vec3(-other.g0.y, other.g0.x, -other.g0.w), vec3(self.g0.x) * vec3(0.0, -other.g1.z, other.g1.y) + vec3(self.g0.y) * vec3(other.g1.z, 0.0, -other.g1.x) + vec3(self.g0.z) * vec3(-other.g1.y, other.g1.x, 0.0) + vec3(self.g1.x) * vec3(-other.g0.w, -other.g0.z, other.g0.y) + vec3(self.g1.y) * vec3(other.g0.z, -other.g0.w, -other.g0.x) + vec3(self.g1.z) * vec3(-other.g0.y, other.g0.x, -other.g0.w), vec4(self.g0.x) * vec4(0.0, 0.0, 0.0, -other.g1.x) + vec4(self.g0.y) * vec4(0.0, 0.0, 0.0, -other.g1.y) + vec4(self.g0.z) * vec4(0.0, 0.0, 0.0, -other.g1.z) + vec4(self.g0.w) * vec4(other.g1.x, other.g1.y, other.g1.z, other.g0.w) + vec4(self.g1.x) * vec4(0.0, -other.g1.z, other.g1.y, 0.0) + vec4(self.g1.y) * vec4(other.g1.z, 0.0, -other.g1.x, 0.0) + vec4(self.g1.z) * vec4(-other.g1.y, other.g1.x, 0.0, 0.0) + vec4(self.g2.x) * vec4(-other.g0.w, -other.g0.z, other.g0.y, 0.0) + vec4(self.g2.y) * vec4(other.g0.z, -other.g0.w, -other.g0.x, 0.0) + vec4(self.g2.z) * vec4(-other.g0.y, other.g0.x, -other.g0.w, 0.0), vec4(0.0), vec3(0.0), vec3(0.0), vec3(self.g0.x) * vec3(0.0, other.g1.z, -other.g1.y) + vec3(self.g0.y) * vec3(-other.g1.z, 0.0, other.g1.x) + vec3(self.g0.z) * vec3(other.g1.y, -other.g1.x, 0.0) + vec3(self.g0.w) * vec3(other.g0.x, other.g0.y, other.g0.z), vec2(self.g0.x) * vec2(other.g0.x, 0.0) + vec2(self.g0.y) * vec2(other.g0.y, 0.0) + vec2(self.g0.z) * vec2(other.g0.z, 0.0) + vec2(self.g1.x) * vec2(0.0, other.g1.x) + vec2(self.g1.y) * vec2(0.0, other.g1.y) + vec2(self.g1.z) * vec2(0.0, other.g1.z) + vec2(self.g2.x) * vec2(0.0, other.g0.x) + vec2(self.g2.y) * vec2(0.0, other.g0.y) + vec2(self.g2.z) * vec2(0.0, other.g0.z));
}

MultiVector circle__wedge_dot__multi_vector(Circle self, MultiVector other) {
    return MultiVector(vec2(self.g0.x) * vec2(other.g8.x, -other.g5.x) + vec2(self.g0.y) * vec2(other.g8.y, -other.g5.y) + vec2(self.g0.z) * vec2(other.g8.z, -other.g5.z) - vec2(self.g0.w) * vec2(other.g6.w, other.g5.w) + vec2(self.g1.x) * vec2(other.g7.x, -other.g4.x) + vec2(self.g1.y) * vec2(other.g7.y, -other.g4.y) + vec2(self.g1.z) * vec2(other.g7.z, -other.g4.z) + vec2(self.g2.x) * vec2(other.g6.x, -other.g3.x) + vec2(self.g2.y) * vec2(other.g6.y, -other.g3.y) + vec2(self.g2.z) * vec2(other.g6.z, -other.g3.z), vec3(self.g0.x) * vec3(other.g10.y, other.g5.z, -other.g5.y) + vec3(self.g0.y) * vec3(-other.g5.z, other.g10.y, other.g5.x) + vec3(self.g0.z) * vec3(other.g5.y, -other.g5.x, other.g10.y) + vec3(self.g0.w) * other.g4 + vec3(self.g1.x) * vec3(-other.g5.w, -other.g9.z, other.g9.y) + vec3(self.g1.y) * vec3(other.g9.z, -other.g5.w, -other.g9.x) + vec3(self.g1.z) * vec3(-other.g9.y, other.g9.x, -other.g5.w) + vec3(self.g2.x) * vec3(-other.g10.x, -other.g3.z, other.g3.y) + vec3(self.g2.y) * vec3(other.g3.z, -other.g10.x, -other.g3.x) + vec3(self.g2.z) * vec3(-other.g3.y, other.g3.x, -other.g10.x), vec2(self.g0.x) * vec2(-other.g9.x, 0.0) + vec2(self.g0.x) * vec2(-other.g4.x, 0.0) + vec2(self.g0.y) * vec2(-other.g9.y, 0.0) + vec2(self.g0.y) * vec2(-other.g4.y, 0.0) + vec2(self.g0.z) * vec2(-other.g9.z, 0.0) + vec2(self.g0.z) * vec2(-other.g4.z, 0.0) + vec2(self.g0.w) * other.g10 * vec2(1.0, -1.0) - vec2(self.g1.x) * vec2(other.g3.x, other.g5.x) - vec2(self.g1.y) * vec2(other.g3.y, other.g5.y) - vec2(self.g1.z) * vec2(other.g3.z, other.g5.z) + vec2(self.g2.x) * vec2(0.0, other.g9.x) + vec2(self.g2.x) * vec2(0.0, -other.g4.x) + vec2(self.g2.y) * vec2(0.0, other.g9.y) + vec2(self.g2.y) * vec2(0.0, -other.g4.y) + vec2(self.g2.z) * vec2(0.0, other.g9.z) + vec2(self.g2.z) * vec2(0.0, -other.g4.z), vec3(self.g0.x) * vec3(other.g6.w, -other.g7.z, other.g7.y) + vec3(self.g0.x) * vec3(-other.g0.y, other.g1.z, -other.g1.y) + vec3(self.g0.y) * vec3(other.g7.z, other.g6.w, -other.g7.x) + vec3(self.g0.y) * vec3(-other.g1.z, -other.g0.y, other.g1.x) + vec3(self.g0.z) * vec3(-other.g7.y, other.g7.x, other.g6.w) + vec3(self.g0.z) * vec3(other.g1.y, -other.g1.x, -other.g0.y) - vec3(self.g0.w) * vec3(other.g6.x, other.g6.y, other.g6.z) + vec3(self.g1.x) * vec3(-other.g2.x, -other.g6.z, other.g6.y) + vec3(self.g1.y) * vec3(other.g6.z, -other.g2.x, -other.g6.x) + vec3(self.g1.z) * vec3(-other.g6.y, other.g6.x, -other.g2.x), vec3(self.g0.x) * vec3(-other.g2.y, -other.g8.z, other.g8.y) + vec3(self.g0.y) * vec3(other.g8.z, -other.g2.y, -other.g8.x) + vec3(self.g0.z) * vec3(-other.g8.y, other.g8.x, -other.g2.y) - vec3(self.g0.w) * other.g1 + vec3(self.g1.x) * vec3(-other.g0.y, -other.g7.z, other.g7.y) + vec3(self.g1.y) * vec3(other.g7.z, -other.g0.y, -other.g7.x) + vec3(self.g1.z) * vec3(-other.g7.y, other.g7.x, -other.g0.y) + vec3(self.g2.x) * vec3(-other.g2.x, -other.g6.z, other.g6.y) + vec3(self.g2.y) * vec3(other.g6.z, -other.g2.x, -other.g6.x) + vec3(self.g2.z) * vec3(-other.g6.y, other.g6.x, -other.g2.x), vec4(self.g0.x) * vec4(0.0, 0.0, 0.0, -other.g8.x) + vec4(self.g0.y) * vec4(0.0, 0.0, 0.0, -other.g8.y) + vec4(self.g0.z) * vec4(0.0, 0.0, 0.0, -other.g8.z) + vec4(self.g0.w) * vec4(other.g8.x, other.g8.y, other.g8.z, other.g0.y) + vec4(self.g1.x) * vec4(-other.g2.y, -other.g8.z, other.g8.y, -other.g1.x) + vec4(self.g1.y) * vec4(other.g8.z, -other.g2.y, -other.g8.x, -other.g1.y) + vec4(self.g1.z) * vec4(-other.g8.y, other.g8.x, -other.g2.y, -other.g1.z) + vec4(self.g2.x) * vec4(-other.g6.w, -other.g7.z, other.g7.y, other.g6.x) + vec4(self.g2.x) * vec4(-other.g0.y, -other.g1.z, other.g1.y, 0.0) + vec4(self.g2.y) * vec4(other.g7.z, -other.g6.w, -other.g7.x, other.g6.y) + vec4(self.g2.y) * vec4(other.g1.z, -other.g0.y, -other.g1.x, 0.0) + vec4(self.g2.z) * vec4(-other.g7.y, other.g7.x, -other.g6.w, other.g6.z) + vec4(self.g2.z) * vec4(-other.g1.y, other.g1.x, -other.g0.y, 0.0), vec4(self.g0.x) * vec4(other.g5.w, other.g9.z, -other.g9.y, -other.g5.x) + vec4(self.g0.x) * vec4(other.g0.x, other.g4.z, -other.g4.y, 0.0) + vec4(self.g0.y) * vec4(-other.g9.z, other.g5.w, other.g9.x, -other.g5.y) + vec4(self.g0.y) * vec4(-other.g4.z, other.g0.x, other.g4.x, 0.0) + vec4(self.g0.z) * vec4(other.g9.y, -other.g9.x, other.g5.w, -other.g5.z) + vec4(self.g0.z) * vec4(other.g4.y, -other.g4.x, other.g0.x, 0.0) + vec4(self.g0.w) * vec4(other.g3.x, other.g3.y, other.g3.z, other.g0.x) + vec4(self.g1.x) * vec4(other.g10.x, other.g3.z, -other.g3.y, other.g9.x) + vec4(self.g1.y) * vec4(-other.g3.z, other.g10.x, other.g3.x, other.g9.y) + vec4(self.g1.z) * vec4(other.g3.y, -other.g3.x, other.g10.x, other.g9.z) + vec4(self.g2.x) * vec4(0.0, 0.0, 0.0, other.g3.x) + vec4(self.g2.y) * vec4(0.0, 0.0, 0.0, other.g3.y) + vec4(self.g2.z) * vec4(0.0, 0.0, 0.0, other.g3.z), vec3(self.g0.x) * vec3(other.g10.y, other.g5.z, -other.g5.y) + vec3(self.g0.y) * vec3(-other.g5.z, other.g10.y, other.g5.x) + vec3(self.g0.z) * vec3(other.g5.y, -other.g5.x, other.g10.y) - vec3(self.g0.w) * other.g9 + vec3(self.g1.x) * vec3(other.g0.x, other.g4.z, -other.g4.y) + vec3(self.g1.y) * vec3(-other.g4.z, other.g0.x, other.g4.x) + vec3(self.g1.z) * vec3(other.g4.y, -other.g4.x, other.g0.x) + vec3(self.g2.x) * vec3(other.g10.x, other.g3.z, -other.g3.y) + vec3(self.g2.y) * vec3(-other.g3.z, other.g10.x, other.g3.x) + vec3(self.g2.z) * vec3(other.g3.y, -other.g3.x, other.g10.x), vec3(0.0) - vec3(self.g0.w) * vec3(other.g5.x, other.g5.y, other.g5.z) + vec3(self.g1.x) * vec3(other.g10.y, other.g5.z, -other.g5.y) + vec3(self.g1.y) * vec3(-other.g5.z, other.g10.y, other.g5.x) + vec3(self.g1.z) * vec3(other.g5.y, -other.g5.x, other.g10.y) + vec3(self.g2.x) * vec3(-other.g5.w, -other.g9.z, other.g9.y) + vec3(self.g2.x) * vec3(other.g0.x, other.g4.z, -other.g4.y) + vec3(self.g2.y) * vec3(other.g9.z, -other.g5.w, -other.g9.x) + vec3(self.g2.y) * vec3(-other.g4.z, other.g0.x, other.g4.x) + vec3(self.g2.z) * vec3(-other.g9.y, other.g9.x, -other.g5.w) + vec3(self.g2.z) * vec3(other.g4.y, -other.g4.x, other.g0.x), vec3(self.g0.x) * vec3(other.g2.y, other.g8.z, -other.g8.y) + vec3(self.g0.y) * vec3(-other.g8.z, other.g2.y, other.g8.x) + vec3(self.g0.z) * vec3(other.g8.y, -other.g8.x, other.g2.y) + vec3(self.g0.w) * other.g7 + vec3(self.g1.x) * vec3(other.g6.w, other.g1.z, -other.g1.y) + vec3(self.g1.y) * vec3(-other.g1.z, other.g6.w, other.g1.x) + vec3(self.g1.z) * vec3(other.g1.y, -other.g1.x, other.g6.w) + vec3(self.g2.x) * vec3(-other.g2.x, -other.g6.z, other.g6.y) + vec3(self.g2.y) * vec3(other.g6.z, -other.g2.x, -other.g6.x) + vec3(self.g2.z) * vec3(-other.g6.y, other.g6.x, -other.g2.x), vec2(self.g0.x) * vec2(other.g7.x, 0.0) + vec2(self.g0.x) * vec2(-other.g1.x, 0.0) + vec2(self.g0.y) * vec2(other.g7.y, 0.0) + vec2(self.g0.y) * vec2(-other.g1.y, 0.0) + vec2(self.g0.z) * vec2(other.g7.z, 0.0) + vec2(self.g0.z) * vec2(-other.g1.z, 0.0) + vec2(self.g0.w) * other.g2 * vec2(-1.0, 1.0) + vec2(self.g1.x) * vec2(other.g6.x, other.g8.x) + vec2(self.g1.y) * vec2(other.g6.y, other.g8.y) + vec2(self.g1.z) * vec2(other.g6.z, other.g8.z) + vec2(self.g2.x) * vec2(0.0, other.g7.x) + vec2(self.g2.x) * vec2(0.0, other.g1.x) + vec2(self.g2.y) * vec2(0.0, other.g7.y) + vec2(self.g2.y) * vec2(0.0, other.g1.y) + vec2(self.g2.z) * vec2(0.0, other.g7.z) + vec2(self.g2.z) * vec2(0.0, other.g1.z));
}

MultiVector circle__wedge_dot__plane(Circle self, Plane other) {
    return MultiVector(vec2(0.0), vec3(self.g0.x, self.g0.y, self.g0.z) * vec3(other.g0.w) + vec3(self.g1.x) * vec3(0.0, -other.g0.z, other.g0.y) + vec3(self.g1.y) * vec3(other.g0.z, 0.0, -other.g0.x) + vec3(self.g1.z) * vec3(-other.g0.y, other.g0.x, 0.0), vec2(self.g0.x) * vec2(-other.g0.x, 0.0) + vec2(self.g0.y) * vec2(-other.g0.y, 0.0) - vec2(self.g0.z, self.g0.w) * vec2(other.g0.z, other.g0.w) + vec2(self.g2.x) * vec2(0.0, other.g0.x) + vec2(self.g2.y) * vec2(0.0, other.g0.y) + vec2(self.g2.z) * vec2(0.0, other.g0.z), vec3(0.0), vec3(0.0), vec4(0.0), vec4(self.g0.x) * vec4(0.0, other.g0.z, -other.g0.y, 0.0) + vec4(self.g0.y) * vec4(-other.g0.z, 0.0, other.g0.x, 0.0) + vec4(self.g0.z) * vec4(other.g0.y, -other.g0.x, 0.0, 0.0) + vec4(self.g1.x) * vec4(0.0, 0.0, 0.0, other.g0.x) + vec4(self.g1.y) * vec4(0.0, 0.0, 0.0, other.g0.y) + vec4(self.g1.z) * vec4(0.0, 0.0, 0.0, other.g0.z), vec3(self.g0.x, self.g0.y, self.g0.z) * vec3(other.g0.w) - vec3(self.g0.w) * vec3(other.g0.x, other.g0.y, other.g0.z), self.g1 * vec3(other.g0.w) + vec3(self.g2.x) * vec3(0.0, -other.g0.z, other.g0.y) + vec3(self.g2.y) * vec3(other.g0.z, 0.0, -other.g0.x) + vec3(self.g2.z) * vec3(-other.g0.y, other.g0.x, 0.0), vec3(0.0), vec2(0.0));
}

MultiVector circle__wedge_dot__round_point(Circle self, RoundPoint other) {
    return MultiVector(vec2(0.0), vec3(0.0), vec2(0.0), vec3(self.g0.x) * vec3(0.0, other.g0.z, -other.g0.y) + vec3(self.g0.y) * vec3(-other.g0.z, 0.0, other.g0.x) + vec3(self.g0.z) * vec3(other.g0.y, -other.g0.x, 0.0) - self.g1 * vec3(other.g1.x), vec3(0.0) - vec3(self.g0.x, self.g0.y, self.g0.z) * vec3(other.g1.y) - vec3(self.g0.w) * other.g0 - self.g2 * vec3(other.g1.x), vec4(self.g1.x) * vec4(-other.g1.y, 0.0, 0.0, -other.g0.x) + vec4(self.g1.y) * vec4(0.0, -other.g1.y, 0.0, -other.g0.y) + vec4(self.g1.z) * vec4(0.0, 0.0, -other.g1.y, -other.g0.z) + vec4(self.g2.x) * vec4(0.0, -other.g0.z, other.g0.y, 0.0) + vec4(self.g2.y) * vec4(other.g0.z, 0.0, -other.g0.x, 0.0) + vec4(self.g2.z) * vec4(-other.g0.y, other.g0.x, 0.0, 0.0), vec4(0.0), vec3(0.0), vec3(0.0), vec3(self.g0.x, self.g0.y, self.g0.z) * vec3(other.g1.y) + vec3(self.g1.x) * vec3(0.0, other.g0.z, -other.g0.y) + vec3(self.g1.y) * vec3(-other.g0.z, 0.0, other.g0.x) + vec3(self.g1.z) * vec3(other.g0.y, -other.g0.x, 0.0) - self.g2 * vec3(other.g1.x), vec2(self.g0.x) * vec2(-other.g0.x, 0.0) + vec2(self.g0.y) * vec2(-other.g0.y, 0.0) + vec2(self.g0.z) * vec2(-other.g0.z, 0.0) + vec2(self.g0.w) * other.g1 * vec2(-1.0, 1.0) + vec2(self.g2.x) * vec2(0.0, other.g0.x) + vec2(self.g2.y) * vec2(0.0, other.g0.y) + vec2(self.g2.z) * vec2(0.0, other.g0.z));
}

Circle circle__wedge_dot__scalar(Circle self, Scalar other) {
    return Circle(self.g0 * vec4(other.g0), self.g1 * vec3(other.g0), self.g2 * vec3(other.g0));
}

MultiVector circle__wedge_dot__sphere(Circle self, Sphere other) {
    return MultiVector(vec2(0.0), vec3(self.g0.x, self.g0.y, self.g0.z) * vec3(other.g1.y) + vec3(self.g1.x) * vec3(0.0, -other.g0.z, other.g0.y) + vec3(self.g1.y) * vec3(other.g0.z, 0.0, -other.g0.x) + vec3(self.g1.z) * vec3(-other.g0.y, other.g0.x, 0.0) - self.g2 * vec3(other.g1.x), vec2(self.g0.x) * vec2(-other.g0.x, 0.0) + vec2(self.g0.y) * vec2(-other.g0.y, 0.0) + vec2(self.g0.z) * vec2(-other.g0.z, 0.0) + vec2(self.g0.w) * other.g1 * vec2(1.0, -1.0) + vec2(self.g2.x) * vec2(0.0, other.g0.x) + vec2(self.g2.y) * vec2(0.0, other.g0.y) + vec2(self.g2.z) * vec2(0.0, other.g0.z), vec3(0.0), vec3(0.0), vec4(0.0), vec4(self.g0.x) * vec4(0.0, other.g0.z, -other.g0.y, 0.0) + vec4(self.g0.y) * vec4(-other.g0.z, 0.0, other.g0.x, 0.0) + vec4(self.g0.z) * vec4(other.g0.y, -other.g0.x, 0.0, 0.0) + vec4(self.g1.x) * vec4(other.g1.x, 0.0, 0.0, other.g0.x) + vec4(self.g1.y) * vec4(0.0, other.g1.x, 0.0, other.g0.y) + vec4(self.g1.z) * vec4(0.0, 0.0, other.g1.x, other.g0.z), vec3(self.g0.x, self.g0.y, self.g0.z) * vec3(other.g1.y) - vec3(self.g0.w) * other.g0 + self.g2 * vec3(other.g1.x), self.g1 * vec3(other.g1.y) + vec3(self.g2.x) * vec3(0.0, -other.g0.z, other.g0.y) + vec3(self.g2.y) * vec3(other.g0.z, 0.0, -other.g0.x) + vec3(self.g2.z) * vec3(-other.g0.y, other.g0.x, 0.0), vec3(0.0), vec2(0.0));
}

Circle dipole__wedge_dot__anti_scalar(Dipole self, AntiScalar other) {
    return Circle(vec4(self.g0.x, self.g0.y, self.g0.z, self.g0.x) * vec4(other.g0, other.g0, other.g0, 0.0) + vec4(self.g2.w) * vec4(0.0, 0.0, 0.0, -other.g0), self.g1 * vec3(other.g0), vec3(self.g2.x, self.g2.y, self.g2.z) * vec3(other.g0));
}

MultiVector dipole__wedge_dot__circle(Dipole self, Circle other) {
    return MultiVector(vec2(self.g0.x) * vec2(0.0, -other.g2.x) + vec2(self.g0.y) * vec2(0.0, -other.g2.y) + vec2(self.g0.z) * vec2(0.0, -other.g2.z) + vec2(self.g1.x) * vec2(0.0, -other.g1.x) + vec2(self.g1.y) * vec2(0.0, -other.g1.y) + vec2(self.g1.z) * vec2(0.0, -other.g1.z) + vec2(self.g2.x) * vec2(0.0, -other.g0.x) + vec2(self.g2.y) * vec2(0.0, -other.g0.y) + vec2(self.g2.z) * vec2(0.0, -other.g0.z) + vec2(self.g2.w) * vec2(0.0, -other.g0.w), vec3(self.g0.x) * vec3(0.0, other.g2.z, -other.g2.y) + vec3(self.g0.y) * vec3(-other.g2.z, 0.0, other.g2.x) + vec3(self.g0.z) * vec3(other.g2.y, -other.g2.x, 0.0) + self.g1 * vec3(other.g0.w) + vec3(self.g2.x) * vec3(0.0, -other.g0.z, other.g0.y) + vec3(self.g2.y) * vec3(other.g0.z, 0.0, -other.g0.x) + vec3(self.g2.z) * vec3(-other.g0.y, other.g0.x, 0.0) - vec3(self.g2.w) * other.g1, vec2(self.g0.x) * vec2(-other.g1.x, 0.0) + vec2(self.g0.y) * vec2(-other.g1.y, 0.0) + vec2(self.g0.z) * vec2(-other.g1.z, 0.0) - vec2(self.g1.x) * vec2(other.g0.x, other.g2.x) - vec2(self.g1.y) * vec2(other.g0.y, other.g2.y) - vec2(self.g1.z) * vec2(other.g0.z, other.g2.z) + vec2(self.g2.x) * vec2(0.0, -other.g1.x) + vec2(self.g2.y) * vec2(0.0, -other.g1.y) + vec2(self.g2.z) * vec2(0.0, -other.g1.z), vec3(0.0), vec3(0.0), vec4(0.0), vec4(self.g0.x) * vec4(-other.g0.w, other.g1.z, -other.g1.y, -other.g2.x) + vec4(self.g0.y) * vec4(-other.g1.z, -other.g0.w, other.g1.x, -other.g2.y) + vec4(self.g0.z) * vec4(other.g1.y, -other.g1.x, -other.g0.w, -other.g2.z) + vec4(self.g1.x) * vec4(0.0, other.g0.z, -other.g0.y, 0.0) + vec4(self.g1.y) * vec4(-other.g0.z, 0.0, other.g0.x, 0.0) + vec4(self.g1.z) * vec4(other.g0.y, -other.g0.x, 0.0, 0.0) + vec4(self.g2.x) * vec4(0.0, 0.0, 0.0, other.g0.x) + vec4(self.g2.y) * vec4(0.0, 0.0, 0.0, other.g0.y) + self.g2.wwwz * other.g0.xyzz * vec4(-1.0, -1.0, -1.0, 1.0), vec3(self.g0.x) * vec3(0.0, other.g2.z, -other.g2.y) + vec3(self.g0.y) * vec3(-other.g2.z, 0.0, other.g2.x) + vec3(self.g0.z) * vec3(other.g2.y, -other.g2.x, 0.0) + vec3(self.g1.x) * vec3(0.0, other.g1.z, -other.g1.y) + vec3(self.g1.y) * vec3(-other.g1.z, 0.0, other.g1.x) + vec3(self.g1.z) * vec3(other.g1.y, -other.g1.x, 0.0) + vec3(self.g2.x) * vec3(0.0, other.g0.z, -other.g0.y) + vec3(self.g2.y) * vec3(-other.g0.z, 0.0, other.g0.x) + vec3(self.g2.z) * vec3(other.g0.y, -other.g0.x, 0.0), vec3(self.g1.x) * vec3(0.0, other.g2.z, -other.g2.y) + vec3(self.g1.y) * vec3(-other.g2.z, 0.0, other.g2.x) + vec3(self.g1.z) * vec3(other.g2.y, -other.g2.x, 0.0) + vec3(self.g2.x) * vec3(other.g0.w, other.g1.z, -other.g1.y) + vec3(self.g2.y) * vec3(-other.g1.z, other.g0.w, other.g1.x) + vec3(self.g2.z) * vec3(other.g1.y, -other.g1.x, other.g0.w) + vec3(self.g2.w) * other.g2, vec3(0.0), vec2(0.0));
}

MultiVector dipole__wedge_dot__dipole(Dipole self, Dipole other) {
    return MultiVector(vec2(self.g0.x) * vec2(-other.g2.x, 0.0) + vec2(self.g0.y) * vec2(-other.g2.y, 0.0) + vec2(self.g0.z) * vec2(-other.g2.z, 0.0) + vec2(self.g1.x) * vec2(-other.g1.x, 0.0) + vec2(self.g1.y) * vec2(-other.g1.y, 0.0) + vec2(self.g1.z) * vec2(-other.g1.z, 0.0) + vec2(self.g2.x) * vec2(-other.g0.x, 0.0) + vec2(self.g2.y) * vec2(-other.g0.y, 0.0) + vec2(self.g2.z) * vec2(-other.g0.z, 0.0) + vec2(self.g2.w) * vec2(other.g2.w, 0.0), vec3(0.0), vec2(0.0), vec3(self.g0.x) * vec3(other.g2.w, other.g1.z, -other.g1.y) + vec3(self.g0.y) * vec3(-other.g1.z, other.g2.w, other.g1.x) + vec3(self.g0.z) * vec3(other.g1.y, -other.g1.x, other.g2.w) + vec3(self.g1.x) * vec3(0.0, other.g0.z, -other.g0.y) + vec3(self.g1.y) * vec3(-other.g0.z, 0.0, other.g0.x) + vec3(self.g1.z) * vec3(other.g0.y, -other.g0.x, 0.0) - vec3(self.g2.w) * other.g0, vec3(self.g0.x) * vec3(0.0, other.g2.z, -other.g2.y) + vec3(self.g0.y) * vec3(-other.g2.z, 0.0, other.g2.x) + vec3(self.g0.z) * vec3(other.g2.y, -other.g2.x, 0.0) + vec3(self.g1.x) * vec3(0.0, other.g1.z, -other.g1.y) + vec3(self.g1.y) * vec3(-other.g1.z, 0.0, other.g1.x) + vec3(self.g1.z) * vec3(other.g1.y, -other.g1.x, 0.0) + vec3(self.g2.x) * vec3(0.0, other.g0.z, -other.g0.y) + vec3(self.g2.y) * vec3(-other.g0.z, 0.0, other.g0.x) + vec3(self.g2.z) * vec3(other.g0.y, -other.g0.x, 0.0), vec4(self.g0.x) * vec4(0.0, 0.0, 0.0, other.g2.x) + vec4(self.g0.y) * vec4(0.0, 0.0, 0.0, other.g2.y) + vec4(self.g0.z) * vec4(0.0, 0.0, 0.0, other.g2.z) + vec4(self.g1.x) * vec4(0.0, other.g2.z, -other.g2.y, 0.0) + vec4(self.g1.y) * vec4(-other.g2.z, 0.0, other.g2.x, 0.0) + vec4(self.g1.z) * vec4(other.g2.y, -other.g2.x, 0.0, 0.0) + vec4(self.g2.x) * vec4(-other.g2.w, other.g1.z, -other.g1.y, -other.g0.x) + vec4(self.g2.y) * vec4(-other.g1.z, -other.g2.w, other.g1.x, -other.g0.y) + vec4(self.g2.z) * vec4(other.g1.y, -other.g1.x, -other.g2.w, -other.g0.z) + vec4(self.g2.w) * vec4(other.g2.x, other.g2.y, other.g2.z, 0.0), vec4(0.0), vec3(0.0), vec3(0.0), vec3(self.g0.x) * vec3(0.0, -other.g2.z, other.g2.y) + vec3(self.g0.y) * vec3(other.g2.z, 0.0, -other.g2.x) + vec3(self.g0.z) * vec3(-other.g2.y, other.g2.x, 0.0) + self.g1 * vec3(other.g2.w) + vec3(self.g2.x) * vec3(0.0, other.g0.z, -other.g0.y) + vec3(self.g2.y) * vec3(-other.g0.z, 0.0, other.g0.x) + vec3(self.g2.z) * vec3(other.g0.y, -other.g0.x, 0.0) + vec3(self.g2.w) * other.g1, vec2(self.g0.x) * vec2(-other.g1.x, 0.0) + vec2(self.g0.y) * vec2(-other.g1.y, 0.0) + vec2(self.g0.z) * vec2(-other.g1.z, 0.0) - vec2(self.g1.x) * vec2(other.g0.x, other.g2.x) - vec2(self.g1.y) * vec2(other.g0.y, other.g2.y) - vec2(self.g1.z) * vec2(other.g0.z, other.g2.z) + vec2(self.g2.x) * vec2(0.0, -other.g1.x) + vec2(self.g2.y) * vec2(0.0, -other.g1.y) + vec2(self.g2.z) * vec2(0.0, -other.g1.z));
}

MultiVector dipole__wedge_dot__dual_num(Dipole self, DualNum other) {
    return MultiVector(vec2(0.0), vec3(0.0), vec2(0.0), self.g0 * vec3(other.g0.x), self.g1 * vec3(other.g0.x), self.g2 * vec4(other.g0.x), vec4(self.g0.x, self.g0.y, self.g0.z, self.g0.x) * vec4(other.g0.y, other.g0.y, other.g0.y, 0.0) + vec4(self.g2.w) * vec4(0.0, 0.0, 0.0, -other.g0.y), self.g1 * vec3(other.g0.y), vec3(self.g2.x, self.g2.y, self.g2.z) * vec3(other.g0.y), vec3(0.0), vec2(0.0));
}

MultiVector dipole__wedge_dot__flat_point(Dipole self, FlatPoint other) {
    return MultiVector(vec2(self.g0.x) * vec2(-other.g0.x, 0.0) + vec2(self.g0.y) * vec2(-other.g0.y, 0.0) + vec2(self.g0.z) * vec2(-other.g0.z, 0.0) + vec2(self.g2.w) * vec2(other.g0.w, 0.0), vec3(0.0), vec2(0.0), self.g0 * vec3(other.g0.w), vec3(self.g0.x) * vec3(0.0, other.g0.z, -other.g0.y) + vec3(self.g0.y) * vec3(-other.g0.z, 0.0, other.g0.x) + vec3(self.g0.z) * vec3(other.g0.y, -other.g0.x, 0.0), vec4(self.g0.x) * vec4(0.0, 0.0, 0.0, other.g0.x) + vec4(self.g0.y) * vec4(0.0, 0.0, 0.0, other.g0.y) + vec4(self.g0.z) * vec4(0.0, 0.0, 0.0, other.g0.z) + vec4(self.g1.x) * vec4(0.0, other.g0.z, -other.g0.y, 0.0) + vec4(self.g1.y) * vec4(-other.g0.z, 0.0, other.g0.x, 0.0) + vec4(self.g1.z) * vec4(other.g0.y, -other.g0.x, 0.0, 0.0) + self.g2.xyzx * vec4(-other.g0.w, -other.g0.w, -other.g0.w, 0.0) + vec4(self.g2.w) * vec4(other.g0.x, other.g0.y, other.g0.z, 0.0), vec4(0.0), vec3(0.0), vec3(0.0), vec3(self.g0.x) * vec3(0.0, -other.g0.z, other.g0.y) + vec3(self.g0.y) * vec3(other.g0.z, 0.0, -other.g0.x) + vec3(self.g0.z) * vec3(-other.g0.y, other.g0.x, 0.0) + self.g1 * vec3(other.g0.w), vec2(self.g1.x) * vec2(0.0, -other.g0.x) + vec2(self.g1.y) * vec2(0.0, -other.g0.y) + vec2(self.g1.z) * vec2(0.0, -other.g0.z));
}

MultiVector dipole__wedge_dot__flector(Dipole self, Flector other) {
    return MultiVector(vec2(self.g0.x) * vec2(-other.g0.x, 0.0) + vec2(self.g0.y) * vec2(-other.g0.y, 0.0) + vec2(self.g0.z) * vec2(-other.g0.z, 0.0) + vec2(self.g2.w) * vec2(other.g0.w, 0.0), vec3(0.0), vec2(0.0), vec3(self.g0.x) * vec3(other.g0.w, other.g1.z, -other.g1.y) + vec3(self.g0.y) * vec3(-other.g1.z, other.g0.w, other.g1.x) + vec3(self.g0.z) * vec3(other.g1.y, -other.g1.x, other.g0.w), vec3(self.g0.x) * vec3(other.g1.w, other.g0.z, -other.g0.y) + vec3(self.g0.y) * vec3(-other.g0.z, other.g1.w, other.g0.x) + vec3(self.g0.z) * vec3(other.g0.y, -other.g0.x, other.g1.w) + vec3(self.g2.w) * vec3(other.g1.x, other.g1.y, other.g1.z), vec4(self.g0.x) * vec4(0.0, 0.0, 0.0, other.g0.x) + vec4(self.g0.y) * vec4(0.0, 0.0, 0.0, other.g0.y) + vec4(self.g0.z) * vec4(0.0, 0.0, 0.0, other.g0.z) + vec4(self.g1.x) * vec4(other.g1.w, other.g0.z, -other.g0.y, -other.g1.x) + vec4(self.g1.y) * vec4(-other.g0.z, other.g1.w, other.g0.x, -other.g1.y) + vec4(self.g1.z) * vec4(other.g0.y, -other.g0.x, other.g1.w, -other.g1.z) + vec4(self.g2.x) * vec4(-other.g0.w, -other.g1.z, other.g1.y, 0.0) + vec4(self.g2.y) * vec4(other.g1.z, -other.g0.w, -other.g1.x, 0.0) + vec4(self.g2.z) * vec4(-other.g1.y, other.g1.x, -other.g0.w, 0.0) + vec4(self.g2.w) * vec4(other.g0.x, other.g0.y, other.g0.z, 0.0), vec4(0.0), vec3(0.0), vec3(0.0), vec3(self.g0.x) * vec3(-other.g1.w, -other.g0.z, other.g0.y) + vec3(self.g0.y) * vec3(other.g0.z, -other.g1.w, -other.g0.x) + vec3(self.g0.z) * vec3(-other.g0.y, other.g0.x, -other.g1.w) + vec3(self.g1.x) * vec3(other.g0.w, other.g1.z, -other.g1.y) + vec3(self.g1.y) * vec3(-other.g1.z, other.g0.w, other.g1.x) + vec3(self.g1.z) * vec3(other.g1.y, -other.g1.x, other.g0.w), vec2(self.g0.x) * vec2(-other.g1.x, 0.0) + vec2(self.g0.y) * vec2(-other.g1.y, 0.0) + vec2(self.g0.z) * vec2(-other.g1.z, 0.0) + vec2(self.g1.x) * vec2(0.0, -other.g0.x) + vec2(self.g1.y) * vec2(0.0, -other.g0.y) + vec2(self.g1.z) * vec2(0.0, -other.g0.z) + vec2(self.g2.x) * vec2(0.0, other.g1.x) + vec2(self.g2.y) * vec2(0.0, other.g1.y) + vec2(self.g2.z) * vec2(0.0, other.g1.z) + vec2(self.g2.w) * vec2(0.0, other.g1.w));
}

MultiVector dipole__wedge_dot__line(Dipole self, Line other) {
    return MultiVector(vec2(self.g0.x) * vec2(0.0, -other.g1.x) + vec2(self.g0.y) * vec2(0.0, -other.g1.y) + vec2(self.g0.z) * vec2(0.0, -other.g1.z) + vec2(self.g1.x) * vec2(0.0, -other.g0.x) + vec2(self.g1.y) * vec2(0.0, -other.g0.y) + vec2(self.g1.z) * vec2(0.0, -other.g0.z), vec3(self.g0.x) * vec3(0.0, other.g1.z, -other.g1.y) + vec3(self.g0.y) * vec3(-other.g1.z, 0.0, other.g1.x) + vec3(self.g0.z) * vec3(other.g1.y, -other.g1.x, 0.0) - vec3(self.g2.w) * other.g0, vec2(self.g0.x) * vec2(-other.g0.x, 0.0) + vec2(self.g0.y) * vec2(-other.g0.y, 0.0) + vec2(self.g0.z) * vec2(-other.g0.z, 0.0) + vec2(self.g1.x) * vec2(0.0, -other.g1.x) + vec2(self.g1.y) * vec2(0.0, -other.g1.y) + vec2(self.g1.z) * vec2(0.0, -other.g1.z) + vec2(self.g2.x) * vec2(0.0, -other.g0.x) + vec2(self.g2.y) * vec2(0.0, -other.g0.y) + vec2(self.g2.z) * vec2(0.0, -other.g0.z), vec3(0.0), vec3(0.0), vec4(0.0), vec4(self.g0.x) * vec4(0.0, other.g0.z, -other.g0.y, -other.g1.x) + vec4(self.g0.y) * vec4(-other.g0.z, 0.0, other.g0.x, -other.g1.y) + vec4(self.g0.z) * vec4(other.g0.y, -other.g0.x, 0.0, -other.g1.z), vec3(self.g0.x) * vec3(0.0, other.g1.z, -other.g1.y) + vec3(self.g0.y) * vec3(-other.g1.z, 0.0, other.g1.x) + vec3(self.g0.z) * vec3(other.g1.y, -other.g1.x, 0.0) + vec3(self.g1.x) * vec3(0.0, other.g0.z, -other.g0.y) + vec3(self.g1.y) * vec3(-other.g0.z, 0.0, other.g0.x) + vec3(self.g1.z) * vec3(other.g0.y, -other.g0.x, 0.0), vec3(self.g1.x) * vec3(0.0, other.g1.z, -other.g1.y) + vec3(self.g1.y) * vec3(-other.g1.z, 0.0, other.g1.x) + vec3(self.g1.z) * vec3(other.g1.y, -other.g1.x, 0.0) + vec3(self.g2.x) * vec3(0.0, other.g0.z, -other.g0.y) + vec3(self.g2.y) * vec3(-other.g0.z, 0.0, other.g0.x) + vec3(self.g2.z) * vec3(other.g0.y, -other.g0.x, 0.0) + vec3(self.g2.w) * other.g1, vec3(0.0), vec2(0.0));
}

MultiVector dipole__wedge_dot__motor(Dipole self, Motor other) {
    return MultiVector(vec2(self.g0.x) * vec2(0.0, -other.g1.x) + vec2(self.g0.y) * vec2(0.0, -other.g1.y) + vec2(self.g0.z) * vec2(0.0, -other.g1.z) + vec2(self.g1.x) * vec2(0.0, -other.g0.x) + vec2(self.g1.y) * vec2(0.0, -other.g0.y) + vec2(self.g1.z) * vec2(0.0, -other.g0.z), vec3(self.g0.x) * vec3(0.0, other.g1.z, -other.g1.y) + vec3(self.g0.y) * vec3(-other.g1.z, 0.0, other.g1.x) + vec3(self.g0.z) * vec3(other.g1.y, -other.g1.x, 0.0) - vec3(self.g2.w) * vec3(other.g0.x, other.g0.y, other.g0.z), vec2(self.g0.x) * vec2(-other.g0.x, 0.0) + vec2(self.g0.y) * vec2(-other.g0.y, 0.0) + vec2(self.g0.z) * vec2(-other.g0.z, 0.0) + vec2(self.g1.x) * vec2(0.0, -other.g1.x) + vec2(self.g1.y) * vec2(0.0, -other.g1.y) + vec2(self.g1.z) * vec2(0.0, -other.g1.z) + vec2(self.g2.x) * vec2(0.0, -other.g0.x) + vec2(self.g2.y) * vec2(0.0, -other.g0.y) + vec2(self.g2.z) * vec2(0.0, -other.g0.z), vec3(0.0), vec3(0.0), vec4(0.0), vec4(self.g0.x) * vec4(other.g0.w, other.g0.z, -other.g0.y, -other.g1.x) + vec4(self.g0.y) * vec4(-other.g0.z, other.g0.w, other.g0.x, -other.g1.y) + vec4(self.g0.z) * vec4(other.g0.y, -other.g0.x, other.g0.w, -other.g1.z) + vec4(self.g2.w) * vec4(0.0, 0.0, 0.0, -other.g0.w), vec3(self.g0.x) * vec3(0.0, other.g1.z, -other.g1.y) + vec3(self.g0.y) * vec3(-other.g1.z, 0.0, other.g1.x) + vec3(self.g0.z) * vec3(other.g1.y, -other.g1.x, 0.0) + vec3(self.g1.x) * vec3(other.g0.w, other.g0.z, -other.g0.y) + vec3(self.g1.y) * vec3(-other.g0.z, other.g0.w, other.g0.x) + vec3(self.g1.z) * vec3(other.g0.y, -other.g0.x, other.g0.w), vec3(self.g1.x) * vec3(0.0, other.g1.z, -other.g1.y) + vec3(self.g1.y) * vec3(-other.g1.z, 0.0, other.g1.x) + vec3(self.g1.z) * vec3(other.g1.y, -other.g1.x, 0.0) + vec3(self.g2.x) * vec3(other.g0.w, other.g0.z, -other.g0.y) + vec3(self.g2.y) * vec3(-other.g0.z, other.g0.w, other.g0.x) + vec3(self.g2.z) * vec3(other.g0.y, -other.g0.x, other.g0.w) + vec3(self.g2.w) * other.g1, vec3(0.0), vec2(0.0));
}

MultiVector dipole__wedge_dot__multi_vector(Dipole self, MultiVector other) {
    return MultiVector(vec2(0.0) - vec2(self.g0.x) * vec2(other.g5.x, other.g8.x) - vec2(self.g0.y) * vec2(other.g5.y, other.g8.y) - vec2(self.g0.z) * vec2(other.g5.z, other.g8.z) - vec2(self.g1.x) * vec2(other.g4.x, other.g7.x) - vec2(self.g1.y) * vec2(other.g4.y, other.g7.y) - vec2(self.g1.z) * vec2(other.g4.z, other.g7.z) - vec2(self.g2.x) * vec2(other.g3.x, other.g6.x) - vec2(self.g2.y) * vec2(other.g3.y, other.g6.y) - vec2(self.g2.z) * vec2(other.g3.z, other.g6.z) + vec2(self.g2.w) * vec2(other.g5.w, -other.g6.w), vec3(self.g0.x) * vec3(other.g2.y, other.g8.z, -other.g8.y) + vec3(self.g0.y) * vec3(-other.g8.z, other.g2.y, other.g8.x) + vec3(self.g0.z) * vec3(other.g8.y, -other.g8.x, other.g2.y) + vec3(self.g1.x) * vec3(other.g6.w, other.g1.z, -other.g1.y) + vec3(self.g1.y) * vec3(-other.g1.z, other.g6.w, other.g1.x) + vec3(self.g1.z) * vec3(other.g1.y, -other.g1.x, other.g6.w) + vec3(self.g2.x) * vec3(-other.g2.x, -other.g6.z, other.g6.y) + vec3(self.g2.y) * vec3(other.g6.z, -other.g2.x, -other.g6.x) + vec3(self.g2.z) * vec3(-other.g6.y, other.g6.x, -other.g2.x) - vec3(self.g2.w) * other.g7, vec2(self.g0.x) * vec2(-other.g7.x, 0.0) + vec2(self.g0.x) * vec2(other.g1.x, 0.0) + vec2(self.g0.y) * vec2(-other.g7.y, 0.0) + vec2(self.g0.y) * vec2(other.g1.y, 0.0) + vec2(self.g0.z) * vec2(-other.g7.z, 0.0) + vec2(self.g0.z) * vec2(other.g1.z, 0.0) - vec2(self.g1.x) * vec2(other.g6.x, other.g8.x) - vec2(self.g1.y) * vec2(other.g6.y, other.g8.y) - vec2(self.g1.z) * vec2(other.g6.z, other.g8.z) + vec2(self.g2.x) * vec2(0.0, -other.g7.x) + vec2(self.g2.x) * vec2(0.0, -other.g1.x) + vec2(self.g2.y) * vec2(0.0, -other.g7.y) + vec2(self.g2.y) * vec2(0.0, -other.g1.y) + vec2(self.g2.z) * vec2(0.0, -other.g7.z) + vec2(self.g2.z) * vec2(0.0, -other.g1.z) + vec2(self.g2.w) * other.g2 * vec2(-1.0, 1.0), vec3(self.g0.x) * vec3(other.g5.w, other.g9.z, -other.g9.y) + vec3(self.g0.x) * vec3(other.g0.x, other.g4.z, -other.g4.y) + vec3(self.g0.y) * vec3(-other.g9.z, other.g5.w, other.g9.x) + vec3(self.g0.y) * vec3(-other.g4.z, other.g0.x, other.g4.x) + vec3(self.g0.z) * vec3(other.g9.y, -other.g9.x, other.g5.w) + vec3(self.g0.z) * vec3(other.g4.y, -other.g4.x, other.g0.x) + vec3(self.g1.x) * vec3(other.g10.x, other.g3.z, -other.g3.y) + vec3(self.g1.y) * vec3(-other.g3.z, other.g10.x, other.g3.x) + vec3(self.g1.z) * vec3(other.g3.y, -other.g3.x, other.g10.x) - vec3(self.g2.w) * other.g3, vec3(self.g0.x) * vec3(other.g10.y, other.g5.z, -other.g5.y) + vec3(self.g0.y) * vec3(-other.g5.z, other.g10.y, other.g5.x) + vec3(self.g0.z) * vec3(other.g5.y, -other.g5.x, other.g10.y) + vec3(self.g1.x) * vec3(other.g0.x, other.g4.z, -other.g4.y) + vec3(self.g1.y) * vec3(-other.g4.z, other.g0.x, other.g4.x) + vec3(self.g1.z) * vec3(other.g4.y, -other.g4.x, other.g0.x) + vec3(self.g2.x) * vec3(other.g10.x, other.g3.z, -other.g3.y) + vec3(self.g2.y) * vec3(-other.g3.z, other.g10.x, other.g3.x) + vec3(self.g2.z) * vec3(other.g3.y, -other.g3.x, other.g10.x) + vec3(self.g2.w) * other.g9, vec4(self.g0.x) * vec4(0.0, 0.0, 0.0, other.g5.x) + vec4(self.g0.y) * vec4(0.0, 0.0, 0.0, other.g5.y) + vec4(self.g0.z) * vec4(0.0, 0.0, 0.0, other.g5.z) + vec4(self.g1.x) * vec4(other.g10.y, other.g5.z, -other.g5.y, -other.g9.x) + vec4(self.g1.y) * vec4(-other.g5.z, other.g10.y, other.g5.x, -other.g9.y) + vec4(self.g1.z) * vec4(other.g5.y, -other.g5.x, other.g10.y, -other.g9.z) + vec4(self.g2.x) * vec4(-other.g5.w, -other.g9.z, other.g9.y, -other.g3.x) + vec4(self.g2.x) * vec4(other.g0.x, other.g4.z, -other.g4.y, 0.0) + vec4(self.g2.y) * vec4(other.g9.z, -other.g5.w, -other.g9.x, -other.g3.y) + vec4(self.g2.y) * vec4(-other.g4.z, other.g0.x, other.g4.x, 0.0) + vec4(self.g2.z) * vec4(-other.g9.y, other.g9.x, -other.g5.w, -other.g3.z) + vec4(self.g2.z) * vec4(other.g4.y, -other.g4.x, other.g0.x, 0.0) + vec4(self.g2.w) * vec4(other.g5.x, other.g5.y, other.g5.z, other.g0.x), vec4(self.g0.x) * vec4(-other.g6.w, other.g7.z, -other.g7.y, -other.g8.x) + vec4(self.g0.x) * vec4(other.g0.y, -other.g1.z, other.g1.y, 0.0) + vec4(self.g0.y) * vec4(-other.g7.z, -other.g6.w, other.g7.x, -other.g8.y) + vec4(self.g0.y) * vec4(other.g1.z, other.g0.y, -other.g1.x, 0.0) + vec4(self.g0.z) * vec4(other.g7.y, -other.g7.x, -other.g6.w, -other.g8.z) + vec4(self.g0.z) * vec4(-other.g1.y, other.g1.x, other.g0.y, 0.0) + vec4(self.g1.x) * vec4(other.g2.x, other.g6.z, -other.g6.y, -other.g1.x) + vec4(self.g1.y) * vec4(-other.g6.z, other.g2.x, other.g6.x, -other.g1.y) + vec4(self.g1.z) * vec4(other.g6.y, -other.g6.x, other.g2.x, -other.g1.z) + vec4(self.g2.x) * vec4(0.0, 0.0, 0.0, other.g6.x) + vec4(self.g2.y) * vec4(0.0, 0.0, 0.0, other.g6.y) + vec4(self.g2.z) * vec4(0.0, 0.0, 0.0, other.g6.z) - vec4(self.g2.w) * vec4(other.g6.x, other.g6.y, other.g6.z, other.g0.y), vec3(self.g0.x) * vec3(other.g2.y, other.g8.z, -other.g8.y) + vec3(self.g0.y) * vec3(-other.g8.z, other.g2.y, other.g8.x) + vec3(self.g0.z) * vec3(other.g8.y, -other.g8.x, other.g2.y) + vec3(self.g1.x) * vec3(other.g0.y, other.g7.z, -other.g7.y) + vec3(self.g1.y) * vec3(-other.g7.z, other.g0.y, other.g7.x) + vec3(self.g1.z) * vec3(other.g7.y, -other.g7.x, other.g0.y) + vec3(self.g2.x) * vec3(other.g2.x, other.g6.z, -other.g6.y) + vec3(self.g2.y) * vec3(-other.g6.z, other.g2.x, other.g6.x) + vec3(self.g2.z) * vec3(other.g6.y, -other.g6.x, other.g2.x) - vec3(self.g2.w) * other.g1, vec3(self.g1.x) * vec3(other.g2.y, other.g8.z, -other.g8.y) + vec3(self.g1.y) * vec3(-other.g8.z, other.g2.y, other.g8.x) + vec3(self.g1.z) * vec3(other.g8.y, -other.g8.x, other.g2.y) + vec3(self.g2.x) * vec3(other.g6.w, other.g7.z, -other.g7.y) + vec3(self.g2.x) * vec3(other.g0.y, other.g1.z, -other.g1.y) + vec3(self.g2.y) * vec3(-other.g7.z, other.g6.w, other.g7.x) + vec3(self.g2.y) * vec3(-other.g1.z, other.g0.y, other.g1.x) + vec3(self.g2.z) * vec3(other.g7.y, -other.g7.x, other.g6.w) + vec3(self.g2.z) * vec3(other.g1.y, -other.g1.x, other.g0.y) + vec3(self.g2.w) * other.g8, vec3(self.g0.x) * vec3(-other.g10.y, -other.g5.z, other.g5.y) + vec3(self.g0.y) * vec3(other.g5.z, -other.g10.y, -other.g5.x) + vec3(self.g0.z) * vec3(-other.g5.y, other.g5.x, -other.g10.y) + vec3(self.g1.x) * vec3(other.g5.w, other.g9.z, -other.g9.y) + vec3(self.g1.y) * vec3(-other.g9.z, other.g5.w, other.g9.x) + vec3(self.g1.z) * vec3(other.g9.y, -other.g9.x, other.g5.w) + vec3(self.g2.x) * vec3(other.g10.x, other.g3.z, -other.g3.y) + vec3(self.g2.y) * vec3(-other.g3.z, other.g10.x, other.g3.x) + vec3(self.g2.z) * vec3(other.g3.y, -other.g3.x, other.g10.x) + vec3(self.g2.w) * other.g4, vec2(self.g0.x) * vec2(-other.g9.x, 0.0) + vec2(self.g0.x) * vec2(-other.g4.x, 0.0) + vec2(self.g0.y) * vec2(-other.g9.y, 0.0) + vec2(self.g0.y) * vec2(-other.g4.y, 0.0) + vec2(self.g0.z) * vec2(-other.g9.z, 0.0) + vec2(self.g0.z) * vec2(-other.g4.z, 0.0) - vec2(self.g1.x) * vec2(other.g3.x, other.g5.x) - vec2(self.g1.y) * vec2(other.g3.y, other.g5.y) - vec2(self.g1.z) * vec2(other.g3.z, other.g5.z) + vec2(self.g2.x) * vec2(0.0, other.g9.x) + vec2(self.g2.x) * vec2(0.0, -other.g4.x) + vec2(self.g2.y) * vec2(0.0, other.g9.y) + vec2(self.g2.y) * vec2(0.0, -other.g4.y) + vec2(self.g2.z) * vec2(0.0, other.g9.z) + vec2(self.g2.z) * vec2(0.0, -other.g4.z) + vec2(self.g2.w) * other.g10 * vec2(-1.0, 1.0));
}

MultiVector dipole__wedge_dot__plane(Dipole self, Plane other) {
    return MultiVector(vec2(0.0), vec3(0.0), vec2(0.0), vec3(self.g0.x) * vec3(0.0, other.g0.z, -other.g0.y) + vec3(self.g0.y) * vec3(-other.g0.z, 0.0, other.g0.x) + vec3(self.g0.z) * vec3(other.g0.y, -other.g0.x, 0.0), self.g0 * vec3(other.g0.w) + vec3(self.g2.w) * vec3(other.g0.x, other.g0.y, other.g0.z), vec4(self.g1.x) * vec4(other.g0.w, 0.0, 0.0, -other.g0.x) + vec4(self.g1.y) * vec4(0.0, other.g0.w, 0.0, -other.g0.y) + vec4(self.g1.z) * vec4(0.0, 0.0, other.g0.w, -other.g0.z) + vec4(self.g2.x) * vec4(0.0, -other.g0.z, other.g0.y, 0.0) + vec4(self.g2.y) * vec4(other.g0.z, 0.0, -other.g0.x, 0.0) + vec4(self.g2.z) * vec4(-other.g0.y, other.g0.x, 0.0, 0.0), vec4(0.0), vec3(0.0), vec3(0.0), vec3(0.0) - self.g0 * vec3(other.g0.w) + vec3(self.g1.x) * vec3(0.0, other.g0.z, -other.g0.y) + vec3(self.g1.y) * vec3(-other.g0.z, 0.0, other.g0.x) + vec3(self.g1.z) * vec3(other.g0.y, -other.g0.x, 0.0), vec2(self.g0.x) * vec2(-other.g0.x, 0.0) + vec2(self.g0.y) * vec2(-other.g0.y, 0.0) + vec2(self.g0.z) * vec2(-other.g0.z, 0.0) + vec2(self.g2.x) * vec2(0.0, other.g0.x) + vec2(self.g2.y) * vec2(0.0, other.g0.y) + vec2(self.g2.z) * vec2(0.0, other.g0.z) + vec2(self.g2.w) * vec2(0.0, other.g0.w));
}

MultiVector dipole__wedge_dot__round_point(Dipole self, RoundPoint other) {
    return MultiVector(vec2(0.0), self.g0 * vec3(other.g1.y) + vec3(self.g1.x) * vec3(0.0, other.g0.z, -other.g0.y) + vec3(self.g1.y) * vec3(-other.g0.z, 0.0, other.g0.x) + vec3(self.g1.z) * vec3(other.g0.y, -other.g0.x, 0.0) - vec3(self.g2.x, self.g2.y, self.g2.z) * vec3(other.g1.x), vec2(self.g0.x) * vec2(other.g0.x, 0.0) + vec2(self.g0.y) * vec2(other.g0.y, 0.0) + vec2(self.g0.z) * vec2(other.g0.z, 0.0) + vec2(self.g2.x) * vec2(0.0, -other.g0.x) + vec2(self.g2.y) * vec2(0.0, -other.g0.y) + vec2(self.g2.z) * vec2(0.0, -other.g0.z) + vec2(self.g2.w) * other.g1 * vec2(-1.0, 1.0), vec3(0.0), vec3(0.0), vec4(0.0), vec4(self.g0.x) * vec4(0.0, -other.g0.z, other.g0.y, 0.0) + vec4(self.g0.y) * vec4(other.g0.z, 0.0, -other.g0.x, 0.0) + vec4(self.g0.z) * vec4(-other.g0.y, other.g0.x, 0.0, 0.0) + vec4(self.g1.x) * vec4(other.g1.x, 0.0, 0.0, -other.g0.x) + vec4(self.g1.y) * vec4(0.0, other.g1.x, 0.0, -other.g0.y) + vec4(self.g1.z) * vec4(0.0, 0.0, other.g1.x, -other.g0.z), self.g0 * vec3(other.g1.y) + vec3(self.g2.x, self.g2.y, self.g2.z) * vec3(other.g1.x) - vec3(self.g2.w) * other.g0, self.g1 * vec3(other.g1.y) + vec3(self.g2.x) * vec3(0.0, other.g0.z, -other.g0.y) + vec3(self.g2.y) * vec3(-other.g0.z, 0.0, other.g0.x) + vec3(self.g2.z) * vec3(other.g0.y, -other.g0.x, 0.0), vec3(0.0), vec2(0.0));
}

Dipole dipole__wedge_dot__scalar(Dipole self, Scalar other) {
    return Dipole(self.g0 * vec3(other.g0), self.g1 * vec3(other.g0), self.g2 * vec4(other.g0));
}

MultiVector dipole__wedge_dot__sphere(Dipole self, Sphere other) {
    return MultiVector(vec2(0.0), vec3(0.0), vec2(0.0), vec3(self.g0.x) * vec3(0.0, other.g0.z, -other.g0.y) + vec3(self.g0.y) * vec3(-other.g0.z, 0.0, other.g0.x) + vec3(self.g0.z) * vec3(other.g0.y, -other.g0.x, 0.0) + self.g1 * vec3(other.g1.x), self.g0 * vec3(other.g1.y) + vec3(self.g2.x, self.g2.y, self.g2.z) * vec3(other.g1.x) + vec3(self.g2.w) * other.g0, vec4(self.g1.x) * vec4(other.g1.y, 0.0, 0.0, -other.g0.x) + vec4(self.g1.y) * vec4(0.0, other.g1.y, 0.0, -other.g0.y) + vec4(self.g1.z) * vec4(0.0, 0.0, other.g1.y, -other.g0.z) + vec4(self.g2.x) * vec4(0.0, -other.g0.z, other.g0.y, 0.0) + vec4(self.g2.y) * vec4(other.g0.z, 0.0, -other.g0.x, 0.0) + vec4(self.g2.z) * vec4(-other.g0.y, other.g0.x, 0.0, 0.0), vec4(0.0), vec3(0.0), vec3(0.0), vec3(0.0) - self.g0 * vec3(other.g1.y) + vec3(self.g1.x) * vec3(0.0, other.g0.z, -other.g0.y) + vec3(self.g1.y) * vec3(-other.g0.z, 0.0, other.g0.x) + vec3(self.g1.z) * vec3(other.g0.y, -other.g0.x, 0.0) + vec3(self.g2.x, self.g2.y, self.g2.z) * vec3(other.g1.x), vec2(self.g0.x) * vec2(-other.g0.x, 0.0) + vec2(self.g0.y) * vec2(-other.g0.y, 0.0) + vec2(self.g0.z) * vec2(-other.g0.z, 0.0) + vec2(self.g2.x) * vec2(0.0, other.g0.x) + vec2(self.g2.y) * vec2(0.0, other.g0.y) + vec2(self.g2.z) * vec2(0.0, other.g0.z) + vec2(self.g2.w) * other.g1 * vec2(-1.0, 1.0));
}

DualNum dual_num__wedge_dot__anti_scalar(DualNum self, AntiScalar other) {
    return DualNum(self.g0.yx * vec2(-other.g0));
}

MultiVector dual_num__wedge_dot__circle(DualNum self, Circle other) {
    return MultiVector(vec2(0.0), vec3(0.0), vec2(0.0), vec3(0.0) - vec3(self.g0.y) * vec3(other.g0.x, other.g0.y, other.g0.z), vec3(0.0) - vec3(self.g0.y) * other.g1, vec4(self.g0.y) * vec4(-other.g2.x, -other.g2.y, -other.g2.z, other.g0.w), vec4(self.g0.x) * other.g0, vec3(self.g0.x) * other.g1, vec3(self.g0.x) * other.g2, vec3(0.0), vec2(0.0));
}

MultiVector dual_num__wedge_dot__dipole(DualNum self, Dipole other) {
    return MultiVector(vec2(0.0), vec3(0.0), vec2(0.0), vec3(self.g0.x) * other.g0, vec3(self.g0.x) * other.g1, vec4(self.g0.x) * other.g2, vec4(self.g0.y) * vec4(other.g0.x, other.g0.y, other.g0.z, -other.g2.w), vec3(self.g0.y) * other.g1, vec3(self.g0.y) * vec3(other.g2.x, other.g2.y, other.g2.z), vec3(0.0), vec2(0.0));
}

DualNum dual_num__wedge_dot__dual_num(DualNum self, DualNum other) {
    return DualNum(vec2(self.g0.x) * other.g0 + vec2(self.g0.y) * other.g0.yx * vec2(-1.0, 1.0));
}

MultiVector dual_num__wedge_dot__flat_point(DualNum self, FlatPoint other) {
    return MultiVector(vec2(0.0), vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0), vec4(self.g0.x) * other.g0, vec4(self.g0.y) * vec4(0.0, 0.0, 0.0, -other.g0.w), vec3(0.0), vec3(self.g0.y) * vec3(other.g0.x, other.g0.y, other.g0.z), vec3(0.0), vec2(0.0));
}

MultiVector dual_num__wedge_dot__flector(DualNum self, Flector other) {
    return MultiVector(vec2(0.0), vec3(0.0) - vec3(self.g0.y) * vec3(other.g1.x, other.g1.y, other.g1.z), vec2(self.g0.y) * vec2(0.0, other.g1.w), vec3(0.0), vec3(0.0), vec4(self.g0.x) * other.g0, vec4(self.g0.y) * vec4(0.0, 0.0, 0.0, -other.g0.w), vec3(0.0), vec3(self.g0.y) * vec3(other.g0.x, other.g0.y, other.g0.z), vec3(self.g0.x) * vec3(other.g1.x, other.g1.y, other.g1.z), vec2(self.g0.x) * vec2(0.0, other.g1.w));
}

MultiVector dual_num__wedge_dot__line(DualNum self, Line other) {
    return MultiVector(vec2(0.0), vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0) - vec3(self.g0.y) * other.g0, vec4(self.g0.y) * vec4(-other.g1.x, -other.g1.y, -other.g1.z, 0.0), vec4(0.0), vec3(self.g0.x) * other.g0, vec3(self.g0.x) * other.g1, vec3(0.0), vec2(0.0));
}

MultiVector dual_num__wedge_dot__motor(DualNum self, Motor other) {
    return MultiVector(self.g0.yx * vec2(-other.g0.w), vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0) - vec3(self.g0.y) * vec3(other.g0.x, other.g0.y, other.g0.z), vec4(self.g0.y) * vec4(-other.g1.x, -other.g1.y, -other.g1.z, 0.0), vec4(0.0), vec3(self.g0.x) * vec3(other.g0.x, other.g0.y, other.g0.z), vec3(self.g0.x) * other.g1, vec3(0.0), vec2(0.0));
}

MultiVector dual_num__wedge_dot__multi_vector(DualNum self, MultiVector other) {
    return MultiVector(vec2(self.g0.x) * other.g0 + vec2(self.g0.y) * other.g0.yx * vec2(-1.0, 1.0), vec3(self.g0.x) * other.g1 - vec3(self.g0.y) * other.g9, vec2(self.g0.x) * other.g2 + vec2(self.g0.y) * other.g10, vec3(self.g0.x) * other.g3 - vec3(self.g0.y) * vec3(other.g6.x, other.g6.y, other.g6.z), vec3(self.g0.x) * other.g4 - vec3(self.g0.y) * other.g7, vec4(self.g0.x) * other.g5 + vec4(self.g0.y) * vec4(-other.g8.x, -other.g8.y, -other.g8.z, other.g6.w), vec4(self.g0.x) * other.g6 + vec4(self.g0.y) * vec4(other.g3.x, other.g3.y, other.g3.z, -other.g5.w), vec3(self.g0.x) * other.g7 + vec3(self.g0.y) * other.g4, vec3(self.g0.x) * other.g8 + vec3(self.g0.y) * vec3(other.g5.x, other.g5.y, other.g5.z), vec3(self.g0.x) * other.g9 + vec3(self.g0.y) * other.g1, vec2(self.g0.x) * other.g10 - vec2(self.g0.y) * other.g2);
}

MultiVector dual_num__wedge_dot__plane(DualNum self, Plane other) {
    return MultiVector(vec2(0.0), vec3(0.0) - vec3(self.g0.y) * vec3(other.g0.x, other.g0.y, other.g0.z), vec2(self.g0.y) * vec2(0.0, other.g0.w), vec3(0.0), vec3(0.0), vec4(0.0), vec4(0.0), vec3(0.0), vec3(0.0), vec3(self.g0.x) * vec3(other.g0.x, other.g0.y, other.g0.z), vec2(self.g0.x) * vec2(0.0, other.g0.w));
}

MultiVector dual_num__wedge_dot__round_point(DualNum self, RoundPoint other) {
    return MultiVector(vec2(0.0), vec3(self.g0.x) * other.g0, vec2(self.g0.x) * other.g1, vec3(0.0), vec3(0.0), vec4(0.0), vec4(0.0), vec3(0.0), vec3(0.0), vec3(self.g0.y) * other.g0, vec2(0.0) - vec2(self.g0.y) * other.g1);
}

DualNum dual_num__wedge_dot__scalar(DualNum self, Scalar other) {
    return DualNum(self.g0 * vec2(other.g0));
}

MultiVector dual_num__wedge_dot__sphere(DualNum self, Sphere other) {
    return MultiVector(vec2(0.0), vec3(0.0) - vec3(self.g0.y) * other.g0, vec2(self.g0.y) * other.g1, vec3(0.0), vec3(0.0), vec4(0.0), vec4(0.0), vec3(0.0), vec3(0.0), vec3(self.g0.x) * other.g0, vec2(self.g0.x) * other.g1);
}

Circle flat_point__wedge_dot__anti_scalar(FlatPoint self, AntiScalar other) {
    return Circle(vec4(self.g0.w) * vec4(0.0, 0.0, 0.0, -other.g0), vec3(0.0), vec3(self.g0.x, self.g0.y, self.g0.z) * vec3(other.g0));
}

MultiVector flat_point__wedge_dot__circle(FlatPoint self, Circle other) {
    return MultiVector(vec2(self.g0.x) * vec2(0.0, -other.g0.x) + vec2(self.g0.y) * vec2(0.0, -other.g0.y) + vec2(self.g0.z) * vec2(0.0, -other.g0.z) + vec2(self.g0.w) * vec2(0.0, -other.g0.w), vec3(self.g0.x) * vec3(0.0, -other.g0.z, other.g0.y) + vec3(self.g0.y) * vec3(other.g0.z, 0.0, -other.g0.x) + vec3(self.g0.z) * vec3(-other.g0.y, other.g0.x, 0.0) - vec3(self.g0.w) * other.g1, vec2(self.g0.x) * vec2(0.0, -other.g1.x) + vec2(self.g0.y) * vec2(0.0, -other.g1.y) + vec2(self.g0.z) * vec2(0.0, -other.g1.z), vec3(0.0), vec3(0.0), vec4(0.0), vec4(self.g0.x) * vec4(0.0, 0.0, 0.0, other.g0.x) + vec4(self.g0.y) * vec4(0.0, 0.0, 0.0, other.g0.y) + self.g0.wwwz * other.g0.xyzz * vec4(-1.0, -1.0, -1.0, 1.0), vec3(self.g0.x) * vec3(0.0, other.g0.z, -other.g0.y) + vec3(self.g0.y) * vec3(-other.g0.z, 0.0, other.g0.x) + vec3(self.g0.z) * vec3(other.g0.y, -other.g0.x, 0.0), vec3(self.g0.x) * vec3(other.g0.w, other.g1.z, -other.g1.y) + vec3(self.g0.y) * vec3(-other.g1.z, other.g0.w, other.g1.x) + vec3(self.g0.z) * vec3(other.g1.y, -other.g1.x, other.g0.w) + vec3(self.g0.w) * other.g2, vec3(0.0), vec2(0.0));
}

MultiVector flat_point__wedge_dot__dipole(FlatPoint self, Dipole other) {
    return MultiVector(vec2(self.g0.x) * vec2(-other.g0.x, 0.0) + vec2(self.g0.y) * vec2(-other.g0.y, 0.0) + vec2(self.g0.z) * vec2(-other.g0.z, 0.0) + vec2(self.g0.w) * vec2(other.g2.w, 0.0), vec3(0.0), vec2(0.0), vec3(0.0) - vec3(self.g0.w) * other.g0, vec3(self.g0.x) * vec3(0.0, other.g0.z, -other.g0.y) + vec3(self.g0.y) * vec3(-other.g0.z, 0.0, other.g0.x) + vec3(self.g0.z) * vec3(other.g0.y, -other.g0.x, 0.0), vec4(self.g0.x) * vec4(-other.g2.w, other.g1.z, -other.g1.y, -other.g0.x) + vec4(self.g0.y) * vec4(-other.g1.z, -other.g2.w, other.g1.x, -other.g0.y) + vec4(self.g0.z) * vec4(other.g1.y, -other.g1.x, -other.g2.w, -other.g0.z) + vec4(self.g0.w) * vec4(other.g2.x, other.g2.y, other.g2.z, 0.0), vec4(0.0), vec3(0.0), vec3(0.0), vec3(self.g0.x) * vec3(0.0, other.g0.z, -other.g0.y) + vec3(self.g0.y) * vec3(-other.g0.z, 0.0, other.g0.x) + vec3(self.g0.z) * vec3(other.g0.y, -other.g0.x, 0.0) + vec3(self.g0.w) * other.g1, vec2(self.g0.x) * vec2(0.0, -other.g1.x) + vec2(self.g0.y) * vec2(0.0, -other.g1.y) + vec2(self.g0.z) * vec2(0.0, -other.g1.z));
}

MultiVector flat_point__wedge_dot__dual_num(FlatPoint self, DualNum other) {
    return MultiVector(vec2(0.0), vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0), self.g0 * vec4(other.g0.x), vec4(self.g0.w) * vec4(0.0, 0.0, 0.0, -other.g0.y), vec3(0.0), vec3(self.g0.x, self.g0.y, self.g0.z) * vec3(other.g0.y), vec3(0.0), vec2(0.0));
}

MultiVector flat_point__wedge_dot__flat_point(FlatPoint self, FlatPoint other) {
    return MultiVector(vec2(self.g0.w) * vec2(other.g0.w, 0.0), vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0), self.g0.xyzx * vec4(-other.g0.w, -other.g0.w, -other.g0.w, 0.0) + vec4(self.g0.w) * vec4(other.g0.x, other.g0.y, other.g0.z, 0.0), vec4(0.0), vec3(0.0), vec3(0.0), vec3(0.0), vec2(0.0));
}

MultiVector flat_point__wedge_dot__flector(FlatPoint self, Flector other) {
    return MultiVector(vec2(self.g0.w) * vec2(other.g0.w, 0.0), vec3(0.0), vec2(0.0), vec3(0.0), vec3(self.g0.w) * vec3(other.g1.x, other.g1.y, other.g1.z), vec4(self.g0.x) * vec4(-other.g0.w, -other.g1.z, other.g1.y, 0.0) + vec4(self.g0.y) * vec4(other.g1.z, -other.g0.w, -other.g1.x, 0.0) + vec4(self.g0.z) * vec4(-other.g1.y, other.g1.x, -other.g0.w, 0.0) + vec4(self.g0.w) * vec4(other.g0.x, other.g0.y, other.g0.z, 0.0), vec4(0.0), vec3(0.0), vec3(0.0), vec3(0.0), vec2(self.g0.x) * vec2(0.0, other.g1.x) + vec2(self.g0.y) * vec2(0.0, other.g1.y) + vec2(self.g0.z) * vec2(0.0, other.g1.z) + vec2(self.g0.w) * vec2(0.0, other.g1.w));
}

MultiVector flat_point__wedge_dot__line(FlatPoint self, Line other) {
    return MultiVector(vec2(0.0), vec3(0.0) - vec3(self.g0.w) * other.g0, vec2(self.g0.x) * vec2(0.0, -other.g0.x) + vec2(self.g0.y) * vec2(0.0, -other.g0.y) + vec2(self.g0.z) * vec2(0.0, -other.g0.z), vec3(0.0), vec3(0.0), vec4(0.0), vec4(0.0), vec3(0.0), vec3(self.g0.x) * vec3(0.0, other.g0.z, -other.g0.y) + vec3(self.g0.y) * vec3(-other.g0.z, 0.0, other.g0.x) + vec3(self.g0.z) * vec3(other.g0.y, -other.g0.x, 0.0) + vec3(self.g0.w) * other.g1, vec3(0.0), vec2(0.0));
}

MultiVector flat_point__wedge_dot__motor(FlatPoint self, Motor other) {
    return MultiVector(vec2(0.0), vec3(0.0) - vec3(self.g0.w) * vec3(other.g0.x, other.g0.y, other.g0.z), vec2(self.g0.x) * vec2(0.0, -other.g0.x) + vec2(self.g0.y) * vec2(0.0, -other.g0.y) + vec2(self.g0.z) * vec2(0.0, -other.g0.z), vec3(0.0), vec3(0.0), vec4(0.0), vec4(self.g0.w) * vec4(0.0, 0.0, 0.0, -other.g0.w), vec3(0.0), vec3(self.g0.x) * vec3(other.g0.w, other.g0.z, -other.g0.y) + vec3(self.g0.y) * vec3(-other.g0.z, other.g0.w, other.g0.x) + vec3(self.g0.z) * vec3(other.g0.y, -other.g0.x, other.g0.w) + vec3(self.g0.w) * other.g1, vec3(0.0), vec2(0.0));
}

MultiVector flat_point__wedge_dot__multi_vector(FlatPoint self, MultiVector other) {
    return MultiVector(vec2(0.0) - vec2(self.g0.x) * vec2(other.g3.x, other.g6.x) - vec2(self.g0.y) * vec2(other.g3.y, other.g6.y) - vec2(self.g0.z) * vec2(other.g3.z, other.g6.z) + vec2(self.g0.w) * vec2(other.g5.w, -other.g6.w), vec3(self.g0.x) * vec3(-other.g2.x, -other.g6.z, other.g6.y) + vec3(self.g0.y) * vec3(other.g6.z, -other.g2.x, -other.g6.x) + vec3(self.g0.z) * vec3(-other.g6.y, other.g6.x, -other.g2.x) - vec3(self.g0.w) * other.g7, vec2(self.g0.x) * vec2(0.0, -other.g7.x) + vec2(self.g0.x) * vec2(0.0, -other.g1.x) + vec2(self.g0.y) * vec2(0.0, -other.g7.y) + vec2(self.g0.y) * vec2(0.0, -other.g1.y) + vec2(self.g0.z) * vec2(0.0, -other.g7.z) + vec2(self.g0.z) * vec2(0.0, -other.g1.z) + vec2(self.g0.w) * other.g2 * vec2(-1.0, 1.0), vec3(0.0) - vec3(self.g0.w) * other.g3, vec3(self.g0.x) * vec3(other.g10.x, other.g3.z, -other.g3.y) + vec3(self.g0.y) * vec3(-other.g3.z, other.g10.x, other.g3.x) + vec3(self.g0.z) * vec3(other.g3.y, -other.g3.x, other.g10.x) + vec3(self.g0.w) * other.g9, vec4(self.g0.x) * vec4(-other.g5.w, -other.g9.z, other.g9.y, -other.g3.x) + vec4(self.g0.x) * vec4(other.g0.x, other.g4.z, -other.g4.y, 0.0) + vec4(self.g0.y) * vec4(other.g9.z, -other.g5.w, -other.g9.x, -other.g3.y) + vec4(self.g0.y) * vec4(-other.g4.z, other.g0.x, other.g4.x, 0.0) + vec4(self.g0.z) * vec4(-other.g9.y, other.g9.x, -other.g5.w, -other.g3.z) + vec4(self.g0.z) * vec4(other.g4.y, -other.g4.x, other.g0.x, 0.0) + vec4(self.g0.w) * vec4(other.g5.x, other.g5.y, other.g5.z, other.g0.x), vec4(self.g0.x) * vec4(0.0, 0.0, 0.0, other.g6.x) + vec4(self.g0.y) * vec4(0.0, 0.0, 0.0, other.g6.y) + vec4(self.g0.z) * vec4(0.0, 0.0, 0.0, other.g6.z) - vec4(self.g0.w) * vec4(other.g6.x, other.g6.y, other.g6.z, other.g0.y), vec3(self.g0.x) * vec3(other.g2.x, other.g6.z, -other.g6.y) + vec3(self.g0.y) * vec3(-other.g6.z, other.g2.x, other.g6.x) + vec3(self.g0.z) * vec3(other.g6.y, -other.g6.x, other.g2.x) - vec3(self.g0.w) * other.g1, vec3(self.g0.x) * vec3(other.g6.w, other.g7.z, -other.g7.y) + vec3(self.g0.x) * vec3(other.g0.y, other.g1.z, -other.g1.y) + vec3(self.g0.y) * vec3(-other.g7.z, other.g6.w, other.g7.x) + vec3(self.g0.y) * vec3(-other.g1.z, other.g0.y, other.g1.x) + vec3(self.g0.z) * vec3(other.g7.y, -other.g7.x, other.g6.w) + vec3(self.g0.z) * vec3(other.g1.y, -other.g1.x, other.g0.y) + vec3(self.g0.w) * other.g8, vec3(self.g0.x) * vec3(other.g10.x, other.g3.z, -other.g3.y) + vec3(self.g0.y) * vec3(-other.g3.z, other.g10.x, other.g3.x) + vec3(self.g0.z) * vec3(other.g3.y, -other.g3.x, other.g10.x) + vec3(self.g0.w) * other.g4, vec2(self.g0.x) * vec2(0.0, other.g9.x) + vec2(self.g0.x) * vec2(0.0, -other.g4.x) + vec2(self.g0.y) * vec2(0.0, other.g9.y) + vec2(self.g0.y) * vec2(0.0, -other.g4.y) + vec2(self.g0.z) * vec2(0.0, other.g9.z) + vec2(self.g0.z) * vec2(0.0, -other.g4.z) + vec2(self.g0.w) * other.g10 * vec2(-1.0, 1.0));
}

MultiVector flat_point__wedge_dot__plane(FlatPoint self, Plane other) {
    return MultiVector(vec2(0.0), vec3(0.0), vec2(0.0), vec3(0.0), vec3(self.g0.w) * vec3(other.g0.x, other.g0.y, other.g0.z), vec4(self.g0.x) * vec4(0.0, -other.g0.z, other.g0.y, 0.0) + vec4(self.g0.y) * vec4(other.g0.z, 0.0, -other.g0.x, 0.0) + vec4(self.g0.z) * vec4(-other.g0.y, other.g0.x, 0.0, 0.0), vec4(0.0), vec3(0.0), vec3(0.0), vec3(0.0), vec2(self.g0.x) * vec2(0.0, other.g0.x) + vec2(self.g0.y) * vec2(0.0, other.g0.y) + vec2(self.g0.z) * vec2(0.0, other.g0.z) + vec2(self.g0.w) * vec2(0.0, other.g0.w));
}

MultiVector flat_point__wedge_dot__round_point(FlatPoint self, RoundPoint other) {
    return MultiVector(vec2(0.0), vec3(0.0) - vec3(self.g0.x, self.g0.y, self.g0.z) * vec3(other.g1.x), vec2(self.g0.x) * vec2(0.0, -other.g0.x) + vec2(self.g0.y) * vec2(0.0, -other.g0.y) + vec2(self.g0.z) * vec2(0.0, -other.g0.z) + vec2(self.g0.w) * other.g1 * vec2(-1.0, 1.0), vec3(0.0), vec3(0.0), vec4(0.0), vec4(0.0), vec3(self.g0.x, self.g0.y, self.g0.z) * vec3(other.g1.x) - vec3(self.g0.w) * other.g0, vec3(self.g0.x) * vec3(0.0, other.g0.z, -other.g0.y) + vec3(self.g0.y) * vec3(-other.g0.z, 0.0, other.g0.x) + vec3(self.g0.z) * vec3(other.g0.y, -other.g0.x, 0.0), vec3(0.0), vec2(0.0));
}

FlatPoint flat_point__wedge_dot__scalar(FlatPoint self, Scalar other) {
    return FlatPoint(self.g0 * vec4(other.g0));
}

MultiVector flat_point__wedge_dot__sphere(FlatPoint self, Sphere other) {
    return MultiVector(vec2(0.0), vec3(0.0), vec2(0.0), vec3(0.0), vec3(self.g0.x, self.g0.y, self.g0.z) * vec3(other.g1.x) + vec3(self.g0.w) * other.g0, vec4(self.g0.x) * vec4(0.0, -other.g0.z, other.g0.y, 0.0) + vec4(self.g0.y) * vec4(other.g0.z, 0.0, -other.g0.x, 0.0) + vec4(self.g0.z) * vec4(-other.g0.y, other.g0.x, 0.0, 0.0), vec4(0.0), vec3(0.0), vec3(0.0), vec3(self.g0.x, self.g0.y, self.g0.z) * vec3(other.g1.x), vec2(self.g0.x) * vec2(0.0, other.g0.x) + vec2(self.g0.y) * vec2(0.0, other.g0.y) + vec2(self.g0.z) * vec2(0.0, other.g0.z) + vec2(self.g0.w) * other.g1 * vec2(-1.0, 1.0));
}

MultiVector flector__wedge_dot__anti_scalar(Flector self, AntiScalar other) {
    return MultiVector(vec2(0.0), vec3(0.0) - vec3(self.g1.x, self.g1.y, self.g1.z) * vec3(other.g0), vec2(self.g1.w) * vec2(0.0, other.g0), vec3(0.0), vec3(0.0), vec4(0.0), vec4(self.g0.w) * vec4(0.0, 0.0, 0.0, -other.g0), vec3(0.0), vec3(self.g0.x, self.g0.y, self.g0.z) * vec3(other.g0), vec3(0.0), vec2(0.0));
}

MultiVector flector__wedge_dot__circle(Flector self, Circle other) {
    return MultiVector(vec2(self.g0.x) * vec2(0.0, -other.g0.x) + vec2(self.g0.y) * vec2(0.0, -other.g0.y) + vec2(self.g0.z) * vec2(0.0, -other.g0.z) + vec2(self.g0.w) * vec2(0.0, -other.g0.w), vec3(self.g0.x) * vec3(0.0, -other.g0.z, other.g0.y) + vec3(self.g0.y) * vec3(other.g0.z, 0.0, -other.g0.x) + vec3(self.g0.z) * vec3(-other.g0.y, other.g0.x, 0.0) - vec3(self.g0.w) * other.g1 + vec3(self.g1.x) * vec3(0.0, -other.g1.z, other.g1.y) + vec3(self.g1.y) * vec3(other.g1.z, 0.0, -other.g1.x) + vec3(self.g1.z) * vec3(-other.g1.y, other.g1.x, 0.0) - vec3(self.g1.w) * vec3(other.g0.x, other.g0.y, other.g0.z), vec2(self.g0.x) * vec2(0.0, -other.g1.x) + vec2(self.g0.y) * vec2(0.0, -other.g1.y) + vec2(self.g0.z) * vec2(0.0, -other.g1.z) + vec2(self.g1.x) * vec2(other.g0.x, -other.g2.x) + vec2(self.g1.y) * vec2(other.g0.y, -other.g2.y) + vec2(self.g1.z) * vec2(other.g0.z, -other.g2.z) + vec2(self.g1.w) * vec2(0.0, other.g0.w), vec3(0.0), vec3(0.0), vec4(0.0), vec4(self.g0.x) * vec4(0.0, 0.0, 0.0, other.g0.x) + vec4(self.g0.y) * vec4(0.0, 0.0, 0.0, other.g0.y) + self.g0.wwwz * other.g0.xyzz * vec4(-1.0, -1.0, -1.0, 1.0) + vec4(self.g1.x) * vec4(0.0, -other.g0.z, other.g0.y, other.g1.x) + vec4(self.g1.y) * vec4(other.g0.z, 0.0, -other.g0.x, other.g1.y) + vec4(self.g1.z) * vec4(-other.g0.y, other.g0.x, 0.0, other.g1.z), vec3(self.g0.x) * vec3(0.0, other.g0.z, -other.g0.y) + vec3(self.g0.y) * vec3(-other.g0.z, 0.0, other.g0.x) + vec3(self.g0.z) * vec3(other.g0.y, -other.g0.x, 0.0) - vec3(self.g1.x, self.g1.y, self.g1.z) * vec3(other.g0.w) + vec3(self.g1.w) * vec3(other.g0.x, other.g0.y, other.g0.z), vec3(self.g0.x) * vec3(other.g0.w, other.g1.z, -other.g1.y) + vec3(self.g0.y) * vec3(-other.g1.z, other.g0.w, other.g1.x) + vec3(self.g0.z) * vec3(other.g1.y, -other.g1.x, other.g0.w) + vec3(self.g0.w) * other.g2 + vec3(self.g1.x) * vec3(0.0, other.g2.z, -other.g2.y) + vec3(self.g1.y) * vec3(-other.g2.z, 0.0, other.g2.x) + vec3(self.g1.z) * vec3(other.g2.y, -other.g2.x, 0.0) + vec3(self.g1.w) * other.g1, vec3(0.0), vec2(0.0));
}

MultiVector flector__wedge_dot__dipole(Flector self, Dipole other) {
    return MultiVector(vec2(self.g0.x) * vec2(-other.g0.x, 0.0) + vec2(self.g0.y) * vec2(-other.g0.y, 0.0) + vec2(self.g0.z) * vec2(-other.g0.z, 0.0) + vec2(self.g0.w) * vec2(other.g2.w, 0.0), vec3(0.0), vec2(0.0), vec3(0.0) - vec3(self.g0.w) * other.g0 + vec3(self.g1.x) * vec3(0.0, -other.g0.z, other.g0.y) + vec3(self.g1.y) * vec3(other.g0.z, 0.0, -other.g0.x) + vec3(self.g1.z) * vec3(-other.g0.y, other.g0.x, 0.0), vec3(self.g0.x) * vec3(0.0, other.g0.z, -other.g0.y) + vec3(self.g0.y) * vec3(-other.g0.z, 0.0, other.g0.x) + vec3(self.g0.z) * vec3(other.g0.y, -other.g0.x, 0.0) + vec3(self.g1.x, self.g1.y, self.g1.z) * vec3(other.g2.w) + vec3(self.g1.w) * other.g0, vec4(self.g0.x) * vec4(-other.g2.w, other.g1.z, -other.g1.y, -other.g0.x) + vec4(self.g0.y) * vec4(-other.g1.z, -other.g2.w, other.g1.x, -other.g0.y) + vec4(self.g0.z) * vec4(other.g1.y, -other.g1.x, -other.g2.w, -other.g0.z) + vec4(self.g0.w) * vec4(other.g2.x, other.g2.y, other.g2.z, 0.0) + vec4(self.g1.x) * vec4(0.0, other.g2.z, -other.g2.y, -other.g1.x) + vec4(self.g1.y) * vec4(-other.g2.z, 0.0, other.g2.x, -other.g1.y) + vec4(self.g1.z) * vec4(other.g2.y, -other.g2.x, 0.0, -other.g1.z) + vec4(self.g1.w) * vec4(other.g1.x, other.g1.y, other.g1.z, 0.0), vec4(0.0), vec3(0.0), vec3(0.0), vec3(self.g0.x) * vec3(0.0, other.g0.z, -other.g0.y) + vec3(self.g0.y) * vec3(-other.g0.z, 0.0, other.g0.x) + vec3(self.g0.z) * vec3(other.g0.y, -other.g0.x, 0.0) + vec3(self.g0.w) * other.g1 + vec3(self.g1.x) * vec3(0.0, other.g1.z, -other.g1.y) + vec3(self.g1.y) * vec3(-other.g1.z, 0.0, other.g1.x) + vec3(self.g1.z) * vec3(other.g1.y, -other.g1.x, 0.0) + vec3(self.g1.w) * other.g0, vec2(self.g0.x) * vec2(0.0, -other.g1.x) + vec2(self.g0.y) * vec2(0.0, -other.g1.y) + vec2(self.g0.z) * vec2(0.0, -other.g1.z) + vec2(self.g1.x) * vec2(other.g0.x, -other.g2.x) + vec2(self.g1.y) * vec2(other.g0.y, -other.g2.y) + vec2(self.g1.z) * vec2(other.g0.z, -other.g2.z) + vec2(self.g1.w) * vec2(0.0, -other.g2.w));
}

MultiVector flector__wedge_dot__dual_num(Flector self, DualNum other) {
    return MultiVector(vec2(0.0), vec3(0.0) - vec3(self.g1.x, self.g1.y, self.g1.z) * vec3(other.g0.y), vec2(self.g1.w) * vec2(0.0, other.g0.y), vec3(0.0), vec3(0.0), self.g0 * vec4(other.g0.x), vec4(self.g0.w) * vec4(0.0, 0.0, 0.0, -other.g0.y), vec3(0.0), vec3(self.g0.x, self.g0.y, self.g0.z) * vec3(other.g0.y), vec3(self.g1.x, self.g1.y, self.g1.z) * vec3(other.g0.x), vec2(self.g1.w) * vec2(0.0, other.g0.x));
}

MultiVector flector__wedge_dot__flat_point(Flector self, FlatPoint other) {
    return MultiVector(vec2(self.g0.w) * vec2(other.g0.w, 0.0), vec3(0.0), vec2(0.0), vec3(0.0), vec3(self.g1.x, self.g1.y, self.g1.z) * vec3(other.g0.w), self.g0.xyzx * vec4(-other.g0.w, -other.g0.w, -other.g0.w, 0.0) + vec4(self.g0.w) * vec4(other.g0.x, other.g0.y, other.g0.z, 0.0) + vec4(self.g1.x) * vec4(0.0, other.g0.z, -other.g0.y, 0.0) + vec4(self.g1.y) * vec4(-other.g0.z, 0.0, other.g0.x, 0.0) + vec4(self.g1.z) * vec4(other.g0.y, -other.g0.x, 0.0, 0.0), vec4(0.0), vec3(0.0), vec3(0.0), vec3(0.0), vec2(self.g1.x) * vec2(0.0, -other.g0.x) + vec2(self.g1.y) * vec2(0.0, -other.g0.y) + vec2(self.g1.z) * vec2(0.0, -other.g0.z) + vec2(self.g1.w) * vec2(0.0, -other.g0.w));
}

MultiVector flector__wedge_dot__flector(Flector self, Flector other) {
    return MultiVector(vec2(self.g0.w) * vec2(other.g0.w, 0.0) + vec2(self.g1.x) * vec2(-other.g1.x, 0.0) + vec2(self.g1.y) * vec2(-other.g1.y, 0.0) + vec2(self.g1.z) * vec2(-other.g1.z, 0.0), vec3(0.0), vec2(0.0), vec3(0.0), vec3(self.g0.w) * vec3(other.g1.x, other.g1.y, other.g1.z) + vec3(self.g1.x) * vec3(other.g0.w, other.g1.z, -other.g1.y) + vec3(self.g1.y) * vec3(-other.g1.z, other.g0.w, other.g1.x) + vec3(self.g1.z) * vec3(other.g1.y, -other.g1.x, other.g0.w), vec4(self.g0.x) * vec4(-other.g0.w, -other.g1.z, other.g1.y, 0.0) + vec4(self.g0.y) * vec4(other.g1.z, -other.g0.w, -other.g1.x, 0.0) + vec4(self.g0.z) * vec4(-other.g1.y, other.g1.x, -other.g0.w, 0.0) + vec4(self.g0.w) * vec4(other.g0.x, other.g0.y, other.g0.z, 0.0) + vec4(self.g1.x) * vec4(other.g1.w, other.g0.z, -other.g0.y, 0.0) + vec4(self.g1.y) * vec4(-other.g0.z, other.g1.w, other.g0.x, 0.0) + vec4(self.g1.z) * vec4(other.g0.y, -other.g0.x, other.g1.w, 0.0) + vec4(self.g1.w) * vec4(-other.g1.x, -other.g1.y, -other.g1.z, 0.0), vec4(0.0), vec3(0.0), vec3(0.0), vec3(0.0), vec2(self.g0.x) * vec2(0.0, other.g1.x) + vec2(self.g0.y) * vec2(0.0, other.g1.y) + vec2(self.g0.z) * vec2(0.0, other.g1.z) + vec2(self.g0.w) * vec2(0.0, other.g1.w) + vec2(self.g1.x) * vec2(0.0, -other.g0.x) + vec2(self.g1.y) * vec2(0.0, -other.g0.y) + vec2(self.g1.z) * vec2(0.0, -other.g0.z) + vec2(self.g1.w) * vec2(0.0, -other.g0.w));
}

MultiVector flector__wedge_dot__line(Flector self, Line other) {
    return MultiVector(vec2(0.0), vec3(0.0) - vec3(self.g0.w) * other.g0 + vec3(self.g1.x) * vec3(0.0, -other.g0.z, other.g0.y) + vec3(self.g1.y) * vec3(other.g0.z, 0.0, -other.g0.x) + vec3(self.g1.z) * vec3(-other.g0.y, other.g0.x, 0.0), vec2(self.g0.x) * vec2(0.0, -other.g0.x) + vec2(self.g0.y) * vec2(0.0, -other.g0.y) + vec2(self.g0.z) * vec2(0.0, -other.g0.z) + vec2(self.g1.x) * vec2(0.0, -other.g1.x) + vec2(self.g1.y) * vec2(0.0, -other.g1.y) + vec2(self.g1.z) * vec2(0.0, -other.g1.z), vec3(0.0), vec3(0.0), vec4(0.0), vec4(self.g1.x) * vec4(0.0, 0.0, 0.0, other.g0.x) + vec4(self.g1.y) * vec4(0.0, 0.0, 0.0, other.g0.y) + vec4(self.g1.z) * vec4(0.0, 0.0, 0.0, other.g0.z), vec3(0.0), vec3(self.g0.x) * vec3(0.0, other.g0.z, -other.g0.y) + vec3(self.g0.y) * vec3(-other.g0.z, 0.0, other.g0.x) + vec3(self.g0.z) * vec3(other.g0.y, -other.g0.x, 0.0) + vec3(self.g0.w) * other.g1 + vec3(self.g1.x) * vec3(0.0, other.g1.z, -other.g1.y) + vec3(self.g1.y) * vec3(-other.g1.z, 0.0, other.g1.x) + vec3(self.g1.z) * vec3(other.g1.y, -other.g1.x, 0.0) + vec3(self.g1.w) * other.g0, vec3(0.0), vec2(0.0));
}

MultiVector flector__wedge_dot__motor(Flector self, Motor other) {
    return MultiVector(vec2(0.0), vec3(0.0) - vec3(self.g0.w) * vec3(other.g0.x, other.g0.y, other.g0.z) + vec3(self.g1.x) * vec3(-other.g0.w, -other.g0.z, other.g0.y) + vec3(self.g1.y) * vec3(other.g0.z, -other.g0.w, -other.g0.x) + vec3(self.g1.z) * vec3(-other.g0.y, other.g0.x, -other.g0.w), vec2(self.g0.x) * vec2(0.0, -other.g0.x) + vec2(self.g0.y) * vec2(0.0, -other.g0.y) + vec2(self.g0.z) * vec2(0.0, -other.g0.z) + vec2(self.g1.x) * vec2(0.0, -other.g1.x) + vec2(self.g1.y) * vec2(0.0, -other.g1.y) + vec2(self.g1.z) * vec2(0.0, -other.g1.z) + vec2(self.g1.w) * vec2(0.0, other.g0.w), vec3(0.0), vec3(0.0), vec4(0.0), vec4(self.g0.w) * vec4(0.0, 0.0, 0.0, -other.g0.w) + vec4(self.g1.x) * vec4(0.0, 0.0, 0.0, other.g0.x) + vec4(self.g1.y) * vec4(0.0, 0.0, 0.0, other.g0.y) + vec4(self.g1.z) * vec4(0.0, 0.0, 0.0, other.g0.z), vec3(0.0), vec3(self.g0.x) * vec3(other.g0.w, other.g0.z, -other.g0.y) + vec3(self.g0.y) * vec3(-other.g0.z, other.g0.w, other.g0.x) + vec3(self.g0.z) * vec3(other.g0.y, -other.g0.x, other.g0.w) + vec3(self.g0.w) * other.g1 + vec3(self.g1.x) * vec3(0.0, other.g1.z, -other.g1.y) + vec3(self.g1.y) * vec3(-other.g1.z, 0.0, other.g1.x) + vec3(self.g1.z) * vec3(other.g1.y, -other.g1.x, 0.0) + vec3(self.g1.w) * vec3(other.g0.x, other.g0.y, other.g0.z), vec3(0.0), vec2(0.0));
}

MultiVector flector__wedge_dot__multi_vector(Flector self, MultiVector other) {
    return MultiVector(vec2(0.0) - vec2(self.g0.x) * vec2(other.g3.x, other.g6.x) - vec2(self.g0.y) * vec2(other.g3.y, other.g6.y) - vec2(self.g0.z) * vec2(other.g3.z, other.g6.z) + vec2(self.g0.w) * vec2(other.g5.w, -other.g6.w) + vec2(self.g1.x) * vec2(-other.g9.x, other.g1.x) + vec2(self.g1.y) * vec2(-other.g9.y, other.g1.y) + vec2(self.g1.z) * vec2(-other.g9.z, other.g1.z) + vec2(self.g1.w) * vec2(other.g10.x, other.g2.x), vec3(self.g0.x) * vec3(-other.g2.x, -other.g6.z, other.g6.y) + vec3(self.g0.y) * vec3(other.g6.z, -other.g2.x, -other.g6.x) + vec3(self.g0.z) * vec3(-other.g6.y, other.g6.x, -other.g2.x) - vec3(self.g0.w) * other.g7 + vec3(self.g1.x) * vec3(-other.g0.y, -other.g7.z, other.g7.y) + vec3(self.g1.y) * vec3(other.g7.z, -other.g0.y, -other.g7.x) + vec3(self.g1.z) * vec3(-other.g7.y, other.g7.x, -other.g0.y) - vec3(self.g1.w) * vec3(other.g6.x, other.g6.y, other.g6.z), vec2(self.g0.x) * vec2(0.0, -other.g7.x) + vec2(self.g0.x) * vec2(0.0, -other.g1.x) + vec2(self.g0.y) * vec2(0.0, -other.g7.y) + vec2(self.g0.y) * vec2(0.0, -other.g1.y) + vec2(self.g0.z) * vec2(0.0, -other.g7.z) + vec2(self.g0.z) * vec2(0.0, -other.g1.z) + vec2(self.g0.w) * other.g2 * vec2(-1.0, 1.0) + vec2(self.g1.x) * vec2(other.g6.x, -other.g8.x) + vec2(self.g1.y) * vec2(other.g6.y, -other.g8.y) + vec2(self.g1.z) * vec2(other.g6.z, -other.g8.z) + vec2(self.g1.w) * vec2(0.0, other.g6.w) + vec2(self.g1.w) * vec2(0.0, other.g0.y), vec3(0.0) - vec3(self.g0.w) * other.g3 + vec3(self.g1.x) * vec3(-other.g10.x, -other.g3.z, other.g3.y) + vec3(self.g1.y) * vec3(other.g3.z, -other.g10.x, -other.g3.x) + vec3(self.g1.z) * vec3(-other.g3.y, other.g3.x, -other.g10.x), vec3(self.g0.x) * vec3(other.g10.x, other.g3.z, -other.g3.y) + vec3(self.g0.y) * vec3(-other.g3.z, other.g10.x, other.g3.x) + vec3(self.g0.z) * vec3(other.g3.y, -other.g3.x, other.g10.x) + vec3(self.g0.w) * other.g9 + vec3(self.g1.x) * vec3(other.g5.w, other.g9.z, -other.g9.y) + vec3(self.g1.y) * vec3(-other.g9.z, other.g5.w, other.g9.x) + vec3(self.g1.z) * vec3(other.g9.y, -other.g9.x, other.g5.w) + vec3(self.g1.w) * other.g3, vec4(self.g0.x) * vec4(-other.g5.w, -other.g9.z, other.g9.y, -other.g3.x) + vec4(self.g0.x) * vec4(other.g0.x, other.g4.z, -other.g4.y, 0.0) + vec4(self.g0.y) * vec4(other.g9.z, -other.g5.w, -other.g9.x, -other.g3.y) + vec4(self.g0.y) * vec4(-other.g4.z, other.g0.x, other.g4.x, 0.0) + vec4(self.g0.z) * vec4(-other.g9.y, other.g9.x, -other.g5.w, -other.g3.z) + vec4(self.g0.z) * vec4(other.g4.y, -other.g4.x, other.g0.x, 0.0) + vec4(self.g0.w) * vec4(other.g5.x, other.g5.y, other.g5.z, other.g0.x) + vec4(self.g1.x) * vec4(other.g10.y, other.g5.z, -other.g5.y, -other.g4.x) + vec4(self.g1.y) * vec4(-other.g5.z, other.g10.y, other.g5.x, -other.g4.y) + vec4(self.g1.z) * vec4(other.g5.y, -other.g5.x, other.g10.y, -other.g4.z) + vec4(self.g1.w) * vec4(-other.g9.x, -other.g9.y, -other.g9.z, other.g10.x) + vec4(self.g1.w) * vec4(other.g4.x, other.g4.y, other.g4.z, 0.0), vec4(self.g0.x) * vec4(0.0, 0.0, 0.0, other.g6.x) + vec4(self.g0.y) * vec4(0.0, 0.0, 0.0, other.g6.y) + vec4(self.g0.z) * vec4(0.0, 0.0, 0.0, other.g6.z) - vec4(self.g0.w) * vec4(other.g6.x, other.g6.y, other.g6.z, other.g0.y) + vec4(self.g1.x) * vec4(-other.g2.x, -other.g6.z, other.g6.y, other.g7.x) + vec4(self.g1.y) * vec4(other.g6.z, -other.g2.x, -other.g6.x, other.g7.y) + vec4(self.g1.z) * vec4(-other.g6.y, other.g6.x, -other.g2.x, other.g7.z) + vec4(self.g1.w) * vec4(0.0, 0.0, 0.0, -other.g2.x), vec3(self.g0.x) * vec3(other.g2.x, other.g6.z, -other.g6.y) + vec3(self.g0.y) * vec3(-other.g6.z, other.g2.x, other.g6.x) + vec3(self.g0.z) * vec3(other.g6.y, -other.g6.x, other.g2.x) - vec3(self.g0.w) * other.g1 + vec3(self.g1.x) * vec3(-other.g6.w, -other.g1.z, other.g1.y) + vec3(self.g1.y) * vec3(other.g1.z, -other.g6.w, -other.g1.x) + vec3(self.g1.z) * vec3(-other.g1.y, other.g1.x, -other.g6.w) + vec3(self.g1.w) * vec3(other.g6.x, other.g6.y, other.g6.z), vec3(self.g0.x) * vec3(other.g6.w, other.g7.z, -other.g7.y) + vec3(self.g0.x) * vec3(other.g0.y, other.g1.z, -other.g1.y) + vec3(self.g0.y) * vec3(-other.g7.z, other.g6.w, other.g7.x) + vec3(self.g0.y) * vec3(-other.g1.z, other.g0.y, other.g1.x) + vec3(self.g0.z) * vec3(other.g7.y, -other.g7.x, other.g6.w) + vec3(self.g0.z) * vec3(other.g1.y, -other.g1.x, other.g0.y) + vec3(self.g0.w) * other.g8 + vec3(self.g1.x) * vec3(other.g2.y, other.g8.z, -other.g8.y) + vec3(self.g1.y) * vec3(-other.g8.z, other.g2.y, other.g8.x) + vec3(self.g1.z) * vec3(other.g8.y, -other.g8.x, other.g2.y) + vec3(self.g1.w) * other.g7 + vec3(self.g1.w) * other.g1, vec3(self.g0.x) * vec3(other.g10.x, other.g3.z, -other.g3.y) + vec3(self.g0.y) * vec3(-other.g3.z, other.g10.x, other.g3.x) + vec3(self.g0.z) * vec3(other.g3.y, -other.g3.x, other.g10.x) + vec3(self.g0.w) * other.g4 + vec3(self.g1.x) * vec3(other.g0.x, other.g4.z, -other.g4.y) + vec3(self.g1.y) * vec3(-other.g4.z, other.g0.x, other.g4.x) + vec3(self.g1.z) * vec3(other.g4.y, -other.g4.x, other.g0.x) + vec3(self.g1.w) * other.g3, vec2(self.g0.x) * vec2(0.0, other.g9.x) + vec2(self.g0.x) * vec2(0.0, -other.g4.x) + vec2(self.g0.y) * vec2(0.0, other.g9.y) + vec2(self.g0.y) * vec2(0.0, -other.g4.y) + vec2(self.g0.z) * vec2(0.0, other.g9.z) + vec2(self.g0.z) * vec2(0.0, -other.g4.z) + vec2(self.g0.w) * other.g10 * vec2(-1.0, 1.0) + vec2(self.g1.x) * vec2(other.g3.x, -other.g5.x) + vec2(self.g1.y) * vec2(other.g3.y, -other.g5.y) + vec2(self.g1.z) * vec2(other.g3.z, -other.g5.z) + vec2(self.g1.w) * vec2(0.0, -other.g5.w) + vec2(self.g1.w) * vec2(0.0, other.g0.x));
}

MultiVector flector__wedge_dot__plane(Flector self, Plane other) {
    return MultiVector(vec2(self.g1.x) * vec2(-other.g0.x, 0.0) + vec2(self.g1.y) * vec2(-other.g0.y, 0.0) + vec2(self.g1.z) * vec2(-other.g0.z, 0.0), vec3(0.0), vec2(0.0), vec3(0.0), vec3(self.g0.w) * vec3(other.g0.x, other.g0.y, other.g0.z) + vec3(self.g1.x) * vec3(0.0, other.g0.z, -other.g0.y) + vec3(self.g1.y) * vec3(-other.g0.z, 0.0, other.g0.x) + vec3(self.g1.z) * vec3(other.g0.y, -other.g0.x, 0.0), vec4(self.g0.x) * vec4(0.0, -other.g0.z, other.g0.y, 0.0) + vec4(self.g0.y) * vec4(other.g0.z, 0.0, -other.g0.x, 0.0) + vec4(self.g0.z) * vec4(-other.g0.y, other.g0.x, 0.0, 0.0) + self.g1.xyzx * vec4(other.g0.w, other.g0.w, other.g0.w, 0.0) + vec4(self.g1.w) * vec4(-other.g0.x, -other.g0.y, -other.g0.z, 0.0), vec4(0.0), vec3(0.0), vec3(0.0), vec3(0.0), vec2(self.g0.x) * vec2(0.0, other.g0.x) + vec2(self.g0.y) * vec2(0.0, other.g0.y) + vec2(self.g0.z) * vec2(0.0, other.g0.z) + vec2(self.g0.w) * vec2(0.0, other.g0.w));
}

MultiVector flector__wedge_dot__round_point(Flector self, RoundPoint other) {
    return MultiVector(vec2(self.g1.x) * vec2(0.0, other.g0.x) + vec2(self.g1.y) * vec2(0.0, other.g0.y) + vec2(self.g1.z) * vec2(0.0, other.g0.z) + vec2(self.g1.w) * vec2(0.0, other.g1.x), vec3(0.0) - vec3(self.g0.x, self.g0.y, self.g0.z) * vec3(other.g1.x), vec2(self.g0.x) * vec2(0.0, -other.g0.x) + vec2(self.g0.y) * vec2(0.0, -other.g0.y) + vec2(self.g0.z) * vec2(0.0, -other.g0.z) + vec2(self.g0.w) * other.g1 * vec2(-1.0, 1.0), vec3(0.0), vec3(0.0), vec4(0.0), vec4(0.0) - self.g1 * vec4(other.g1.x), vec3(self.g0.x, self.g0.y, self.g0.z) * vec3(other.g1.x) - vec3(self.g0.w) * other.g0 + vec3(self.g1.x) * vec3(0.0, -other.g0.z, other.g0.y) + vec3(self.g1.y) * vec3(other.g0.z, 0.0, -other.g0.x) + vec3(self.g1.z) * vec3(-other.g0.y, other.g0.x, 0.0), vec3(self.g0.x) * vec3(0.0, other.g0.z, -other.g0.y) + vec3(self.g0.y) * vec3(-other.g0.z, 0.0, other.g0.x) + vec3(self.g0.z) * vec3(other.g0.y, -other.g0.x, 0.0) + vec3(self.g1.x, self.g1.y, self.g1.z) * vec3(other.g1.y) + vec3(self.g1.w) * other.g0, vec3(0.0), vec2(0.0));
}

Flector flector__wedge_dot__scalar(Flector self, Scalar other) {
    return Flector(self.g0 * vec4(other.g0), self.g1 * vec4(other.g0));
}

MultiVector flector__wedge_dot__sphere(Flector self, Sphere other) {
    return MultiVector(vec2(self.g1.x) * vec2(-other.g0.x, 0.0) + vec2(self.g1.y) * vec2(-other.g0.y, 0.0) + vec2(self.g1.z) * vec2(-other.g0.z, 0.0) + vec2(self.g1.w) * vec2(other.g1.x, 0.0), vec3(0.0), vec2(0.0), vec3(0.0) - vec3(self.g1.x, self.g1.y, self.g1.z) * vec3(other.g1.x), vec3(self.g0.x, self.g0.y, self.g0.z) * vec3(other.g1.x) + vec3(self.g0.w) * other.g0 + vec3(self.g1.x) * vec3(0.0, other.g0.z, -other.g0.y) + vec3(self.g1.y) * vec3(-other.g0.z, 0.0, other.g0.x) + vec3(self.g1.z) * vec3(other.g0.y, -other.g0.x, 0.0), vec4(self.g0.x) * vec4(0.0, -other.g0.z, other.g0.y, 0.0) + vec4(self.g0.y) * vec4(other.g0.z, 0.0, -other.g0.x, 0.0) + vec4(self.g0.z) * vec4(-other.g0.y, other.g0.x, 0.0, 0.0) + self.g1.xyzx * vec4(other.g1.y, other.g1.y, other.g1.y, 0.0) + vec4(self.g1.w) * vec4(-other.g0.x, -other.g0.y, -other.g0.z, other.g1.x), vec4(0.0), vec3(0.0), vec3(0.0), vec3(self.g0.x, self.g0.y, self.g0.z) * vec3(other.g1.x), vec2(self.g0.x) * vec2(0.0, other.g0.x) + vec2(self.g0.y) * vec2(0.0, other.g0.y) + vec2(self.g0.z) * vec2(0.0, other.g0.z) + vec2(self.g0.w) * other.g1 * vec2(-1.0, 1.0));
}

Dipole line__wedge_dot__anti_scalar(Line self, AntiScalar other) {
    return Dipole(vec3(0.0), vec3(0.0) - self.g0 * vec3(other.g0), vec4(self.g1.x, self.g1.y, self.g1.z, self.g1.x) * vec4(-other.g0, -other.g0, -other.g0, 0.0));
}

MultiVector line__wedge_dot__circle(Line self, Circle other) {
    return MultiVector(vec2(self.g0.x) * vec2(other.g1.x, 0.0) + vec2(self.g0.y) * vec2(other.g1.y, 0.0) + vec2(self.g0.z) * vec2(other.g1.z, 0.0) + vec2(self.g1.x) * vec2(other.g0.x, 0.0) + vec2(self.g1.y) * vec2(other.g0.y, 0.0) + vec2(self.g1.z) * vec2(other.g0.z, 0.0), vec3(0.0), vec2(0.0), vec3(self.g0.x) * vec3(0.0, -other.g0.z, other.g0.y) + vec3(self.g0.y) * vec3(other.g0.z, 0.0, -other.g0.x) + vec3(self.g0.z) * vec3(-other.g0.y, other.g0.x, 0.0), vec3(self.g0.x) * vec3(0.0, -other.g1.z, other.g1.y) + vec3(self.g0.y) * vec3(other.g1.z, 0.0, -other.g1.x) + vec3(self.g0.z) * vec3(-other.g1.y, other.g1.x, 0.0) + vec3(self.g1.x) * vec3(0.0, -other.g0.z, other.g0.y) + vec3(self.g1.y) * vec3(other.g0.z, 0.0, -other.g0.x) + vec3(self.g1.z) * vec3(-other.g0.y, other.g0.x, 0.0), vec4(self.g0.x) * vec4(0.0, -other.g2.z, other.g2.y, 0.0) + vec4(self.g0.y) * vec4(other.g2.z, 0.0, -other.g2.x, 0.0) + vec4(self.g0.z) * vec4(-other.g2.y, other.g2.x, 0.0, 0.0) + vec4(self.g1.x) * vec4(-other.g0.w, -other.g1.z, other.g1.y, other.g0.x) + vec4(self.g1.y) * vec4(other.g1.z, -other.g0.w, -other.g1.x, other.g0.y) + vec4(self.g1.z) * vec4(-other.g1.y, other.g1.x, -other.g0.w, other.g0.z), vec4(0.0), vec3(0.0), vec3(0.0), self.g0 * vec3(other.g0.w) + vec3(self.g1.x) * vec3(0.0, -other.g0.z, other.g0.y) + vec3(self.g1.y) * vec3(other.g0.z, 0.0, -other.g0.x) + vec3(self.g1.z) * vec3(-other.g0.y, other.g0.x, 0.0), vec2(self.g0.x) * vec2(other.g0.x, other.g2.x) + vec2(self.g0.y) * vec2(other.g0.y, other.g2.y) + vec2(self.g0.z) * vec2(other.g0.z, other.g2.z) + vec2(self.g1.x) * vec2(0.0, other.g1.x) + vec2(self.g1.y) * vec2(0.0, other.g1.y) + vec2(self.g1.z) * vec2(0.0, other.g1.z));
}

MultiVector line__wedge_dot__dipole(Line self, Dipole other) {
    return MultiVector(vec2(self.g0.x) * vec2(0.0, -other.g1.x) + vec2(self.g0.y) * vec2(0.0, -other.g1.y) + vec2(self.g0.z) * vec2(0.0, -other.g1.z) + vec2(self.g1.x) * vec2(0.0, -other.g0.x) + vec2(self.g1.y) * vec2(0.0, -other.g0.y) + vec2(self.g1.z) * vec2(0.0, -other.g0.z), vec3(0.0) - self.g0 * vec3(other.g2.w) + vec3(self.g1.x) * vec3(0.0, -other.g0.z, other.g0.y) + vec3(self.g1.y) * vec3(other.g0.z, 0.0, -other.g0.x) + vec3(self.g1.z) * vec3(-other.g0.y, other.g0.x, 0.0), vec2(0.0) - vec2(self.g0.x) * vec2(other.g0.x, other.g2.x) - vec2(self.g0.y) * vec2(other.g0.y, other.g2.y) - vec2(self.g0.z) * vec2(other.g0.z, other.g2.z) + vec2(self.g1.x) * vec2(0.0, -other.g1.x) + vec2(self.g1.y) * vec2(0.0, -other.g1.y) + vec2(self.g1.z) * vec2(0.0, -other.g1.z), vec3(0.0), vec3(0.0), vec4(0.0), vec4(self.g0.x) * vec4(0.0, other.g0.z, -other.g0.y, 0.0) + vec4(self.g0.y) * vec4(-other.g0.z, 0.0, other.g0.x, 0.0) + vec4(self.g0.z) * vec4(other.g0.y, -other.g0.x, 0.0, 0.0) + vec4(self.g1.x) * vec4(0.0, 0.0, 0.0, other.g0.x) + vec4(self.g1.y) * vec4(0.0, 0.0, 0.0, other.g0.y) + vec4(self.g1.z) * vec4(0.0, 0.0, 0.0, other.g0.z), vec3(self.g0.x) * vec3(0.0, other.g1.z, -other.g1.y) + vec3(self.g0.y) * vec3(-other.g1.z, 0.0, other.g1.x) + vec3(self.g0.z) * vec3(other.g1.y, -other.g1.x, 0.0) + vec3(self.g1.x) * vec3(0.0, other.g0.z, -other.g0.y) + vec3(self.g1.y) * vec3(-other.g0.z, 0.0, other.g0.x) + vec3(self.g1.z) * vec3(other.g0.y, -other.g0.x, 0.0), vec3(self.g0.x) * vec3(0.0, other.g2.z, -other.g2.y) + vec3(self.g0.y) * vec3(-other.g2.z, 0.0, other.g2.x) + vec3(self.g0.z) * vec3(other.g2.y, -other.g2.x, 0.0) + vec3(self.g1.x) * vec3(-other.g2.w, other.g1.z, -other.g1.y) + vec3(self.g1.y) * vec3(-other.g1.z, -other.g2.w, other.g1.x) + vec3(self.g1.z) * vec3(other.g1.y, -other.g1.x, -other.g2.w), vec3(0.0), vec2(0.0));
}

MultiVector line__wedge_dot__dual_num(Line self, DualNum other) {
    return MultiVector(vec2(0.0), vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0) - self.g0 * vec3(other.g0.y), vec4(self.g1.x, self.g1.y, self.g1.z, self.g1.x) * vec4(-other.g0.y, -other.g0.y, -other.g0.y, 0.0), vec4(0.0), self.g0 * vec3(other.g0.x), self.g1 * vec3(other.g0.x), vec3(0.0), vec2(0.0));
}

MultiVector line__wedge_dot__flat_point(Line self, FlatPoint other) {
    return MultiVector(vec2(0.0), vec3(0.0) - self.g0 * vec3(other.g0.w), vec2(self.g0.x) * vec2(0.0, -other.g0.x) + vec2(self.g0.y) * vec2(0.0, -other.g0.y) + vec2(self.g0.z) * vec2(0.0, -other.g0.z), vec3(0.0), vec3(0.0), vec4(0.0), vec4(0.0), vec3(0.0), vec3(self.g0.x) * vec3(0.0, other.g0.z, -other.g0.y) + vec3(self.g0.y) * vec3(-other.g0.z, 0.0, other.g0.x) + vec3(self.g0.z) * vec3(other.g0.y, -other.g0.x, 0.0) - self.g1 * vec3(other.g0.w), vec3(0.0), vec2(0.0));
}

MultiVector line__wedge_dot__flector(Line self, Flector other) {
    return MultiVector(vec2(0.0), vec3(self.g0.x) * vec3(-other.g0.w, -other.g1.z, other.g1.y) + vec3(self.g0.y) * vec3(other.g1.z, -other.g0.w, -other.g1.x) + vec3(self.g0.z) * vec3(-other.g1.y, other.g1.x, -other.g0.w), vec2(self.g0.x) * vec2(0.0, -other.g0.x) + vec2(self.g0.y) * vec2(0.0, -other.g0.y) + vec2(self.g0.z) * vec2(0.0, -other.g0.z) + vec2(self.g1.x) * vec2(0.0, other.g1.x) + vec2(self.g1.y) * vec2(0.0, other.g1.y) + vec2(self.g1.z) * vec2(0.0, other.g1.z), vec3(0.0), vec3(0.0), vec4(0.0), vec4(self.g0.x) * vec4(0.0, 0.0, 0.0, other.g1.x) + vec4(self.g0.y) * vec4(0.0, 0.0, 0.0, other.g1.y) + vec4(self.g0.z) * vec4(0.0, 0.0, 0.0, other.g1.z), vec3(0.0), vec3(self.g0.x) * vec3(other.g1.w, other.g0.z, -other.g0.y) + vec3(self.g0.y) * vec3(-other.g0.z, other.g1.w, other.g0.x) + vec3(self.g0.z) * vec3(other.g0.y, -other.g0.x, other.g1.w) + vec3(self.g1.x) * vec3(-other.g0.w, -other.g1.z, other.g1.y) + vec3(self.g1.y) * vec3(other.g1.z, -other.g0.w, -other.g1.x) + vec3(self.g1.z) * vec3(-other.g1.y, other.g1.x, -other.g0.w), vec3(0.0), vec2(0.0));
}

MultiVector line__wedge_dot__line(Line self, Line other) {
    return MultiVector(vec2(self.g0.x) * vec2(other.g0.x, 0.0) + vec2(self.g0.y) * vec2(other.g0.y, 0.0) + vec2(self.g0.z) * vec2(other.g0.z, 0.0), vec3(0.0), vec2(0.0), vec3(0.0), vec3(self.g0.x) * vec3(0.0, -other.g0.z, other.g0.y) + vec3(self.g0.y) * vec3(other.g0.z, 0.0, -other.g0.x) + vec3(self.g0.z) * vec3(-other.g0.y, other.g0.x, 0.0), vec4(self.g0.x) * vec4(0.0, -other.g1.z, other.g1.y, 0.0) + vec4(self.g0.y) * vec4(other.g1.z, 0.0, -other.g1.x, 0.0) + vec4(self.g0.z) * vec4(-other.g1.y, other.g1.x, 0.0, 0.0) + vec4(self.g1.x) * vec4(0.0, -other.g0.z, other.g0.y, 0.0) + vec4(self.g1.y) * vec4(other.g0.z, 0.0, -other.g0.x, 0.0) + vec4(self.g1.z) * vec4(-other.g0.y, other.g0.x, 0.0, 0.0), vec4(0.0), vec3(0.0), vec3(0.0), vec3(0.0), vec2(self.g0.x) * vec2(0.0, other.g1.x) + vec2(self.g0.y) * vec2(0.0, other.g1.y) + vec2(self.g0.z) * vec2(0.0, other.g1.z) + vec2(self.g1.x) * vec2(0.0, other.g0.x) + vec2(self.g1.y) * vec2(0.0, other.g0.y) + vec2(self.g1.z) * vec2(0.0, other.g0.z));
}

MultiVector line__wedge_dot__motor(Line self, Motor other) {
    return MultiVector(vec2(self.g0.x) * vec2(other.g0.x, 0.0) + vec2(self.g0.y) * vec2(other.g0.y, 0.0) + vec2(self.g0.z) * vec2(other.g0.z, 0.0), vec3(0.0), vec2(0.0), vec3(0.0), vec3(self.g0.x) * vec3(-other.g0.w, -other.g0.z, other.g0.y) + vec3(self.g0.y) * vec3(other.g0.z, -other.g0.w, -other.g0.x) + vec3(self.g0.z) * vec3(-other.g0.y, other.g0.x, -other.g0.w), vec4(self.g0.x) * vec4(0.0, -other.g1.z, other.g1.y, 0.0) + vec4(self.g0.y) * vec4(other.g1.z, 0.0, -other.g1.x, 0.0) + vec4(self.g0.z) * vec4(-other.g1.y, other.g1.x, 0.0, 0.0) + vec4(self.g1.x) * vec4(-other.g0.w, -other.g0.z, other.g0.y, 0.0) + vec4(self.g1.y) * vec4(other.g0.z, -other.g0.w, -other.g0.x, 0.0) + vec4(self.g1.z) * vec4(-other.g0.y, other.g0.x, -other.g0.w, 0.0), vec4(0.0), vec3(0.0), vec3(0.0), vec3(0.0), vec2(self.g0.x) * vec2(0.0, other.g1.x) + vec2(self.g0.y) * vec2(0.0, other.g1.y) + vec2(self.g0.z) * vec2(0.0, other.g1.z) + vec2(self.g1.x) * vec2(0.0, other.g0.x) + vec2(self.g1.y) * vec2(0.0, other.g0.y) + vec2(self.g1.z) * vec2(0.0, other.g0.z));
}

MultiVector line__wedge_dot__multi_vector(Line self, MultiVector other) {
    return MultiVector(vec2(self.g0.x) * vec2(other.g7.x, -other.g4.x) + vec2(self.g0.y) * vec2(other.g7.y, -other.g4.y) + vec2(self.g0.z) * vec2(other.g7.z, -other.g4.z) + vec2(self.g1.x) * vec2(other.g6.x, -other.g3.x) + vec2(self.g1.y) * vec2(other.g6.y, -other.g3.y) + vec2(self.g1.z) * vec2(other.g6.z, -other.g3.z), vec3(self.g0.x) * vec3(-other.g5.w, -other.g9.z, other.g9.y) + vec3(self.g0.y) * vec3(other.g9.z, -other.g5.w, -other.g9.x) + vec3(self.g0.z) * vec3(-other.g9.y, other.g9.x, -other.g5.w) + vec3(self.g1.x) * vec3(-other.g10.x, -other.g3.z, other.g3.y) + vec3(self.g1.y) * vec3(other.g3.z, -other.g10.x, -other.g3.x) + vec3(self.g1.z) * vec3(-other.g3.y, other.g3.x, -other.g10.x), vec2(0.0) - vec2(self.g0.x) * vec2(other.g3.x, other.g5.x) - vec2(self.g0.y) * vec2(other.g3.y, other.g5.y) - vec2(self.g0.z) * vec2(other.g3.z, other.g5.z) + vec2(self.g1.x) * vec2(0.0, other.g9.x) + vec2(self.g1.x) * vec2(0.0, -other.g4.x) + vec2(self.g1.y) * vec2(0.0, other.g9.y) + vec2(self.g1.y) * vec2(0.0, -other.g4.y) + vec2(self.g1.z) * vec2(0.0, other.g9.z) + vec2(self.g1.z) * vec2(0.0, -other.g4.z), vec3(self.g0.x) * vec3(-other.g2.x, -other.g6.z, other.g6.y) + vec3(self.g0.y) * vec3(other.g6.z, -other.g2.x, -other.g6.x) + vec3(self.g0.z) * vec3(-other.g6.y, other.g6.x, -other.g2.x), vec3(self.g0.x) * vec3(-other.g0.y, -other.g7.z, other.g7.y) + vec3(self.g0.y) * vec3(other.g7.z, -other.g0.y, -other.g7.x) + vec3(self.g0.z) * vec3(-other.g7.y, other.g7.x, -other.g0.y) + vec3(self.g1.x) * vec3(-other.g2.x, -other.g6.z, other.g6.y) + vec3(self.g1.y) * vec3(other.g6.z, -other.g2.x, -other.g6.x) + vec3(self.g1.z) * vec3(-other.g6.y, other.g6.x, -other.g2.x), vec4(self.g0.x) * vec4(-other.g2.y, -other.g8.z, other.g8.y, -other.g1.x) + vec4(self.g0.y) * vec4(other.g8.z, -other.g2.y, -other.g8.x, -other.g1.y) + vec4(self.g0.z) * vec4(-other.g8.y, other.g8.x, -other.g2.y, -other.g1.z) + vec4(self.g1.x) * vec4(-other.g6.w, -other.g7.z, other.g7.y, other.g6.x) + vec4(self.g1.x) * vec4(-other.g0.y, -other.g1.z, other.g1.y, 0.0) + vec4(self.g1.y) * vec4(other.g7.z, -other.g6.w, -other.g7.x, other.g6.y) + vec4(self.g1.y) * vec4(other.g1.z, -other.g0.y, -other.g1.x, 0.0) + vec4(self.g1.z) * vec4(-other.g7.y, other.g7.x, -other.g6.w, other.g6.z) + vec4(self.g1.z) * vec4(-other.g1.y, other.g1.x, -other.g0.y, 0.0), vec4(self.g0.x) * vec4(other.g10.x, other.g3.z, -other.g3.y, other.g9.x) + vec4(self.g0.y) * vec4(-other.g3.z, other.g10.x, other.g3.x, other.g9.y) + vec4(self.g0.z) * vec4(other.g3.y, -other.g3.x, other.g10.x, other.g9.z) + vec4(self.g1.x) * vec4(0.0, 0.0, 0.0, other.g3.x) + vec4(self.g1.y) * vec4(0.0, 0.0, 0.0, other.g3.y) + vec4(self.g1.z) * vec4(0.0, 0.0, 0.0, other.g3.z), vec3(self.g0.x) * vec3(other.g0.x, other.g4.z, -other.g4.y) + vec3(self.g0.y) * vec3(-other.g4.z, other.g0.x, other.g4.x) + vec3(self.g0.z) * vec3(other.g4.y, -other.g4.x, other.g0.x) + vec3(self.g1.x) * vec3(other.g10.x, other.g3.z, -other.g3.y) + vec3(self.g1.y) * vec3(-other.g3.z, other.g10.x, other.g3.x) + vec3(self.g1.z) * vec3(other.g3.y, -other.g3.x, other.g10.x), vec3(self.g0.x) * vec3(other.g10.y, other.g5.z, -other.g5.y) + vec3(self.g0.y) * vec3(-other.g5.z, other.g10.y, other.g5.x) + vec3(self.g0.z) * vec3(other.g5.y, -other.g5.x, other.g10.y) + vec3(self.g1.x) * vec3(-other.g5.w, -other.g9.z, other.g9.y) + vec3(self.g1.x) * vec3(other.g0.x, other.g4.z, -other.g4.y) + vec3(self.g1.y) * vec3(other.g9.z, -other.g5.w, -other.g9.x) + vec3(self.g1.y) * vec3(-other.g4.z, other.g0.x, other.g4.x) + vec3(self.g1.z) * vec3(-other.g9.y, other.g9.x, -other.g5.w) + vec3(self.g1.z) * vec3(other.g4.y, -other.g4.x, other.g0.x), vec3(self.g0.x) * vec3(other.g6.w, other.g1.z, -other.g1.y) + vec3(self.g0.y) * vec3(-other.g1.z, other.g6.w, other.g1.x) + vec3(self.g0.z) * vec3(other.g1.y, -other.g1.x, other.g6.w) + vec3(self.g1.x) * vec3(-other.g2.x, -other.g6.z, other.g6.y) + vec3(self.g1.y) * vec3(other.g6.z, -other.g2.x, -other.g6.x) + vec3(self.g1.z) * vec3(-other.g6.y, other.g6.x, -other.g2.x), vec2(self.g0.x) * vec2(other.g6.x, other.g8.x) + vec2(self.g0.y) * vec2(other.g6.y, other.g8.y) + vec2(self.g0.z) * vec2(other.g6.z, other.g8.z) + vec2(self.g1.x) * vec2(0.0, other.g7.x) + vec2(self.g1.x) * vec2(0.0, other.g1.x) + vec2(self.g1.y) * vec2(0.0, other.g7.y) + vec2(self.g1.y) * vec2(0.0, other.g1.y) + vec2(self.g1.z) * vec2(0.0, other.g7.z) + vec2(self.g1.z) * vec2(0.0, other.g1.z));
}

MultiVector line__wedge_dot__plane(Line self, Plane other) {
    return MultiVector(vec2(0.0), vec3(self.g0.x) * vec3(0.0, -other.g0.z, other.g0.y) + vec3(self.g0.y) * vec3(other.g0.z, 0.0, -other.g0.x) + vec3(self.g0.z) * vec3(-other.g0.y, other.g0.x, 0.0), vec2(self.g1.x) * vec2(0.0, other.g0.x) + vec2(self.g1.y) * vec2(0.0, other.g0.y) + vec2(self.g1.z) * vec2(0.0, other.g0.z), vec3(0.0), vec3(0.0), vec4(0.0), vec4(self.g0.x) * vec4(0.0, 0.0, 0.0, other.g0.x) + vec4(self.g0.y) * vec4(0.0, 0.0, 0.0, other.g0.y) + vec4(self.g0.z) * vec4(0.0, 0.0, 0.0, other.g0.z), vec3(0.0), self.g0 * vec3(other.g0.w) + vec3(self.g1.x) * vec3(0.0, -other.g0.z, other.g0.y) + vec3(self.g1.y) * vec3(other.g0.z, 0.0, -other.g0.x) + vec3(self.g1.z) * vec3(-other.g0.y, other.g0.x, 0.0), vec3(0.0), vec2(0.0));
}

MultiVector line__wedge_dot__round_point(Line self, RoundPoint other) {
    return MultiVector(vec2(0.0), vec3(0.0), vec2(0.0), vec3(0.0) - self.g0 * vec3(other.g1.x), vec3(0.0) - self.g1 * vec3(other.g1.x), vec4(self.g0.x) * vec4(-other.g1.y, 0.0, 0.0, -other.g0.x) + vec4(self.g0.y) * vec4(0.0, -other.g1.y, 0.0, -other.g0.y) + vec4(self.g0.z) * vec4(0.0, 0.0, -other.g1.y, -other.g0.z) + vec4(self.g1.x) * vec4(0.0, -other.g0.z, other.g0.y, 0.0) + vec4(self.g1.y) * vec4(other.g0.z, 0.0, -other.g0.x, 0.0) + vec4(self.g1.z) * vec4(-other.g0.y, other.g0.x, 0.0, 0.0), vec4(0.0), vec3(0.0), vec3(0.0), vec3(self.g0.x) * vec3(0.0, other.g0.z, -other.g0.y) + vec3(self.g0.y) * vec3(-other.g0.z, 0.0, other.g0.x) + vec3(self.g0.z) * vec3(other.g0.y, -other.g0.x, 0.0) - self.g1 * vec3(other.g1.x), vec2(self.g1.x) * vec2(0.0, other.g0.x) + vec2(self.g1.y) * vec2(0.0, other.g0.y) + vec2(self.g1.z) * vec2(0.0, other.g0.z));
}

Line line__wedge_dot__scalar(Line self, Scalar other) {
    return Line(self.g0 * vec3(other.g0), self.g1 * vec3(other.g0));
}

MultiVector line__wedge_dot__sphere(Line self, Sphere other) {
    return MultiVector(vec2(0.0), vec3(self.g0.x) * vec3(0.0, -other.g0.z, other.g0.y) + vec3(self.g0.y) * vec3(other.g0.z, 0.0, -other.g0.x) + vec3(self.g0.z) * vec3(-other.g0.y, other.g0.x, 0.0) - self.g1 * vec3(other.g1.x), vec2(self.g1.x) * vec2(0.0, other.g0.x) + vec2(self.g1.y) * vec2(0.0, other.g0.y) + vec2(self.g1.z) * vec2(0.0, other.g0.z), vec3(0.0), vec3(0.0), vec4(0.0), vec4(self.g0.x) * vec4(other.g1.x, 0.0, 0.0, other.g0.x) + vec4(self.g0.y) * vec4(0.0, other.g1.x, 0.0, other.g0.y) + vec4(self.g0.z) * vec4(0.0, 0.0, other.g1.x, other.g0.z), self.g1 * vec3(other.g1.x), self.g0 * vec3(other.g1.y) + vec3(self.g1.x) * vec3(0.0, -other.g0.z, other.g0.y) + vec3(self.g1.y) * vec3(other.g0.z, 0.0, -other.g0.x) + vec3(self.g1.z) * vec3(-other.g0.y, other.g0.x, 0.0), vec3(0.0), vec2(0.0));
}

MultiVector motor__wedge_dot__anti_scalar(Motor self, AntiScalar other) {
    return MultiVector(vec2(self.g0.w) * vec2(-other.g0, 0.0), vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0) - vec3(self.g0.x, self.g0.y, self.g0.z) * vec3(other.g0), vec4(self.g1.x, self.g1.y, self.g1.z, self.g1.x) * vec4(-other.g0, -other.g0, -other.g0, 0.0), vec4(0.0), vec3(0.0), vec3(0.0), vec3(0.0), vec2(0.0));
}

MultiVector motor__wedge_dot__circle(Motor self, Circle other) {
    return MultiVector(vec2(self.g0.x) * vec2(other.g1.x, 0.0) + vec2(self.g0.y) * vec2(other.g1.y, 0.0) + vec2(self.g0.z) * vec2(other.g1.z, 0.0) + vec2(self.g1.x) * vec2(other.g0.x, 0.0) + vec2(self.g1.y) * vec2(other.g0.y, 0.0) + vec2(self.g1.z) * vec2(other.g0.z, 0.0), vec3(0.0), vec2(0.0), vec3(self.g0.x) * vec3(0.0, -other.g0.z, other.g0.y) + vec3(self.g0.y) * vec3(other.g0.z, 0.0, -other.g0.x) + vec3(self.g0.z) * vec3(-other.g0.y, other.g0.x, 0.0) - vec3(self.g0.w) * vec3(other.g0.x, other.g0.y, other.g0.z), vec3(self.g0.x) * vec3(0.0, -other.g1.z, other.g1.y) + vec3(self.g0.y) * vec3(other.g1.z, 0.0, -other.g1.x) + vec3(self.g0.z) * vec3(-other.g1.y, other.g1.x, 0.0) - vec3(self.g0.w) * other.g1 + vec3(self.g1.x) * vec3(0.0, -other.g0.z, other.g0.y) + vec3(self.g1.y) * vec3(other.g0.z, 0.0, -other.g0.x) + vec3(self.g1.z) * vec3(-other.g0.y, other.g0.x, 0.0), vec4(self.g0.x) * vec4(0.0, -other.g2.z, other.g2.y, 0.0) + vec4(self.g0.y) * vec4(other.g2.z, 0.0, -other.g2.x, 0.0) + vec4(self.g0.z) * vec4(-other.g2.y, other.g2.x, 0.0, 0.0) + vec4(self.g0.w) * vec4(-other.g2.x, -other.g2.y, -other.g2.z, other.g0.w) + vec4(self.g1.x) * vec4(-other.g0.w, -other.g1.z, other.g1.y, other.g0.x) + vec4(self.g1.y) * vec4(other.g1.z, -other.g0.w, -other.g1.x, other.g0.y) + vec4(self.g1.z) * vec4(-other.g1.y, other.g1.x, -other.g0.w, other.g0.z), vec4(0.0), vec3(0.0), vec3(0.0), vec3(self.g0.x, self.g0.y, self.g0.z) * vec3(other.g0.w) + vec3(self.g1.x) * vec3(0.0, -other.g0.z, other.g0.y) + vec3(self.g1.y) * vec3(other.g0.z, 0.0, -other.g0.x) + vec3(self.g1.z) * vec3(-other.g0.y, other.g0.x, 0.0), vec2(self.g0.x) * vec2(other.g0.x, other.g2.x) + vec2(self.g0.y) * vec2(other.g0.y, other.g2.y) + vec2(self.g0.z) * vec2(other.g0.z, other.g2.z) + vec2(self.g1.x) * vec2(0.0, other.g1.x) + vec2(self.g1.y) * vec2(0.0, other.g1.y) + vec2(self.g1.z) * vec2(0.0, other.g1.z));
}

MultiVector motor__wedge_dot__dipole(Motor self, Dipole other) {
    return MultiVector(vec2(self.g0.x) * vec2(0.0, -other.g1.x) + vec2(self.g0.y) * vec2(0.0, -other.g1.y) + vec2(self.g0.z) * vec2(0.0, -other.g1.z) + vec2(self.g1.x) * vec2(0.0, -other.g0.x) + vec2(self.g1.y) * vec2(0.0, -other.g0.y) + vec2(self.g1.z) * vec2(0.0, -other.g0.z), vec3(0.0) - vec3(self.g0.x, self.g0.y, self.g0.z) * vec3(other.g2.w) + vec3(self.g1.x) * vec3(0.0, -other.g0.z, other.g0.y) + vec3(self.g1.y) * vec3(other.g0.z, 0.0, -other.g0.x) + vec3(self.g1.z) * vec3(-other.g0.y, other.g0.x, 0.0), vec2(0.0) - vec2(self.g0.x) * vec2(other.g0.x, other.g2.x) - vec2(self.g0.y) * vec2(other.g0.y, other.g2.y) - vec2(self.g0.z) * vec2(other.g0.z, other.g2.z) + vec2(self.g1.x) * vec2(0.0, -other.g1.x) + vec2(self.g1.y) * vec2(0.0, -other.g1.y) + vec2(self.g1.z) * vec2(0.0, -other.g1.z), vec3(0.0), vec3(0.0), vec4(0.0), vec4(self.g0.x) * vec4(0.0, other.g0.z, -other.g0.y, 0.0) + vec4(self.g0.y) * vec4(-other.g0.z, 0.0, other.g0.x, 0.0) + vec4(self.g0.z) * vec4(other.g0.y, -other.g0.x, 0.0, 0.0) + vec4(self.g0.w) * vec4(other.g0.x, other.g0.y, other.g0.z, -other.g2.w) + vec4(self.g1.x) * vec4(0.0, 0.0, 0.0, other.g0.x) + vec4(self.g1.y) * vec4(0.0, 0.0, 0.0, other.g0.y) + vec4(self.g1.z) * vec4(0.0, 0.0, 0.0, other.g0.z), vec3(self.g0.x) * vec3(0.0, other.g1.z, -other.g1.y) + vec3(self.g0.y) * vec3(-other.g1.z, 0.0, other.g1.x) + vec3(self.g0.z) * vec3(other.g1.y, -other.g1.x, 0.0) + vec3(self.g0.w) * other.g1 + vec3(self.g1.x) * vec3(0.0, other.g0.z, -other.g0.y) + vec3(self.g1.y) * vec3(-other.g0.z, 0.0, other.g0.x) + vec3(self.g1.z) * vec3(other.g0.y, -other.g0.x, 0.0), vec3(self.g0.x) * vec3(0.0, other.g2.z, -other.g2.y) + vec3(self.g0.y) * vec3(-other.g2.z, 0.0, other.g2.x) + vec3(self.g0.z) * vec3(other.g2.y, -other.g2.x, 0.0) + vec3(self.g0.w) * vec3(other.g2.x, other.g2.y, other.g2.z) + vec3(self.g1.x) * vec3(-other.g2.w, other.g1.z, -other.g1.y) + vec3(self.g1.y) * vec3(-other.g1.z, -other.g2.w, other.g1.x) + vec3(self.g1.z) * vec3(other.g1.y, -other.g1.x, -other.g2.w), vec3(0.0), vec2(0.0));
}

MultiVector motor__wedge_dot__dual_num(Motor self, DualNum other) {
    return MultiVector(vec2(self.g0.w) * other.g0.yx * vec2(-1.0, 1.0), vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0) - vec3(self.g0.x, self.g0.y, self.g0.z) * vec3(other.g0.y), vec4(self.g1.x, self.g1.y, self.g1.z, self.g1.x) * vec4(-other.g0.y, -other.g0.y, -other.g0.y, 0.0), vec4(0.0), vec3(self.g0.x, self.g0.y, self.g0.z) * vec3(other.g0.x), self.g1 * vec3(other.g0.x), vec3(0.0), vec2(0.0));
}

MultiVector motor__wedge_dot__flat_point(Motor self, FlatPoint other) {
    return MultiVector(vec2(0.0), vec3(0.0) - vec3(self.g0.x, self.g0.y, self.g0.z) * vec3(other.g0.w), vec2(self.g0.x) * vec2(0.0, -other.g0.x) + vec2(self.g0.y) * vec2(0.0, -other.g0.y) + vec2(self.g0.z) * vec2(0.0, -other.g0.z), vec3(0.0), vec3(0.0), vec4(0.0), vec4(self.g0.w) * vec4(0.0, 0.0, 0.0, -other.g0.w), vec3(0.0), vec3(self.g0.x) * vec3(0.0, other.g0.z, -other.g0.y) + vec3(self.g0.y) * vec3(-other.g0.z, 0.0, other.g0.x) + vec3(self.g0.z) * vec3(other.g0.y, -other.g0.x, 0.0) + vec3(self.g0.w) * vec3(other.g0.x, other.g0.y, other.g0.z) - self.g1 * vec3(other.g0.w), vec3(0.0), vec2(0.0));
}

MultiVector motor__wedge_dot__flector(Motor self, Flector other) {
    return MultiVector(vec2(0.0), vec3(self.g0.x) * vec3(-other.g0.w, -other.g1.z, other.g1.y) + vec3(self.g0.y) * vec3(other.g1.z, -other.g0.w, -other.g1.x) + vec3(self.g0.z) * vec3(-other.g1.y, other.g1.x, -other.g0.w) - vec3(self.g0.w) * vec3(other.g1.x, other.g1.y, other.g1.z), vec2(self.g0.x) * vec2(0.0, -other.g0.x) + vec2(self.g0.y) * vec2(0.0, -other.g0.y) + vec2(self.g0.z) * vec2(0.0, -other.g0.z) + vec2(self.g0.w) * vec2(0.0, other.g1.w) + vec2(self.g1.x) * vec2(0.0, other.g1.x) + vec2(self.g1.y) * vec2(0.0, other.g1.y) + vec2(self.g1.z) * vec2(0.0, other.g1.z), vec3(0.0), vec3(0.0), vec4(0.0), vec4(self.g0.x) * vec4(0.0, 0.0, 0.0, other.g1.x) + vec4(self.g0.y) * vec4(0.0, 0.0, 0.0, other.g1.y) + vec4(self.g0.z) * vec4(0.0, 0.0, 0.0, other.g1.z) + vec4(self.g0.w) * vec4(0.0, 0.0, 0.0, -other.g0.w), vec3(0.0), vec3(self.g0.x) * vec3(other.g1.w, other.g0.z, -other.g0.y) + vec3(self.g0.y) * vec3(-other.g0.z, other.g1.w, other.g0.x) + vec3(self.g0.z) * vec3(other.g0.y, -other.g0.x, other.g1.w) + vec3(self.g0.w) * vec3(other.g0.x, other.g0.y, other.g0.z) + vec3(self.g1.x) * vec3(-other.g0.w, -other.g1.z, other.g1.y) + vec3(self.g1.y) * vec3(other.g1.z, -other.g0.w, -other.g1.x) + vec3(self.g1.z) * vec3(-other.g1.y, other.g1.x, -other.g0.w), vec3(0.0), vec2(0.0));
}

MultiVector motor__wedge_dot__line(Motor self, Line other) {
    return MultiVector(vec2(self.g0.x) * vec2(other.g0.x, 0.0) + vec2(self.g0.y) * vec2(other.g0.y, 0.0) + vec2(self.g0.z) * vec2(other.g0.z, 0.0), vec3(0.0), vec2(0.0), vec3(0.0), vec3(self.g0.x) * vec3(0.0, -other.g0.z, other.g0.y) + vec3(self.g0.y) * vec3(other.g0.z, 0.0, -other.g0.x) + vec3(self.g0.z) * vec3(-other.g0.y, other.g0.x, 0.0) - vec3(self.g0.w) * other.g0, vec4(self.g0.x) * vec4(0.0, -other.g1.z, other.g1.y, 0.0) + vec4(self.g0.y) * vec4(other.g1.z, 0.0, -other.g1.x, 0.0) + vec4(self.g0.z) * vec4(-other.g1.y, other.g1.x, 0.0, 0.0) + vec4(self.g0.w) * vec4(-other.g1.x, -other.g1.y, -other.g1.z, 0.0) + vec4(self.g1.x) * vec4(0.0, -other.g0.z, other.g0.y, 0.0) + vec4(self.g1.y) * vec4(other.g0.z, 0.0, -other.g0.x, 0.0) + vec4(self.g1.z) * vec4(-other.g0.y, other.g0.x, 0.0, 0.0), vec4(0.0), vec3(0.0), vec3(0.0), vec3(0.0), vec2(self.g0.x) * vec2(0.0, other.g1.x) + vec2(self.g0.y) * vec2(0.0, other.g1.y) + vec2(self.g0.z) * vec2(0.0, other.g1.z) + vec2(self.g1.x) * vec2(0.0, other.g0.x) + vec2(self.g1.y) * vec2(0.0, other.g0.y) + vec2(self.g1.z) * vec2(0.0, other.g0.z));
}

MultiVector motor__wedge_dot__motor(Motor self, Motor other) {
    return MultiVector(vec2(self.g0.x) * vec2(other.g0.x, 0.0) + vec2(self.g0.y) * vec2(other.g0.y, 0.0) + vec2(self.g0.z) * vec2(other.g0.z, 0.0) + vec2(self.g0.w) * vec2(-other.g0.w, 0.0), vec3(0.0), vec2(0.0), vec3(0.0), vec3(self.g0.x) * vec3(-other.g0.w, -other.g0.z, other.g0.y) + vec3(self.g0.y) * vec3(other.g0.z, -other.g0.w, -other.g0.x) + vec3(self.g0.z) * vec3(-other.g0.y, other.g0.x, -other.g0.w) - vec3(self.g0.w) * vec3(other.g0.x, other.g0.y, other.g0.z), vec4(self.g0.x) * vec4(0.0, -other.g1.z, other.g1.y, 0.0) + vec4(self.g0.y) * vec4(other.g1.z, 0.0, -other.g1.x, 0.0) + vec4(self.g0.z) * vec4(-other.g1.y, other.g1.x, 0.0, 0.0) + vec4(self.g0.w) * vec4(-other.g1.x, -other.g1.y, -other.g1.z, 0.0) + vec4(self.g1.x) * vec4(-other.g0.w, -other.g0.z, other.g0.y, 0.0) + vec4(self.g1.y) * vec4(other.g0.z, -other.g0.w, -other.g0.x, 0.0) + vec4(self.g1.z) * vec4(-other.g0.y, other.g0.x, -other.g0.w, 0.0), vec4(0.0), vec3(0.0), vec3(0.0), vec3(0.0), vec2(self.g0.x) * vec2(0.0, other.g1.x) + vec2(self.g0.y) * vec2(0.0, other.g1.y) + vec2(self.g0.z) * vec2(0.0, other.g1.z) + vec2(self.g1.x) * vec2(0.0, other.g0.x) + vec2(self.g1.y) * vec2(0.0, other.g0.y) + vec2(self.g1.z) * vec2(0.0, other.g0.z));
}

MultiVector motor__wedge_dot__multi_vector(Motor self, MultiVector other) {
    return MultiVector(vec2(self.g0.x) * vec2(other.g7.x, -other.g4.x) + vec2(self.g0.y) * vec2(other.g7.y, -other.g4.y) + vec2(self.g0.z) * vec2(other.g7.z, -other.g4.z) + vec2(self.g0.w) * other.g0.yx * vec2(-1.0, 1.0) + vec2(self.g1.x) * vec2(other.g6.x, -other.g3.x) + vec2(self.g1.y) * vec2(other.g6.y, -other.g3.y) + vec2(self.g1.z) * vec2(other.g6.z, -other.g3.z), vec3(self.g0.x) * vec3(-other.g5.w, -other.g9.z, other.g9.y) + vec3(self.g0.y) * vec3(other.g9.z, -other.g5.w, -other.g9.x) + vec3(self.g0.z) * vec3(-other.g9.y, other.g9.x, -other.g5.w) - vec3(self.g0.w) * other.g9 + vec3(self.g1.x) * vec3(-other.g10.x, -other.g3.z, other.g3.y) + vec3(self.g1.y) * vec3(other.g3.z, -other.g10.x, -other.g3.x) + vec3(self.g1.z) * vec3(-other.g3.y, other.g3.x, -other.g10.x), vec2(0.0) - vec2(self.g0.x) * vec2(other.g3.x, other.g5.x) - vec2(self.g0.y) * vec2(other.g3.y, other.g5.y) - vec2(self.g0.z) * vec2(other.g3.z, other.g5.z) + vec2(self.g0.w) * other.g10 + vec2(self.g1.x) * vec2(0.0, other.g9.x) + vec2(self.g1.x) * vec2(0.0, -other.g4.x) + vec2(self.g1.y) * vec2(0.0, other.g9.y) + vec2(self.g1.y) * vec2(0.0, -other.g4.y) + vec2(self.g1.z) * vec2(0.0, other.g9.z) + vec2(self.g1.z) * vec2(0.0, -other.g4.z), vec3(self.g0.x) * vec3(-other.g2.x, -other.g6.z, other.g6.y) + vec3(self.g0.y) * vec3(other.g6.z, -other.g2.x, -other.g6.x) + vec3(self.g0.z) * vec3(-other.g6.y, other.g6.x, -other.g2.x) - vec3(self.g0.w) * vec3(other.g6.x, other.g6.y, other.g6.z), vec3(self.g0.x) * vec3(-other.g0.y, -other.g7.z, other.g7.y) + vec3(self.g0.y) * vec3(other.g7.z, -other.g0.y, -other.g7.x) + vec3(self.g0.z) * vec3(-other.g7.y, other.g7.x, -other.g0.y) - vec3(self.g0.w) * other.g7 + vec3(self.g1.x) * vec3(-other.g2.x, -other.g6.z, other.g6.y) + vec3(self.g1.y) * vec3(other.g6.z, -other.g2.x, -other.g6.x) + vec3(self.g1.z) * vec3(-other.g6.y, other.g6.x, -other.g2.x), vec4(self.g0.x) * vec4(-other.g2.y, -other.g8.z, other.g8.y, -other.g1.x) + vec4(self.g0.y) * vec4(other.g8.z, -other.g2.y, -other.g8.x, -other.g1.y) + vec4(self.g0.z) * vec4(-other.g8.y, other.g8.x, -other.g2.y, -other.g1.z) + vec4(self.g0.w) * vec4(-other.g8.x, -other.g8.y, -other.g8.z, other.g6.w) + vec4(self.g1.x) * vec4(-other.g6.w, -other.g7.z, other.g7.y, other.g6.x) + vec4(self.g1.x) * vec4(-other.g0.y, -other.g1.z, other.g1.y, 0.0) + vec4(self.g1.y) * vec4(other.g7.z, -other.g6.w, -other.g7.x, other.g6.y) + vec4(self.g1.y) * vec4(other.g1.z, -other.g0.y, -other.g1.x, 0.0) + vec4(self.g1.z) * vec4(-other.g7.y, other.g7.x, -other.g6.w, other.g6.z) + vec4(self.g1.z) * vec4(-other.g1.y, other.g1.x, -other.g0.y, 0.0), vec4(self.g0.x) * vec4(other.g10.x, other.g3.z, -other.g3.y, other.g9.x) + vec4(self.g0.y) * vec4(-other.g3.z, other.g10.x, other.g3.x, other.g9.y) + vec4(self.g0.z) * vec4(other.g3.y, -other.g3.x, other.g10.x, other.g9.z) + vec4(self.g0.w) * vec4(other.g3.x, other.g3.y, other.g3.z, -other.g5.w) + vec4(self.g1.x) * vec4(0.0, 0.0, 0.0, other.g3.x) + vec4(self.g1.y) * vec4(0.0, 0.0, 0.0, other.g3.y) + vec4(self.g1.z) * vec4(0.0, 0.0, 0.0, other.g3.z), vec3(self.g0.x) * vec3(other.g0.x, other.g4.z, -other.g4.y) + vec3(self.g0.y) * vec3(-other.g4.z, other.g0.x, other.g4.x) + vec3(self.g0.z) * vec3(other.g4.y, -other.g4.x, other.g0.x) + vec3(self.g0.w) * other.g4 + vec3(self.g1.x) * vec3(other.g10.x, other.g3.z, -other.g3.y) + vec3(self.g1.y) * vec3(-other.g3.z, other.g10.x, other.g3.x) + vec3(self.g1.z) * vec3(other.g3.y, -other.g3.x, other.g10.x), vec3(self.g0.x) * vec3(other.g10.y, other.g5.z, -other.g5.y) + vec3(self.g0.y) * vec3(-other.g5.z, other.g10.y, other.g5.x) + vec3(self.g0.z) * vec3(other.g5.y, -other.g5.x, other.g10.y) + vec3(self.g0.w) * vec3(other.g5.x, other.g5.y, other.g5.z) + vec3(self.g1.x) * vec3(-other.g5.w, -other.g9.z, other.g9.y) + vec3(self.g1.x) * vec3(other.g0.x, other.g4.z, -other.g4.y) + vec3(self.g1.y) * vec3(other.g9.z, -other.g5.w, -other.g9.x) + vec3(self.g1.y) * vec3(-other.g4.z, other.g0.x, other.g4.x) + vec3(self.g1.z) * vec3(-other.g9.y, other.g9.x, -other.g5.w) + vec3(self.g1.z) * vec3(other.g4.y, -other.g4.x, other.g0.x), vec3(self.g0.x) * vec3(other.g6.w, other.g1.z, -other.g1.y) + vec3(self.g0.y) * vec3(-other.g1.z, other.g6.w, other.g1.x) + vec3(self.g0.z) * vec3(other.g1.y, -other.g1.x, other.g6.w) + vec3(self.g0.w) * other.g1 + vec3(self.g1.x) * vec3(-other.g2.x, -other.g6.z, other.g6.y) + vec3(self.g1.y) * vec3(other.g6.z, -other.g2.x, -other.g6.x) + vec3(self.g1.z) * vec3(-other.g6.y, other.g6.x, -other.g2.x), vec2(self.g0.x) * vec2(other.g6.x, other.g8.x) + vec2(self.g0.y) * vec2(other.g6.y, other.g8.y) + vec2(self.g0.z) * vec2(other.g6.z, other.g8.z) - vec2(self.g0.w) * other.g2 + vec2(self.g1.x) * vec2(0.0, other.g7.x) + vec2(self.g1.x) * vec2(0.0, other.g1.x) + vec2(self.g1.y) * vec2(0.0, other.g7.y) + vec2(self.g1.y) * vec2(0.0, other.g1.y) + vec2(self.g1.z) * vec2(0.0, other.g7.z) + vec2(self.g1.z) * vec2(0.0, other.g1.z));
}

MultiVector motor__wedge_dot__plane(Motor self, Plane other) {
    return MultiVector(vec2(0.0), vec3(self.g0.x) * vec3(0.0, -other.g0.z, other.g0.y) + vec3(self.g0.y) * vec3(other.g0.z, 0.0, -other.g0.x) + vec3(self.g0.z) * vec3(-other.g0.y, other.g0.x, 0.0) - vec3(self.g0.w) * vec3(other.g0.x, other.g0.y, other.g0.z), vec2(self.g0.w) * vec2(0.0, other.g0.w) + vec2(self.g1.x) * vec2(0.0, other.g0.x) + vec2(self.g1.y) * vec2(0.0, other.g0.y) + vec2(self.g1.z) * vec2(0.0, other.g0.z), vec3(0.0), vec3(0.0), vec4(0.0), vec4(self.g0.x) * vec4(0.0, 0.0, 0.0, other.g0.x) + vec4(self.g0.y) * vec4(0.0, 0.0, 0.0, other.g0.y) + vec4(self.g0.z) * vec4(0.0, 0.0, 0.0, other.g0.z), vec3(0.0), vec3(self.g0.x, self.g0.y, self.g0.z) * vec3(other.g0.w) + vec3(self.g1.x) * vec3(0.0, -other.g0.z, other.g0.y) + vec3(self.g1.y) * vec3(other.g0.z, 0.0, -other.g0.x) + vec3(self.g1.z) * vec3(-other.g0.y, other.g0.x, 0.0), vec3(0.0), vec2(0.0));
}

MultiVector motor__wedge_dot__round_point(Motor self, RoundPoint other) {
    return MultiVector(vec2(0.0), vec3(0.0), vec2(0.0), vec3(0.0) - vec3(self.g0.x, self.g0.y, self.g0.z) * vec3(other.g1.x), vec3(0.0) - self.g1 * vec3(other.g1.x), vec4(self.g0.x) * vec4(-other.g1.y, 0.0, 0.0, -other.g0.x) + vec4(self.g0.y) * vec4(0.0, -other.g1.y, 0.0, -other.g0.y) + vec4(self.g0.z) * vec4(0.0, 0.0, -other.g1.y, -other.g0.z) + vec4(self.g1.x) * vec4(0.0, -other.g0.z, other.g0.y, 0.0) + vec4(self.g1.y) * vec4(other.g0.z, 0.0, -other.g0.x, 0.0) + vec4(self.g1.z) * vec4(-other.g0.y, other.g0.x, 0.0, 0.0), vec4(0.0), vec3(0.0), vec3(0.0), vec3(self.g0.x) * vec3(0.0, other.g0.z, -other.g0.y) + vec3(self.g0.y) * vec3(-other.g0.z, 0.0, other.g0.x) + vec3(self.g0.z) * vec3(other.g0.y, -other.g0.x, 0.0) + vec3(self.g0.w) * other.g0 - self.g1 * vec3(other.g1.x), vec2(0.0) - vec2(self.g0.w) * other.g1 + vec2(self.g1.x) * vec2(0.0, other.g0.x) + vec2(self.g1.y) * vec2(0.0, other.g0.y) + vec2(self.g1.z) * vec2(0.0, other.g0.z));
}

Motor motor__wedge_dot__scalar(Motor self, Scalar other) {
    return Motor(self.g0 * vec4(other.g0), self.g1 * vec3(other.g0));
}

MultiVector motor__wedge_dot__sphere(Motor self, Sphere other) {
    return MultiVector(vec2(0.0), vec3(self.g0.x) * vec3(0.0, -other.g0.z, other.g0.y) + vec3(self.g0.y) * vec3(other.g0.z, 0.0, -other.g0.x) + vec3(self.g0.z) * vec3(-other.g0.y, other.g0.x, 0.0) - vec3(self.g0.w) * other.g0 - self.g1 * vec3(other.g1.x), vec2(self.g0.w) * other.g1 + vec2(self.g1.x) * vec2(0.0, other.g0.x) + vec2(self.g1.y) * vec2(0.0, other.g0.y) + vec2(self.g1.z) * vec2(0.0, other.g0.z), vec3(0.0), vec3(0.0), vec4(0.0), vec4(self.g0.x) * vec4(other.g1.x, 0.0, 0.0, other.g0.x) + vec4(self.g0.y) * vec4(0.0, other.g1.x, 0.0, other.g0.y) + vec4(self.g0.z) * vec4(0.0, 0.0, other.g1.x, other.g0.z), self.g1 * vec3(other.g1.x), vec3(self.g0.x, self.g0.y, self.g0.z) * vec3(other.g1.y) + vec3(self.g1.x) * vec3(0.0, -other.g0.z, other.g0.y) + vec3(self.g1.y) * vec3(other.g0.z, 0.0, -other.g0.x) + vec3(self.g1.z) * vec3(-other.g0.y, other.g0.x, 0.0), vec3(0.0), vec2(0.0));
}

MultiVector multi_vector__wedge_dot__anti_scalar(MultiVector self, AntiScalar other) {
    return MultiVector(self.g0.yx * vec2(-other.g0), vec3(0.0) - self.g9 * vec3(other.g0), self.g10 * vec2(other.g0), vec3(0.0) - vec3(self.g6.x, self.g6.y, self.g6.z) * vec3(other.g0), vec3(0.0) - self.g7 * vec3(other.g0), vec4(self.g6.w) * vec4(0.0, 0.0, 0.0, other.g0) + vec4(self.g8.x, self.g8.y, self.g8.z, self.g8.x) * vec4(-other.g0, -other.g0, -other.g0, 0.0), vec4(self.g3.x, self.g3.y, self.g3.z, self.g3.x) * vec4(other.g0, other.g0, other.g0, 0.0) + vec4(self.g5.w) * vec4(0.0, 0.0, 0.0, -other.g0), self.g4 * vec3(other.g0), vec3(self.g5.x, self.g5.y, self.g5.z) * vec3(other.g0), self.g1 * vec3(other.g0), vec2(0.0) - self.g2 * vec2(other.g0));
}

MultiVector multi_vector__wedge_dot__circle(MultiVector self, Circle other) {
    return MultiVector(vec2(self.g3.x) * vec2(0.0, -other.g2.x) + vec2(self.g3.y) * vec2(0.0, -other.g2.y) + vec2(self.g3.z) * vec2(0.0, -other.g2.z) + vec2(self.g4.x) * vec2(0.0, -other.g1.x) + vec2(self.g4.y) * vec2(0.0, -other.g1.y) + vec2(self.g4.z) * vec2(0.0, -other.g1.z) + vec2(self.g5.x) * vec2(0.0, -other.g0.x) + vec2(self.g5.y) * vec2(0.0, -other.g0.y) + vec2(self.g5.z) * vec2(0.0, -other.g0.z) + vec2(self.g5.w) * vec2(0.0, -other.g0.w) + vec2(self.g6.x) * vec2(other.g2.x, 0.0) + vec2(self.g6.y) * vec2(other.g2.y, 0.0) + vec2(self.g6.z) * vec2(other.g2.z, 0.0) + vec2(self.g6.w) * vec2(-other.g0.w, 0.0) + vec2(self.g7.x) * vec2(other.g1.x, 0.0) + vec2(self.g7.y) * vec2(other.g1.y, 0.0) + vec2(self.g7.z) * vec2(other.g1.z, 0.0) + vec2(self.g8.x) * vec2(other.g0.x, 0.0) + vec2(self.g8.y) * vec2(other.g0.y, 0.0) + vec2(self.g8.z) * vec2(other.g0.z, 0.0), vec3(self.g3.x) * vec3(0.0, other.g2.z, -other.g2.y) + vec3(self.g3.y) * vec3(-other.g2.z, 0.0, other.g2.x) + vec3(self.g3.z) * vec3(other.g2.y, -other.g2.x, 0.0) + self.g4 * vec3(other.g0.w) + vec3(self.g5.x) * vec3(0.0, -other.g0.z, other.g0.y) + vec3(self.g5.y) * vec3(other.g0.z, 0.0, -other.g0.x) + vec3(self.g5.z) * vec3(-other.g0.y, other.g0.x, 0.0) - vec3(self.g5.w) * other.g1 + vec3(self.g9.x) * vec3(0.0, -other.g1.z, other.g1.y) + vec3(self.g9.y) * vec3(other.g1.z, 0.0, -other.g1.x) + vec3(self.g9.z) * vec3(-other.g1.y, other.g1.x, 0.0) + vec3(self.g10.x) * other.g2 - vec3(self.g10.y) * vec3(other.g0.x, other.g0.y, other.g0.z), vec2(self.g3.x) * vec2(-other.g1.x, 0.0) + vec2(self.g3.y) * vec2(-other.g1.y, 0.0) + vec2(self.g3.z) * vec2(-other.g1.z, 0.0) - vec2(self.g4.x) * vec2(other.g0.x, other.g2.x) - vec2(self.g4.y) * vec2(other.g0.y, other.g2.y) - vec2(self.g4.z) * vec2(other.g0.z, other.g2.z) + vec2(self.g5.x) * vec2(0.0, -other.g1.x) + vec2(self.g5.y) * vec2(0.0, -other.g1.y) + vec2(self.g5.z) * vec2(0.0, -other.g1.z) + vec2(self.g9.x) * vec2(other.g0.x, -other.g2.x) + vec2(self.g9.y) * vec2(other.g0.y, -other.g2.y) + vec2(self.g9.z) * vec2(other.g0.z, -other.g2.z) + self.g10 * vec2(-other.g0.w), vec3(0.0) - vec3(self.g0.y) * vec3(other.g0.x, other.g0.y, other.g0.z) + vec3(self.g1.x) * vec3(0.0, -other.g0.z, other.g0.y) + vec3(self.g1.y) * vec3(other.g0.z, 0.0, -other.g0.x) + vec3(self.g1.z) * vec3(-other.g0.y, other.g0.x, 0.0) - vec3(self.g2.x) * other.g1 + vec3(self.g6.x) * vec3(other.g0.w, -other.g1.z, other.g1.y) + vec3(self.g6.y) * vec3(other.g1.z, other.g0.w, -other.g1.x) + vec3(self.g6.z) * vec3(-other.g1.y, other.g1.x, other.g0.w) - vec3(self.g6.w) * vec3(other.g0.x, other.g0.y, other.g0.z) + vec3(self.g7.x) * vec3(0.0, -other.g0.z, other.g0.y) + vec3(self.g7.y) * vec3(other.g0.z, 0.0, -other.g0.x) + vec3(self.g7.z) * vec3(-other.g0.y, other.g0.x, 0.0), vec3(0.0) - vec3(self.g0.y) * other.g1 - self.g1 * vec3(other.g0.w) - vec3(self.g2.x) * other.g2 - vec3(self.g2.y) * vec3(other.g0.x, other.g0.y, other.g0.z) + vec3(self.g6.x) * vec3(0.0, -other.g2.z, other.g2.y) + vec3(self.g6.y) * vec3(other.g2.z, 0.0, -other.g2.x) + vec3(self.g6.z) * vec3(-other.g2.y, other.g2.x, 0.0) + vec3(self.g7.x) * vec3(0.0, -other.g1.z, other.g1.y) + vec3(self.g7.y) * vec3(other.g1.z, 0.0, -other.g1.x) + vec3(self.g7.z) * vec3(-other.g1.y, other.g1.x, 0.0) + vec3(self.g8.x) * vec3(0.0, -other.g0.z, other.g0.y) + vec3(self.g8.y) * vec3(other.g0.z, 0.0, -other.g0.x) + vec3(self.g8.z) * vec3(-other.g0.y, other.g0.x, 0.0), vec4(self.g0.y) * vec4(-other.g2.x, -other.g2.y, -other.g2.z, other.g0.w) + vec4(self.g1.x) * vec4(0.0, other.g2.z, -other.g2.y, -other.g1.x) + vec4(self.g1.y) * vec4(-other.g2.z, 0.0, other.g2.x, -other.g1.y) + vec4(self.g1.z) * vec4(other.g2.y, -other.g2.x, 0.0, -other.g1.z) + vec4(self.g2.y) * vec4(-other.g1.x, -other.g1.y, -other.g1.z, 0.0) + vec4(self.g6.x) * vec4(0.0, 0.0, 0.0, -other.g2.x) + vec4(self.g6.y) * vec4(0.0, 0.0, 0.0, -other.g2.y) + self.g6.wwwz * vec4(other.g2.x, other.g2.y, other.g2.z, -other.g2.z) + vec4(self.g7.x) * vec4(0.0, -other.g2.z, other.g2.y, 0.0) + vec4(self.g7.y) * vec4(other.g2.z, 0.0, -other.g2.x, 0.0) + vec4(self.g7.z) * vec4(-other.g2.y, other.g2.x, 0.0, 0.0) + vec4(self.g8.x) * vec4(-other.g0.w, -other.g1.z, other.g1.y, other.g0.x) + vec4(self.g8.y) * vec4(other.g1.z, -other.g0.w, -other.g1.x, other.g0.y) + vec4(self.g8.z) * vec4(-other.g1.y, other.g1.x, -other.g0.w, other.g0.z), vec4(self.g0.x) * other.g0 + vec4(self.g3.x) * vec4(-other.g0.w, other.g1.z, -other.g1.y, -other.g2.x) + vec4(self.g3.y) * vec4(-other.g1.z, -other.g0.w, other.g1.x, -other.g2.y) + vec4(self.g3.z) * vec4(other.g1.y, -other.g1.x, -other.g0.w, -other.g2.z) + vec4(self.g4.x) * vec4(0.0, other.g0.z, -other.g0.y, 0.0) + vec4(self.g4.y) * vec4(-other.g0.z, 0.0, other.g0.x, 0.0) + vec4(self.g4.z) * vec4(other.g0.y, -other.g0.x, 0.0, 0.0) + vec4(self.g5.x) * vec4(0.0, 0.0, 0.0, other.g0.x) + vec4(self.g5.y) * vec4(0.0, 0.0, 0.0, other.g0.y) + self.g5.wwwz * other.g0.xyzz * vec4(-1.0, -1.0, -1.0, 1.0) + vec4(self.g9.x) * vec4(0.0, -other.g0.z, other.g0.y, other.g1.x) + vec4(self.g9.y) * vec4(other.g0.z, 0.0, -other.g0.x, other.g1.y) + vec4(self.g9.z) * vec4(-other.g0.y, other.g0.x, 0.0, other.g1.z) + vec4(self.g10.x) * vec4(other.g1.x, other.g1.y, other.g1.z, 0.0), vec3(self.g0.x) * other.g1 + vec3(self.g3.x) * vec3(0.0, other.g2.z, -other.g2.y) + vec3(self.g3.y) * vec3(-other.g2.z, 0.0, other.g2.x) + vec3(self.g3.z) * vec3(other.g2.y, -other.g2.x, 0.0) + vec3(self.g4.x) * vec3(0.0, other.g1.z, -other.g1.y) + vec3(self.g4.y) * vec3(-other.g1.z, 0.0, other.g1.x) + vec3(self.g4.z) * vec3(other.g1.y, -other.g1.x, 0.0) + vec3(self.g5.x) * vec3(0.0, other.g0.z, -other.g0.y) + vec3(self.g5.y) * vec3(-other.g0.z, 0.0, other.g0.x) + vec3(self.g5.z) * vec3(other.g0.y, -other.g0.x, 0.0) - self.g9 * vec3(other.g0.w) + vec3(self.g10.x) * other.g2 + vec3(self.g10.y) * vec3(other.g0.x, other.g0.y, other.g0.z), vec3(self.g0.x) * other.g2 + vec3(self.g4.x) * vec3(0.0, other.g2.z, -other.g2.y) + vec3(self.g4.y) * vec3(-other.g2.z, 0.0, other.g2.x) + vec3(self.g4.z) * vec3(other.g2.y, -other.g2.x, 0.0) + vec3(self.g5.x) * vec3(other.g0.w, other.g1.z, -other.g1.y) + vec3(self.g5.y) * vec3(-other.g1.z, other.g0.w, other.g1.x) + vec3(self.g5.z) * vec3(other.g1.y, -other.g1.x, other.g0.w) + vec3(self.g5.w) * other.g2 + vec3(self.g9.x) * vec3(0.0, other.g2.z, -other.g2.y) + vec3(self.g9.y) * vec3(-other.g2.z, 0.0, other.g2.x) + vec3(self.g9.z) * vec3(other.g2.y, -other.g2.x, 0.0) + vec3(self.g10.y) * other.g1, vec3(self.g1.x) * vec3(0.0, other.g1.z, -other.g1.y) + vec3(self.g1.y) * vec3(-other.g1.z, 0.0, other.g1.x) + vec3(self.g1.z) * vec3(other.g1.y, -other.g1.x, 0.0) + vec3(self.g2.x) * other.g2 - vec3(self.g2.y) * vec3(other.g0.x, other.g0.y, other.g0.z) + vec3(self.g6.x) * vec3(0.0, other.g2.z, -other.g2.y) + vec3(self.g6.y) * vec3(-other.g2.z, 0.0, other.g2.x) + vec3(self.g6.z) * vec3(other.g2.y, -other.g2.x, 0.0) + vec3(self.g6.w) * other.g1 + self.g7 * vec3(other.g0.w) + vec3(self.g8.x) * vec3(0.0, -other.g0.z, other.g0.y) + vec3(self.g8.y) * vec3(other.g0.z, 0.0, -other.g0.x) + vec3(self.g8.z) * vec3(-other.g0.y, other.g0.x, 0.0), vec2(self.g1.x) * vec2(other.g0.x, -other.g2.x) + vec2(self.g1.y) * vec2(other.g0.y, -other.g2.y) + vec2(self.g1.z) * vec2(other.g0.z, -other.g2.z) + self.g2 * vec2(other.g0.w) + vec2(self.g6.x) * vec2(other.g1.x, 0.0) + vec2(self.g6.y) * vec2(other.g1.y, 0.0) + vec2(self.g6.z) * vec2(other.g1.z, 0.0) + vec2(self.g7.x) * vec2(other.g0.x, other.g2.x) + vec2(self.g7.y) * vec2(other.g0.y, other.g2.y) + vec2(self.g7.z) * vec2(other.g0.z, other.g2.z) + vec2(self.g8.x) * vec2(0.0, other.g1.x) + vec2(self.g8.y) * vec2(0.0, other.g1.y) + vec2(self.g8.z) * vec2(0.0, other.g1.z));
}

MultiVector multi_vector__wedge_dot__dipole(MultiVector self, Dipole other) {
    return MultiVector(vec2(self.g3.x) * vec2(-other.g2.x, 0.0) + vec2(self.g3.y) * vec2(-other.g2.y, 0.0) + vec2(self.g3.z) * vec2(-other.g2.z, 0.0) + vec2(self.g4.x) * vec2(-other.g1.x, 0.0) + vec2(self.g4.y) * vec2(-other.g1.y, 0.0) + vec2(self.g4.z) * vec2(-other.g1.z, 0.0) + vec2(self.g5.x) * vec2(-other.g0.x, 0.0) + vec2(self.g5.y) * vec2(-other.g0.y, 0.0) + vec2(self.g5.z) * vec2(-other.g0.z, 0.0) + vec2(self.g5.w) * vec2(other.g2.w, 0.0) + vec2(self.g6.x) * vec2(0.0, -other.g2.x) + vec2(self.g6.y) * vec2(0.0, -other.g2.y) + vec2(self.g6.z) * vec2(0.0, -other.g2.z) + vec2(self.g6.w) * vec2(0.0, -other.g2.w) + vec2(self.g7.x) * vec2(0.0, -other.g1.x) + vec2(self.g7.y) * vec2(0.0, -other.g1.y) + vec2(self.g7.z) * vec2(0.0, -other.g1.z) + vec2(self.g8.x) * vec2(0.0, -other.g0.x) + vec2(self.g8.y) * vec2(0.0, -other.g0.y) + vec2(self.g8.z) * vec2(0.0, -other.g0.z), vec3(self.g1.x) * vec3(0.0, other.g1.z, -other.g1.y) + vec3(self.g1.y) * vec3(-other.g1.z, 0.0, other.g1.x) + vec3(self.g1.z) * vec3(other.g1.y, -other.g1.x, 0.0) + vec3(self.g2.x) * vec3(other.g2.x, other.g2.y, other.g2.z) - vec3(self.g2.y) * other.g0 + vec3(self.g6.x) * vec3(0.0, other.g2.z, -other.g2.y) + vec3(self.g6.y) * vec3(-other.g2.z, 0.0, other.g2.x) + vec3(self.g6.z) * vec3(other.g2.y, -other.g2.x, 0.0) + vec3(self.g6.w) * other.g1 - self.g7 * vec3(other.g2.w) + vec3(self.g8.x) * vec3(0.0, -other.g0.z, other.g0.y) + vec3(self.g8.y) * vec3(other.g0.z, 0.0, -other.g0.x) + vec3(self.g8.z) * vec3(-other.g0.y, other.g0.x, 0.0), vec2(self.g1.x) * vec2(-other.g0.x, other.g2.x) + vec2(self.g1.y) * vec2(-other.g0.y, other.g2.y) + vec2(self.g1.z) * vec2(-other.g0.z, other.g2.z) + self.g2 * vec2(other.g2.w) + vec2(self.g6.x) * vec2(-other.g1.x, 0.0) + vec2(self.g6.y) * vec2(-other.g1.y, 0.0) + vec2(self.g6.z) * vec2(-other.g1.z, 0.0) - vec2(self.g7.x) * vec2(other.g0.x, other.g2.x) - vec2(self.g7.y) * vec2(other.g0.y, other.g2.y) - vec2(self.g7.z) * vec2(other.g0.z, other.g2.z) + vec2(self.g8.x) * vec2(0.0, -other.g1.x) + vec2(self.g8.y) * vec2(0.0, -other.g1.y) + vec2(self.g8.z) * vec2(0.0, -other.g1.z), vec3(self.g0.x) * other.g0 + vec3(self.g3.x) * vec3(other.g2.w, other.g1.z, -other.g1.y) + vec3(self.g3.y) * vec3(-other.g1.z, other.g2.w, other.g1.x) + vec3(self.g3.z) * vec3(other.g1.y, -other.g1.x, other.g2.w) + vec3(self.g4.x) * vec3(0.0, other.g0.z, -other.g0.y) + vec3(self.g4.y) * vec3(-other.g0.z, 0.0, other.g0.x) + vec3(self.g4.z) * vec3(other.g0.y, -other.g0.x, 0.0) - vec3(self.g5.w) * other.g0 + vec3(self.g9.x) * vec3(0.0, -other.g0.z, other.g0.y) + vec3(self.g9.y) * vec3(other.g0.z, 0.0, -other.g0.x) + vec3(self.g9.z) * vec3(-other.g0.y, other.g0.x, 0.0) + vec3(self.g10.x) * other.g1, vec3(self.g0.x) * other.g1 + vec3(self.g3.x) * vec3(0.0, other.g2.z, -other.g2.y) + vec3(self.g3.y) * vec3(-other.g2.z, 0.0, other.g2.x) + vec3(self.g3.z) * vec3(other.g2.y, -other.g2.x, 0.0) + vec3(self.g4.x) * vec3(0.0, other.g1.z, -other.g1.y) + vec3(self.g4.y) * vec3(-other.g1.z, 0.0, other.g1.x) + vec3(self.g4.z) * vec3(other.g1.y, -other.g1.x, 0.0) + vec3(self.g5.x) * vec3(0.0, other.g0.z, -other.g0.y) + vec3(self.g5.y) * vec3(-other.g0.z, 0.0, other.g0.x) + vec3(self.g5.z) * vec3(other.g0.y, -other.g0.x, 0.0) + self.g9 * vec3(other.g2.w) + vec3(self.g10.x) * vec3(other.g2.x, other.g2.y, other.g2.z) + vec3(self.g10.y) * other.g0, vec4(self.g0.x) * other.g2 + vec4(self.g3.x) * vec4(0.0, 0.0, 0.0, other.g2.x) + vec4(self.g3.y) * vec4(0.0, 0.0, 0.0, other.g2.y) + vec4(self.g3.z) * vec4(0.0, 0.0, 0.0, other.g2.z) + vec4(self.g4.x) * vec4(0.0, other.g2.z, -other.g2.y, 0.0) + vec4(self.g4.y) * vec4(-other.g2.z, 0.0, other.g2.x, 0.0) + vec4(self.g4.z) * vec4(other.g2.y, -other.g2.x, 0.0, 0.0) + vec4(self.g5.x) * vec4(-other.g2.w, other.g1.z, -other.g1.y, -other.g0.x) + vec4(self.g5.y) * vec4(-other.g1.z, -other.g2.w, other.g1.x, -other.g0.y) + vec4(self.g5.z) * vec4(other.g1.y, -other.g1.x, -other.g2.w, -other.g0.z) + vec4(self.g5.w) * vec4(other.g2.x, other.g2.y, other.g2.z, 0.0) + vec4(self.g9.x) * vec4(0.0, other.g2.z, -other.g2.y, -other.g1.x) + vec4(self.g9.y) * vec4(-other.g2.z, 0.0, other.g2.x, -other.g1.y) + vec4(self.g9.z) * vec4(other.g2.y, -other.g2.x, 0.0, -other.g1.z) + vec4(self.g10.y) * vec4(other.g1.x, other.g1.y, other.g1.z, 0.0), vec4(self.g0.y) * vec4(other.g0.x, other.g0.y, other.g0.z, -other.g2.w) + vec4(self.g1.x) * vec4(0.0, other.g0.z, -other.g0.y, -other.g1.x) + vec4(self.g1.y) * vec4(-other.g0.z, 0.0, other.g0.x, -other.g1.y) + vec4(self.g1.z) * vec4(other.g0.y, -other.g0.x, 0.0, -other.g1.z) + vec4(self.g2.x) * vec4(other.g1.x, other.g1.y, other.g1.z, 0.0) + vec4(self.g6.x) * vec4(other.g2.w, other.g1.z, -other.g1.y, -other.g2.x) + vec4(self.g6.y) * vec4(-other.g1.z, other.g2.w, other.g1.x, -other.g2.y) + vec4(self.g6.z) * vec4(other.g1.y, -other.g1.x, other.g2.w, -other.g2.z) + vec4(self.g6.w) * vec4(other.g0.x, other.g0.y, other.g0.z, 0.0) + vec4(self.g7.x) * vec4(0.0, other.g0.z, -other.g0.y, 0.0) + vec4(self.g7.y) * vec4(-other.g0.z, 0.0, other.g0.x, 0.0) + vec4(self.g7.z) * vec4(other.g0.y, -other.g0.x, 0.0, 0.0) + vec4(self.g8.x) * vec4(0.0, 0.0, 0.0, other.g0.x) + vec4(self.g8.y) * vec4(0.0, 0.0, 0.0, other.g0.y) + vec4(self.g8.z) * vec4(0.0, 0.0, 0.0, other.g0.z), vec3(self.g0.y) * other.g1 - self.g1 * vec3(other.g2.w) + vec3(self.g2.x) * vec3(other.g2.x, other.g2.y, other.g2.z) + vec3(self.g2.y) * other.g0 + vec3(self.g6.x) * vec3(0.0, other.g2.z, -other.g2.y) + vec3(self.g6.y) * vec3(-other.g2.z, 0.0, other.g2.x) + vec3(self.g6.z) * vec3(other.g2.y, -other.g2.x, 0.0) + vec3(self.g7.x) * vec3(0.0, other.g1.z, -other.g1.y) + vec3(self.g7.y) * vec3(-other.g1.z, 0.0, other.g1.x) + vec3(self.g7.z) * vec3(other.g1.y, -other.g1.x, 0.0) + vec3(self.g8.x) * vec3(0.0, other.g0.z, -other.g0.y) + vec3(self.g8.y) * vec3(-other.g0.z, 0.0, other.g0.x) + vec3(self.g8.z) * vec3(other.g0.y, -other.g0.x, 0.0), vec3(self.g0.y) * vec3(other.g2.x, other.g2.y, other.g2.z) + vec3(self.g1.x) * vec3(0.0, -other.g2.z, other.g2.y) + vec3(self.g1.y) * vec3(other.g2.z, 0.0, -other.g2.x) + vec3(self.g1.z) * vec3(-other.g2.y, other.g2.x, 0.0) + vec3(self.g2.y) * other.g1 - vec3(self.g6.w) * vec3(other.g2.x, other.g2.y, other.g2.z) + vec3(self.g7.x) * vec3(0.0, other.g2.z, -other.g2.y) + vec3(self.g7.y) * vec3(-other.g2.z, 0.0, other.g2.x) + vec3(self.g7.z) * vec3(other.g2.y, -other.g2.x, 0.0) + vec3(self.g8.x) * vec3(-other.g2.w, other.g1.z, -other.g1.y) + vec3(self.g8.y) * vec3(-other.g1.z, -other.g2.w, other.g1.x) + vec3(self.g8.z) * vec3(other.g1.y, -other.g1.x, -other.g2.w), vec3(self.g3.x) * vec3(0.0, -other.g2.z, other.g2.y) + vec3(self.g3.y) * vec3(other.g2.z, 0.0, -other.g2.x) + vec3(self.g3.z) * vec3(-other.g2.y, other.g2.x, 0.0) + self.g4 * vec3(other.g2.w) + vec3(self.g5.x) * vec3(0.0, other.g0.z, -other.g0.y) + vec3(self.g5.y) * vec3(-other.g0.z, 0.0, other.g0.x) + vec3(self.g5.z) * vec3(other.g0.y, -other.g0.x, 0.0) + vec3(self.g5.w) * other.g1 + vec3(self.g9.x) * vec3(0.0, other.g1.z, -other.g1.y) + vec3(self.g9.y) * vec3(-other.g1.z, 0.0, other.g1.x) + vec3(self.g9.z) * vec3(other.g1.y, -other.g1.x, 0.0) - vec3(self.g10.x) * vec3(other.g2.x, other.g2.y, other.g2.z) + vec3(self.g10.y) * other.g0, vec2(self.g3.x) * vec2(-other.g1.x, 0.0) + vec2(self.g3.y) * vec2(-other.g1.y, 0.0) + vec2(self.g3.z) * vec2(-other.g1.z, 0.0) - vec2(self.g4.x) * vec2(other.g0.x, other.g2.x) - vec2(self.g4.y) * vec2(other.g0.y, other.g2.y) - vec2(self.g4.z) * vec2(other.g0.z, other.g2.z) + vec2(self.g5.x) * vec2(0.0, -other.g1.x) + vec2(self.g5.y) * vec2(0.0, -other.g1.y) + vec2(self.g5.z) * vec2(0.0, -other.g1.z) + vec2(self.g9.x) * vec2(other.g0.x, -other.g2.x) + vec2(self.g9.y) * vec2(other.g0.y, -other.g2.y) + vec2(self.g9.z) * vec2(other.g0.z, -other.g2.z) + self.g10 * vec2(other.g2.w));
}

MultiVector multi_vector__wedge_dot__dual_num(MultiVector self, DualNum other) {
    return MultiVector(vec2(self.g0.x) * other.g0 + vec2(self.g0.y) * other.g0.yx * vec2(-1.0, 1.0), self.g1 * vec3(other.g0.x) - self.g9 * vec3(other.g0.y), self.g2 * vec2(other.g0.x) + self.g10 * vec2(other.g0.y), self.g3 * vec3(other.g0.x) - vec3(self.g6.x, self.g6.y, self.g6.z) * vec3(other.g0.y), self.g4 * vec3(other.g0.x) - self.g7 * vec3(other.g0.y), self.g5 * vec4(other.g0.x) + vec4(self.g6.w) * vec4(0.0, 0.0, 0.0, other.g0.y) + vec4(self.g8.x, self.g8.y, self.g8.z, self.g8.x) * vec4(-other.g0.y, -other.g0.y, -other.g0.y, 0.0), vec4(self.g3.x, self.g3.y, self.g3.z, self.g3.x) * vec4(other.g0.y, other.g0.y, other.g0.y, 0.0) + vec4(self.g5.w) * vec4(0.0, 0.0, 0.0, -other.g0.y) + self.g6 * vec4(other.g0.x), self.g4 * vec3(other.g0.y) + self.g7 * vec3(other.g0.x), vec3(self.g5.x, self.g5.y, self.g5.z) * vec3(other.g0.y) + self.g8 * vec3(other.g0.x), self.g1 * vec3(other.g0.y) + self.g9 * vec3(other.g0.x), vec2(0.0) - self.g2 * vec2(other.g0.y) + self.g10 * vec2(other.g0.x));
}

MultiVector multi_vector__wedge_dot__flat_point(MultiVector self, FlatPoint other) {
    return MultiVector(vec2(self.g3.x) * vec2(-other.g0.x, 0.0) + vec2(self.g3.y) * vec2(-other.g0.y, 0.0) + vec2(self.g3.z) * vec2(-other.g0.z, 0.0) + vec2(self.g5.w) * vec2(other.g0.w, 0.0) + vec2(self.g6.x) * vec2(0.0, -other.g0.x) + vec2(self.g6.y) * vec2(0.0, -other.g0.y) + vec2(self.g6.z) * vec2(0.0, -other.g0.z) + vec2(self.g6.w) * vec2(0.0, -other.g0.w), vec3(self.g2.x) * vec3(other.g0.x, other.g0.y, other.g0.z) + vec3(self.g6.x) * vec3(0.0, other.g0.z, -other.g0.y) + vec3(self.g6.y) * vec3(-other.g0.z, 0.0, other.g0.x) + vec3(self.g6.z) * vec3(other.g0.y, -other.g0.x, 0.0) - self.g7 * vec3(other.g0.w), vec2(self.g1.x) * vec2(0.0, other.g0.x) + vec2(self.g1.y) * vec2(0.0, other.g0.y) + vec2(self.g1.z) * vec2(0.0, other.g0.z) + self.g2 * vec2(other.g0.w) + vec2(self.g7.x) * vec2(0.0, -other.g0.x) + vec2(self.g7.y) * vec2(0.0, -other.g0.y) + vec2(self.g7.z) * vec2(0.0, -other.g0.z), self.g3 * vec3(other.g0.w), vec3(self.g3.x) * vec3(0.0, other.g0.z, -other.g0.y) + vec3(self.g3.y) * vec3(-other.g0.z, 0.0, other.g0.x) + vec3(self.g3.z) * vec3(other.g0.y, -other.g0.x, 0.0) + self.g9 * vec3(other.g0.w) + vec3(self.g10.x) * vec3(other.g0.x, other.g0.y, other.g0.z), vec4(self.g0.x) * other.g0 + vec4(self.g3.x) * vec4(0.0, 0.0, 0.0, other.g0.x) + vec4(self.g3.y) * vec4(0.0, 0.0, 0.0, other.g0.y) + vec4(self.g3.z) * vec4(0.0, 0.0, 0.0, other.g0.z) + vec4(self.g4.x) * vec4(0.0, other.g0.z, -other.g0.y, 0.0) + vec4(self.g4.y) * vec4(-other.g0.z, 0.0, other.g0.x, 0.0) + vec4(self.g4.z) * vec4(other.g0.y, -other.g0.x, 0.0, 0.0) + self.g5.xyzx * vec4(-other.g0.w, -other.g0.w, -other.g0.w, 0.0) + vec4(self.g5.w) * vec4(other.g0.x, other.g0.y, other.g0.z, 0.0) + vec4(self.g9.x) * vec4(0.0, other.g0.z, -other.g0.y, 0.0) + vec4(self.g9.y) * vec4(-other.g0.z, 0.0, other.g0.x, 0.0) + vec4(self.g9.z) * vec4(other.g0.y, -other.g0.x, 0.0, 0.0), vec4(self.g0.y) * vec4(0.0, 0.0, 0.0, -other.g0.w) + vec4(self.g6.x) * vec4(other.g0.w, 0.0, 0.0, -other.g0.x) + vec4(self.g6.y) * vec4(0.0, other.g0.w, 0.0, -other.g0.y) + vec4(self.g6.z) * vec4(0.0, 0.0, other.g0.w, -other.g0.z), vec3(0.0) - self.g1 * vec3(other.g0.w) + vec3(self.g2.x) * vec3(other.g0.x, other.g0.y, other.g0.z) + vec3(self.g6.x) * vec3(0.0, other.g0.z, -other.g0.y) + vec3(self.g6.y) * vec3(-other.g0.z, 0.0, other.g0.x) + vec3(self.g6.z) * vec3(other.g0.y, -other.g0.x, 0.0), vec3(self.g0.y) * vec3(other.g0.x, other.g0.y, other.g0.z) + vec3(self.g1.x) * vec3(0.0, -other.g0.z, other.g0.y) + vec3(self.g1.y) * vec3(other.g0.z, 0.0, -other.g0.x) + vec3(self.g1.z) * vec3(-other.g0.y, other.g0.x, 0.0) - vec3(self.g6.w) * vec3(other.g0.x, other.g0.y, other.g0.z) + vec3(self.g7.x) * vec3(0.0, other.g0.z, -other.g0.y) + vec3(self.g7.y) * vec3(-other.g0.z, 0.0, other.g0.x) + vec3(self.g7.z) * vec3(other.g0.y, -other.g0.x, 0.0) - self.g8 * vec3(other.g0.w), vec3(self.g3.x) * vec3(0.0, -other.g0.z, other.g0.y) + vec3(self.g3.y) * vec3(other.g0.z, 0.0, -other.g0.x) + vec3(self.g3.z) * vec3(-other.g0.y, other.g0.x, 0.0) + self.g4 * vec3(other.g0.w) - vec3(self.g10.x) * vec3(other.g0.x, other.g0.y, other.g0.z), vec2(self.g4.x) * vec2(0.0, -other.g0.x) + vec2(self.g4.y) * vec2(0.0, -other.g0.y) + vec2(self.g4.z) * vec2(0.0, -other.g0.z) + vec2(self.g9.x) * vec2(0.0, -other.g0.x) + vec2(self.g9.y) * vec2(0.0, -other.g0.y) + vec2(self.g9.z) * vec2(0.0, -other.g0.z) + self.g10 * vec2(other.g0.w));
}

MultiVector multi_vector__wedge_dot__flector(MultiVector self, Flector other) {
    return MultiVector(vec2(self.g1.x) * vec2(0.0, other.g1.x) + vec2(self.g1.y) * vec2(0.0, other.g1.y) + vec2(self.g1.z) * vec2(0.0, other.g1.z) + vec2(self.g2.x) * vec2(0.0, other.g1.w) + vec2(self.g3.x) * vec2(-other.g0.x, 0.0) + vec2(self.g3.y) * vec2(-other.g0.y, 0.0) + vec2(self.g3.z) * vec2(-other.g0.z, 0.0) + vec2(self.g5.w) * vec2(other.g0.w, 0.0) + vec2(self.g6.x) * vec2(0.0, -other.g0.x) + vec2(self.g6.y) * vec2(0.0, -other.g0.y) + vec2(self.g6.z) * vec2(0.0, -other.g0.z) + vec2(self.g6.w) * vec2(0.0, -other.g0.w) + vec2(self.g9.x) * vec2(-other.g1.x, 0.0) + vec2(self.g9.y) * vec2(-other.g1.y, 0.0) + vec2(self.g9.z) * vec2(-other.g1.z, 0.0) + vec2(self.g10.x) * vec2(other.g1.w, 0.0), vec3(0.0) - vec3(self.g0.y) * vec3(other.g1.x, other.g1.y, other.g1.z) + vec3(self.g2.x) * vec3(other.g0.x, other.g0.y, other.g0.z) + vec3(self.g6.x) * vec3(other.g1.w, other.g0.z, -other.g0.y) + vec3(self.g6.y) * vec3(-other.g0.z, other.g1.w, other.g0.x) + vec3(self.g6.z) * vec3(other.g0.y, -other.g0.x, other.g1.w) + vec3(self.g7.x) * vec3(-other.g0.w, -other.g1.z, other.g1.y) + vec3(self.g7.y) * vec3(other.g1.z, -other.g0.w, -other.g1.x) + vec3(self.g7.z) * vec3(-other.g1.y, other.g1.x, -other.g0.w), vec2(self.g0.y) * vec2(0.0, other.g1.w) + vec2(self.g1.x) * vec2(0.0, other.g0.x) + vec2(self.g1.y) * vec2(0.0, other.g0.y) + vec2(self.g1.z) * vec2(0.0, other.g0.z) + self.g2 * vec2(other.g0.w) + vec2(self.g6.x) * vec2(-other.g1.x, 0.0) + vec2(self.g6.y) * vec2(-other.g1.y, 0.0) - vec2(self.g6.z, self.g6.w) * vec2(other.g1.z, other.g1.w) + vec2(self.g7.x) * vec2(0.0, -other.g0.x) + vec2(self.g7.y) * vec2(0.0, -other.g0.y) + vec2(self.g7.z) * vec2(0.0, -other.g0.z) + vec2(self.g8.x) * vec2(0.0, other.g1.x) + vec2(self.g8.y) * vec2(0.0, other.g1.y) + vec2(self.g8.z) * vec2(0.0, other.g1.z), vec3(self.g3.x) * vec3(other.g0.w, other.g1.z, -other.g1.y) + vec3(self.g3.y) * vec3(-other.g1.z, other.g0.w, other.g1.x) + vec3(self.g3.z) * vec3(other.g1.y, -other.g1.x, other.g0.w) + vec3(self.g10.x) * vec3(other.g1.x, other.g1.y, other.g1.z), vec3(self.g3.x) * vec3(other.g1.w, other.g0.z, -other.g0.y) + vec3(self.g3.y) * vec3(-other.g0.z, other.g1.w, other.g0.x) + vec3(self.g3.z) * vec3(other.g0.y, -other.g0.x, other.g1.w) + vec3(self.g5.w) * vec3(other.g1.x, other.g1.y, other.g1.z) + vec3(self.g9.x) * vec3(other.g0.w, other.g1.z, -other.g1.y) + vec3(self.g9.y) * vec3(-other.g1.z, other.g0.w, other.g1.x) + vec3(self.g9.z) * vec3(other.g1.y, -other.g1.x, other.g0.w) + vec3(self.g10.x) * vec3(other.g0.x, other.g0.y, other.g0.z), vec4(self.g0.x) * other.g0 + vec4(self.g3.x) * vec4(0.0, 0.0, 0.0, other.g0.x) + vec4(self.g3.y) * vec4(0.0, 0.0, 0.0, other.g0.y) + vec4(self.g3.z) * vec4(0.0, 0.0, 0.0, other.g0.z) + vec4(self.g4.x) * vec4(other.g1.w, other.g0.z, -other.g0.y, -other.g1.x) + vec4(self.g4.y) * vec4(-other.g0.z, other.g1.w, other.g0.x, -other.g1.y) + vec4(self.g4.z) * vec4(other.g0.y, -other.g0.x, other.g1.w, -other.g1.z) + vec4(self.g5.x) * vec4(-other.g0.w, -other.g1.z, other.g1.y, 0.0) + vec4(self.g5.y) * vec4(other.g1.z, -other.g0.w, -other.g1.x, 0.0) + vec4(self.g5.z) * vec4(-other.g1.y, other.g1.x, -other.g0.w, 0.0) + vec4(self.g5.w) * vec4(other.g0.x, other.g0.y, other.g0.z, 0.0) + vec4(self.g9.x) * vec4(other.g1.w, other.g0.z, -other.g0.y, 0.0) + vec4(self.g9.y) * vec4(-other.g0.z, other.g1.w, other.g0.x, 0.0) + vec4(self.g9.z) * vec4(other.g0.y, -other.g0.x, other.g1.w, 0.0) - vec4(self.g10.y, self.g10.y, self.g10.y, self.g10.x) * other.g1, vec4(self.g0.y) * vec4(0.0, 0.0, 0.0, -other.g0.w) + vec4(self.g2.x) * other.g1 + vec4(self.g6.x) * vec4(other.g0.w, other.g1.z, -other.g1.y, -other.g0.x) + vec4(self.g6.y) * vec4(-other.g1.z, other.g0.w, other.g1.x, -other.g0.y) + vec4(self.g6.z) * vec4(other.g1.y, -other.g1.x, other.g0.w, -other.g0.z) + vec4(self.g7.x) * vec4(0.0, 0.0, 0.0, other.g1.x) + vec4(self.g7.y) * vec4(0.0, 0.0, 0.0, other.g1.y) + vec4(self.g7.z) * vec4(0.0, 0.0, 0.0, other.g1.z), vec3(self.g1.x) * vec3(-other.g0.w, -other.g1.z, other.g1.y) + vec3(self.g1.y) * vec3(other.g1.z, -other.g0.w, -other.g1.x) + vec3(self.g1.z) * vec3(-other.g1.y, other.g1.x, -other.g0.w) + vec3(self.g2.x) * vec3(other.g0.x, other.g0.y, other.g0.z) + vec3(self.g6.x) * vec3(other.g1.w, other.g0.z, -other.g0.y) + vec3(self.g6.y) * vec3(-other.g0.z, other.g1.w, other.g0.x) + vec3(self.g6.z) * vec3(other.g0.y, -other.g0.x, other.g1.w) - vec3(self.g6.w) * vec3(other.g1.x, other.g1.y, other.g1.z), vec3(self.g0.y) * vec3(other.g0.x, other.g0.y, other.g0.z) + vec3(self.g1.x) * vec3(-other.g1.w, -other.g0.z, other.g0.y) + vec3(self.g1.y) * vec3(other.g0.z, -other.g1.w, -other.g0.x) + vec3(self.g1.z) * vec3(-other.g0.y, other.g0.x, -other.g1.w) - vec3(self.g2.y) * vec3(other.g1.x, other.g1.y, other.g1.z) - vec3(self.g6.w) * vec3(other.g0.x, other.g0.y, other.g0.z) + vec3(self.g7.x) * vec3(other.g1.w, other.g0.z, -other.g0.y) + vec3(self.g7.y) * vec3(-other.g0.z, other.g1.w, other.g0.x) + vec3(self.g7.z) * vec3(other.g0.y, -other.g0.x, other.g1.w) + vec3(self.g8.x) * vec3(-other.g0.w, -other.g1.z, other.g1.y) + vec3(self.g8.y) * vec3(other.g1.z, -other.g0.w, -other.g1.x) + vec3(self.g8.z) * vec3(-other.g1.y, other.g1.x, -other.g0.w), vec3(self.g0.x) * vec3(other.g1.x, other.g1.y, other.g1.z) + vec3(self.g3.x) * vec3(-other.g1.w, -other.g0.z, other.g0.y) + vec3(self.g3.y) * vec3(other.g0.z, -other.g1.w, -other.g0.x) + vec3(self.g3.z) * vec3(-other.g0.y, other.g0.x, -other.g1.w) + vec3(self.g4.x) * vec3(other.g0.w, other.g1.z, -other.g1.y) + vec3(self.g4.y) * vec3(-other.g1.z, other.g0.w, other.g1.x) + vec3(self.g4.z) * vec3(other.g1.y, -other.g1.x, other.g0.w) - vec3(self.g10.x) * vec3(other.g0.x, other.g0.y, other.g0.z), vec2(self.g0.x) * vec2(0.0, other.g1.w) + vec2(self.g3.x) * vec2(-other.g1.x, 0.0) + vec2(self.g3.y) * vec2(-other.g1.y, 0.0) + vec2(self.g3.z) * vec2(-other.g1.z, 0.0) + vec2(self.g4.x) * vec2(0.0, -other.g0.x) + vec2(self.g4.y) * vec2(0.0, -other.g0.y) + vec2(self.g4.z) * vec2(0.0, -other.g0.z) + vec2(self.g5.x) * vec2(0.0, other.g1.x) + vec2(self.g5.y) * vec2(0.0, other.g1.y) + vec2(self.g5.z) * vec2(0.0, other.g1.z) + vec2(self.g5.w) * vec2(0.0, other.g1.w) + vec2(self.g9.x) * vec2(0.0, -other.g0.x) + vec2(self.g9.y) * vec2(0.0, -other.g0.y) + vec2(self.g9.z) * vec2(0.0, -other.g0.z) + self.g10 * vec2(other.g0.w));
}

MultiVector multi_vector__wedge_dot__line(MultiVector self, Line other) {
    return MultiVector(vec2(self.g3.x) * vec2(0.0, -other.g1.x) + vec2(self.g3.y) * vec2(0.0, -other.g1.y) + vec2(self.g3.z) * vec2(0.0, -other.g1.z) + vec2(self.g4.x) * vec2(0.0, -other.g0.x) + vec2(self.g4.y) * vec2(0.0, -other.g0.y) + vec2(self.g4.z) * vec2(0.0, -other.g0.z) + vec2(self.g6.x) * vec2(other.g1.x, 0.0) + vec2(self.g6.y) * vec2(other.g1.y, 0.0) + vec2(self.g6.z) * vec2(other.g1.z, 0.0) + vec2(self.g7.x) * vec2(other.g0.x, 0.0) + vec2(self.g7.y) * vec2(other.g0.y, 0.0) + vec2(self.g7.z) * vec2(other.g0.z, 0.0), vec3(self.g3.x) * vec3(0.0, other.g1.z, -other.g1.y) + vec3(self.g3.y) * vec3(-other.g1.z, 0.0, other.g1.x) + vec3(self.g3.z) * vec3(other.g1.y, -other.g1.x, 0.0) - vec3(self.g5.w) * other.g0 + vec3(self.g9.x) * vec3(0.0, -other.g0.z, other.g0.y) + vec3(self.g9.y) * vec3(other.g0.z, 0.0, -other.g0.x) + vec3(self.g9.z) * vec3(-other.g0.y, other.g0.x, 0.0) + vec3(self.g10.x) * other.g1, vec2(self.g3.x) * vec2(-other.g0.x, 0.0) + vec2(self.g3.y) * vec2(-other.g0.y, 0.0) + vec2(self.g3.z) * vec2(-other.g0.z, 0.0) + vec2(self.g4.x) * vec2(0.0, -other.g1.x) + vec2(self.g4.y) * vec2(0.0, -other.g1.y) + vec2(self.g4.z) * vec2(0.0, -other.g1.z) + vec2(self.g5.x) * vec2(0.0, -other.g0.x) + vec2(self.g5.y) * vec2(0.0, -other.g0.y) + vec2(self.g5.z) * vec2(0.0, -other.g0.z) + vec2(self.g9.x) * vec2(0.0, -other.g1.x) + vec2(self.g9.y) * vec2(0.0, -other.g1.y) + vec2(self.g9.z) * vec2(0.0, -other.g1.z), vec3(0.0) - vec3(self.g2.x) * other.g0 + vec3(self.g6.x) * vec3(0.0, -other.g0.z, other.g0.y) + vec3(self.g6.y) * vec3(other.g0.z, 0.0, -other.g0.x) + vec3(self.g6.z) * vec3(-other.g0.y, other.g0.x, 0.0), vec3(0.0) - vec3(self.g0.y) * other.g0 - vec3(self.g2.x) * other.g1 + vec3(self.g6.x) * vec3(0.0, -other.g1.z, other.g1.y) + vec3(self.g6.y) * vec3(other.g1.z, 0.0, -other.g1.x) + vec3(self.g6.z) * vec3(-other.g1.y, other.g1.x, 0.0) + vec3(self.g7.x) * vec3(0.0, -other.g0.z, other.g0.y) + vec3(self.g7.y) * vec3(other.g0.z, 0.0, -other.g0.x) + vec3(self.g7.z) * vec3(-other.g0.y, other.g0.x, 0.0), vec4(self.g0.y) * vec4(-other.g1.x, -other.g1.y, -other.g1.z, 0.0) + vec4(self.g1.x) * vec4(0.0, other.g1.z, -other.g1.y, -other.g0.x) + vec4(self.g1.y) * vec4(-other.g1.z, 0.0, other.g1.x, -other.g0.y) + vec4(self.g1.z) * vec4(other.g1.y, -other.g1.x, 0.0, -other.g0.z) + vec4(self.g2.y) * vec4(-other.g0.x, -other.g0.y, -other.g0.z, 0.0) + vec4(self.g6.x) * vec4(0.0, 0.0, 0.0, -other.g1.x) + vec4(self.g6.y) * vec4(0.0, 0.0, 0.0, -other.g1.y) + self.g6.wwwz * vec4(other.g1.x, other.g1.y, other.g1.z, -other.g1.z) + vec4(self.g7.x) * vec4(0.0, -other.g1.z, other.g1.y, 0.0) + vec4(self.g7.y) * vec4(other.g1.z, 0.0, -other.g1.x, 0.0) + vec4(self.g7.z) * vec4(-other.g1.y, other.g1.x, 0.0, 0.0) + vec4(self.g8.x) * vec4(0.0, -other.g0.z, other.g0.y, 0.0) + vec4(self.g8.y) * vec4(other.g0.z, 0.0, -other.g0.x, 0.0) + vec4(self.g8.z) * vec4(-other.g0.y, other.g0.x, 0.0, 0.0), vec4(self.g3.x) * vec4(0.0, other.g0.z, -other.g0.y, -other.g1.x) + vec4(self.g3.y) * vec4(-other.g0.z, 0.0, other.g0.x, -other.g1.y) + vec4(self.g3.z) * vec4(other.g0.y, -other.g0.x, 0.0, -other.g1.z) + vec4(self.g9.x) * vec4(0.0, 0.0, 0.0, other.g0.x) + vec4(self.g9.y) * vec4(0.0, 0.0, 0.0, other.g0.y) + vec4(self.g9.z) * vec4(0.0, 0.0, 0.0, other.g0.z) + vec4(self.g10.x) * vec4(other.g0.x, other.g0.y, other.g0.z, 0.0), vec3(self.g0.x) * other.g0 + vec3(self.g3.x) * vec3(0.0, other.g1.z, -other.g1.y) + vec3(self.g3.y) * vec3(-other.g1.z, 0.0, other.g1.x) + vec3(self.g3.z) * vec3(other.g1.y, -other.g1.x, 0.0) + vec3(self.g4.x) * vec3(0.0, other.g0.z, -other.g0.y) + vec3(self.g4.y) * vec3(-other.g0.z, 0.0, other.g0.x) + vec3(self.g4.z) * vec3(other.g0.y, -other.g0.x, 0.0) + vec3(self.g10.x) * other.g1, vec3(self.g0.x) * other.g1 + vec3(self.g4.x) * vec3(0.0, other.g1.z, -other.g1.y) + vec3(self.g4.y) * vec3(-other.g1.z, 0.0, other.g1.x) + vec3(self.g4.z) * vec3(other.g1.y, -other.g1.x, 0.0) + vec3(self.g5.x) * vec3(0.0, other.g0.z, -other.g0.y) + vec3(self.g5.y) * vec3(-other.g0.z, 0.0, other.g0.x) + vec3(self.g5.z) * vec3(other.g0.y, -other.g0.x, 0.0) + vec3(self.g5.w) * other.g1 + vec3(self.g9.x) * vec3(0.0, other.g1.z, -other.g1.y) + vec3(self.g9.y) * vec3(-other.g1.z, 0.0, other.g1.x) + vec3(self.g9.z) * vec3(other.g1.y, -other.g1.x, 0.0) + vec3(self.g10.y) * other.g0, vec3(self.g1.x) * vec3(0.0, other.g0.z, -other.g0.y) + vec3(self.g1.y) * vec3(-other.g0.z, 0.0, other.g0.x) + vec3(self.g1.z) * vec3(other.g0.y, -other.g0.x, 0.0) + vec3(self.g2.x) * other.g1 + vec3(self.g6.x) * vec3(0.0, other.g1.z, -other.g1.y) + vec3(self.g6.y) * vec3(-other.g1.z, 0.0, other.g1.x) + vec3(self.g6.z) * vec3(other.g1.y, -other.g1.x, 0.0) + vec3(self.g6.w) * other.g0, vec2(self.g1.x) * vec2(0.0, -other.g1.x) + vec2(self.g1.y) * vec2(0.0, -other.g1.y) + vec2(self.g1.z) * vec2(0.0, -other.g1.z) + vec2(self.g6.x) * vec2(other.g0.x, 0.0) + vec2(self.g6.y) * vec2(other.g0.y, 0.0) + vec2(self.g6.z) * vec2(other.g0.z, 0.0) + vec2(self.g7.x) * vec2(0.0, other.g1.x) + vec2(self.g7.y) * vec2(0.0, other.g1.y) + vec2(self.g7.z) * vec2(0.0, other.g1.z) + vec2(self.g8.x) * vec2(0.0, other.g0.x) + vec2(self.g8.y) * vec2(0.0, other.g0.y) + vec2(self.g8.z) * vec2(0.0, other.g0.z));
}

MultiVector multi_vector__wedge_dot__motor(MultiVector self, Motor other) {
    return MultiVector(self.g0.yx * vec2(-other.g0.w) + vec2(self.g3.x) * vec2(0.0, -other.g1.x) + vec2(self.g3.y) * vec2(0.0, -other.g1.y) + vec2(self.g3.z) * vec2(0.0, -other.g1.z) + vec2(self.g4.x) * vec2(0.0, -other.g0.x) + vec2(self.g4.y) * vec2(0.0, -other.g0.y) + vec2(self.g4.z) * vec2(0.0, -other.g0.z) + vec2(self.g6.x) * vec2(other.g1.x, 0.0) + vec2(self.g6.y) * vec2(other.g1.y, 0.0) + vec2(self.g6.z) * vec2(other.g1.z, 0.0) + vec2(self.g7.x) * vec2(other.g0.x, 0.0) + vec2(self.g7.y) * vec2(other.g0.y, 0.0) + vec2(self.g7.z) * vec2(other.g0.z, 0.0), vec3(self.g3.x) * vec3(0.0, other.g1.z, -other.g1.y) + vec3(self.g3.y) * vec3(-other.g1.z, 0.0, other.g1.x) + vec3(self.g3.z) * vec3(other.g1.y, -other.g1.x, 0.0) - vec3(self.g5.w) * vec3(other.g0.x, other.g0.y, other.g0.z) + vec3(self.g9.x) * vec3(-other.g0.w, -other.g0.z, other.g0.y) + vec3(self.g9.y) * vec3(other.g0.z, -other.g0.w, -other.g0.x) + vec3(self.g9.z) * vec3(-other.g0.y, other.g0.x, -other.g0.w) + vec3(self.g10.x) * other.g1, vec2(self.g3.x) * vec2(-other.g0.x, 0.0) + vec2(self.g3.y) * vec2(-other.g0.y, 0.0) + vec2(self.g3.z) * vec2(-other.g0.z, 0.0) + vec2(self.g4.x) * vec2(0.0, -other.g1.x) + vec2(self.g4.y) * vec2(0.0, -other.g1.y) + vec2(self.g4.z) * vec2(0.0, -other.g1.z) + vec2(self.g5.x) * vec2(0.0, -other.g0.x) + vec2(self.g5.y) * vec2(0.0, -other.g0.y) + vec2(self.g5.z) * vec2(0.0, -other.g0.z) + vec2(self.g9.x) * vec2(0.0, -other.g1.x) + vec2(self.g9.y) * vec2(0.0, -other.g1.y) + vec2(self.g9.z) * vec2(0.0, -other.g1.z) + self.g10 * vec2(other.g0.w), vec3(0.0) - vec3(self.g2.x) * vec3(other.g0.x, other.g0.y, other.g0.z) + vec3(self.g6.x) * vec3(-other.g0.w, -other.g0.z, other.g0.y) + vec3(self.g6.y) * vec3(other.g0.z, -other.g0.w, -other.g0.x) + vec3(self.g6.z) * vec3(-other.g0.y, other.g0.x, -other.g0.w), vec3(0.0) - vec3(self.g0.y) * vec3(other.g0.x, other.g0.y, other.g0.z) - vec3(self.g2.x) * other.g1 + vec3(self.g6.x) * vec3(0.0, -other.g1.z, other.g1.y) + vec3(self.g6.y) * vec3(other.g1.z, 0.0, -other.g1.x) + vec3(self.g6.z) * vec3(-other.g1.y, other.g1.x, 0.0) + vec3(self.g7.x) * vec3(-other.g0.w, -other.g0.z, other.g0.y) + vec3(self.g7.y) * vec3(other.g0.z, -other.g0.w, -other.g0.x) + vec3(self.g7.z) * vec3(-other.g0.y, other.g0.x, -other.g0.w), vec4(self.g0.y) * vec4(-other.g1.x, -other.g1.y, -other.g1.z, 0.0) + vec4(self.g1.x) * vec4(0.0, other.g1.z, -other.g1.y, -other.g0.x) + vec4(self.g1.y) * vec4(-other.g1.z, 0.0, other.g1.x, -other.g0.y) + vec4(self.g1.z) * vec4(other.g1.y, -other.g1.x, 0.0, -other.g0.z) + vec4(self.g2.y) * vec4(-other.g0.x, -other.g0.y, -other.g0.z, 0.0) + vec4(self.g6.x) * vec4(0.0, 0.0, 0.0, -other.g1.x) + vec4(self.g6.y) * vec4(0.0, 0.0, 0.0, -other.g1.y) + vec4(self.g6.z) * vec4(0.0, 0.0, 0.0, -other.g1.z) + vec4(self.g6.w) * vec4(other.g1.x, other.g1.y, other.g1.z, other.g0.w) + vec4(self.g7.x) * vec4(0.0, -other.g1.z, other.g1.y, 0.0) + vec4(self.g7.y) * vec4(other.g1.z, 0.0, -other.g1.x, 0.0) + vec4(self.g7.z) * vec4(-other.g1.y, other.g1.x, 0.0, 0.0) + vec4(self.g8.x) * vec4(-other.g0.w, -other.g0.z, other.g0.y, 0.0) + vec4(self.g8.y) * vec4(other.g0.z, -other.g0.w, -other.g0.x, 0.0) + vec4(self.g8.z) * vec4(-other.g0.y, other.g0.x, -other.g0.w, 0.0), vec4(self.g3.x) * vec4(other.g0.w, other.g0.z, -other.g0.y, -other.g1.x) + vec4(self.g3.y) * vec4(-other.g0.z, other.g0.w, other.g0.x, -other.g1.y) + vec4(self.g3.z) * vec4(other.g0.y, -other.g0.x, other.g0.w, -other.g1.z) + vec4(self.g5.w) * vec4(0.0, 0.0, 0.0, -other.g0.w) + vec4(self.g9.x) * vec4(0.0, 0.0, 0.0, other.g0.x) + vec4(self.g9.y) * vec4(0.0, 0.0, 0.0, other.g0.y) + vec4(self.g9.z) * vec4(0.0, 0.0, 0.0, other.g0.z) + vec4(self.g10.x) * vec4(other.g0.x, other.g0.y, other.g0.z, 0.0), vec3(self.g0.x) * vec3(other.g0.x, other.g0.y, other.g0.z) + vec3(self.g3.x) * vec3(0.0, other.g1.z, -other.g1.y) + vec3(self.g3.y) * vec3(-other.g1.z, 0.0, other.g1.x) + vec3(self.g3.z) * vec3(other.g1.y, -other.g1.x, 0.0) + vec3(self.g4.x) * vec3(other.g0.w, other.g0.z, -other.g0.y) + vec3(self.g4.y) * vec3(-other.g0.z, other.g0.w, other.g0.x) + vec3(self.g4.z) * vec3(other.g0.y, -other.g0.x, other.g0.w) + vec3(self.g10.x) * other.g1, vec3(self.g0.x) * other.g1 + vec3(self.g4.x) * vec3(0.0, other.g1.z, -other.g1.y) + vec3(self.g4.y) * vec3(-other.g1.z, 0.0, other.g1.x) + vec3(self.g4.z) * vec3(other.g1.y, -other.g1.x, 0.0) + vec3(self.g5.x) * vec3(other.g0.w, other.g0.z, -other.g0.y) + vec3(self.g5.y) * vec3(-other.g0.z, other.g0.w, other.g0.x) + vec3(self.g5.z) * vec3(other.g0.y, -other.g0.x, other.g0.w) + vec3(self.g5.w) * other.g1 + vec3(self.g9.x) * vec3(0.0, other.g1.z, -other.g1.y) + vec3(self.g9.y) * vec3(-other.g1.z, 0.0, other.g1.x) + vec3(self.g9.z) * vec3(other.g1.y, -other.g1.x, 0.0) + vec3(self.g10.y) * vec3(other.g0.x, other.g0.y, other.g0.z), vec3(self.g1.x) * vec3(other.g0.w, other.g0.z, -other.g0.y) + vec3(self.g1.y) * vec3(-other.g0.z, other.g0.w, other.g0.x) + vec3(self.g1.z) * vec3(other.g0.y, -other.g0.x, other.g0.w) + vec3(self.g2.x) * other.g1 + vec3(self.g6.x) * vec3(0.0, other.g1.z, -other.g1.y) + vec3(self.g6.y) * vec3(-other.g1.z, 0.0, other.g1.x) + vec3(self.g6.z) * vec3(other.g1.y, -other.g1.x, 0.0) + vec3(self.g6.w) * vec3(other.g0.x, other.g0.y, other.g0.z), vec2(self.g1.x) * vec2(0.0, -other.g1.x) + vec2(self.g1.y) * vec2(0.0, -other.g1.y) + vec2(self.g1.z) * vec2(0.0, -other.g1.z) - self.g2 * vec2(other.g0.w) + vec2(self.g6.x) * vec2(other.g0.x, 0.0) + vec2(self.g6.y) * vec2(other.g0.y, 0.0) + vec2(self.g6.z) * vec2(other.g0.z, 0.0) + vec2(self.g7.x) * vec2(0.0, other.g1.x) + vec2(self.g7.y) * vec2(0.0, other.g1.y) + vec2(self.g7.z) * vec2(0.0, other.g1.z) + vec2(self.g8.x) * vec2(0.0, other.g0.x) + vec2(self.g8.y) * vec2(0.0, other.g0.y) + vec2(self.g8.z) * vec2(0.0, other.g0.z));
}

MultiVector multi_vector__wedge_dot__multi_vector(MultiVector self, MultiVector other) {
    return MultiVector(vec2(self.g0.x) * other.g0 + vec2(self.g0.y) * other.g0.yx * vec2(-1.0, 1.0) + vec2(self.g1.x) * vec2(other.g1.x, other.g9.x) + vec2(self.g1.y) * vec2(other.g1.y, other.g9.y) + vec2(self.g1.z) * vec2(other.g1.z, other.g9.z) + vec2(self.g2.x) * vec2(-other.g2.y, other.g10.y) + vec2(self.g2.y) * vec2(-other.g2.x, other.g10.x) - vec2(self.g3.x) * vec2(other.g5.x, other.g8.x) - vec2(self.g3.y) * vec2(other.g5.y, other.g8.y) - vec2(self.g3.z) * vec2(other.g5.z, other.g8.z) - vec2(self.g4.x) * vec2(other.g4.x, other.g7.x) - vec2(self.g4.y) * vec2(other.g4.y, other.g7.y) - vec2(self.g4.z) * vec2(other.g4.z, other.g7.z) - vec2(self.g5.x) * vec2(other.g3.x, other.g6.x) - vec2(self.g5.y) * vec2(other.g3.y, other.g6.y) - vec2(self.g5.z) * vec2(other.g3.z, other.g6.z) + vec2(self.g5.w) * vec2(other.g5.w, -other.g6.w) + vec2(self.g6.x) * vec2(other.g8.x, -other.g5.x) + vec2(self.g6.y) * vec2(other.g8.y, -other.g5.y) + vec2(self.g6.z) * vec2(other.g8.z, -other.g5.z) - vec2(self.g6.w) * vec2(other.g6.w, other.g5.w) + vec2(self.g7.x) * vec2(other.g7.x, -other.g4.x) + vec2(self.g7.y) * vec2(other.g7.y, -other.g4.y) + vec2(self.g7.z) * vec2(other.g7.z, -other.g4.z) + vec2(self.g8.x) * vec2(other.g6.x, -other.g3.x) + vec2(self.g8.y) * vec2(other.g6.y, -other.g3.y) + vec2(self.g8.z) * vec2(other.g6.z, -other.g3.z) + vec2(self.g9.x) * vec2(-other.g9.x, other.g1.x) + vec2(self.g9.y) * vec2(-other.g9.y, other.g1.y) + vec2(self.g9.z) * vec2(-other.g9.z, other.g1.z) + vec2(self.g10.x) * vec2(other.g10.y, other.g2.y) + vec2(self.g10.y) * vec2(other.g10.x, other.g2.x), vec3(self.g0.x) * other.g1 - vec3(self.g0.y) * other.g9 + vec3(self.g1.x) * vec3(other.g0.x, other.g4.z, -other.g4.y) + vec3(self.g1.y) * vec3(-other.g4.z, other.g0.x, other.g4.x) + vec3(self.g1.z) * vec3(other.g4.y, -other.g4.x, other.g0.x) + vec3(self.g2.x) * vec3(other.g5.x, other.g5.y, other.g5.z) - vec3(self.g2.y) * other.g3 + vec3(self.g3.x) * vec3(other.g2.y, other.g8.z, -other.g8.y) + vec3(self.g3.y) * vec3(-other.g8.z, other.g2.y, other.g8.x) + vec3(self.g3.z) * vec3(other.g8.y, -other.g8.x, other.g2.y) + vec3(self.g4.x) * vec3(other.g6.w, other.g1.z, -other.g1.y) + vec3(self.g4.y) * vec3(-other.g1.z, other.g6.w, other.g1.x) + vec3(self.g4.z) * vec3(other.g1.y, -other.g1.x, other.g6.w) + vec3(self.g5.x) * vec3(-other.g2.x, -other.g6.z, other.g6.y) + vec3(self.g5.y) * vec3(other.g6.z, -other.g2.x, -other.g6.x) + vec3(self.g5.z) * vec3(-other.g6.y, other.g6.x, -other.g2.x) - vec3(self.g5.w) * other.g7 + vec3(self.g6.x) * vec3(other.g10.y, other.g5.z, -other.g5.y) + vec3(self.g6.y) * vec3(-other.g5.z, other.g10.y, other.g5.x) + vec3(self.g6.z) * vec3(other.g5.y, -other.g5.x, other.g10.y) + vec3(self.g6.w) * other.g4 + vec3(self.g7.x) * vec3(-other.g5.w, -other.g9.z, other.g9.y) + vec3(self.g7.y) * vec3(other.g9.z, -other.g5.w, -other.g9.x) + vec3(self.g7.z) * vec3(-other.g9.y, other.g9.x, -other.g5.w) + vec3(self.g8.x) * vec3(-other.g10.x, -other.g3.z, other.g3.y) + vec3(self.g8.y) * vec3(other.g3.z, -other.g10.x, -other.g3.x) + vec3(self.g8.z) * vec3(-other.g3.y, other.g3.x, -other.g10.x) + vec3(self.g9.x) * vec3(-other.g0.y, -other.g7.z, other.g7.y) + vec3(self.g9.y) * vec3(other.g7.z, -other.g0.y, -other.g7.x) + vec3(self.g9.z) * vec3(-other.g7.y, other.g7.x, -other.g0.y) + vec3(self.g10.x) * other.g8 - vec3(self.g10.y) * vec3(other.g6.x, other.g6.y, other.g6.z), vec2(self.g0.x) * other.g2 + vec2(self.g0.y) * other.g10 + vec2(self.g1.x) * vec2(-other.g3.x, other.g5.x) + vec2(self.g1.y) * vec2(-other.g3.y, other.g5.y) + vec2(self.g1.z) * vec2(-other.g3.z, other.g5.z) + self.g2 * vec2(other.g5.w) + self.g2 * vec2(other.g0.x) + vec2(self.g3.x) * vec2(-other.g7.x, 0.0) + vec2(self.g3.x) * vec2(other.g1.x, 0.0) + vec2(self.g3.y) * vec2(-other.g7.y, 0.0) + vec2(self.g3.y) * vec2(other.g1.y, 0.0) + vec2(self.g3.z) * vec2(-other.g7.z, 0.0) + vec2(self.g3.z) * vec2(other.g1.z, 0.0) - vec2(self.g4.x) * vec2(other.g6.x, other.g8.x) - vec2(self.g4.y) * vec2(other.g6.y, other.g8.y) - vec2(self.g4.z) * vec2(other.g6.z, other.g8.z) + vec2(self.g5.x) * vec2(0.0, -other.g7.x) + vec2(self.g5.x) * vec2(0.0, -other.g1.x) + vec2(self.g5.y) * vec2(0.0, -other.g7.y) + vec2(self.g5.y) * vec2(0.0, -other.g1.y) + vec2(self.g5.z) * vec2(0.0, -other.g7.z) + vec2(self.g5.z) * vec2(0.0, -other.g1.z) + vec2(self.g5.w) * other.g2 * vec2(-1.0, 1.0) + vec2(self.g6.x) * vec2(-other.g9.x, 0.0) + vec2(self.g6.x) * vec2(-other.g4.x, 0.0) + vec2(self.g6.y) * vec2(-other.g9.y, 0.0) + vec2(self.g6.y) * vec2(-other.g4.y, 0.0) + vec2(self.g6.z) * vec2(-other.g9.z, 0.0) + vec2(self.g6.z) * vec2(-other.g4.z, 0.0) + vec2(self.g6.w) * other.g10 * vec2(1.0, -1.0) - vec2(self.g7.x) * vec2(other.g3.x, other.g5.x) - vec2(self.g7.y) * vec2(other.g3.y, other.g5.y) - vec2(self.g7.z) * vec2(other.g3.z, other.g5.z) + vec2(self.g8.x) * vec2(0.0, other.g9.x) + vec2(self.g8.x) * vec2(0.0, -other.g4.x) + vec2(self.g8.y) * vec2(0.0, other.g9.y) + vec2(self.g8.y) * vec2(0.0, -other.g4.y) + vec2(self.g8.z) * vec2(0.0, other.g9.z) + vec2(self.g8.z) * vec2(0.0, -other.g4.z) + vec2(self.g9.x) * vec2(other.g6.x, -other.g8.x) + vec2(self.g9.y) * vec2(other.g6.y, -other.g8.y) + vec2(self.g9.z) * vec2(other.g6.z, -other.g8.z) + self.g10 * vec2(-other.g6.w) + self.g10 * vec2(other.g0.y), vec3(self.g0.x) * other.g3 - vec3(self.g0.y) * vec3(other.g6.x, other.g6.y, other.g6.z) + vec3(self.g1.x) * vec3(-other.g2.x, -other.g6.z, other.g6.y) + vec3(self.g1.y) * vec3(other.g6.z, -other.g2.x, -other.g6.x) + vec3(self.g1.z) * vec3(-other.g6.y, other.g6.x, -other.g2.x) - vec3(self.g2.x) * other.g7 + vec3(self.g2.x) * other.g1 + vec3(self.g3.x) * vec3(other.g5.w, other.g9.z, -other.g9.y) + vec3(self.g3.x) * vec3(other.g0.x, other.g4.z, -other.g4.y) + vec3(self.g3.y) * vec3(-other.g9.z, other.g5.w, other.g9.x) + vec3(self.g3.y) * vec3(-other.g4.z, other.g0.x, other.g4.x) + vec3(self.g3.z) * vec3(other.g9.y, -other.g9.x, other.g5.w) + vec3(self.g3.z) * vec3(other.g4.y, -other.g4.x, other.g0.x) + vec3(self.g4.x) * vec3(other.g10.x, other.g3.z, -other.g3.y) + vec3(self.g4.y) * vec3(-other.g3.z, other.g10.x, other.g3.x) + vec3(self.g4.z) * vec3(other.g3.y, -other.g3.x, other.g10.x) - vec3(self.g5.w) * other.g3 + vec3(self.g6.x) * vec3(other.g6.w, -other.g7.z, other.g7.y) + vec3(self.g6.x) * vec3(-other.g0.y, other.g1.z, -other.g1.y) + vec3(self.g6.y) * vec3(other.g7.z, other.g6.w, -other.g7.x) + vec3(self.g6.y) * vec3(-other.g1.z, -other.g0.y, other.g1.x) + vec3(self.g6.z) * vec3(-other.g7.y, other.g7.x, other.g6.w) + vec3(self.g6.z) * vec3(other.g1.y, -other.g1.x, -other.g0.y) - vec3(self.g6.w) * vec3(other.g6.x, other.g6.y, other.g6.z) + vec3(self.g7.x) * vec3(-other.g2.x, -other.g6.z, other.g6.y) + vec3(self.g7.y) * vec3(other.g6.z, -other.g2.x, -other.g6.x) + vec3(self.g7.z) * vec3(-other.g6.y, other.g6.x, -other.g2.x) + vec3(self.g9.x) * vec3(-other.g10.x, -other.g3.z, other.g3.y) + vec3(self.g9.y) * vec3(other.g3.z, -other.g10.x, -other.g3.x) + vec3(self.g9.z) * vec3(-other.g3.y, other.g3.x, -other.g10.x) + vec3(self.g10.x) * other.g9 + vec3(self.g10.x) * other.g4, vec3(self.g0.x) * other.g4 - vec3(self.g0.y) * other.g7 + vec3(self.g1.x) * vec3(-other.g6.w, -other.g1.z, other.g1.y) + vec3(self.g1.y) * vec3(other.g1.z, -other.g6.w, -other.g1.x) + vec3(self.g1.z) * vec3(-other.g1.y, other.g1.x, -other.g6.w) - vec3(self.g2.x) * other.g8 - vec3(self.g2.y) * vec3(other.g6.x, other.g6.y, other.g6.z) + vec3(self.g3.x) * vec3(other.g10.y, other.g5.z, -other.g5.y) + vec3(self.g3.y) * vec3(-other.g5.z, other.g10.y, other.g5.x) + vec3(self.g3.z) * vec3(other.g5.y, -other.g5.x, other.g10.y) + vec3(self.g4.x) * vec3(other.g0.x, other.g4.z, -other.g4.y) + vec3(self.g4.y) * vec3(-other.g4.z, other.g0.x, other.g4.x) + vec3(self.g4.z) * vec3(other.g4.y, -other.g4.x, other.g0.x) + vec3(self.g5.x) * vec3(other.g10.x, other.g3.z, -other.g3.y) + vec3(self.g5.y) * vec3(-other.g3.z, other.g10.x, other.g3.x) + vec3(self.g5.z) * vec3(other.g3.y, -other.g3.x, other.g10.x) + vec3(self.g5.w) * other.g9 + vec3(self.g6.x) * vec3(-other.g2.y, -other.g8.z, other.g8.y) + vec3(self.g6.y) * vec3(other.g8.z, -other.g2.y, -other.g8.x) + vec3(self.g6.z) * vec3(-other.g8.y, other.g8.x, -other.g2.y) - vec3(self.g6.w) * other.g1 + vec3(self.g7.x) * vec3(-other.g0.y, -other.g7.z, other.g7.y) + vec3(self.g7.y) * vec3(other.g7.z, -other.g0.y, -other.g7.x) + vec3(self.g7.z) * vec3(-other.g7.y, other.g7.x, -other.g0.y) + vec3(self.g8.x) * vec3(-other.g2.x, -other.g6.z, other.g6.y) + vec3(self.g8.y) * vec3(other.g6.z, -other.g2.x, -other.g6.x) + vec3(self.g8.z) * vec3(-other.g6.y, other.g6.x, -other.g2.x) + vec3(self.g9.x) * vec3(other.g5.w, other.g9.z, -other.g9.y) + vec3(self.g9.y) * vec3(-other.g9.z, other.g5.w, other.g9.x) + vec3(self.g9.z) * vec3(other.g9.y, -other.g9.x, other.g5.w) + vec3(self.g10.x) * vec3(other.g5.x, other.g5.y, other.g5.z) + vec3(self.g10.y) * other.g3, vec4(self.g0.x) * other.g5 + vec4(self.g0.y) * vec4(-other.g8.x, -other.g8.y, -other.g8.z, other.g6.w) + vec4(self.g1.x) * vec4(other.g2.y, other.g8.z, -other.g8.y, -other.g7.x) + vec4(self.g1.y) * vec4(-other.g8.z, other.g2.y, other.g8.x, -other.g7.y) + vec4(self.g1.z) * vec4(other.g8.y, -other.g8.x, other.g2.y, -other.g7.z) + vec4(self.g2.x) * vec4(0.0, 0.0, 0.0, other.g2.y) - vec4(self.g2.y) * vec4(other.g7.x, other.g7.y, other.g7.z, other.g2.x) + vec4(self.g2.y) * vec4(-other.g1.x, -other.g1.y, -other.g1.z, 0.0) + vec4(self.g3.x) * vec4(0.0, 0.0, 0.0, other.g5.x) + vec4(self.g3.y) * vec4(0.0, 0.0, 0.0, other.g5.y) + vec4(self.g3.z) * vec4(0.0, 0.0, 0.0, other.g5.z) + vec4(self.g4.x) * vec4(other.g10.y, other.g5.z, -other.g5.y, -other.g9.x) + vec4(self.g4.y) * vec4(-other.g5.z, other.g10.y, other.g5.x, -other.g9.y) + vec4(self.g4.z) * vec4(other.g5.y, -other.g5.x, other.g10.y, -other.g9.z) + vec4(self.g5.x) * vec4(-other.g5.w, -other.g9.z, other.g9.y, -other.g3.x) + vec4(self.g5.x) * vec4(other.g0.x, other.g4.z, -other.g4.y, 0.0) + vec4(self.g5.y) * vec4(other.g9.z, -other.g5.w, -other.g9.x, -other.g3.y) + vec4(self.g5.y) * vec4(-other.g4.z, other.g0.x, other.g4.x, 0.0) + vec4(self.g5.z) * vec4(-other.g9.y, other.g9.x, -other.g5.w, -other.g3.z) + vec4(self.g5.z) * vec4(other.g4.y, -other.g4.x, other.g0.x, 0.0) + vec4(self.g5.w) * vec4(other.g5.x, other.g5.y, other.g5.z, other.g0.x) + vec4(self.g6.x) * vec4(0.0, 0.0, 0.0, -other.g8.x) + vec4(self.g6.y) * vec4(0.0, 0.0, 0.0, -other.g8.y) + vec4(self.g6.z) * vec4(0.0, 0.0, 0.0, -other.g8.z) + vec4(self.g6.w) * vec4(other.g8.x, other.g8.y, other.g8.z, other.g0.y) + vec4(self.g7.x) * vec4(-other.g2.y, -other.g8.z, other.g8.y, -other.g1.x) + vec4(self.g7.y) * vec4(other.g8.z, -other.g2.y, -other.g8.x, -other.g1.y) + vec4(self.g7.z) * vec4(-other.g8.y, other.g8.x, -other.g2.y, -other.g1.z) + vec4(self.g8.x) * vec4(-other.g6.w, -other.g7.z, other.g7.y, other.g6.x) + vec4(self.g8.x) * vec4(-other.g0.y, -other.g1.z, other.g1.y, 0.0) + vec4(self.g8.y) * vec4(other.g7.z, -other.g6.w, -other.g7.x, other.g6.y) + vec4(self.g8.y) * vec4(other.g1.z, -other.g0.y, -other.g1.x, 0.0) + vec4(self.g8.z) * vec4(-other.g7.y, other.g7.x, -other.g6.w, other.g6.z) + vec4(self.g8.z) * vec4(-other.g1.y, other.g1.x, -other.g0.y, 0.0) + vec4(self.g9.x) * vec4(other.g10.y, other.g5.z, -other.g5.y, -other.g4.x) + vec4(self.g9.y) * vec4(-other.g5.z, other.g10.y, other.g5.x, -other.g4.y) + vec4(self.g9.z) * vec4(other.g5.y, -other.g5.x, other.g10.y, -other.g4.z) + vec4(self.g10.x) * vec4(0.0, 0.0, 0.0, -other.g10.y) + vec4(self.g10.y) * vec4(-other.g9.x, -other.g9.y, -other.g9.z, other.g10.x) + vec4(self.g10.y) * vec4(other.g4.x, other.g4.y, other.g4.z, 0.0), vec4(self.g0.x) * other.g6 + vec4(self.g0.y) * vec4(other.g3.x, other.g3.y, other.g3.z, -other.g5.w) + vec4(self.g1.x) * vec4(other.g10.x, other.g3.z, -other.g3.y, -other.g4.x) + vec4(self.g1.y) * vec4(-other.g3.z, other.g10.x, other.g3.x, -other.g4.y) + vec4(self.g1.z) * vec4(other.g3.y, -other.g3.x, other.g10.x, -other.g4.z) + vec4(self.g2.x) * vec4(other.g9.x, other.g9.y, other.g9.z, other.g10.y) + vec4(self.g2.x) * vec4(other.g4.x, other.g4.y, other.g4.z, 0.0) + vec4(self.g2.y) * vec4(0.0, 0.0, 0.0, -other.g10.x) + vec4(self.g3.x) * vec4(-other.g6.w, other.g7.z, -other.g7.y, -other.g8.x) + vec4(self.g3.x) * vec4(other.g0.y, -other.g1.z, other.g1.y, 0.0) + vec4(self.g3.y) * vec4(-other.g7.z, -other.g6.w, other.g7.x, -other.g8.y) + vec4(self.g3.y) * vec4(other.g1.z, other.g0.y, -other.g1.x, 0.0) + vec4(self.g3.z) * vec4(other.g7.y, -other.g7.x, -other.g6.w, -other.g8.z) + vec4(self.g3.z) * vec4(-other.g1.y, other.g1.x, other.g0.y, 0.0) + vec4(self.g4.x) * vec4(other.g2.x, other.g6.z, -other.g6.y, -other.g1.x) + vec4(self.g4.y) * vec4(-other.g6.z, other.g2.x, other.g6.x, -other.g1.y) + vec4(self.g4.z) * vec4(other.g6.y, -other.g6.x, other.g2.x, -other.g1.z) + vec4(self.g5.x) * vec4(0.0, 0.0, 0.0, other.g6.x) + vec4(self.g5.y) * vec4(0.0, 0.0, 0.0, other.g6.y) + vec4(self.g5.z) * vec4(0.0, 0.0, 0.0, other.g6.z) - vec4(self.g5.w) * vec4(other.g6.x, other.g6.y, other.g6.z, other.g0.y) + vec4(self.g6.x) * vec4(other.g5.w, other.g9.z, -other.g9.y, -other.g5.x) + vec4(self.g6.x) * vec4(other.g0.x, other.g4.z, -other.g4.y, 0.0) + vec4(self.g6.y) * vec4(-other.g9.z, other.g5.w, other.g9.x, -other.g5.y) + vec4(self.g6.y) * vec4(-other.g4.z, other.g0.x, other.g4.x, 0.0) + vec4(self.g6.z) * vec4(other.g9.y, -other.g9.x, other.g5.w, -other.g5.z) + vec4(self.g6.z) * vec4(other.g4.y, -other.g4.x, other.g0.x, 0.0) + vec4(self.g6.w) * vec4(other.g3.x, other.g3.y, other.g3.z, other.g0.x) + vec4(self.g7.x) * vec4(other.g10.x, other.g3.z, -other.g3.y, other.g9.x) + vec4(self.g7.y) * vec4(-other.g3.z, other.g10.x, other.g3.x, other.g9.y) + vec4(self.g7.z) * vec4(other.g3.y, -other.g3.x, other.g10.x, other.g9.z) + vec4(self.g8.x) * vec4(0.0, 0.0, 0.0, other.g3.x) + vec4(self.g8.y) * vec4(0.0, 0.0, 0.0, other.g3.y) + vec4(self.g8.z) * vec4(0.0, 0.0, 0.0, other.g3.z) + vec4(self.g9.x) * vec4(-other.g2.x, -other.g6.z, other.g6.y, other.g7.x) + vec4(self.g9.y) * vec4(other.g6.z, -other.g2.x, -other.g6.x, other.g7.y) + vec4(self.g9.z) * vec4(-other.g6.y, other.g6.x, -other.g2.x, other.g7.z) + vec4(self.g10.x) * vec4(other.g7.x, other.g7.y, other.g7.z, other.g2.y) + vec4(self.g10.x) * vec4(-other.g1.x, -other.g1.y, -other.g1.z, 0.0) + vec4(self.g10.y) * vec4(0.0, 0.0, 0.0, -other.g2.x), vec3(self.g0.x) * other.g7 + vec3(self.g0.y) * other.g4 + vec3(self.g1.x) * vec3(-other.g5.w, -other.g9.z, other.g9.y) + vec3(self.g1.y) * vec3(other.g9.z, -other.g5.w, -other.g9.x) + vec3(self.g1.z) * vec3(-other.g9.y, other.g9.x, -other.g5.w) + vec3(self.g2.x) * vec3(other.g5.x, other.g5.y, other.g5.z) + vec3(self.g2.y) * other.g3 + vec3(self.g3.x) * vec3(other.g2.y, other.g8.z, -other.g8.y) + vec3(self.g3.y) * vec3(-other.g8.z, other.g2.y, other.g8.x) + vec3(self.g3.z) * vec3(other.g8.y, -other.g8.x, other.g2.y) + vec3(self.g4.x) * vec3(other.g0.y, other.g7.z, -other.g7.y) + vec3(self.g4.y) * vec3(-other.g7.z, other.g0.y, other.g7.x) + vec3(self.g4.z) * vec3(other.g7.y, -other.g7.x, other.g0.y) + vec3(self.g5.x) * vec3(other.g2.x, other.g6.z, -other.g6.y) + vec3(self.g5.y) * vec3(-other.g6.z, other.g2.x, other.g6.x) + vec3(self.g5.z) * vec3(other.g6.y, -other.g6.x, other.g2.x) - vec3(self.g5.w) * other.g1 + vec3(self.g6.x) * vec3(other.g10.y, other.g5.z, -other.g5.y) + vec3(self.g6.y) * vec3(-other.g5.z, other.g10.y, other.g5.x) + vec3(self.g6.z) * vec3(other.g5.y, -other.g5.x, other.g10.y) - vec3(self.g6.w) * other.g9 + vec3(self.g7.x) * vec3(other.g0.x, other.g4.z, -other.g4.y) + vec3(self.g7.y) * vec3(-other.g4.z, other.g0.x, other.g4.x) + vec3(self.g7.z) * vec3(other.g4.y, -other.g4.x, other.g0.x) + vec3(self.g8.x) * vec3(other.g10.x, other.g3.z, -other.g3.y) + vec3(self.g8.y) * vec3(-other.g3.z, other.g10.x, other.g3.x) + vec3(self.g8.z) * vec3(other.g3.y, -other.g3.x, other.g10.x) + vec3(self.g9.x) * vec3(-other.g6.w, -other.g1.z, other.g1.y) + vec3(self.g9.y) * vec3(other.g1.z, -other.g6.w, -other.g1.x) + vec3(self.g9.z) * vec3(-other.g1.y, other.g1.x, -other.g6.w) + vec3(self.g10.x) * other.g8 + vec3(self.g10.y) * vec3(other.g6.x, other.g6.y, other.g6.z), vec3(self.g0.x) * other.g8 + vec3(self.g0.y) * vec3(other.g5.x, other.g5.y, other.g5.z) + vec3(self.g1.x) * vec3(-other.g10.y, -other.g5.z, other.g5.y) + vec3(self.g1.y) * vec3(other.g5.z, -other.g10.y, -other.g5.x) + vec3(self.g1.z) * vec3(-other.g5.y, other.g5.x, -other.g10.y) - vec3(self.g2.y) * other.g9 + vec3(self.g2.y) * other.g4 + vec3(self.g4.x) * vec3(other.g2.y, other.g8.z, -other.g8.y) + vec3(self.g4.y) * vec3(-other.g8.z, other.g2.y, other.g8.x) + vec3(self.g4.z) * vec3(other.g8.y, -other.g8.x, other.g2.y) + vec3(self.g5.x) * vec3(other.g6.w, other.g7.z, -other.g7.y) + vec3(self.g5.x) * vec3(other.g0.y, other.g1.z, -other.g1.y) + vec3(self.g5.y) * vec3(-other.g7.z, other.g6.w, other.g7.x) + vec3(self.g5.y) * vec3(-other.g1.z, other.g0.y, other.g1.x) + vec3(self.g5.z) * vec3(other.g7.y, -other.g7.x, other.g6.w) + vec3(self.g5.z) * vec3(other.g1.y, -other.g1.x, other.g0.y) + vec3(self.g5.w) * other.g8 - vec3(self.g6.w) * vec3(other.g5.x, other.g5.y, other.g5.z) + vec3(self.g7.x) * vec3(other.g10.y, other.g5.z, -other.g5.y) + vec3(self.g7.y) * vec3(-other.g5.z, other.g10.y, other.g5.x) + vec3(self.g7.z) * vec3(other.g5.y, -other.g5.x, other.g10.y) + vec3(self.g8.x) * vec3(-other.g5.w, -other.g9.z, other.g9.y) + vec3(self.g8.x) * vec3(other.g0.x, other.g4.z, -other.g4.y) + vec3(self.g8.y) * vec3(other.g9.z, -other.g5.w, -other.g9.x) + vec3(self.g8.y) * vec3(-other.g4.z, other.g0.x, other.g4.x) + vec3(self.g8.z) * vec3(-other.g9.y, other.g9.x, -other.g5.w) + vec3(self.g8.z) * vec3(other.g4.y, -other.g4.x, other.g0.x) + vec3(self.g9.x) * vec3(other.g2.y, other.g8.z, -other.g8.y) + vec3(self.g9.y) * vec3(-other.g8.z, other.g2.y, other.g8.x) + vec3(self.g9.z) * vec3(other.g8.y, -other.g8.x, other.g2.y) + vec3(self.g10.y) * other.g7 + vec3(self.g10.y) * other.g1, vec3(self.g0.x) * other.g9 + vec3(self.g0.y) * other.g1 + vec3(self.g1.x) * vec3(other.g0.y, other.g7.z, -other.g7.y) + vec3(self.g1.y) * vec3(-other.g7.z, other.g0.y, other.g7.x) + vec3(self.g1.z) * vec3(other.g7.y, -other.g7.x, other.g0.y) + vec3(self.g2.x) * other.g8 - vec3(self.g2.y) * vec3(other.g6.x, other.g6.y, other.g6.z) + vec3(self.g3.x) * vec3(-other.g10.y, -other.g5.z, other.g5.y) + vec3(self.g3.y) * vec3(other.g5.z, -other.g10.y, -other.g5.x) + vec3(self.g3.z) * vec3(-other.g5.y, other.g5.x, -other.g10.y) + vec3(self.g4.x) * vec3(other.g5.w, other.g9.z, -other.g9.y) + vec3(self.g4.y) * vec3(-other.g9.z, other.g5.w, other.g9.x) + vec3(self.g4.z) * vec3(other.g9.y, -other.g9.x, other.g5.w) + vec3(self.g5.x) * vec3(other.g10.x, other.g3.z, -other.g3.y) + vec3(self.g5.y) * vec3(-other.g3.z, other.g10.x, other.g3.x) + vec3(self.g5.z) * vec3(other.g3.y, -other.g3.x, other.g10.x) + vec3(self.g5.w) * other.g4 + vec3(self.g6.x) * vec3(other.g2.y, other.g8.z, -other.g8.y) + vec3(self.g6.y) * vec3(-other.g8.z, other.g2.y, other.g8.x) + vec3(self.g6.z) * vec3(other.g8.y, -other.g8.x, other.g2.y) + vec3(self.g6.w) * other.g7 + vec3(self.g7.x) * vec3(other.g6.w, other.g1.z, -other.g1.y) + vec3(self.g7.y) * vec3(-other.g1.z, other.g6.w, other.g1.x) + vec3(self.g7.z) * vec3(other.g1.y, -other.g1.x, other.g6.w) + vec3(self.g8.x) * vec3(-other.g2.x, -other.g6.z, other.g6.y) + vec3(self.g8.y) * vec3(other.g6.z, -other.g2.x, -other.g6.x) + vec3(self.g8.z) * vec3(-other.g6.y, other.g6.x, -other.g2.x) + vec3(self.g9.x) * vec3(other.g0.x, other.g4.z, -other.g4.y) + vec3(self.g9.y) * vec3(-other.g4.z, other.g0.x, other.g4.x) + vec3(self.g9.z) * vec3(other.g4.y, -other.g4.x, other.g0.x) - vec3(self.g10.x) * vec3(other.g5.x, other.g5.y, other.g5.z) + vec3(self.g10.y) * other.g3, vec2(self.g0.x) * other.g10 - vec2(self.g0.y) * other.g2 + vec2(self.g1.x) * vec2(other.g6.x, -other.g8.x) + vec2(self.g1.y) * vec2(other.g6.y, -other.g8.y) + vec2(self.g1.z) * vec2(other.g6.z, -other.g8.z) + self.g2 * vec2(other.g6.w) - self.g2 * vec2(other.g0.y) + vec2(self.g3.x) * vec2(-other.g9.x, 0.0) + vec2(self.g3.x) * vec2(-other.g4.x, 0.0) + vec2(self.g3.y) * vec2(-other.g9.y, 0.0) + vec2(self.g3.y) * vec2(-other.g4.y, 0.0) + vec2(self.g3.z) * vec2(-other.g9.z, 0.0) + vec2(self.g3.z) * vec2(-other.g4.z, 0.0) - vec2(self.g4.x) * vec2(other.g3.x, other.g5.x) - vec2(self.g4.y) * vec2(other.g3.y, other.g5.y) - vec2(self.g4.z) * vec2(other.g3.z, other.g5.z) + vec2(self.g5.x) * vec2(0.0, other.g9.x) + vec2(self.g5.x) * vec2(0.0, -other.g4.x) + vec2(self.g5.y) * vec2(0.0, other.g9.y) + vec2(self.g5.y) * vec2(0.0, -other.g4.y) + vec2(self.g5.z) * vec2(0.0, other.g9.z) + vec2(self.g5.z) * vec2(0.0, -other.g4.z) + vec2(self.g5.w) * other.g10 * vec2(-1.0, 1.0) + vec2(self.g6.x) * vec2(other.g7.x, 0.0) + vec2(self.g6.x) * vec2(-other.g1.x, 0.0) + vec2(self.g6.y) * vec2(other.g7.y, 0.0) + vec2(self.g6.y) * vec2(-other.g1.y, 0.0) + vec2(self.g6.z) * vec2(other.g7.z, 0.0) + vec2(self.g6.z) * vec2(-other.g1.z, 0.0) + vec2(self.g6.w) * other.g2 * vec2(-1.0, 1.0) + vec2(self.g7.x) * vec2(other.g6.x, other.g8.x) + vec2(self.g7.y) * vec2(other.g6.y, other.g8.y) + vec2(self.g7.z) * vec2(other.g6.z, other.g8.z) + vec2(self.g8.x) * vec2(0.0, other.g7.x) + vec2(self.g8.x) * vec2(0.0, other.g1.x) + vec2(self.g8.y) * vec2(0.0, other.g7.y) + vec2(self.g8.y) * vec2(0.0, other.g1.y) + vec2(self.g8.z) * vec2(0.0, other.g7.z) + vec2(self.g8.z) * vec2(0.0, other.g1.z) + vec2(self.g9.x) * vec2(other.g3.x, -other.g5.x) + vec2(self.g9.y) * vec2(other.g3.y, -other.g5.y) + vec2(self.g9.z) * vec2(other.g3.z, -other.g5.z) + self.g10 * vec2(other.g5.w) + self.g10 * vec2(other.g0.x));
}

MultiVector multi_vector__wedge_dot__plane(MultiVector self, Plane other) {
    return MultiVector(vec2(self.g1.x) * vec2(0.0, other.g0.x) + vec2(self.g1.y) * vec2(0.0, other.g0.y) + vec2(self.g1.z) * vec2(0.0, other.g0.z) + vec2(self.g2.x) * vec2(0.0, other.g0.w) + vec2(self.g9.x) * vec2(-other.g0.x, 0.0) + vec2(self.g9.y) * vec2(-other.g0.y, 0.0) + vec2(self.g9.z) * vec2(-other.g0.z, 0.0) + vec2(self.g10.x) * vec2(other.g0.w, 0.0), vec3(0.0) - vec3(self.g0.y) * vec3(other.g0.x, other.g0.y, other.g0.z) + vec3(self.g6.x, self.g6.y, self.g6.z) * vec3(other.g0.w) + vec3(self.g7.x) * vec3(0.0, -other.g0.z, other.g0.y) + vec3(self.g7.y) * vec3(other.g0.z, 0.0, -other.g0.x) + vec3(self.g7.z) * vec3(-other.g0.y, other.g0.x, 0.0), vec2(self.g0.y) * vec2(0.0, other.g0.w) + vec2(self.g6.x) * vec2(-other.g0.x, 0.0) + vec2(self.g6.y) * vec2(-other.g0.y, 0.0) - vec2(self.g6.z, self.g6.w) * vec2(other.g0.z, other.g0.w) + vec2(self.g8.x) * vec2(0.0, other.g0.x) + vec2(self.g8.y) * vec2(0.0, other.g0.y) + vec2(self.g8.z) * vec2(0.0, other.g0.z), vec3(self.g3.x) * vec3(0.0, other.g0.z, -other.g0.y) + vec3(self.g3.y) * vec3(-other.g0.z, 0.0, other.g0.x) + vec3(self.g3.z) * vec3(other.g0.y, -other.g0.x, 0.0) + vec3(self.g10.x) * vec3(other.g0.x, other.g0.y, other.g0.z), self.g3 * vec3(other.g0.w) + vec3(self.g5.w) * vec3(other.g0.x, other.g0.y, other.g0.z) + vec3(self.g9.x) * vec3(0.0, other.g0.z, -other.g0.y) + vec3(self.g9.y) * vec3(-other.g0.z, 0.0, other.g0.x) + vec3(self.g9.z) * vec3(other.g0.y, -other.g0.x, 0.0), vec4(self.g4.x) * vec4(other.g0.w, 0.0, 0.0, -other.g0.x) + vec4(self.g4.y) * vec4(0.0, other.g0.w, 0.0, -other.g0.y) + vec4(self.g4.z) * vec4(0.0, 0.0, other.g0.w, -other.g0.z) + vec4(self.g5.x) * vec4(0.0, -other.g0.z, other.g0.y, 0.0) + vec4(self.g5.y) * vec4(other.g0.z, 0.0, -other.g0.x, 0.0) + vec4(self.g5.z) * vec4(-other.g0.y, other.g0.x, 0.0, 0.0) + vec4(self.g9.x, self.g9.y, self.g9.z, self.g9.x) * vec4(other.g0.w, other.g0.w, other.g0.w, 0.0) - vec4(self.g10.y, self.g10.y, self.g10.y, self.g10.x) * other.g0, vec4(self.g2.x) * other.g0 + vec4(self.g6.x) * vec4(0.0, other.g0.z, -other.g0.y, 0.0) + vec4(self.g6.y) * vec4(-other.g0.z, 0.0, other.g0.x, 0.0) + vec4(self.g6.z) * vec4(other.g0.y, -other.g0.x, 0.0, 0.0) + vec4(self.g7.x) * vec4(0.0, 0.0, 0.0, other.g0.x) + vec4(self.g7.y) * vec4(0.0, 0.0, 0.0, other.g0.y) + vec4(self.g7.z) * vec4(0.0, 0.0, 0.0, other.g0.z), vec3(self.g1.x) * vec3(0.0, -other.g0.z, other.g0.y) + vec3(self.g1.y) * vec3(other.g0.z, 0.0, -other.g0.x) + vec3(self.g1.z) * vec3(-other.g0.y, other.g0.x, 0.0) + vec3(self.g6.x, self.g6.y, self.g6.z) * vec3(other.g0.w) - vec3(self.g6.w) * vec3(other.g0.x, other.g0.y, other.g0.z), vec3(0.0) - self.g1 * vec3(other.g0.w) - vec3(self.g2.y) * vec3(other.g0.x, other.g0.y, other.g0.z) + self.g7 * vec3(other.g0.w) + vec3(self.g8.x) * vec3(0.0, -other.g0.z, other.g0.y) + vec3(self.g8.y) * vec3(other.g0.z, 0.0, -other.g0.x) + vec3(self.g8.z) * vec3(-other.g0.y, other.g0.x, 0.0), vec3(self.g0.x) * vec3(other.g0.x, other.g0.y, other.g0.z) - self.g3 * vec3(other.g0.w) + vec3(self.g4.x) * vec3(0.0, other.g0.z, -other.g0.y) + vec3(self.g4.y) * vec3(-other.g0.z, 0.0, other.g0.x) + vec3(self.g4.z) * vec3(other.g0.y, -other.g0.x, 0.0), vec2(self.g0.x) * vec2(0.0, other.g0.w) + vec2(self.g3.x) * vec2(-other.g0.x, 0.0) + vec2(self.g3.y) * vec2(-other.g0.y, 0.0) + vec2(self.g3.z) * vec2(-other.g0.z, 0.0) + vec2(self.g5.x) * vec2(0.0, other.g0.x) + vec2(self.g5.y) * vec2(0.0, other.g0.y) + vec2(self.g5.z) * vec2(0.0, other.g0.z) + vec2(self.g5.w) * vec2(0.0, other.g0.w));
}

MultiVector multi_vector__wedge_dot__round_point(MultiVector self, RoundPoint other) {
    return MultiVector(vec2(self.g1.x) * vec2(other.g0.x, 0.0) + vec2(self.g1.y) * vec2(other.g0.y, 0.0) + vec2(self.g1.z) * vec2(other.g0.z, 0.0) + vec2(self.g2.x) * vec2(-other.g1.y, 0.0) + vec2(self.g2.y) * vec2(-other.g1.x, 0.0) + vec2(self.g9.x) * vec2(0.0, other.g0.x) + vec2(self.g9.y) * vec2(0.0, other.g0.y) + vec2(self.g9.z) * vec2(0.0, other.g0.z) + vec2(self.g10.x) * vec2(0.0, other.g1.y) + vec2(self.g10.y) * vec2(0.0, other.g1.x), vec3(self.g0.x) * other.g0 + self.g3 * vec3(other.g1.y) + vec3(self.g4.x) * vec3(0.0, other.g0.z, -other.g0.y) + vec3(self.g4.y) * vec3(-other.g0.z, 0.0, other.g0.x) + vec3(self.g4.z) * vec3(other.g0.y, -other.g0.x, 0.0) - vec3(self.g5.x, self.g5.y, self.g5.z) * vec3(other.g1.x), vec2(self.g0.x) * other.g1 + vec2(self.g3.x) * vec2(other.g0.x, 0.0) + vec2(self.g3.y) * vec2(other.g0.y, 0.0) + vec2(self.g3.z) * vec2(other.g0.z, 0.0) + vec2(self.g5.x) * vec2(0.0, -other.g0.x) + vec2(self.g5.y) * vec2(0.0, -other.g0.y) + vec2(self.g5.z) * vec2(0.0, -other.g0.z) + vec2(self.g5.w) * other.g1 * vec2(-1.0, 1.0), vec3(0.0) - self.g1 * vec3(other.g1.x) + vec3(self.g2.x) * other.g0 + vec3(self.g6.x) * vec3(0.0, other.g0.z, -other.g0.y) + vec3(self.g6.y) * vec3(-other.g0.z, 0.0, other.g0.x) + vec3(self.g6.z) * vec3(other.g0.y, -other.g0.x, 0.0) - self.g7 * vec3(other.g1.x), vec3(self.g1.x) * vec3(0.0, -other.g0.z, other.g0.y) + vec3(self.g1.y) * vec3(other.g0.z, 0.0, -other.g0.x) + vec3(self.g1.z) * vec3(-other.g0.y, other.g0.x, 0.0) - vec3(self.g6.x, self.g6.y, self.g6.z) * vec3(other.g1.y) - vec3(self.g6.w) * other.g0 - self.g8 * vec3(other.g1.x), vec4(self.g1.x, self.g1.y, self.g1.z, self.g1.x) * vec4(other.g1.y, other.g1.y, other.g1.y, 0.0) + vec4(self.g2.x) * vec4(0.0, 0.0, 0.0, other.g1.y) - vec4(self.g2.y) * vec4(other.g0.x, other.g0.y, other.g0.z, other.g1.x) + vec4(self.g7.x) * vec4(-other.g1.y, 0.0, 0.0, -other.g0.x) + vec4(self.g7.y) * vec4(0.0, -other.g1.y, 0.0, -other.g0.y) + vec4(self.g7.z) * vec4(0.0, 0.0, -other.g1.y, -other.g0.z) + vec4(self.g8.x) * vec4(0.0, -other.g0.z, other.g0.y, 0.0) + vec4(self.g8.y) * vec4(other.g0.z, 0.0, -other.g0.x, 0.0) + vec4(self.g8.z) * vec4(-other.g0.y, other.g0.x, 0.0, 0.0), vec4(self.g3.x) * vec4(0.0, -other.g0.z, other.g0.y, 0.0) + vec4(self.g3.y) * vec4(other.g0.z, 0.0, -other.g0.x, 0.0) + vec4(self.g3.z) * vec4(-other.g0.y, other.g0.x, 0.0, 0.0) + vec4(self.g4.x) * vec4(other.g1.x, 0.0, 0.0, -other.g0.x) + vec4(self.g4.y) * vec4(0.0, other.g1.x, 0.0, -other.g0.y) + vec4(self.g4.z) * vec4(0.0, 0.0, other.g1.x, -other.g0.z) + vec4(self.g9.x, self.g9.y, self.g9.z, self.g9.x) * vec4(-other.g1.x, -other.g1.x, -other.g1.x, 0.0) + vec4(self.g10.x) * vec4(-other.g0.x, -other.g0.y, -other.g0.z, other.g1.y) + vec4(self.g10.y) * vec4(0.0, 0.0, 0.0, -other.g1.x), self.g3 * vec3(other.g1.y) + vec3(self.g5.x, self.g5.y, self.g5.z) * vec3(other.g1.x) - vec3(self.g5.w) * other.g0 + vec3(self.g9.x) * vec3(0.0, -other.g0.z, other.g0.y) + vec3(self.g9.y) * vec3(other.g0.z, 0.0, -other.g0.x) + vec3(self.g9.z) * vec3(-other.g0.y, other.g0.x, 0.0), self.g4 * vec3(other.g1.y) + vec3(self.g5.x) * vec3(0.0, other.g0.z, -other.g0.y) + vec3(self.g5.y) * vec3(-other.g0.z, 0.0, other.g0.x) + vec3(self.g5.z) * vec3(other.g0.y, -other.g0.x, 0.0) + self.g9 * vec3(other.g1.y) + vec3(self.g10.y) * other.g0, vec3(self.g0.y) * other.g0 + vec3(self.g6.x, self.g6.y, self.g6.z) * vec3(other.g1.y) + vec3(self.g7.x) * vec3(0.0, other.g0.z, -other.g0.y) + vec3(self.g7.y) * vec3(-other.g0.z, 0.0, other.g0.x) + vec3(self.g7.z) * vec3(other.g0.y, -other.g0.x, 0.0) - self.g8 * vec3(other.g1.x), vec2(0.0) - vec2(self.g0.y) * other.g1 + vec2(self.g6.x) * vec2(-other.g0.x, 0.0) + vec2(self.g6.y) * vec2(-other.g0.y, 0.0) + vec2(self.g6.z) * vec2(-other.g0.z, 0.0) + vec2(self.g6.w) * other.g1 * vec2(-1.0, 1.0) + vec2(self.g8.x) * vec2(0.0, other.g0.x) + vec2(self.g8.y) * vec2(0.0, other.g0.y) + vec2(self.g8.z) * vec2(0.0, other.g0.z));
}

MultiVector multi_vector__wedge_dot__scalar(MultiVector self, Scalar other) {
    return MultiVector(self.g0 * vec2(other.g0), self.g1 * vec3(other.g0), self.g2 * vec2(other.g0), self.g3 * vec3(other.g0), self.g4 * vec3(other.g0), self.g5 * vec4(other.g0), self.g6 * vec4(other.g0), self.g7 * vec3(other.g0), self.g8 * vec3(other.g0), self.g9 * vec3(other.g0), self.g10 * vec2(other.g0));
}

MultiVector multi_vector__wedge_dot__sphere(MultiVector self, Sphere other) {
    return MultiVector(vec2(self.g1.x) * vec2(0.0, other.g0.x) + vec2(self.g1.y) * vec2(0.0, other.g0.y) + vec2(self.g1.z) * vec2(0.0, other.g0.z) + vec2(self.g2.x) * vec2(0.0, other.g1.y) + vec2(self.g2.y) * vec2(0.0, other.g1.x) + vec2(self.g9.x) * vec2(-other.g0.x, 0.0) + vec2(self.g9.y) * vec2(-other.g0.y, 0.0) + vec2(self.g9.z) * vec2(-other.g0.z, 0.0) + vec2(self.g10.x) * vec2(other.g1.y, 0.0) + vec2(self.g10.y) * vec2(other.g1.x, 0.0), vec3(0.0) - vec3(self.g0.y) * other.g0 + vec3(self.g6.x, self.g6.y, self.g6.z) * vec3(other.g1.y) + vec3(self.g7.x) * vec3(0.0, -other.g0.z, other.g0.y) + vec3(self.g7.y) * vec3(other.g0.z, 0.0, -other.g0.x) + vec3(self.g7.z) * vec3(-other.g0.y, other.g0.x, 0.0) - self.g8 * vec3(other.g1.x), vec2(self.g0.y) * other.g1 + vec2(self.g6.x) * vec2(-other.g0.x, 0.0) + vec2(self.g6.y) * vec2(-other.g0.y, 0.0) + vec2(self.g6.z) * vec2(-other.g0.z, 0.0) + vec2(self.g6.w) * other.g1 * vec2(1.0, -1.0) + vec2(self.g8.x) * vec2(0.0, other.g0.x) + vec2(self.g8.y) * vec2(0.0, other.g0.y) + vec2(self.g8.z) * vec2(0.0, other.g0.z), vec3(self.g3.x) * vec3(0.0, other.g0.z, -other.g0.y) + vec3(self.g3.y) * vec3(-other.g0.z, 0.0, other.g0.x) + vec3(self.g3.z) * vec3(other.g0.y, -other.g0.x, 0.0) + self.g4 * vec3(other.g1.x) - self.g9 * vec3(other.g1.x) + vec3(self.g10.x) * other.g0, self.g3 * vec3(other.g1.y) + vec3(self.g5.x, self.g5.y, self.g5.z) * vec3(other.g1.x) + vec3(self.g5.w) * other.g0 + vec3(self.g9.x) * vec3(0.0, other.g0.z, -other.g0.y) + vec3(self.g9.y) * vec3(-other.g0.z, 0.0, other.g0.x) + vec3(self.g9.z) * vec3(other.g0.y, -other.g0.x, 0.0), vec4(self.g4.x) * vec4(other.g1.y, 0.0, 0.0, -other.g0.x) + vec4(self.g4.y) * vec4(0.0, other.g1.y, 0.0, -other.g0.y) + vec4(self.g4.z) * vec4(0.0, 0.0, other.g1.y, -other.g0.z) + vec4(self.g5.x) * vec4(0.0, -other.g0.z, other.g0.y, 0.0) + vec4(self.g5.y) * vec4(other.g0.z, 0.0, -other.g0.x, 0.0) + vec4(self.g5.z) * vec4(-other.g0.y, other.g0.x, 0.0, 0.0) + vec4(self.g9.x, self.g9.y, self.g9.z, self.g9.x) * vec4(other.g1.y, other.g1.y, other.g1.y, 0.0) + vec4(self.g10.x) * vec4(0.0, 0.0, 0.0, -other.g1.y) + vec4(self.g10.y) * vec4(-other.g0.x, -other.g0.y, -other.g0.z, other.g1.x), vec4(self.g1.x, self.g1.y, self.g1.z, self.g1.x) * vec4(other.g1.x, other.g1.x, other.g1.x, 0.0) + vec4(self.g2.x) * vec4(other.g0.x, other.g0.y, other.g0.z, other.g1.y) + vec4(self.g2.y) * vec4(0.0, 0.0, 0.0, -other.g1.x) + vec4(self.g6.x) * vec4(0.0, other.g0.z, -other.g0.y, 0.0) + vec4(self.g6.y) * vec4(-other.g0.z, 0.0, other.g0.x, 0.0) + vec4(self.g6.z) * vec4(other.g0.y, -other.g0.x, 0.0, 0.0) + vec4(self.g7.x) * vec4(other.g1.x, 0.0, 0.0, other.g0.x) + vec4(self.g7.y) * vec4(0.0, other.g1.x, 0.0, other.g0.y) + vec4(self.g7.z) * vec4(0.0, 0.0, other.g1.x, other.g0.z), vec3(self.g1.x) * vec3(0.0, -other.g0.z, other.g0.y) + vec3(self.g1.y) * vec3(other.g0.z, 0.0, -other.g0.x) + vec3(self.g1.z) * vec3(-other.g0.y, other.g0.x, 0.0) + vec3(self.g6.x, self.g6.y, self.g6.z) * vec3(other.g1.y) - vec3(self.g6.w) * other.g0 + self.g8 * vec3(other.g1.x), vec3(0.0) - self.g1 * vec3(other.g1.y) - vec3(self.g2.y) * other.g0 + self.g7 * vec3(other.g1.y) + vec3(self.g8.x) * vec3(0.0, -other.g0.z, other.g0.y) + vec3(self.g8.y) * vec3(other.g0.z, 0.0, -other.g0.x) + vec3(self.g8.z) * vec3(-other.g0.y, other.g0.x, 0.0), vec3(self.g0.x) * other.g0 - self.g3 * vec3(other.g1.y) + vec3(self.g4.x) * vec3(0.0, other.g0.z, -other.g0.y) + vec3(self.g4.y) * vec3(-other.g0.z, 0.0, other.g0.x) + vec3(self.g4.z) * vec3(other.g0.y, -other.g0.x, 0.0) + vec3(self.g5.x, self.g5.y, self.g5.z) * vec3(other.g1.x), vec2(self.g0.x) * other.g1 + vec2(self.g3.x) * vec2(-other.g0.x, 0.0) + vec2(self.g3.y) * vec2(-other.g0.y, 0.0) + vec2(self.g3.z) * vec2(-other.g0.z, 0.0) + vec2(self.g5.x) * vec2(0.0, other.g0.x) + vec2(self.g5.y) * vec2(0.0, other.g0.y) + vec2(self.g5.z) * vec2(0.0, other.g0.z) + vec2(self.g5.w) * other.g1 * vec2(-1.0, 1.0));
}

RoundPoint plane__wedge_dot__anti_scalar(Plane self, AntiScalar other) {
    return RoundPoint(vec3(0.0) - vec3(self.g0.x, self.g0.y, self.g0.z) * vec3(other.g0), vec2(self.g0.w) * vec2(0.0, other.g0));
}

MultiVector plane__wedge_dot__circle(Plane self, Circle other) {
    return MultiVector(vec2(0.0), vec3(self.g0.x) * vec3(0.0, -other.g1.z, other.g1.y) + vec3(self.g0.y) * vec3(other.g1.z, 0.0, -other.g1.x) + vec3(self.g0.z) * vec3(-other.g1.y, other.g1.x, 0.0) - vec3(self.g0.w) * vec3(other.g0.x, other.g0.y, other.g0.z), vec2(self.g0.x) * vec2(other.g0.x, -other.g2.x) + vec2(self.g0.y) * vec2(other.g0.y, -other.g2.y) + vec2(self.g0.z) * vec2(other.g0.z, -other.g2.z) + vec2(self.g0.w) * vec2(0.0, other.g0.w), vec3(0.0), vec3(0.0), vec4(0.0), vec4(self.g0.x) * vec4(0.0, -other.g0.z, other.g0.y, other.g1.x) + vec4(self.g0.y) * vec4(other.g0.z, 0.0, -other.g0.x, other.g1.y) + vec4(self.g0.z) * vec4(-other.g0.y, other.g0.x, 0.0, other.g1.z), vec3(0.0) - vec3(self.g0.x, self.g0.y, self.g0.z) * vec3(other.g0.w) + vec3(self.g0.w) * vec3(other.g0.x, other.g0.y, other.g0.z), vec3(self.g0.x) * vec3(0.0, other.g2.z, -other.g2.y) + vec3(self.g0.y) * vec3(-other.g2.z, 0.0, other.g2.x) + vec3(self.g0.z) * vec3(other.g2.y, -other.g2.x, 0.0) + vec3(self.g0.w) * other.g1, vec3(0.0), vec2(0.0));
}

MultiVector plane__wedge_dot__dipole(Plane self, Dipole other) {
    return MultiVector(vec2(0.0), vec3(0.0), vec2(0.0), vec3(self.g0.x) * vec3(0.0, -other.g0.z, other.g0.y) + vec3(self.g0.y) * vec3(other.g0.z, 0.0, -other.g0.x) + vec3(self.g0.z) * vec3(-other.g0.y, other.g0.x, 0.0), vec3(self.g0.x, self.g0.y, self.g0.z) * vec3(other.g2.w) + vec3(self.g0.w) * other.g0, vec4(self.g0.x) * vec4(0.0, other.g2.z, -other.g2.y, -other.g1.x) + vec4(self.g0.y) * vec4(-other.g2.z, 0.0, other.g2.x, -other.g1.y) + vec4(self.g0.z) * vec4(other.g2.y, -other.g2.x, 0.0, -other.g1.z) + vec4(self.g0.w) * vec4(other.g1.x, other.g1.y, other.g1.z, 0.0), vec4(0.0), vec3(0.0), vec3(0.0), vec3(self.g0.x) * vec3(0.0, other.g1.z, -other.g1.y) + vec3(self.g0.y) * vec3(-other.g1.z, 0.0, other.g1.x) + vec3(self.g0.z) * vec3(other.g1.y, -other.g1.x, 0.0) + vec3(self.g0.w) * other.g0, vec2(self.g0.x) * vec2(other.g0.x, -other.g2.x) + vec2(self.g0.y) * vec2(other.g0.y, -other.g2.y) + vec2(self.g0.z) * vec2(other.g0.z, -other.g2.z) + vec2(self.g0.w) * vec2(0.0, -other.g2.w));
}

MultiVector plane__wedge_dot__dual_num(Plane self, DualNum other) {
    return MultiVector(vec2(0.0), vec3(0.0) - vec3(self.g0.x, self.g0.y, self.g0.z) * vec3(other.g0.y), vec2(self.g0.w) * vec2(0.0, other.g0.y), vec3(0.0), vec3(0.0), vec4(0.0), vec4(0.0), vec3(0.0), vec3(0.0), vec3(self.g0.x, self.g0.y, self.g0.z) * vec3(other.g0.x), vec2(self.g0.w) * vec2(0.0, other.g0.x));
}

MultiVector plane__wedge_dot__flat_point(Plane self, FlatPoint other) {
    return MultiVector(vec2(0.0), vec3(0.0), vec2(0.0), vec3(0.0), vec3(self.g0.x, self.g0.y, self.g0.z) * vec3(other.g0.w), vec4(self.g0.x) * vec4(0.0, other.g0.z, -other.g0.y, 0.0) + vec4(self.g0.y) * vec4(-other.g0.z, 0.0, other.g0.x, 0.0) + vec4(self.g0.z) * vec4(other.g0.y, -other.g0.x, 0.0, 0.0), vec4(0.0), vec3(0.0), vec3(0.0), vec3(0.0), vec2(self.g0.x) * vec2(0.0, -other.g0.x) + vec2(self.g0.y) * vec2(0.0, -other.g0.y) + vec2(self.g0.z) * vec2(0.0, -other.g0.z) + vec2(self.g0.w) * vec2(0.0, -other.g0.w));
}

MultiVector plane__wedge_dot__flector(Plane self, Flector other) {
    return MultiVector(vec2(self.g0.x) * vec2(-other.g1.x, 0.0) + vec2(self.g0.y) * vec2(-other.g1.y, 0.0) + vec2(self.g0.z) * vec2(-other.g1.z, 0.0), vec3(0.0), vec2(0.0), vec3(0.0), vec3(self.g0.x) * vec3(other.g0.w, other.g1.z, -other.g1.y) + vec3(self.g0.y) * vec3(-other.g1.z, other.g0.w, other.g1.x) + vec3(self.g0.z) * vec3(other.g1.y, -other.g1.x, other.g0.w), vec4(self.g0.x) * vec4(other.g1.w, other.g0.z, -other.g0.y, 0.0) + vec4(self.g0.y) * vec4(-other.g0.z, other.g1.w, other.g0.x, 0.0) + vec4(self.g0.z) * vec4(other.g0.y, -other.g0.x, other.g1.w, 0.0) + vec4(self.g0.w) * vec4(-other.g1.x, -other.g1.y, -other.g1.z, 0.0), vec4(0.0), vec3(0.0), vec3(0.0), vec3(0.0), vec2(self.g0.x) * vec2(0.0, -other.g0.x) + vec2(self.g0.y) * vec2(0.0, -other.g0.y) + vec2(self.g0.z) * vec2(0.0, -other.g0.z) + vec2(self.g0.w) * vec2(0.0, -other.g0.w));
}

MultiVector plane__wedge_dot__line(Plane self, Line other) {
    return MultiVector(vec2(0.0), vec3(self.g0.x) * vec3(0.0, -other.g0.z, other.g0.y) + vec3(self.g0.y) * vec3(other.g0.z, 0.0, -other.g0.x) + vec3(self.g0.z) * vec3(-other.g0.y, other.g0.x, 0.0), vec2(self.g0.x) * vec2(0.0, -other.g1.x) + vec2(self.g0.y) * vec2(0.0, -other.g1.y) + vec2(self.g0.z) * vec2(0.0, -other.g1.z), vec3(0.0), vec3(0.0), vec4(0.0), vec4(self.g0.x) * vec4(0.0, 0.0, 0.0, other.g0.x) + vec4(self.g0.y) * vec4(0.0, 0.0, 0.0, other.g0.y) + vec4(self.g0.z) * vec4(0.0, 0.0, 0.0, other.g0.z), vec3(0.0), vec3(self.g0.x) * vec3(0.0, other.g1.z, -other.g1.y) + vec3(self.g0.y) * vec3(-other.g1.z, 0.0, other.g1.x) + vec3(self.g0.z) * vec3(other.g1.y, -other.g1.x, 0.0) + vec3(self.g0.w) * other.g0, vec3(0.0), vec2(0.0));
}

MultiVector plane__wedge_dot__motor(Plane self, Motor other) {
    return MultiVector(vec2(0.0), vec3(self.g0.x) * vec3(-other.g0.w, -other.g0.z, other.g0.y) + vec3(self.g0.y) * vec3(other.g0.z, -other.g0.w, -other.g0.x) + vec3(self.g0.z) * vec3(-other.g0.y, other.g0.x, -other.g0.w), vec2(self.g0.x) * vec2(0.0, -other.g1.x) + vec2(self.g0.y) * vec2(0.0, -other.g1.y) + vec2(self.g0.z) * vec2(0.0, -other.g1.z) + vec2(self.g0.w) * vec2(0.0, other.g0.w), vec3(0.0), vec3(0.0), vec4(0.0), vec4(self.g0.x) * vec4(0.0, 0.0, 0.0, other.g0.x) + vec4(self.g0.y) * vec4(0.0, 0.0, 0.0, other.g0.y) + vec4(self.g0.z) * vec4(0.0, 0.0, 0.0, other.g0.z), vec3(0.0), vec3(self.g0.x) * vec3(0.0, other.g1.z, -other.g1.y) + vec3(self.g0.y) * vec3(-other.g1.z, 0.0, other.g1.x) + vec3(self.g0.z) * vec3(other.g1.y, -other.g1.x, 0.0) + vec3(self.g0.w) * vec3(other.g0.x, other.g0.y, other.g0.z), vec3(0.0), vec2(0.0));
}

MultiVector plane__wedge_dot__multi_vector(Plane self, MultiVector other) {
    return MultiVector(vec2(self.g0.x) * vec2(-other.g9.x, other.g1.x) + vec2(self.g0.y) * vec2(-other.g9.y, other.g1.y) + vec2(self.g0.z) * vec2(-other.g9.z, other.g1.z) + vec2(self.g0.w) * vec2(other.g10.x, other.g2.x), vec3(self.g0.x) * vec3(-other.g0.y, -other.g7.z, other.g7.y) + vec3(self.g0.y) * vec3(other.g7.z, -other.g0.y, -other.g7.x) + vec3(self.g0.z) * vec3(-other.g7.y, other.g7.x, -other.g0.y) - vec3(self.g0.w) * vec3(other.g6.x, other.g6.y, other.g6.z), vec2(self.g0.x) * vec2(other.g6.x, -other.g8.x) + vec2(self.g0.y) * vec2(other.g6.y, -other.g8.y) + vec2(self.g0.z) * vec2(other.g6.z, -other.g8.z) + vec2(self.g0.w) * vec2(0.0, other.g6.w) + vec2(self.g0.w) * vec2(0.0, other.g0.y), vec3(self.g0.x) * vec3(-other.g10.x, -other.g3.z, other.g3.y) + vec3(self.g0.y) * vec3(other.g3.z, -other.g10.x, -other.g3.x) + vec3(self.g0.z) * vec3(-other.g3.y, other.g3.x, -other.g10.x), vec3(self.g0.x) * vec3(other.g5.w, other.g9.z, -other.g9.y) + vec3(self.g0.y) * vec3(-other.g9.z, other.g5.w, other.g9.x) + vec3(self.g0.z) * vec3(other.g9.y, -other.g9.x, other.g5.w) + vec3(self.g0.w) * other.g3, vec4(self.g0.x) * vec4(other.g10.y, other.g5.z, -other.g5.y, -other.g4.x) + vec4(self.g0.y) * vec4(-other.g5.z, other.g10.y, other.g5.x, -other.g4.y) + vec4(self.g0.z) * vec4(other.g5.y, -other.g5.x, other.g10.y, -other.g4.z) + vec4(self.g0.w) * vec4(-other.g9.x, -other.g9.y, -other.g9.z, other.g10.x) + vec4(self.g0.w) * vec4(other.g4.x, other.g4.y, other.g4.z, 0.0), vec4(self.g0.x) * vec4(-other.g2.x, -other.g6.z, other.g6.y, other.g7.x) + vec4(self.g0.y) * vec4(other.g6.z, -other.g2.x, -other.g6.x, other.g7.y) + vec4(self.g0.z) * vec4(-other.g6.y, other.g6.x, -other.g2.x, other.g7.z) + vec4(self.g0.w) * vec4(0.0, 0.0, 0.0, -other.g2.x), vec3(self.g0.x) * vec3(-other.g6.w, -other.g1.z, other.g1.y) + vec3(self.g0.y) * vec3(other.g1.z, -other.g6.w, -other.g1.x) + vec3(self.g0.z) * vec3(-other.g1.y, other.g1.x, -other.g6.w) + vec3(self.g0.w) * vec3(other.g6.x, other.g6.y, other.g6.z), vec3(self.g0.x) * vec3(other.g2.y, other.g8.z, -other.g8.y) + vec3(self.g0.y) * vec3(-other.g8.z, other.g2.y, other.g8.x) + vec3(self.g0.z) * vec3(other.g8.y, -other.g8.x, other.g2.y) + vec3(self.g0.w) * other.g7 + vec3(self.g0.w) * other.g1, vec3(self.g0.x) * vec3(other.g0.x, other.g4.z, -other.g4.y) + vec3(self.g0.y) * vec3(-other.g4.z, other.g0.x, other.g4.x) + vec3(self.g0.z) * vec3(other.g4.y, -other.g4.x, other.g0.x) + vec3(self.g0.w) * other.g3, vec2(self.g0.x) * vec2(other.g3.x, -other.g5.x) + vec2(self.g0.y) * vec2(other.g3.y, -other.g5.y) + vec2(self.g0.z) * vec2(other.g3.z, -other.g5.z) + vec2(self.g0.w) * vec2(0.0, -other.g5.w) + vec2(self.g0.w) * vec2(0.0, other.g0.x));
}

MultiVector plane__wedge_dot__plane(Plane self, Plane other) {
    return MultiVector(vec2(self.g0.x) * vec2(-other.g0.x, 0.0) + vec2(self.g0.y) * vec2(-other.g0.y, 0.0) + vec2(self.g0.z) * vec2(-other.g0.z, 0.0), vec3(0.0), vec2(0.0), vec3(0.0), vec3(self.g0.x) * vec3(0.0, other.g0.z, -other.g0.y) + vec3(self.g0.y) * vec3(-other.g0.z, 0.0, other.g0.x) + vec3(self.g0.z) * vec3(other.g0.y, -other.g0.x, 0.0), self.g0.xyzx * vec4(other.g0.w, other.g0.w, other.g0.w, 0.0) + vec4(self.g0.w) * vec4(-other.g0.x, -other.g0.y, -other.g0.z, 0.0), vec4(0.0), vec3(0.0), vec3(0.0), vec3(0.0), vec2(0.0));
}

MultiVector plane__wedge_dot__round_point(Plane self, RoundPoint other) {
    return MultiVector(vec2(self.g0.x) * vec2(0.0, other.g0.x) + vec2(self.g0.y) * vec2(0.0, other.g0.y) + vec2(self.g0.z) * vec2(0.0, other.g0.z) + vec2(self.g0.w) * vec2(0.0, other.g1.x), vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0), vec4(0.0), vec4(0.0) - self.g0 * vec4(other.g1.x), vec3(self.g0.x) * vec3(0.0, -other.g0.z, other.g0.y) + vec3(self.g0.y) * vec3(other.g0.z, 0.0, -other.g0.x) + vec3(self.g0.z) * vec3(-other.g0.y, other.g0.x, 0.0), vec3(self.g0.x, self.g0.y, self.g0.z) * vec3(other.g1.y) + vec3(self.g0.w) * other.g0, vec3(0.0), vec2(0.0));
}

Plane plane__wedge_dot__scalar(Plane self, Scalar other) {
    return Plane(self.g0 * vec4(other.g0));
}

MultiVector plane__wedge_dot__sphere(Plane self, Sphere other) {
    return MultiVector(vec2(self.g0.x) * vec2(-other.g0.x, 0.0) + vec2(self.g0.y) * vec2(-other.g0.y, 0.0) + vec2(self.g0.z) * vec2(-other.g0.z, 0.0) + vec2(self.g0.w) * vec2(other.g1.x, 0.0), vec3(0.0), vec2(0.0), vec3(0.0) - vec3(self.g0.x, self.g0.y, self.g0.z) * vec3(other.g1.x), vec3(self.g0.x) * vec3(0.0, other.g0.z, -other.g0.y) + vec3(self.g0.y) * vec3(-other.g0.z, 0.0, other.g0.x) + vec3(self.g0.z) * vec3(other.g0.y, -other.g0.x, 0.0), self.g0.xyzx * vec4(other.g1.y, other.g1.y, other.g1.y, 0.0) + vec4(self.g0.w) * vec4(-other.g0.x, -other.g0.y, -other.g0.z, other.g1.x), vec4(0.0), vec3(0.0), vec3(0.0), vec3(0.0), vec2(0.0));
}

Sphere round_point__wedge_dot__anti_scalar(RoundPoint self, AntiScalar other) {
    return Sphere(self.g0 * vec3(other.g0), vec2(0.0) - self.g1 * vec2(other.g0));
}

MultiVector round_point__wedge_dot__circle(RoundPoint self, Circle other) {
    return MultiVector(vec2(0.0), vec3(0.0), vec2(0.0), vec3(self.g0.x) * vec3(0.0, -other.g0.z, other.g0.y) + vec3(self.g0.y) * vec3(other.g0.z, 0.0, -other.g0.x) + vec3(self.g0.z) * vec3(-other.g0.y, other.g0.x, 0.0) - vec3(self.g1.x) * other.g1, vec3(0.0) - self.g0 * vec3(other.g0.w) - vec3(self.g1.x) * other.g2 - vec3(self.g1.y) * vec3(other.g0.x, other.g0.y, other.g0.z), vec4(self.g0.x) * vec4(0.0, other.g2.z, -other.g2.y, -other.g1.x) + vec4(self.g0.y) * vec4(-other.g2.z, 0.0, other.g2.x, -other.g1.y) + vec4(self.g0.z) * vec4(other.g2.y, -other.g2.x, 0.0, -other.g1.z) + vec4(self.g1.y) * vec4(-other.g1.x, -other.g1.y, -other.g1.z, 0.0), vec4(0.0), vec3(0.0), vec3(0.0), vec3(self.g0.x) * vec3(0.0, other.g1.z, -other.g1.y) + vec3(self.g0.y) * vec3(-other.g1.z, 0.0, other.g1.x) + vec3(self.g0.z) * vec3(other.g1.y, -other.g1.x, 0.0) + vec3(self.g1.x) * other.g2 - vec3(self.g1.y) * vec3(other.g0.x, other.g0.y, other.g0.z), vec2(self.g0.x) * vec2(other.g0.x, -other.g2.x) + vec2(self.g0.y) * vec2(other.g0.y, -other.g2.y) + vec2(self.g0.z) * vec2(other.g0.z, -other.g2.z) + self.g1 * vec2(other.g0.w));
}

MultiVector round_point__wedge_dot__dipole(RoundPoint self, Dipole other) {
    return MultiVector(vec2(0.0), vec3(self.g0.x) * vec3(0.0, other.g1.z, -other.g1.y) + vec3(self.g0.y) * vec3(-other.g1.z, 0.0, other.g1.x) + vec3(self.g0.z) * vec3(other.g1.y, -other.g1.x, 0.0) + vec3(self.g1.x) * vec3(other.g2.x, other.g2.y, other.g2.z) - vec3(self.g1.y) * other.g0, vec2(self.g0.x) * vec2(-other.g0.x, other.g2.x) + vec2(self.g0.y) * vec2(-other.g0.y, other.g2.y) + vec2(self.g0.z) * vec2(-other.g0.z, other.g2.z) + self.g1 * vec2(other.g2.w), vec3(0.0), vec3(0.0), vec4(0.0), vec4(self.g0.x) * vec4(0.0, other.g0.z, -other.g0.y, -other.g1.x) + vec4(self.g0.y) * vec4(-other.g0.z, 0.0, other.g0.x, -other.g1.y) + vec4(self.g0.z) * vec4(other.g0.y, -other.g0.x, 0.0, -other.g1.z) + vec4(self.g1.x) * vec4(other.g1.x, other.g1.y, other.g1.z, 0.0), vec3(0.0) - self.g0 * vec3(other.g2.w) + vec3(self.g1.x) * vec3(other.g2.x, other.g2.y, other.g2.z) + vec3(self.g1.y) * other.g0, vec3(self.g0.x) * vec3(0.0, -other.g2.z, other.g2.y) + vec3(self.g0.y) * vec3(other.g2.z, 0.0, -other.g2.x) + vec3(self.g0.z) * vec3(-other.g2.y, other.g2.x, 0.0) + vec3(self.g1.y) * other.g1, vec3(0.0), vec2(0.0));
}

MultiVector round_point__wedge_dot__dual_num(RoundPoint self, DualNum other) {
    return MultiVector(vec2(0.0), self.g0 * vec3(other.g0.x), self.g1 * vec2(other.g0.x), vec3(0.0), vec3(0.0), vec4(0.0), vec4(0.0), vec3(0.0), vec3(0.0), self.g0 * vec3(other.g0.y), vec2(0.0) - self.g1 * vec2(other.g0.y));
}

MultiVector round_point__wedge_dot__flat_point(RoundPoint self, FlatPoint other) {
    return MultiVector(vec2(0.0), vec3(self.g1.x) * vec3(other.g0.x, other.g0.y, other.g0.z), vec2(self.g0.x) * vec2(0.0, other.g0.x) + vec2(self.g0.y) * vec2(0.0, other.g0.y) + vec2(self.g0.z) * vec2(0.0, other.g0.z) + self.g1 * vec2(other.g0.w), vec3(0.0), vec3(0.0), vec4(0.0), vec4(0.0), vec3(0.0) - self.g0 * vec3(other.g0.w) + vec3(self.g1.x) * vec3(other.g0.x, other.g0.y, other.g0.z), vec3(self.g0.x) * vec3(0.0, -other.g0.z, other.g0.y) + vec3(self.g0.y) * vec3(other.g0.z, 0.0, -other.g0.x) + vec3(self.g0.z) * vec3(-other.g0.y, other.g0.x, 0.0), vec3(0.0), vec2(0.0));
}

MultiVector round_point__wedge_dot__flector(RoundPoint self, Flector other) {
    return MultiVector(vec2(self.g0.x) * vec2(0.0, other.g1.x) + vec2(self.g0.y) * vec2(0.0, other.g1.y) + vec2(self.g0.z) * vec2(0.0, other.g1.z) + vec2(self.g1.x) * vec2(0.0, other.g1.w), vec3(self.g1.x) * vec3(other.g0.x, other.g0.y, other.g0.z), vec2(self.g0.x) * vec2(0.0, other.g0.x) + vec2(self.g0.y) * vec2(0.0, other.g0.y) + vec2(self.g0.z) * vec2(0.0, other.g0.z) + self.g1 * vec2(other.g0.w), vec3(0.0), vec3(0.0), vec4(0.0), vec4(self.g1.x) * other.g1, vec3(self.g0.x) * vec3(-other.g0.w, -other.g1.z, other.g1.y) + vec3(self.g0.y) * vec3(other.g1.z, -other.g0.w, -other.g1.x) + vec3(self.g0.z) * vec3(-other.g1.y, other.g1.x, -other.g0.w) + vec3(self.g1.x) * vec3(other.g0.x, other.g0.y, other.g0.z), vec3(self.g0.x) * vec3(-other.g1.w, -other.g0.z, other.g0.y) + vec3(self.g0.y) * vec3(other.g0.z, -other.g1.w, -other.g0.x) + vec3(self.g0.z) * vec3(-other.g0.y, other.g0.x, -other.g1.w) - vec3(self.g1.y) * vec3(other.g1.x, other.g1.y, other.g1.z), vec3(0.0), vec2(0.0));
}

MultiVector round_point__wedge_dot__line(RoundPoint self, Line other) {
    return MultiVector(vec2(0.0), vec3(0.0), vec2(0.0), vec3(0.0) - vec3(self.g1.x) * other.g0, vec3(0.0) - vec3(self.g1.x) * other.g1, vec4(self.g0.x) * vec4(0.0, other.g1.z, -other.g1.y, -other.g0.x) + vec4(self.g0.y) * vec4(-other.g1.z, 0.0, other.g1.x, -other.g0.y) + vec4(self.g0.z) * vec4(other.g1.y, -other.g1.x, 0.0, -other.g0.z) + vec4(self.g1.y) * vec4(-other.g0.x, -other.g0.y, -other.g0.z, 0.0), vec4(0.0), vec3(0.0), vec3(0.0), vec3(self.g0.x) * vec3(0.0, other.g0.z, -other.g0.y) + vec3(self.g0.y) * vec3(-other.g0.z, 0.0, other.g0.x) + vec3(self.g0.z) * vec3(other.g0.y, -other.g0.x, 0.0) + vec3(self.g1.x) * other.g1, vec2(self.g0.x) * vec2(0.0, -other.g1.x) + vec2(self.g0.y) * vec2(0.0, -other.g1.y) + vec2(self.g0.z) * vec2(0.0, -other.g1.z));
}

MultiVector round_point__wedge_dot__motor(RoundPoint self, Motor other) {
    return MultiVector(vec2(0.0), vec3(0.0), vec2(0.0), vec3(0.0) - vec3(self.g1.x) * vec3(other.g0.x, other.g0.y, other.g0.z), vec3(0.0) - vec3(self.g1.x) * other.g1, vec4(self.g0.x) * vec4(0.0, other.g1.z, -other.g1.y, -other.g0.x) + vec4(self.g0.y) * vec4(-other.g1.z, 0.0, other.g1.x, -other.g0.y) + vec4(self.g0.z) * vec4(other.g1.y, -other.g1.x, 0.0, -other.g0.z) + vec4(self.g1.y) * vec4(-other.g0.x, -other.g0.y, -other.g0.z, 0.0), vec4(0.0), vec3(0.0), vec3(0.0), vec3(self.g0.x) * vec3(other.g0.w, other.g0.z, -other.g0.y) + vec3(self.g0.y) * vec3(-other.g0.z, other.g0.w, other.g0.x) + vec3(self.g0.z) * vec3(other.g0.y, -other.g0.x, other.g0.w) + vec3(self.g1.x) * other.g1, vec2(self.g0.x) * vec2(0.0, -other.g1.x) + vec2(self.g0.y) * vec2(0.0, -other.g1.y) + vec2(self.g0.z) * vec2(0.0, -other.g1.z) - self.g1 * vec2(other.g0.w));
}

MultiVector round_point__wedge_dot__multi_vector(RoundPoint self, MultiVector other) {
    return MultiVector(vec2(self.g0.x) * vec2(other.g1.x, other.g9.x) + vec2(self.g0.y) * vec2(other.g1.y, other.g9.y) + vec2(self.g0.z) * vec2(other.g1.z, other.g9.z) + vec2(self.g1.x) * vec2(-other.g2.y, other.g10.y) + vec2(self.g1.y) * vec2(-other.g2.x, other.g10.x), vec3(self.g0.x) * vec3(other.g0.x, other.g4.z, -other.g4.y) + vec3(self.g0.y) * vec3(-other.g4.z, other.g0.x, other.g4.x) + vec3(self.g0.z) * vec3(other.g4.y, -other.g4.x, other.g0.x) + vec3(self.g1.x) * vec3(other.g5.x, other.g5.y, other.g5.z) - vec3(self.g1.y) * other.g3, vec2(self.g0.x) * vec2(-other.g3.x, other.g5.x) + vec2(self.g0.y) * vec2(-other.g3.y, other.g5.y) + vec2(self.g0.z) * vec2(-other.g3.z, other.g5.z) + self.g1 * vec2(other.g5.w) + self.g1 * vec2(other.g0.x), vec3(self.g0.x) * vec3(-other.g2.x, -other.g6.z, other.g6.y) + vec3(self.g0.y) * vec3(other.g6.z, -other.g2.x, -other.g6.x) + vec3(self.g0.z) * vec3(-other.g6.y, other.g6.x, -other.g2.x) - vec3(self.g1.x) * other.g7 + vec3(self.g1.x) * other.g1, vec3(self.g0.x) * vec3(-other.g6.w, -other.g1.z, other.g1.y) + vec3(self.g0.y) * vec3(other.g1.z, -other.g6.w, -other.g1.x) + vec3(self.g0.z) * vec3(-other.g1.y, other.g1.x, -other.g6.w) - vec3(self.g1.x) * other.g8 - vec3(self.g1.y) * vec3(other.g6.x, other.g6.y, other.g6.z), vec4(self.g0.x) * vec4(other.g2.y, other.g8.z, -other.g8.y, -other.g7.x) + vec4(self.g0.y) * vec4(-other.g8.z, other.g2.y, other.g8.x, -other.g7.y) + vec4(self.g0.z) * vec4(other.g8.y, -other.g8.x, other.g2.y, -other.g7.z) + vec4(self.g1.x) * vec4(0.0, 0.0, 0.0, other.g2.y) - vec4(self.g1.y) * vec4(other.g7.x, other.g7.y, other.g7.z, other.g2.x) + vec4(self.g1.y) * vec4(-other.g1.x, -other.g1.y, -other.g1.z, 0.0), vec4(self.g0.x) * vec4(other.g10.x, other.g3.z, -other.g3.y, -other.g4.x) + vec4(self.g0.y) * vec4(-other.g3.z, other.g10.x, other.g3.x, -other.g4.y) + vec4(self.g0.z) * vec4(other.g3.y, -other.g3.x, other.g10.x, -other.g4.z) + vec4(self.g1.x) * vec4(other.g9.x, other.g9.y, other.g9.z, other.g10.y) + vec4(self.g1.x) * vec4(other.g4.x, other.g4.y, other.g4.z, 0.0) + vec4(self.g1.y) * vec4(0.0, 0.0, 0.0, -other.g10.x), vec3(self.g0.x) * vec3(-other.g5.w, -other.g9.z, other.g9.y) + vec3(self.g0.y) * vec3(other.g9.z, -other.g5.w, -other.g9.x) + vec3(self.g0.z) * vec3(-other.g9.y, other.g9.x, -other.g5.w) + vec3(self.g1.x) * vec3(other.g5.x, other.g5.y, other.g5.z) + vec3(self.g1.y) * other.g3, vec3(self.g0.x) * vec3(-other.g10.y, -other.g5.z, other.g5.y) + vec3(self.g0.y) * vec3(other.g5.z, -other.g10.y, -other.g5.x) + vec3(self.g0.z) * vec3(-other.g5.y, other.g5.x, -other.g10.y) - vec3(self.g1.y) * other.g9 + vec3(self.g1.y) * other.g4, vec3(self.g0.x) * vec3(other.g0.y, other.g7.z, -other.g7.y) + vec3(self.g0.y) * vec3(-other.g7.z, other.g0.y, other.g7.x) + vec3(self.g0.z) * vec3(other.g7.y, -other.g7.x, other.g0.y) + vec3(self.g1.x) * other.g8 - vec3(self.g1.y) * vec3(other.g6.x, other.g6.y, other.g6.z), vec2(self.g0.x) * vec2(other.g6.x, -other.g8.x) + vec2(self.g0.y) * vec2(other.g6.y, -other.g8.y) + vec2(self.g0.z) * vec2(other.g6.z, -other.g8.z) + self.g1 * vec2(other.g6.w) - self.g1 * vec2(other.g0.y));
}

MultiVector round_point__wedge_dot__plane(RoundPoint self, Plane other) {
    return MultiVector(vec2(self.g0.x) * vec2(0.0, other.g0.x) + vec2(self.g0.y) * vec2(0.0, other.g0.y) + vec2(self.g0.z) * vec2(0.0, other.g0.z) + vec2(self.g1.x) * vec2(0.0, other.g0.w), vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0), vec4(0.0), vec4(self.g1.x) * other.g0, vec3(self.g0.x) * vec3(0.0, -other.g0.z, other.g0.y) + vec3(self.g0.y) * vec3(other.g0.z, 0.0, -other.g0.x) + vec3(self.g0.z) * vec3(-other.g0.y, other.g0.x, 0.0), vec3(0.0) - self.g0 * vec3(other.g0.w) - vec3(self.g1.y) * vec3(other.g0.x, other.g0.y, other.g0.z), vec3(0.0), vec2(0.0));
}

MultiVector round_point__wedge_dot__round_point(RoundPoint self, RoundPoint other) {
    return MultiVector(vec2(self.g0.x) * vec2(other.g0.x, 0.0) + vec2(self.g0.y) * vec2(other.g0.y, 0.0) + vec2(self.g0.z) * vec2(other.g0.z, 0.0) + vec2(self.g1.x) * vec2(-other.g1.y, 0.0) + vec2(self.g1.y) * vec2(-other.g1.x, 0.0), vec3(0.0), vec2(0.0), vec3(0.0) - self.g0 * vec3(other.g1.x) + vec3(self.g1.x) * other.g0, vec3(self.g0.x) * vec3(0.0, -other.g0.z, other.g0.y) + vec3(self.g0.y) * vec3(other.g0.z, 0.0, -other.g0.x) + vec3(self.g0.z) * vec3(-other.g0.y, other.g0.x, 0.0), vec4(self.g0.x, self.g0.y, self.g0.z, self.g0.x) * vec4(other.g1.y, other.g1.y, other.g1.y, 0.0) + vec4(self.g1.x) * vec4(0.0, 0.0, 0.0, other.g1.y) - vec4(self.g1.y) * vec4(other.g0.x, other.g0.y, other.g0.z, other.g1.x), vec4(0.0), vec3(0.0), vec3(0.0), vec3(0.0), vec2(0.0));
}

RoundPoint round_point__wedge_dot__scalar(RoundPoint self, Scalar other) {
    return RoundPoint(self.g0 * vec3(other.g0), self.g1 * vec2(other.g0));
}

MultiVector round_point__wedge_dot__sphere(RoundPoint self, Sphere other) {
    return MultiVector(vec2(self.g0.x) * vec2(0.0, other.g0.x) + vec2(self.g0.y) * vec2(0.0, other.g0.y) + vec2(self.g0.z) * vec2(0.0, other.g0.z) + vec2(self.g1.x) * vec2(0.0, other.g1.y) + vec2(self.g1.y) * vec2(0.0, other.g1.x), vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0), vec4(0.0), vec4(self.g0.x, self.g0.y, self.g0.z, self.g0.x) * vec4(other.g1.x, other.g1.x, other.g1.x, 0.0) + vec4(self.g1.x) * vec4(other.g0.x, other.g0.y, other.g0.z, other.g1.y) + vec4(self.g1.y) * vec4(0.0, 0.0, 0.0, -other.g1.x), vec3(self.g0.x) * vec3(0.0, -other.g0.z, other.g0.y) + vec3(self.g0.y) * vec3(other.g0.z, 0.0, -other.g0.x) + vec3(self.g0.z) * vec3(-other.g0.y, other.g0.x, 0.0), vec3(0.0) - self.g0 * vec3(other.g1.y) - vec3(self.g1.y) * other.g0, vec3(0.0), vec2(0.0));
}

AntiScalar scalar__wedge_dot__anti_scalar(Scalar self, AntiScalar other) {
    return AntiScalar(self.g0 * other.g0);
}

Circle scalar__wedge_dot__circle(Scalar self, Circle other) {
    return Circle(vec4(self.g0) * other.g0, vec3(self.g0) * other.g1, vec3(self.g0) * other.g2);
}

Dipole scalar__wedge_dot__dipole(Scalar self, Dipole other) {
    return Dipole(vec3(self.g0) * other.g0, vec3(self.g0) * other.g1, vec4(self.g0) * other.g2);
}

DualNum scalar__wedge_dot__dual_num(Scalar self, DualNum other) {
    return DualNum(vec2(self.g0) * other.g0);
}

FlatPoint scalar__wedge_dot__flat_point(Scalar self, FlatPoint other) {
    return FlatPoint(vec4(self.g0) * other.g0);
}

Flector scalar__wedge_dot__flector(Scalar self, Flector other) {
    return Flector(vec4(self.g0) * other.g0, vec4(self.g0) * other.g1);
}

Line scalar__wedge_dot__line(Scalar self, Line other) {
    return Line(vec3(self.g0) * other.g0, vec3(self.g0) * other.g1);
}

Motor scalar__wedge_dot__motor(Scalar self, Motor other) {
    return Motor(vec4(self.g0) * other.g0, vec3(self.g0) * other.g1);
}

MultiVector scalar__wedge_dot__multi_vector(Scalar self, MultiVector other) {
    return MultiVector(vec2(self.g0) * other.g0, vec3(self.g0) * other.g1, vec2(self.g0) * other.g2, vec3(self.g0) * other.g3, vec3(self.g0) * other.g4, vec4(self.g0) * other.g5, vec4(self.g0) * other.g6, vec3(self.g0) * other.g7, vec3(self.g0) * other.g8, vec3(self.g0) * other.g9, vec2(self.g0) * other.g10);
}

Plane scalar__wedge_dot__plane(Scalar self, Plane other) {
    return Plane(vec4(self.g0) * other.g0);
}

RoundPoint scalar__wedge_dot__round_point(Scalar self, RoundPoint other) {
    return RoundPoint(vec3(self.g0) * other.g0, vec2(self.g0) * other.g1);
}

Scalar scalar__wedge_dot__scalar(Scalar self, Scalar other) {
    return Scalar(self.g0 * other.g0);
}

Sphere scalar__wedge_dot__sphere(Scalar self, Sphere other) {
    return Sphere(vec3(self.g0) * other.g0, vec2(self.g0) * other.g1);
}

RoundPoint sphere__wedge_dot__anti_scalar(Sphere self, AntiScalar other) {
    return RoundPoint(vec3(0.0) - self.g0 * vec3(other.g0), self.g1 * vec2(other.g0));
}

MultiVector sphere__wedge_dot__circle(Sphere self, Circle other) {
    return MultiVector(vec2(0.0), vec3(self.g0.x) * vec3(0.0, -other.g1.z, other.g1.y) + vec3(self.g0.y) * vec3(other.g1.z, 0.0, -other.g1.x) + vec3(self.g0.z) * vec3(-other.g1.y, other.g1.x, 0.0) + vec3(self.g1.x) * other.g2 - vec3(self.g1.y) * vec3(other.g0.x, other.g0.y, other.g0.z), vec2(self.g0.x) * vec2(other.g0.x, -other.g2.x) + vec2(self.g0.y) * vec2(other.g0.y, -other.g2.y) + vec2(self.g0.z) * vec2(other.g0.z, -other.g2.z) + self.g1 * vec2(-other.g0.w), vec3(0.0), vec3(0.0), vec4(0.0), vec4(self.g0.x) * vec4(0.0, -other.g0.z, other.g0.y, other.g1.x) + vec4(self.g0.y) * vec4(other.g0.z, 0.0, -other.g0.x, other.g1.y) + vec4(self.g0.z) * vec4(-other.g0.y, other.g0.x, 0.0, other.g1.z) + vec4(self.g1.x) * vec4(other.g1.x, other.g1.y, other.g1.z, 0.0), vec3(0.0) - self.g0 * vec3(other.g0.w) + vec3(self.g1.x) * other.g2 + vec3(self.g1.y) * vec3(other.g0.x, other.g0.y, other.g0.z), vec3(self.g0.x) * vec3(0.0, other.g2.z, -other.g2.y) + vec3(self.g0.y) * vec3(-other.g2.z, 0.0, other.g2.x) + vec3(self.g0.z) * vec3(other.g2.y, -other.g2.x, 0.0) + vec3(self.g1.y) * other.g1, vec3(0.0), vec2(0.0));
}

MultiVector sphere__wedge_dot__dipole(Sphere self, Dipole other) {
    return MultiVector(vec2(0.0), vec3(0.0), vec2(0.0), vec3(self.g0.x) * vec3(0.0, -other.g0.z, other.g0.y) + vec3(self.g0.y) * vec3(other.g0.z, 0.0, -other.g0.x) + vec3(self.g0.z) * vec3(-other.g0.y, other.g0.x, 0.0) + vec3(self.g1.x) * other.g1, self.g0 * vec3(other.g2.w) + vec3(self.g1.x) * vec3(other.g2.x, other.g2.y, other.g2.z) + vec3(self.g1.y) * other.g0, vec4(self.g0.x) * vec4(0.0, other.g2.z, -other.g2.y, -other.g1.x) + vec4(self.g0.y) * vec4(-other.g2.z, 0.0, other.g2.x, -other.g1.y) + vec4(self.g0.z) * vec4(other.g2.y, -other.g2.x, 0.0, -other.g1.z) + vec4(self.g1.y) * vec4(other.g1.x, other.g1.y, other.g1.z, 0.0), vec4(0.0), vec3(0.0), vec3(0.0), vec3(self.g0.x) * vec3(0.0, other.g1.z, -other.g1.y) + vec3(self.g0.y) * vec3(-other.g1.z, 0.0, other.g1.x) + vec3(self.g0.z) * vec3(other.g1.y, -other.g1.x, 0.0) - vec3(self.g1.x) * vec3(other.g2.x, other.g2.y, other.g2.z) + vec3(self.g1.y) * other.g0, vec2(self.g0.x) * vec2(other.g0.x, -other.g2.x) + vec2(self.g0.y) * vec2(other.g0.y, -other.g2.y) + vec2(self.g0.z) * vec2(other.g0.z, -other.g2.z) + self.g1 * vec2(other.g2.w));
}

MultiVector sphere__wedge_dot__dual_num(Sphere self, DualNum other) {
    return MultiVector(vec2(0.0), vec3(0.0) - self.g0 * vec3(other.g0.y), self.g1 * vec2(other.g0.y), vec3(0.0), vec3(0.0), vec4(0.0), vec4(0.0), vec3(0.0), vec3(0.0), self.g0 * vec3(other.g0.x), self.g1 * vec2(other.g0.x));
}

MultiVector sphere__wedge_dot__flat_point(Sphere self, FlatPoint other) {
    return MultiVector(vec2(0.0), vec3(0.0), vec2(0.0), vec3(0.0), self.g0 * vec3(other.g0.w) + vec3(self.g1.x) * vec3(other.g0.x, other.g0.y, other.g0.z), vec4(self.g0.x) * vec4(0.0, other.g0.z, -other.g0.y, 0.0) + vec4(self.g0.y) * vec4(-other.g0.z, 0.0, other.g0.x, 0.0) + vec4(self.g0.z) * vec4(other.g0.y, -other.g0.x, 0.0, 0.0), vec4(0.0), vec3(0.0), vec3(0.0), vec3(0.0) - vec3(self.g1.x) * vec3(other.g0.x, other.g0.y, other.g0.z), vec2(self.g0.x) * vec2(0.0, -other.g0.x) + vec2(self.g0.y) * vec2(0.0, -other.g0.y) + vec2(self.g0.z) * vec2(0.0, -other.g0.z) + self.g1 * vec2(other.g0.w));
}

MultiVector sphere__wedge_dot__flector(Sphere self, Flector other) {
    return MultiVector(vec2(self.g0.x) * vec2(-other.g1.x, 0.0) + vec2(self.g0.y) * vec2(-other.g1.y, 0.0) + vec2(self.g0.z) * vec2(-other.g1.z, 0.0) + vec2(self.g1.x) * vec2(other.g1.w, 0.0), vec3(0.0), vec2(0.0), vec3(self.g1.x) * vec3(other.g1.x, other.g1.y, other.g1.z), vec3(self.g0.x) * vec3(other.g0.w, other.g1.z, -other.g1.y) + vec3(self.g0.y) * vec3(-other.g1.z, other.g0.w, other.g1.x) + vec3(self.g0.z) * vec3(other.g1.y, -other.g1.x, other.g0.w) + vec3(self.g1.x) * vec3(other.g0.x, other.g0.y, other.g0.z), vec4(self.g0.x) * vec4(other.g1.w, other.g0.z, -other.g0.y, 0.0) + vec4(self.g0.y) * vec4(-other.g0.z, other.g1.w, other.g0.x, 0.0) + vec4(self.g0.z) * vec4(other.g0.y, -other.g0.x, other.g1.w, 0.0) - vec4(self.g1.y, self.g1.y, self.g1.y, self.g1.x) * other.g1, vec4(0.0), vec3(0.0), vec3(0.0), vec3(0.0) - vec3(self.g1.x) * vec3(other.g0.x, other.g0.y, other.g0.z), vec2(self.g0.x) * vec2(0.0, -other.g0.x) + vec2(self.g0.y) * vec2(0.0, -other.g0.y) + vec2(self.g0.z) * vec2(0.0, -other.g0.z) + self.g1 * vec2(other.g0.w));
}

MultiVector sphere__wedge_dot__line(Sphere self, Line other) {
    return MultiVector(vec2(0.0), vec3(self.g0.x) * vec3(0.0, -other.g0.z, other.g0.y) + vec3(self.g0.y) * vec3(other.g0.z, 0.0, -other.g0.x) + vec3(self.g0.z) * vec3(-other.g0.y, other.g0.x, 0.0) + vec3(self.g1.x) * other.g1, vec2(self.g0.x) * vec2(0.0, -other.g1.x) + vec2(self.g0.y) * vec2(0.0, -other.g1.y) + vec2(self.g0.z) * vec2(0.0, -other.g1.z), vec3(0.0), vec3(0.0), vec4(0.0), vec4(self.g0.x) * vec4(0.0, 0.0, 0.0, other.g0.x) + vec4(self.g0.y) * vec4(0.0, 0.0, 0.0, other.g0.y) + vec4(self.g0.z) * vec4(0.0, 0.0, 0.0, other.g0.z) + vec4(self.g1.x) * vec4(other.g0.x, other.g0.y, other.g0.z, 0.0), vec3(self.g1.x) * other.g1, vec3(self.g0.x) * vec3(0.0, other.g1.z, -other.g1.y) + vec3(self.g0.y) * vec3(-other.g1.z, 0.0, other.g1.x) + vec3(self.g0.z) * vec3(other.g1.y, -other.g1.x, 0.0) + vec3(self.g1.y) * other.g0, vec3(0.0), vec2(0.0));
}

MultiVector sphere__wedge_dot__motor(Sphere self, Motor other) {
    return MultiVector(vec2(0.0), vec3(self.g0.x) * vec3(-other.g0.w, -other.g0.z, other.g0.y) + vec3(self.g0.y) * vec3(other.g0.z, -other.g0.w, -other.g0.x) + vec3(self.g0.z) * vec3(-other.g0.y, other.g0.x, -other.g0.w) + vec3(self.g1.x) * other.g1, vec2(self.g0.x) * vec2(0.0, -other.g1.x) + vec2(self.g0.y) * vec2(0.0, -other.g1.y) + vec2(self.g0.z) * vec2(0.0, -other.g1.z) + self.g1 * vec2(other.g0.w), vec3(0.0), vec3(0.0), vec4(0.0), vec4(self.g0.x) * vec4(0.0, 0.0, 0.0, other.g0.x) + vec4(self.g0.y) * vec4(0.0, 0.0, 0.0, other.g0.y) + vec4(self.g0.z) * vec4(0.0, 0.0, 0.0, other.g0.z) + vec4(self.g1.x) * vec4(other.g0.x, other.g0.y, other.g0.z, 0.0), vec3(self.g1.x) * other.g1, vec3(self.g0.x) * vec3(0.0, other.g1.z, -other.g1.y) + vec3(self.g0.y) * vec3(-other.g1.z, 0.0, other.g1.x) + vec3(self.g0.z) * vec3(other.g1.y, -other.g1.x, 0.0) + vec3(self.g1.y) * vec3(other.g0.x, other.g0.y, other.g0.z), vec3(0.0), vec2(0.0));
}

MultiVector sphere__wedge_dot__multi_vector(Sphere self, MultiVector other) {
    return MultiVector(vec2(self.g0.x) * vec2(-other.g9.x, other.g1.x) + vec2(self.g0.y) * vec2(-other.g9.y, other.g1.y) + vec2(self.g0.z) * vec2(-other.g9.z, other.g1.z) + vec2(self.g1.x) * vec2(other.g10.y, other.g2.y) + vec2(self.g1.y) * vec2(other.g10.x, other.g2.x), vec3(self.g0.x) * vec3(-other.g0.y, -other.g7.z, other.g7.y) + vec3(self.g0.y) * vec3(other.g7.z, -other.g0.y, -other.g7.x) + vec3(self.g0.z) * vec3(-other.g7.y, other.g7.x, -other.g0.y) + vec3(self.g1.x) * other.g8 - vec3(self.g1.y) * vec3(other.g6.x, other.g6.y, other.g6.z), vec2(self.g0.x) * vec2(other.g6.x, -other.g8.x) + vec2(self.g0.y) * vec2(other.g6.y, -other.g8.y) + vec2(self.g0.z) * vec2(other.g6.z, -other.g8.z) + self.g1 * vec2(-other.g6.w) + self.g1 * vec2(other.g0.y), vec3(self.g0.x) * vec3(-other.g10.x, -other.g3.z, other.g3.y) + vec3(self.g0.y) * vec3(other.g3.z, -other.g10.x, -other.g3.x) + vec3(self.g0.z) * vec3(-other.g3.y, other.g3.x, -other.g10.x) + vec3(self.g1.x) * other.g9 + vec3(self.g1.x) * other.g4, vec3(self.g0.x) * vec3(other.g5.w, other.g9.z, -other.g9.y) + vec3(self.g0.y) * vec3(-other.g9.z, other.g5.w, other.g9.x) + vec3(self.g0.z) * vec3(other.g9.y, -other.g9.x, other.g5.w) + vec3(self.g1.x) * vec3(other.g5.x, other.g5.y, other.g5.z) + vec3(self.g1.y) * other.g3, vec4(self.g0.x) * vec4(other.g10.y, other.g5.z, -other.g5.y, -other.g4.x) + vec4(self.g0.y) * vec4(-other.g5.z, other.g10.y, other.g5.x, -other.g4.y) + vec4(self.g0.z) * vec4(other.g5.y, -other.g5.x, other.g10.y, -other.g4.z) + vec4(self.g1.x) * vec4(0.0, 0.0, 0.0, -other.g10.y) + vec4(self.g1.y) * vec4(-other.g9.x, -other.g9.y, -other.g9.z, other.g10.x) + vec4(self.g1.y) * vec4(other.g4.x, other.g4.y, other.g4.z, 0.0), vec4(self.g0.x) * vec4(-other.g2.x, -other.g6.z, other.g6.y, other.g7.x) + vec4(self.g0.y) * vec4(other.g6.z, -other.g2.x, -other.g6.x, other.g7.y) + vec4(self.g0.z) * vec4(-other.g6.y, other.g6.x, -other.g2.x, other.g7.z) + vec4(self.g1.x) * vec4(other.g7.x, other.g7.y, other.g7.z, other.g2.y) + vec4(self.g1.x) * vec4(-other.g1.x, -other.g1.y, -other.g1.z, 0.0) + vec4(self.g1.y) * vec4(0.0, 0.0, 0.0, -other.g2.x), vec3(self.g0.x) * vec3(-other.g6.w, -other.g1.z, other.g1.y) + vec3(self.g0.y) * vec3(other.g1.z, -other.g6.w, -other.g1.x) + vec3(self.g0.z) * vec3(-other.g1.y, other.g1.x, -other.g6.w) + vec3(self.g1.x) * other.g8 + vec3(self.g1.y) * vec3(other.g6.x, other.g6.y, other.g6.z), vec3(self.g0.x) * vec3(other.g2.y, other.g8.z, -other.g8.y) + vec3(self.g0.y) * vec3(-other.g8.z, other.g2.y, other.g8.x) + vec3(self.g0.z) * vec3(other.g8.y, -other.g8.x, other.g2.y) + vec3(self.g1.y) * other.g7 + vec3(self.g1.y) * other.g1, vec3(self.g0.x) * vec3(other.g0.x, other.g4.z, -other.g4.y) + vec3(self.g0.y) * vec3(-other.g4.z, other.g0.x, other.g4.x) + vec3(self.g0.z) * vec3(other.g4.y, -other.g4.x, other.g0.x) - vec3(self.g1.x) * vec3(other.g5.x, other.g5.y, other.g5.z) + vec3(self.g1.y) * other.g3, vec2(self.g0.x) * vec2(other.g3.x, -other.g5.x) + vec2(self.g0.y) * vec2(other.g3.y, -other.g5.y) + vec2(self.g0.z) * vec2(other.g3.z, -other.g5.z) + self.g1 * vec2(other.g5.w) + self.g1 * vec2(other.g0.x));
}

MultiVector sphere__wedge_dot__plane(Sphere self, Plane other) {
    return MultiVector(vec2(self.g0.x) * vec2(-other.g0.x, 0.0) + vec2(self.g0.y) * vec2(-other.g0.y, 0.0) + vec2(self.g0.z) * vec2(-other.g0.z, 0.0) + vec2(self.g1.x) * vec2(other.g0.w, 0.0), vec3(0.0), vec2(0.0), vec3(self.g1.x) * vec3(other.g0.x, other.g0.y, other.g0.z), vec3(self.g0.x) * vec3(0.0, other.g0.z, -other.g0.y) + vec3(self.g0.y) * vec3(-other.g0.z, 0.0, other.g0.x) + vec3(self.g0.z) * vec3(other.g0.y, -other.g0.x, 0.0), vec4(self.g0.x, self.g0.y, self.g0.z, self.g0.x) * vec4(other.g0.w, other.g0.w, other.g0.w, 0.0) - vec4(self.g1.y, self.g1.y, self.g1.y, self.g1.x) * other.g0, vec4(0.0), vec3(0.0), vec3(0.0), vec3(0.0), vec2(0.0));
}

MultiVector sphere__wedge_dot__round_point(Sphere self, RoundPoint other) {
    return MultiVector(vec2(self.g0.x) * vec2(0.0, other.g0.x) + vec2(self.g0.y) * vec2(0.0, other.g0.y) + vec2(self.g0.z) * vec2(0.0, other.g0.z) + vec2(self.g1.x) * vec2(0.0, other.g1.y) + vec2(self.g1.y) * vec2(0.0, other.g1.x), vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0), vec4(0.0), vec4(self.g0.x, self.g0.y, self.g0.z, self.g0.x) * vec4(-other.g1.x, -other.g1.x, -other.g1.x, 0.0) + vec4(self.g1.x) * vec4(-other.g0.x, -other.g0.y, -other.g0.z, other.g1.y) + vec4(self.g1.y) * vec4(0.0, 0.0, 0.0, -other.g1.x), vec3(self.g0.x) * vec3(0.0, -other.g0.z, other.g0.y) + vec3(self.g0.y) * vec3(other.g0.z, 0.0, -other.g0.x) + vec3(self.g0.z) * vec3(-other.g0.y, other.g0.x, 0.0), self.g0 * vec3(other.g1.y) + vec3(self.g1.y) * other.g0, vec3(0.0), vec2(0.0));
}

Sphere sphere__wedge_dot__scalar(Sphere self, Scalar other) {
    return Sphere(self.g0 * vec3(other.g0), self.g1 * vec2(other.g0));
}

MultiVector sphere__wedge_dot__sphere(Sphere self, Sphere other) {
    return MultiVector(vec2(self.g0.x) * vec2(-other.g0.x, 0.0) + vec2(self.g0.y) * vec2(-other.g0.y, 0.0) + vec2(self.g0.z) * vec2(-other.g0.z, 0.0) + vec2(self.g1.x) * vec2(other.g1.y, 0.0) + vec2(self.g1.y) * vec2(other.g1.x, 0.0), vec3(0.0), vec2(0.0), vec3(0.0) - self.g0 * vec3(other.g1.x) + vec3(self.g1.x) * other.g0, vec3(self.g0.x) * vec3(0.0, other.g0.z, -other.g0.y) + vec3(self.g0.y) * vec3(-other.g0.z, 0.0, other.g0.x) + vec3(self.g0.z) * vec3(other.g0.y, -other.g0.x, 0.0), vec4(self.g0.x, self.g0.y, self.g0.z, self.g0.x) * vec4(other.g1.y, other.g1.y, other.g1.y, 0.0) + vec4(self.g1.x) * vec4(0.0, 0.0, 0.0, -other.g1.y) + vec4(self.g1.y) * vec4(-other.g0.x, -other.g0.y, -other.g0.z, other.g1.x), vec4(0.0), vec3(0.0), vec3(0.0), vec3(0.0), vec2(0.0));
}

AntiScalar anti_scalar__anti_wedge__anti_scalar(AntiScalar self, AntiScalar other) {
    return AntiScalar(self.g0 * other.g0);
}

Circle anti_scalar__anti_wedge__circle(AntiScalar self, Circle other) {
    return Circle(vec4(self.g0) * other.g0, vec3(self.g0) * other.g1, vec3(self.g0) * other.g2);
}

Dipole anti_scalar__anti_wedge__dipole(AntiScalar self, Dipole other) {
    return Dipole(vec3(self.g0) * other.g0, vec3(self.g0) * other.g1, vec4(self.g0) * other.g2);
}

DualNum anti_scalar__anti_wedge__dual_num(AntiScalar self, DualNum other) {
    return DualNum(vec2(self.g0) * other.g0);
}

FlatPoint anti_scalar__anti_wedge__flat_point(AntiScalar self, FlatPoint other) {
    return FlatPoint(vec4(self.g0) * other.g0);
}

Flector anti_scalar__anti_wedge__flector(AntiScalar self, Flector other) {
    return Flector(vec4(self.g0) * other.g0, vec4(self.g0) * other.g1);
}

Line anti_scalar__anti_wedge__line(AntiScalar self, Line other) {
    return Line(vec3(self.g0) * other.g0, vec3(self.g0) * other.g1);
}

Motor anti_scalar__anti_wedge__motor(AntiScalar self, Motor other) {
    return Motor(vec4(self.g0) * other.g0, vec3(self.g0) * other.g1);
}

MultiVector anti_scalar__anti_wedge__multi_vector(AntiScalar self, MultiVector other) {
    return MultiVector(vec2(self.g0) * other.g0, vec3(self.g0) * other.g1, vec2(self.g0) * other.g2, vec3(self.g0) * other.g3, vec3(self.g0) * other.g4, vec4(self.g0) * other.g5, vec4(self.g0) * other.g6, vec3(self.g0) * other.g7, vec3(self.g0) * other.g8, vec3(self.g0) * other.g9, vec2(self.g0) * other.g10);
}

Plane anti_scalar__anti_wedge__plane(AntiScalar self, Plane other) {
    return Plane(vec4(self.g0) * other.g0);
}

RoundPoint anti_scalar__anti_wedge__round_point(AntiScalar self, RoundPoint other) {
    return RoundPoint(vec3(self.g0) * other.g0, vec2(self.g0) * other.g1);
}

Scalar anti_scalar__anti_wedge__scalar(AntiScalar self, Scalar other) {
    return Scalar(self.g0 * other.g0);
}

Sphere anti_scalar__anti_wedge__sphere(AntiScalar self, Sphere other) {
    return Sphere(vec3(self.g0) * other.g0, vec2(self.g0) * other.g1);
}

Circle circle__anti_wedge__anti_scalar(Circle self, AntiScalar other) {
    return Circle(self.g0 * vec4(other.g0), self.g1 * vec3(other.g0), self.g2 * vec3(other.g0));
}

RoundPoint circle__anti_wedge__circle(Circle self, Circle other) {
    return RoundPoint(vec3(self.g0.x) * vec3(0.0, other.g2.z, -other.g2.y) + vec3(self.g0.y) * vec3(-other.g2.z, 0.0, other.g2.x) + vec3(self.g0.z) * vec3(other.g2.y, -other.g2.x, 0.0) + vec3(self.g0.w) * other.g1 + self.g1 * vec3(other.g0.w) + vec3(self.g2.x) * vec3(0.0, -other.g0.z, other.g0.y) + vec3(self.g2.y) * vec3(other.g0.z, 0.0, -other.g0.x) + vec3(self.g2.z) * vec3(-other.g0.y, other.g0.x, 0.0), vec2(self.g0.x) * vec2(-other.g1.x, 0.0) + vec2(self.g0.y) * vec2(-other.g1.y, 0.0) + vec2(self.g0.z) * vec2(-other.g1.z, 0.0) - vec2(self.g1.x) * vec2(other.g0.x, other.g2.x) - vec2(self.g1.y) * vec2(other.g0.y, other.g2.y) - vec2(self.g1.z) * vec2(other.g0.z, other.g2.z) + vec2(self.g2.x) * vec2(0.0, -other.g1.x) + vec2(self.g2.y) * vec2(0.0, -other.g1.y) + vec2(self.g2.z) * vec2(0.0, -other.g1.z));
}

Scalar circle__anti_wedge__dipole(Circle self, Dipole other) {
    return Scalar(0.0 - self.g0.x * other.g2.x - self.g0.y * other.g2.y - self.g0.z * other.g2.z - self.g0.w * other.g2.w - self.g1.x * other.g1.x - self.g1.y * other.g1.y - self.g1.z * other.g1.z - self.g2.x * other.g0.x - self.g2.y * other.g0.y - self.g2.z * other.g0.z);
}

Circle circle__anti_wedge__dual_num(Circle self, DualNum other) {
    return Circle(self.g0 * vec4(other.g0.y), self.g1 * vec3(other.g0.y), self.g2 * vec3(other.g0.y));
}

Scalar circle__anti_wedge__flat_point(Circle self, FlatPoint other) {
    return Scalar(0.0 - self.g0.x * other.g0.x - self.g0.y * other.g0.y - self.g0.z * other.g0.z - self.g0.w * other.g0.w);
}

MultiVector circle__anti_wedge__flector(Circle self, Flector other) {
    return MultiVector(vec2(self.g0.x) * vec2(-other.g0.x, 0.0) + vec2(self.g0.y) * vec2(-other.g0.y, 0.0) + vec2(self.g0.z) * vec2(-other.g0.z, 0.0) + vec2(self.g0.w) * vec2(-other.g0.w, 0.0), vec3(0.0), vec2(0.0), vec3(self.g0.x) * vec3(0.0, other.g1.z, -other.g1.y) + vec3(self.g0.y) * vec3(-other.g1.z, 0.0, other.g1.x) + vec3(self.g0.z) * vec3(other.g1.y, -other.g1.x, 0.0), vec3(self.g0.x, self.g0.y, self.g0.z) * vec3(other.g1.w) - vec3(self.g0.w) * vec3(other.g1.x, other.g1.y, other.g1.z), vec4(self.g1.x) * vec4(other.g1.w, 0.0, 0.0, -other.g1.x) + vec4(self.g1.y) * vec4(0.0, other.g1.w, 0.0, -other.g1.y) + vec4(self.g1.z) * vec4(0.0, 0.0, other.g1.w, -other.g1.z) + vec4(self.g2.x) * vec4(0.0, -other.g1.z, other.g1.y, 0.0) + vec4(self.g2.y) * vec4(other.g1.z, 0.0, -other.g1.x, 0.0) + vec4(self.g2.z) * vec4(-other.g1.y, other.g1.x, 0.0, 0.0), vec4(0.0), vec3(0.0), vec3(0.0), vec3(0.0), vec2(0.0));
}

RoundPoint circle__anti_wedge__line(Circle self, Line other) {
    return RoundPoint(vec3(self.g0.x) * vec3(0.0, other.g1.z, -other.g1.y) + vec3(self.g0.y) * vec3(-other.g1.z, 0.0, other.g1.x) + vec3(self.g0.z) * vec3(other.g1.y, -other.g1.x, 0.0) + vec3(self.g0.w) * other.g0, vec2(self.g0.x) * vec2(-other.g0.x, 0.0) + vec2(self.g0.y) * vec2(-other.g0.y, 0.0) + vec2(self.g0.z) * vec2(-other.g0.z, 0.0) + vec2(self.g1.x) * vec2(0.0, -other.g1.x) + vec2(self.g1.y) * vec2(0.0, -other.g1.y) + vec2(self.g1.z) * vec2(0.0, -other.g1.z) + vec2(self.g2.x) * vec2(0.0, -other.g0.x) + vec2(self.g2.y) * vec2(0.0, -other.g0.y) + vec2(self.g2.z) * vec2(0.0, -other.g0.z));
}

MultiVector circle__anti_wedge__motor(Circle self, Motor other) {
    return MultiVector(vec2(0.0), vec3(self.g0.x) * vec3(0.0, other.g1.z, -other.g1.y) + vec3(self.g0.y) * vec3(-other.g1.z, 0.0, other.g1.x) + vec3(self.g0.z) * vec3(other.g1.y, -other.g1.x, 0.0) + vec3(self.g0.w) * vec3(other.g0.x, other.g0.y, other.g0.z), vec2(self.g0.x) * vec2(-other.g0.x, 0.0) + vec2(self.g0.y) * vec2(-other.g0.y, 0.0) + vec2(self.g0.z) * vec2(-other.g0.z, 0.0) + vec2(self.g1.x) * vec2(0.0, -other.g1.x) + vec2(self.g1.y) * vec2(0.0, -other.g1.y) + vec2(self.g1.z) * vec2(0.0, -other.g1.z) + vec2(self.g2.x) * vec2(0.0, -other.g0.x) + vec2(self.g2.y) * vec2(0.0, -other.g0.y) + vec2(self.g2.z) * vec2(0.0, -other.g0.z), vec3(0.0), vec3(0.0), vec4(0.0), self.g0 * vec4(other.g0.w), self.g1 * vec3(other.g0.w), self.g2 * vec3(other.g0.w), vec3(0.0), vec2(0.0));
}

MultiVector circle__anti_wedge__multi_vector(Circle self, MultiVector other) {
    return MultiVector(vec2(self.g0.x) * vec2(-other.g5.x, 0.0) + vec2(self.g0.y) * vec2(-other.g5.y, 0.0) + vec2(self.g0.z) * vec2(-other.g5.z, 0.0) + vec2(self.g0.w) * vec2(-other.g5.w, 0.0) + vec2(self.g1.x) * vec2(-other.g4.x, 0.0) + vec2(self.g1.y) * vec2(-other.g4.y, 0.0) + vec2(self.g1.z) * vec2(-other.g4.z, 0.0) + vec2(self.g2.x) * vec2(-other.g3.x, 0.0) + vec2(self.g2.y) * vec2(-other.g3.y, 0.0) + vec2(self.g2.z) * vec2(-other.g3.z, 0.0), vec3(self.g0.x) * vec3(0.0, other.g8.z, -other.g8.y) + vec3(self.g0.y) * vec3(-other.g8.z, 0.0, other.g8.x) + vec3(self.g0.z) * vec3(other.g8.y, -other.g8.x, 0.0) + vec3(self.g0.w) * other.g7 + self.g1 * vec3(other.g6.w) + vec3(self.g2.x) * vec3(0.0, -other.g6.z, other.g6.y) + vec3(self.g2.y) * vec3(other.g6.z, 0.0, -other.g6.x) + vec3(self.g2.z) * vec3(-other.g6.y, other.g6.x, 0.0), vec2(self.g0.x) * vec2(-other.g7.x, 0.0) + vec2(self.g0.y) * vec2(-other.g7.y, 0.0) + vec2(self.g0.z) * vec2(-other.g7.z, 0.0) - vec2(self.g1.x) * vec2(other.g6.x, other.g8.x) - vec2(self.g1.y) * vec2(other.g6.y, other.g8.y) - vec2(self.g1.z) * vec2(other.g6.z, other.g8.z) + vec2(self.g2.x) * vec2(0.0, -other.g7.x) + vec2(self.g2.y) * vec2(0.0, -other.g7.y) + vec2(self.g2.z) * vec2(0.0, -other.g7.z), vec3(self.g0.x) * vec3(0.0, other.g9.z, -other.g9.y) + vec3(self.g0.y) * vec3(-other.g9.z, 0.0, other.g9.x) + vec3(self.g0.z) * vec3(other.g9.y, -other.g9.x, 0.0) + self.g1 * vec3(other.g10.x), vec3(self.g0.x, self.g0.y, self.g0.z) * vec3(other.g10.y) - vec3(self.g0.w) * other.g9 + self.g2 * vec3(other.g10.x), vec4(self.g1.x) * vec4(other.g10.y, 0.0, 0.0, -other.g9.x) + vec4(self.g1.y) * vec4(0.0, other.g10.y, 0.0, -other.g9.y) + vec4(self.g1.z) * vec4(0.0, 0.0, other.g10.y, -other.g9.z) + vec4(self.g2.x) * vec4(0.0, -other.g9.z, other.g9.y, 0.0) + vec4(self.g2.y) * vec4(other.g9.z, 0.0, -other.g9.x, 0.0) + vec4(self.g2.z) * vec4(-other.g9.y, other.g9.x, 0.0, 0.0), self.g0 * vec4(other.g0.y), self.g1 * vec3(other.g0.y), self.g2 * vec3(other.g0.y), vec3(0.0), vec2(0.0));
}

Dipole circle__anti_wedge__plane(Circle self, Plane other) {
    return Dipole(vec3(self.g0.x) * vec3(0.0, other.g0.z, -other.g0.y) + vec3(self.g0.y) * vec3(-other.g0.z, 0.0, other.g0.x) + vec3(self.g0.z) * vec3(other.g0.y, -other.g0.x, 0.0), vec3(self.g0.x, self.g0.y, self.g0.z) * vec3(other.g0.w) - vec3(self.g0.w) * vec3(other.g0.x, other.g0.y, other.g0.z), vec4(self.g1.x) * vec4(other.g0.w, 0.0, 0.0, -other.g0.x) + vec4(self.g1.y) * vec4(0.0, other.g0.w, 0.0, -other.g0.y) + vec4(self.g1.z) * vec4(0.0, 0.0, other.g0.w, -other.g0.z) + vec4(self.g2.x) * vec4(0.0, -other.g0.z, other.g0.y, 0.0) + vec4(self.g2.y) * vec4(other.g0.z, 0.0, -other.g0.x, 0.0) + vec4(self.g2.z) * vec4(-other.g0.y, other.g0.x, 0.0, 0.0));
}

Dipole circle__anti_wedge__sphere(Circle self, Sphere other) {
    return Dipole(vec3(self.g0.x) * vec3(0.0, other.g0.z, -other.g0.y) + vec3(self.g0.y) * vec3(-other.g0.z, 0.0, other.g0.x) + vec3(self.g0.z) * vec3(other.g0.y, -other.g0.x, 0.0) + self.g1 * vec3(other.g1.x), vec3(self.g0.x, self.g0.y, self.g0.z) * vec3(other.g1.y) - vec3(self.g0.w) * other.g0 + self.g2 * vec3(other.g1.x), vec4(self.g1.x) * vec4(other.g1.y, 0.0, 0.0, -other.g0.x) + vec4(self.g1.y) * vec4(0.0, other.g1.y, 0.0, -other.g0.y) + vec4(self.g1.z) * vec4(0.0, 0.0, other.g1.y, -other.g0.z) + vec4(self.g2.x) * vec4(0.0, -other.g0.z, other.g0.y, 0.0) + vec4(self.g2.y) * vec4(other.g0.z, 0.0, -other.g0.x, 0.0) + vec4(self.g2.z) * vec4(-other.g0.y, other.g0.x, 0.0, 0.0));
}

Dipole dipole__anti_wedge__anti_scalar(Dipole self, AntiScalar other) {
    return Dipole(self.g0 * vec3(other.g0), self.g1 * vec3(other.g0), self.g2 * vec4(other.g0));
}

Scalar dipole__anti_wedge__circle(Dipole self, Circle other) {
    return Scalar(0.0 - self.g0.x * other.g2.x - self.g0.y * other.g2.y - self.g0.z * other.g2.z - self.g1.x * other.g1.x - self.g1.y * other.g1.y - self.g1.z * other.g1.z - self.g2.x * other.g0.x - self.g2.y * other.g0.y - self.g2.z * other.g0.z - self.g2.w * other.g0.w);
}

Dipole dipole__anti_wedge__dual_num(Dipole self, DualNum other) {
    return Dipole(self.g0 * vec3(other.g0.y), self.g1 * vec3(other.g0.y), self.g2 * vec4(other.g0.y));
}

RoundPoint dipole__anti_wedge__flector(Dipole self, Flector other) {
    return RoundPoint(self.g0 * vec3(other.g1.w) + vec3(self.g1.x) * vec3(0.0, -other.g1.z, other.g1.y) + vec3(self.g1.y) * vec3(other.g1.z, 0.0, -other.g1.x) + vec3(self.g1.z) * vec3(-other.g1.y, other.g1.x, 0.0), vec2(self.g0.x) * vec2(-other.g1.x, 0.0) + vec2(self.g0.y) * vec2(-other.g1.y, 0.0) + vec2(self.g0.z) * vec2(-other.g1.z, 0.0) + vec2(self.g2.x) * vec2(0.0, other.g1.x) + vec2(self.g2.y) * vec2(0.0, other.g1.y) + vec2(self.g2.z) * vec2(0.0, other.g1.z) + vec2(self.g2.w) * vec2(0.0, other.g1.w));
}

Scalar dipole__anti_wedge__line(Dipole self, Line other) {
    return Scalar(0.0 - self.g0.x * other.g1.x - self.g0.y * other.g1.y - self.g0.z * other.g1.z - self.g1.x * other.g0.x - self.g1.y * other.g0.y - self.g1.z * other.g0.z);
}

MultiVector dipole__anti_wedge__motor(Dipole self, Motor other) {
    return MultiVector(vec2(self.g0.x) * vec2(-other.g1.x, 0.0) + vec2(self.g0.y) * vec2(-other.g1.y, 0.0) + vec2(self.g0.z) * vec2(-other.g1.z, 0.0) + vec2(self.g1.x) * vec2(-other.g0.x, 0.0) + vec2(self.g1.y) * vec2(-other.g0.y, 0.0) + vec2(self.g1.z) * vec2(-other.g0.z, 0.0), vec3(0.0), vec2(0.0), self.g0 * vec3(other.g0.w), self.g1 * vec3(other.g0.w), self.g2 * vec4(other.g0.w), vec4(0.0), vec3(0.0), vec3(0.0), vec3(0.0), vec2(0.0));
}

MultiVector dipole__anti_wedge__multi_vector(Dipole self, MultiVector other) {
    return MultiVector(vec2(self.g0.x) * vec2(-other.g8.x, 0.0) + vec2(self.g0.y) * vec2(-other.g8.y, 0.0) + vec2(self.g0.z) * vec2(-other.g8.z, 0.0) + vec2(self.g1.x) * vec2(-other.g7.x, 0.0) + vec2(self.g1.y) * vec2(-other.g7.y, 0.0) + vec2(self.g1.z) * vec2(-other.g7.z, 0.0) + vec2(self.g2.x) * vec2(-other.g6.x, 0.0) + vec2(self.g2.y) * vec2(-other.g6.y, 0.0) + vec2(self.g2.z) * vec2(-other.g6.z, 0.0) + vec2(self.g2.w) * vec2(-other.g6.w, 0.0), self.g0 * vec3(other.g10.y) + vec3(self.g1.x) * vec3(0.0, -other.g9.z, other.g9.y) + vec3(self.g1.y) * vec3(other.g9.z, 0.0, -other.g9.x) + vec3(self.g1.z) * vec3(-other.g9.y, other.g9.x, 0.0) - vec3(self.g2.x, self.g2.y, self.g2.z) * vec3(other.g10.x), vec2(self.g0.x) * vec2(-other.g9.x, 0.0) + vec2(self.g0.y) * vec2(-other.g9.y, 0.0) + vec2(self.g0.z) * vec2(-other.g9.z, 0.0) + vec2(self.g2.x) * vec2(0.0, other.g9.x) + vec2(self.g2.y) * vec2(0.0, other.g9.y) + vec2(self.g2.z) * vec2(0.0, other.g9.z) + vec2(self.g2.w) * other.g10 * vec2(-1.0, 1.0), self.g0 * vec3(other.g0.y), self.g1 * vec3(other.g0.y), self.g2 * vec4(other.g0.y), vec4(0.0), vec3(0.0), vec3(0.0), vec3(0.0), vec2(0.0));
}

RoundPoint dipole__anti_wedge__plane(Dipole self, Plane other) {
    return RoundPoint(self.g0 * vec3(other.g0.w) + vec3(self.g1.x) * vec3(0.0, -other.g0.z, other.g0.y) + vec3(self.g1.y) * vec3(other.g0.z, 0.0, -other.g0.x) + vec3(self.g1.z) * vec3(-other.g0.y, other.g0.x, 0.0), vec2(self.g0.x) * vec2(-other.g0.x, 0.0) + vec2(self.g0.y) * vec2(-other.g0.y, 0.0) + vec2(self.g0.z) * vec2(-other.g0.z, 0.0) + vec2(self.g2.x) * vec2(0.0, other.g0.x) + vec2(self.g2.y) * vec2(0.0, other.g0.y) + vec2(self.g2.z) * vec2(0.0, other.g0.z) + vec2(self.g2.w) * vec2(0.0, other.g0.w));
}

RoundPoint dipole__anti_wedge__sphere(Dipole self, Sphere other) {
    return RoundPoint(self.g0 * vec3(other.g1.y) + vec3(self.g1.x) * vec3(0.0, -other.g0.z, other.g0.y) + vec3(self.g1.y) * vec3(other.g0.z, 0.0, -other.g0.x) + vec3(self.g1.z) * vec3(-other.g0.y, other.g0.x, 0.0) - vec3(self.g2.x, self.g2.y, self.g2.z) * vec3(other.g1.x), vec2(self.g0.x) * vec2(-other.g0.x, 0.0) + vec2(self.g0.y) * vec2(-other.g0.y, 0.0) + vec2(self.g0.z) * vec2(-other.g0.z, 0.0) + vec2(self.g2.x) * vec2(0.0, other.g0.x) + vec2(self.g2.y) * vec2(0.0, other.g0.y) + vec2(self.g2.z) * vec2(0.0, other.g0.z) + vec2(self.g2.w) * other.g1 * vec2(-1.0, 1.0));
}

DualNum dual_num__anti_wedge__anti_scalar(DualNum self, AntiScalar other) {
    return DualNum(self.g0 * vec2(other.g0));
}

Circle dual_num__anti_wedge__circle(DualNum self, Circle other) {
    return Circle(vec4(self.g0.y) * other.g0, vec3(self.g0.y) * other.g1, vec3(self.g0.y) * other.g2);
}

Dipole dual_num__anti_wedge__dipole(DualNum self, Dipole other) {
    return Dipole(vec3(self.g0.y) * other.g0, vec3(self.g0.y) * other.g1, vec4(self.g0.y) * other.g2);
}

DualNum dual_num__anti_wedge__dual_num(DualNum self, DualNum other) {
    return DualNum(vec2(self.g0.x) * vec2(other.g0.y, 0.0) + vec2(self.g0.y) * other.g0);
}

FlatPoint dual_num__anti_wedge__flat_point(DualNum self, FlatPoint other) {
    return FlatPoint(vec4(self.g0.y) * other.g0);
}

Flector dual_num__anti_wedge__flector(DualNum self, Flector other) {
    return Flector(vec4(self.g0.y) * other.g0, vec4(self.g0.y) * other.g1);
}

Line dual_num__anti_wedge__line(DualNum self, Line other) {
    return Line(vec3(self.g0.y) * other.g0, vec3(self.g0.y) * other.g1);
}

MultiVector dual_num__anti_wedge__motor(DualNum self, Motor other) {
    return MultiVector(self.g0 * vec2(other.g0.w), vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0), vec4(0.0), vec4(0.0), vec3(self.g0.y) * vec3(other.g0.x, other.g0.y, other.g0.z), vec3(self.g0.y) * other.g1, vec3(0.0), vec2(0.0));
}

MultiVector dual_num__anti_wedge__multi_vector(DualNum self, MultiVector other) {
    return MultiVector(vec2(self.g0.x) * vec2(other.g0.y, 0.0) + vec2(self.g0.y) * other.g0, vec3(self.g0.y) * other.g1, vec2(self.g0.y) * other.g2, vec3(self.g0.y) * other.g3, vec3(self.g0.y) * other.g4, vec4(self.g0.y) * other.g5, vec4(self.g0.y) * other.g6, vec3(self.g0.y) * other.g7, vec3(self.g0.y) * other.g8, vec3(self.g0.y) * other.g9, vec2(self.g0.y) * other.g10);
}

Plane dual_num__anti_wedge__plane(DualNum self, Plane other) {
    return Plane(vec4(self.g0.y) * other.g0);
}

RoundPoint dual_num__anti_wedge__round_point(DualNum self, RoundPoint other) {
    return RoundPoint(vec3(self.g0.y) * other.g0, vec2(self.g0.y) * other.g1);
}

Scalar dual_num__anti_wedge__scalar(DualNum self, Scalar other) {
    return Scalar(self.g0.y * other.g0);
}

Sphere dual_num__anti_wedge__sphere(DualNum self, Sphere other) {
    return Sphere(vec3(self.g0.y) * other.g0, vec2(self.g0.y) * other.g1);
}

FlatPoint flat_point__anti_wedge__anti_scalar(FlatPoint self, AntiScalar other) {
    return FlatPoint(self.g0 * vec4(other.g0));
}

Scalar flat_point__anti_wedge__circle(FlatPoint self, Circle other) {
    return Scalar(0.0 - self.g0.x * other.g0.x - self.g0.y * other.g0.y - self.g0.z * other.g0.z - self.g0.w * other.g0.w);
}

FlatPoint flat_point__anti_wedge__dual_num(FlatPoint self, DualNum other) {
    return FlatPoint(self.g0 * vec4(other.g0.y));
}

RoundPoint flat_point__anti_wedge__flector(FlatPoint self, Flector other) {
    return RoundPoint(vec3(0.0), vec2(self.g0.x) * vec2(0.0, other.g1.x) + vec2(self.g0.y) * vec2(0.0, other.g1.y) + vec2(self.g0.z) * vec2(0.0, other.g1.z) + vec2(self.g0.w) * vec2(0.0, other.g1.w));
}

FlatPoint flat_point__anti_wedge__motor(FlatPoint self, Motor other) {
    return FlatPoint(self.g0 * vec4(other.g0.w));
}

MultiVector flat_point__anti_wedge__multi_vector(FlatPoint self, MultiVector other) {
    return MultiVector(vec2(self.g0.x) * vec2(-other.g6.x, 0.0) + vec2(self.g0.y) * vec2(-other.g6.y, 0.0) + vec2(self.g0.z) * vec2(-other.g6.z, 0.0) + vec2(self.g0.w) * vec2(-other.g6.w, 0.0), vec3(0.0) - vec3(self.g0.x, self.g0.y, self.g0.z) * vec3(other.g10.x), vec2(self.g0.x) * vec2(0.0, other.g9.x) + vec2(self.g0.y) * vec2(0.0, other.g9.y) + vec2(self.g0.z) * vec2(0.0, other.g9.z) + vec2(self.g0.w) * other.g10 * vec2(-1.0, 1.0), vec3(0.0), vec3(0.0), self.g0 * vec4(other.g0.y), vec4(0.0), vec3(0.0), vec3(0.0), vec3(0.0), vec2(0.0));
}

RoundPoint flat_point__anti_wedge__plane(FlatPoint self, Plane other) {
    return RoundPoint(vec3(0.0), vec2(self.g0.x) * vec2(0.0, other.g0.x) + vec2(self.g0.y) * vec2(0.0, other.g0.y) + vec2(self.g0.z) * vec2(0.0, other.g0.z) + vec2(self.g0.w) * vec2(0.0, other.g0.w));
}

RoundPoint flat_point__anti_wedge__sphere(FlatPoint self, Sphere other) {
    return RoundPoint(vec3(0.0) - vec3(self.g0.x, self.g0.y, self.g0.z) * vec3(other.g1.x), vec2(self.g0.x) * vec2(0.0, other.g0.x) + vec2(self.g0.y) * vec2(0.0, other.g0.y) + vec2(self.g0.z) * vec2(0.0, other.g0.z) + vec2(self.g0.w) * other.g1 * vec2(-1.0, 1.0));
}

Flector flector__anti_wedge__anti_scalar(Flector self, AntiScalar other) {
    return Flector(self.g0 * vec4(other.g0), self.g1 * vec4(other.g0));
}

MultiVector flector__anti_wedge__circle(Flector self, Circle other) {
    return MultiVector(vec2(self.g0.x) * vec2(-other.g0.x, 0.0) + vec2(self.g0.y) * vec2(-other.g0.y, 0.0) + vec2(self.g0.z) * vec2(-other.g0.z, 0.0) + vec2(self.g0.w) * vec2(-other.g0.w, 0.0), vec3(0.0), vec2(0.0), vec3(self.g1.x) * vec3(0.0, -other.g0.z, other.g0.y) + vec3(self.g1.y) * vec3(other.g0.z, 0.0, -other.g0.x) + vec3(self.g1.z) * vec3(-other.g0.y, other.g0.x, 0.0), vec3(0.0) - vec3(self.g1.x, self.g1.y, self.g1.z) * vec3(other.g0.w) + vec3(self.g1.w) * vec3(other.g0.x, other.g0.y, other.g0.z), vec4(self.g1.x) * vec4(0.0, other.g2.z, -other.g2.y, -other.g1.x) + vec4(self.g1.y) * vec4(-other.g2.z, 0.0, other.g2.x, -other.g1.y) + vec4(self.g1.z) * vec4(other.g2.y, -other.g2.x, 0.0, -other.g1.z) + vec4(self.g1.w) * vec4(other.g1.x, other.g1.y, other.g1.z, 0.0), vec4(0.0), vec3(0.0), vec3(0.0), vec3(0.0), vec2(0.0));
}

RoundPoint flector__anti_wedge__dipole(Flector self, Dipole other) {
    return RoundPoint(vec3(self.g1.x) * vec3(0.0, -other.g1.z, other.g1.y) + vec3(self.g1.y) * vec3(other.g1.z, 0.0, -other.g1.x) + vec3(self.g1.z) * vec3(-other.g1.y, other.g1.x, 0.0) - vec3(self.g1.w) * other.g0, vec2(self.g1.x) * vec2(other.g0.x, -other.g2.x) + vec2(self.g1.y) * vec2(other.g0.y, -other.g2.y) + vec2(self.g1.z) * vec2(other.g0.z, -other.g2.z) + vec2(self.g1.w) * vec2(0.0, -other.g2.w));
}

Flector flector__anti_wedge__dual_num(Flector self, DualNum other) {
    return Flector(self.g0 * vec4(other.g0.y), self.g1 * vec4(other.g0.y));
}

RoundPoint flector__anti_wedge__flat_point(Flector self, FlatPoint other) {
    return RoundPoint(vec3(0.0), vec2(self.g1.x) * vec2(0.0, -other.g0.x) + vec2(self.g1.y) * vec2(0.0, -other.g0.y) + vec2(self.g1.z) * vec2(0.0, -other.g0.z) + vec2(self.g1.w) * vec2(0.0, -other.g0.w));
}

MultiVector flector__anti_wedge__flector(Flector self, Flector other) {
    return MultiVector(vec2(0.0), vec3(0.0), vec2(self.g0.x) * vec2(0.0, other.g1.x) + vec2(self.g0.y) * vec2(0.0, other.g1.y) + vec2(self.g0.z) * vec2(0.0, other.g1.z) + vec2(self.g0.w) * vec2(0.0, other.g1.w) + vec2(self.g1.x) * vec2(0.0, -other.g0.x) + vec2(self.g1.y) * vec2(0.0, -other.g0.y) + vec2(self.g1.z) * vec2(0.0, -other.g0.z) + vec2(self.g1.w) * vec2(0.0, -other.g0.w), vec3(0.0), vec3(0.0), vec4(0.0), vec4(0.0), vec3(self.g1.x) * vec3(0.0, other.g1.z, -other.g1.y) + vec3(self.g1.y) * vec3(-other.g1.z, 0.0, other.g1.x) + vec3(self.g1.z) * vec3(other.g1.y, -other.g1.x, 0.0), vec3(self.g1.x, self.g1.y, self.g1.z) * vec3(other.g1.w) - vec3(self.g1.w) * vec3(other.g1.x, other.g1.y, other.g1.z), vec3(0.0), vec2(0.0));
}

FlatPoint flector__anti_wedge__line(Flector self, Line other) {
    return FlatPoint(vec4(self.g1.x) * vec4(0.0, other.g1.z, -other.g1.y, -other.g0.x) + vec4(self.g1.y) * vec4(-other.g1.z, 0.0, other.g1.x, -other.g0.y) + vec4(self.g1.z) * vec4(other.g1.y, -other.g1.x, 0.0, -other.g0.z) + vec4(self.g1.w) * vec4(other.g0.x, other.g0.y, other.g0.z, 0.0));
}

Flector flector__anti_wedge__motor(Flector self, Motor other) {
    return Flector(self.g0 * vec4(other.g0.w) + vec4(self.g1.x) * vec4(0.0, other.g1.z, -other.g1.y, -other.g0.x) + vec4(self.g1.y) * vec4(-other.g1.z, 0.0, other.g1.x, -other.g0.y) + vec4(self.g1.z) * vec4(other.g1.y, -other.g1.x, 0.0, -other.g0.z) + vec4(self.g1.w) * vec4(other.g0.x, other.g0.y, other.g0.z, 0.0), self.g1 * vec4(other.g0.w));
}

MultiVector flector__anti_wedge__multi_vector(Flector self, MultiVector other) {
    return MultiVector(vec2(self.g0.x) * vec2(-other.g6.x, 0.0) + vec2(self.g0.y) * vec2(-other.g6.y, 0.0) + vec2(self.g0.z) * vec2(-other.g6.z, 0.0) + vec2(self.g0.w) * vec2(-other.g6.w, 0.0) + vec2(self.g1.x) * vec2(other.g1.x, 0.0) + vec2(self.g1.y) * vec2(other.g1.y, 0.0) + vec2(self.g1.z) * vec2(other.g1.z, 0.0) + vec2(self.g1.w) * vec2(other.g2.x, 0.0), vec3(0.0) - vec3(self.g0.x, self.g0.y, self.g0.z) * vec3(other.g10.x) + vec3(self.g1.x) * vec3(0.0, -other.g4.z, other.g4.y) + vec3(self.g1.y) * vec3(other.g4.z, 0.0, -other.g4.x) + vec3(self.g1.z) * vec3(-other.g4.y, other.g4.x, 0.0) - vec3(self.g1.w) * other.g3, vec2(self.g0.x) * vec2(0.0, other.g9.x) + vec2(self.g0.y) * vec2(0.0, other.g9.y) + vec2(self.g0.z) * vec2(0.0, other.g9.z) + vec2(self.g0.w) * other.g10 * vec2(-1.0, 1.0) + vec2(self.g1.x) * vec2(other.g3.x, -other.g5.x) + vec2(self.g1.y) * vec2(other.g3.y, -other.g5.y) + vec2(self.g1.z) * vec2(other.g3.z, -other.g5.z) + vec2(self.g1.w) * vec2(0.0, -other.g5.w), vec3(self.g1.x) * vec3(0.0, -other.g6.z, other.g6.y) + vec3(self.g1.y) * vec3(other.g6.z, 0.0, -other.g6.x) + vec3(self.g1.z) * vec3(-other.g6.y, other.g6.x, 0.0), vec3(0.0) - vec3(self.g1.x, self.g1.y, self.g1.z) * vec3(other.g6.w) + vec3(self.g1.w) * vec3(other.g6.x, other.g6.y, other.g6.z), self.g0 * vec4(other.g0.y) + vec4(self.g1.x) * vec4(0.0, other.g8.z, -other.g8.y, -other.g7.x) + vec4(self.g1.y) * vec4(-other.g8.z, 0.0, other.g8.x, -other.g7.y) + vec4(self.g1.z) * vec4(other.g8.y, -other.g8.x, 0.0, -other.g7.z) + vec4(self.g1.w) * vec4(other.g7.x, other.g7.y, other.g7.z, 0.0), vec4(0.0) - self.g1 * vec4(other.g10.x), vec3(self.g1.x) * vec3(0.0, other.g9.z, -other.g9.y) + vec3(self.g1.y) * vec3(-other.g9.z, 0.0, other.g9.x) + vec3(self.g1.z) * vec3(other.g9.y, -other.g9.x, 0.0), vec3(self.g1.x, self.g1.y, self.g1.z) * vec3(other.g10.y) - vec3(self.g1.w) * other.g9, vec3(self.g1.x, self.g1.y, self.g1.z) * vec3(other.g0.y), vec2(self.g1.w) * vec2(0.0, other.g0.y));
}

MultiVector flector__anti_wedge__plane(Flector self, Plane other) {
    return MultiVector(vec2(0.0), vec3(0.0), vec2(self.g0.x) * vec2(0.0, other.g0.x) + vec2(self.g0.y) * vec2(0.0, other.g0.y) + vec2(self.g0.z) * vec2(0.0, other.g0.z) + vec2(self.g0.w) * vec2(0.0, other.g0.w), vec3(0.0), vec3(0.0), vec4(0.0), vec4(0.0), vec3(self.g1.x) * vec3(0.0, other.g0.z, -other.g0.y) + vec3(self.g1.y) * vec3(-other.g0.z, 0.0, other.g0.x) + vec3(self.g1.z) * vec3(other.g0.y, -other.g0.x, 0.0), vec3(self.g1.x, self.g1.y, self.g1.z) * vec3(other.g0.w) - vec3(self.g1.w) * vec3(other.g0.x, other.g0.y, other.g0.z), vec3(0.0), vec2(0.0));
}

Scalar flector__anti_wedge__round_point(Flector self, RoundPoint other) {
    return Scalar(self.g1.x * other.g0.x + self.g1.y * other.g0.y + self.g1.z * other.g0.z + self.g1.w * other.g1.x);
}

MultiVector flector__anti_wedge__sphere(Flector self, Sphere other) {
    return MultiVector(vec2(0.0), vec3(0.0) - vec3(self.g0.x, self.g0.y, self.g0.z) * vec3(other.g1.x), vec2(self.g0.x) * vec2(0.0, other.g0.x) + vec2(self.g0.y) * vec2(0.0, other.g0.y) + vec2(self.g0.z) * vec2(0.0, other.g0.z) + vec2(self.g0.w) * other.g1 * vec2(-1.0, 1.0), vec3(0.0), vec3(0.0), vec4(0.0), vec4(0.0) - self.g1 * vec4(other.g1.x), vec3(self.g1.x) * vec3(0.0, other.g0.z, -other.g0.y) + vec3(self.g1.y) * vec3(-other.g0.z, 0.0, other.g0.x) + vec3(self.g1.z) * vec3(other.g0.y, -other.g0.x, 0.0), vec3(self.g1.x, self.g1.y, self.g1.z) * vec3(other.g1.y) - vec3(self.g1.w) * other.g0, vec3(0.0), vec2(0.0));
}

Line line__anti_wedge__anti_scalar(Line self, AntiScalar other) {
    return Line(self.g0 * vec3(other.g0), self.g1 * vec3(other.g0));
}

RoundPoint line__anti_wedge__circle(Line self, Circle other) {
    return RoundPoint(self.g0 * vec3(other.g0.w) + vec3(self.g1.x) * vec3(0.0, -other.g0.z, other.g0.y) + vec3(self.g1.y) * vec3(other.g0.z, 0.0, -other.g0.x) + vec3(self.g1.z) * vec3(-other.g0.y, other.g0.x, 0.0), vec2(0.0) - vec2(self.g0.x) * vec2(other.g0.x, other.g2.x) - vec2(self.g0.y) * vec2(other.g0.y, other.g2.y) - vec2(self.g0.z) * vec2(other.g0.z, other.g2.z) + vec2(self.g1.x) * vec2(0.0, -other.g1.x) + vec2(self.g1.y) * vec2(0.0, -other.g1.y) + vec2(self.g1.z) * vec2(0.0, -other.g1.z));
}

Scalar line__anti_wedge__dipole(Line self, Dipole other) {
    return Scalar(0.0 - self.g0.x * other.g1.x - self.g0.y * other.g1.y - self.g0.z * other.g1.z - self.g1.x * other.g0.x - self.g1.y * other.g0.y - self.g1.z * other.g0.z);
}

Line line__anti_wedge__dual_num(Line self, DualNum other) {
    return Line(self.g0 * vec3(other.g0.y), self.g1 * vec3(other.g0.y));
}

FlatPoint line__anti_wedge__flector(Line self, Flector other) {
    return FlatPoint(vec4(self.g0.x) * vec4(other.g1.w, 0.0, 0.0, -other.g1.x) + vec4(self.g0.y) * vec4(0.0, other.g1.w, 0.0, -other.g1.y) + vec4(self.g0.z) * vec4(0.0, 0.0, other.g1.w, -other.g1.z) + vec4(self.g1.x) * vec4(0.0, -other.g1.z, other.g1.y, 0.0) + vec4(self.g1.y) * vec4(other.g1.z, 0.0, -other.g1.x, 0.0) + vec4(self.g1.z) * vec4(-other.g1.y, other.g1.x, 0.0, 0.0));
}

RoundPoint line__anti_wedge__line(Line self, Line other) {
    return RoundPoint(vec3(0.0), vec2(self.g0.x) * vec2(0.0, -other.g1.x) + vec2(self.g0.y) * vec2(0.0, -other.g1.y) + vec2(self.g0.z) * vec2(0.0, -other.g1.z) + vec2(self.g1.x) * vec2(0.0, -other.g0.x) + vec2(self.g1.y) * vec2(0.0, -other.g0.y) + vec2(self.g1.z) * vec2(0.0, -other.g0.z));
}

MultiVector line__anti_wedge__motor(Line self, Motor other) {
    return MultiVector(vec2(0.0), vec3(0.0), vec2(self.g0.x) * vec2(0.0, -other.g1.x) + vec2(self.g0.y) * vec2(0.0, -other.g1.y) + vec2(self.g0.z) * vec2(0.0, -other.g1.z) + vec2(self.g1.x) * vec2(0.0, -other.g0.x) + vec2(self.g1.y) * vec2(0.0, -other.g0.y) + vec2(self.g1.z) * vec2(0.0, -other.g0.z), vec3(0.0), vec3(0.0), vec4(0.0), vec4(0.0), self.g0 * vec3(other.g0.w), self.g1 * vec3(other.g0.w), vec3(0.0), vec2(0.0));
}

MultiVector line__anti_wedge__multi_vector(Line self, MultiVector other) {
    return MultiVector(vec2(self.g0.x) * vec2(-other.g4.x, 0.0) + vec2(self.g0.y) * vec2(-other.g4.y, 0.0) + vec2(self.g0.z) * vec2(-other.g4.z, 0.0) + vec2(self.g1.x) * vec2(-other.g3.x, 0.0) + vec2(self.g1.y) * vec2(-other.g3.y, 0.0) + vec2(self.g1.z) * vec2(-other.g3.z, 0.0), self.g0 * vec3(other.g6.w) + vec3(self.g1.x) * vec3(0.0, -other.g6.z, other.g6.y) + vec3(self.g1.y) * vec3(other.g6.z, 0.0, -other.g6.x) + vec3(self.g1.z) * vec3(-other.g6.y, other.g6.x, 0.0), vec2(0.0) - vec2(self.g0.x) * vec2(other.g6.x, other.g8.x) - vec2(self.g0.y) * vec2(other.g6.y, other.g8.y) - vec2(self.g0.z) * vec2(other.g6.z, other.g8.z) + vec2(self.g1.x) * vec2(0.0, -other.g7.x) + vec2(self.g1.y) * vec2(0.0, -other.g7.y) + vec2(self.g1.z) * vec2(0.0, -other.g7.z), self.g0 * vec3(other.g10.x), self.g1 * vec3(other.g10.x), vec4(self.g0.x) * vec4(other.g10.y, 0.0, 0.0, -other.g9.x) + vec4(self.g0.y) * vec4(0.0, other.g10.y, 0.0, -other.g9.y) + vec4(self.g0.z) * vec4(0.0, 0.0, other.g10.y, -other.g9.z) + vec4(self.g1.x) * vec4(0.0, -other.g9.z, other.g9.y, 0.0) + vec4(self.g1.y) * vec4(other.g9.z, 0.0, -other.g9.x, 0.0) + vec4(self.g1.z) * vec4(-other.g9.y, other.g9.x, 0.0, 0.0), vec4(0.0), self.g0 * vec3(other.g0.y), self.g1 * vec3(other.g0.y), vec3(0.0), vec2(0.0));
}

FlatPoint line__anti_wedge__plane(Line self, Plane other) {
    return FlatPoint(vec4(self.g0.x) * vec4(other.g0.w, 0.0, 0.0, -other.g0.x) + vec4(self.g0.y) * vec4(0.0, other.g0.w, 0.0, -other.g0.y) + vec4(self.g0.z) * vec4(0.0, 0.0, other.g0.w, -other.g0.z) + vec4(self.g1.x) * vec4(0.0, -other.g0.z, other.g0.y, 0.0) + vec4(self.g1.y) * vec4(other.g0.z, 0.0, -other.g0.x, 0.0) + vec4(self.g1.z) * vec4(-other.g0.y, other.g0.x, 0.0, 0.0));
}

Dipole line__anti_wedge__sphere(Line self, Sphere other) {
    return Dipole(self.g0 * vec3(other.g1.x), self.g1 * vec3(other.g1.x), vec4(self.g0.x) * vec4(other.g1.y, 0.0, 0.0, -other.g0.x) + vec4(self.g0.y) * vec4(0.0, other.g1.y, 0.0, -other.g0.y) + vec4(self.g0.z) * vec4(0.0, 0.0, other.g1.y, -other.g0.z) + vec4(self.g1.x) * vec4(0.0, -other.g0.z, other.g0.y, 0.0) + vec4(self.g1.y) * vec4(other.g0.z, 0.0, -other.g0.x, 0.0) + vec4(self.g1.z) * vec4(-other.g0.y, other.g0.x, 0.0, 0.0));
}

Motor motor__anti_wedge__anti_scalar(Motor self, AntiScalar other) {
    return Motor(self.g0 * vec4(other.g0), self.g1 * vec3(other.g0));
}

MultiVector motor__anti_wedge__circle(Motor self, Circle other) {
    return MultiVector(vec2(0.0), vec3(self.g0.x, self.g0.y, self.g0.z) * vec3(other.g0.w) + vec3(self.g1.x) * vec3(0.0, -other.g0.z, other.g0.y) + vec3(self.g1.y) * vec3(other.g0.z, 0.0, -other.g0.x) + vec3(self.g1.z) * vec3(-other.g0.y, other.g0.x, 0.0), vec2(0.0) - vec2(self.g0.x) * vec2(other.g0.x, other.g2.x) - vec2(self.g0.y) * vec2(other.g0.y, other.g2.y) - vec2(self.g0.z) * vec2(other.g0.z, other.g2.z) + vec2(self.g1.x) * vec2(0.0, -other.g1.x) + vec2(self.g1.y) * vec2(0.0, -other.g1.y) + vec2(self.g1.z) * vec2(0.0, -other.g1.z), vec3(0.0), vec3(0.0), vec4(0.0), vec4(self.g0.w) * other.g0, vec3(self.g0.w) * other.g1, vec3(self.g0.w) * other.g2, vec3(0.0), vec2(0.0));
}

MultiVector motor__anti_wedge__dipole(Motor self, Dipole other) {
    return MultiVector(vec2(self.g0.x) * vec2(-other.g1.x, 0.0) + vec2(self.g0.y) * vec2(-other.g1.y, 0.0) + vec2(self.g0.z) * vec2(-other.g1.z, 0.0) + vec2(self.g1.x) * vec2(-other.g0.x, 0.0) + vec2(self.g1.y) * vec2(-other.g0.y, 0.0) + vec2(self.g1.z) * vec2(-other.g0.z, 0.0), vec3(0.0), vec2(0.0), vec3(self.g0.w) * other.g0, vec3(self.g0.w) * other.g1, vec4(self.g0.w) * other.g2, vec4(0.0), vec3(0.0), vec3(0.0), vec3(0.0), vec2(0.0));
}

MultiVector motor__anti_wedge__dual_num(Motor self, DualNum other) {
    return MultiVector(vec2(self.g0.w) * other.g0, vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0), vec4(0.0), vec4(0.0), vec3(self.g0.x, self.g0.y, self.g0.z) * vec3(other.g0.y), self.g1 * vec3(other.g0.y), vec3(0.0), vec2(0.0));
}

FlatPoint motor__anti_wedge__flat_point(Motor self, FlatPoint other) {
    return FlatPoint(vec4(self.g0.w) * other.g0);
}

Flector motor__anti_wedge__flector(Motor self, Flector other) {
    return Flector(vec4(self.g0.x) * vec4(other.g1.w, 0.0, 0.0, -other.g1.x) + vec4(self.g0.y) * vec4(0.0, other.g1.w, 0.0, -other.g1.y) + vec4(self.g0.z) * vec4(0.0, 0.0, other.g1.w, -other.g1.z) + vec4(self.g0.w) * other.g0 + vec4(self.g1.x) * vec4(0.0, -other.g1.z, other.g1.y, 0.0) + vec4(self.g1.y) * vec4(other.g1.z, 0.0, -other.g1.x, 0.0) + vec4(self.g1.z) * vec4(-other.g1.y, other.g1.x, 0.0, 0.0), vec4(self.g0.w) * other.g1);
}

MultiVector motor__anti_wedge__line(Motor self, Line other) {
    return MultiVector(vec2(0.0), vec3(0.0), vec2(self.g0.x) * vec2(0.0, -other.g1.x) + vec2(self.g0.y) * vec2(0.0, -other.g1.y) + vec2(self.g0.z) * vec2(0.0, -other.g1.z) + vec2(self.g1.x) * vec2(0.0, -other.g0.x) + vec2(self.g1.y) * vec2(0.0, -other.g0.y) + vec2(self.g1.z) * vec2(0.0, -other.g0.z), vec3(0.0), vec3(0.0), vec4(0.0), vec4(0.0), vec3(self.g0.w) * other.g0, vec3(self.g0.w) * other.g1, vec3(0.0), vec2(0.0));
}

MultiVector motor__anti_wedge__motor(Motor self, Motor other) {
    return MultiVector(vec2(self.g0.w) * vec2(0.0, other.g0.w), vec3(0.0), vec2(self.g0.x) * vec2(0.0, -other.g1.x) + vec2(self.g0.y) * vec2(0.0, -other.g1.y) + vec2(self.g0.z) * vec2(0.0, -other.g1.z) + vec2(self.g1.x) * vec2(0.0, -other.g0.x) + vec2(self.g1.y) * vec2(0.0, -other.g0.y) + vec2(self.g1.z) * vec2(0.0, -other.g0.z), vec3(0.0), vec3(0.0), vec4(0.0), vec4(0.0), vec3(self.g0.x, self.g0.y, self.g0.z) * vec3(other.g0.w) + vec3(self.g0.w) * vec3(other.g0.x, other.g0.y, other.g0.z), vec3(self.g0.w) * other.g1 + self.g1 * vec3(other.g0.w), vec3(0.0), vec2(0.0));
}

MultiVector motor__anti_wedge__multi_vector(Motor self, MultiVector other) {
    return MultiVector(vec2(self.g0.x) * vec2(-other.g4.x, 0.0) + vec2(self.g0.y) * vec2(-other.g4.y, 0.0) + vec2(self.g0.z) * vec2(-other.g4.z, 0.0) + vec2(self.g0.w) * other.g0 + vec2(self.g1.x) * vec2(-other.g3.x, 0.0) + vec2(self.g1.y) * vec2(-other.g3.y, 0.0) + vec2(self.g1.z) * vec2(-other.g3.z, 0.0), vec3(self.g0.x, self.g0.y, self.g0.z) * vec3(other.g6.w) + vec3(self.g0.w) * other.g1 + vec3(self.g1.x) * vec3(0.0, -other.g6.z, other.g6.y) + vec3(self.g1.y) * vec3(other.g6.z, 0.0, -other.g6.x) + vec3(self.g1.z) * vec3(-other.g6.y, other.g6.x, 0.0), vec2(0.0) - vec2(self.g0.x) * vec2(other.g6.x, other.g8.x) - vec2(self.g0.y) * vec2(other.g6.y, other.g8.y) - vec2(self.g0.z) * vec2(other.g6.z, other.g8.z) + vec2(self.g0.w) * other.g2 + vec2(self.g1.x) * vec2(0.0, -other.g7.x) + vec2(self.g1.y) * vec2(0.0, -other.g7.y) + vec2(self.g1.z) * vec2(0.0, -other.g7.z), vec3(self.g0.x, self.g0.y, self.g0.z) * vec3(other.g10.x) + vec3(self.g0.w) * other.g3, vec3(self.g0.w) * other.g4 + self.g1 * vec3(other.g10.x), vec4(self.g0.x) * vec4(other.g10.y, 0.0, 0.0, -other.g9.x) + vec4(self.g0.y) * vec4(0.0, other.g10.y, 0.0, -other.g9.y) + vec4(self.g0.z) * vec4(0.0, 0.0, other.g10.y, -other.g9.z) + vec4(self.g0.w) * other.g5 + vec4(self.g1.x) * vec4(0.0, -other.g9.z, other.g9.y, 0.0) + vec4(self.g1.y) * vec4(other.g9.z, 0.0, -other.g9.x, 0.0) + vec4(self.g1.z) * vec4(-other.g9.y, other.g9.x, 0.0, 0.0), vec4(self.g0.w) * other.g6, vec3(self.g0.x, self.g0.y, self.g0.z) * vec3(other.g0.y) + vec3(self.g0.w) * other.g7, vec3(self.g0.w) * other.g8 + self.g1 * vec3(other.g0.y), vec3(self.g0.w) * other.g9, vec2(self.g0.w) * other.g10);
}

Flector motor__anti_wedge__plane(Motor self, Plane other) {
    return Flector(vec4(self.g0.x) * vec4(other.g0.w, 0.0, 0.0, -other.g0.x) + vec4(self.g0.y) * vec4(0.0, other.g0.w, 0.0, -other.g0.y) + vec4(self.g0.z) * vec4(0.0, 0.0, other.g0.w, -other.g0.z) + vec4(self.g1.x) * vec4(0.0, -other.g0.z, other.g0.y, 0.0) + vec4(self.g1.y) * vec4(other.g0.z, 0.0, -other.g0.x, 0.0) + vec4(self.g1.z) * vec4(-other.g0.y, other.g0.x, 0.0, 0.0), vec4(self.g0.w) * other.g0);
}

RoundPoint motor__anti_wedge__round_point(Motor self, RoundPoint other) {
    return RoundPoint(vec3(self.g0.w) * other.g0, vec2(self.g0.w) * other.g1);
}

Scalar motor__anti_wedge__scalar(Motor self, Scalar other) {
    return Scalar(self.g0.w * other.g0);
}

MultiVector motor__anti_wedge__sphere(Motor self, Sphere other) {
    return MultiVector(vec2(0.0), vec3(0.0), vec2(0.0), vec3(self.g0.x, self.g0.y, self.g0.z) * vec3(other.g1.x), self.g1 * vec3(other.g1.x), vec4(self.g0.x) * vec4(other.g1.y, 0.0, 0.0, -other.g0.x) + vec4(self.g0.y) * vec4(0.0, other.g1.y, 0.0, -other.g0.y) + vec4(self.g0.z) * vec4(0.0, 0.0, other.g1.y, -other.g0.z) + vec4(self.g1.x) * vec4(0.0, -other.g0.z, other.g0.y, 0.0) + vec4(self.g1.y) * vec4(other.g0.z, 0.0, -other.g0.x, 0.0) + vec4(self.g1.z) * vec4(-other.g0.y, other.g0.x, 0.0, 0.0), vec4(0.0), vec3(0.0), vec3(0.0), vec3(self.g0.w) * other.g0, vec2(self.g0.w) * other.g1);
}

MultiVector multi_vector__anti_wedge__anti_scalar(MultiVector self, AntiScalar other) {
    return MultiVector(self.g0 * vec2(other.g0), self.g1 * vec3(other.g0), self.g2 * vec2(other.g0), self.g3 * vec3(other.g0), self.g4 * vec3(other.g0), self.g5 * vec4(other.g0), self.g6 * vec4(other.g0), self.g7 * vec3(other.g0), self.g8 * vec3(other.g0), self.g9 * vec3(other.g0), self.g10 * vec2(other.g0));
}

MultiVector multi_vector__anti_wedge__circle(MultiVector self, Circle other) {
    return MultiVector(vec2(self.g3.x) * vec2(-other.g2.x, 0.0) + vec2(self.g3.y) * vec2(-other.g2.y, 0.0) + vec2(self.g3.z) * vec2(-other.g2.z, 0.0) + vec2(self.g4.x) * vec2(-other.g1.x, 0.0) + vec2(self.g4.y) * vec2(-other.g1.y, 0.0) + vec2(self.g4.z) * vec2(-other.g1.z, 0.0) + vec2(self.g5.x) * vec2(-other.g0.x, 0.0) + vec2(self.g5.y) * vec2(-other.g0.y, 0.0) + vec2(self.g5.z) * vec2(-other.g0.z, 0.0) + vec2(self.g5.w) * vec2(-other.g0.w, 0.0), vec3(self.g6.x) * vec3(0.0, other.g2.z, -other.g2.y) + vec3(self.g6.y) * vec3(-other.g2.z, 0.0, other.g2.x) + vec3(self.g6.z) * vec3(other.g2.y, -other.g2.x, 0.0) + vec3(self.g6.w) * other.g1 + self.g7 * vec3(other.g0.w) + vec3(self.g8.x) * vec3(0.0, -other.g0.z, other.g0.y) + vec3(self.g8.y) * vec3(other.g0.z, 0.0, -other.g0.x) + vec3(self.g8.z) * vec3(-other.g0.y, other.g0.x, 0.0), vec2(self.g6.x) * vec2(-other.g1.x, 0.0) + vec2(self.g6.y) * vec2(-other.g1.y, 0.0) + vec2(self.g6.z) * vec2(-other.g1.z, 0.0) - vec2(self.g7.x) * vec2(other.g0.x, other.g2.x) - vec2(self.g7.y) * vec2(other.g0.y, other.g2.y) - vec2(self.g7.z) * vec2(other.g0.z, other.g2.z) + vec2(self.g8.x) * vec2(0.0, -other.g1.x) + vec2(self.g8.y) * vec2(0.0, -other.g1.y) + vec2(self.g8.z) * vec2(0.0, -other.g1.z), vec3(self.g9.x) * vec3(0.0, -other.g0.z, other.g0.y) + vec3(self.g9.y) * vec3(other.g0.z, 0.0, -other.g0.x) + vec3(self.g9.z) * vec3(-other.g0.y, other.g0.x, 0.0) + vec3(self.g10.x) * other.g1, vec3(0.0) - self.g9 * vec3(other.g0.w) + vec3(self.g10.x) * other.g2 + vec3(self.g10.y) * vec3(other.g0.x, other.g0.y, other.g0.z), vec4(self.g9.x) * vec4(0.0, other.g2.z, -other.g2.y, -other.g1.x) + vec4(self.g9.y) * vec4(-other.g2.z, 0.0, other.g2.x, -other.g1.y) + vec4(self.g9.z) * vec4(other.g2.y, -other.g2.x, 0.0, -other.g1.z) + vec4(self.g10.y) * vec4(other.g1.x, other.g1.y, other.g1.z, 0.0), vec4(self.g0.y) * other.g0, vec3(self.g0.y) * other.g1, vec3(self.g0.y) * other.g2, vec3(0.0), vec2(0.0));
}

MultiVector multi_vector__anti_wedge__dipole(MultiVector self, Dipole other) {
    return MultiVector(vec2(self.g6.x) * vec2(-other.g2.x, 0.0) + vec2(self.g6.y) * vec2(-other.g2.y, 0.0) + vec2(self.g6.z) * vec2(-other.g2.z, 0.0) + vec2(self.g6.w) * vec2(-other.g2.w, 0.0) + vec2(self.g7.x) * vec2(-other.g1.x, 0.0) + vec2(self.g7.y) * vec2(-other.g1.y, 0.0) + vec2(self.g7.z) * vec2(-other.g1.z, 0.0) + vec2(self.g8.x) * vec2(-other.g0.x, 0.0) + vec2(self.g8.y) * vec2(-other.g0.y, 0.0) + vec2(self.g8.z) * vec2(-other.g0.z, 0.0), vec3(self.g9.x) * vec3(0.0, -other.g1.z, other.g1.y) + vec3(self.g9.y) * vec3(other.g1.z, 0.0, -other.g1.x) + vec3(self.g9.z) * vec3(-other.g1.y, other.g1.x, 0.0) + vec3(self.g10.x) * vec3(other.g2.x, other.g2.y, other.g2.z) - vec3(self.g10.y) * other.g0, vec2(self.g9.x) * vec2(other.g0.x, -other.g2.x) + vec2(self.g9.y) * vec2(other.g0.y, -other.g2.y) + vec2(self.g9.z) * vec2(other.g0.z, -other.g2.z) + self.g10 * vec2(other.g2.w), vec3(self.g0.y) * other.g0, vec3(self.g0.y) * other.g1, vec4(self.g0.y) * other.g2, vec4(0.0), vec3(0.0), vec3(0.0), vec3(0.0), vec2(0.0));
}

MultiVector multi_vector__anti_wedge__dual_num(MultiVector self, DualNum other) {
    return MultiVector(vec2(self.g0.x) * vec2(other.g0.y, 0.0) + vec2(self.g0.y) * other.g0, self.g1 * vec3(other.g0.y), self.g2 * vec2(other.g0.y), self.g3 * vec3(other.g0.y), self.g4 * vec3(other.g0.y), self.g5 * vec4(other.g0.y), self.g6 * vec4(other.g0.y), self.g7 * vec3(other.g0.y), self.g8 * vec3(other.g0.y), self.g9 * vec3(other.g0.y), self.g10 * vec2(other.g0.y));
}

MultiVector multi_vector__anti_wedge__flat_point(MultiVector self, FlatPoint other) {
    return MultiVector(vec2(self.g6.x) * vec2(-other.g0.x, 0.0) + vec2(self.g6.y) * vec2(-other.g0.y, 0.0) + vec2(self.g6.z) * vec2(-other.g0.z, 0.0) + vec2(self.g6.w) * vec2(-other.g0.w, 0.0), vec3(self.g10.x) * vec3(other.g0.x, other.g0.y, other.g0.z), vec2(self.g9.x) * vec2(0.0, -other.g0.x) + vec2(self.g9.y) * vec2(0.0, -other.g0.y) + vec2(self.g9.z) * vec2(0.0, -other.g0.z) + self.g10 * vec2(other.g0.w), vec3(0.0), vec3(0.0), vec4(self.g0.y) * other.g0, vec4(0.0), vec3(0.0), vec3(0.0), vec3(0.0), vec2(0.0));
}

MultiVector multi_vector__anti_wedge__flector(MultiVector self, Flector other) {
    return MultiVector(vec2(self.g1.x) * vec2(other.g1.x, 0.0) + vec2(self.g1.y) * vec2(other.g1.y, 0.0) + vec2(self.g1.z) * vec2(other.g1.z, 0.0) + vec2(self.g2.x) * vec2(other.g1.w, 0.0) + vec2(self.g6.x) * vec2(-other.g0.x, 0.0) + vec2(self.g6.y) * vec2(-other.g0.y, 0.0) + vec2(self.g6.z) * vec2(-other.g0.z, 0.0) + vec2(self.g6.w) * vec2(-other.g0.w, 0.0), self.g3 * vec3(other.g1.w) + vec3(self.g4.x) * vec3(0.0, -other.g1.z, other.g1.y) + vec3(self.g4.y) * vec3(other.g1.z, 0.0, -other.g1.x) + vec3(self.g4.z) * vec3(-other.g1.y, other.g1.x, 0.0) + vec3(self.g10.x) * vec3(other.g0.x, other.g0.y, other.g0.z), vec2(self.g3.x) * vec2(-other.g1.x, 0.0) + vec2(self.g3.y) * vec2(-other.g1.y, 0.0) + vec2(self.g3.z) * vec2(-other.g1.z, 0.0) + vec2(self.g5.x) * vec2(0.0, other.g1.x) + vec2(self.g5.y) * vec2(0.0, other.g1.y) + vec2(self.g5.z) * vec2(0.0, other.g1.z) + vec2(self.g5.w) * vec2(0.0, other.g1.w) + vec2(self.g9.x) * vec2(0.0, -other.g0.x) + vec2(self.g9.y) * vec2(0.0, -other.g0.y) + vec2(self.g9.z) * vec2(0.0, -other.g0.z) + self.g10 * vec2(other.g0.w), vec3(self.g6.x) * vec3(0.0, other.g1.z, -other.g1.y) + vec3(self.g6.y) * vec3(-other.g1.z, 0.0, other.g1.x) + vec3(self.g6.z) * vec3(other.g1.y, -other.g1.x, 0.0), vec3(self.g6.x, self.g6.y, self.g6.z) * vec3(other.g1.w) - vec3(self.g6.w) * vec3(other.g1.x, other.g1.y, other.g1.z), vec4(self.g0.y) * other.g0 + vec4(self.g7.x) * vec4(other.g1.w, 0.0, 0.0, -other.g1.x) + vec4(self.g7.y) * vec4(0.0, other.g1.w, 0.0, -other.g1.y) + vec4(self.g7.z) * vec4(0.0, 0.0, other.g1.w, -other.g1.z) + vec4(self.g8.x) * vec4(0.0, -other.g1.z, other.g1.y, 0.0) + vec4(self.g8.y) * vec4(other.g1.z, 0.0, -other.g1.x, 0.0) + vec4(self.g8.z) * vec4(-other.g1.y, other.g1.x, 0.0, 0.0), vec4(self.g10.x) * other.g1, vec3(self.g9.x) * vec3(0.0, other.g1.z, -other.g1.y) + vec3(self.g9.y) * vec3(-other.g1.z, 0.0, other.g1.x) + vec3(self.g9.z) * vec3(other.g1.y, -other.g1.x, 0.0), self.g9 * vec3(other.g1.w) - vec3(self.g10.y) * vec3(other.g1.x, other.g1.y, other.g1.z), vec3(self.g0.y) * vec3(other.g1.x, other.g1.y, other.g1.z), vec2(self.g0.y) * vec2(0.0, other.g1.w));
}

MultiVector multi_vector__anti_wedge__line(MultiVector self, Line other) {
    return MultiVector(vec2(self.g3.x) * vec2(-other.g1.x, 0.0) + vec2(self.g3.y) * vec2(-other.g1.y, 0.0) + vec2(self.g3.z) * vec2(-other.g1.z, 0.0) + vec2(self.g4.x) * vec2(-other.g0.x, 0.0) + vec2(self.g4.y) * vec2(-other.g0.y, 0.0) + vec2(self.g4.z) * vec2(-other.g0.z, 0.0), vec3(self.g6.x) * vec3(0.0, other.g1.z, -other.g1.y) + vec3(self.g6.y) * vec3(-other.g1.z, 0.0, other.g1.x) + vec3(self.g6.z) * vec3(other.g1.y, -other.g1.x, 0.0) + vec3(self.g6.w) * other.g0, vec2(self.g6.x) * vec2(-other.g0.x, 0.0) + vec2(self.g6.y) * vec2(-other.g0.y, 0.0) + vec2(self.g6.z) * vec2(-other.g0.z, 0.0) + vec2(self.g7.x) * vec2(0.0, -other.g1.x) + vec2(self.g7.y) * vec2(0.0, -other.g1.y) + vec2(self.g7.z) * vec2(0.0, -other.g1.z) + vec2(self.g8.x) * vec2(0.0, -other.g0.x) + vec2(self.g8.y) * vec2(0.0, -other.g0.y) + vec2(self.g8.z) * vec2(0.0, -other.g0.z), vec3(self.g10.x) * other.g0, vec3(self.g10.x) * other.g1, vec4(self.g9.x) * vec4(0.0, other.g1.z, -other.g1.y, -other.g0.x) + vec4(self.g9.y) * vec4(-other.g1.z, 0.0, other.g1.x, -other.g0.y) + vec4(self.g9.z) * vec4(other.g1.y, -other.g1.x, 0.0, -other.g0.z) + vec4(self.g10.y) * vec4(other.g0.x, other.g0.y, other.g0.z, 0.0), vec4(0.0), vec3(self.g0.y) * other.g0, vec3(self.g0.y) * other.g1, vec3(0.0), vec2(0.0));
}

MultiVector multi_vector__anti_wedge__motor(MultiVector self, Motor other) {
    return MultiVector(self.g0 * vec2(other.g0.w) + vec2(self.g3.x) * vec2(-other.g1.x, 0.0) + vec2(self.g3.y) * vec2(-other.g1.y, 0.0) + vec2(self.g3.z) * vec2(-other.g1.z, 0.0) + vec2(self.g4.x) * vec2(-other.g0.x, 0.0) + vec2(self.g4.y) * vec2(-other.g0.y, 0.0) + vec2(self.g4.z) * vec2(-other.g0.z, 0.0), self.g1 * vec3(other.g0.w) + vec3(self.g6.x) * vec3(0.0, other.g1.z, -other.g1.y) + vec3(self.g6.y) * vec3(-other.g1.z, 0.0, other.g1.x) + vec3(self.g6.z) * vec3(other.g1.y, -other.g1.x, 0.0) + vec3(self.g6.w) * vec3(other.g0.x, other.g0.y, other.g0.z), self.g2 * vec2(other.g0.w) + vec2(self.g6.x) * vec2(-other.g0.x, 0.0) + vec2(self.g6.y) * vec2(-other.g0.y, 0.0) + vec2(self.g6.z) * vec2(-other.g0.z, 0.0) + vec2(self.g7.x) * vec2(0.0, -other.g1.x) + vec2(self.g7.y) * vec2(0.0, -other.g1.y) + vec2(self.g7.z) * vec2(0.0, -other.g1.z) + vec2(self.g8.x) * vec2(0.0, -other.g0.x) + vec2(self.g8.y) * vec2(0.0, -other.g0.y) + vec2(self.g8.z) * vec2(0.0, -other.g0.z), self.g3 * vec3(other.g0.w) + vec3(self.g10.x) * vec3(other.g0.x, other.g0.y, other.g0.z), self.g4 * vec3(other.g0.w) + vec3(self.g10.x) * other.g1, self.g5 * vec4(other.g0.w) + vec4(self.g9.x) * vec4(0.0, other.g1.z, -other.g1.y, -other.g0.x) + vec4(self.g9.y) * vec4(-other.g1.z, 0.0, other.g1.x, -other.g0.y) + vec4(self.g9.z) * vec4(other.g1.y, -other.g1.x, 0.0, -other.g0.z) + vec4(self.g10.y) * vec4(other.g0.x, other.g0.y, other.g0.z, 0.0), self.g6 * vec4(other.g0.w), vec3(self.g0.y) * vec3(other.g0.x, other.g0.y, other.g0.z) + self.g7 * vec3(other.g0.w), vec3(self.g0.y) * other.g1 + self.g8 * vec3(other.g0.w), self.g9 * vec3(other.g0.w), self.g10 * vec2(other.g0.w));
}

MultiVector multi_vector__anti_wedge__multi_vector(MultiVector self, MultiVector other) {
    return MultiVector(vec2(self.g0.x) * vec2(other.g0.y, 0.0) + vec2(self.g0.y) * other.g0 + vec2(self.g1.x) * vec2(other.g9.x, 0.0) + vec2(self.g1.y) * vec2(other.g9.y, 0.0) + vec2(self.g1.z) * vec2(other.g9.z, 0.0) + vec2(self.g2.x) * vec2(other.g10.y, 0.0) + vec2(self.g2.y) * vec2(other.g10.x, 0.0) + vec2(self.g3.x) * vec2(-other.g8.x, 0.0) + vec2(self.g3.y) * vec2(-other.g8.y, 0.0) + vec2(self.g3.z) * vec2(-other.g8.z, 0.0) + vec2(self.g4.x) * vec2(-other.g7.x, 0.0) + vec2(self.g4.y) * vec2(-other.g7.y, 0.0) + vec2(self.g4.z) * vec2(-other.g7.z, 0.0) + vec2(self.g5.x) * vec2(-other.g6.x, 0.0) + vec2(self.g5.y) * vec2(-other.g6.y, 0.0) + vec2(self.g5.z) * vec2(-other.g6.z, 0.0) + vec2(self.g5.w) * vec2(-other.g6.w, 0.0) + vec2(self.g6.x) * vec2(-other.g5.x, 0.0) + vec2(self.g6.y) * vec2(-other.g5.y, 0.0) + vec2(self.g6.z) * vec2(-other.g5.z, 0.0) + vec2(self.g6.w) * vec2(-other.g5.w, 0.0) + vec2(self.g7.x) * vec2(-other.g4.x, 0.0) + vec2(self.g7.y) * vec2(-other.g4.y, 0.0) + vec2(self.g7.z) * vec2(-other.g4.z, 0.0) + vec2(self.g8.x) * vec2(-other.g3.x, 0.0) + vec2(self.g8.y) * vec2(-other.g3.y, 0.0) + vec2(self.g8.z) * vec2(-other.g3.z, 0.0) + vec2(self.g9.x) * vec2(other.g1.x, 0.0) + vec2(self.g9.y) * vec2(other.g1.y, 0.0) + vec2(self.g9.z) * vec2(other.g1.z, 0.0) + vec2(self.g10.x) * vec2(other.g2.y, 0.0) + vec2(self.g10.y) * vec2(other.g2.x, 0.0), vec3(self.g0.y) * other.g1 + self.g1 * vec3(other.g0.y) + self.g3 * vec3(other.g10.y) + vec3(self.g4.x) * vec3(0.0, -other.g9.z, other.g9.y) + vec3(self.g4.y) * vec3(other.g9.z, 0.0, -other.g9.x) + vec3(self.g4.z) * vec3(-other.g9.y, other.g9.x, 0.0) - vec3(self.g5.x, self.g5.y, self.g5.z) * vec3(other.g10.x) + vec3(self.g6.x) * vec3(0.0, other.g8.z, -other.g8.y) + vec3(self.g6.y) * vec3(-other.g8.z, 0.0, other.g8.x) + vec3(self.g6.z) * vec3(other.g8.y, -other.g8.x, 0.0) + vec3(self.g6.w) * other.g7 + self.g7 * vec3(other.g6.w) + vec3(self.g8.x) * vec3(0.0, -other.g6.z, other.g6.y) + vec3(self.g8.y) * vec3(other.g6.z, 0.0, -other.g6.x) + vec3(self.g8.z) * vec3(-other.g6.y, other.g6.x, 0.0) + vec3(self.g9.x) * vec3(0.0, -other.g4.z, other.g4.y) + vec3(self.g9.y) * vec3(other.g4.z, 0.0, -other.g4.x) + vec3(self.g9.z) * vec3(-other.g4.y, other.g4.x, 0.0) + vec3(self.g10.x) * vec3(other.g5.x, other.g5.y, other.g5.z) - vec3(self.g10.y) * other.g3, vec2(self.g0.y) * other.g2 + self.g2 * vec2(other.g0.y) + vec2(self.g3.x) * vec2(-other.g9.x, 0.0) + vec2(self.g3.y) * vec2(-other.g9.y, 0.0) + vec2(self.g3.z) * vec2(-other.g9.z, 0.0) + vec2(self.g5.x) * vec2(0.0, other.g9.x) + vec2(self.g5.y) * vec2(0.0, other.g9.y) + vec2(self.g5.z) * vec2(0.0, other.g9.z) + vec2(self.g5.w) * other.g10 * vec2(-1.0, 1.0) + vec2(self.g6.x) * vec2(-other.g7.x, 0.0) + vec2(self.g6.y) * vec2(-other.g7.y, 0.0) + vec2(self.g6.z) * vec2(-other.g7.z, 0.0) - vec2(self.g7.x) * vec2(other.g6.x, other.g8.x) - vec2(self.g7.y) * vec2(other.g6.y, other.g8.y) - vec2(self.g7.z) * vec2(other.g6.z, other.g8.z) + vec2(self.g8.x) * vec2(0.0, -other.g7.x) + vec2(self.g8.y) * vec2(0.0, -other.g7.y) + vec2(self.g8.z) * vec2(0.0, -other.g7.z) + vec2(self.g9.x) * vec2(other.g3.x, -other.g5.x) + vec2(self.g9.y) * vec2(other.g3.y, -other.g5.y) + vec2(self.g9.z) * vec2(other.g3.z, -other.g5.z) + self.g10 * vec2(other.g5.w), vec3(self.g0.y) * other.g3 + self.g3 * vec3(other.g0.y) + vec3(self.g6.x) * vec3(0.0, other.g9.z, -other.g9.y) + vec3(self.g6.y) * vec3(-other.g9.z, 0.0, other.g9.x) + vec3(self.g6.z) * vec3(other.g9.y, -other.g9.x, 0.0) + self.g7 * vec3(other.g10.x) + vec3(self.g9.x) * vec3(0.0, -other.g6.z, other.g6.y) + vec3(self.g9.y) * vec3(other.g6.z, 0.0, -other.g6.x) + vec3(self.g9.z) * vec3(-other.g6.y, other.g6.x, 0.0) + vec3(self.g10.x) * other.g7, vec3(self.g0.y) * other.g4 + self.g4 * vec3(other.g0.y) + vec3(self.g6.x, self.g6.y, self.g6.z) * vec3(other.g10.y) - vec3(self.g6.w) * other.g9 + self.g8 * vec3(other.g10.x) - self.g9 * vec3(other.g6.w) + vec3(self.g10.x) * other.g8 + vec3(self.g10.y) * vec3(other.g6.x, other.g6.y, other.g6.z), vec4(self.g0.y) * other.g5 + self.g5 * vec4(other.g0.y) + vec4(self.g7.x) * vec4(other.g10.y, 0.0, 0.0, -other.g9.x) + vec4(self.g7.y) * vec4(0.0, other.g10.y, 0.0, -other.g9.y) + vec4(self.g7.z) * vec4(0.0, 0.0, other.g10.y, -other.g9.z) + vec4(self.g8.x) * vec4(0.0, -other.g9.z, other.g9.y, 0.0) + vec4(self.g8.y) * vec4(other.g9.z, 0.0, -other.g9.x, 0.0) + vec4(self.g8.z) * vec4(-other.g9.y, other.g9.x, 0.0, 0.0) + vec4(self.g9.x) * vec4(0.0, other.g8.z, -other.g8.y, -other.g7.x) + vec4(self.g9.y) * vec4(-other.g8.z, 0.0, other.g8.x, -other.g7.y) + vec4(self.g9.z) * vec4(other.g8.y, -other.g8.x, 0.0, -other.g7.z) + vec4(self.g10.y) * vec4(other.g7.x, other.g7.y, other.g7.z, 0.0), vec4(self.g0.y) * other.g6 + self.g6 * vec4(other.g0.y) + vec4(self.g9.x, self.g9.y, self.g9.z, self.g9.x) * vec4(-other.g10.x, -other.g10.x, -other.g10.x, 0.0) + vec4(self.g10.x) * vec4(other.g9.x, other.g9.y, other.g9.z, other.g10.y) + vec4(self.g10.y) * vec4(0.0, 0.0, 0.0, -other.g10.x), vec3(self.g0.y) * other.g7 + self.g7 * vec3(other.g0.y) + vec3(self.g9.x) * vec3(0.0, other.g9.z, -other.g9.y) + vec3(self.g9.y) * vec3(-other.g9.z, 0.0, other.g9.x) + vec3(self.g9.z) * vec3(other.g9.y, -other.g9.x, 0.0), vec3(self.g0.y) * other.g8 + self.g8 * vec3(other.g0.y) + self.g9 * vec3(other.g10.y) - vec3(self.g10.y) * other.g9, vec3(self.g0.y) * other.g9 + self.g9 * vec3(other.g0.y), vec2(self.g0.y) * other.g10 + self.g10 * vec2(other.g0.y));
}

MultiVector multi_vector__anti_wedge__plane(MultiVector self, Plane other) {
    return MultiVector(vec2(self.g1.x) * vec2(other.g0.x, 0.0) + vec2(self.g1.y) * vec2(other.g0.y, 0.0) + vec2(self.g1.z) * vec2(other.g0.z, 0.0) + vec2(self.g2.x) * vec2(other.g0.w, 0.0), self.g3 * vec3(other.g0.w) + vec3(self.g4.x) * vec3(0.0, -other.g0.z, other.g0.y) + vec3(self.g4.y) * vec3(other.g0.z, 0.0, -other.g0.x) + vec3(self.g4.z) * vec3(-other.g0.y, other.g0.x, 0.0), vec2(self.g3.x) * vec2(-other.g0.x, 0.0) + vec2(self.g3.y) * vec2(-other.g0.y, 0.0) + vec2(self.g3.z) * vec2(-other.g0.z, 0.0) + vec2(self.g5.x) * vec2(0.0, other.g0.x) + vec2(self.g5.y) * vec2(0.0, other.g0.y) + vec2(self.g5.z) * vec2(0.0, other.g0.z) + vec2(self.g5.w) * vec2(0.0, other.g0.w), vec3(self.g6.x) * vec3(0.0, other.g0.z, -other.g0.y) + vec3(self.g6.y) * vec3(-other.g0.z, 0.0, other.g0.x) + vec3(self.g6.z) * vec3(other.g0.y, -other.g0.x, 0.0), vec3(self.g6.x, self.g6.y, self.g6.z) * vec3(other.g0.w) - vec3(self.g6.w) * vec3(other.g0.x, other.g0.y, other.g0.z), vec4(self.g7.x) * vec4(other.g0.w, 0.0, 0.0, -other.g0.x) + vec4(self.g7.y) * vec4(0.0, other.g0.w, 0.0, -other.g0.y) + vec4(self.g7.z) * vec4(0.0, 0.0, other.g0.w, -other.g0.z) + vec4(self.g8.x) * vec4(0.0, -other.g0.z, other.g0.y, 0.0) + vec4(self.g8.y) * vec4(other.g0.z, 0.0, -other.g0.x, 0.0) + vec4(self.g8.z) * vec4(-other.g0.y, other.g0.x, 0.0, 0.0), vec4(self.g10.x) * other.g0, vec3(self.g9.x) * vec3(0.0, other.g0.z, -other.g0.y) + vec3(self.g9.y) * vec3(-other.g0.z, 0.0, other.g0.x) + vec3(self.g9.z) * vec3(other.g0.y, -other.g0.x, 0.0), self.g9 * vec3(other.g0.w) - vec3(self.g10.y) * vec3(other.g0.x, other.g0.y, other.g0.z), vec3(self.g0.y) * vec3(other.g0.x, other.g0.y, other.g0.z), vec2(self.g0.y) * vec2(0.0, other.g0.w));
}

MultiVector multi_vector__anti_wedge__round_point(MultiVector self, RoundPoint other) {
    return MultiVector(vec2(self.g9.x) * vec2(other.g0.x, 0.0) + vec2(self.g9.y) * vec2(other.g0.y, 0.0) + vec2(self.g9.z) * vec2(other.g0.z, 0.0) + vec2(self.g10.x) * vec2(other.g1.y, 0.0) + vec2(self.g10.y) * vec2(other.g1.x, 0.0), vec3(self.g0.y) * other.g0, vec2(self.g0.y) * other.g1, vec3(0.0), vec3(0.0), vec4(0.0), vec4(0.0), vec3(0.0), vec3(0.0), vec3(0.0), vec2(0.0));
}

Scalar multi_vector__anti_wedge__scalar(MultiVector self, Scalar other) {
    return Scalar(self.g0.y * other.g0);
}

MultiVector multi_vector__anti_wedge__sphere(MultiVector self, Sphere other) {
    return MultiVector(vec2(self.g1.x) * vec2(other.g0.x, 0.0) + vec2(self.g1.y) * vec2(other.g0.y, 0.0) + vec2(self.g1.z) * vec2(other.g0.z, 0.0) + vec2(self.g2.x) * vec2(other.g1.y, 0.0) + vec2(self.g2.y) * vec2(other.g1.x, 0.0), self.g3 * vec3(other.g1.y) + vec3(self.g4.x) * vec3(0.0, -other.g0.z, other.g0.y) + vec3(self.g4.y) * vec3(other.g0.z, 0.0, -other.g0.x) + vec3(self.g4.z) * vec3(-other.g0.y, other.g0.x, 0.0) - vec3(self.g5.x, self.g5.y, self.g5.z) * vec3(other.g1.x), vec2(self.g3.x) * vec2(-other.g0.x, 0.0) + vec2(self.g3.y) * vec2(-other.g0.y, 0.0) + vec2(self.g3.z) * vec2(-other.g0.z, 0.0) + vec2(self.g5.x) * vec2(0.0, other.g0.x) + vec2(self.g5.y) * vec2(0.0, other.g0.y) + vec2(self.g5.z) * vec2(0.0, other.g0.z) + vec2(self.g5.w) * other.g1 * vec2(-1.0, 1.0), vec3(self.g6.x) * vec3(0.0, other.g0.z, -other.g0.y) + vec3(self.g6.y) * vec3(-other.g0.z, 0.0, other.g0.x) + vec3(self.g6.z) * vec3(other.g0.y, -other.g0.x, 0.0) + self.g7 * vec3(other.g1.x), vec3(self.g6.x, self.g6.y, self.g6.z) * vec3(other.g1.y) - vec3(self.g6.w) * other.g0 + self.g8 * vec3(other.g1.x), vec4(self.g7.x) * vec4(other.g1.y, 0.0, 0.0, -other.g0.x) + vec4(self.g7.y) * vec4(0.0, other.g1.y, 0.0, -other.g0.y) + vec4(self.g7.z) * vec4(0.0, 0.0, other.g1.y, -other.g0.z) + vec4(self.g8.x) * vec4(0.0, -other.g0.z, other.g0.y, 0.0) + vec4(self.g8.y) * vec4(other.g0.z, 0.0, -other.g0.x, 0.0) + vec4(self.g8.z) * vec4(-other.g0.y, other.g0.x, 0.0, 0.0), vec4(self.g9.x, self.g9.y, self.g9.z, self.g9.x) * vec4(-other.g1.x, -other.g1.x, -other.g1.x, 0.0) + vec4(self.g10.x) * vec4(other.g0.x, other.g0.y, other.g0.z, other.g1.y) + vec4(self.g10.y) * vec4(0.0, 0.0, 0.0, -other.g1.x), vec3(self.g9.x) * vec3(0.0, other.g0.z, -other.g0.y) + vec3(self.g9.y) * vec3(-other.g0.z, 0.0, other.g0.x) + vec3(self.g9.z) * vec3(other.g0.y, -other.g0.x, 0.0), self.g9 * vec3(other.g1.y) - vec3(self.g10.y) * other.g0, vec3(self.g0.y) * other.g0, vec2(self.g0.y) * other.g1);
}

Plane plane__anti_wedge__anti_scalar(Plane self, AntiScalar other) {
    return Plane(self.g0 * vec4(other.g0));
}

Dipole plane__anti_wedge__circle(Plane self, Circle other) {
    return Dipole(vec3(self.g0.x) * vec3(0.0, -other.g0.z, other.g0.y) + vec3(self.g0.y) * vec3(other.g0.z, 0.0, -other.g0.x) + vec3(self.g0.z) * vec3(-other.g0.y, other.g0.x, 0.0), vec3(0.0) - vec3(self.g0.x, self.g0.y, self.g0.z) * vec3(other.g0.w) + vec3(self.g0.w) * vec3(other.g0.x, other.g0.y, other.g0.z), vec4(self.g0.x) * vec4(0.0, other.g2.z, -other.g2.y, -other.g1.x) + vec4(self.g0.y) * vec4(-other.g2.z, 0.0, other.g2.x, -other.g1.y) + vec4(self.g0.z) * vec4(other.g2.y, -other.g2.x, 0.0, -other.g1.z) + vec4(self.g0.w) * vec4(other.g1.x, other.g1.y, other.g1.z, 0.0));
}

RoundPoint plane__anti_wedge__dipole(Plane self, Dipole other) {
    return RoundPoint(vec3(self.g0.x) * vec3(0.0, -other.g1.z, other.g1.y) + vec3(self.g0.y) * vec3(other.g1.z, 0.0, -other.g1.x) + vec3(self.g0.z) * vec3(-other.g1.y, other.g1.x, 0.0) - vec3(self.g0.w) * other.g0, vec2(self.g0.x) * vec2(other.g0.x, -other.g2.x) + vec2(self.g0.y) * vec2(other.g0.y, -other.g2.y) + vec2(self.g0.z) * vec2(other.g0.z, -other.g2.z) + vec2(self.g0.w) * vec2(0.0, -other.g2.w));
}

Plane plane__anti_wedge__dual_num(Plane self, DualNum other) {
    return Plane(self.g0 * vec4(other.g0.y));
}

RoundPoint plane__anti_wedge__flat_point(Plane self, FlatPoint other) {
    return RoundPoint(vec3(0.0), vec2(self.g0.x) * vec2(0.0, -other.g0.x) + vec2(self.g0.y) * vec2(0.0, -other.g0.y) + vec2(self.g0.z) * vec2(0.0, -other.g0.z) + vec2(self.g0.w) * vec2(0.0, -other.g0.w));
}

MultiVector plane__anti_wedge__flector(Plane self, Flector other) {
    return MultiVector(vec2(0.0), vec3(0.0), vec2(self.g0.x) * vec2(0.0, -other.g0.x) + vec2(self.g0.y) * vec2(0.0, -other.g0.y) + vec2(self.g0.z) * vec2(0.0, -other.g0.z) + vec2(self.g0.w) * vec2(0.0, -other.g0.w), vec3(0.0), vec3(0.0), vec4(0.0), vec4(0.0), vec3(self.g0.x) * vec3(0.0, other.g1.z, -other.g1.y) + vec3(self.g0.y) * vec3(-other.g1.z, 0.0, other.g1.x) + vec3(self.g0.z) * vec3(other.g1.y, -other.g1.x, 0.0), vec3(self.g0.x, self.g0.y, self.g0.z) * vec3(other.g1.w) - vec3(self.g0.w) * vec3(other.g1.x, other.g1.y, other.g1.z), vec3(0.0), vec2(0.0));
}

FlatPoint plane__anti_wedge__line(Plane self, Line other) {
    return FlatPoint(vec4(self.g0.x) * vec4(0.0, other.g1.z, -other.g1.y, -other.g0.x) + vec4(self.g0.y) * vec4(-other.g1.z, 0.0, other.g1.x, -other.g0.y) + vec4(self.g0.z) * vec4(other.g1.y, -other.g1.x, 0.0, -other.g0.z) + vec4(self.g0.w) * vec4(other.g0.x, other.g0.y, other.g0.z, 0.0));
}

Flector plane__anti_wedge__motor(Plane self, Motor other) {
    return Flector(vec4(self.g0.x) * vec4(0.0, other.g1.z, -other.g1.y, -other.g0.x) + vec4(self.g0.y) * vec4(-other.g1.z, 0.0, other.g1.x, -other.g0.y) + vec4(self.g0.z) * vec4(other.g1.y, -other.g1.x, 0.0, -other.g0.z) + vec4(self.g0.w) * vec4(other.g0.x, other.g0.y, other.g0.z, 0.0), self.g0 * vec4(other.g0.w));
}

MultiVector plane__anti_wedge__multi_vector(Plane self, MultiVector other) {
    return MultiVector(vec2(self.g0.x) * vec2(other.g1.x, 0.0) + vec2(self.g0.y) * vec2(other.g1.y, 0.0) + vec2(self.g0.z) * vec2(other.g1.z, 0.0) + vec2(self.g0.w) * vec2(other.g2.x, 0.0), vec3(self.g0.x) * vec3(0.0, -other.g4.z, other.g4.y) + vec3(self.g0.y) * vec3(other.g4.z, 0.0, -other.g4.x) + vec3(self.g0.z) * vec3(-other.g4.y, other.g4.x, 0.0) - vec3(self.g0.w) * other.g3, vec2(self.g0.x) * vec2(other.g3.x, -other.g5.x) + vec2(self.g0.y) * vec2(other.g3.y, -other.g5.y) + vec2(self.g0.z) * vec2(other.g3.z, -other.g5.z) + vec2(self.g0.w) * vec2(0.0, -other.g5.w), vec3(self.g0.x) * vec3(0.0, -other.g6.z, other.g6.y) + vec3(self.g0.y) * vec3(other.g6.z, 0.0, -other.g6.x) + vec3(self.g0.z) * vec3(-other.g6.y, other.g6.x, 0.0), vec3(0.0) - vec3(self.g0.x, self.g0.y, self.g0.z) * vec3(other.g6.w) + vec3(self.g0.w) * vec3(other.g6.x, other.g6.y, other.g6.z), vec4(self.g0.x) * vec4(0.0, other.g8.z, -other.g8.y, -other.g7.x) + vec4(self.g0.y) * vec4(-other.g8.z, 0.0, other.g8.x, -other.g7.y) + vec4(self.g0.z) * vec4(other.g8.y, -other.g8.x, 0.0, -other.g7.z) + vec4(self.g0.w) * vec4(other.g7.x, other.g7.y, other.g7.z, 0.0), vec4(0.0) - self.g0 * vec4(other.g10.x), vec3(self.g0.x) * vec3(0.0, other.g9.z, -other.g9.y) + vec3(self.g0.y) * vec3(-other.g9.z, 0.0, other.g9.x) + vec3(self.g0.z) * vec3(other.g9.y, -other.g9.x, 0.0), vec3(self.g0.x, self.g0.y, self.g0.z) * vec3(other.g10.y) - vec3(self.g0.w) * other.g9, vec3(self.g0.x, self.g0.y, self.g0.z) * vec3(other.g0.y), vec2(self.g0.w) * vec2(0.0, other.g0.y));
}

Line plane__anti_wedge__plane(Plane self, Plane other) {
    return Line(vec3(self.g0.x) * vec3(0.0, other.g0.z, -other.g0.y) + vec3(self.g0.y) * vec3(-other.g0.z, 0.0, other.g0.x) + vec3(self.g0.z) * vec3(other.g0.y, -other.g0.x, 0.0), vec3(self.g0.x, self.g0.y, self.g0.z) * vec3(other.g0.w) - vec3(self.g0.w) * vec3(other.g0.x, other.g0.y, other.g0.z));
}

Scalar plane__anti_wedge__round_point(Plane self, RoundPoint other) {
    return Scalar(self.g0.x * other.g0.x + self.g0.y * other.g0.y + self.g0.z * other.g0.z + self.g0.w * other.g1.x);
}

Circle plane__anti_wedge__sphere(Plane self, Sphere other) {
    return Circle(vec4(0.0) - self.g0 * vec4(other.g1.x), vec3(self.g0.x) * vec3(0.0, other.g0.z, -other.g0.y) + vec3(self.g0.y) * vec3(-other.g0.z, 0.0, other.g0.x) + vec3(self.g0.z) * vec3(other.g0.y, -other.g0.x, 0.0), vec3(self.g0.x, self.g0.y, self.g0.z) * vec3(other.g1.y) - vec3(self.g0.w) * other.g0);
}

RoundPoint round_point__anti_wedge__anti_scalar(RoundPoint self, AntiScalar other) {
    return RoundPoint(self.g0 * vec3(other.g0), self.g1 * vec2(other.g0));
}

RoundPoint round_point__anti_wedge__dual_num(RoundPoint self, DualNum other) {
    return RoundPoint(self.g0 * vec3(other.g0.y), self.g1 * vec2(other.g0.y));
}

Scalar round_point__anti_wedge__flector(RoundPoint self, Flector other) {
    return Scalar(self.g0.x * other.g1.x + self.g0.y * other.g1.y + self.g0.z * other.g1.z + self.g1.x * other.g1.w);
}

RoundPoint round_point__anti_wedge__motor(RoundPoint self, Motor other) {
    return RoundPoint(self.g0 * vec3(other.g0.w), self.g1 * vec2(other.g0.w));
}

MultiVector round_point__anti_wedge__multi_vector(RoundPoint self, MultiVector other) {
    return MultiVector(vec2(self.g0.x) * vec2(other.g9.x, 0.0) + vec2(self.g0.y) * vec2(other.g9.y, 0.0) + vec2(self.g0.z) * vec2(other.g9.z, 0.0) + vec2(self.g1.x) * vec2(other.g10.y, 0.0) + vec2(self.g1.y) * vec2(other.g10.x, 0.0), self.g0 * vec3(other.g0.y), self.g1 * vec2(other.g0.y), vec3(0.0), vec3(0.0), vec4(0.0), vec4(0.0), vec3(0.0), vec3(0.0), vec3(0.0), vec2(0.0));
}

Scalar round_point__anti_wedge__plane(RoundPoint self, Plane other) {
    return Scalar(self.g0.x * other.g0.x + self.g0.y * other.g0.y + self.g0.z * other.g0.z + self.g1.x * other.g0.w);
}

Scalar round_point__anti_wedge__sphere(RoundPoint self, Sphere other) {
    return Scalar(self.g0.x * other.g0.x + self.g0.y * other.g0.y + self.g0.z * other.g0.z + self.g1.x * other.g1.y + self.g1.y * other.g1.x);
}

Scalar scalar__anti_wedge__anti_scalar(Scalar self, AntiScalar other) {
    return Scalar(self.g0 * other.g0);
}

Scalar scalar__anti_wedge__dual_num(Scalar self, DualNum other) {
    return Scalar(self.g0 * other.g0.y);
}

Scalar scalar__anti_wedge__motor(Scalar self, Motor other) {
    return Scalar(self.g0 * other.g0.w);
}

Scalar scalar__anti_wedge__multi_vector(Scalar self, MultiVector other) {
    return Scalar(self.g0 * other.g0.y);
}

Sphere sphere__anti_wedge__anti_scalar(Sphere self, AntiScalar other) {
    return Sphere(self.g0 * vec3(other.g0), self.g1 * vec2(other.g0));
}

Dipole sphere__anti_wedge__circle(Sphere self, Circle other) {
    return Dipole(vec3(self.g0.x) * vec3(0.0, -other.g0.z, other.g0.y) + vec3(self.g0.y) * vec3(other.g0.z, 0.0, -other.g0.x) + vec3(self.g0.z) * vec3(-other.g0.y, other.g0.x, 0.0) + vec3(self.g1.x) * other.g1, vec3(0.0) - self.g0 * vec3(other.g0.w) + vec3(self.g1.x) * other.g2 + vec3(self.g1.y) * vec3(other.g0.x, other.g0.y, other.g0.z), vec4(self.g0.x) * vec4(0.0, other.g2.z, -other.g2.y, -other.g1.x) + vec4(self.g0.y) * vec4(-other.g2.z, 0.0, other.g2.x, -other.g1.y) + vec4(self.g0.z) * vec4(other.g2.y, -other.g2.x, 0.0, -other.g1.z) + vec4(self.g1.y) * vec4(other.g1.x, other.g1.y, other.g1.z, 0.0));
}

RoundPoint sphere__anti_wedge__dipole(Sphere self, Dipole other) {
    return RoundPoint(vec3(self.g0.x) * vec3(0.0, -other.g1.z, other.g1.y) + vec3(self.g0.y) * vec3(other.g1.z, 0.0, -other.g1.x) + vec3(self.g0.z) * vec3(-other.g1.y, other.g1.x, 0.0) + vec3(self.g1.x) * vec3(other.g2.x, other.g2.y, other.g2.z) - vec3(self.g1.y) * other.g0, vec2(self.g0.x) * vec2(other.g0.x, -other.g2.x) + vec2(self.g0.y) * vec2(other.g0.y, -other.g2.y) + vec2(self.g0.z) * vec2(other.g0.z, -other.g2.z) + self.g1 * vec2(other.g2.w));
}

Sphere sphere__anti_wedge__dual_num(Sphere self, DualNum other) {
    return Sphere(self.g0 * vec3(other.g0.y), self.g1 * vec2(other.g0.y));
}

RoundPoint sphere__anti_wedge__flat_point(Sphere self, FlatPoint other) {
    return RoundPoint(vec3(self.g1.x) * vec3(other.g0.x, other.g0.y, other.g0.z), vec2(self.g0.x) * vec2(0.0, -other.g0.x) + vec2(self.g0.y) * vec2(0.0, -other.g0.y) + vec2(self.g0.z) * vec2(0.0, -other.g0.z) + self.g1 * vec2(other.g0.w));
}

MultiVector sphere__anti_wedge__flector(Sphere self, Flector other) {
    return MultiVector(vec2(0.0), vec3(self.g1.x) * vec3(other.g0.x, other.g0.y, other.g0.z), vec2(self.g0.x) * vec2(0.0, -other.g0.x) + vec2(self.g0.y) * vec2(0.0, -other.g0.y) + vec2(self.g0.z) * vec2(0.0, -other.g0.z) + self.g1 * vec2(other.g0.w), vec3(0.0), vec3(0.0), vec4(0.0), vec4(self.g1.x) * other.g1, vec3(self.g0.x) * vec3(0.0, other.g1.z, -other.g1.y) + vec3(self.g0.y) * vec3(-other.g1.z, 0.0, other.g1.x) + vec3(self.g0.z) * vec3(other.g1.y, -other.g1.x, 0.0), self.g0 * vec3(other.g1.w) - vec3(self.g1.y) * vec3(other.g1.x, other.g1.y, other.g1.z), vec3(0.0), vec2(0.0));
}

Dipole sphere__anti_wedge__line(Sphere self, Line other) {
    return Dipole(vec3(self.g1.x) * other.g0, vec3(self.g1.x) * other.g1, vec4(self.g0.x) * vec4(0.0, other.g1.z, -other.g1.y, -other.g0.x) + vec4(self.g0.y) * vec4(-other.g1.z, 0.0, other.g1.x, -other.g0.y) + vec4(self.g0.z) * vec4(other.g1.y, -other.g1.x, 0.0, -other.g0.z) + vec4(self.g1.y) * vec4(other.g0.x, other.g0.y, other.g0.z, 0.0));
}

MultiVector sphere__anti_wedge__motor(Sphere self, Motor other) {
    return MultiVector(vec2(0.0), vec3(0.0), vec2(0.0), vec3(self.g1.x) * vec3(other.g0.x, other.g0.y, other.g0.z), vec3(self.g1.x) * other.g1, vec4(self.g0.x) * vec4(0.0, other.g1.z, -other.g1.y, -other.g0.x) + vec4(self.g0.y) * vec4(-other.g1.z, 0.0, other.g1.x, -other.g0.y) + vec4(self.g0.z) * vec4(other.g1.y, -other.g1.x, 0.0, -other.g0.z) + vec4(self.g1.y) * vec4(other.g0.x, other.g0.y, other.g0.z, 0.0), vec4(0.0), vec3(0.0), vec3(0.0), self.g0 * vec3(other.g0.w), self.g1 * vec2(other.g0.w));
}

MultiVector sphere__anti_wedge__multi_vector(Sphere self, MultiVector other) {
    return MultiVector(vec2(self.g0.x) * vec2(other.g1.x, 0.0) + vec2(self.g0.y) * vec2(other.g1.y, 0.0) + vec2(self.g0.z) * vec2(other.g1.z, 0.0) + vec2(self.g1.x) * vec2(other.g2.y, 0.0) + vec2(self.g1.y) * vec2(other.g2.x, 0.0), vec3(self.g0.x) * vec3(0.0, -other.g4.z, other.g4.y) + vec3(self.g0.y) * vec3(other.g4.z, 0.0, -other.g4.x) + vec3(self.g0.z) * vec3(-other.g4.y, other.g4.x, 0.0) + vec3(self.g1.x) * vec3(other.g5.x, other.g5.y, other.g5.z) - vec3(self.g1.y) * other.g3, vec2(self.g0.x) * vec2(other.g3.x, -other.g5.x) + vec2(self.g0.y) * vec2(other.g3.y, -other.g5.y) + vec2(self.g0.z) * vec2(other.g3.z, -other.g5.z) + self.g1 * vec2(other.g5.w), vec3(self.g0.x) * vec3(0.0, -other.g6.z, other.g6.y) + vec3(self.g0.y) * vec3(other.g6.z, 0.0, -other.g6.x) + vec3(self.g0.z) * vec3(-other.g6.y, other.g6.x, 0.0) + vec3(self.g1.x) * other.g7, vec3(0.0) - self.g0 * vec3(other.g6.w) + vec3(self.g1.x) * other.g8 + vec3(self.g1.y) * vec3(other.g6.x, other.g6.y, other.g6.z), vec4(self.g0.x) * vec4(0.0, other.g8.z, -other.g8.y, -other.g7.x) + vec4(self.g0.y) * vec4(-other.g8.z, 0.0, other.g8.x, -other.g7.y) + vec4(self.g0.z) * vec4(other.g8.y, -other.g8.x, 0.0, -other.g7.z) + vec4(self.g1.y) * vec4(other.g7.x, other.g7.y, other.g7.z, 0.0), vec4(self.g0.x, self.g0.y, self.g0.z, self.g0.x) * vec4(-other.g10.x, -other.g10.x, -other.g10.x, 0.0) + vec4(self.g1.x) * vec4(other.g9.x, other.g9.y, other.g9.z, other.g10.y) + vec4(self.g1.y) * vec4(0.0, 0.0, 0.0, -other.g10.x), vec3(self.g0.x) * vec3(0.0, other.g9.z, -other.g9.y) + vec3(self.g0.y) * vec3(-other.g9.z, 0.0, other.g9.x) + vec3(self.g0.z) * vec3(other.g9.y, -other.g9.x, 0.0), self.g0 * vec3(other.g10.y) - vec3(self.g1.y) * other.g9, self.g0 * vec3(other.g0.y), self.g1 * vec2(other.g0.y));
}

Circle sphere__anti_wedge__plane(Sphere self, Plane other) {
    return Circle(vec4(self.g1.x) * other.g0, vec3(self.g0.x) * vec3(0.0, other.g0.z, -other.g0.y) + vec3(self.g0.y) * vec3(-other.g0.z, 0.0, other.g0.x) + vec3(self.g0.z) * vec3(other.g0.y, -other.g0.x, 0.0), self.g0 * vec3(other.g0.w) - vec3(self.g1.y) * vec3(other.g0.x, other.g0.y, other.g0.z));
}

Scalar sphere__anti_wedge__round_point(Sphere self, RoundPoint other) {
    return Scalar(self.g0.x * other.g0.x + self.g0.y * other.g0.y + self.g0.z * other.g0.z + self.g1.x * other.g1.y + self.g1.y * other.g1.x);
}

Circle sphere__anti_wedge__sphere(Sphere self, Sphere other) {
    return Circle(vec4(self.g0.x, self.g0.y, self.g0.z, self.g0.x) * vec4(-other.g1.x, -other.g1.x, -other.g1.x, 0.0) + vec4(self.g1.x) * vec4(other.g0.x, other.g0.y, other.g0.z, other.g1.y) + vec4(self.g1.y) * vec4(0.0, 0.0, 0.0, -other.g1.x), vec3(self.g0.x) * vec3(0.0, other.g0.z, -other.g0.y) + vec3(self.g0.y) * vec3(-other.g0.z, 0.0, other.g0.x) + vec3(self.g0.z) * vec3(other.g0.y, -other.g0.x, 0.0), self.g0 * vec3(other.g1.y) - vec3(self.g1.y) * other.g0);
}

AntiScalar anti_scalar__wedge__dual_num(AntiScalar self, DualNum other) {
    return AntiScalar(self.g0 * other.g0.x);
}

AntiScalar anti_scalar__wedge__multi_vector(AntiScalar self, MultiVector other) {
    return AntiScalar(self.g0 * other.g0.x);
}

AntiScalar anti_scalar__wedge__scalar(AntiScalar self, Scalar other) {
    return AntiScalar(self.g0 * other.g0);
}

AntiScalar circle__wedge__dipole(Circle self, Dipole other) {
    return AntiScalar(0.0 - self.g0.x * other.g2.x - self.g0.y * other.g2.y - self.g0.z * other.g2.z - self.g0.w * other.g2.w - self.g1.x * other.g1.x - self.g1.y * other.g1.y - self.g1.z * other.g1.z - self.g2.x * other.g0.x - self.g2.y * other.g0.y - self.g2.z * other.g0.z);
}

Circle circle__wedge__dual_num(Circle self, DualNum other) {
    return Circle(self.g0 * vec4(other.g0.x), self.g1 * vec3(other.g0.x), self.g2 * vec3(other.g0.x));
}

AntiScalar circle__wedge__flat_point(Circle self, FlatPoint other) {
    return AntiScalar(0.0 - self.g0.x * other.g0.x - self.g0.y * other.g0.y - self.g0.z * other.g0.z - self.g0.w * other.g0.w);
}

AntiScalar circle__wedge__flector(Circle self, Flector other) {
    return AntiScalar(0.0 - self.g0.x * other.g0.x - self.g0.y * other.g0.y - self.g0.z * other.g0.z - self.g0.w * other.g0.w);
}

MultiVector circle__wedge__multi_vector(Circle self, MultiVector other) {
    return MultiVector(vec2(self.g0.x) * vec2(0.0, -other.g5.x) + vec2(self.g0.y) * vec2(0.0, -other.g5.y) + vec2(self.g0.z) * vec2(0.0, -other.g5.z) + vec2(self.g0.w) * vec2(0.0, -other.g5.w) + vec2(self.g1.x) * vec2(0.0, -other.g4.x) + vec2(self.g1.y) * vec2(0.0, -other.g4.y) + vec2(self.g1.z) * vec2(0.0, -other.g4.z) + vec2(self.g2.x) * vec2(0.0, -other.g3.x) + vec2(self.g2.y) * vec2(0.0, -other.g3.y) + vec2(self.g2.z) * vec2(0.0, -other.g3.z), vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0), vec4(0.0), self.g0 * vec4(other.g0.x), self.g1 * vec3(other.g0.x), self.g2 * vec3(other.g0.x), vec3(self.g0.x, self.g0.y, self.g0.z) * vec3(other.g2.y) + vec3(self.g1.x) * vec3(0.0, other.g1.z, -other.g1.y) + vec3(self.g1.y) * vec3(-other.g1.z, 0.0, other.g1.x) + vec3(self.g1.z) * vec3(other.g1.y, -other.g1.x, 0.0) - self.g2 * vec3(other.g2.x), vec2(self.g0.x) * vec2(-other.g1.x, 0.0) + vec2(self.g0.y) * vec2(-other.g1.y, 0.0) + vec2(self.g0.z) * vec2(-other.g1.z, 0.0) + vec2(self.g0.w) * other.g2 * vec2(-1.0, 1.0) + vec2(self.g2.x) * vec2(0.0, other.g1.x) + vec2(self.g2.y) * vec2(0.0, other.g1.y) + vec2(self.g2.z) * vec2(0.0, other.g1.z));
}

Sphere circle__wedge__round_point(Circle self, RoundPoint other) {
    return Sphere(vec3(self.g0.x, self.g0.y, self.g0.z) * vec3(other.g1.y) + vec3(self.g1.x) * vec3(0.0, other.g0.z, -other.g0.y) + vec3(self.g1.y) * vec3(-other.g0.z, 0.0, other.g0.x) + vec3(self.g1.z) * vec3(other.g0.y, -other.g0.x, 0.0) - self.g2 * vec3(other.g1.x), vec2(self.g0.x) * vec2(-other.g0.x, 0.0) + vec2(self.g0.y) * vec2(-other.g0.y, 0.0) + vec2(self.g0.z) * vec2(-other.g0.z, 0.0) + vec2(self.g0.w) * other.g1 * vec2(-1.0, 1.0) + vec2(self.g2.x) * vec2(0.0, other.g0.x) + vec2(self.g2.y) * vec2(0.0, other.g0.y) + vec2(self.g2.z) * vec2(0.0, other.g0.z));
}

Circle circle__wedge__scalar(Circle self, Scalar other) {
    return Circle(self.g0 * vec4(other.g0), self.g1 * vec3(other.g0), self.g2 * vec3(other.g0));
}

AntiScalar dipole__wedge__circle(Dipole self, Circle other) {
    return AntiScalar(0.0 - self.g0.x * other.g2.x - self.g0.y * other.g2.y - self.g0.z * other.g2.z - self.g1.x * other.g1.x - self.g1.y * other.g1.y - self.g1.z * other.g1.z - self.g2.x * other.g0.x - self.g2.y * other.g0.y - self.g2.z * other.g0.z - self.g2.w * other.g0.w);
}

Sphere dipole__wedge__dipole(Dipole self, Dipole other) {
    return Sphere(vec3(self.g0.x) * vec3(0.0, -other.g2.z, other.g2.y) + vec3(self.g0.y) * vec3(other.g2.z, 0.0, -other.g2.x) + vec3(self.g0.z) * vec3(-other.g2.y, other.g2.x, 0.0) + self.g1 * vec3(other.g2.w) + vec3(self.g2.x) * vec3(0.0, other.g0.z, -other.g0.y) + vec3(self.g2.y) * vec3(-other.g0.z, 0.0, other.g0.x) + vec3(self.g2.z) * vec3(other.g0.y, -other.g0.x, 0.0) + vec3(self.g2.w) * other.g1, vec2(self.g0.x) * vec2(-other.g1.x, 0.0) + vec2(self.g0.y) * vec2(-other.g1.y, 0.0) + vec2(self.g0.z) * vec2(-other.g1.z, 0.0) - vec2(self.g1.x) * vec2(other.g0.x, other.g2.x) - vec2(self.g1.y) * vec2(other.g0.y, other.g2.y) - vec2(self.g1.z) * vec2(other.g0.z, other.g2.z) + vec2(self.g2.x) * vec2(0.0, -other.g1.x) + vec2(self.g2.y) * vec2(0.0, -other.g1.y) + vec2(self.g2.z) * vec2(0.0, -other.g1.z));
}

Dipole dipole__wedge__dual_num(Dipole self, DualNum other) {
    return Dipole(self.g0 * vec3(other.g0.x), self.g1 * vec3(other.g0.x), self.g2 * vec4(other.g0.x));
}

Plane dipole__wedge__flat_point(Dipole self, FlatPoint other) {
    return Plane(vec4(self.g0.x) * vec4(0.0, -other.g0.z, other.g0.y, 0.0) + vec4(self.g0.y) * vec4(other.g0.z, 0.0, -other.g0.x, 0.0) + vec4(self.g0.z) * vec4(-other.g0.y, other.g0.x, 0.0, 0.0) + vec4(self.g1.x) * vec4(other.g0.w, 0.0, 0.0, -other.g0.x) + vec4(self.g1.y) * vec4(0.0, other.g0.w, 0.0, -other.g0.y) + vec4(self.g1.z) * vec4(0.0, 0.0, other.g0.w, -other.g0.z));
}

Plane dipole__wedge__flector(Dipole self, Flector other) {
    return Plane(vec4(self.g0.x) * vec4(0.0, -other.g0.z, other.g0.y, 0.0) + vec4(self.g0.y) * vec4(other.g0.z, 0.0, -other.g0.x, 0.0) + vec4(self.g0.z) * vec4(-other.g0.y, other.g0.x, 0.0, 0.0) + vec4(self.g1.x) * vec4(other.g0.w, 0.0, 0.0, -other.g0.x) + vec4(self.g1.y) * vec4(0.0, other.g0.w, 0.0, -other.g0.y) + vec4(self.g1.z) * vec4(0.0, 0.0, other.g0.w, -other.g0.z));
}

AntiScalar dipole__wedge__line(Dipole self, Line other) {
    return AntiScalar(0.0 - self.g0.x * other.g1.x - self.g0.y * other.g1.y - self.g0.z * other.g1.z - self.g1.x * other.g0.x - self.g1.y * other.g0.y - self.g1.z * other.g0.z);
}

AntiScalar dipole__wedge__motor(Dipole self, Motor other) {
    return AntiScalar(0.0 - self.g0.x * other.g1.x - self.g0.y * other.g1.y - self.g0.z * other.g1.z - self.g1.x * other.g0.x - self.g1.y * other.g0.y - self.g1.z * other.g0.z);
}

MultiVector dipole__wedge__multi_vector(Dipole self, MultiVector other) {
    return MultiVector(vec2(self.g0.x) * vec2(0.0, -other.g8.x) + vec2(self.g0.y) * vec2(0.0, -other.g8.y) + vec2(self.g0.z) * vec2(0.0, -other.g8.z) + vec2(self.g1.x) * vec2(0.0, -other.g7.x) + vec2(self.g1.y) * vec2(0.0, -other.g7.y) + vec2(self.g1.z) * vec2(0.0, -other.g7.z) + vec2(self.g2.x) * vec2(0.0, -other.g6.x) + vec2(self.g2.y) * vec2(0.0, -other.g6.y) + vec2(self.g2.z) * vec2(0.0, -other.g6.z) + vec2(self.g2.w) * vec2(0.0, -other.g6.w), vec3(0.0), vec2(0.0), self.g0 * vec3(other.g0.x), self.g1 * vec3(other.g0.x), self.g2 * vec4(other.g0.x), vec4(self.g0.x) * vec4(0.0, -other.g1.z, other.g1.y, 0.0) + vec4(self.g0.y) * vec4(other.g1.z, 0.0, -other.g1.x, 0.0) + vec4(self.g0.z) * vec4(-other.g1.y, other.g1.x, 0.0, 0.0) + vec4(self.g1.x) * vec4(other.g2.x, 0.0, 0.0, -other.g1.x) + vec4(self.g1.y) * vec4(0.0, other.g2.x, 0.0, -other.g1.y) + vec4(self.g1.z) * vec4(0.0, 0.0, other.g2.x, -other.g1.z), self.g0 * vec3(other.g2.y) + vec3(self.g2.x, self.g2.y, self.g2.z) * vec3(other.g2.x) - vec3(self.g2.w) * other.g1, self.g1 * vec3(other.g2.y) + vec3(self.g2.x) * vec3(0.0, other.g1.z, -other.g1.y) + vec3(self.g2.y) * vec3(-other.g1.z, 0.0, other.g1.x) + vec3(self.g2.z) * vec3(other.g1.y, -other.g1.x, 0.0), vec3(self.g0.x) * vec3(0.0, -other.g5.z, other.g5.y) + vec3(self.g0.y) * vec3(other.g5.z, 0.0, -other.g5.x) + vec3(self.g0.z) * vec3(-other.g5.y, other.g5.x, 0.0) + self.g1 * vec3(other.g5.w) + vec3(self.g2.x) * vec3(0.0, other.g3.z, -other.g3.y) + vec3(self.g2.y) * vec3(-other.g3.z, 0.0, other.g3.x) + vec3(self.g2.z) * vec3(other.g3.y, -other.g3.x, 0.0) + vec3(self.g2.w) * other.g4, vec2(self.g0.x) * vec2(-other.g4.x, 0.0) + vec2(self.g0.y) * vec2(-other.g4.y, 0.0) + vec2(self.g0.z) * vec2(-other.g4.z, 0.0) - vec2(self.g1.x) * vec2(other.g3.x, other.g5.x) - vec2(self.g1.y) * vec2(other.g3.y, other.g5.y) - vec2(self.g1.z) * vec2(other.g3.z, other.g5.z) + vec2(self.g2.x) * vec2(0.0, -other.g4.x) + vec2(self.g2.y) * vec2(0.0, -other.g4.y) + vec2(self.g2.z) * vec2(0.0, -other.g4.z));
}

Circle dipole__wedge__round_point(Dipole self, RoundPoint other) {
    return Circle(vec4(self.g0.x) * vec4(0.0, -other.g0.z, other.g0.y, 0.0) + vec4(self.g0.y) * vec4(other.g0.z, 0.0, -other.g0.x, 0.0) + vec4(self.g0.z) * vec4(-other.g0.y, other.g0.x, 0.0, 0.0) + vec4(self.g1.x) * vec4(other.g1.x, 0.0, 0.0, -other.g0.x) + vec4(self.g1.y) * vec4(0.0, other.g1.x, 0.0, -other.g0.y) + vec4(self.g1.z) * vec4(0.0, 0.0, other.g1.x, -other.g0.z), self.g0 * vec3(other.g1.y) + vec3(self.g2.x, self.g2.y, self.g2.z) * vec3(other.g1.x) - vec3(self.g2.w) * other.g0, self.g1 * vec3(other.g1.y) + vec3(self.g2.x) * vec3(0.0, other.g0.z, -other.g0.y) + vec3(self.g2.y) * vec3(-other.g0.z, 0.0, other.g0.x) + vec3(self.g2.z) * vec3(other.g0.y, -other.g0.x, 0.0));
}

Dipole dipole__wedge__scalar(Dipole self, Scalar other) {
    return Dipole(self.g0 * vec3(other.g0), self.g1 * vec3(other.g0), self.g2 * vec4(other.g0));
}

AntiScalar dual_num__wedge__anti_scalar(DualNum self, AntiScalar other) {
    return AntiScalar(self.g0.x * other.g0);
}

Circle dual_num__wedge__circle(DualNum self, Circle other) {
    return Circle(vec4(self.g0.x) * other.g0, vec3(self.g0.x) * other.g1, vec3(self.g0.x) * other.g2);
}

Dipole dual_num__wedge__dipole(DualNum self, Dipole other) {
    return Dipole(vec3(self.g0.x) * other.g0, vec3(self.g0.x) * other.g1, vec4(self.g0.x) * other.g2);
}

DualNum dual_num__wedge__dual_num(DualNum self, DualNum other) {
    return DualNum(vec2(self.g0.x) * other.g0 + vec2(self.g0.y) * vec2(0.0, other.g0.x));
}

FlatPoint dual_num__wedge__flat_point(DualNum self, FlatPoint other) {
    return FlatPoint(vec4(self.g0.x) * other.g0);
}

Flector dual_num__wedge__flector(DualNum self, Flector other) {
    return Flector(vec4(self.g0.x) * other.g0, vec4(self.g0.x) * other.g1);
}

Line dual_num__wedge__line(DualNum self, Line other) {
    return Line(vec3(self.g0.x) * other.g0, vec3(self.g0.x) * other.g1);
}

Motor dual_num__wedge__motor(DualNum self, Motor other) {
    return Motor(vec4(self.g0.x) * other.g0, vec3(self.g0.x) * other.g1);
}

MultiVector dual_num__wedge__multi_vector(DualNum self, MultiVector other) {
    return MultiVector(vec2(self.g0.x) * other.g0 + vec2(self.g0.y) * vec2(0.0, other.g0.x), vec3(self.g0.x) * other.g1, vec2(self.g0.x) * other.g2, vec3(self.g0.x) * other.g3, vec3(self.g0.x) * other.g4, vec4(self.g0.x) * other.g5, vec4(self.g0.x) * other.g6, vec3(self.g0.x) * other.g7, vec3(self.g0.x) * other.g8, vec3(self.g0.x) * other.g9, vec2(self.g0.x) * other.g10);
}

Plane dual_num__wedge__plane(DualNum self, Plane other) {
    return Plane(vec4(self.g0.x) * other.g0);
}

RoundPoint dual_num__wedge__round_point(DualNum self, RoundPoint other) {
    return RoundPoint(vec3(self.g0.x) * other.g0, vec2(self.g0.x) * other.g1);
}

DualNum dual_num__wedge__scalar(DualNum self, Scalar other) {
    return DualNum(self.g0 * vec2(other.g0));
}

Sphere dual_num__wedge__sphere(DualNum self, Sphere other) {
    return Sphere(vec3(self.g0.x) * other.g0, vec2(self.g0.x) * other.g1);
}

AntiScalar flat_point__wedge__circle(FlatPoint self, Circle other) {
    return AntiScalar(0.0 - self.g0.x * other.g0.x - self.g0.y * other.g0.y - self.g0.z * other.g0.z - self.g0.w * other.g0.w);
}

Plane flat_point__wedge__dipole(FlatPoint self, Dipole other) {
    return Plane(vec4(self.g0.x) * vec4(0.0, other.g0.z, -other.g0.y, -other.g1.x) + vec4(self.g0.y) * vec4(-other.g0.z, 0.0, other.g0.x, -other.g1.y) + vec4(self.g0.z) * vec4(other.g0.y, -other.g0.x, 0.0, -other.g1.z) + vec4(self.g0.w) * vec4(other.g1.x, other.g1.y, other.g1.z, 0.0));
}

FlatPoint flat_point__wedge__dual_num(FlatPoint self, DualNum other) {
    return FlatPoint(self.g0 * vec4(other.g0.x));
}

MultiVector flat_point__wedge__multi_vector(FlatPoint self, MultiVector other) {
    return MultiVector(vec2(self.g0.x) * vec2(0.0, -other.g6.x) + vec2(self.g0.y) * vec2(0.0, -other.g6.y) + vec2(self.g0.z) * vec2(0.0, -other.g6.z) + vec2(self.g0.w) * vec2(0.0, -other.g6.w), vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0), self.g0 * vec4(other.g0.x), vec4(0.0), vec3(self.g0.x, self.g0.y, self.g0.z) * vec3(other.g2.x) - vec3(self.g0.w) * other.g1, vec3(self.g0.x) * vec3(0.0, other.g1.z, -other.g1.y) + vec3(self.g0.y) * vec3(-other.g1.z, 0.0, other.g1.x) + vec3(self.g0.z) * vec3(other.g1.y, -other.g1.x, 0.0), vec3(self.g0.x) * vec3(0.0, other.g3.z, -other.g3.y) + vec3(self.g0.y) * vec3(-other.g3.z, 0.0, other.g3.x) + vec3(self.g0.z) * vec3(other.g3.y, -other.g3.x, 0.0) + vec3(self.g0.w) * other.g4, vec2(self.g0.x) * vec2(0.0, -other.g4.x) + vec2(self.g0.y) * vec2(0.0, -other.g4.y) + vec2(self.g0.z) * vec2(0.0, -other.g4.z));
}

Line flat_point__wedge__round_point(FlatPoint self, RoundPoint other) {
    return Line(vec3(self.g0.x, self.g0.y, self.g0.z) * vec3(other.g1.x) - vec3(self.g0.w) * other.g0, vec3(self.g0.x) * vec3(0.0, other.g0.z, -other.g0.y) + vec3(self.g0.y) * vec3(-other.g0.z, 0.0, other.g0.x) + vec3(self.g0.z) * vec3(other.g0.y, -other.g0.x, 0.0));
}

FlatPoint flat_point__wedge__scalar(FlatPoint self, Scalar other) {
    return FlatPoint(self.g0 * vec4(other.g0));
}

AntiScalar flector__wedge__circle(Flector self, Circle other) {
    return AntiScalar(0.0 - self.g0.x * other.g0.x - self.g0.y * other.g0.y - self.g0.z * other.g0.z - self.g0.w * other.g0.w);
}

Plane flector__wedge__dipole(Flector self, Dipole other) {
    return Plane(vec4(self.g0.x) * vec4(0.0, other.g0.z, -other.g0.y, -other.g1.x) + vec4(self.g0.y) * vec4(-other.g0.z, 0.0, other.g0.x, -other.g1.y) + vec4(self.g0.z) * vec4(other.g0.y, -other.g0.x, 0.0, -other.g1.z) + vec4(self.g0.w) * vec4(other.g1.x, other.g1.y, other.g1.z, 0.0));
}

Flector flector__wedge__dual_num(Flector self, DualNum other) {
    return Flector(self.g0 * vec4(other.g0.x), self.g1 * vec4(other.g0.x));
}

MultiVector flector__wedge__multi_vector(Flector self, MultiVector other) {
    return MultiVector(vec2(self.g0.x) * vec2(0.0, -other.g6.x) + vec2(self.g0.y) * vec2(0.0, -other.g6.y) + vec2(self.g0.z) * vec2(0.0, -other.g6.z) + vec2(self.g0.w) * vec2(0.0, -other.g6.w) + vec2(self.g1.x) * vec2(0.0, other.g1.x) + vec2(self.g1.y) * vec2(0.0, other.g1.y) + vec2(self.g1.z) * vec2(0.0, other.g1.z) + vec2(self.g1.w) * vec2(0.0, other.g2.x), vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0), self.g0 * vec4(other.g0.x), vec4(0.0), vec3(self.g0.x, self.g0.y, self.g0.z) * vec3(other.g2.x) - vec3(self.g0.w) * other.g1, vec3(self.g0.x) * vec3(0.0, other.g1.z, -other.g1.y) + vec3(self.g0.y) * vec3(-other.g1.z, 0.0, other.g1.x) + vec3(self.g0.z) * vec3(other.g1.y, -other.g1.x, 0.0), vec3(self.g0.x) * vec3(0.0, other.g3.z, -other.g3.y) + vec3(self.g0.y) * vec3(-other.g3.z, 0.0, other.g3.x) + vec3(self.g0.z) * vec3(other.g3.y, -other.g3.x, 0.0) + vec3(self.g0.w) * other.g4 + vec3(self.g1.x, self.g1.y, self.g1.z) * vec3(other.g0.x), vec2(self.g0.x) * vec2(0.0, -other.g4.x) + vec2(self.g0.y) * vec2(0.0, -other.g4.y) + vec2(self.g0.z) * vec2(0.0, -other.g4.z) + vec2(self.g1.w) * vec2(0.0, other.g0.x));
}

Motor flector__wedge__round_point(Flector self, RoundPoint other) {
    return Motor(self.g0.xyzx * vec4(other.g1.x, other.g1.x, other.g1.x, 0.0) + vec4(self.g0.w) * vec4(-other.g0.x, -other.g0.y, -other.g0.z, 0.0) + vec4(self.g1.x) * vec4(0.0, 0.0, 0.0, other.g0.x) + vec4(self.g1.y) * vec4(0.0, 0.0, 0.0, other.g0.y) + vec4(self.g1.z) * vec4(0.0, 0.0, 0.0, other.g0.z) + vec4(self.g1.w) * vec4(0.0, 0.0, 0.0, other.g1.x), vec3(self.g0.x) * vec3(0.0, other.g0.z, -other.g0.y) + vec3(self.g0.y) * vec3(-other.g0.z, 0.0, other.g0.x) + vec3(self.g0.z) * vec3(other.g0.y, -other.g0.x, 0.0));
}

Flector flector__wedge__scalar(Flector self, Scalar other) {
    return Flector(self.g0 * vec4(other.g0), self.g1 * vec4(other.g0));
}

AntiScalar line__wedge__dipole(Line self, Dipole other) {
    return AntiScalar(0.0 - self.g0.x * other.g1.x - self.g0.y * other.g1.y - self.g0.z * other.g1.z - self.g1.x * other.g0.x - self.g1.y * other.g0.y - self.g1.z * other.g0.z);
}

Line line__wedge__dual_num(Line self, DualNum other) {
    return Line(self.g0 * vec3(other.g0.x), self.g1 * vec3(other.g0.x));
}

MultiVector line__wedge__multi_vector(Line self, MultiVector other) {
    return MultiVector(vec2(self.g0.x) * vec2(0.0, -other.g4.x) + vec2(self.g0.y) * vec2(0.0, -other.g4.y) + vec2(self.g0.z) * vec2(0.0, -other.g4.z) + vec2(self.g1.x) * vec2(0.0, -other.g3.x) + vec2(self.g1.y) * vec2(0.0, -other.g3.y) + vec2(self.g1.z) * vec2(0.0, -other.g3.z), vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0), vec4(0.0), vec4(0.0), self.g0 * vec3(other.g0.x), self.g1 * vec3(other.g0.x), vec3(self.g0.x) * vec3(0.0, other.g1.z, -other.g1.y) + vec3(self.g0.y) * vec3(-other.g1.z, 0.0, other.g1.x) + vec3(self.g0.z) * vec3(other.g1.y, -other.g1.x, 0.0) - self.g1 * vec3(other.g2.x), vec2(self.g1.x) * vec2(0.0, other.g1.x) + vec2(self.g1.y) * vec2(0.0, other.g1.y) + vec2(self.g1.z) * vec2(0.0, other.g1.z));
}

Plane line__wedge__round_point(Line self, RoundPoint other) {
    return Plane(vec4(self.g0.x) * vec4(0.0, other.g0.z, -other.g0.y, 0.0) + vec4(self.g0.y) * vec4(-other.g0.z, 0.0, other.g0.x, 0.0) + vec4(self.g0.z) * vec4(other.g0.y, -other.g0.x, 0.0, 0.0) + vec4(self.g1.x) * vec4(-other.g1.x, 0.0, 0.0, other.g0.x) + vec4(self.g1.y) * vec4(0.0, -other.g1.x, 0.0, other.g0.y) + vec4(self.g1.z) * vec4(0.0, 0.0, -other.g1.x, other.g0.z));
}

Line line__wedge__scalar(Line self, Scalar other) {
    return Line(self.g0 * vec3(other.g0), self.g1 * vec3(other.g0));
}

AntiScalar motor__wedge__dipole(Motor self, Dipole other) {
    return AntiScalar(0.0 - self.g0.x * other.g1.x - self.g0.y * other.g1.y - self.g0.z * other.g1.z - self.g1.x * other.g0.x - self.g1.y * other.g0.y - self.g1.z * other.g0.z);
}

Motor motor__wedge__dual_num(Motor self, DualNum other) {
    return Motor(self.g0 * vec4(other.g0.x), self.g1 * vec3(other.g0.x));
}

MultiVector motor__wedge__multi_vector(Motor self, MultiVector other) {
    return MultiVector(vec2(self.g0.x) * vec2(0.0, -other.g4.x) + vec2(self.g0.y) * vec2(0.0, -other.g4.y) + vec2(self.g0.z) * vec2(0.0, -other.g4.z) + vec2(self.g0.w) * vec2(0.0, other.g0.x) + vec2(self.g1.x) * vec2(0.0, -other.g3.x) + vec2(self.g1.y) * vec2(0.0, -other.g3.y) + vec2(self.g1.z) * vec2(0.0, -other.g3.z), vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0), vec4(0.0), vec4(0.0), vec3(self.g0.x, self.g0.y, self.g0.z) * vec3(other.g0.x), self.g1 * vec3(other.g0.x), vec3(self.g0.x) * vec3(0.0, other.g1.z, -other.g1.y) + vec3(self.g0.y) * vec3(-other.g1.z, 0.0, other.g1.x) + vec3(self.g0.z) * vec3(other.g1.y, -other.g1.x, 0.0) - self.g1 * vec3(other.g2.x), vec2(self.g1.x) * vec2(0.0, other.g1.x) + vec2(self.g1.y) * vec2(0.0, other.g1.y) + vec2(self.g1.z) * vec2(0.0, other.g1.z));
}

Plane motor__wedge__round_point(Motor self, RoundPoint other) {
    return Plane(vec4(self.g0.x) * vec4(0.0, other.g0.z, -other.g0.y, 0.0) + vec4(self.g0.y) * vec4(-other.g0.z, 0.0, other.g0.x, 0.0) + vec4(self.g0.z) * vec4(other.g0.y, -other.g0.x, 0.0, 0.0) + vec4(self.g1.x) * vec4(-other.g1.x, 0.0, 0.0, other.g0.x) + vec4(self.g1.y) * vec4(0.0, -other.g1.x, 0.0, other.g0.y) + vec4(self.g1.z) * vec4(0.0, 0.0, -other.g1.x, other.g0.z));
}

Motor motor__wedge__scalar(Motor self, Scalar other) {
    return Motor(self.g0 * vec4(other.g0), self.g1 * vec3(other.g0));
}

AntiScalar multi_vector__wedge__anti_scalar(MultiVector self, AntiScalar other) {
    return AntiScalar(self.g0.x * other.g0);
}

MultiVector multi_vector__wedge__circle(MultiVector self, Circle other) {
    return MultiVector(vec2(self.g3.x) * vec2(0.0, -other.g2.x) + vec2(self.g3.y) * vec2(0.0, -other.g2.y) + vec2(self.g3.z) * vec2(0.0, -other.g2.z) + vec2(self.g4.x) * vec2(0.0, -other.g1.x) + vec2(self.g4.y) * vec2(0.0, -other.g1.y) + vec2(self.g4.z) * vec2(0.0, -other.g1.z) + vec2(self.g5.x) * vec2(0.0, -other.g0.x) + vec2(self.g5.y) * vec2(0.0, -other.g0.y) + vec2(self.g5.z) * vec2(0.0, -other.g0.z) + vec2(self.g5.w) * vec2(0.0, -other.g0.w), vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0), vec4(0.0), vec4(self.g0.x) * other.g0, vec3(self.g0.x) * other.g1, vec3(self.g0.x) * other.g2, vec3(self.g1.x) * vec3(0.0, other.g1.z, -other.g1.y) + vec3(self.g1.y) * vec3(-other.g1.z, 0.0, other.g1.x) + vec3(self.g1.z) * vec3(other.g1.y, -other.g1.x, 0.0) + vec3(self.g2.x) * other.g2 - vec3(self.g2.y) * vec3(other.g0.x, other.g0.y, other.g0.z), vec2(self.g1.x) * vec2(other.g0.x, -other.g2.x) + vec2(self.g1.y) * vec2(other.g0.y, -other.g2.y) + vec2(self.g1.z) * vec2(other.g0.z, -other.g2.z) + self.g2 * vec2(other.g0.w));
}

MultiVector multi_vector__wedge__dipole(MultiVector self, Dipole other) {
    return MultiVector(vec2(self.g6.x) * vec2(0.0, -other.g2.x) + vec2(self.g6.y) * vec2(0.0, -other.g2.y) + vec2(self.g6.z) * vec2(0.0, -other.g2.z) + vec2(self.g6.w) * vec2(0.0, -other.g2.w) + vec2(self.g7.x) * vec2(0.0, -other.g1.x) + vec2(self.g7.y) * vec2(0.0, -other.g1.y) + vec2(self.g7.z) * vec2(0.0, -other.g1.z) + vec2(self.g8.x) * vec2(0.0, -other.g0.x) + vec2(self.g8.y) * vec2(0.0, -other.g0.y) + vec2(self.g8.z) * vec2(0.0, -other.g0.z), vec3(0.0), vec2(0.0), vec3(self.g0.x) * other.g0, vec3(self.g0.x) * other.g1, vec4(self.g0.x) * other.g2, vec4(self.g1.x) * vec4(0.0, other.g0.z, -other.g0.y, -other.g1.x) + vec4(self.g1.y) * vec4(-other.g0.z, 0.0, other.g0.x, -other.g1.y) + vec4(self.g1.z) * vec4(other.g0.y, -other.g0.x, 0.0, -other.g1.z) + vec4(self.g2.x) * vec4(other.g1.x, other.g1.y, other.g1.z, 0.0), vec3(0.0) - self.g1 * vec3(other.g2.w) + vec3(self.g2.x) * vec3(other.g2.x, other.g2.y, other.g2.z) + vec3(self.g2.y) * other.g0, vec3(self.g1.x) * vec3(0.0, -other.g2.z, other.g2.y) + vec3(self.g1.y) * vec3(other.g2.z, 0.0, -other.g2.x) + vec3(self.g1.z) * vec3(-other.g2.y, other.g2.x, 0.0) + vec3(self.g2.y) * other.g1, vec3(self.g3.x) * vec3(0.0, -other.g2.z, other.g2.y) + vec3(self.g3.y) * vec3(other.g2.z, 0.0, -other.g2.x) + vec3(self.g3.z) * vec3(-other.g2.y, other.g2.x, 0.0) + self.g4 * vec3(other.g2.w) + vec3(self.g5.x) * vec3(0.0, other.g0.z, -other.g0.y) + vec3(self.g5.y) * vec3(-other.g0.z, 0.0, other.g0.x) + vec3(self.g5.z) * vec3(other.g0.y, -other.g0.x, 0.0) + vec3(self.g5.w) * other.g1, vec2(self.g3.x) * vec2(-other.g1.x, 0.0) + vec2(self.g3.y) * vec2(-other.g1.y, 0.0) + vec2(self.g3.z) * vec2(-other.g1.z, 0.0) - vec2(self.g4.x) * vec2(other.g0.x, other.g2.x) - vec2(self.g4.y) * vec2(other.g0.y, other.g2.y) - vec2(self.g4.z) * vec2(other.g0.z, other.g2.z) + vec2(self.g5.x) * vec2(0.0, -other.g1.x) + vec2(self.g5.y) * vec2(0.0, -other.g1.y) + vec2(self.g5.z) * vec2(0.0, -other.g1.z));
}

MultiVector multi_vector__wedge__dual_num(MultiVector self, DualNum other) {
    return MultiVector(vec2(self.g0.x) * other.g0 + vec2(self.g0.y) * vec2(0.0, other.g0.x), self.g1 * vec3(other.g0.x), self.g2 * vec2(other.g0.x), self.g3 * vec3(other.g0.x), self.g4 * vec3(other.g0.x), self.g5 * vec4(other.g0.x), self.g6 * vec4(other.g0.x), self.g7 * vec3(other.g0.x), self.g8 * vec3(other.g0.x), self.g9 * vec3(other.g0.x), self.g10 * vec2(other.g0.x));
}

MultiVector multi_vector__wedge__flat_point(MultiVector self, FlatPoint other) {
    return MultiVector(vec2(self.g6.x) * vec2(0.0, -other.g0.x) + vec2(self.g6.y) * vec2(0.0, -other.g0.y) + vec2(self.g6.z) * vec2(0.0, -other.g0.z) + vec2(self.g6.w) * vec2(0.0, -other.g0.w), vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0), vec4(self.g0.x) * other.g0, vec4(0.0), vec3(0.0) - self.g1 * vec3(other.g0.w) + vec3(self.g2.x) * vec3(other.g0.x, other.g0.y, other.g0.z), vec3(self.g1.x) * vec3(0.0, -other.g0.z, other.g0.y) + vec3(self.g1.y) * vec3(other.g0.z, 0.0, -other.g0.x) + vec3(self.g1.z) * vec3(-other.g0.y, other.g0.x, 0.0), vec3(self.g3.x) * vec3(0.0, -other.g0.z, other.g0.y) + vec3(self.g3.y) * vec3(other.g0.z, 0.0, -other.g0.x) + vec3(self.g3.z) * vec3(-other.g0.y, other.g0.x, 0.0) + self.g4 * vec3(other.g0.w), vec2(self.g4.x) * vec2(0.0, -other.g0.x) + vec2(self.g4.y) * vec2(0.0, -other.g0.y) + vec2(self.g4.z) * vec2(0.0, -other.g0.z));
}

MultiVector multi_vector__wedge__flector(MultiVector self, Flector other) {
    return MultiVector(vec2(self.g1.x) * vec2(0.0, other.g1.x) + vec2(self.g1.y) * vec2(0.0, other.g1.y) + vec2(self.g1.z) * vec2(0.0, other.g1.z) + vec2(self.g2.x) * vec2(0.0, other.g1.w) + vec2(self.g6.x) * vec2(0.0, -other.g0.x) + vec2(self.g6.y) * vec2(0.0, -other.g0.y) + vec2(self.g6.z) * vec2(0.0, -other.g0.z) + vec2(self.g6.w) * vec2(0.0, -other.g0.w), vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0), vec4(self.g0.x) * other.g0, vec4(0.0), vec3(0.0) - self.g1 * vec3(other.g0.w) + vec3(self.g2.x) * vec3(other.g0.x, other.g0.y, other.g0.z), vec3(self.g1.x) * vec3(0.0, -other.g0.z, other.g0.y) + vec3(self.g1.y) * vec3(other.g0.z, 0.0, -other.g0.x) + vec3(self.g1.z) * vec3(-other.g0.y, other.g0.x, 0.0), vec3(self.g0.x) * vec3(other.g1.x, other.g1.y, other.g1.z) + vec3(self.g3.x) * vec3(0.0, -other.g0.z, other.g0.y) + vec3(self.g3.y) * vec3(other.g0.z, 0.0, -other.g0.x) + vec3(self.g3.z) * vec3(-other.g0.y, other.g0.x, 0.0) + self.g4 * vec3(other.g0.w), vec2(self.g0.x) * vec2(0.0, other.g1.w) + vec2(self.g4.x) * vec2(0.0, -other.g0.x) + vec2(self.g4.y) * vec2(0.0, -other.g0.y) + vec2(self.g4.z) * vec2(0.0, -other.g0.z));
}

MultiVector multi_vector__wedge__line(MultiVector self, Line other) {
    return MultiVector(vec2(self.g3.x) * vec2(0.0, -other.g1.x) + vec2(self.g3.y) * vec2(0.0, -other.g1.y) + vec2(self.g3.z) * vec2(0.0, -other.g1.z) + vec2(self.g4.x) * vec2(0.0, -other.g0.x) + vec2(self.g4.y) * vec2(0.0, -other.g0.y) + vec2(self.g4.z) * vec2(0.0, -other.g0.z), vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0), vec4(0.0), vec4(0.0), vec3(self.g0.x) * other.g0, vec3(self.g0.x) * other.g1, vec3(self.g1.x) * vec3(0.0, other.g0.z, -other.g0.y) + vec3(self.g1.y) * vec3(-other.g0.z, 0.0, other.g0.x) + vec3(self.g1.z) * vec3(other.g0.y, -other.g0.x, 0.0) + vec3(self.g2.x) * other.g1, vec2(self.g1.x) * vec2(0.0, -other.g1.x) + vec2(self.g1.y) * vec2(0.0, -other.g1.y) + vec2(self.g1.z) * vec2(0.0, -other.g1.z));
}

MultiVector multi_vector__wedge__motor(MultiVector self, Motor other) {
    return MultiVector(vec2(self.g0.x) * vec2(0.0, other.g0.w) + vec2(self.g3.x) * vec2(0.0, -other.g1.x) + vec2(self.g3.y) * vec2(0.0, -other.g1.y) + vec2(self.g3.z) * vec2(0.0, -other.g1.z) + vec2(self.g4.x) * vec2(0.0, -other.g0.x) + vec2(self.g4.y) * vec2(0.0, -other.g0.y) + vec2(self.g4.z) * vec2(0.0, -other.g0.z), vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0), vec4(0.0), vec4(0.0), vec3(self.g0.x) * vec3(other.g0.x, other.g0.y, other.g0.z), vec3(self.g0.x) * other.g1, vec3(self.g1.x) * vec3(0.0, other.g0.z, -other.g0.y) + vec3(self.g1.y) * vec3(-other.g0.z, 0.0, other.g0.x) + vec3(self.g1.z) * vec3(other.g0.y, -other.g0.x, 0.0) + vec3(self.g2.x) * other.g1, vec2(self.g1.x) * vec2(0.0, -other.g1.x) + vec2(self.g1.y) * vec2(0.0, -other.g1.y) + vec2(self.g1.z) * vec2(0.0, -other.g1.z));
}

MultiVector multi_vector__wedge__multi_vector(MultiVector self, MultiVector other) {
    return MultiVector(vec2(self.g0.x) * other.g0 + vec2(self.g0.y) * vec2(0.0, other.g0.x) + vec2(self.g1.x) * vec2(0.0, other.g9.x) + vec2(self.g1.y) * vec2(0.0, other.g9.y) + vec2(self.g1.z) * vec2(0.0, other.g9.z) + vec2(self.g2.x) * vec2(0.0, other.g10.y) + vec2(self.g2.y) * vec2(0.0, other.g10.x) + vec2(self.g3.x) * vec2(0.0, -other.g8.x) + vec2(self.g3.y) * vec2(0.0, -other.g8.y) + vec2(self.g3.z) * vec2(0.0, -other.g8.z) + vec2(self.g4.x) * vec2(0.0, -other.g7.x) + vec2(self.g4.y) * vec2(0.0, -other.g7.y) + vec2(self.g4.z) * vec2(0.0, -other.g7.z) + vec2(self.g5.x) * vec2(0.0, -other.g6.x) + vec2(self.g5.y) * vec2(0.0, -other.g6.y) + vec2(self.g5.z) * vec2(0.0, -other.g6.z) + vec2(self.g5.w) * vec2(0.0, -other.g6.w) + vec2(self.g6.x) * vec2(0.0, -other.g5.x) + vec2(self.g6.y) * vec2(0.0, -other.g5.y) + vec2(self.g6.z) * vec2(0.0, -other.g5.z) + vec2(self.g6.w) * vec2(0.0, -other.g5.w) + vec2(self.g7.x) * vec2(0.0, -other.g4.x) + vec2(self.g7.y) * vec2(0.0, -other.g4.y) + vec2(self.g7.z) * vec2(0.0, -other.g4.z) + vec2(self.g8.x) * vec2(0.0, -other.g3.x) + vec2(self.g8.y) * vec2(0.0, -other.g3.y) + vec2(self.g8.z) * vec2(0.0, -other.g3.z) + vec2(self.g9.x) * vec2(0.0, other.g1.x) + vec2(self.g9.y) * vec2(0.0, other.g1.y) + vec2(self.g9.z) * vec2(0.0, other.g1.z) + vec2(self.g10.x) * vec2(0.0, other.g2.y) + vec2(self.g10.y) * vec2(0.0, other.g2.x), vec3(self.g0.x) * other.g1 + self.g1 * vec3(other.g0.x), vec2(self.g0.x) * other.g2 + self.g2 * vec2(other.g0.x), vec3(self.g0.x) * other.g3 - self.g1 * vec3(other.g2.x) + vec3(self.g2.x) * other.g1 + self.g3 * vec3(other.g0.x), vec3(self.g0.x) * other.g4 + vec3(self.g1.x) * vec3(0.0, -other.g1.z, other.g1.y) + vec3(self.g1.y) * vec3(other.g1.z, 0.0, -other.g1.x) + vec3(self.g1.z) * vec3(-other.g1.y, other.g1.x, 0.0) + self.g4 * vec3(other.g0.x), vec4(self.g0.x) * other.g5 + vec4(self.g1.x, self.g1.y, self.g1.z, self.g1.x) * vec4(other.g2.y, other.g2.y, other.g2.y, 0.0) + vec4(self.g2.x) * vec4(0.0, 0.0, 0.0, other.g2.y) - vec4(self.g2.y) * vec4(other.g1.x, other.g1.y, other.g1.z, other.g2.x) + self.g5 * vec4(other.g0.x), vec4(self.g0.x) * other.g6 + vec4(self.g1.x) * vec4(0.0, other.g3.z, -other.g3.y, -other.g4.x) + vec4(self.g1.y) * vec4(-other.g3.z, 0.0, other.g3.x, -other.g4.y) + vec4(self.g1.z) * vec4(other.g3.y, -other.g3.x, 0.0, -other.g4.z) + vec4(self.g2.x) * vec4(other.g4.x, other.g4.y, other.g4.z, 0.0) + vec4(self.g3.x) * vec4(0.0, -other.g1.z, other.g1.y, 0.0) + vec4(self.g3.y) * vec4(other.g1.z, 0.0, -other.g1.x, 0.0) + vec4(self.g3.z) * vec4(-other.g1.y, other.g1.x, 0.0, 0.0) + vec4(self.g4.x) * vec4(other.g2.x, 0.0, 0.0, -other.g1.x) + vec4(self.g4.y) * vec4(0.0, other.g2.x, 0.0, -other.g1.y) + vec4(self.g4.z) * vec4(0.0, 0.0, other.g2.x, -other.g1.z) + self.g6 * vec4(other.g0.x), vec3(self.g0.x) * other.g7 - self.g1 * vec3(other.g5.w) + vec3(self.g2.x) * vec3(other.g5.x, other.g5.y, other.g5.z) + vec3(self.g2.y) * other.g3 + self.g3 * vec3(other.g2.y) + vec3(self.g5.x, self.g5.y, self.g5.z) * vec3(other.g2.x) - vec3(self.g5.w) * other.g1 + self.g7 * vec3(other.g0.x), vec3(self.g0.x) * other.g8 + vec3(self.g1.x) * vec3(0.0, -other.g5.z, other.g5.y) + vec3(self.g1.y) * vec3(other.g5.z, 0.0, -other.g5.x) + vec3(self.g1.z) * vec3(-other.g5.y, other.g5.x, 0.0) + vec3(self.g2.y) * other.g4 + self.g4 * vec3(other.g2.y) + vec3(self.g5.x) * vec3(0.0, other.g1.z, -other.g1.y) + vec3(self.g5.y) * vec3(-other.g1.z, 0.0, other.g1.x) + vec3(self.g5.z) * vec3(other.g1.y, -other.g1.x, 0.0) + self.g8 * vec3(other.g0.x), vec3(self.g0.x) * other.g9 + vec3(self.g1.x) * vec3(0.0, other.g7.z, -other.g7.y) + vec3(self.g1.y) * vec3(-other.g7.z, 0.0, other.g7.x) + vec3(self.g1.z) * vec3(other.g7.y, -other.g7.x, 0.0) + vec3(self.g2.x) * other.g8 - vec3(self.g2.y) * vec3(other.g6.x, other.g6.y, other.g6.z) + vec3(self.g3.x) * vec3(0.0, -other.g5.z, other.g5.y) + vec3(self.g3.y) * vec3(other.g5.z, 0.0, -other.g5.x) + vec3(self.g3.z) * vec3(-other.g5.y, other.g5.x, 0.0) + self.g4 * vec3(other.g5.w) + vec3(self.g5.x) * vec3(0.0, other.g3.z, -other.g3.y) + vec3(self.g5.y) * vec3(-other.g3.z, 0.0, other.g3.x) + vec3(self.g5.z) * vec3(other.g3.y, -other.g3.x, 0.0) + vec3(self.g5.w) * other.g4 + vec3(self.g6.x, self.g6.y, self.g6.z) * vec3(other.g2.y) + vec3(self.g7.x) * vec3(0.0, other.g1.z, -other.g1.y) + vec3(self.g7.y) * vec3(-other.g1.z, 0.0, other.g1.x) + vec3(self.g7.z) * vec3(other.g1.y, -other.g1.x, 0.0) - self.g8 * vec3(other.g2.x) + self.g9 * vec3(other.g0.x), vec2(self.g0.x) * other.g10 + vec2(self.g1.x) * vec2(other.g6.x, -other.g8.x) + vec2(self.g1.y) * vec2(other.g6.y, -other.g8.y) + vec2(self.g1.z) * vec2(other.g6.z, -other.g8.z) + self.g2 * vec2(other.g6.w) + vec2(self.g3.x) * vec2(-other.g4.x, 0.0) + vec2(self.g3.y) * vec2(-other.g4.y, 0.0) + vec2(self.g3.z) * vec2(-other.g4.z, 0.0) - vec2(self.g4.x) * vec2(other.g3.x, other.g5.x) - vec2(self.g4.y) * vec2(other.g3.y, other.g5.y) - vec2(self.g4.z) * vec2(other.g3.z, other.g5.z) + vec2(self.g5.x) * vec2(0.0, -other.g4.x) + vec2(self.g5.y) * vec2(0.0, -other.g4.y) + vec2(self.g5.z) * vec2(0.0, -other.g4.z) + vec2(self.g6.x) * vec2(-other.g1.x, 0.0) + vec2(self.g6.y) * vec2(-other.g1.y, 0.0) + vec2(self.g6.z) * vec2(-other.g1.z, 0.0) + vec2(self.g6.w) * other.g2 * vec2(-1.0, 1.0) + vec2(self.g8.x) * vec2(0.0, other.g1.x) + vec2(self.g8.y) * vec2(0.0, other.g1.y) + vec2(self.g8.z) * vec2(0.0, other.g1.z) + self.g10 * vec2(other.g0.x));
}

MultiVector multi_vector__wedge__plane(MultiVector self, Plane other) {
    return MultiVector(vec2(self.g1.x) * vec2(0.0, other.g0.x) + vec2(self.g1.y) * vec2(0.0, other.g0.y) + vec2(self.g1.z) * vec2(0.0, other.g0.z) + vec2(self.g2.x) * vec2(0.0, other.g0.w), vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0), vec4(0.0), vec4(0.0), vec3(0.0), vec3(0.0), vec3(self.g0.x) * vec3(other.g0.x, other.g0.y, other.g0.z), vec2(self.g0.x) * vec2(0.0, other.g0.w));
}

MultiVector multi_vector__wedge__round_point(MultiVector self, RoundPoint other) {
    return MultiVector(vec2(self.g9.x) * vec2(0.0, other.g0.x) + vec2(self.g9.y) * vec2(0.0, other.g0.y) + vec2(self.g9.z) * vec2(0.0, other.g0.z) + vec2(self.g10.x) * vec2(0.0, other.g1.y) + vec2(self.g10.y) * vec2(0.0, other.g1.x), vec3(self.g0.x) * other.g0, vec2(self.g0.x) * other.g1, vec3(0.0) - self.g1 * vec3(other.g1.x) + vec3(self.g2.x) * other.g0, vec3(self.g1.x) * vec3(0.0, -other.g0.z, other.g0.y) + vec3(self.g1.y) * vec3(other.g0.z, 0.0, -other.g0.x) + vec3(self.g1.z) * vec3(-other.g0.y, other.g0.x, 0.0), vec4(self.g1.x, self.g1.y, self.g1.z, self.g1.x) * vec4(other.g1.y, other.g1.y, other.g1.y, 0.0) + vec4(self.g2.x) * vec4(0.0, 0.0, 0.0, other.g1.y) - vec4(self.g2.y) * vec4(other.g0.x, other.g0.y, other.g0.z, other.g1.x), vec4(self.g3.x) * vec4(0.0, -other.g0.z, other.g0.y, 0.0) + vec4(self.g3.y) * vec4(other.g0.z, 0.0, -other.g0.x, 0.0) + vec4(self.g3.z) * vec4(-other.g0.y, other.g0.x, 0.0, 0.0) + vec4(self.g4.x) * vec4(other.g1.x, 0.0, 0.0, -other.g0.x) + vec4(self.g4.y) * vec4(0.0, other.g1.x, 0.0, -other.g0.y) + vec4(self.g4.z) * vec4(0.0, 0.0, other.g1.x, -other.g0.z), self.g3 * vec3(other.g1.y) + vec3(self.g5.x, self.g5.y, self.g5.z) * vec3(other.g1.x) - vec3(self.g5.w) * other.g0, self.g4 * vec3(other.g1.y) + vec3(self.g5.x) * vec3(0.0, other.g0.z, -other.g0.y) + vec3(self.g5.y) * vec3(-other.g0.z, 0.0, other.g0.x) + vec3(self.g5.z) * vec3(other.g0.y, -other.g0.x, 0.0), vec3(self.g6.x, self.g6.y, self.g6.z) * vec3(other.g1.y) + vec3(self.g7.x) * vec3(0.0, other.g0.z, -other.g0.y) + vec3(self.g7.y) * vec3(-other.g0.z, 0.0, other.g0.x) + vec3(self.g7.z) * vec3(other.g0.y, -other.g0.x, 0.0) - self.g8 * vec3(other.g1.x), vec2(self.g6.x) * vec2(-other.g0.x, 0.0) + vec2(self.g6.y) * vec2(-other.g0.y, 0.0) + vec2(self.g6.z) * vec2(-other.g0.z, 0.0) + vec2(self.g6.w) * other.g1 * vec2(-1.0, 1.0) + vec2(self.g8.x) * vec2(0.0, other.g0.x) + vec2(self.g8.y) * vec2(0.0, other.g0.y) + vec2(self.g8.z) * vec2(0.0, other.g0.z));
}

MultiVector multi_vector__wedge__scalar(MultiVector self, Scalar other) {
    return MultiVector(self.g0 * vec2(other.g0), self.g1 * vec3(other.g0), self.g2 * vec2(other.g0), self.g3 * vec3(other.g0), self.g4 * vec3(other.g0), self.g5 * vec4(other.g0), self.g6 * vec4(other.g0), self.g7 * vec3(other.g0), self.g8 * vec3(other.g0), self.g9 * vec3(other.g0), self.g10 * vec2(other.g0));
}

MultiVector multi_vector__wedge__sphere(MultiVector self, Sphere other) {
    return MultiVector(vec2(self.g1.x) * vec2(0.0, other.g0.x) + vec2(self.g1.y) * vec2(0.0, other.g0.y) + vec2(self.g1.z) * vec2(0.0, other.g0.z) + vec2(self.g2.x) * vec2(0.0, other.g1.y) + vec2(self.g2.y) * vec2(0.0, other.g1.x), vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0), vec4(0.0), vec4(0.0), vec3(0.0), vec3(0.0), vec3(self.g0.x) * other.g0, vec2(self.g0.x) * other.g1);
}

Plane plane__wedge__dual_num(Plane self, DualNum other) {
    return Plane(self.g0 * vec4(other.g0.x));
}

MultiVector plane__wedge__multi_vector(Plane self, MultiVector other) {
    return MultiVector(vec2(self.g0.x) * vec2(0.0, other.g1.x) + vec2(self.g0.y) * vec2(0.0, other.g1.y) + vec2(self.g0.z) * vec2(0.0, other.g1.z) + vec2(self.g0.w) * vec2(0.0, other.g2.x), vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0), vec4(0.0), vec4(0.0), vec3(0.0), vec3(0.0), vec3(self.g0.x, self.g0.y, self.g0.z) * vec3(other.g0.x), vec2(self.g0.w) * vec2(0.0, other.g0.x));
}

AntiScalar plane__wedge__round_point(Plane self, RoundPoint other) {
    return AntiScalar(self.g0.x * other.g0.x + self.g0.y * other.g0.y + self.g0.z * other.g0.z + self.g0.w * other.g1.x);
}

Plane plane__wedge__scalar(Plane self, Scalar other) {
    return Plane(self.g0 * vec4(other.g0));
}

Sphere round_point__wedge__circle(RoundPoint self, Circle other) {
    return Sphere(vec3(self.g0.x) * vec3(0.0, other.g1.z, -other.g1.y) + vec3(self.g0.y) * vec3(-other.g1.z, 0.0, other.g1.x) + vec3(self.g0.z) * vec3(other.g1.y, -other.g1.x, 0.0) + vec3(self.g1.x) * other.g2 - vec3(self.g1.y) * vec3(other.g0.x, other.g0.y, other.g0.z), vec2(self.g0.x) * vec2(other.g0.x, -other.g2.x) + vec2(self.g0.y) * vec2(other.g0.y, -other.g2.y) + vec2(self.g0.z) * vec2(other.g0.z, -other.g2.z) + self.g1 * vec2(other.g0.w));
}

Circle round_point__wedge__dipole(RoundPoint self, Dipole other) {
    return Circle(vec4(self.g0.x) * vec4(0.0, other.g0.z, -other.g0.y, -other.g1.x) + vec4(self.g0.y) * vec4(-other.g0.z, 0.0, other.g0.x, -other.g1.y) + vec4(self.g0.z) * vec4(other.g0.y, -other.g0.x, 0.0, -other.g1.z) + vec4(self.g1.x) * vec4(other.g1.x, other.g1.y, other.g1.z, 0.0), vec3(0.0) - self.g0 * vec3(other.g2.w) + vec3(self.g1.x) * vec3(other.g2.x, other.g2.y, other.g2.z) + vec3(self.g1.y) * other.g0, vec3(self.g0.x) * vec3(0.0, -other.g2.z, other.g2.y) + vec3(self.g0.y) * vec3(other.g2.z, 0.0, -other.g2.x) + vec3(self.g0.z) * vec3(-other.g2.y, other.g2.x, 0.0) + vec3(self.g1.y) * other.g1);
}

RoundPoint round_point__wedge__dual_num(RoundPoint self, DualNum other) {
    return RoundPoint(self.g0 * vec3(other.g0.x), self.g1 * vec2(other.g0.x));
}

Line round_point__wedge__flat_point(RoundPoint self, FlatPoint other) {
    return Line(vec3(0.0) - self.g0 * vec3(other.g0.w) + vec3(self.g1.x) * vec3(other.g0.x, other.g0.y, other.g0.z), vec3(self.g0.x) * vec3(0.0, -other.g0.z, other.g0.y) + vec3(self.g0.y) * vec3(other.g0.z, 0.0, -other.g0.x) + vec3(self.g0.z) * vec3(-other.g0.y, other.g0.x, 0.0));
}

Motor round_point__wedge__flector(RoundPoint self, Flector other) {
    return Motor(vec4(self.g0.x) * vec4(-other.g0.w, 0.0, 0.0, other.g1.x) + vec4(self.g0.y) * vec4(0.0, -other.g0.w, 0.0, other.g1.y) + vec4(self.g0.z) * vec4(0.0, 0.0, -other.g0.w, other.g1.z) + vec4(self.g1.x) * vec4(other.g0.x, other.g0.y, other.g0.z, other.g1.w), vec3(self.g0.x) * vec3(0.0, -other.g0.z, other.g0.y) + vec3(self.g0.y) * vec3(other.g0.z, 0.0, -other.g0.x) + vec3(self.g0.z) * vec3(-other.g0.y, other.g0.x, 0.0));
}

Plane round_point__wedge__line(RoundPoint self, Line other) {
    return Plane(vec4(self.g0.x) * vec4(0.0, other.g0.z, -other.g0.y, -other.g1.x) + vec4(self.g0.y) * vec4(-other.g0.z, 0.0, other.g0.x, -other.g1.y) + vec4(self.g0.z) * vec4(other.g0.y, -other.g0.x, 0.0, -other.g1.z) + vec4(self.g1.x) * vec4(other.g1.x, other.g1.y, other.g1.z, 0.0));
}

Plane round_point__wedge__motor(RoundPoint self, Motor other) {
    return Plane(vec4(self.g0.x) * vec4(0.0, other.g0.z, -other.g0.y, -other.g1.x) + vec4(self.g0.y) * vec4(-other.g0.z, 0.0, other.g0.x, -other.g1.y) + vec4(self.g0.z) * vec4(other.g0.y, -other.g0.x, 0.0, -other.g1.z) + vec4(self.g1.x) * vec4(other.g1.x, other.g1.y, other.g1.z, 0.0));
}

MultiVector round_point__wedge__multi_vector(RoundPoint self, MultiVector other) {
    return MultiVector(vec2(self.g0.x) * vec2(0.0, other.g9.x) + vec2(self.g0.y) * vec2(0.0, other.g9.y) + vec2(self.g0.z) * vec2(0.0, other.g9.z) + vec2(self.g1.x) * vec2(0.0, other.g10.y) + vec2(self.g1.y) * vec2(0.0, other.g10.x), self.g0 * vec3(other.g0.x), self.g1 * vec2(other.g0.x), vec3(0.0) - self.g0 * vec3(other.g2.x) + vec3(self.g1.x) * other.g1, vec3(self.g0.x) * vec3(0.0, -other.g1.z, other.g1.y) + vec3(self.g0.y) * vec3(other.g1.z, 0.0, -other.g1.x) + vec3(self.g0.z) * vec3(-other.g1.y, other.g1.x, 0.0), vec4(self.g0.x, self.g0.y, self.g0.z, self.g0.x) * vec4(other.g2.y, other.g2.y, other.g2.y, 0.0) + vec4(self.g1.x) * vec4(0.0, 0.0, 0.0, other.g2.y) - vec4(self.g1.y) * vec4(other.g1.x, other.g1.y, other.g1.z, other.g2.x), vec4(self.g0.x) * vec4(0.0, other.g3.z, -other.g3.y, -other.g4.x) + vec4(self.g0.y) * vec4(-other.g3.z, 0.0, other.g3.x, -other.g4.y) + vec4(self.g0.z) * vec4(other.g3.y, -other.g3.x, 0.0, -other.g4.z) + vec4(self.g1.x) * vec4(other.g4.x, other.g4.y, other.g4.z, 0.0), vec3(0.0) - self.g0 * vec3(other.g5.w) + vec3(self.g1.x) * vec3(other.g5.x, other.g5.y, other.g5.z) + vec3(self.g1.y) * other.g3, vec3(self.g0.x) * vec3(0.0, -other.g5.z, other.g5.y) + vec3(self.g0.y) * vec3(other.g5.z, 0.0, -other.g5.x) + vec3(self.g0.z) * vec3(-other.g5.y, other.g5.x, 0.0) + vec3(self.g1.y) * other.g4, vec3(self.g0.x) * vec3(0.0, other.g7.z, -other.g7.y) + vec3(self.g0.y) * vec3(-other.g7.z, 0.0, other.g7.x) + vec3(self.g0.z) * vec3(other.g7.y, -other.g7.x, 0.0) + vec3(self.g1.x) * other.g8 - vec3(self.g1.y) * vec3(other.g6.x, other.g6.y, other.g6.z), vec2(self.g0.x) * vec2(other.g6.x, -other.g8.x) + vec2(self.g0.y) * vec2(other.g6.y, -other.g8.y) + vec2(self.g0.z) * vec2(other.g6.z, -other.g8.z) + self.g1 * vec2(other.g6.w));
}

AntiScalar round_point__wedge__plane(RoundPoint self, Plane other) {
    return AntiScalar(self.g0.x * other.g0.x + self.g0.y * other.g0.y + self.g0.z * other.g0.z + self.g1.x * other.g0.w);
}

Dipole round_point__wedge__round_point(RoundPoint self, RoundPoint other) {
    return Dipole(vec3(0.0) - self.g0 * vec3(other.g1.x) + vec3(self.g1.x) * other.g0, vec3(self.g0.x) * vec3(0.0, -other.g0.z, other.g0.y) + vec3(self.g0.y) * vec3(other.g0.z, 0.0, -other.g0.x) + vec3(self.g0.z) * vec3(-other.g0.y, other.g0.x, 0.0), vec4(self.g0.x, self.g0.y, self.g0.z, self.g0.x) * vec4(other.g1.y, other.g1.y, other.g1.y, 0.0) + vec4(self.g1.x) * vec4(0.0, 0.0, 0.0, other.g1.y) - vec4(self.g1.y) * vec4(other.g0.x, other.g0.y, other.g0.z, other.g1.x));
}

RoundPoint round_point__wedge__scalar(RoundPoint self, Scalar other) {
    return RoundPoint(self.g0 * vec3(other.g0), self.g1 * vec2(other.g0));
}

AntiScalar round_point__wedge__sphere(RoundPoint self, Sphere other) {
    return AntiScalar(self.g0.x * other.g0.x + self.g0.y * other.g0.y + self.g0.z * other.g0.z + self.g1.x * other.g1.y + self.g1.y * other.g1.x);
}

AntiScalar scalar__wedge__anti_scalar(Scalar self, AntiScalar other) {
    return AntiScalar(self.g0 * other.g0);
}

Circle scalar__wedge__circle(Scalar self, Circle other) {
    return Circle(vec4(self.g0) * other.g0, vec3(self.g0) * other.g1, vec3(self.g0) * other.g2);
}

Dipole scalar__wedge__dipole(Scalar self, Dipole other) {
    return Dipole(vec3(self.g0) * other.g0, vec3(self.g0) * other.g1, vec4(self.g0) * other.g2);
}

DualNum scalar__wedge__dual_num(Scalar self, DualNum other) {
    return DualNum(vec2(self.g0) * other.g0);
}

FlatPoint scalar__wedge__flat_point(Scalar self, FlatPoint other) {
    return FlatPoint(vec4(self.g0) * other.g0);
}

Flector scalar__wedge__flector(Scalar self, Flector other) {
    return Flector(vec4(self.g0) * other.g0, vec4(self.g0) * other.g1);
}

Line scalar__wedge__line(Scalar self, Line other) {
    return Line(vec3(self.g0) * other.g0, vec3(self.g0) * other.g1);
}

Motor scalar__wedge__motor(Scalar self, Motor other) {
    return Motor(vec4(self.g0) * other.g0, vec3(self.g0) * other.g1);
}

MultiVector scalar__wedge__multi_vector(Scalar self, MultiVector other) {
    return MultiVector(vec2(self.g0) * other.g0, vec3(self.g0) * other.g1, vec2(self.g0) * other.g2, vec3(self.g0) * other.g3, vec3(self.g0) * other.g4, vec4(self.g0) * other.g5, vec4(self.g0) * other.g6, vec3(self.g0) * other.g7, vec3(self.g0) * other.g8, vec3(self.g0) * other.g9, vec2(self.g0) * other.g10);
}

Plane scalar__wedge__plane(Scalar self, Plane other) {
    return Plane(vec4(self.g0) * other.g0);
}

RoundPoint scalar__wedge__round_point(Scalar self, RoundPoint other) {
    return RoundPoint(vec3(self.g0) * other.g0, vec2(self.g0) * other.g1);
}

Scalar scalar__wedge__scalar(Scalar self, Scalar other) {
    return Scalar(self.g0 * other.g0);
}

Sphere scalar__wedge__sphere(Scalar self, Sphere other) {
    return Sphere(vec3(self.g0) * other.g0, vec2(self.g0) * other.g1);
}

Sphere sphere__wedge__dual_num(Sphere self, DualNum other) {
    return Sphere(self.g0 * vec3(other.g0.x), self.g1 * vec2(other.g0.x));
}

MultiVector sphere__wedge__multi_vector(Sphere self, MultiVector other) {
    return MultiVector(vec2(self.g0.x) * vec2(0.0, other.g1.x) + vec2(self.g0.y) * vec2(0.0, other.g1.y) + vec2(self.g0.z) * vec2(0.0, other.g1.z) + vec2(self.g1.x) * vec2(0.0, other.g2.y) + vec2(self.g1.y) * vec2(0.0, other.g2.x), vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0), vec4(0.0), vec4(0.0), vec3(0.0), vec3(0.0), self.g0 * vec3(other.g0.x), self.g1 * vec2(other.g0.x));
}

AntiScalar sphere__wedge__round_point(Sphere self, RoundPoint other) {
    return AntiScalar(self.g0.x * other.g0.x + self.g0.y * other.g0.y + self.g0.z * other.g0.z + self.g1.x * other.g1.y + self.g1.y * other.g1.x);
}

Sphere sphere__wedge__scalar(Sphere self, Scalar other) {
    return Sphere(self.g0 * vec3(other.g0), self.g1 * vec2(other.g0));
}

AntiScalar anti_scalar__anti_dot__anti_scalar(AntiScalar self, AntiScalar other) {
    return AntiScalar(self.g0 * other.g0);
}

AntiScalar anti_scalar__anti_dot__dual_num(AntiScalar self, DualNum other) {
    return AntiScalar(self.g0 * other.g0.y);
}

AntiScalar anti_scalar__anti_dot__motor(AntiScalar self, Motor other) {
    return AntiScalar(self.g0 * other.g0.w);
}

AntiScalar anti_scalar__anti_dot__multi_vector(AntiScalar self, MultiVector other) {
    return AntiScalar(self.g0 * other.g0.y);
}

AntiScalar circle__anti_dot__circle(Circle self, Circle other) {
    return AntiScalar(0.0 - self.g0.x * other.g2.x - self.g0.y * other.g2.y - self.g0.z * other.g2.z + self.g0.w * other.g0.w - self.g1.x * other.g1.x - self.g1.y * other.g1.y - self.g1.z * other.g1.z - self.g2.x * other.g0.x - self.g2.y * other.g0.y - self.g2.z * other.g0.z);
}

AntiScalar circle__anti_dot__line(Circle self, Line other) {
    return AntiScalar(0.0 - self.g0.x * other.g1.x - self.g0.y * other.g1.y - self.g0.z * other.g1.z - self.g1.x * other.g0.x - self.g1.y * other.g0.y - self.g1.z * other.g0.z);
}

AntiScalar circle__anti_dot__motor(Circle self, Motor other) {
    return AntiScalar(0.0 - self.g0.x * other.g1.x - self.g0.y * other.g1.y - self.g0.z * other.g1.z - self.g1.x * other.g0.x - self.g1.y * other.g0.y - self.g1.z * other.g0.z);
}

AntiScalar circle__anti_dot__multi_vector(Circle self, MultiVector other) {
    return AntiScalar(0.0 - self.g0.x * other.g8.x - self.g0.y * other.g8.y - self.g0.z * other.g8.z + self.g0.w * other.g6.w - self.g1.x * other.g7.x - self.g1.y * other.g7.y - self.g1.z * other.g7.z - self.g2.x * other.g6.x - self.g2.y * other.g6.y - self.g2.z * other.g6.z);
}

AntiScalar dipole__anti_dot__dipole(Dipole self, Dipole other) {
    return AntiScalar(self.g0.x * other.g2.x + self.g0.y * other.g2.y + self.g0.z * other.g2.z + self.g1.x * other.g1.x + self.g1.y * other.g1.y + self.g1.z * other.g1.z + self.g2.x * other.g0.x + self.g2.y * other.g0.y + self.g2.z * other.g0.z - self.g2.w * other.g2.w);
}

AntiScalar dipole__anti_dot__flat_point(Dipole self, FlatPoint other) {
    return AntiScalar(self.g0.x * other.g0.x + self.g0.y * other.g0.y + self.g0.z * other.g0.z - self.g2.w * other.g0.w);
}

AntiScalar dipole__anti_dot__flector(Dipole self, Flector other) {
    return AntiScalar(self.g0.x * other.g0.x + self.g0.y * other.g0.y + self.g0.z * other.g0.z - self.g2.w * other.g0.w);
}

AntiScalar dipole__anti_dot__multi_vector(Dipole self, MultiVector other) {
    return AntiScalar(self.g0.x * other.g5.x + self.g0.y * other.g5.y + self.g0.z * other.g5.z + self.g1.x * other.g4.x + self.g1.y * other.g4.y + self.g1.z * other.g4.z + self.g2.x * other.g3.x + self.g2.y * other.g3.y + self.g2.z * other.g3.z - self.g2.w * other.g5.w);
}

AntiScalar dual_num__anti_dot__anti_scalar(DualNum self, AntiScalar other) {
    return AntiScalar(self.g0.y * other.g0);
}

AntiScalar dual_num__anti_dot__dual_num(DualNum self, DualNum other) {
    return AntiScalar(0.0 - self.g0.x * other.g0.x + self.g0.y * other.g0.y);
}

AntiScalar dual_num__anti_dot__motor(DualNum self, Motor other) {
    return AntiScalar(self.g0.y * other.g0.w);
}

AntiScalar dual_num__anti_dot__multi_vector(DualNum self, MultiVector other) {
    return AntiScalar(0.0 - self.g0.x * other.g0.x + self.g0.y * other.g0.y);
}

AntiScalar dual_num__anti_dot__scalar(DualNum self, Scalar other) {
    return AntiScalar(0.0 - self.g0.x * other.g0);
}

AntiScalar flat_point__anti_dot__dipole(FlatPoint self, Dipole other) {
    return AntiScalar(self.g0.x * other.g0.x + self.g0.y * other.g0.y + self.g0.z * other.g0.z - self.g0.w * other.g2.w);
}

AntiScalar flat_point__anti_dot__flat_point(FlatPoint self, FlatPoint other) {
    return AntiScalar(0.0 - self.g0.w * other.g0.w);
}

AntiScalar flat_point__anti_dot__flector(FlatPoint self, Flector other) {
    return AntiScalar(0.0 - self.g0.w * other.g0.w);
}

AntiScalar flat_point__anti_dot__multi_vector(FlatPoint self, MultiVector other) {
    return AntiScalar(self.g0.x * other.g3.x + self.g0.y * other.g3.y + self.g0.z * other.g3.z - self.g0.w * other.g5.w);
}

AntiScalar flector__anti_dot__dipole(Flector self, Dipole other) {
    return AntiScalar(self.g0.x * other.g0.x + self.g0.y * other.g0.y + self.g0.z * other.g0.z - self.g0.w * other.g2.w);
}

AntiScalar flector__anti_dot__flat_point(Flector self, FlatPoint other) {
    return AntiScalar(0.0 - self.g0.w * other.g0.w);
}

AntiScalar flector__anti_dot__flector(Flector self, Flector other) {
    return AntiScalar(0.0 - self.g0.w * other.g0.w + self.g1.x * other.g1.x + self.g1.y * other.g1.y + self.g1.z * other.g1.z);
}

AntiScalar flector__anti_dot__multi_vector(Flector self, MultiVector other) {
    return AntiScalar(self.g0.x * other.g3.x + self.g0.y * other.g3.y + self.g0.z * other.g3.z - self.g0.w * other.g5.w + self.g1.x * other.g9.x + self.g1.y * other.g9.y + self.g1.z * other.g9.z - self.g1.w * other.g10.x);
}

AntiScalar flector__anti_dot__plane(Flector self, Plane other) {
    return AntiScalar(self.g1.x * other.g0.x + self.g1.y * other.g0.y + self.g1.z * other.g0.z);
}

AntiScalar flector__anti_dot__sphere(Flector self, Sphere other) {
    return AntiScalar(self.g1.x * other.g0.x + self.g1.y * other.g0.y + self.g1.z * other.g0.z - self.g1.w * other.g1.x);
}

AntiScalar line__anti_dot__circle(Line self, Circle other) {
    return AntiScalar(0.0 - self.g0.x * other.g1.x - self.g0.y * other.g1.y - self.g0.z * other.g1.z - self.g1.x * other.g0.x - self.g1.y * other.g0.y - self.g1.z * other.g0.z);
}

AntiScalar line__anti_dot__line(Line self, Line other) {
    return AntiScalar(0.0 - self.g0.x * other.g0.x - self.g0.y * other.g0.y - self.g0.z * other.g0.z);
}

AntiScalar line__anti_dot__motor(Line self, Motor other) {
    return AntiScalar(0.0 - self.g0.x * other.g0.x - self.g0.y * other.g0.y - self.g0.z * other.g0.z);
}

AntiScalar line__anti_dot__multi_vector(Line self, MultiVector other) {
    return AntiScalar(0.0 - self.g0.x * other.g7.x - self.g0.y * other.g7.y - self.g0.z * other.g7.z - self.g1.x * other.g6.x - self.g1.y * other.g6.y - self.g1.z * other.g6.z);
}

AntiScalar motor__anti_dot__anti_scalar(Motor self, AntiScalar other) {
    return AntiScalar(self.g0.w * other.g0);
}

AntiScalar motor__anti_dot__circle(Motor self, Circle other) {
    return AntiScalar(0.0 - self.g0.x * other.g1.x - self.g0.y * other.g1.y - self.g0.z * other.g1.z - self.g1.x * other.g0.x - self.g1.y * other.g0.y - self.g1.z * other.g0.z);
}

AntiScalar motor__anti_dot__dual_num(Motor self, DualNum other) {
    return AntiScalar(self.g0.w * other.g0.y);
}

AntiScalar motor__anti_dot__line(Motor self, Line other) {
    return AntiScalar(0.0 - self.g0.x * other.g0.x - self.g0.y * other.g0.y - self.g0.z * other.g0.z);
}

AntiScalar motor__anti_dot__motor(Motor self, Motor other) {
    return AntiScalar(0.0 - self.g0.x * other.g0.x - self.g0.y * other.g0.y - self.g0.z * other.g0.z + self.g0.w * other.g0.w);
}

AntiScalar motor__anti_dot__multi_vector(Motor self, MultiVector other) {
    return AntiScalar(0.0 - self.g0.x * other.g7.x - self.g0.y * other.g7.y - self.g0.z * other.g7.z + self.g0.w * other.g0.y - self.g1.x * other.g6.x - self.g1.y * other.g6.y - self.g1.z * other.g6.z);
}

AntiScalar multi_vector__anti_dot__anti_scalar(MultiVector self, AntiScalar other) {
    return AntiScalar(self.g0.y * other.g0);
}

AntiScalar multi_vector__anti_dot__circle(MultiVector self, Circle other) {
    return AntiScalar(0.0 - self.g6.x * other.g2.x - self.g6.y * other.g2.y - self.g6.z * other.g2.z + self.g6.w * other.g0.w - self.g7.x * other.g1.x - self.g7.y * other.g1.y - self.g7.z * other.g1.z - self.g8.x * other.g0.x - self.g8.y * other.g0.y - self.g8.z * other.g0.z);
}

AntiScalar multi_vector__anti_dot__dipole(MultiVector self, Dipole other) {
    return AntiScalar(self.g3.x * other.g2.x + self.g3.y * other.g2.y + self.g3.z * other.g2.z + self.g4.x * other.g1.x + self.g4.y * other.g1.y + self.g4.z * other.g1.z + self.g5.x * other.g0.x + self.g5.y * other.g0.y + self.g5.z * other.g0.z - self.g5.w * other.g2.w);
}

AntiScalar multi_vector__anti_dot__dual_num(MultiVector self, DualNum other) {
    return AntiScalar(0.0 - self.g0.x * other.g0.x + self.g0.y * other.g0.y);
}

AntiScalar multi_vector__anti_dot__flat_point(MultiVector self, FlatPoint other) {
    return AntiScalar(self.g3.x * other.g0.x + self.g3.y * other.g0.y + self.g3.z * other.g0.z - self.g5.w * other.g0.w);
}

AntiScalar multi_vector__anti_dot__flector(MultiVector self, Flector other) {
    return AntiScalar(self.g3.x * other.g0.x + self.g3.y * other.g0.y + self.g3.z * other.g0.z - self.g5.w * other.g0.w + self.g9.x * other.g1.x + self.g9.y * other.g1.y + self.g9.z * other.g1.z - self.g10.x * other.g1.w);
}

AntiScalar multi_vector__anti_dot__line(MultiVector self, Line other) {
    return AntiScalar(0.0 - self.g6.x * other.g1.x - self.g6.y * other.g1.y - self.g6.z * other.g1.z - self.g7.x * other.g0.x - self.g7.y * other.g0.y - self.g7.z * other.g0.z);
}

AntiScalar multi_vector__anti_dot__motor(MultiVector self, Motor other) {
    return AntiScalar(self.g0.y * other.g0.w - self.g6.x * other.g1.x - self.g6.y * other.g1.y - self.g6.z * other.g1.z - self.g7.x * other.g0.x - self.g7.y * other.g0.y - self.g7.z * other.g0.z);
}

AntiScalar multi_vector__anti_dot__multi_vector(MultiVector self, MultiVector other) {
    return AntiScalar(0.0 - self.g0.x * other.g0.x + self.g0.y * other.g0.y - self.g1.x * other.g1.x - self.g1.y * other.g1.y - self.g1.z * other.g1.z + self.g2.x * other.g2.y + self.g2.y * other.g2.x + self.g3.x * other.g5.x + self.g3.y * other.g5.y + self.g3.z * other.g5.z + self.g4.x * other.g4.x + self.g4.y * other.g4.y + self.g4.z * other.g4.z + self.g5.x * other.g3.x + self.g5.y * other.g3.y + self.g5.z * other.g3.z - self.g5.w * other.g5.w - self.g6.x * other.g8.x - self.g6.y * other.g8.y - self.g6.z * other.g8.z + self.g6.w * other.g6.w - self.g7.x * other.g7.x - self.g7.y * other.g7.y - self.g7.z * other.g7.z - self.g8.x * other.g6.x - self.g8.y * other.g6.y - self.g8.z * other.g6.z + self.g9.x * other.g9.x + self.g9.y * other.g9.y + self.g9.z * other.g9.z - self.g10.x * other.g10.y - self.g10.y * other.g10.x);
}

AntiScalar multi_vector__anti_dot__plane(MultiVector self, Plane other) {
    return AntiScalar(self.g9.x * other.g0.x + self.g9.y * other.g0.y + self.g9.z * other.g0.z - self.g10.x * other.g0.w);
}

AntiScalar multi_vector__anti_dot__round_point(MultiVector self, RoundPoint other) {
    return AntiScalar(0.0 - self.g1.x * other.g0.x - self.g1.y * other.g0.y - self.g1.z * other.g0.z + self.g2.x * other.g1.y + self.g2.y * other.g1.x);
}

AntiScalar multi_vector__anti_dot__scalar(MultiVector self, Scalar other) {
    return AntiScalar(0.0 - self.g0.x * other.g0);
}

AntiScalar multi_vector__anti_dot__sphere(MultiVector self, Sphere other) {
    return AntiScalar(self.g9.x * other.g0.x + self.g9.y * other.g0.y + self.g9.z * other.g0.z - self.g10.x * other.g1.y - self.g10.y * other.g1.x);
}

AntiScalar plane__anti_dot__flector(Plane self, Flector other) {
    return AntiScalar(self.g0.x * other.g1.x + self.g0.y * other.g1.y + self.g0.z * other.g1.z);
}

AntiScalar plane__anti_dot__multi_vector(Plane self, MultiVector other) {
    return AntiScalar(self.g0.x * other.g9.x + self.g0.y * other.g9.y + self.g0.z * other.g9.z - self.g0.w * other.g10.x);
}

AntiScalar plane__anti_dot__plane(Plane self, Plane other) {
    return AntiScalar(self.g0.x * other.g0.x + self.g0.y * other.g0.y + self.g0.z * other.g0.z);
}

AntiScalar plane__anti_dot__sphere(Plane self, Sphere other) {
    return AntiScalar(self.g0.x * other.g0.x + self.g0.y * other.g0.y + self.g0.z * other.g0.z - self.g0.w * other.g1.x);
}

AntiScalar round_point__anti_dot__multi_vector(RoundPoint self, MultiVector other) {
    return AntiScalar(0.0 - self.g0.x * other.g1.x - self.g0.y * other.g1.y - self.g0.z * other.g1.z + self.g1.x * other.g2.y + self.g1.y * other.g2.x);
}

AntiScalar round_point__anti_dot__round_point(RoundPoint self, RoundPoint other) {
    return AntiScalar(0.0 - self.g0.x * other.g0.x - self.g0.y * other.g0.y - self.g0.z * other.g0.z + self.g1.x * other.g1.y + self.g1.y * other.g1.x);
}

AntiScalar scalar__anti_dot__dual_num(Scalar self, DualNum other) {
    return AntiScalar(0.0 - self.g0 * other.g0.x);
}

AntiScalar scalar__anti_dot__multi_vector(Scalar self, MultiVector other) {
    return AntiScalar(0.0 - self.g0 * other.g0.x);
}

AntiScalar scalar__anti_dot__scalar(Scalar self, Scalar other) {
    return AntiScalar(0.0 - self.g0 * other.g0);
}

AntiScalar sphere__anti_dot__flector(Sphere self, Flector other) {
    return AntiScalar(self.g0.x * other.g1.x + self.g0.y * other.g1.y + self.g0.z * other.g1.z - self.g1.x * other.g1.w);
}

AntiScalar sphere__anti_dot__multi_vector(Sphere self, MultiVector other) {
    return AntiScalar(self.g0.x * other.g9.x + self.g0.y * other.g9.y + self.g0.z * other.g9.z - self.g1.x * other.g10.y - self.g1.y * other.g10.x);
}

AntiScalar sphere__anti_dot__plane(Sphere self, Plane other) {
    return AntiScalar(self.g0.x * other.g0.x + self.g0.y * other.g0.y + self.g0.z * other.g0.z - self.g1.x * other.g0.w);
}

AntiScalar sphere__anti_dot__sphere(Sphere self, Sphere other) {
    return AntiScalar(self.g0.x * other.g0.x + self.g0.y * other.g0.y + self.g0.z * other.g0.z - self.g1.x * other.g1.y - self.g1.y * other.g1.x);
}

Scalar anti_scalar__dot__anti_scalar(AntiScalar self, AntiScalar other) {
    return Scalar(0.0 - self.g0 * other.g0);
}

Scalar anti_scalar__dot__dual_num(AntiScalar self, DualNum other) {
    return Scalar(0.0 - self.g0 * other.g0.y);
}

Scalar anti_scalar__dot__motor(AntiScalar self, Motor other) {
    return Scalar(0.0 - self.g0 * other.g0.w);
}

Scalar anti_scalar__dot__multi_vector(AntiScalar self, MultiVector other) {
    return Scalar(0.0 - self.g0 * other.g0.y);
}

Scalar circle__dot__circle(Circle self, Circle other) {
    return Scalar(self.g0.x * other.g2.x + self.g0.y * other.g2.y + self.g0.z * other.g2.z - self.g0.w * other.g0.w + self.g1.x * other.g1.x + self.g1.y * other.g1.y + self.g1.z * other.g1.z + self.g2.x * other.g0.x + self.g2.y * other.g0.y + self.g2.z * other.g0.z);
}

Scalar circle__dot__line(Circle self, Line other) {
    return Scalar(self.g0.x * other.g1.x + self.g0.y * other.g1.y + self.g0.z * other.g1.z + self.g1.x * other.g0.x + self.g1.y * other.g0.y + self.g1.z * other.g0.z);
}

Scalar circle__dot__motor(Circle self, Motor other) {
    return Scalar(self.g0.x * other.g1.x + self.g0.y * other.g1.y + self.g0.z * other.g1.z + self.g1.x * other.g0.x + self.g1.y * other.g0.y + self.g1.z * other.g0.z);
}

Scalar circle__dot__multi_vector(Circle self, MultiVector other) {
    return Scalar(self.g0.x * other.g8.x + self.g0.y * other.g8.y + self.g0.z * other.g8.z - self.g0.w * other.g6.w + self.g1.x * other.g7.x + self.g1.y * other.g7.y + self.g1.z * other.g7.z + self.g2.x * other.g6.x + self.g2.y * other.g6.y + self.g2.z * other.g6.z);
}

Scalar dipole__dot__dipole(Dipole self, Dipole other) {
    return Scalar(0.0 - self.g0.x * other.g2.x - self.g0.y * other.g2.y - self.g0.z * other.g2.z - self.g1.x * other.g1.x - self.g1.y * other.g1.y - self.g1.z * other.g1.z - self.g2.x * other.g0.x - self.g2.y * other.g0.y - self.g2.z * other.g0.z + self.g2.w * other.g2.w);
}

Scalar dipole__dot__flat_point(Dipole self, FlatPoint other) {
    return Scalar(0.0 - self.g0.x * other.g0.x - self.g0.y * other.g0.y - self.g0.z * other.g0.z + self.g2.w * other.g0.w);
}

Scalar dipole__dot__flector(Dipole self, Flector other) {
    return Scalar(0.0 - self.g0.x * other.g0.x - self.g0.y * other.g0.y - self.g0.z * other.g0.z + self.g2.w * other.g0.w);
}

Scalar dipole__dot__multi_vector(Dipole self, MultiVector other) {
    return Scalar(0.0 - self.g0.x * other.g5.x - self.g0.y * other.g5.y - self.g0.z * other.g5.z - self.g1.x * other.g4.x - self.g1.y * other.g4.y - self.g1.z * other.g4.z - self.g2.x * other.g3.x - self.g2.y * other.g3.y - self.g2.z * other.g3.z + self.g2.w * other.g5.w);
}

Scalar dual_num__dot__anti_scalar(DualNum self, AntiScalar other) {
    return Scalar(0.0 - self.g0.y * other.g0);
}

Scalar dual_num__dot__dual_num(DualNum self, DualNum other) {
    return Scalar(self.g0.x * other.g0.x - self.g0.y * other.g0.y);
}

Scalar dual_num__dot__motor(DualNum self, Motor other) {
    return Scalar(0.0 - self.g0.y * other.g0.w);
}

Scalar dual_num__dot__multi_vector(DualNum self, MultiVector other) {
    return Scalar(self.g0.x * other.g0.x - self.g0.y * other.g0.y);
}

Scalar dual_num__dot__scalar(DualNum self, Scalar other) {
    return Scalar(self.g0.x * other.g0);
}

Scalar flat_point__dot__dipole(FlatPoint self, Dipole other) {
    return Scalar(0.0 - self.g0.x * other.g0.x - self.g0.y * other.g0.y - self.g0.z * other.g0.z + self.g0.w * other.g2.w);
}

Scalar flat_point__dot__flat_point(FlatPoint self, FlatPoint other) {
    return Scalar(self.g0.w * other.g0.w);
}

Scalar flat_point__dot__flector(FlatPoint self, Flector other) {
    return Scalar(self.g0.w * other.g0.w);
}

Scalar flat_point__dot__multi_vector(FlatPoint self, MultiVector other) {
    return Scalar(0.0 - self.g0.x * other.g3.x - self.g0.y * other.g3.y - self.g0.z * other.g3.z + self.g0.w * other.g5.w);
}

Scalar flector__dot__dipole(Flector self, Dipole other) {
    return Scalar(0.0 - self.g0.x * other.g0.x - self.g0.y * other.g0.y - self.g0.z * other.g0.z + self.g0.w * other.g2.w);
}

Scalar flector__dot__flat_point(Flector self, FlatPoint other) {
    return Scalar(self.g0.w * other.g0.w);
}

Scalar flector__dot__flector(Flector self, Flector other) {
    return Scalar(self.g0.w * other.g0.w - self.g1.x * other.g1.x - self.g1.y * other.g1.y - self.g1.z * other.g1.z);
}

Scalar flector__dot__multi_vector(Flector self, MultiVector other) {
    return Scalar(0.0 - self.g0.x * other.g3.x - self.g0.y * other.g3.y - self.g0.z * other.g3.z + self.g0.w * other.g5.w - self.g1.x * other.g9.x - self.g1.y * other.g9.y - self.g1.z * other.g9.z + self.g1.w * other.g10.x);
}

Scalar flector__dot__plane(Flector self, Plane other) {
    return Scalar(0.0 - self.g1.x * other.g0.x - self.g1.y * other.g0.y - self.g1.z * other.g0.z);
}

Scalar flector__dot__sphere(Flector self, Sphere other) {
    return Scalar(0.0 - self.g1.x * other.g0.x - self.g1.y * other.g0.y - self.g1.z * other.g0.z + self.g1.w * other.g1.x);
}

Scalar line__dot__circle(Line self, Circle other) {
    return Scalar(self.g0.x * other.g1.x + self.g0.y * other.g1.y + self.g0.z * other.g1.z + self.g1.x * other.g0.x + self.g1.y * other.g0.y + self.g1.z * other.g0.z);
}

Scalar line__dot__line(Line self, Line other) {
    return Scalar(self.g0.x * other.g0.x + self.g0.y * other.g0.y + self.g0.z * other.g0.z);
}

Scalar line__dot__motor(Line self, Motor other) {
    return Scalar(self.g0.x * other.g0.x + self.g0.y * other.g0.y + self.g0.z * other.g0.z);
}

Scalar line__dot__multi_vector(Line self, MultiVector other) {
    return Scalar(self.g0.x * other.g7.x + self.g0.y * other.g7.y + self.g0.z * other.g7.z + self.g1.x * other.g6.x + self.g1.y * other.g6.y + self.g1.z * other.g6.z);
}

Scalar motor__dot__anti_scalar(Motor self, AntiScalar other) {
    return Scalar(0.0 - self.g0.w * other.g0);
}

Scalar motor__dot__circle(Motor self, Circle other) {
    return Scalar(self.g0.x * other.g1.x + self.g0.y * other.g1.y + self.g0.z * other.g1.z + self.g1.x * other.g0.x + self.g1.y * other.g0.y + self.g1.z * other.g0.z);
}

Scalar motor__dot__dual_num(Motor self, DualNum other) {
    return Scalar(0.0 - self.g0.w * other.g0.y);
}

Scalar motor__dot__line(Motor self, Line other) {
    return Scalar(self.g0.x * other.g0.x + self.g0.y * other.g0.y + self.g0.z * other.g0.z);
}

Scalar motor__dot__motor(Motor self, Motor other) {
    return Scalar(self.g0.x * other.g0.x + self.g0.y * other.g0.y + self.g0.z * other.g0.z - self.g0.w * other.g0.w);
}

Scalar motor__dot__multi_vector(Motor self, MultiVector other) {
    return Scalar(self.g0.x * other.g7.x + self.g0.y * other.g7.y + self.g0.z * other.g7.z - self.g0.w * other.g0.y + self.g1.x * other.g6.x + self.g1.y * other.g6.y + self.g1.z * other.g6.z);
}

Scalar multi_vector__dot__anti_scalar(MultiVector self, AntiScalar other) {
    return Scalar(0.0 - self.g0.y * other.g0);
}

Scalar multi_vector__dot__circle(MultiVector self, Circle other) {
    return Scalar(self.g6.x * other.g2.x + self.g6.y * other.g2.y + self.g6.z * other.g2.z - self.g6.w * other.g0.w + self.g7.x * other.g1.x + self.g7.y * other.g1.y + self.g7.z * other.g1.z + self.g8.x * other.g0.x + self.g8.y * other.g0.y + self.g8.z * other.g0.z);
}

Scalar multi_vector__dot__dipole(MultiVector self, Dipole other) {
    return Scalar(0.0 - self.g3.x * other.g2.x - self.g3.y * other.g2.y - self.g3.z * other.g2.z - self.g4.x * other.g1.x - self.g4.y * other.g1.y - self.g4.z * other.g1.z - self.g5.x * other.g0.x - self.g5.y * other.g0.y - self.g5.z * other.g0.z + self.g5.w * other.g2.w);
}

Scalar multi_vector__dot__dual_num(MultiVector self, DualNum other) {
    return Scalar(self.g0.x * other.g0.x - self.g0.y * other.g0.y);
}

Scalar multi_vector__dot__flat_point(MultiVector self, FlatPoint other) {
    return Scalar(0.0 - self.g3.x * other.g0.x - self.g3.y * other.g0.y - self.g3.z * other.g0.z + self.g5.w * other.g0.w);
}

Scalar multi_vector__dot__flector(MultiVector self, Flector other) {
    return Scalar(0.0 - self.g3.x * other.g0.x - self.g3.y * other.g0.y - self.g3.z * other.g0.z + self.g5.w * other.g0.w - self.g9.x * other.g1.x - self.g9.y * other.g1.y - self.g9.z * other.g1.z + self.g10.x * other.g1.w);
}

Scalar multi_vector__dot__line(MultiVector self, Line other) {
    return Scalar(self.g6.x * other.g1.x + self.g6.y * other.g1.y + self.g6.z * other.g1.z + self.g7.x * other.g0.x + self.g7.y * other.g0.y + self.g7.z * other.g0.z);
}

Scalar multi_vector__dot__motor(MultiVector self, Motor other) {
    return Scalar(0.0 - self.g0.y * other.g0.w + self.g6.x * other.g1.x + self.g6.y * other.g1.y + self.g6.z * other.g1.z + self.g7.x * other.g0.x + self.g7.y * other.g0.y + self.g7.z * other.g0.z);
}

Scalar multi_vector__dot__multi_vector(MultiVector self, MultiVector other) {
    return Scalar(self.g0.x * other.g0.x - self.g0.y * other.g0.y + self.g1.x * other.g1.x + self.g1.y * other.g1.y + self.g1.z * other.g1.z - self.g2.x * other.g2.y - self.g2.y * other.g2.x - self.g3.x * other.g5.x - self.g3.y * other.g5.y - self.g3.z * other.g5.z - self.g4.x * other.g4.x - self.g4.y * other.g4.y - self.g4.z * other.g4.z - self.g5.x * other.g3.x - self.g5.y * other.g3.y - self.g5.z * other.g3.z + self.g5.w * other.g5.w + self.g6.x * other.g8.x + self.g6.y * other.g8.y + self.g6.z * other.g8.z - self.g6.w * other.g6.w + self.g7.x * other.g7.x + self.g7.y * other.g7.y + self.g7.z * other.g7.z + self.g8.x * other.g6.x + self.g8.y * other.g6.y + self.g8.z * other.g6.z - self.g9.x * other.g9.x - self.g9.y * other.g9.y - self.g9.z * other.g9.z + self.g10.x * other.g10.y + self.g10.y * other.g10.x);
}

Scalar multi_vector__dot__plane(MultiVector self, Plane other) {
    return Scalar(0.0 - self.g9.x * other.g0.x - self.g9.y * other.g0.y - self.g9.z * other.g0.z + self.g10.x * other.g0.w);
}

Scalar multi_vector__dot__round_point(MultiVector self, RoundPoint other) {
    return Scalar(self.g1.x * other.g0.x + self.g1.y * other.g0.y + self.g1.z * other.g0.z - self.g2.x * other.g1.y - self.g2.y * other.g1.x);
}

Scalar multi_vector__dot__scalar(MultiVector self, Scalar other) {
    return Scalar(self.g0.x * other.g0);
}

Scalar multi_vector__dot__sphere(MultiVector self, Sphere other) {
    return Scalar(0.0 - self.g9.x * other.g0.x - self.g9.y * other.g0.y - self.g9.z * other.g0.z + self.g10.x * other.g1.y + self.g10.y * other.g1.x);
}

Scalar plane__dot__flector(Plane self, Flector other) {
    return Scalar(0.0 - self.g0.x * other.g1.x - self.g0.y * other.g1.y - self.g0.z * other.g1.z);
}

Scalar plane__dot__multi_vector(Plane self, MultiVector other) {
    return Scalar(0.0 - self.g0.x * other.g9.x - self.g0.y * other.g9.y - self.g0.z * other.g9.z + self.g0.w * other.g10.x);
}

Scalar plane__dot__plane(Plane self, Plane other) {
    return Scalar(0.0 - self.g0.x * other.g0.x - self.g0.y * other.g0.y - self.g0.z * other.g0.z);
}

Scalar plane__dot__sphere(Plane self, Sphere other) {
    return Scalar(0.0 - self.g0.x * other.g0.x - self.g0.y * other.g0.y - self.g0.z * other.g0.z + self.g0.w * other.g1.x);
}

Scalar round_point__dot__multi_vector(RoundPoint self, MultiVector other) {
    return Scalar(self.g0.x * other.g1.x + self.g0.y * other.g1.y + self.g0.z * other.g1.z - self.g1.x * other.g2.y - self.g1.y * other.g2.x);
}

Scalar round_point__dot__round_point(RoundPoint self, RoundPoint other) {
    return Scalar(self.g0.x * other.g0.x + self.g0.y * other.g0.y + self.g0.z * other.g0.z - self.g1.x * other.g1.y - self.g1.y * other.g1.x);
}

Scalar scalar__dot__dual_num(Scalar self, DualNum other) {
    return Scalar(self.g0 * other.g0.x);
}

Scalar scalar__dot__multi_vector(Scalar self, MultiVector other) {
    return Scalar(self.g0 * other.g0.x);
}

Scalar scalar__dot__scalar(Scalar self, Scalar other) {
    return Scalar(self.g0 * other.g0);
}

Scalar sphere__dot__flector(Sphere self, Flector other) {
    return Scalar(0.0 - self.g0.x * other.g1.x - self.g0.y * other.g1.y - self.g0.z * other.g1.z + self.g1.x * other.g1.w);
}

Scalar sphere__dot__multi_vector(Sphere self, MultiVector other) {
    return Scalar(0.0 - self.g0.x * other.g9.x - self.g0.y * other.g9.y - self.g0.z * other.g9.z + self.g1.x * other.g10.y + self.g1.y * other.g10.x);
}

Scalar sphere__dot__plane(Sphere self, Plane other) {
    return Scalar(0.0 - self.g0.x * other.g0.x - self.g0.y * other.g0.y - self.g0.z * other.g0.z + self.g1.x * other.g0.w);
}

Scalar sphere__dot__sphere(Sphere self, Sphere other) {
    return Scalar(0.0 - self.g0.x * other.g0.x - self.g0.y * other.g0.y - self.g0.z * other.g0.z + self.g1.x * other.g1.y + self.g1.y * other.g1.x);
}

Circle circle__flat_bulk(Circle self) {
    return Circle(vec4(0.0), vec3(0.0), self.g2);
}

Dipole dipole__flat_bulk(Dipole self) {
    return Dipole(vec3(0.0), vec3(0.0), self.g2 * vec4(1.0, 1.0, 1.0, 0.0));
}

FlatPoint flat_point__flat_bulk(FlatPoint self) {
    return FlatPoint(self.g0 * vec4(1.0, 1.0, 1.0, 0.0));
}

Flector flector__flat_bulk(Flector self) {
    return Flector(self.g0 * vec4(1.0, 1.0, 1.0, 0.0), self.g1 * vec4(0.0, 0.0, 0.0, 1.0));
}

Line line__flat_bulk(Line self) {
    return Line(vec3(0.0), self.g1);
}

Motor motor__flat_bulk(Motor self) {
    return Motor(vec4(0.0), self.g1);
}

MultiVector multi_vector__flat_bulk(MultiVector self) {
    return MultiVector(vec2(0.0), vec3(0.0), self.g2 * vec2(0.0, 1.0), vec3(0.0), vec3(0.0), self.g5 * vec4(1.0, 1.0, 1.0, 0.0), vec4(0.0), vec3(0.0), self.g8, vec3(0.0), self.g10 * vec2(0.0, 1.0));
}

Plane plane__flat_bulk(Plane self) {
    return Plane(self.g0 * vec4(0.0, 0.0, 0.0, 1.0));
}

RoundPoint round_point__flat_bulk(RoundPoint self) {
    return RoundPoint(vec3(0.0), self.g1 * vec2(0.0, 1.0));
}

Sphere sphere__flat_bulk(Sphere self) {
    return Sphere(vec3(0.0), self.g1 * vec2(0.0, 1.0));
}

AntiScalar anti_scalar__flat_weight(AntiScalar self) {
    return self;
}

Circle circle__flat_weight(Circle self) {
    return Circle(vec4(0.0), self.g1, vec3(0.0));
}

Dipole dipole__flat_weight(Dipole self) {
    return Dipole(vec3(0.0), vec3(0.0), self.g2 * vec4(0.0, 0.0, 0.0, 1.0));
}

DualNum dual_num__flat_weight(DualNum self) {
    return DualNum(self.g0 * vec2(0.0, 1.0));
}

FlatPoint flat_point__flat_weight(FlatPoint self) {
    return FlatPoint(self.g0 * vec4(0.0, 0.0, 0.0, 1.0));
}

Flector flector__flat_weight(Flector self) {
    return Flector(self.g0 * vec4(0.0, 0.0, 0.0, 1.0), self.g1 * vec4(1.0, 1.0, 1.0, 0.0));
}

Line line__flat_weight(Line self) {
    return Line(self.g0, vec3(0.0));
}

Motor motor__flat_weight(Motor self) {
    return Motor(self.g0, vec3(0.0));
}

MultiVector multi_vector__flat_weight(MultiVector self) {
    return MultiVector(self.g0 * vec2(0.0, 1.0), vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0), self.g5 * vec4(0.0, 0.0, 0.0, 1.0), vec4(0.0), self.g7, vec3(0.0), self.g9, vec2(0.0));
}

Plane plane__flat_weight(Plane self) {
    return Plane(self.g0 * vec4(1.0, 1.0, 1.0, 0.0));
}

Sphere sphere__flat_weight(Sphere self) {
    return Sphere(self.g0, vec2(0.0));
}

Circle circle__round_bulk(Circle self) {
    return Circle(self.g0 * vec4(0.0, 0.0, 0.0, 1.0), vec3(0.0), vec3(0.0));
}

Dipole dipole__round_bulk(Dipole self) {
    return Dipole(vec3(0.0), self.g1, vec4(0.0));
}

DualNum dual_num__round_bulk(DualNum self) {
    return DualNum(self.g0 * vec2(1.0, 0.0));
}

MultiVector multi_vector__round_bulk(MultiVector self) {
    return MultiVector(self.g0 * vec2(1.0, 0.0), self.g1, vec2(0.0), vec3(0.0), self.g4, vec4(0.0), self.g6 * vec4(0.0, 0.0, 0.0, 1.0), vec3(0.0), vec3(0.0), vec3(0.0), vec2(0.0));
}

RoundPoint round_point__round_bulk(RoundPoint self) {
    return RoundPoint(self.g0, vec2(0.0));
}

Scalar scalar__round_bulk(Scalar self) {
    return self;
}

Circle circle__round_weight(Circle self) {
    return Circle(self.g0 * vec4(1.0, 1.0, 1.0, 0.0), vec3(0.0), vec3(0.0));
}

Dipole dipole__round_weight(Dipole self) {
    return Dipole(self.g0, vec3(0.0), vec4(0.0));
}

MultiVector multi_vector__round_weight(MultiVector self) {
    return MultiVector(vec2(0.0), vec3(0.0), self.g2 * vec2(1.0, 0.0), self.g3, vec3(0.0), vec4(0.0), self.g6 * vec4(1.0, 1.0, 1.0, 0.0), vec3(0.0), vec3(0.0), vec3(0.0), self.g10 * vec2(1.0, 0.0));
}

RoundPoint round_point__round_weight(RoundPoint self) {
    return RoundPoint(vec3(0.0), self.g1 * vec2(1.0, 0.0));
}

Sphere sphere__round_weight(Sphere self) {
    return Sphere(vec3(0.0), self.g1 * vec2(1.0, 0.0));
}

Scalar anti_scalar__anti_dual(AntiScalar self) {
    return Scalar(self.g0);
}

Dipole circle__anti_dual(Circle self) {
    return Dipole(vec3(-self.g0.x, self.g0.y, self.g0.z), self.g1 * vec3(-1.0), vec4(-self.g2.x, -self.g2.y, -self.g2.z, self.g0.w));
}

Circle dipole__anti_dual(Dipole self) {
    return Circle(vec4(self.g0.x, self.g0.y, self.g0.z, -self.g2.w), self.g1, vec3(self.g2.x, self.g2.y, self.g2.z));
}

DualNum dual_num__anti_dual(DualNum self) {
    return DualNum(self.g0.yx * vec2(1.0, -1.0));
}

Circle flat_point__anti_dual(FlatPoint self) {
    return Circle(vec4(0.0, 0.0, 0.0, -self.g0.w), vec3(0.0), vec3(self.g0.x, self.g0.y, self.g0.z));
}

MultiVector flector__anti_dual(Flector self) {
    return MultiVector(vec2(0.0), vec3(self.g1.x, self.g1.y, self.g1.z), vec2(0.0, -self.g1.w), vec3(0.0), vec3(0.0), vec4(0.0), vec4(0.0, 0.0, 0.0, -self.g0.w), vec3(0.0), vec3(self.g0.x, self.g0.y, self.g0.z), vec3(0.0), vec2(0.0));
}

Dipole line__anti_dual(Line self) {
    return Dipole(vec3(0.0), self.g0 * vec3(-1.0), vec4(-self.g1.x, -self.g1.y, -self.g1.z, 0.0));
}

MultiVector motor__anti_dual(Motor self) {
    return MultiVector(vec2(self.g0.w, 0.0), vec3(0.0), vec2(0.0), vec3(0.0), vec3(-self.g0.x, self.g0.y, self.g0.z), vec4(-self.g1.x, -self.g1.y, -self.g1.z, 0.0), vec4(0.0), vec3(0.0), vec3(0.0), vec3(0.0), vec2(0.0));
}

MultiVector multi_vector__anti_dual(MultiVector self) {
    return MultiVector(self.g0.yx * vec2(1.0, -1.0), self.g9, self.g10 * vec2(-1.0), vec3(-self.g6.x, self.g6.y, self.g6.z), self.g7 * vec3(-1.0), vec4(-self.g8.x, -self.g8.y, -self.g8.z, self.g6.w), vec4(self.g3.x, self.g3.y, self.g3.z, -self.g5.w), self.g4, vec3(self.g5.x, self.g5.y, self.g5.z), self.g1 * vec3(-1.0), self.g2);
}

RoundPoint plane__anti_dual(Plane self) {
    return RoundPoint(vec3(self.g0.x, self.g0.y, self.g0.z), vec2(0.0, -self.g0.w));
}

Sphere round_point__anti_dual(RoundPoint self) {
    return Sphere(self.g0 * vec3(-1.0), self.g1);
}

AntiScalar scalar__anti_dual(Scalar self) {
    return AntiScalar(-self.g0);
}

RoundPoint sphere__anti_dual(Sphere self) {
    return RoundPoint(self.g0, self.g1 * vec2(-1.0));
}

AntiScalar anti_scalar__anti_reversal(AntiScalar self) {
    return self;
}

Circle circle__anti_reversal(Circle self) {
    return Circle(self.g0 * vec4(-1.0), self.g1 * vec3(-1.0), self.g2 * vec3(-1.0));
}

Dipole dipole__anti_reversal(Dipole self) {
    return Dipole(self.g0 * vec3(-1.0), self.g1 * vec3(-1.0), self.g2 * vec4(-1.0));
}

DualNum dual_num__anti_reversal(DualNum self) {
    return self;
}

FlatPoint flat_point__anti_reversal(FlatPoint self) {
    return FlatPoint(self.g0 * vec4(-1.0));
}

Flector flector__anti_reversal(Flector self) {
    return Flector(self.g0 * vec4(-1.0), self.g1);
}

Line line__anti_reversal(Line self) {
    return Line(self.g0 * vec3(-1.0), self.g1 * vec3(-1.0));
}

Motor motor__anti_reversal(Motor self) {
    return Motor(self.g0 * vec4(-1.0, -1.0, -1.0, 1.0), self.g1 * vec3(-1.0));
}

MultiVector multi_vector__anti_reversal(MultiVector self) {
    return MultiVector(self.g0, self.g1, self.g2, self.g3 * vec3(-1.0), self.g4 * vec3(-1.0), self.g5 * vec4(-1.0), self.g6 * vec4(-1.0), self.g7 * vec3(-1.0), self.g8 * vec3(-1.0), self.g9, self.g10);
}

Plane plane__anti_reversal(Plane self) {
    return self;
}

RoundPoint round_point__anti_reversal(RoundPoint self) {
    return self;
}

Scalar scalar__anti_reversal(Scalar self) {
    return self;
}

Sphere sphere__anti_reversal(Sphere self) {
    return self;
}

AntiScalar anti_scalar__automorphism(AntiScalar self) {
    return AntiScalar(-self.g0);
}

Circle circle__automorphism(Circle self) {
    return Circle(self.g0 * vec4(-1.0), self.g1 * vec3(-1.0), self.g2 * vec3(-1.0));
}

Dipole dipole__automorphism(Dipole self) {
    return self;
}

DualNum dual_num__automorphism(DualNum self) {
    return DualNum(self.g0 * vec2(1.0, -1.0));
}

FlatPoint flat_point__automorphism(FlatPoint self) {
    return self;
}

Flector flector__automorphism(Flector self) {
    return self;
}

Line line__automorphism(Line self) {
    return Line(self.g0 * vec3(-1.0), self.g1 * vec3(-1.0));
}

Motor motor__automorphism(Motor self) {
    return Motor(self.g0 * vec4(-1.0), self.g1 * vec3(-1.0));
}

MultiVector multi_vector__automorphism(MultiVector self) {
    return MultiVector(self.g0 * vec2(1.0, -1.0), self.g1 * vec3(-1.0), self.g2 * vec2(-1.0), self.g3, self.g4, self.g5, self.g6 * vec4(-1.0), self.g7 * vec3(-1.0), self.g8 * vec3(-1.0), self.g9, self.g10);
}

Plane plane__automorphism(Plane self) {
    return self;
}

RoundPoint round_point__automorphism(RoundPoint self) {
    return RoundPoint(self.g0 * vec3(-1.0), self.g1 * vec2(-1.0));
}

Scalar scalar__automorphism(Scalar self) {
    return self;
}

Sphere sphere__automorphism(Sphere self) {
    return self;
}

Scalar anti_scalar__complement(AntiScalar self) {
    return Scalar(self.g0);
}

Dipole circle__complement(Circle self) {
    return Dipole(self.g2 * vec3(-1.0), self.g1 * vec3(-1.0), self.g0 * vec4(-1.0));
}

Circle dipole__complement(Dipole self) {
    return Circle(self.g2 * vec4(-1.0), self.g1 * vec3(-1.0), self.g0 * vec3(-1.0));
}

DualNum dual_num__complement(DualNum self) {
    return DualNum(self.g0.yx);
}

Circle flat_point__complement(FlatPoint self) {
    return Circle(self.g0 * vec4(-1.0), vec3(0.0), vec3(0.0));
}

MultiVector flector__complement(Flector self) {
    return MultiVector(vec2(0.0), vec3(self.g1.x, self.g1.y, self.g1.z), vec2(self.g1.w, 0.0), vec3(0.0), vec3(0.0), vec4(0.0), self.g0 * vec4(-1.0), vec3(0.0), vec3(0.0), vec3(0.0), vec2(0.0));
}

Dipole line__complement(Line self) {
    return Dipole(self.g1 * vec3(-1.0), self.g0 * vec3(-1.0), vec4(0.0));
}

MultiVector motor__complement(Motor self) {
    return MultiVector(vec2(self.g0.w, 0.0), vec3(0.0), vec2(0.0), self.g1 * vec3(-1.0), vec3(-self.g0.x, self.g0.y, self.g0.z), vec4(0.0), vec4(0.0), vec3(0.0), vec3(0.0), vec3(0.0), vec2(0.0));
}

MultiVector multi_vector__complement(MultiVector self) {
    return MultiVector(self.g0.yx, self.g9, self.g10.yx, self.g8 * vec3(-1.0), self.g7 * vec3(-1.0), self.g6 * vec4(-1.0), self.g5 * vec4(-1.0), self.g4 * vec3(-1.0), self.g3 * vec3(-1.0), self.g1, self.g2.yx);
}

RoundPoint plane__complement(Plane self) {
    return RoundPoint(vec3(self.g0.x, self.g0.y, self.g0.z), vec2(self.g0.w, 0.0));
}

Sphere round_point__complement(RoundPoint self) {
    return Sphere(self.g0, self.g1.yx);
}

AntiScalar scalar__complement(Scalar self) {
    return AntiScalar(self.g0);
}

RoundPoint sphere__complement(Sphere self) {
    return RoundPoint(self.g0, self.g1.yx);
}

AntiScalar anti_scalar__conformal_conjugate(AntiScalar self) {
    return AntiScalar(-self.g0);
}

Circle circle__conformal_conjugate(Circle self) {
    return Circle(self.g0, self.g1 * vec3(-1.0), self.g2 * vec3(-1.0));
}

Dipole dipole__conformal_conjugate(Dipole self) {
    return Dipole(self.g0, self.g1, self.g2 * vec4(-1.0));
}

DualNum dual_num__conformal_conjugate(DualNum self) {
    return DualNum(self.g0 * vec2(1.0, -1.0));
}

FlatPoint flat_point__conformal_conjugate(FlatPoint self) {
    return FlatPoint(self.g0 * vec4(-1.0));
}

Flector flector__conformal_conjugate(Flector self) {
    return Flector(self.g0 * vec4(-1.0), self.g1 * vec4(-1.0));
}

Line line__conformal_conjugate(Line self) {
    return Line(self.g0 * vec3(-1.0), self.g1 * vec3(-1.0));
}

Motor motor__conformal_conjugate(Motor self) {
    return Motor(self.g0 * vec4(-1.0), self.g1 * vec3(-1.0));
}

MultiVector multi_vector__conformal_conjugate(MultiVector self) {
    return MultiVector(self.g0 * vec2(1.0, -1.0), self.g1, self.g2 * vec2(1.0, -1.0), self.g3, self.g4, self.g5 * vec4(-1.0), self.g6, self.g7 * vec3(-1.0), self.g8 * vec3(-1.0), self.g9 * vec3(-1.0), self.g10 * vec2(1.0, -1.0));
}

Plane plane__conformal_conjugate(Plane self) {
    return Plane(self.g0 * vec4(-1.0));
}

RoundPoint round_point__conformal_conjugate(RoundPoint self) {
    return RoundPoint(self.g0, self.g1 * vec2(1.0, -1.0));
}

Scalar scalar__conformal_conjugate(Scalar self) {
    return self;
}

Sphere sphere__conformal_conjugate(Sphere self) {
    return Sphere(self.g0 * vec3(-1.0), self.g1 * vec2(1.0, -1.0));
}

AntiScalar anti_scalar__conjugation(AntiScalar self) {
    return AntiScalar(-self.g0);
}

Circle circle__conjugation(Circle self) {
    return self;
}

Dipole dipole__conjugation(Dipole self) {
    return Dipole(self.g0 * vec3(-1.0), self.g1 * vec3(-1.0), self.g2 * vec4(-1.0));
}

DualNum dual_num__conjugation(DualNum self) {
    return DualNum(self.g0 * vec2(1.0, -1.0));
}

FlatPoint flat_point__conjugation(FlatPoint self) {
    return FlatPoint(self.g0 * vec4(-1.0));
}

Flector flector__conjugation(Flector self) {
    return Flector(self.g0 * vec4(-1.0), self.g1);
}

Line line__conjugation(Line self) {
    return self;
}

Motor motor__conjugation(Motor self) {
    return Motor(self.g0 * vec4(1.0, 1.0, 1.0, -1.0), self.g1);
}

MultiVector multi_vector__conjugation(MultiVector self) {
    return MultiVector(self.g0 * vec2(1.0, -1.0), self.g1 * vec3(-1.0), self.g2 * vec2(-1.0), self.g3 * vec3(-1.0), self.g4 * vec3(-1.0), self.g5 * vec4(-1.0), self.g6, self.g7, self.g8, self.g9, self.g10);
}

Plane plane__conjugation(Plane self) {
    return self;
}

RoundPoint round_point__conjugation(RoundPoint self) {
    return RoundPoint(self.g0 * vec3(-1.0), self.g1 * vec2(-1.0));
}

Scalar scalar__conjugation(Scalar self) {
    return self;
}

Sphere sphere__conjugation(Sphere self) {
    return self;
}

AntiScalar anti_scalar__double_complement(AntiScalar self) {
    return self;
}

Circle circle__double_complement(Circle self) {
    return self;
}

Dipole dipole__double_complement(Dipole self) {
    return self;
}

DualNum dual_num__double_complement(DualNum self) {
    return self;
}

FlatPoint flat_point__double_complement(FlatPoint self) {
    return self;
}

Flector flector__double_complement(Flector self) {
    return self;
}

Line line__double_complement(Line self) {
    return self;
}

Motor motor__double_complement(Motor self) {
    return self;
}

MultiVector multi_vector__double_complement(MultiVector self) {
    return self;
}

Plane plane__double_complement(Plane self) {
    return self;
}

RoundPoint round_point__double_complement(RoundPoint self) {
    return self;
}

Scalar scalar__double_complement(Scalar self) {
    return self;
}

Sphere sphere__double_complement(Sphere self) {
    return self;
}

Scalar anti_scalar__dual(AntiScalar self) {
    return Scalar(-self.g0);
}

Dipole circle__dual(Circle self) {
    return Dipole(vec3(self.g0.x, self.g0.y, self.g0.z), self.g1, vec4(self.g2.x, self.g2.y, self.g2.z, -self.g0.w));
}

Circle dipole__dual(Dipole self) {
    return Circle(vec4(-self.g0.x, -self.g0.y, -self.g0.z, self.g2.w), self.g1 * vec3(-1.0), vec3(-self.g2.x, self.g2.y, self.g2.z));
}

DualNum dual_num__dual(DualNum self) {
    return DualNum(self.g0.yx * vec2(-1.0, 1.0));
}

Circle flat_point__dual(FlatPoint self) {
    return Circle(vec4(0.0, 0.0, 0.0, self.g0.w), vec3(0.0), vec3(-self.g0.x, self.g0.y, self.g0.z));
}

MultiVector flector__dual(Flector self) {
    return MultiVector(vec2(0.0), vec3(-self.g1.x, self.g1.y, self.g1.z), vec2(0.0, self.g1.w), vec3(0.0), vec3(0.0), vec4(0.0), vec4(0.0, 0.0, 0.0, self.g0.w), vec3(0.0), vec3(-self.g0.x, self.g0.y, self.g0.z), vec3(0.0), vec2(0.0));
}

Dipole line__dual(Line self) {
    return Dipole(vec3(0.0), self.g0, vec4(self.g1.x, self.g1.y, self.g1.z, 0.0));
}

MultiVector motor__dual(Motor self) {
    return MultiVector(vec2(-self.g0.w, 0.0), vec3(0.0), vec2(0.0), vec3(0.0), vec3(self.g0.x, self.g0.y, self.g0.z), vec4(self.g1.x, self.g1.y, self.g1.z, 0.0), vec4(0.0), vec3(0.0), vec3(0.0), vec3(0.0), vec2(0.0));
}

MultiVector multi_vector__dual(MultiVector self) {
    return MultiVector(self.g0.yx * vec2(-1.0, 1.0), self.g9 * vec3(-1.0), self.g10, vec3(self.g6.x, self.g6.y, self.g6.z), self.g7, vec4(self.g8.x, self.g8.y, self.g8.z, -self.g6.w), vec4(-self.g3.x, -self.g3.y, -self.g3.z, self.g5.w), self.g4 * vec3(-1.0), vec3(-self.g5.x, self.g5.y, self.g5.z), self.g1, self.g2 * vec2(-1.0));
}

RoundPoint plane__dual(Plane self) {
    return RoundPoint(vec3(-self.g0.x, self.g0.y, self.g0.z), vec2(0.0, self.g0.w));
}

Sphere round_point__dual(RoundPoint self) {
    return Sphere(self.g0, self.g1 * vec2(-1.0));
}

AntiScalar scalar__dual(Scalar self) {
    return AntiScalar(self.g0);
}

RoundPoint sphere__dual(Sphere self) {
    return RoundPoint(self.g0 * vec3(-1.0), self.g1);
}

AntiScalar anti_scalar__reversal(AntiScalar self) {
    return self;
}

Circle circle__reversal(Circle self) {
    return Circle(self.g0 * vec4(-1.0), self.g1 * vec3(-1.0), self.g2 * vec3(-1.0));
}

Dipole dipole__reversal(Dipole self) {
    return Dipole(self.g0 * vec3(-1.0), self.g1 * vec3(-1.0), self.g2 * vec4(-1.0));
}

DualNum dual_num__reversal(DualNum self) {
    return self;
}

FlatPoint flat_point__reversal(FlatPoint self) {
    return FlatPoint(self.g0 * vec4(-1.0));
}

Flector flector__reversal(Flector self) {
    return Flector(self.g0 * vec4(-1.0), self.g1);
}

Line line__reversal(Line self) {
    return Line(self.g0 * vec3(-1.0), self.g1 * vec3(-1.0));
}

Motor motor__reversal(Motor self) {
    return Motor(self.g0 * vec4(-1.0, -1.0, -1.0, 1.0), self.g1 * vec3(-1.0));
}

MultiVector multi_vector__reversal(MultiVector self) {
    return MultiVector(self.g0, self.g1, self.g2, self.g3 * vec3(-1.0), self.g4 * vec3(-1.0), self.g5 * vec4(-1.0), self.g6 * vec4(-1.0), self.g7 * vec3(-1.0), self.g8 * vec3(-1.0), self.g9, self.g10);
}

Plane plane__reversal(Plane self) {
    return self;
}

RoundPoint round_point__reversal(RoundPoint self) {
    return self;
}

Scalar scalar__reversal(Scalar self) {
    return self;
}

Sphere sphere__reversal(Sphere self) {
    return self;
}

Dipole circle__flat_bulk_dual(Circle self) {
    return circle__complement(circle__flat_bulk(self));
}

Circle dipole__flat_bulk_dual(Dipole self) {
    return dipole__complement(dipole__flat_bulk(self));
}

Circle flat_point__flat_bulk_dual(FlatPoint self) {
    return flat_point__complement(flat_point__flat_bulk(self));
}

MultiVector flector__flat_bulk_dual(Flector self) {
    return flector__complement(flector__flat_bulk(self));
}

Dipole line__flat_bulk_dual(Line self) {
    return line__complement(line__flat_bulk(self));
}

MultiVector motor__flat_bulk_dual(Motor self) {
    return motor__complement(motor__flat_bulk(self));
}

MultiVector multi_vector__flat_bulk_dual(MultiVector self) {
    return multi_vector__complement(multi_vector__flat_bulk(self));
}

RoundPoint plane__flat_bulk_dual(Plane self) {
    return plane__complement(plane__flat_bulk(self));
}

Sphere round_point__flat_bulk_dual(RoundPoint self) {
    return round_point__complement(round_point__flat_bulk(self));
}

RoundPoint sphere__flat_bulk_dual(Sphere self) {
    return sphere__complement(sphere__flat_bulk(self));
}

Scalar anti_scalar__flat_weight_dual(AntiScalar self) {
    return anti_scalar__complement(anti_scalar__flat_weight(self));
}

Dipole circle__flat_weight_dual(Circle self) {
    return circle__complement(circle__flat_weight(self));
}

Circle dipole__flat_weight_dual(Dipole self) {
    return dipole__complement(dipole__flat_weight(self));
}

DualNum dual_num__flat_weight_dual(DualNum self) {
    return dual_num__complement(dual_num__flat_weight(self));
}

Circle flat_point__flat_weight_dual(FlatPoint self) {
    return flat_point__complement(flat_point__flat_weight(self));
}

MultiVector flector__flat_weight_dual(Flector self) {
    return flector__complement(flector__flat_weight(self));
}

Dipole line__flat_weight_dual(Line self) {
    return line__complement(line__flat_weight(self));
}

MultiVector motor__flat_weight_dual(Motor self) {
    return motor__complement(motor__flat_weight(self));
}

MultiVector multi_vector__flat_weight_dual(MultiVector self) {
    return multi_vector__complement(multi_vector__flat_weight(self));
}

RoundPoint plane__flat_weight_dual(Plane self) {
    return plane__complement(plane__flat_weight(self));
}

RoundPoint sphere__flat_weight_dual(Sphere self) {
    return sphere__complement(sphere__flat_weight(self));
}

Dipole circle__round_bulk_dual(Circle self) {
    return circle__complement(circle__round_bulk(self));
}

Circle dipole__round_bulk_dual(Dipole self) {
    return dipole__complement(dipole__round_bulk(self));
}

DualNum dual_num__round_bulk_dual(DualNum self) {
    return dual_num__complement(dual_num__round_bulk(self));
}

MultiVector multi_vector__round_bulk_dual(MultiVector self) {
    return multi_vector__complement(multi_vector__round_bulk(self));
}

Sphere round_point__round_bulk_dual(RoundPoint self) {
    return round_point__complement(round_point__round_bulk(self));
}

AntiScalar scalar__round_bulk_dual(Scalar self) {
    return scalar__complement(scalar__round_bulk(self));
}

Dipole circle__round_weight_dual(Circle self) {
    return circle__complement(circle__round_weight(self));
}

Circle dipole__round_weight_dual(Dipole self) {
    return dipole__complement(dipole__round_weight(self));
}

MultiVector multi_vector__round_weight_dual(MultiVector self) {
    return multi_vector__complement(multi_vector__round_weight(self));
}

Sphere round_point__round_weight_dual(RoundPoint self) {
    return round_point__complement(round_point__round_weight(self));
}

RoundPoint sphere__round_weight_dual(Sphere self) {
    return sphere__complement(sphere__round_weight(self));
}

int anti_scalar__anti_grade() {
    return 0;
}

int circle__anti_grade() {
    return 2;
}

int dipole__anti_grade() {
    return 3;
}

int flat_point__anti_grade() {
    return 3;
}

int line__anti_grade() {
    return 2;
}

int plane__anti_grade() {
    return 1;
}

int round_point__anti_grade() {
    return 4;
}

int scalar__anti_grade() {
    return 5;
}

int sphere__anti_grade() {
    return 1;
}

int anti_scalar__grade() {
    return 5;
}

int circle__grade() {
    return 3;
}

int dipole__grade() {
    return 2;
}

int flat_point__grade() {
    return 2;
}

int line__grade() {
    return 3;
}

int plane__grade() {
    return 4;
}

int round_point__grade() {
    return 1;
}

int scalar__grade() {
    return 0;
}

int sphere__grade() {
    return 4;
}

AntiScalar anti_scalar__anti_sqrt(AntiScalar self) {
    return AntiScalar(sqrt(self.g0));
}

DualNum dual_num__anti_sqrt(DualNum self) {
    float s = self.g0.x;
    float t = self.g0.y;
    float sqrt_t = sqrt(t);
    return DualNum(vec2(s / (2.0 * sqrt_t), sqrt_t));
}

Plane anti_scalar__attitude(AntiScalar self) {
    return anti_scalar__anti_wedge__plane(self, Plane(vec4(0.0, 0.0, 0.0, 1.0)));
}

Dipole circle__attitude(Circle self) {
    return circle__anti_wedge__plane(self, Plane(vec4(0.0, 0.0, 0.0, 1.0)));
}

RoundPoint dipole__attitude(Dipole self) {
    return dipole__anti_wedge__plane(self, Plane(vec4(0.0, 0.0, 0.0, 1.0)));
}

Plane dual_num__attitude(DualNum self) {
    return dual_num__anti_wedge__plane(self, Plane(vec4(0.0, 0.0, 0.0, 1.0)));
}

RoundPoint flat_point__attitude(FlatPoint self) {
    return flat_point__anti_wedge__plane(self, Plane(vec4(0.0, 0.0, 0.0, 1.0)));
}

MultiVector flector__attitude(Flector self) {
    return flector__anti_wedge__plane(self, Plane(vec4(0.0, 0.0, 0.0, 1.0)));
}

FlatPoint line__attitude(Line self) {
    return line__anti_wedge__plane(self, Plane(vec4(0.0, 0.0, 0.0, 1.0)));
}

Flector motor__attitude(Motor self) {
    return motor__anti_wedge__plane(self, Plane(vec4(0.0, 0.0, 0.0, 1.0)));
}

MultiVector multi_vector__attitude(MultiVector self) {
    return multi_vector__anti_wedge__plane(self, Plane(vec4(0.0, 0.0, 0.0, 1.0)));
}

Line plane__attitude(Plane self) {
    return plane__anti_wedge__plane(self, Plane(vec4(0.0, 0.0, 0.0, 1.0)));
}

Scalar round_point__attitude(RoundPoint self) {
    return round_point__anti_wedge__plane(self, Plane(vec4(0.0, 0.0, 0.0, 1.0)));
}

Circle sphere__attitude(Sphere self) {
    return sphere__anti_wedge__plane(self, Plane(vec4(0.0, 0.0, 0.0, 1.0)));
}

Sphere circle__carrier(Circle self) {
    return circle__wedge__round_point(self, RoundPoint(vec3(0.0), vec2(0.0, 1.0)));
}

Circle dipole__carrier(Dipole self) {
    return dipole__wedge__round_point(self, RoundPoint(vec3(0.0), vec2(0.0, 1.0)));
}

RoundPoint dual_num__carrier(DualNum self) {
    return dual_num__wedge__round_point(self, RoundPoint(vec3(0.0), vec2(0.0, 1.0)));
}

MultiVector multi_vector__carrier(MultiVector self) {
    return multi_vector__wedge__round_point(self, RoundPoint(vec3(0.0), vec2(0.0, 1.0)));
}

Dipole round_point__carrier(RoundPoint self) {
    return round_point__wedge__round_point(self, RoundPoint(vec3(0.0), vec2(0.0, 1.0)));
}

RoundPoint scalar__carrier(Scalar self) {
    return scalar__wedge__round_point(self, RoundPoint(vec3(0.0), vec2(0.0, 1.0)));
}

AntiScalar sphere__carrier(Sphere self) {
    return sphere__wedge__round_point(self, RoundPoint(vec3(0.0), vec2(0.0, 1.0)));
}

Circle circle__co_carrier(Circle self) {
    return dipole__wedge__round_point(circle__anti_dual(self), RoundPoint(vec3(0.0), vec2(0.0, 1.0)));
}

Sphere dipole__co_carrier(Dipole self) {
    return circle__wedge__round_point(dipole__anti_dual(self), RoundPoint(vec3(0.0), vec2(0.0, 1.0)));
}

RoundPoint dual_num__co_carrier(DualNum self) {
    return dual_num__wedge__round_point(dual_num__anti_dual(self), RoundPoint(vec3(0.0), vec2(0.0, 1.0)));
}

MultiVector multi_vector__co_carrier(MultiVector self) {
    return multi_vector__wedge__round_point(multi_vector__anti_dual(self), RoundPoint(vec3(0.0), vec2(0.0, 1.0)));
}

AntiScalar round_point__co_carrier(RoundPoint self) {
    return sphere__wedge__round_point(round_point__anti_dual(self), RoundPoint(vec3(0.0), vec2(0.0, 1.0)));
}

Dipole sphere__co_carrier(Sphere self) {
    return round_point__wedge__round_point(sphere__anti_dual(self), RoundPoint(vec3(0.0), vec2(0.0, 1.0)));
}

DualNum dual_num__sqrt(DualNum self) {
    float s = self.g0.x;
    float t = self.g0.y;
    float sqrt_s = sqrt(s);
    return DualNum(vec2(sqrt_s, t / (2.0 * sqrt_s)));
}

Scalar scalar__sqrt(Scalar self) {
    return Scalar(sqrt(self.g0));
}

RoundPoint circle__center(Circle self) {
    return circle__anti_wedge__circle(circle__co_carrier(self), self);
}

RoundPoint dipole__center(Dipole self) {
    return sphere__anti_wedge__dipole(dipole__co_carrier(self), self);
}

RoundPoint dual_num__center(DualNum self) {
    return round_point__anti_wedge__dual_num(dual_num__co_carrier(self), self);
}

MultiVector multi_vector__center(MultiVector self) {
    return multi_vector__anti_wedge__multi_vector(multi_vector__co_carrier(self), self);
}

RoundPoint round_point__center(RoundPoint self) {
    return anti_scalar__anti_wedge__round_point(round_point__co_carrier(self), self);
}

RoundPoint sphere__center(Sphere self) {
    return dipole__anti_wedge__sphere(sphere__co_carrier(self), self);
}

Sphere circle__container(Circle self) {
    return circle__wedge__round_point(self, sphere__anti_dual(circle__carrier(self)));
}

Sphere dipole__container(Dipole self) {
    return dipole__wedge__dipole(self, circle__anti_dual(dipole__carrier(self)));
}

Sphere dual_num__container(DualNum self) {
    return dual_num__wedge__sphere(self, round_point__anti_dual(dual_num__carrier(self)));
}

MultiVector multi_vector__container(MultiVector self) {
    return multi_vector__wedge__multi_vector(self, multi_vector__anti_dual(multi_vector__carrier(self)));
}

Sphere round_point__container(RoundPoint self) {
    return round_point__wedge__circle(self, dipole__anti_dual(round_point__carrier(self)));
}

Sphere scalar__container(Scalar self) {
    return scalar__wedge__sphere(self, round_point__anti_dual(scalar__carrier(self)));
}

Sphere sphere__container(Sphere self) {
    return sphere__wedge__scalar(self, anti_scalar__anti_dual(sphere__carrier(self)));
}

Circle circle__partner(Circle self) {
    return sphere__anti_wedge__sphere(sphere__neg(dipole__container(circle__dual(self))), circle__carrier(self));
}

Dipole dipole__partner(Dipole self) {
    return sphere__anti_wedge__circle(sphere__neg(circle__container(dipole__dual(self))), dipole__carrier(self));
}

Scalar dual_num__partner(DualNum self) {
    return sphere__anti_wedge__round_point(sphere__neg(dual_num__container(dual_num__dual(self))), dual_num__carrier(self));
}

MultiVector multi_vector__partner(MultiVector self) {
    return multi_vector__anti_wedge__multi_vector(multi_vector__neg(multi_vector__container(multi_vector__dual(self))), multi_vector__carrier(self));
}

RoundPoint round_point__partner(RoundPoint self) {
    return sphere__anti_wedge__dipole(sphere__neg(sphere__container(round_point__dual(self))), round_point__carrier(self));
}

Sphere sphere__partner(Sphere self) {
    return sphere__anti_wedge__anti_scalar(sphere__neg(round_point__container(sphere__dual(self))), sphere__carrier(self));
}

AntiScalar anti_scalar__anti_inverse(AntiScalar self) {
    return anti_scalar__anti_wedge_dot__anti_scalar(self, anti_scalar__div__anti_scalar(anti_scalar__unit(), anti_scalar__anti_dot__anti_scalar(self, self)));
}

Circle circle__anti_inverse(Circle self) {
    return circle__anti_wedge_dot__anti_scalar(self, anti_scalar__div__anti_scalar(anti_scalar__unit(), circle__anti_dot__circle(self, self)));
}

Dipole dipole__anti_inverse(Dipole self) {
    return dipole__anti_wedge_dot__anti_scalar(self, anti_scalar__div__anti_scalar(anti_scalar__unit(), dipole__anti_dot__dipole(self, self)));
}

DualNum dual_num__anti_inverse(DualNum self) {
    float s = self.g0.x;
    float t = self.g0.y;
    return DualNum(vec2(-1.0 * s / (t * t), 1.0 / t));
}

FlatPoint flat_point__anti_inverse(FlatPoint self) {
    return flat_point__anti_wedge_dot__anti_scalar(self, anti_scalar__div__anti_scalar(anti_scalar__unit(), flat_point__anti_dot__flat_point(self, self)));
}

Flector flector__anti_inverse(Flector self) {
    return flector__anti_wedge_dot__anti_scalar(self, anti_scalar__div__anti_scalar(anti_scalar__unit(), flector__anti_dot__flector(self, self)));
}

Line line__anti_inverse(Line self) {
    return line__anti_wedge_dot__anti_scalar(self, anti_scalar__div__anti_scalar(anti_scalar__unit(), line__anti_dot__line(self, self)));
}

Motor motor__anti_inverse(Motor self) {
    return motor__anti_wedge_dot__anti_scalar(self, anti_scalar__div__anti_scalar(anti_scalar__unit(), motor__anti_dot__motor(self, self)));
}

MultiVector multi_vector__anti_inverse(MultiVector self) {
    return multi_vector__anti_wedge_dot__anti_scalar(self, anti_scalar__div__anti_scalar(anti_scalar__unit(), multi_vector__anti_dot__multi_vector(self, self)));
}

Plane plane__anti_inverse(Plane self) {
    return plane__anti_wedge_dot__anti_scalar(self, anti_scalar__div__anti_scalar(anti_scalar__unit(), plane__anti_dot__plane(self, self)));
}

RoundPoint round_point__anti_inverse(RoundPoint self) {
    return round_point__anti_wedge_dot__anti_scalar(self, anti_scalar__div__anti_scalar(anti_scalar__unit(), round_point__anti_dot__round_point(self, self)));
}

Scalar scalar__anti_inverse(Scalar self) {
    return scalar__anti_wedge_dot__anti_scalar(self, anti_scalar__div__anti_scalar(anti_scalar__unit(), scalar__anti_dot__scalar(self, self)));
}

Sphere sphere__anti_inverse(Sphere self) {
    return sphere__anti_wedge_dot__anti_scalar(self, anti_scalar__div__anti_scalar(anti_scalar__unit(), sphere__anti_dot__sphere(self, self)));
}

AntiScalar anti_scalar__inverse(AntiScalar self) {
    return anti_scalar__wedge_dot__scalar(self, scalar__div__scalar(scalar__unit(), anti_scalar__dot__anti_scalar(self, self)));
}

Circle circle__inverse(Circle self) {
    return circle__wedge_dot__scalar(self, scalar__div__scalar(scalar__unit(), circle__dot__circle(self, self)));
}

Dipole dipole__inverse(Dipole self) {
    return dipole__wedge_dot__scalar(self, scalar__div__scalar(scalar__unit(), dipole__dot__dipole(self, self)));
}

DualNum dual_num__inverse(DualNum self) {
    float s = self.g0.x;
    float t = self.g0.y;
    return DualNum(vec2(1.0 / s, -1.0 * t / (s * s)));
}

FlatPoint flat_point__inverse(FlatPoint self) {
    return flat_point__wedge_dot__scalar(self, scalar__div__scalar(scalar__unit(), flat_point__dot__flat_point(self, self)));
}

Flector flector__inverse(Flector self) {
    return flector__wedge_dot__scalar(self, scalar__div__scalar(scalar__unit(), flector__dot__flector(self, self)));
}

Line line__inverse(Line self) {
    return line__wedge_dot__scalar(self, scalar__div__scalar(scalar__unit(), line__dot__line(self, self)));
}

Motor motor__inverse(Motor self) {
    return motor__wedge_dot__scalar(self, scalar__div__scalar(scalar__unit(), motor__dot__motor(self, self)));
}

MultiVector multi_vector__inverse(MultiVector self) {
    return multi_vector__wedge_dot__scalar(self, scalar__div__scalar(scalar__unit(), multi_vector__dot__multi_vector(self, self)));
}

Plane plane__inverse(Plane self) {
    return plane__wedge_dot__scalar(self, scalar__div__scalar(scalar__unit(), plane__dot__plane(self, self)));
}

RoundPoint round_point__inverse(RoundPoint self) {
    return round_point__wedge_dot__scalar(self, scalar__div__scalar(scalar__unit(), round_point__dot__round_point(self, self)));
}

Scalar scalar__inverse(Scalar self) {
    return scalar__wedge_dot__scalar(self, scalar__div__scalar(scalar__unit(), scalar__dot__scalar(self, self)));
}

Sphere sphere__inverse(Sphere self) {
    return sphere__wedge_dot__scalar(self, scalar__div__scalar(scalar__unit(), sphere__dot__sphere(self, self)));
}

DualNum dual_num__anti_cos(DualNum self) {
    float s = self.g0.x;
    float t = self.g0.y;
    return DualNum(vec2(-1.0 * s * sin(t), cos(t)));
}

AntiScalar anti_scalar__anti_cosh(AntiScalar self) {
    return AntiScalar(cosh(self.g0));
}

DualNum dual_num__anti_cosh(DualNum self) {
    float s = self.g0.x;
    float t = self.g0.y;
    return DualNum(vec2(s * sinh(t), cosh(t)));
}

AntiScalar anti_scalar__anti_exp(AntiScalar self) {
    return AntiScalar(exp(self.g0));
}

DualNum dual_num__anti_exp(DualNum self) {
    float s = self.g0.x;
    float t = self.g0.y;
    float exp_t = exp(t);
    return DualNum(vec2(s * exp_t, exp_t));
}

AntiScalar anti_scalar__anti_inverse_sqrt(AntiScalar self) {
    return AntiScalar(1.0 / sqrt(self.g0));
}

DualNum dual_num__anti_inverse_sqrt(DualNum self) {
    float s = self.g0.x;
    float t = self.g0.y;
    float sqrt_t = sqrt(t);
    return DualNum(vec2(-1.0 * s / (2.0 * t * sqrt_t), 1.0 / sqrt_t));
}

AntiScalar anti_scalar__anti_pow(AntiScalar self, float other) {
    return AntiScalar(pow(self.g0, other));
}

DualNum dual_num__anti_pow(DualNum self, float other) {
    float s = self.g0.x;
    float t = self.g0.y;
    return DualNum(vec2(other * pow(t, other - 1.0) * s, pow(t, other)));
}

DualNum dual_num__anti_sin(DualNum self) {
    float s = self.g0.x;
    float t = self.g0.y;
    return DualNum(vec2(s * cos(t), sin(t)));
}

AntiScalar anti_scalar__anti_sinh(AntiScalar self) {
    return AntiScalar(sinh(self.g0));
}

DualNum dual_num__anti_sinh(DualNum self) {
    float s = self.g0.x;
    float t = self.g0.y;
    return DualNum(vec2(s * cosh(t), sinh(t)));
}

DualNum dual_num__anti_tan(DualNum self) {
    float s = self.g0.x;
    float t = self.g0.y;
    float tan_t = tan(t);
    return DualNum(vec2(s * (1.0 + tan_t * tan_t), tan_t));
}

AntiScalar anti_scalar__anti_tanh(AntiScalar self) {
    return AntiScalar(tanh(self.g0));
}

DualNum dual_num__anti_tanh(DualNum self) {
    float s = self.g0.x;
    float t = self.g0.y;
    float tanh_t = tanh(t);
    return DualNum(vec2(s * (1.0 - tanh_t * tanh_t), tanh_t));
}

DualNum dual_num__cos(DualNum self) {
    float s = self.g0.x;
    float t = self.g0.y;
    return DualNum(vec2(cos(s), -1.0 * t * sin(s)));
}

DualNum dual_num__cosh(DualNum self) {
    float s = self.g0.x;
    float t = self.g0.y;
    return DualNum(vec2(cosh(s), t * sinh(s)));
}

Scalar scalar__cosh(Scalar self) {
    return Scalar(cosh(self.g0));
}

DualNum dual_num__exp(DualNum self) {
    float s = self.g0.x;
    float t = self.g0.y;
    float exp_s = exp(s);
    return DualNum(vec2(exp_s, t * exp_s));
}

Scalar scalar__exp(Scalar self) {
    return Scalar(exp(self.g0));
}

DualNum dual_num__inverse_sqrt(DualNum self) {
    float s = self.g0.x;
    float t = self.g0.y;
    float sqrt_s = sqrt(s);
    return DualNum(vec2(1.0 / sqrt_s, -1.0 * t / (2.0 * s * sqrt_s)));
}

Scalar scalar__inverse_sqrt(Scalar self) {
    return Scalar(1.0 / sqrt(self.g0));
}

DualNum dual_num__pow(DualNum self, float other) {
    float s = self.g0.x;
    float t = self.g0.y;
    return DualNum(vec2(pow(s, other), other * pow(s, other - 1.0) * t));
}

Scalar scalar__pow(Scalar self, float other) {
    return Scalar(pow(self.g0, other));
}

DualNum dual_num__sin(DualNum self) {
    float s = self.g0.x;
    float t = self.g0.y;
    return DualNum(vec2(sin(s), t * cos(s)));
}

DualNum dual_num__sinh(DualNum self) {
    float s = self.g0.x;
    float t = self.g0.y;
    return DualNum(vec2(sinh(s), t * cosh(s)));
}

Scalar scalar__sinh(Scalar self) {
    return Scalar(sinh(self.g0));
}

DualNum dual_num__tan(DualNum self) {
    float s = self.g0.x;
    float t = self.g0.y;
    float tan_s = tan(s);
    return DualNum(vec2(tan_s, t * (1.0 + tan_s * tan_s)));
}

DualNum dual_num__tanh(DualNum self) {
    float s = self.g0.x;
    float t = self.g0.y;
    float tanh_s = tanh(s);
    return DualNum(vec2(tanh_s, t * (1.0 - tanh_s * tanh_s)));
}

Scalar scalar__tanh(Scalar self) {
    return Scalar(tanh(self.g0));
}

Scalar circle__round_bulk_norm_squared(Circle self) {
    Circle round_bulk_carrier = circle__round_bulk(self);
    return circle__dot__circle(round_bulk_carrier, round_bulk_carrier);
}

Scalar dipole__round_bulk_norm_squared(Dipole self) {
    Dipole round_bulk_carrier = dipole__round_bulk(self);
    return dipole__dot__dipole(round_bulk_carrier, round_bulk_carrier);
}

Scalar dual_num__round_bulk_norm_squared(DualNum self) {
    DualNum round_bulk_carrier = dual_num__round_bulk(self);
    return dual_num__dot__dual_num(round_bulk_carrier, round_bulk_carrier);
}

Scalar multi_vector__round_bulk_norm_squared(MultiVector self) {
    MultiVector round_bulk_carrier = multi_vector__round_bulk(self);
    return multi_vector__dot__multi_vector(round_bulk_carrier, round_bulk_carrier);
}

Scalar round_point__round_bulk_norm_squared(RoundPoint self) {
    RoundPoint round_bulk_carrier = round_point__round_bulk(self);
    return round_point__dot__round_point(round_bulk_carrier, round_bulk_carrier);
}

Scalar scalar__round_bulk_norm_squared(Scalar self) {
    Scalar round_bulk_carrier = scalar__round_bulk(self);
    return scalar__dot__scalar(round_bulk_carrier, round_bulk_carrier);
}

Scalar circle__round_bulk_norm(Circle self) {
    return scalar__sqrt(circle__round_bulk_norm_squared(self));
}

Scalar dipole__round_bulk_norm(Dipole self) {
    return scalar__sqrt(dipole__round_bulk_norm_squared(self));
}

Scalar dual_num__round_bulk_norm(DualNum self) {
    return scalar__sqrt(dual_num__round_bulk_norm_squared(self));
}

Scalar multi_vector__round_bulk_norm(MultiVector self) {
    return scalar__sqrt(multi_vector__round_bulk_norm_squared(self));
}

Scalar round_point__round_bulk_norm(RoundPoint self) {
    return scalar__sqrt(round_point__round_bulk_norm_squared(self));
}

Scalar scalar__round_bulk_norm(Scalar self) {
    return scalar__sqrt(scalar__round_bulk_norm_squared(self));
}

AntiScalar circle__round_weight_norm_squared(Circle self) {
    Sphere round_weight_carrier = circle__wedge__round_point(circle__round_weight(self), RoundPoint(vec3(0.0), vec2(0.0, 1.0)));
    return sphere__anti_dot__sphere(round_weight_carrier, round_weight_carrier);
}

AntiScalar dipole__round_weight_norm_squared(Dipole self) {
    Circle round_weight_carrier = dipole__wedge__round_point(dipole__round_weight(self), RoundPoint(vec3(0.0), vec2(0.0, 1.0)));
    return circle__anti_dot__circle(round_weight_carrier, round_weight_carrier);
}

AntiScalar multi_vector__round_weight_norm_squared(MultiVector self) {
    MultiVector round_weight_carrier = multi_vector__wedge__round_point(multi_vector__round_weight(self), RoundPoint(vec3(0.0), vec2(0.0, 1.0)));
    return multi_vector__anti_dot__multi_vector(round_weight_carrier, round_weight_carrier);
}

AntiScalar round_point__round_weight_norm_squared(RoundPoint self) {
    Dipole round_weight_carrier = round_point__wedge__round_point(round_point__round_weight(self), RoundPoint(vec3(0.0), vec2(0.0, 1.0)));
    return dipole__anti_dot__dipole(round_weight_carrier, round_weight_carrier);
}

AntiScalar sphere__round_weight_norm_squared(Sphere self) {
    AntiScalar round_weight_carrier = sphere__wedge__round_point(sphere__round_weight(self), RoundPoint(vec3(0.0), vec2(0.0, 1.0)));
    return anti_scalar__anti_dot__anti_scalar(round_weight_carrier, round_weight_carrier);
}

AntiScalar circle__round_weight_norm(Circle self) {
    return anti_scalar__anti_sqrt(circle__round_weight_norm_squared(self));
}

AntiScalar dipole__round_weight_norm(Dipole self) {
    return anti_scalar__anti_sqrt(dipole__round_weight_norm_squared(self));
}

AntiScalar multi_vector__round_weight_norm(MultiVector self) {
    return anti_scalar__anti_sqrt(multi_vector__round_weight_norm_squared(self));
}

AntiScalar round_point__round_weight_norm(RoundPoint self) {
    return anti_scalar__anti_sqrt(round_point__round_weight_norm_squared(self));
}

AntiScalar sphere__round_weight_norm(Sphere self) {
    return anti_scalar__anti_sqrt(sphere__round_weight_norm_squared(self));
}

DualNum circle__round_norm_squared(Circle self) {
    return scalar__add__anti_scalar(circle__round_bulk_norm_squared(self), circle__round_weight_norm_squared(self));
}

DualNum dipole__round_norm_squared(Dipole self) {
    return scalar__add__anti_scalar(dipole__round_bulk_norm_squared(self), dipole__round_weight_norm_squared(self));
}

DualNum multi_vector__round_norm_squared(MultiVector self) {
    return scalar__add__anti_scalar(multi_vector__round_bulk_norm_squared(self), multi_vector__round_weight_norm_squared(self));
}

DualNum round_point__round_norm_squared(RoundPoint self) {
    return scalar__add__anti_scalar(round_point__round_bulk_norm_squared(self), round_point__round_weight_norm_squared(self));
}

DualNum circle__round_norm(Circle self) {
    return scalar__add__anti_scalar(circle__round_bulk_norm(self), circle__round_weight_norm(self));
}

DualNum dipole__round_norm(Dipole self) {
    return scalar__add__anti_scalar(dipole__round_bulk_norm(self), dipole__round_weight_norm(self));
}

DualNum multi_vector__round_norm(MultiVector self) {
    return scalar__add__anti_scalar(multi_vector__round_bulk_norm(self), multi_vector__round_weight_norm(self));
}

DualNum round_point__round_norm(RoundPoint self) {
    return scalar__add__anti_scalar(round_point__round_bulk_norm(self), round_point__round_weight_norm(self));
}

float circle__unitized_round_norm_squared(Circle self) {
    return circle__round_bulk_norm_squared(self).g0 / circle__round_weight_norm_squared(self).g0;
}

float dipole__unitized_round_norm_squared(Dipole self) {
    return dipole__round_bulk_norm_squared(self).g0 / dipole__round_weight_norm_squared(self).g0;
}

float multi_vector__unitized_round_norm_squared(MultiVector self) {
    return multi_vector__round_bulk_norm_squared(self).g0 / multi_vector__round_weight_norm_squared(self).g0;
}

float round_point__unitized_round_norm_squared(RoundPoint self) {
    return round_point__round_bulk_norm_squared(self).g0 / round_point__round_weight_norm_squared(self).g0;
}

float circle__unitized_round_norm(Circle self) {
    return sqrt(circle__unitized_round_norm_squared(self));
}

float dipole__unitized_round_norm(Dipole self) {
    return sqrt(dipole__unitized_round_norm_squared(self));
}

float multi_vector__unitized_round_norm(MultiVector self) {
    return sqrt(multi_vector__unitized_round_norm_squared(self));
}

float round_point__unitized_round_norm(RoundPoint self) {
    return sqrt(round_point__unitized_round_norm_squared(self));
}

Scalar circle__flat_bulk_norm_squared(Circle self) {
    Sphere flat_bulk_thing = circle__wedge__round_point(circle__flat_bulk(self), RoundPoint(vec3(0.0), vec2(1.0, 0.0)));
    return sphere__dot__sphere(flat_bulk_thing, flat_bulk_thing);
}

Scalar dipole__flat_bulk_norm_squared(Dipole self) {
    Circle flat_bulk_thing = dipole__wedge__round_point(dipole__flat_bulk(self), RoundPoint(vec3(0.0), vec2(1.0, 0.0)));
    return circle__dot__circle(flat_bulk_thing, flat_bulk_thing);
}

Scalar flat_point__flat_bulk_norm_squared(FlatPoint self) {
    Line flat_bulk_thing = flat_point__wedge__round_point(flat_point__flat_bulk(self), RoundPoint(vec3(0.0), vec2(1.0, 0.0)));
    return line__dot__line(flat_bulk_thing, flat_bulk_thing);
}

Scalar flector__flat_bulk_norm_squared(Flector self) {
    Motor flat_bulk_thing = flector__wedge__round_point(flector__flat_bulk(self), RoundPoint(vec3(0.0), vec2(1.0, 0.0)));
    return motor__dot__motor(flat_bulk_thing, flat_bulk_thing);
}

Scalar line__flat_bulk_norm_squared(Line self) {
    Plane flat_bulk_thing = line__wedge__round_point(line__flat_bulk(self), RoundPoint(vec3(0.0), vec2(1.0, 0.0)));
    return plane__dot__plane(flat_bulk_thing, flat_bulk_thing);
}

Scalar motor__flat_bulk_norm_squared(Motor self) {
    Plane flat_bulk_thing = motor__wedge__round_point(motor__flat_bulk(self), RoundPoint(vec3(0.0), vec2(1.0, 0.0)));
    return plane__dot__plane(flat_bulk_thing, flat_bulk_thing);
}

Scalar multi_vector__flat_bulk_norm_squared(MultiVector self) {
    MultiVector flat_bulk_thing = multi_vector__wedge__round_point(multi_vector__flat_bulk(self), RoundPoint(vec3(0.0), vec2(1.0, 0.0)));
    return multi_vector__dot__multi_vector(flat_bulk_thing, flat_bulk_thing);
}

Scalar plane__flat_bulk_norm_squared(Plane self) {
    AntiScalar flat_bulk_thing = plane__wedge__round_point(plane__flat_bulk(self), RoundPoint(vec3(0.0), vec2(1.0, 0.0)));
    return anti_scalar__dot__anti_scalar(flat_bulk_thing, flat_bulk_thing);
}

Scalar round_point__flat_bulk_norm_squared(RoundPoint self) {
    Dipole flat_bulk_thing = round_point__wedge__round_point(round_point__flat_bulk(self), RoundPoint(vec3(0.0), vec2(1.0, 0.0)));
    return dipole__dot__dipole(flat_bulk_thing, flat_bulk_thing);
}

Scalar sphere__flat_bulk_norm_squared(Sphere self) {
    AntiScalar flat_bulk_thing = sphere__wedge__round_point(sphere__flat_bulk(self), RoundPoint(vec3(0.0), vec2(1.0, 0.0)));
    return anti_scalar__dot__anti_scalar(flat_bulk_thing, flat_bulk_thing);
}

Scalar circle__flat_bulk_norm(Circle self) {
    return scalar__sqrt(circle__flat_bulk_norm_squared(self));
}

Scalar dipole__flat_bulk_norm(Dipole self) {
    return scalar__sqrt(dipole__flat_bulk_norm_squared(self));
}

Scalar flat_point__flat_bulk_norm(FlatPoint self) {
    return scalar__sqrt(flat_point__flat_bulk_norm_squared(self));
}

Scalar flector__flat_bulk_norm(Flector self) {
    return scalar__sqrt(flector__flat_bulk_norm_squared(self));
}

Scalar line__flat_bulk_norm(Line self) {
    return scalar__sqrt(line__flat_bulk_norm_squared(self));
}

Scalar motor__flat_bulk_norm(Motor self) {
    return scalar__sqrt(motor__flat_bulk_norm_squared(self));
}

Scalar multi_vector__flat_bulk_norm(MultiVector self) {
    return scalar__sqrt(multi_vector__flat_bulk_norm_squared(self));
}

Scalar plane__flat_bulk_norm(Plane self) {
    return scalar__sqrt(plane__flat_bulk_norm_squared(self));
}

Scalar round_point__flat_bulk_norm(RoundPoint self) {
    return scalar__sqrt(round_point__flat_bulk_norm_squared(self));
}

Scalar sphere__flat_bulk_norm(Sphere self) {
    return scalar__sqrt(sphere__flat_bulk_norm_squared(self));
}

AntiScalar anti_scalar__flat_weight_norm_squared(AntiScalar self) {
    AntiScalar flat_weight = anti_scalar__flat_weight(self);
    return anti_scalar__anti_dot__anti_scalar(flat_weight, flat_weight);
}

AntiScalar circle__flat_weight_norm_squared(Circle self) {
    Circle flat_weight = circle__flat_weight(self);
    return circle__anti_dot__circle(flat_weight, flat_weight);
}

AntiScalar dipole__flat_weight_norm_squared(Dipole self) {
    Dipole flat_weight = dipole__flat_weight(self);
    return dipole__anti_dot__dipole(flat_weight, flat_weight);
}

AntiScalar dual_num__flat_weight_norm_squared(DualNum self) {
    DualNum flat_weight = dual_num__flat_weight(self);
    return dual_num__anti_dot__dual_num(flat_weight, flat_weight);
}

AntiScalar flat_point__flat_weight_norm_squared(FlatPoint self) {
    FlatPoint flat_weight = flat_point__flat_weight(self);
    return flat_point__anti_dot__flat_point(flat_weight, flat_weight);
}

AntiScalar flector__flat_weight_norm_squared(Flector self) {
    Flector flat_weight = flector__flat_weight(self);
    return flector__anti_dot__flector(flat_weight, flat_weight);
}

AntiScalar line__flat_weight_norm_squared(Line self) {
    Line flat_weight = line__flat_weight(self);
    return line__anti_dot__line(flat_weight, flat_weight);
}

AntiScalar motor__flat_weight_norm_squared(Motor self) {
    Motor flat_weight = motor__flat_weight(self);
    return motor__anti_dot__motor(flat_weight, flat_weight);
}

AntiScalar multi_vector__flat_weight_norm_squared(MultiVector self) {
    MultiVector flat_weight = multi_vector__flat_weight(self);
    return multi_vector__anti_dot__multi_vector(flat_weight, flat_weight);
}

AntiScalar plane__flat_weight_norm_squared(Plane self) {
    Plane flat_weight = plane__flat_weight(self);
    return plane__anti_dot__plane(flat_weight, flat_weight);
}

AntiScalar sphere__flat_weight_norm_squared(Sphere self) {
    Sphere flat_weight = sphere__flat_weight(self);
    return sphere__anti_dot__sphere(flat_weight, flat_weight);
}

AntiScalar anti_scalar__flat_weight_norm(AntiScalar self) {
    return anti_scalar__anti_sqrt(anti_scalar__flat_weight_norm_squared(self));
}

AntiScalar circle__flat_weight_norm(Circle self) {
    return anti_scalar__anti_sqrt(circle__flat_weight_norm_squared(self));
}

AntiScalar dipole__flat_weight_norm(Dipole self) {
    return anti_scalar__anti_sqrt(dipole__flat_weight_norm_squared(self));
}

AntiScalar dual_num__flat_weight_norm(DualNum self) {
    return anti_scalar__anti_sqrt(dual_num__flat_weight_norm_squared(self));
}

AntiScalar flat_point__flat_weight_norm(FlatPoint self) {
    return anti_scalar__anti_sqrt(flat_point__flat_weight_norm_squared(self));
}

AntiScalar flector__flat_weight_norm(Flector self) {
    return anti_scalar__anti_sqrt(flector__flat_weight_norm_squared(self));
}

AntiScalar line__flat_weight_norm(Line self) {
    return anti_scalar__anti_sqrt(line__flat_weight_norm_squared(self));
}

AntiScalar motor__flat_weight_norm(Motor self) {
    return anti_scalar__anti_sqrt(motor__flat_weight_norm_squared(self));
}

AntiScalar multi_vector__flat_weight_norm(MultiVector self) {
    return anti_scalar__anti_sqrt(multi_vector__flat_weight_norm_squared(self));
}

AntiScalar plane__flat_weight_norm(Plane self) {
    return anti_scalar__anti_sqrt(plane__flat_weight_norm_squared(self));
}

AntiScalar sphere__flat_weight_norm(Sphere self) {
    return anti_scalar__anti_sqrt(sphere__flat_weight_norm_squared(self));
}

DualNum circle__flat_norm_squared(Circle self) {
    return scalar__add__anti_scalar(circle__flat_bulk_norm_squared(self), circle__flat_weight_norm_squared(self));
}

DualNum dipole__flat_norm_squared(Dipole self) {
    return scalar__add__anti_scalar(dipole__flat_bulk_norm_squared(self), dipole__flat_weight_norm_squared(self));
}

DualNum flat_point__flat_norm_squared(FlatPoint self) {
    return scalar__add__anti_scalar(flat_point__flat_bulk_norm_squared(self), flat_point__flat_weight_norm_squared(self));
}

DualNum flector__flat_norm_squared(Flector self) {
    return scalar__add__anti_scalar(flector__flat_bulk_norm_squared(self), flector__flat_weight_norm_squared(self));
}

DualNum line__flat_norm_squared(Line self) {
    return scalar__add__anti_scalar(line__flat_bulk_norm_squared(self), line__flat_weight_norm_squared(self));
}

DualNum motor__flat_norm_squared(Motor self) {
    return scalar__add__anti_scalar(motor__flat_bulk_norm_squared(self), motor__flat_weight_norm_squared(self));
}

DualNum multi_vector__flat_norm_squared(MultiVector self) {
    return scalar__add__anti_scalar(multi_vector__flat_bulk_norm_squared(self), multi_vector__flat_weight_norm_squared(self));
}

DualNum plane__flat_norm_squared(Plane self) {
    return scalar__add__anti_scalar(plane__flat_bulk_norm_squared(self), plane__flat_weight_norm_squared(self));
}

DualNum sphere__flat_norm_squared(Sphere self) {
    return scalar__add__anti_scalar(sphere__flat_bulk_norm_squared(self), sphere__flat_weight_norm_squared(self));
}

DualNum circle__flat_norm(Circle self) {
    return scalar__add__anti_scalar(circle__flat_bulk_norm(self), circle__flat_weight_norm(self));
}

DualNum dipole__flat_norm(Dipole self) {
    return scalar__add__anti_scalar(dipole__flat_bulk_norm(self), dipole__flat_weight_norm(self));
}

DualNum flat_point__flat_norm(FlatPoint self) {
    return scalar__add__anti_scalar(flat_point__flat_bulk_norm(self), flat_point__flat_weight_norm(self));
}

DualNum flector__flat_norm(Flector self) {
    return scalar__add__anti_scalar(flector__flat_bulk_norm(self), flector__flat_weight_norm(self));
}

DualNum line__flat_norm(Line self) {
    return scalar__add__anti_scalar(line__flat_bulk_norm(self), line__flat_weight_norm(self));
}

DualNum motor__flat_norm(Motor self) {
    return scalar__add__anti_scalar(motor__flat_bulk_norm(self), motor__flat_weight_norm(self));
}

DualNum multi_vector__flat_norm(MultiVector self) {
    return scalar__add__anti_scalar(multi_vector__flat_bulk_norm(self), multi_vector__flat_weight_norm(self));
}

DualNum plane__flat_norm(Plane self) {
    return scalar__add__anti_scalar(plane__flat_bulk_norm(self), plane__flat_weight_norm(self));
}

DualNum sphere__flat_norm(Sphere self) {
    return scalar__add__anti_scalar(sphere__flat_bulk_norm(self), sphere__flat_weight_norm(self));
}

float circle__unitized_flat_norm_squared(Circle self) {
    return circle__flat_bulk_norm_squared(self).g0 / circle__flat_weight_norm_squared(self).g0;
}

float dipole__unitized_flat_norm_squared(Dipole self) {
    return dipole__flat_bulk_norm_squared(self).g0 / dipole__flat_weight_norm_squared(self).g0;
}

float flat_point__unitized_flat_norm_squared(FlatPoint self) {
    return flat_point__flat_bulk_norm_squared(self).g0 / flat_point__flat_weight_norm_squared(self).g0;
}

float flector__unitized_flat_norm_squared(Flector self) {
    return flector__flat_bulk_norm_squared(self).g0 / flector__flat_weight_norm_squared(self).g0;
}

float line__unitized_flat_norm_squared(Line self) {
    return line__flat_bulk_norm_squared(self).g0 / line__flat_weight_norm_squared(self).g0;
}

float motor__unitized_flat_norm_squared(Motor self) {
    return motor__flat_bulk_norm_squared(self).g0 / motor__flat_weight_norm_squared(self).g0;
}

float multi_vector__unitized_flat_norm_squared(MultiVector self) {
    return multi_vector__flat_bulk_norm_squared(self).g0 / multi_vector__flat_weight_norm_squared(self).g0;
}

float plane__unitized_flat_norm_squared(Plane self) {
    return plane__flat_bulk_norm_squared(self).g0 / plane__flat_weight_norm_squared(self).g0;
}

float sphere__unitized_flat_norm_squared(Sphere self) {
    return sphere__flat_bulk_norm_squared(self).g0 / sphere__flat_weight_norm_squared(self).g0;
}

float circle__unitized_flat_norm(Circle self) {
    return sqrt(circle__unitized_flat_norm_squared(self));
}

float dipole__unitized_flat_norm(Dipole self) {
    return sqrt(dipole__unitized_flat_norm_squared(self));
}

float flat_point__unitized_flat_norm(FlatPoint self) {
    return sqrt(flat_point__unitized_flat_norm_squared(self));
}

float flector__unitized_flat_norm(Flector self) {
    return sqrt(flector__unitized_flat_norm_squared(self));
}

float line__unitized_flat_norm(Line self) {
    return sqrt(line__unitized_flat_norm_squared(self));
}

float motor__unitized_flat_norm(Motor self) {
    return sqrt(motor__unitized_flat_norm_squared(self));
}

float multi_vector__unitized_flat_norm(MultiVector self) {
    return sqrt(multi_vector__unitized_flat_norm_squared(self));
}

float plane__unitized_flat_norm(Plane self) {
    return sqrt(plane__unitized_flat_norm_squared(self));
}

float sphere__unitized_flat_norm(Sphere self) {
    return sqrt(sphere__unitized_flat_norm_squared(self));
}

Scalar circle__center_norm_squared(Circle self) {
    return scalar__add__scalar(circle__round_bulk_norm_squared(self), anti_scalar__anti_dual(circle__flat_weight_norm_squared(self)));
}

Scalar dipole__center_norm_squared(Dipole self) {
    return scalar__add__scalar(dipole__round_bulk_norm_squared(self), anti_scalar__anti_dual(dipole__flat_weight_norm_squared(self)));
}

Scalar dual_num__center_norm_squared(DualNum self) {
    return scalar__add__scalar(dual_num__round_bulk_norm_squared(self), anti_scalar__anti_dual(dual_num__flat_weight_norm_squared(self)));
}

Scalar multi_vector__center_norm_squared(MultiVector self) {
    return scalar__add__scalar(multi_vector__round_bulk_norm_squared(self), anti_scalar__anti_dual(multi_vector__flat_weight_norm_squared(self)));
}

Scalar circle__center_norm(Circle self) {
    return scalar__sqrt(circle__center_norm_squared(self));
}

Scalar dipole__center_norm(Dipole self) {
    return scalar__sqrt(dipole__center_norm_squared(self));
}

Scalar dual_num__center_norm(DualNum self) {
    return scalar__sqrt(dual_num__center_norm_squared(self));
}

Scalar multi_vector__center_norm(MultiVector self) {
    return scalar__sqrt(multi_vector__center_norm_squared(self));
}

float circle__unitized_center_norm_squared(Circle self) {
    return circle__center_norm_squared(self).g0 / circle__round_weight_norm_squared(self).g0;
}

float dipole__unitized_center_norm_squared(Dipole self) {
    return dipole__center_norm_squared(self).g0 / dipole__round_weight_norm_squared(self).g0;
}

float multi_vector__unitized_center_norm_squared(MultiVector self) {
    return multi_vector__center_norm_squared(self).g0 / multi_vector__round_weight_norm_squared(self).g0;
}

float circle__unitized_center_norm(Circle self) {
    return sqrt(circle__unitized_center_norm_squared(self));
}

float dipole__unitized_center_norm(Dipole self) {
    return sqrt(dipole__unitized_center_norm_squared(self));
}

float multi_vector__unitized_center_norm(MultiVector self) {
    return sqrt(multi_vector__unitized_center_norm_squared(self));
}

Scalar circle__radius_norm_squared(Circle self) {
    return anti_scalar__anti_dual(circle__anti_dot__circle(self, self));
}

Scalar dipole__radius_norm_squared(Dipole self) {
    return anti_scalar__anti_dual(dipole__anti_dot__dipole(self, self));
}

Scalar dual_num__radius_norm_squared(DualNum self) {
    return anti_scalar__anti_dual(dual_num__anti_dot__dual_num(self, self));
}

Scalar multi_vector__radius_norm_squared(MultiVector self) {
    return anti_scalar__anti_dual(multi_vector__anti_dot__multi_vector(self, self));
}

Scalar round_point__radius_norm_squared(RoundPoint self) {
    return anti_scalar__anti_dual(round_point__anti_dot__round_point(self, self));
}

Scalar sphere__radius_norm_squared(Sphere self) {
    return anti_scalar__anti_dual(sphere__anti_dot__sphere(self, self));
}

Scalar circle__radius_norm(Circle self) {
    return scalar__sqrt(circle__radius_norm_squared(self));
}

Scalar dipole__radius_norm(Dipole self) {
    return scalar__sqrt(dipole__radius_norm_squared(self));
}

Scalar dual_num__radius_norm(DualNum self) {
    return scalar__sqrt(dual_num__radius_norm_squared(self));
}

Scalar multi_vector__radius_norm(MultiVector self) {
    return scalar__sqrt(multi_vector__radius_norm_squared(self));
}

Scalar round_point__radius_norm(RoundPoint self) {
    return scalar__sqrt(round_point__radius_norm_squared(self));
}

Scalar sphere__radius_norm(Sphere self) {
    return scalar__sqrt(sphere__radius_norm_squared(self));
}

float circle__unitized_radius_norm_squared(Circle self) {
    return circle__radius_norm_squared(self).g0 / circle__round_weight_norm_squared(self).g0;
}

float dipole__unitized_radius_norm_squared(Dipole self) {
    return dipole__radius_norm_squared(self).g0 / dipole__round_weight_norm_squared(self).g0;
}

float multi_vector__unitized_radius_norm_squared(MultiVector self) {
    return multi_vector__radius_norm_squared(self).g0 / multi_vector__round_weight_norm_squared(self).g0;
}

float round_point__unitized_radius_norm_squared(RoundPoint self) {
    return round_point__radius_norm_squared(self).g0 / round_point__round_weight_norm_squared(self).g0;
}

float sphere__unitized_radius_norm_squared(Sphere self) {
    return sphere__radius_norm_squared(self).g0 / sphere__round_weight_norm_squared(self).g0;
}

float circle__unitized_radius_norm(Circle self) {
    return sqrt(circle__unitized_radius_norm_squared(self));
}

float dipole__unitized_radius_norm(Dipole self) {
    return sqrt(dipole__unitized_radius_norm_squared(self));
}

float multi_vector__unitized_radius_norm(MultiVector self) {
    return sqrt(multi_vector__unitized_radius_norm_squared(self));
}

float round_point__unitized_radius_norm(RoundPoint self) {
    return sqrt(round_point__unitized_radius_norm_squared(self));
}

float sphere__unitized_radius_norm(Sphere self) {
    return sqrt(sphere__unitized_radius_norm_squared(self));
}

Circle circle__unitize(Circle self) {
    return circle__wedge_dot__scalar(self, Scalar(1.0 / circle__round_weight_norm(self).g0));
}

Dipole dipole__unitize(Dipole self) {
    return dipole__wedge_dot__scalar(self, Scalar(1.0 / dipole__round_weight_norm(self).g0));
}

MultiVector multi_vector__unitize(MultiVector self) {
    return multi_vector__wedge_dot__scalar(self, Scalar(1.0 / multi_vector__round_weight_norm(self).g0));
}

RoundPoint round_point__unitize(RoundPoint self) {
    return round_point__wedge_dot__scalar(self, Scalar(1.0 / round_point__round_weight_norm(self).g0));
}

Sphere sphere__unitize(Sphere self) {
    return sphere__wedge_dot__scalar(self, Scalar(1.0 / sphere__round_weight_norm(self).g0));
}

Circle anti_scalar__sandwich__circle(AntiScalar self, Circle other) {
    return circle__anti_wedge_dot__anti_scalar(anti_scalar__anti_wedge_dot__circle(self, other), anti_scalar__anti_reversal(self));
}

Dipole anti_scalar__sandwich__dipole(AntiScalar self, Dipole other) {
    return dipole__anti_wedge_dot__anti_scalar(anti_scalar__anti_wedge_dot__dipole(self, other), anti_scalar__anti_reversal(self));
}

FlatPoint anti_scalar__sandwich__flat_point(AntiScalar self, FlatPoint other) {
    return flat_point__anti_wedge_dot__anti_scalar(anti_scalar__anti_wedge_dot__flat_point(self, other), anti_scalar__anti_reversal(self));
}

Flector anti_scalar__sandwich__flector(AntiScalar self, Flector other) {
    return flector__anti_wedge_dot__anti_scalar(anti_scalar__anti_wedge_dot__flector(self, other), anti_scalar__anti_reversal(self));
}

Line anti_scalar__sandwich__line(AntiScalar self, Line other) {
    return line__anti_wedge_dot__anti_scalar(anti_scalar__anti_wedge_dot__line(self, other), anti_scalar__anti_reversal(self));
}

Motor anti_scalar__sandwich__motor(AntiScalar self, Motor other) {
    return motor__anti_wedge_dot__anti_scalar(anti_scalar__anti_wedge_dot__motor(self, other), anti_scalar__anti_reversal(self));
}

MultiVector anti_scalar__sandwich__multi_vector(AntiScalar self, MultiVector other) {
    return multi_vector__anti_wedge_dot__anti_scalar(anti_scalar__anti_wedge_dot__multi_vector(self, other), anti_scalar__anti_reversal(self));
}

Plane anti_scalar__sandwich__plane(AntiScalar self, Plane other) {
    return plane__anti_wedge_dot__anti_scalar(anti_scalar__anti_wedge_dot__plane(self, other), anti_scalar__anti_reversal(self));
}

RoundPoint anti_scalar__sandwich__round_point(AntiScalar self, RoundPoint other) {
    return round_point__anti_wedge_dot__anti_scalar(anti_scalar__anti_wedge_dot__round_point(self, other), anti_scalar__anti_reversal(self));
}

Sphere anti_scalar__sandwich__sphere(AntiScalar self, Sphere other) {
    return sphere__anti_wedge_dot__anti_scalar(anti_scalar__anti_wedge_dot__sphere(self, other), anti_scalar__anti_reversal(self));
}

Circle circle__sandwich__circle(Circle self, Circle other) {
    return multi_vector__into__circle(multi_vector__anti_wedge_dot__circle(circle__anti_wedge_dot__circle(self, other), circle__anti_reversal(self)));
}

Dipole circle__sandwich__dipole(Circle self, Dipole other) {
    return multi_vector__into__dipole(multi_vector__anti_wedge_dot__circle(circle__anti_wedge_dot__dipole(self, other), circle__anti_reversal(self)));
}

FlatPoint circle__sandwich__flat_point(Circle self, FlatPoint other) {
    return multi_vector__into__flat_point(multi_vector__anti_wedge_dot__circle(circle__anti_wedge_dot__flat_point(self, other), circle__anti_reversal(self)));
}

Flector circle__sandwich__flector(Circle self, Flector other) {
    return multi_vector__into__flector(multi_vector__anti_wedge_dot__circle(circle__anti_wedge_dot__flector(self, other), circle__anti_reversal(self)));
}

Line circle__sandwich__line(Circle self, Line other) {
    return multi_vector__into__line(multi_vector__anti_wedge_dot__circle(circle__anti_wedge_dot__line(self, other), circle__anti_reversal(self)));
}

Motor circle__sandwich__motor(Circle self, Motor other) {
    return multi_vector__into__motor(multi_vector__anti_wedge_dot__circle(circle__anti_wedge_dot__motor(self, other), circle__anti_reversal(self)));
}

MultiVector circle__sandwich__multi_vector(Circle self, MultiVector other) {
    return multi_vector__anti_wedge_dot__circle(circle__anti_wedge_dot__multi_vector(self, other), circle__anti_reversal(self));
}

Plane circle__sandwich__plane(Circle self, Plane other) {
    return multi_vector__into__plane(multi_vector__anti_wedge_dot__circle(circle__anti_wedge_dot__plane(self, other), circle__anti_reversal(self)));
}

RoundPoint circle__sandwich__round_point(Circle self, RoundPoint other) {
    return multi_vector__into__round_point(multi_vector__anti_wedge_dot__circle(circle__anti_wedge_dot__round_point(self, other), circle__anti_reversal(self)));
}

Sphere circle__sandwich__sphere(Circle self, Sphere other) {
    return multi_vector__into__sphere(multi_vector__anti_wedge_dot__circle(circle__anti_wedge_dot__sphere(self, other), circle__anti_reversal(self)));
}

Circle dipole__sandwich__circle(Dipole self, Circle other) {
    return multi_vector__into__circle(multi_vector__anti_wedge_dot__dipole(dipole__anti_wedge_dot__circle(self, other), dipole__anti_reversal(self)));
}

Dipole dipole__sandwich__dipole(Dipole self, Dipole other) {
    return multi_vector__into__dipole(multi_vector__anti_wedge_dot__dipole(dipole__anti_wedge_dot__dipole(self, other), dipole__anti_reversal(self)));
}

FlatPoint dipole__sandwich__flat_point(Dipole self, FlatPoint other) {
    return multi_vector__into__flat_point(multi_vector__anti_wedge_dot__dipole(dipole__anti_wedge_dot__flat_point(self, other), dipole__anti_reversal(self)));
}

Flector dipole__sandwich__flector(Dipole self, Flector other) {
    return multi_vector__into__flector(multi_vector__anti_wedge_dot__dipole(dipole__anti_wedge_dot__flector(self, other), dipole__anti_reversal(self)));
}

Line dipole__sandwich__line(Dipole self, Line other) {
    return multi_vector__into__line(multi_vector__anti_wedge_dot__dipole(dipole__anti_wedge_dot__line(self, other), dipole__anti_reversal(self)));
}

Motor dipole__sandwich__motor(Dipole self, Motor other) {
    return multi_vector__into__motor(multi_vector__anti_wedge_dot__dipole(dipole__anti_wedge_dot__motor(self, other), dipole__anti_reversal(self)));
}

MultiVector dipole__sandwich__multi_vector(Dipole self, MultiVector other) {
    return multi_vector__anti_wedge_dot__dipole(dipole__anti_wedge_dot__multi_vector(self, other), dipole__anti_reversal(self));
}

Plane dipole__sandwich__plane(Dipole self, Plane other) {
    return multi_vector__into__plane(multi_vector__anti_wedge_dot__dipole(dipole__anti_wedge_dot__plane(self, other), dipole__anti_reversal(self)));
}

RoundPoint dipole__sandwich__round_point(Dipole self, RoundPoint other) {
    return multi_vector__into__round_point(multi_vector__anti_wedge_dot__dipole(dipole__anti_wedge_dot__round_point(self, other), dipole__anti_reversal(self)));
}

Sphere dipole__sandwich__sphere(Dipole self, Sphere other) {
    return multi_vector__into__sphere(multi_vector__anti_wedge_dot__dipole(dipole__anti_wedge_dot__sphere(self, other), dipole__anti_reversal(self)));
}

Circle dual_num__sandwich__circle(DualNum self, Circle other) {
    return multi_vector__into__circle(multi_vector__anti_wedge_dot__dual_num(dual_num__anti_wedge_dot__circle(self, other), dual_num__anti_reversal(self)));
}

Dipole dual_num__sandwich__dipole(DualNum self, Dipole other) {
    return multi_vector__into__dipole(multi_vector__anti_wedge_dot__dual_num(dual_num__anti_wedge_dot__dipole(self, other), dual_num__anti_reversal(self)));
}

FlatPoint dual_num__sandwich__flat_point(DualNum self, FlatPoint other) {
    return multi_vector__into__flat_point(multi_vector__anti_wedge_dot__dual_num(dual_num__anti_wedge_dot__flat_point(self, other), dual_num__anti_reversal(self)));
}

Flector dual_num__sandwich__flector(DualNum self, Flector other) {
    return multi_vector__into__flector(multi_vector__anti_wedge_dot__dual_num(dual_num__anti_wedge_dot__flector(self, other), dual_num__anti_reversal(self)));
}

Line dual_num__sandwich__line(DualNum self, Line other) {
    return multi_vector__into__line(multi_vector__anti_wedge_dot__dual_num(dual_num__anti_wedge_dot__line(self, other), dual_num__anti_reversal(self)));
}

Motor dual_num__sandwich__motor(DualNum self, Motor other) {
    return multi_vector__into__motor(multi_vector__anti_wedge_dot__dual_num(dual_num__anti_wedge_dot__motor(self, other), dual_num__anti_reversal(self)));
}

MultiVector dual_num__sandwich__multi_vector(DualNum self, MultiVector other) {
    return multi_vector__anti_wedge_dot__dual_num(dual_num__anti_wedge_dot__multi_vector(self, other), dual_num__anti_reversal(self));
}

Plane dual_num__sandwich__plane(DualNum self, Plane other) {
    return multi_vector__into__plane(multi_vector__anti_wedge_dot__dual_num(dual_num__anti_wedge_dot__plane(self, other), dual_num__anti_reversal(self)));
}

RoundPoint dual_num__sandwich__round_point(DualNum self, RoundPoint other) {
    return multi_vector__into__round_point(multi_vector__anti_wedge_dot__dual_num(dual_num__anti_wedge_dot__round_point(self, other), dual_num__anti_reversal(self)));
}

Sphere dual_num__sandwich__sphere(DualNum self, Sphere other) {
    return multi_vector__into__sphere(multi_vector__anti_wedge_dot__dual_num(dual_num__anti_wedge_dot__sphere(self, other), dual_num__anti_reversal(self)));
}

Circle flat_point__sandwich__circle(FlatPoint self, Circle other) {
    return multi_vector__into__circle(multi_vector__anti_wedge_dot__flat_point(flat_point__anti_wedge_dot__circle(self, other), flat_point__anti_reversal(self)));
}

Dipole flat_point__sandwich__dipole(FlatPoint self, Dipole other) {
    return multi_vector__into__dipole(multi_vector__anti_wedge_dot__flat_point(flat_point__anti_wedge_dot__dipole(self, other), flat_point__anti_reversal(self)));
}

FlatPoint flat_point__sandwich__flat_point(FlatPoint self, FlatPoint other) {
    return flector__into__flat_point(motor__anti_wedge_dot__flat_point(flat_point__anti_wedge_dot__flat_point(self, other), flat_point__anti_reversal(self)));
}

Flector flat_point__sandwich__flector(FlatPoint self, Flector other) {
    return multi_vector__into__flector(multi_vector__anti_wedge_dot__flat_point(flat_point__anti_wedge_dot__flector(self, other), flat_point__anti_reversal(self)));
}

Line flat_point__sandwich__line(FlatPoint self, Line other) {
    return multi_vector__into__line(flector__anti_wedge_dot__flat_point(flat_point__anti_wedge_dot__line(self, other), flat_point__anti_reversal(self)));
}

Motor flat_point__sandwich__motor(FlatPoint self, Motor other) {
    return multi_vector__into__motor(flector__anti_wedge_dot__flat_point(flat_point__anti_wedge_dot__motor(self, other), flat_point__anti_reversal(self)));
}

MultiVector flat_point__sandwich__multi_vector(FlatPoint self, MultiVector other) {
    return multi_vector__anti_wedge_dot__flat_point(flat_point__anti_wedge_dot__multi_vector(self, other), flat_point__anti_reversal(self));
}

Plane flat_point__sandwich__plane(FlatPoint self, Plane other) {
    return multi_vector__into__plane(multi_vector__anti_wedge_dot__flat_point(flat_point__anti_wedge_dot__plane(self, other), flat_point__anti_reversal(self)));
}

RoundPoint flat_point__sandwich__round_point(FlatPoint self, RoundPoint other) {
    return multi_vector__into__round_point(multi_vector__anti_wedge_dot__flat_point(flat_point__anti_wedge_dot__round_point(self, other), flat_point__anti_reversal(self)));
}

Sphere flat_point__sandwich__sphere(FlatPoint self, Sphere other) {
    return multi_vector__into__sphere(multi_vector__anti_wedge_dot__flat_point(flat_point__anti_wedge_dot__sphere(self, other), flat_point__anti_reversal(self)));
}

Circle flector__sandwich__circle(Flector self, Circle other) {
    return multi_vector__into__circle(multi_vector__anti_wedge_dot__flector(flector__anti_wedge_dot__circle(self, other), flector__anti_reversal(self)));
}

Dipole flector__sandwich__dipole(Flector self, Dipole other) {
    return multi_vector__into__dipole(multi_vector__anti_wedge_dot__flector(flector__anti_wedge_dot__dipole(self, other), flector__anti_reversal(self)));
}

FlatPoint flector__sandwich__flat_point(Flector self, FlatPoint other) {
    return multi_vector__into__flat_point(multi_vector__anti_wedge_dot__flector(flector__anti_wedge_dot__flat_point(self, other), flector__anti_reversal(self)));
}

Flector flector__sandwich__flector(Flector self, Flector other) {
    return multi_vector__into__flector(multi_vector__anti_wedge_dot__flector(flector__anti_wedge_dot__flector(self, other), flector__anti_reversal(self)));
}

Line flector__sandwich__line(Flector self, Line other) {
    return multi_vector__into__line(flector__anti_wedge_dot__flector(flector__anti_wedge_dot__line(self, other), flector__anti_reversal(self)));
}

Motor flector__sandwich__motor(Flector self, Motor other) {
    return multi_vector__into__motor(flector__anti_wedge_dot__flector(flector__anti_wedge_dot__motor(self, other), flector__anti_reversal(self)));
}

MultiVector flector__sandwich__multi_vector(Flector self, MultiVector other) {
    return multi_vector__anti_wedge_dot__flector(flector__anti_wedge_dot__multi_vector(self, other), flector__anti_reversal(self));
}

Plane flector__sandwich__plane(Flector self, Plane other) {
    return multi_vector__into__plane(multi_vector__anti_wedge_dot__flector(flector__anti_wedge_dot__plane(self, other), flector__anti_reversal(self)));
}

RoundPoint flector__sandwich__round_point(Flector self, RoundPoint other) {
    return multi_vector__into__round_point(multi_vector__anti_wedge_dot__flector(flector__anti_wedge_dot__round_point(self, other), flector__anti_reversal(self)));
}

Sphere flector__sandwich__sphere(Flector self, Sphere other) {
    return multi_vector__into__sphere(multi_vector__anti_wedge_dot__flector(flector__anti_wedge_dot__sphere(self, other), flector__anti_reversal(self)));
}

Circle line__sandwich__circle(Line self, Circle other) {
    return multi_vector__into__circle(multi_vector__anti_wedge_dot__line(line__anti_wedge_dot__circle(self, other), line__anti_reversal(self)));
}

Dipole line__sandwich__dipole(Line self, Dipole other) {
    return multi_vector__into__dipole(multi_vector__anti_wedge_dot__line(line__anti_wedge_dot__dipole(self, other), line__anti_reversal(self)));
}

FlatPoint line__sandwich__flat_point(Line self, FlatPoint other) {
    return flector__into__flat_point(flector__anti_wedge_dot__line(line__anti_wedge_dot__flat_point(self, other), line__anti_reversal(self)));
}

Flector line__sandwich__flector(Line self, Flector other) {
    return flector__anti_wedge_dot__line(line__anti_wedge_dot__flector(self, other), line__anti_reversal(self));
}

Line line__sandwich__line(Line self, Line other) {
    return multi_vector__into__line(multi_vector__anti_wedge_dot__line(line__anti_wedge_dot__line(self, other), line__anti_reversal(self)));
}

Motor line__sandwich__motor(Line self, Motor other) {
    return multi_vector__into__motor(multi_vector__anti_wedge_dot__line(line__anti_wedge_dot__motor(self, other), line__anti_reversal(self)));
}

MultiVector line__sandwich__multi_vector(Line self, MultiVector other) {
    return multi_vector__anti_wedge_dot__line(line__anti_wedge_dot__multi_vector(self, other), line__anti_reversal(self));
}

Plane line__sandwich__plane(Line self, Plane other) {
    return flector__into__plane(flector__anti_wedge_dot__line(line__anti_wedge_dot__plane(self, other), line__anti_reversal(self)));
}

RoundPoint line__sandwich__round_point(Line self, RoundPoint other) {
    return multi_vector__into__round_point(multi_vector__anti_wedge_dot__line(line__anti_wedge_dot__round_point(self, other), line__anti_reversal(self)));
}

Sphere line__sandwich__sphere(Line self, Sphere other) {
    return multi_vector__into__sphere(multi_vector__anti_wedge_dot__line(line__anti_wedge_dot__sphere(self, other), line__anti_reversal(self)));
}

Circle motor__sandwich__circle(Motor self, Circle other) {
    return multi_vector__into__circle(multi_vector__anti_wedge_dot__motor(motor__anti_wedge_dot__circle(self, other), motor__anti_reversal(self)));
}

Dipole motor__sandwich__dipole(Motor self, Dipole other) {
    return multi_vector__into__dipole(multi_vector__anti_wedge_dot__motor(motor__anti_wedge_dot__dipole(self, other), motor__anti_reversal(self)));
}

FlatPoint motor__sandwich__flat_point(Motor self, FlatPoint other) {
    return flector__into__flat_point(flector__anti_wedge_dot__motor(motor__anti_wedge_dot__flat_point(self, other), motor__anti_reversal(self)));
}

Flector motor__sandwich__flector(Motor self, Flector other) {
    return flector__anti_wedge_dot__motor(motor__anti_wedge_dot__flector(self, other), motor__anti_reversal(self));
}

Line motor__sandwich__line(Motor self, Line other) {
    return multi_vector__into__line(multi_vector__anti_wedge_dot__motor(motor__anti_wedge_dot__line(self, other), motor__anti_reversal(self)));
}

Motor motor__sandwich__motor(Motor self, Motor other) {
    return multi_vector__into__motor(multi_vector__anti_wedge_dot__motor(motor__anti_wedge_dot__motor(self, other), motor__anti_reversal(self)));
}

MultiVector motor__sandwich__multi_vector(Motor self, MultiVector other) {
    return multi_vector__anti_wedge_dot__motor(motor__anti_wedge_dot__multi_vector(self, other), motor__anti_reversal(self));
}

Plane motor__sandwich__plane(Motor self, Plane other) {
    return flector__into__plane(flector__anti_wedge_dot__motor(motor__anti_wedge_dot__plane(self, other), motor__anti_reversal(self)));
}

RoundPoint motor__sandwich__round_point(Motor self, RoundPoint other) {
    return multi_vector__into__round_point(multi_vector__anti_wedge_dot__motor(motor__anti_wedge_dot__round_point(self, other), motor__anti_reversal(self)));
}

Sphere motor__sandwich__sphere(Motor self, Sphere other) {
    return multi_vector__into__sphere(multi_vector__anti_wedge_dot__motor(motor__anti_wedge_dot__sphere(self, other), motor__anti_reversal(self)));
}

Circle multi_vector__sandwich__circle(MultiVector self, Circle other) {
    return multi_vector__into__circle(multi_vector__anti_wedge_dot__multi_vector(multi_vector__anti_wedge_dot__circle(self, other), multi_vector__anti_reversal(self)));
}

Dipole multi_vector__sandwich__dipole(MultiVector self, Dipole other) {
    return multi_vector__into__dipole(multi_vector__anti_wedge_dot__multi_vector(multi_vector__anti_wedge_dot__dipole(self, other), multi_vector__anti_reversal(self)));
}

FlatPoint multi_vector__sandwich__flat_point(MultiVector self, FlatPoint other) {
    return multi_vector__into__flat_point(multi_vector__anti_wedge_dot__multi_vector(multi_vector__anti_wedge_dot__flat_point(self, other), multi_vector__anti_reversal(self)));
}

Flector multi_vector__sandwich__flector(MultiVector self, Flector other) {
    return multi_vector__into__flector(multi_vector__anti_wedge_dot__multi_vector(multi_vector__anti_wedge_dot__flector(self, other), multi_vector__anti_reversal(self)));
}

Line multi_vector__sandwich__line(MultiVector self, Line other) {
    return multi_vector__into__line(multi_vector__anti_wedge_dot__multi_vector(multi_vector__anti_wedge_dot__line(self, other), multi_vector__anti_reversal(self)));
}

Motor multi_vector__sandwich__motor(MultiVector self, Motor other) {
    return multi_vector__into__motor(multi_vector__anti_wedge_dot__multi_vector(multi_vector__anti_wedge_dot__motor(self, other), multi_vector__anti_reversal(self)));
}

MultiVector multi_vector__sandwich__multi_vector(MultiVector self, MultiVector other) {
    return multi_vector__anti_wedge_dot__multi_vector(multi_vector__anti_wedge_dot__multi_vector(self, other), multi_vector__anti_reversal(self));
}

Plane multi_vector__sandwich__plane(MultiVector self, Plane other) {
    return multi_vector__into__plane(multi_vector__anti_wedge_dot__multi_vector(multi_vector__anti_wedge_dot__plane(self, other), multi_vector__anti_reversal(self)));
}

RoundPoint multi_vector__sandwich__round_point(MultiVector self, RoundPoint other) {
    return multi_vector__into__round_point(multi_vector__anti_wedge_dot__multi_vector(multi_vector__anti_wedge_dot__round_point(self, other), multi_vector__anti_reversal(self)));
}

Sphere multi_vector__sandwich__sphere(MultiVector self, Sphere other) {
    return multi_vector__into__sphere(multi_vector__anti_wedge_dot__multi_vector(multi_vector__anti_wedge_dot__sphere(self, other), multi_vector__anti_reversal(self)));
}

Circle plane__sandwich__circle(Plane self, Circle other) {
    return multi_vector__into__circle(multi_vector__anti_wedge_dot__plane(plane__anti_wedge_dot__circle(self, other), plane__anti_reversal(self)));
}

Dipole plane__sandwich__dipole(Plane self, Dipole other) {
    return multi_vector__into__dipole(multi_vector__anti_wedge_dot__plane(plane__anti_wedge_dot__dipole(self, other), plane__anti_reversal(self)));
}

FlatPoint plane__sandwich__flat_point(Plane self, FlatPoint other) {
    return multi_vector__into__flat_point(multi_vector__anti_wedge_dot__plane(plane__anti_wedge_dot__flat_point(self, other), plane__anti_reversal(self)));
}

Flector plane__sandwich__flector(Plane self, Flector other) {
    return multi_vector__into__flector(multi_vector__anti_wedge_dot__plane(plane__anti_wedge_dot__flector(self, other), plane__anti_reversal(self)));
}

Line plane__sandwich__line(Plane self, Line other) {
    return multi_vector__into__line(flector__anti_wedge_dot__plane(plane__anti_wedge_dot__line(self, other), plane__anti_reversal(self)));
}

Motor plane__sandwich__motor(Plane self, Motor other) {
    return multi_vector__into__motor(flector__anti_wedge_dot__plane(plane__anti_wedge_dot__motor(self, other), plane__anti_reversal(self)));
}

MultiVector plane__sandwich__multi_vector(Plane self, MultiVector other) {
    return multi_vector__anti_wedge_dot__plane(plane__anti_wedge_dot__multi_vector(self, other), plane__anti_reversal(self));
}

Plane plane__sandwich__plane(Plane self, Plane other) {
    return flector__into__plane(motor__anti_wedge_dot__plane(plane__anti_wedge_dot__plane(self, other), plane__anti_reversal(self)));
}

RoundPoint plane__sandwich__round_point(Plane self, RoundPoint other) {
    return multi_vector__into__round_point(multi_vector__anti_wedge_dot__plane(plane__anti_wedge_dot__round_point(self, other), plane__anti_reversal(self)));
}

Sphere plane__sandwich__sphere(Plane self, Sphere other) {
    return multi_vector__into__sphere(multi_vector__anti_wedge_dot__plane(plane__anti_wedge_dot__sphere(self, other), plane__anti_reversal(self)));
}

Circle round_point__sandwich__circle(RoundPoint self, Circle other) {
    return multi_vector__into__circle(multi_vector__anti_wedge_dot__round_point(round_point__anti_wedge_dot__circle(self, other), round_point__anti_reversal(self)));
}

Dipole round_point__sandwich__dipole(RoundPoint self, Dipole other) {
    return multi_vector__into__dipole(multi_vector__anti_wedge_dot__round_point(round_point__anti_wedge_dot__dipole(self, other), round_point__anti_reversal(self)));
}

FlatPoint round_point__sandwich__flat_point(RoundPoint self, FlatPoint other) {
    return multi_vector__into__flat_point(multi_vector__anti_wedge_dot__round_point(round_point__anti_wedge_dot__flat_point(self, other), round_point__anti_reversal(self)));
}

Flector round_point__sandwich__flector(RoundPoint self, Flector other) {
    return multi_vector__into__flector(multi_vector__anti_wedge_dot__round_point(round_point__anti_wedge_dot__flector(self, other), round_point__anti_reversal(self)));
}

Line round_point__sandwich__line(RoundPoint self, Line other) {
    return multi_vector__into__line(multi_vector__anti_wedge_dot__round_point(round_point__anti_wedge_dot__line(self, other), round_point__anti_reversal(self)));
}

Motor round_point__sandwich__motor(RoundPoint self, Motor other) {
    return multi_vector__into__motor(multi_vector__anti_wedge_dot__round_point(round_point__anti_wedge_dot__motor(self, other), round_point__anti_reversal(self)));
}

MultiVector round_point__sandwich__multi_vector(RoundPoint self, MultiVector other) {
    return multi_vector__anti_wedge_dot__round_point(round_point__anti_wedge_dot__multi_vector(self, other), round_point__anti_reversal(self));
}

Plane round_point__sandwich__plane(RoundPoint self, Plane other) {
    return multi_vector__into__plane(multi_vector__anti_wedge_dot__round_point(round_point__anti_wedge_dot__plane(self, other), round_point__anti_reversal(self)));
}

RoundPoint round_point__sandwich__round_point(RoundPoint self, RoundPoint other) {
    return multi_vector__into__round_point(multi_vector__anti_wedge_dot__round_point(round_point__anti_wedge_dot__round_point(self, other), round_point__anti_reversal(self)));
}

Sphere round_point__sandwich__sphere(RoundPoint self, Sphere other) {
    return multi_vector__into__sphere(multi_vector__anti_wedge_dot__round_point(round_point__anti_wedge_dot__sphere(self, other), round_point__anti_reversal(self)));
}

Circle scalar__sandwich__circle(Scalar self, Circle other) {
    return dipole__anti_wedge_dot__scalar(scalar__anti_wedge_dot__circle(self, other), scalar__anti_reversal(self));
}

Dipole scalar__sandwich__dipole(Scalar self, Dipole other) {
    return circle__anti_wedge_dot__scalar(scalar__anti_wedge_dot__dipole(self, other), scalar__anti_reversal(self));
}

FlatPoint scalar__sandwich__flat_point(Scalar self, FlatPoint other) {
    return dipole__into__flat_point(circle__anti_wedge_dot__scalar(scalar__anti_wedge_dot__flat_point(self, other), scalar__anti_reversal(self)));
}

Flector scalar__sandwich__flector(Scalar self, Flector other) {
    return multi_vector__into__flector(multi_vector__anti_wedge_dot__scalar(scalar__anti_wedge_dot__flector(self, other), scalar__anti_reversal(self)));
}

Line scalar__sandwich__line(Scalar self, Line other) {
    return circle__into__line(dipole__anti_wedge_dot__scalar(scalar__anti_wedge_dot__line(self, other), scalar__anti_reversal(self)));
}

Motor scalar__sandwich__motor(Scalar self, Motor other) {
    return multi_vector__into__motor(multi_vector__anti_wedge_dot__scalar(scalar__anti_wedge_dot__motor(self, other), scalar__anti_reversal(self)));
}

MultiVector scalar__sandwich__multi_vector(Scalar self, MultiVector other) {
    return multi_vector__anti_wedge_dot__scalar(scalar__anti_wedge_dot__multi_vector(self, other), scalar__anti_reversal(self));
}

Plane scalar__sandwich__plane(Scalar self, Plane other) {
    return sphere__into__plane(round_point__anti_wedge_dot__scalar(scalar__anti_wedge_dot__plane(self, other), scalar__anti_reversal(self)));
}

RoundPoint scalar__sandwich__round_point(Scalar self, RoundPoint other) {
    return sphere__anti_wedge_dot__scalar(scalar__anti_wedge_dot__round_point(self, other), scalar__anti_reversal(self));
}

Sphere scalar__sandwich__sphere(Scalar self, Sphere other) {
    return round_point__anti_wedge_dot__scalar(scalar__anti_wedge_dot__sphere(self, other), scalar__anti_reversal(self));
}

Circle sphere__sandwich__circle(Sphere self, Circle other) {
    return multi_vector__into__circle(multi_vector__anti_wedge_dot__sphere(sphere__anti_wedge_dot__circle(self, other), sphere__anti_reversal(self)));
}

Dipole sphere__sandwich__dipole(Sphere self, Dipole other) {
    return multi_vector__into__dipole(multi_vector__anti_wedge_dot__sphere(sphere__anti_wedge_dot__dipole(self, other), sphere__anti_reversal(self)));
}

FlatPoint sphere__sandwich__flat_point(Sphere self, FlatPoint other) {
    return multi_vector__into__flat_point(multi_vector__anti_wedge_dot__sphere(sphere__anti_wedge_dot__flat_point(self, other), sphere__anti_reversal(self)));
}

Flector sphere__sandwich__flector(Sphere self, Flector other) {
    return multi_vector__into__flector(multi_vector__anti_wedge_dot__sphere(sphere__anti_wedge_dot__flector(self, other), sphere__anti_reversal(self)));
}

Line sphere__sandwich__line(Sphere self, Line other) {
    return multi_vector__into__line(multi_vector__anti_wedge_dot__sphere(sphere__anti_wedge_dot__line(self, other), sphere__anti_reversal(self)));
}

Motor sphere__sandwich__motor(Sphere self, Motor other) {
    return multi_vector__into__motor(multi_vector__anti_wedge_dot__sphere(sphere__anti_wedge_dot__motor(self, other), sphere__anti_reversal(self)));
}

MultiVector sphere__sandwich__multi_vector(Sphere self, MultiVector other) {
    return multi_vector__anti_wedge_dot__sphere(sphere__anti_wedge_dot__multi_vector(self, other), sphere__anti_reversal(self));
}

Plane sphere__sandwich__plane(Sphere self, Plane other) {
    return multi_vector__into__plane(multi_vector__anti_wedge_dot__sphere(sphere__anti_wedge_dot__plane(self, other), sphere__anti_reversal(self)));
}

RoundPoint sphere__sandwich__round_point(Sphere self, RoundPoint other) {
    return multi_vector__into__round_point(multi_vector__anti_wedge_dot__sphere(sphere__anti_wedge_dot__round_point(self, other), sphere__anti_reversal(self)));
}

Sphere sphere__sandwich__sphere(Sphere self, Sphere other) {
    return multi_vector__into__sphere(multi_vector__anti_wedge_dot__sphere(sphere__anti_wedge_dot__sphere(self, other), sphere__anti_reversal(self)));
}

AntiScalar anti_scalar__geometric_anti_quotient__anti_scalar(AntiScalar self, AntiScalar other) {
    return anti_scalar__anti_wedge_dot__anti_scalar(self, anti_scalar__anti_inverse(other));
}

Circle anti_scalar__geometric_anti_quotient__circle(AntiScalar self, Circle other) {
    return anti_scalar__anti_wedge_dot__circle(self, circle__anti_inverse(other));
}

Dipole anti_scalar__geometric_anti_quotient__dipole(AntiScalar self, Dipole other) {
    return anti_scalar__anti_wedge_dot__dipole(self, dipole__anti_inverse(other));
}

DualNum anti_scalar__geometric_anti_quotient__dual_num(AntiScalar self, DualNum other) {
    return anti_scalar__anti_wedge_dot__dual_num(self, dual_num__anti_inverse(other));
}

FlatPoint anti_scalar__geometric_anti_quotient__flat_point(AntiScalar self, FlatPoint other) {
    return anti_scalar__anti_wedge_dot__flat_point(self, flat_point__anti_inverse(other));
}

Flector anti_scalar__geometric_anti_quotient__flector(AntiScalar self, Flector other) {
    return anti_scalar__anti_wedge_dot__flector(self, flector__anti_inverse(other));
}

Line anti_scalar__geometric_anti_quotient__line(AntiScalar self, Line other) {
    return anti_scalar__anti_wedge_dot__line(self, line__anti_inverse(other));
}

Motor anti_scalar__geometric_anti_quotient__motor(AntiScalar self, Motor other) {
    return anti_scalar__anti_wedge_dot__motor(self, motor__anti_inverse(other));
}

MultiVector anti_scalar__geometric_anti_quotient__multi_vector(AntiScalar self, MultiVector other) {
    return anti_scalar__anti_wedge_dot__multi_vector(self, multi_vector__anti_inverse(other));
}

Plane anti_scalar__geometric_anti_quotient__plane(AntiScalar self, Plane other) {
    return anti_scalar__anti_wedge_dot__plane(self, plane__anti_inverse(other));
}

RoundPoint anti_scalar__geometric_anti_quotient__round_point(AntiScalar self, RoundPoint other) {
    return anti_scalar__anti_wedge_dot__round_point(self, round_point__anti_inverse(other));
}

Scalar anti_scalar__geometric_anti_quotient__scalar(AntiScalar self, Scalar other) {
    return anti_scalar__anti_wedge_dot__scalar(self, scalar__anti_inverse(other));
}

Sphere anti_scalar__geometric_anti_quotient__sphere(AntiScalar self, Sphere other) {
    return anti_scalar__anti_wedge_dot__sphere(self, sphere__anti_inverse(other));
}

Circle circle__geometric_anti_quotient__anti_scalar(Circle self, AntiScalar other) {
    return circle__anti_wedge_dot__anti_scalar(self, anti_scalar__anti_inverse(other));
}

MultiVector circle__geometric_anti_quotient__circle(Circle self, Circle other) {
    return circle__anti_wedge_dot__circle(self, circle__anti_inverse(other));
}

MultiVector circle__geometric_anti_quotient__dipole(Circle self, Dipole other) {
    return circle__anti_wedge_dot__dipole(self, dipole__anti_inverse(other));
}

MultiVector circle__geometric_anti_quotient__dual_num(Circle self, DualNum other) {
    return circle__anti_wedge_dot__dual_num(self, dual_num__anti_inverse(other));
}

MultiVector circle__geometric_anti_quotient__flat_point(Circle self, FlatPoint other) {
    return circle__anti_wedge_dot__flat_point(self, flat_point__anti_inverse(other));
}

MultiVector circle__geometric_anti_quotient__flector(Circle self, Flector other) {
    return circle__anti_wedge_dot__flector(self, flector__anti_inverse(other));
}

MultiVector circle__geometric_anti_quotient__line(Circle self, Line other) {
    return circle__anti_wedge_dot__line(self, line__anti_inverse(other));
}

MultiVector circle__geometric_anti_quotient__motor(Circle self, Motor other) {
    return circle__anti_wedge_dot__motor(self, motor__anti_inverse(other));
}

MultiVector circle__geometric_anti_quotient__multi_vector(Circle self, MultiVector other) {
    return circle__anti_wedge_dot__multi_vector(self, multi_vector__anti_inverse(other));
}

MultiVector circle__geometric_anti_quotient__plane(Circle self, Plane other) {
    return circle__anti_wedge_dot__plane(self, plane__anti_inverse(other));
}

MultiVector circle__geometric_anti_quotient__round_point(Circle self, RoundPoint other) {
    return circle__anti_wedge_dot__round_point(self, round_point__anti_inverse(other));
}

Dipole circle__geometric_anti_quotient__scalar(Circle self, Scalar other) {
    return circle__anti_wedge_dot__scalar(self, scalar__anti_inverse(other));
}

MultiVector circle__geometric_anti_quotient__sphere(Circle self, Sphere other) {
    return circle__anti_wedge_dot__sphere(self, sphere__anti_inverse(other));
}

Dipole dipole__geometric_anti_quotient__anti_scalar(Dipole self, AntiScalar other) {
    return dipole__anti_wedge_dot__anti_scalar(self, anti_scalar__anti_inverse(other));
}

MultiVector dipole__geometric_anti_quotient__circle(Dipole self, Circle other) {
    return dipole__anti_wedge_dot__circle(self, circle__anti_inverse(other));
}

MultiVector dipole__geometric_anti_quotient__dipole(Dipole self, Dipole other) {
    return dipole__anti_wedge_dot__dipole(self, dipole__anti_inverse(other));
}

MultiVector dipole__geometric_anti_quotient__dual_num(Dipole self, DualNum other) {
    return dipole__anti_wedge_dot__dual_num(self, dual_num__anti_inverse(other));
}

MultiVector dipole__geometric_anti_quotient__flat_point(Dipole self, FlatPoint other) {
    return dipole__anti_wedge_dot__flat_point(self, flat_point__anti_inverse(other));
}

MultiVector dipole__geometric_anti_quotient__flector(Dipole self, Flector other) {
    return dipole__anti_wedge_dot__flector(self, flector__anti_inverse(other));
}

MultiVector dipole__geometric_anti_quotient__line(Dipole self, Line other) {
    return dipole__anti_wedge_dot__line(self, line__anti_inverse(other));
}

MultiVector dipole__geometric_anti_quotient__motor(Dipole self, Motor other) {
    return dipole__anti_wedge_dot__motor(self, motor__anti_inverse(other));
}

MultiVector dipole__geometric_anti_quotient__multi_vector(Dipole self, MultiVector other) {
    return dipole__anti_wedge_dot__multi_vector(self, multi_vector__anti_inverse(other));
}

MultiVector dipole__geometric_anti_quotient__plane(Dipole self, Plane other) {
    return dipole__anti_wedge_dot__plane(self, plane__anti_inverse(other));
}

MultiVector dipole__geometric_anti_quotient__round_point(Dipole self, RoundPoint other) {
    return dipole__anti_wedge_dot__round_point(self, round_point__anti_inverse(other));
}

Circle dipole__geometric_anti_quotient__scalar(Dipole self, Scalar other) {
    return dipole__anti_wedge_dot__scalar(self, scalar__anti_inverse(other));
}

MultiVector dipole__geometric_anti_quotient__sphere(Dipole self, Sphere other) {
    return dipole__anti_wedge_dot__sphere(self, sphere__anti_inverse(other));
}

DualNum dual_num__geometric_anti_quotient__anti_scalar(DualNum self, AntiScalar other) {
    return dual_num__anti_wedge_dot__anti_scalar(self, anti_scalar__anti_inverse(other));
}

MultiVector dual_num__geometric_anti_quotient__circle(DualNum self, Circle other) {
    return dual_num__anti_wedge_dot__circle(self, circle__anti_inverse(other));
}

MultiVector dual_num__geometric_anti_quotient__dipole(DualNum self, Dipole other) {
    return dual_num__anti_wedge_dot__dipole(self, dipole__anti_inverse(other));
}

DualNum dual_num__geometric_anti_quotient__dual_num(DualNum self, DualNum other) {
    return dual_num__anti_wedge_dot__dual_num(self, dual_num__anti_inverse(other));
}

MultiVector dual_num__geometric_anti_quotient__flat_point(DualNum self, FlatPoint other) {
    return dual_num__anti_wedge_dot__flat_point(self, flat_point__anti_inverse(other));
}

MultiVector dual_num__geometric_anti_quotient__flector(DualNum self, Flector other) {
    return dual_num__anti_wedge_dot__flector(self, flector__anti_inverse(other));
}

MultiVector dual_num__geometric_anti_quotient__line(DualNum self, Line other) {
    return dual_num__anti_wedge_dot__line(self, line__anti_inverse(other));
}

MultiVector dual_num__geometric_anti_quotient__motor(DualNum self, Motor other) {
    return dual_num__anti_wedge_dot__motor(self, motor__anti_inverse(other));
}

MultiVector dual_num__geometric_anti_quotient__multi_vector(DualNum self, MultiVector other) {
    return dual_num__anti_wedge_dot__multi_vector(self, multi_vector__anti_inverse(other));
}

MultiVector dual_num__geometric_anti_quotient__plane(DualNum self, Plane other) {
    return dual_num__anti_wedge_dot__plane(self, plane__anti_inverse(other));
}

MultiVector dual_num__geometric_anti_quotient__round_point(DualNum self, RoundPoint other) {
    return dual_num__anti_wedge_dot__round_point(self, round_point__anti_inverse(other));
}

DualNum dual_num__geometric_anti_quotient__scalar(DualNum self, Scalar other) {
    return dual_num__anti_wedge_dot__scalar(self, scalar__anti_inverse(other));
}

MultiVector dual_num__geometric_anti_quotient__sphere(DualNum self, Sphere other) {
    return dual_num__anti_wedge_dot__sphere(self, sphere__anti_inverse(other));
}

FlatPoint flat_point__geometric_anti_quotient__anti_scalar(FlatPoint self, AntiScalar other) {
    return flat_point__anti_wedge_dot__anti_scalar(self, anti_scalar__anti_inverse(other));
}

MultiVector flat_point__geometric_anti_quotient__circle(FlatPoint self, Circle other) {
    return flat_point__anti_wedge_dot__circle(self, circle__anti_inverse(other));
}

MultiVector flat_point__geometric_anti_quotient__dipole(FlatPoint self, Dipole other) {
    return flat_point__anti_wedge_dot__dipole(self, dipole__anti_inverse(other));
}

MultiVector flat_point__geometric_anti_quotient__dual_num(FlatPoint self, DualNum other) {
    return flat_point__anti_wedge_dot__dual_num(self, dual_num__anti_inverse(other));
}

Motor flat_point__geometric_anti_quotient__flat_point(FlatPoint self, FlatPoint other) {
    return flat_point__anti_wedge_dot__flat_point(self, flat_point__anti_inverse(other));
}

MultiVector flat_point__geometric_anti_quotient__flector(FlatPoint self, Flector other) {
    return flat_point__anti_wedge_dot__flector(self, flector__anti_inverse(other));
}

Flector flat_point__geometric_anti_quotient__line(FlatPoint self, Line other) {
    return flat_point__anti_wedge_dot__line(self, line__anti_inverse(other));
}

Flector flat_point__geometric_anti_quotient__motor(FlatPoint self, Motor other) {
    return flat_point__anti_wedge_dot__motor(self, motor__anti_inverse(other));
}

MultiVector flat_point__geometric_anti_quotient__multi_vector(FlatPoint self, MultiVector other) {
    return flat_point__anti_wedge_dot__multi_vector(self, multi_vector__anti_inverse(other));
}

MultiVector flat_point__geometric_anti_quotient__plane(FlatPoint self, Plane other) {
    return flat_point__anti_wedge_dot__plane(self, plane__anti_inverse(other));
}

MultiVector flat_point__geometric_anti_quotient__round_point(FlatPoint self, RoundPoint other) {
    return flat_point__anti_wedge_dot__round_point(self, round_point__anti_inverse(other));
}

Circle flat_point__geometric_anti_quotient__scalar(FlatPoint self, Scalar other) {
    return flat_point__anti_wedge_dot__scalar(self, scalar__anti_inverse(other));
}

MultiVector flat_point__geometric_anti_quotient__sphere(FlatPoint self, Sphere other) {
    return flat_point__anti_wedge_dot__sphere(self, sphere__anti_inverse(other));
}

Flector flector__geometric_anti_quotient__anti_scalar(Flector self, AntiScalar other) {
    return flector__anti_wedge_dot__anti_scalar(self, anti_scalar__anti_inverse(other));
}

MultiVector flector__geometric_anti_quotient__circle(Flector self, Circle other) {
    return flector__anti_wedge_dot__circle(self, circle__anti_inverse(other));
}

MultiVector flector__geometric_anti_quotient__dipole(Flector self, Dipole other) {
    return flector__anti_wedge_dot__dipole(self, dipole__anti_inverse(other));
}

MultiVector flector__geometric_anti_quotient__dual_num(Flector self, DualNum other) {
    return flector__anti_wedge_dot__dual_num(self, dual_num__anti_inverse(other));
}

MultiVector flector__geometric_anti_quotient__flat_point(Flector self, FlatPoint other) {
    return flector__anti_wedge_dot__flat_point(self, flat_point__anti_inverse(other));
}

MultiVector flector__geometric_anti_quotient__flector(Flector self, Flector other) {
    return flector__anti_wedge_dot__flector(self, flector__anti_inverse(other));
}

Flector flector__geometric_anti_quotient__line(Flector self, Line other) {
    return flector__anti_wedge_dot__line(self, line__anti_inverse(other));
}

Flector flector__geometric_anti_quotient__motor(Flector self, Motor other) {
    return flector__anti_wedge_dot__motor(self, motor__anti_inverse(other));
}

MultiVector flector__geometric_anti_quotient__multi_vector(Flector self, MultiVector other) {
    return flector__anti_wedge_dot__multi_vector(self, multi_vector__anti_inverse(other));
}

MultiVector flector__geometric_anti_quotient__plane(Flector self, Plane other) {
    return flector__anti_wedge_dot__plane(self, plane__anti_inverse(other));
}

MultiVector flector__geometric_anti_quotient__round_point(Flector self, RoundPoint other) {
    return flector__anti_wedge_dot__round_point(self, round_point__anti_inverse(other));
}

MultiVector flector__geometric_anti_quotient__scalar(Flector self, Scalar other) {
    return flector__anti_wedge_dot__scalar(self, scalar__anti_inverse(other));
}

MultiVector flector__geometric_anti_quotient__sphere(Flector self, Sphere other) {
    return flector__anti_wedge_dot__sphere(self, sphere__anti_inverse(other));
}

Line line__geometric_anti_quotient__anti_scalar(Line self, AntiScalar other) {
    return line__anti_wedge_dot__anti_scalar(self, anti_scalar__anti_inverse(other));
}

MultiVector line__geometric_anti_quotient__circle(Line self, Circle other) {
    return line__anti_wedge_dot__circle(self, circle__anti_inverse(other));
}

MultiVector line__geometric_anti_quotient__dipole(Line self, Dipole other) {
    return line__anti_wedge_dot__dipole(self, dipole__anti_inverse(other));
}

MultiVector line__geometric_anti_quotient__dual_num(Line self, DualNum other) {
    return line__anti_wedge_dot__dual_num(self, dual_num__anti_inverse(other));
}

Flector line__geometric_anti_quotient__flat_point(Line self, FlatPoint other) {
    return line__anti_wedge_dot__flat_point(self, flat_point__anti_inverse(other));
}

Flector line__geometric_anti_quotient__flector(Line self, Flector other) {
    return line__anti_wedge_dot__flector(self, flector__anti_inverse(other));
}

MultiVector line__geometric_anti_quotient__line(Line self, Line other) {
    return line__anti_wedge_dot__line(self, line__anti_inverse(other));
}

MultiVector line__geometric_anti_quotient__motor(Line self, Motor other) {
    return line__anti_wedge_dot__motor(self, motor__anti_inverse(other));
}

MultiVector line__geometric_anti_quotient__multi_vector(Line self, MultiVector other) {
    return line__anti_wedge_dot__multi_vector(self, multi_vector__anti_inverse(other));
}

Flector line__geometric_anti_quotient__plane(Line self, Plane other) {
    return line__anti_wedge_dot__plane(self, plane__anti_inverse(other));
}

MultiVector line__geometric_anti_quotient__round_point(Line self, RoundPoint other) {
    return line__anti_wedge_dot__round_point(self, round_point__anti_inverse(other));
}

Dipole line__geometric_anti_quotient__scalar(Line self, Scalar other) {
    return line__anti_wedge_dot__scalar(self, scalar__anti_inverse(other));
}

MultiVector line__geometric_anti_quotient__sphere(Line self, Sphere other) {
    return line__anti_wedge_dot__sphere(self, sphere__anti_inverse(other));
}

Motor motor__geometric_anti_quotient__anti_scalar(Motor self, AntiScalar other) {
    return motor__anti_wedge_dot__anti_scalar(self, anti_scalar__anti_inverse(other));
}

MultiVector motor__geometric_anti_quotient__circle(Motor self, Circle other) {
    return motor__anti_wedge_dot__circle(self, circle__anti_inverse(other));
}

MultiVector motor__geometric_anti_quotient__dipole(Motor self, Dipole other) {
    return motor__anti_wedge_dot__dipole(self, dipole__anti_inverse(other));
}

MultiVector motor__geometric_anti_quotient__dual_num(Motor self, DualNum other) {
    return motor__anti_wedge_dot__dual_num(self, dual_num__anti_inverse(other));
}

Flector motor__geometric_anti_quotient__flat_point(Motor self, FlatPoint other) {
    return motor__anti_wedge_dot__flat_point(self, flat_point__anti_inverse(other));
}

Flector motor__geometric_anti_quotient__flector(Motor self, Flector other) {
    return motor__anti_wedge_dot__flector(self, flector__anti_inverse(other));
}

MultiVector motor__geometric_anti_quotient__line(Motor self, Line other) {
    return motor__anti_wedge_dot__line(self, line__anti_inverse(other));
}

MultiVector motor__geometric_anti_quotient__motor(Motor self, Motor other) {
    return motor__anti_wedge_dot__motor(self, motor__anti_inverse(other));
}

MultiVector motor__geometric_anti_quotient__multi_vector(Motor self, MultiVector other) {
    return motor__anti_wedge_dot__multi_vector(self, multi_vector__anti_inverse(other));
}

Flector motor__geometric_anti_quotient__plane(Motor self, Plane other) {
    return motor__anti_wedge_dot__plane(self, plane__anti_inverse(other));
}

MultiVector motor__geometric_anti_quotient__round_point(Motor self, RoundPoint other) {
    return motor__anti_wedge_dot__round_point(self, round_point__anti_inverse(other));
}

MultiVector motor__geometric_anti_quotient__scalar(Motor self, Scalar other) {
    return motor__anti_wedge_dot__scalar(self, scalar__anti_inverse(other));
}

MultiVector motor__geometric_anti_quotient__sphere(Motor self, Sphere other) {
    return motor__anti_wedge_dot__sphere(self, sphere__anti_inverse(other));
}

MultiVector multi_vector__geometric_anti_quotient__anti_scalar(MultiVector self, AntiScalar other) {
    return multi_vector__anti_wedge_dot__anti_scalar(self, anti_scalar__anti_inverse(other));
}

MultiVector multi_vector__geometric_anti_quotient__circle(MultiVector self, Circle other) {
    return multi_vector__anti_wedge_dot__circle(self, circle__anti_inverse(other));
}

MultiVector multi_vector__geometric_anti_quotient__dipole(MultiVector self, Dipole other) {
    return multi_vector__anti_wedge_dot__dipole(self, dipole__anti_inverse(other));
}

MultiVector multi_vector__geometric_anti_quotient__dual_num(MultiVector self, DualNum other) {
    return multi_vector__anti_wedge_dot__dual_num(self, dual_num__anti_inverse(other));
}

MultiVector multi_vector__geometric_anti_quotient__flat_point(MultiVector self, FlatPoint other) {
    return multi_vector__anti_wedge_dot__flat_point(self, flat_point__anti_inverse(other));
}

MultiVector multi_vector__geometric_anti_quotient__flector(MultiVector self, Flector other) {
    return multi_vector__anti_wedge_dot__flector(self, flector__anti_inverse(other));
}

MultiVector multi_vector__geometric_anti_quotient__line(MultiVector self, Line other) {
    return multi_vector__anti_wedge_dot__line(self, line__anti_inverse(other));
}

MultiVector multi_vector__geometric_anti_quotient__motor(MultiVector self, Motor other) {
    return multi_vector__anti_wedge_dot__motor(self, motor__anti_inverse(other));
}

MultiVector multi_vector__geometric_anti_quotient__multi_vector(MultiVector self, MultiVector other) {
    return multi_vector__anti_wedge_dot__multi_vector(self, multi_vector__anti_inverse(other));
}

MultiVector multi_vector__geometric_anti_quotient__plane(MultiVector self, Plane other) {
    return multi_vector__anti_wedge_dot__plane(self, plane__anti_inverse(other));
}

MultiVector multi_vector__geometric_anti_quotient__round_point(MultiVector self, RoundPoint other) {
    return multi_vector__anti_wedge_dot__round_point(self, round_point__anti_inverse(other));
}

MultiVector multi_vector__geometric_anti_quotient__scalar(MultiVector self, Scalar other) {
    return multi_vector__anti_wedge_dot__scalar(self, scalar__anti_inverse(other));
}

MultiVector multi_vector__geometric_anti_quotient__sphere(MultiVector self, Sphere other) {
    return multi_vector__anti_wedge_dot__sphere(self, sphere__anti_inverse(other));
}

Plane plane__geometric_anti_quotient__anti_scalar(Plane self, AntiScalar other) {
    return plane__anti_wedge_dot__anti_scalar(self, anti_scalar__anti_inverse(other));
}

MultiVector plane__geometric_anti_quotient__circle(Plane self, Circle other) {
    return plane__anti_wedge_dot__circle(self, circle__anti_inverse(other));
}

MultiVector plane__geometric_anti_quotient__dipole(Plane self, Dipole other) {
    return plane__anti_wedge_dot__dipole(self, dipole__anti_inverse(other));
}

MultiVector plane__geometric_anti_quotient__dual_num(Plane self, DualNum other) {
    return plane__anti_wedge_dot__dual_num(self, dual_num__anti_inverse(other));
}

MultiVector plane__geometric_anti_quotient__flat_point(Plane self, FlatPoint other) {
    return plane__anti_wedge_dot__flat_point(self, flat_point__anti_inverse(other));
}

MultiVector plane__geometric_anti_quotient__flector(Plane self, Flector other) {
    return plane__anti_wedge_dot__flector(self, flector__anti_inverse(other));
}

Flector plane__geometric_anti_quotient__line(Plane self, Line other) {
    return plane__anti_wedge_dot__line(self, line__anti_inverse(other));
}

Flector plane__geometric_anti_quotient__motor(Plane self, Motor other) {
    return plane__anti_wedge_dot__motor(self, motor__anti_inverse(other));
}

MultiVector plane__geometric_anti_quotient__multi_vector(Plane self, MultiVector other) {
    return plane__anti_wedge_dot__multi_vector(self, multi_vector__anti_inverse(other));
}

Motor plane__geometric_anti_quotient__plane(Plane self, Plane other) {
    return plane__anti_wedge_dot__plane(self, plane__anti_inverse(other));
}

MultiVector plane__geometric_anti_quotient__round_point(Plane self, RoundPoint other) {
    return plane__anti_wedge_dot__round_point(self, round_point__anti_inverse(other));
}

RoundPoint plane__geometric_anti_quotient__scalar(Plane self, Scalar other) {
    return plane__anti_wedge_dot__scalar(self, scalar__anti_inverse(other));
}

MultiVector plane__geometric_anti_quotient__sphere(Plane self, Sphere other) {
    return plane__anti_wedge_dot__sphere(self, sphere__anti_inverse(other));
}

RoundPoint round_point__geometric_anti_quotient__anti_scalar(RoundPoint self, AntiScalar other) {
    return round_point__anti_wedge_dot__anti_scalar(self, anti_scalar__anti_inverse(other));
}

MultiVector round_point__geometric_anti_quotient__circle(RoundPoint self, Circle other) {
    return round_point__anti_wedge_dot__circle(self, circle__anti_inverse(other));
}

MultiVector round_point__geometric_anti_quotient__dipole(RoundPoint self, Dipole other) {
    return round_point__anti_wedge_dot__dipole(self, dipole__anti_inverse(other));
}

MultiVector round_point__geometric_anti_quotient__dual_num(RoundPoint self, DualNum other) {
    return round_point__anti_wedge_dot__dual_num(self, dual_num__anti_inverse(other));
}

MultiVector round_point__geometric_anti_quotient__flat_point(RoundPoint self, FlatPoint other) {
    return round_point__anti_wedge_dot__flat_point(self, flat_point__anti_inverse(other));
}

MultiVector round_point__geometric_anti_quotient__flector(RoundPoint self, Flector other) {
    return round_point__anti_wedge_dot__flector(self, flector__anti_inverse(other));
}

MultiVector round_point__geometric_anti_quotient__line(RoundPoint self, Line other) {
    return round_point__anti_wedge_dot__line(self, line__anti_inverse(other));
}

MultiVector round_point__geometric_anti_quotient__motor(RoundPoint self, Motor other) {
    return round_point__anti_wedge_dot__motor(self, motor__anti_inverse(other));
}

MultiVector round_point__geometric_anti_quotient__multi_vector(RoundPoint self, MultiVector other) {
    return round_point__anti_wedge_dot__multi_vector(self, multi_vector__anti_inverse(other));
}

MultiVector round_point__geometric_anti_quotient__plane(RoundPoint self, Plane other) {
    return round_point__anti_wedge_dot__plane(self, plane__anti_inverse(other));
}

MultiVector round_point__geometric_anti_quotient__round_point(RoundPoint self, RoundPoint other) {
    return round_point__anti_wedge_dot__round_point(self, round_point__anti_inverse(other));
}

Sphere round_point__geometric_anti_quotient__scalar(RoundPoint self, Scalar other) {
    return round_point__anti_wedge_dot__scalar(self, scalar__anti_inverse(other));
}

MultiVector round_point__geometric_anti_quotient__sphere(RoundPoint self, Sphere other) {
    return round_point__anti_wedge_dot__sphere(self, sphere__anti_inverse(other));
}

Scalar scalar__geometric_anti_quotient__anti_scalar(Scalar self, AntiScalar other) {
    return scalar__anti_wedge_dot__anti_scalar(self, anti_scalar__anti_inverse(other));
}

Dipole scalar__geometric_anti_quotient__circle(Scalar self, Circle other) {
    return scalar__anti_wedge_dot__circle(self, circle__anti_inverse(other));
}

Circle scalar__geometric_anti_quotient__dipole(Scalar self, Dipole other) {
    return scalar__anti_wedge_dot__dipole(self, dipole__anti_inverse(other));
}

DualNum scalar__geometric_anti_quotient__dual_num(Scalar self, DualNum other) {
    return scalar__anti_wedge_dot__dual_num(self, dual_num__anti_inverse(other));
}

Circle scalar__geometric_anti_quotient__flat_point(Scalar self, FlatPoint other) {
    return scalar__anti_wedge_dot__flat_point(self, flat_point__anti_inverse(other));
}

MultiVector scalar__geometric_anti_quotient__flector(Scalar self, Flector other) {
    return scalar__anti_wedge_dot__flector(self, flector__anti_inverse(other));
}

Dipole scalar__geometric_anti_quotient__line(Scalar self, Line other) {
    return scalar__anti_wedge_dot__line(self, line__anti_inverse(other));
}

MultiVector scalar__geometric_anti_quotient__motor(Scalar self, Motor other) {
    return scalar__anti_wedge_dot__motor(self, motor__anti_inverse(other));
}

MultiVector scalar__geometric_anti_quotient__multi_vector(Scalar self, MultiVector other) {
    return scalar__anti_wedge_dot__multi_vector(self, multi_vector__anti_inverse(other));
}

RoundPoint scalar__geometric_anti_quotient__plane(Scalar self, Plane other) {
    return scalar__anti_wedge_dot__plane(self, plane__anti_inverse(other));
}

Sphere scalar__geometric_anti_quotient__round_point(Scalar self, RoundPoint other) {
    return scalar__anti_wedge_dot__round_point(self, round_point__anti_inverse(other));
}

AntiScalar scalar__geometric_anti_quotient__scalar(Scalar self, Scalar other) {
    return scalar__anti_wedge_dot__scalar(self, scalar__anti_inverse(other));
}

RoundPoint scalar__geometric_anti_quotient__sphere(Scalar self, Sphere other) {
    return scalar__anti_wedge_dot__sphere(self, sphere__anti_inverse(other));
}

Sphere sphere__geometric_anti_quotient__anti_scalar(Sphere self, AntiScalar other) {
    return sphere__anti_wedge_dot__anti_scalar(self, anti_scalar__anti_inverse(other));
}

MultiVector sphere__geometric_anti_quotient__circle(Sphere self, Circle other) {
    return sphere__anti_wedge_dot__circle(self, circle__anti_inverse(other));
}

MultiVector sphere__geometric_anti_quotient__dipole(Sphere self, Dipole other) {
    return sphere__anti_wedge_dot__dipole(self, dipole__anti_inverse(other));
}

MultiVector sphere__geometric_anti_quotient__dual_num(Sphere self, DualNum other) {
    return sphere__anti_wedge_dot__dual_num(self, dual_num__anti_inverse(other));
}

MultiVector sphere__geometric_anti_quotient__flat_point(Sphere self, FlatPoint other) {
    return sphere__anti_wedge_dot__flat_point(self, flat_point__anti_inverse(other));
}

MultiVector sphere__geometric_anti_quotient__flector(Sphere self, Flector other) {
    return sphere__anti_wedge_dot__flector(self, flector__anti_inverse(other));
}

MultiVector sphere__geometric_anti_quotient__line(Sphere self, Line other) {
    return sphere__anti_wedge_dot__line(self, line__anti_inverse(other));
}

MultiVector sphere__geometric_anti_quotient__motor(Sphere self, Motor other) {
    return sphere__anti_wedge_dot__motor(self, motor__anti_inverse(other));
}

MultiVector sphere__geometric_anti_quotient__multi_vector(Sphere self, MultiVector other) {
    return sphere__anti_wedge_dot__multi_vector(self, multi_vector__anti_inverse(other));
}

MultiVector sphere__geometric_anti_quotient__plane(Sphere self, Plane other) {
    return sphere__anti_wedge_dot__plane(self, plane__anti_inverse(other));
}

MultiVector sphere__geometric_anti_quotient__round_point(Sphere self, RoundPoint other) {
    return sphere__anti_wedge_dot__round_point(self, round_point__anti_inverse(other));
}

RoundPoint sphere__geometric_anti_quotient__scalar(Sphere self, Scalar other) {
    return sphere__anti_wedge_dot__scalar(self, scalar__anti_inverse(other));
}

MultiVector sphere__geometric_anti_quotient__sphere(Sphere self, Sphere other) {
    return sphere__anti_wedge_dot__sphere(self, sphere__anti_inverse(other));
}

Scalar anti_scalar__geometric_quotient__anti_scalar(AntiScalar self, AntiScalar other) {
    return anti_scalar__wedge_dot__anti_scalar(self, anti_scalar__inverse(other));
}

Dipole anti_scalar__geometric_quotient__circle(AntiScalar self, Circle other) {
    return anti_scalar__wedge_dot__circle(self, circle__inverse(other));
}

Circle anti_scalar__geometric_quotient__dipole(AntiScalar self, Dipole other) {
    return anti_scalar__wedge_dot__dipole(self, dipole__inverse(other));
}

DualNum anti_scalar__geometric_quotient__dual_num(AntiScalar self, DualNum other) {
    return anti_scalar__wedge_dot__dual_num(self, dual_num__inverse(other));
}

Circle anti_scalar__geometric_quotient__flat_point(AntiScalar self, FlatPoint other) {
    return anti_scalar__wedge_dot__flat_point(self, flat_point__inverse(other));
}

MultiVector anti_scalar__geometric_quotient__flector(AntiScalar self, Flector other) {
    return anti_scalar__wedge_dot__flector(self, flector__inverse(other));
}

Dipole anti_scalar__geometric_quotient__line(AntiScalar self, Line other) {
    return anti_scalar__wedge_dot__line(self, line__inverse(other));
}

MultiVector anti_scalar__geometric_quotient__motor(AntiScalar self, Motor other) {
    return anti_scalar__wedge_dot__motor(self, motor__inverse(other));
}

MultiVector anti_scalar__geometric_quotient__multi_vector(AntiScalar self, MultiVector other) {
    return anti_scalar__wedge_dot__multi_vector(self, multi_vector__inverse(other));
}

RoundPoint anti_scalar__geometric_quotient__plane(AntiScalar self, Plane other) {
    return anti_scalar__wedge_dot__plane(self, plane__inverse(other));
}

Sphere anti_scalar__geometric_quotient__round_point(AntiScalar self, RoundPoint other) {
    return anti_scalar__wedge_dot__round_point(self, round_point__inverse(other));
}

AntiScalar anti_scalar__geometric_quotient__scalar(AntiScalar self, Scalar other) {
    return anti_scalar__wedge_dot__scalar(self, scalar__inverse(other));
}

RoundPoint anti_scalar__geometric_quotient__sphere(AntiScalar self, Sphere other) {
    return anti_scalar__wedge_dot__sphere(self, sphere__inverse(other));
}

Dipole circle__geometric_quotient__anti_scalar(Circle self, AntiScalar other) {
    return circle__wedge_dot__anti_scalar(self, anti_scalar__inverse(other));
}

MultiVector circle__geometric_quotient__circle(Circle self, Circle other) {
    return circle__wedge_dot__circle(self, circle__inverse(other));
}

MultiVector circle__geometric_quotient__dipole(Circle self, Dipole other) {
    return circle__wedge_dot__dipole(self, dipole__inverse(other));
}

MultiVector circle__geometric_quotient__dual_num(Circle self, DualNum other) {
    return circle__wedge_dot__dual_num(self, dual_num__inverse(other));
}

MultiVector circle__geometric_quotient__flat_point(Circle self, FlatPoint other) {
    return circle__wedge_dot__flat_point(self, flat_point__inverse(other));
}

MultiVector circle__geometric_quotient__flector(Circle self, Flector other) {
    return circle__wedge_dot__flector(self, flector__inverse(other));
}

MultiVector circle__geometric_quotient__line(Circle self, Line other) {
    return circle__wedge_dot__line(self, line__inverse(other));
}

MultiVector circle__geometric_quotient__motor(Circle self, Motor other) {
    return circle__wedge_dot__motor(self, motor__inverse(other));
}

MultiVector circle__geometric_quotient__multi_vector(Circle self, MultiVector other) {
    return circle__wedge_dot__multi_vector(self, multi_vector__inverse(other));
}

MultiVector circle__geometric_quotient__plane(Circle self, Plane other) {
    return circle__wedge_dot__plane(self, plane__inverse(other));
}

MultiVector circle__geometric_quotient__round_point(Circle self, RoundPoint other) {
    return circle__wedge_dot__round_point(self, round_point__inverse(other));
}

Circle circle__geometric_quotient__scalar(Circle self, Scalar other) {
    return circle__wedge_dot__scalar(self, scalar__inverse(other));
}

MultiVector circle__geometric_quotient__sphere(Circle self, Sphere other) {
    return circle__wedge_dot__sphere(self, sphere__inverse(other));
}

Circle dipole__geometric_quotient__anti_scalar(Dipole self, AntiScalar other) {
    return dipole__wedge_dot__anti_scalar(self, anti_scalar__inverse(other));
}

MultiVector dipole__geometric_quotient__circle(Dipole self, Circle other) {
    return dipole__wedge_dot__circle(self, circle__inverse(other));
}

MultiVector dipole__geometric_quotient__dipole(Dipole self, Dipole other) {
    return dipole__wedge_dot__dipole(self, dipole__inverse(other));
}

MultiVector dipole__geometric_quotient__dual_num(Dipole self, DualNum other) {
    return dipole__wedge_dot__dual_num(self, dual_num__inverse(other));
}

MultiVector dipole__geometric_quotient__flat_point(Dipole self, FlatPoint other) {
    return dipole__wedge_dot__flat_point(self, flat_point__inverse(other));
}

MultiVector dipole__geometric_quotient__flector(Dipole self, Flector other) {
    return dipole__wedge_dot__flector(self, flector__inverse(other));
}

MultiVector dipole__geometric_quotient__line(Dipole self, Line other) {
    return dipole__wedge_dot__line(self, line__inverse(other));
}

MultiVector dipole__geometric_quotient__motor(Dipole self, Motor other) {
    return dipole__wedge_dot__motor(self, motor__inverse(other));
}

MultiVector dipole__geometric_quotient__multi_vector(Dipole self, MultiVector other) {
    return dipole__wedge_dot__multi_vector(self, multi_vector__inverse(other));
}

MultiVector dipole__geometric_quotient__plane(Dipole self, Plane other) {
    return dipole__wedge_dot__plane(self, plane__inverse(other));
}

MultiVector dipole__geometric_quotient__round_point(Dipole self, RoundPoint other) {
    return dipole__wedge_dot__round_point(self, round_point__inverse(other));
}

Dipole dipole__geometric_quotient__scalar(Dipole self, Scalar other) {
    return dipole__wedge_dot__scalar(self, scalar__inverse(other));
}

MultiVector dipole__geometric_quotient__sphere(Dipole self, Sphere other) {
    return dipole__wedge_dot__sphere(self, sphere__inverse(other));
}

DualNum dual_num__geometric_quotient__anti_scalar(DualNum self, AntiScalar other) {
    return dual_num__wedge_dot__anti_scalar(self, anti_scalar__inverse(other));
}

MultiVector dual_num__geometric_quotient__circle(DualNum self, Circle other) {
    return dual_num__wedge_dot__circle(self, circle__inverse(other));
}

MultiVector dual_num__geometric_quotient__dipole(DualNum self, Dipole other) {
    return dual_num__wedge_dot__dipole(self, dipole__inverse(other));
}

DualNum dual_num__geometric_quotient__dual_num(DualNum self, DualNum other) {
    return dual_num__wedge_dot__dual_num(self, dual_num__inverse(other));
}

MultiVector dual_num__geometric_quotient__flat_point(DualNum self, FlatPoint other) {
    return dual_num__wedge_dot__flat_point(self, flat_point__inverse(other));
}

MultiVector dual_num__geometric_quotient__flector(DualNum self, Flector other) {
    return dual_num__wedge_dot__flector(self, flector__inverse(other));
}

MultiVector dual_num__geometric_quotient__line(DualNum self, Line other) {
    return dual_num__wedge_dot__line(self, line__inverse(other));
}

MultiVector dual_num__geometric_quotient__motor(DualNum self, Motor other) {
    return dual_num__wedge_dot__motor(self, motor__inverse(other));
}

MultiVector dual_num__geometric_quotient__multi_vector(DualNum self, MultiVector other) {
    return dual_num__wedge_dot__multi_vector(self, multi_vector__inverse(other));
}

MultiVector dual_num__geometric_quotient__plane(DualNum self, Plane other) {
    return dual_num__wedge_dot__plane(self, plane__inverse(other));
}

MultiVector dual_num__geometric_quotient__round_point(DualNum self, RoundPoint other) {
    return dual_num__wedge_dot__round_point(self, round_point__inverse(other));
}

DualNum dual_num__geometric_quotient__scalar(DualNum self, Scalar other) {
    return dual_num__wedge_dot__scalar(self, scalar__inverse(other));
}

MultiVector dual_num__geometric_quotient__sphere(DualNum self, Sphere other) {
    return dual_num__wedge_dot__sphere(self, sphere__inverse(other));
}

Circle flat_point__geometric_quotient__anti_scalar(FlatPoint self, AntiScalar other) {
    return flat_point__wedge_dot__anti_scalar(self, anti_scalar__inverse(other));
}

MultiVector flat_point__geometric_quotient__circle(FlatPoint self, Circle other) {
    return flat_point__wedge_dot__circle(self, circle__inverse(other));
}

MultiVector flat_point__geometric_quotient__dipole(FlatPoint self, Dipole other) {
    return flat_point__wedge_dot__dipole(self, dipole__inverse(other));
}

MultiVector flat_point__geometric_quotient__dual_num(FlatPoint self, DualNum other) {
    return flat_point__wedge_dot__dual_num(self, dual_num__inverse(other));
}

MultiVector flat_point__geometric_quotient__flat_point(FlatPoint self, FlatPoint other) {
    return flat_point__wedge_dot__flat_point(self, flat_point__inverse(other));
}

MultiVector flat_point__geometric_quotient__flector(FlatPoint self, Flector other) {
    return flat_point__wedge_dot__flector(self, flector__inverse(other));
}

MultiVector flat_point__geometric_quotient__line(FlatPoint self, Line other) {
    return flat_point__wedge_dot__line(self, line__inverse(other));
}

MultiVector flat_point__geometric_quotient__motor(FlatPoint self, Motor other) {
    return flat_point__wedge_dot__motor(self, motor__inverse(other));
}

MultiVector flat_point__geometric_quotient__multi_vector(FlatPoint self, MultiVector other) {
    return flat_point__wedge_dot__multi_vector(self, multi_vector__inverse(other));
}

MultiVector flat_point__geometric_quotient__plane(FlatPoint self, Plane other) {
    return flat_point__wedge_dot__plane(self, plane__inverse(other));
}

MultiVector flat_point__geometric_quotient__round_point(FlatPoint self, RoundPoint other) {
    return flat_point__wedge_dot__round_point(self, round_point__inverse(other));
}

FlatPoint flat_point__geometric_quotient__scalar(FlatPoint self, Scalar other) {
    return flat_point__wedge_dot__scalar(self, scalar__inverse(other));
}

MultiVector flat_point__geometric_quotient__sphere(FlatPoint self, Sphere other) {
    return flat_point__wedge_dot__sphere(self, sphere__inverse(other));
}

MultiVector flector__geometric_quotient__anti_scalar(Flector self, AntiScalar other) {
    return flector__wedge_dot__anti_scalar(self, anti_scalar__inverse(other));
}

MultiVector flector__geometric_quotient__circle(Flector self, Circle other) {
    return flector__wedge_dot__circle(self, circle__inverse(other));
}

MultiVector flector__geometric_quotient__dipole(Flector self, Dipole other) {
    return flector__wedge_dot__dipole(self, dipole__inverse(other));
}

MultiVector flector__geometric_quotient__dual_num(Flector self, DualNum other) {
    return flector__wedge_dot__dual_num(self, dual_num__inverse(other));
}

MultiVector flector__geometric_quotient__flat_point(Flector self, FlatPoint other) {
    return flector__wedge_dot__flat_point(self, flat_point__inverse(other));
}

MultiVector flector__geometric_quotient__flector(Flector self, Flector other) {
    return flector__wedge_dot__flector(self, flector__inverse(other));
}

MultiVector flector__geometric_quotient__line(Flector self, Line other) {
    return flector__wedge_dot__line(self, line__inverse(other));
}

MultiVector flector__geometric_quotient__motor(Flector self, Motor other) {
    return flector__wedge_dot__motor(self, motor__inverse(other));
}

MultiVector flector__geometric_quotient__multi_vector(Flector self, MultiVector other) {
    return flector__wedge_dot__multi_vector(self, multi_vector__inverse(other));
}

MultiVector flector__geometric_quotient__plane(Flector self, Plane other) {
    return flector__wedge_dot__plane(self, plane__inverse(other));
}

MultiVector flector__geometric_quotient__round_point(Flector self, RoundPoint other) {
    return flector__wedge_dot__round_point(self, round_point__inverse(other));
}

Flector flector__geometric_quotient__scalar(Flector self, Scalar other) {
    return flector__wedge_dot__scalar(self, scalar__inverse(other));
}

MultiVector flector__geometric_quotient__sphere(Flector self, Sphere other) {
    return flector__wedge_dot__sphere(self, sphere__inverse(other));
}

Dipole line__geometric_quotient__anti_scalar(Line self, AntiScalar other) {
    return line__wedge_dot__anti_scalar(self, anti_scalar__inverse(other));
}

MultiVector line__geometric_quotient__circle(Line self, Circle other) {
    return line__wedge_dot__circle(self, circle__inverse(other));
}

MultiVector line__geometric_quotient__dipole(Line self, Dipole other) {
    return line__wedge_dot__dipole(self, dipole__inverse(other));
}

MultiVector line__geometric_quotient__dual_num(Line self, DualNum other) {
    return line__wedge_dot__dual_num(self, dual_num__inverse(other));
}

MultiVector line__geometric_quotient__flat_point(Line self, FlatPoint other) {
    return line__wedge_dot__flat_point(self, flat_point__inverse(other));
}

MultiVector line__geometric_quotient__flector(Line self, Flector other) {
    return line__wedge_dot__flector(self, flector__inverse(other));
}

MultiVector line__geometric_quotient__line(Line self, Line other) {
    return line__wedge_dot__line(self, line__inverse(other));
}

MultiVector line__geometric_quotient__motor(Line self, Motor other) {
    return line__wedge_dot__motor(self, motor__inverse(other));
}

MultiVector line__geometric_quotient__multi_vector(Line self, MultiVector other) {
    return line__wedge_dot__multi_vector(self, multi_vector__inverse(other));
}

MultiVector line__geometric_quotient__plane(Line self, Plane other) {
    return line__wedge_dot__plane(self, plane__inverse(other));
}

MultiVector line__geometric_quotient__round_point(Line self, RoundPoint other) {
    return line__wedge_dot__round_point(self, round_point__inverse(other));
}

Line line__geometric_quotient__scalar(Line self, Scalar other) {
    return line__wedge_dot__scalar(self, scalar__inverse(other));
}

MultiVector line__geometric_quotient__sphere(Line self, Sphere other) {
    return line__wedge_dot__sphere(self, sphere__inverse(other));
}

MultiVector motor__geometric_quotient__anti_scalar(Motor self, AntiScalar other) {
    return motor__wedge_dot__anti_scalar(self, anti_scalar__inverse(other));
}

MultiVector motor__geometric_quotient__circle(Motor self, Circle other) {
    return motor__wedge_dot__circle(self, circle__inverse(other));
}

MultiVector motor__geometric_quotient__dipole(Motor self, Dipole other) {
    return motor__wedge_dot__dipole(self, dipole__inverse(other));
}

MultiVector motor__geometric_quotient__dual_num(Motor self, DualNum other) {
    return motor__wedge_dot__dual_num(self, dual_num__inverse(other));
}

MultiVector motor__geometric_quotient__flat_point(Motor self, FlatPoint other) {
    return motor__wedge_dot__flat_point(self, flat_point__inverse(other));
}

MultiVector motor__geometric_quotient__flector(Motor self, Flector other) {
    return motor__wedge_dot__flector(self, flector__inverse(other));
}

MultiVector motor__geometric_quotient__line(Motor self, Line other) {
    return motor__wedge_dot__line(self, line__inverse(other));
}

MultiVector motor__geometric_quotient__motor(Motor self, Motor other) {
    return motor__wedge_dot__motor(self, motor__inverse(other));
}

MultiVector motor__geometric_quotient__multi_vector(Motor self, MultiVector other) {
    return motor__wedge_dot__multi_vector(self, multi_vector__inverse(other));
}

MultiVector motor__geometric_quotient__plane(Motor self, Plane other) {
    return motor__wedge_dot__plane(self, plane__inverse(other));
}

MultiVector motor__geometric_quotient__round_point(Motor self, RoundPoint other) {
    return motor__wedge_dot__round_point(self, round_point__inverse(other));
}

Motor motor__geometric_quotient__scalar(Motor self, Scalar other) {
    return motor__wedge_dot__scalar(self, scalar__inverse(other));
}

MultiVector motor__geometric_quotient__sphere(Motor self, Sphere other) {
    return motor__wedge_dot__sphere(self, sphere__inverse(other));
}

MultiVector multi_vector__geometric_quotient__anti_scalar(MultiVector self, AntiScalar other) {
    return multi_vector__wedge_dot__anti_scalar(self, anti_scalar__inverse(other));
}

MultiVector multi_vector__geometric_quotient__circle(MultiVector self, Circle other) {
    return multi_vector__wedge_dot__circle(self, circle__inverse(other));
}

MultiVector multi_vector__geometric_quotient__dipole(MultiVector self, Dipole other) {
    return multi_vector__wedge_dot__dipole(self, dipole__inverse(other));
}

MultiVector multi_vector__geometric_quotient__dual_num(MultiVector self, DualNum other) {
    return multi_vector__wedge_dot__dual_num(self, dual_num__inverse(other));
}

MultiVector multi_vector__geometric_quotient__flat_point(MultiVector self, FlatPoint other) {
    return multi_vector__wedge_dot__flat_point(self, flat_point__inverse(other));
}

MultiVector multi_vector__geometric_quotient__flector(MultiVector self, Flector other) {
    return multi_vector__wedge_dot__flector(self, flector__inverse(other));
}

MultiVector multi_vector__geometric_quotient__line(MultiVector self, Line other) {
    return multi_vector__wedge_dot__line(self, line__inverse(other));
}

MultiVector multi_vector__geometric_quotient__motor(MultiVector self, Motor other) {
    return multi_vector__wedge_dot__motor(self, motor__inverse(other));
}

MultiVector multi_vector__geometric_quotient__multi_vector(MultiVector self, MultiVector other) {
    return multi_vector__wedge_dot__multi_vector(self, multi_vector__inverse(other));
}

MultiVector multi_vector__geometric_quotient__plane(MultiVector self, Plane other) {
    return multi_vector__wedge_dot__plane(self, plane__inverse(other));
}

MultiVector multi_vector__geometric_quotient__round_point(MultiVector self, RoundPoint other) {
    return multi_vector__wedge_dot__round_point(self, round_point__inverse(other));
}

MultiVector multi_vector__geometric_quotient__scalar(MultiVector self, Scalar other) {
    return multi_vector__wedge_dot__scalar(self, scalar__inverse(other));
}

MultiVector multi_vector__geometric_quotient__sphere(MultiVector self, Sphere other) {
    return multi_vector__wedge_dot__sphere(self, sphere__inverse(other));
}

RoundPoint plane__geometric_quotient__anti_scalar(Plane self, AntiScalar other) {
    return plane__wedge_dot__anti_scalar(self, anti_scalar__inverse(other));
}

MultiVector plane__geometric_quotient__circle(Plane self, Circle other) {
    return plane__wedge_dot__circle(self, circle__inverse(other));
}

MultiVector plane__geometric_quotient__dipole(Plane self, Dipole other) {
    return plane__wedge_dot__dipole(self, dipole__inverse(other));
}

MultiVector plane__geometric_quotient__dual_num(Plane self, DualNum other) {
    return plane__wedge_dot__dual_num(self, dual_num__inverse(other));
}

MultiVector plane__geometric_quotient__flat_point(Plane self, FlatPoint other) {
    return plane__wedge_dot__flat_point(self, flat_point__inverse(other));
}

MultiVector plane__geometric_quotient__flector(Plane self, Flector other) {
    return plane__wedge_dot__flector(self, flector__inverse(other));
}

MultiVector plane__geometric_quotient__line(Plane self, Line other) {
    return plane__wedge_dot__line(self, line__inverse(other));
}

MultiVector plane__geometric_quotient__motor(Plane self, Motor other) {
    return plane__wedge_dot__motor(self, motor__inverse(other));
}

MultiVector plane__geometric_quotient__multi_vector(Plane self, MultiVector other) {
    return plane__wedge_dot__multi_vector(self, multi_vector__inverse(other));
}

MultiVector plane__geometric_quotient__plane(Plane self, Plane other) {
    return plane__wedge_dot__plane(self, plane__inverse(other));
}

MultiVector plane__geometric_quotient__round_point(Plane self, RoundPoint other) {
    return plane__wedge_dot__round_point(self, round_point__inverse(other));
}

Plane plane__geometric_quotient__scalar(Plane self, Scalar other) {
    return plane__wedge_dot__scalar(self, scalar__inverse(other));
}

MultiVector plane__geometric_quotient__sphere(Plane self, Sphere other) {
    return plane__wedge_dot__sphere(self, sphere__inverse(other));
}

Sphere round_point__geometric_quotient__anti_scalar(RoundPoint self, AntiScalar other) {
    return round_point__wedge_dot__anti_scalar(self, anti_scalar__inverse(other));
}

MultiVector round_point__geometric_quotient__circle(RoundPoint self, Circle other) {
    return round_point__wedge_dot__circle(self, circle__inverse(other));
}

MultiVector round_point__geometric_quotient__dipole(RoundPoint self, Dipole other) {
    return round_point__wedge_dot__dipole(self, dipole__inverse(other));
}

MultiVector round_point__geometric_quotient__dual_num(RoundPoint self, DualNum other) {
    return round_point__wedge_dot__dual_num(self, dual_num__inverse(other));
}

MultiVector round_point__geometric_quotient__flat_point(RoundPoint self, FlatPoint other) {
    return round_point__wedge_dot__flat_point(self, flat_point__inverse(other));
}

MultiVector round_point__geometric_quotient__flector(RoundPoint self, Flector other) {
    return round_point__wedge_dot__flector(self, flector__inverse(other));
}

MultiVector round_point__geometric_quotient__line(RoundPoint self, Line other) {
    return round_point__wedge_dot__line(self, line__inverse(other));
}

MultiVector round_point__geometric_quotient__motor(RoundPoint self, Motor other) {
    return round_point__wedge_dot__motor(self, motor__inverse(other));
}

MultiVector round_point__geometric_quotient__multi_vector(RoundPoint self, MultiVector other) {
    return round_point__wedge_dot__multi_vector(self, multi_vector__inverse(other));
}

MultiVector round_point__geometric_quotient__plane(RoundPoint self, Plane other) {
    return round_point__wedge_dot__plane(self, plane__inverse(other));
}

MultiVector round_point__geometric_quotient__round_point(RoundPoint self, RoundPoint other) {
    return round_point__wedge_dot__round_point(self, round_point__inverse(other));
}

RoundPoint round_point__geometric_quotient__scalar(RoundPoint self, Scalar other) {
    return round_point__wedge_dot__scalar(self, scalar__inverse(other));
}

MultiVector round_point__geometric_quotient__sphere(RoundPoint self, Sphere other) {
    return round_point__wedge_dot__sphere(self, sphere__inverse(other));
}

AntiScalar scalar__geometric_quotient__anti_scalar(Scalar self, AntiScalar other) {
    return scalar__wedge_dot__anti_scalar(self, anti_scalar__inverse(other));
}

Circle scalar__geometric_quotient__circle(Scalar self, Circle other) {
    return scalar__wedge_dot__circle(self, circle__inverse(other));
}

Dipole scalar__geometric_quotient__dipole(Scalar self, Dipole other) {
    return scalar__wedge_dot__dipole(self, dipole__inverse(other));
}

DualNum scalar__geometric_quotient__dual_num(Scalar self, DualNum other) {
    return scalar__wedge_dot__dual_num(self, dual_num__inverse(other));
}

FlatPoint scalar__geometric_quotient__flat_point(Scalar self, FlatPoint other) {
    return scalar__wedge_dot__flat_point(self, flat_point__inverse(other));
}

Flector scalar__geometric_quotient__flector(Scalar self, Flector other) {
    return scalar__wedge_dot__flector(self, flector__inverse(other));
}

Line scalar__geometric_quotient__line(Scalar self, Line other) {
    return scalar__wedge_dot__line(self, line__inverse(other));
}

Motor scalar__geometric_quotient__motor(Scalar self, Motor other) {
    return scalar__wedge_dot__motor(self, motor__inverse(other));
}

MultiVector scalar__geometric_quotient__multi_vector(Scalar self, MultiVector other) {
    return scalar__wedge_dot__multi_vector(self, multi_vector__inverse(other));
}

Plane scalar__geometric_quotient__plane(Scalar self, Plane other) {
    return scalar__wedge_dot__plane(self, plane__inverse(other));
}

RoundPoint scalar__geometric_quotient__round_point(Scalar self, RoundPoint other) {
    return scalar__wedge_dot__round_point(self, round_point__inverse(other));
}

Scalar scalar__geometric_quotient__scalar(Scalar self, Scalar other) {
    return scalar__wedge_dot__scalar(self, scalar__inverse(other));
}

Sphere scalar__geometric_quotient__sphere(Scalar self, Sphere other) {
    return scalar__wedge_dot__sphere(self, sphere__inverse(other));
}

RoundPoint sphere__geometric_quotient__anti_scalar(Sphere self, AntiScalar other) {
    return sphere__wedge_dot__anti_scalar(self, anti_scalar__inverse(other));
}

MultiVector sphere__geometric_quotient__circle(Sphere self, Circle other) {
    return sphere__wedge_dot__circle(self, circle__inverse(other));
}

MultiVector sphere__geometric_quotient__dipole(Sphere self, Dipole other) {
    return sphere__wedge_dot__dipole(self, dipole__inverse(other));
}

MultiVector sphere__geometric_quotient__dual_num(Sphere self, DualNum other) {
    return sphere__wedge_dot__dual_num(self, dual_num__inverse(other));
}

MultiVector sphere__geometric_quotient__flat_point(Sphere self, FlatPoint other) {
    return sphere__wedge_dot__flat_point(self, flat_point__inverse(other));
}

MultiVector sphere__geometric_quotient__flector(Sphere self, Flector other) {
    return sphere__wedge_dot__flector(self, flector__inverse(other));
}

MultiVector sphere__geometric_quotient__line(Sphere self, Line other) {
    return sphere__wedge_dot__line(self, line__inverse(other));
}

MultiVector sphere__geometric_quotient__motor(Sphere self, Motor other) {
    return sphere__wedge_dot__motor(self, motor__inverse(other));
}

MultiVector sphere__geometric_quotient__multi_vector(Sphere self, MultiVector other) {
    return sphere__wedge_dot__multi_vector(self, multi_vector__inverse(other));
}

MultiVector sphere__geometric_quotient__plane(Sphere self, Plane other) {
    return sphere__wedge_dot__plane(self, plane__inverse(other));
}

MultiVector sphere__geometric_quotient__round_point(Sphere self, RoundPoint other) {
    return sphere__wedge_dot__round_point(self, round_point__inverse(other));
}

Sphere sphere__geometric_quotient__scalar(Sphere self, Scalar other) {
    return sphere__wedge_dot__scalar(self, scalar__inverse(other));
}

MultiVector sphere__geometric_quotient__sphere(Sphere self, Sphere other) {
    return sphere__wedge_dot__sphere(self, sphere__inverse(other));
}

Scalar circle__contraction__circle(Circle self, Circle other) {
    return circle__anti_wedge__dipole(self, circle__anti_dual(other));
}

RoundPoint circle__contraction__dipole(Circle self, Dipole other) {
    return circle__anti_wedge__circle(self, dipole__anti_dual(other));
}

RoundPoint circle__contraction__flat_point(Circle self, FlatPoint other) {
    return circle__anti_wedge__circle(self, flat_point__anti_dual(other));
}

MultiVector circle__contraction__flector(Circle self, Flector other) {
    return circle__anti_wedge__multi_vector(self, flector__anti_dual(other));
}

Scalar circle__contraction__line(Circle self, Line other) {
    return circle__anti_wedge__dipole(self, line__anti_dual(other));
}

MultiVector circle__contraction__motor(Circle self, Motor other) {
    return circle__anti_wedge__multi_vector(self, motor__anti_dual(other));
}

MultiVector circle__contraction__multi_vector(Circle self, MultiVector other) {
    return circle__anti_wedge__multi_vector(self, multi_vector__anti_dual(other));
}

Dipole circle__contraction__round_point(Circle self, RoundPoint other) {
    return circle__anti_wedge__sphere(self, round_point__anti_dual(other));
}

Scalar dipole__contraction__dipole(Dipole self, Dipole other) {
    return dipole__anti_wedge__circle(self, dipole__anti_dual(other));
}

Scalar dipole__contraction__flat_point(Dipole self, FlatPoint other) {
    return dipole__anti_wedge__circle(self, flat_point__anti_dual(other));
}

MultiVector dipole__contraction__flector(Dipole self, Flector other) {
    return dipole__anti_wedge__multi_vector(self, flector__anti_dual(other));
}

MultiVector dipole__contraction__motor(Dipole self, Motor other) {
    return dipole__anti_wedge__multi_vector(self, motor__anti_dual(other));
}

MultiVector dipole__contraction__multi_vector(Dipole self, MultiVector other) {
    return dipole__anti_wedge__multi_vector(self, multi_vector__anti_dual(other));
}

RoundPoint dipole__contraction__round_point(Dipole self, RoundPoint other) {
    return dipole__anti_wedge__sphere(self, round_point__anti_dual(other));
}

Scalar flat_point__contraction__dipole(FlatPoint self, Dipole other) {
    return flat_point__anti_wedge__circle(self, dipole__anti_dual(other));
}

Scalar flat_point__contraction__flat_point(FlatPoint self, FlatPoint other) {
    return flat_point__anti_wedge__circle(self, flat_point__anti_dual(other));
}

MultiVector flat_point__contraction__flector(FlatPoint self, Flector other) {
    return flat_point__anti_wedge__multi_vector(self, flector__anti_dual(other));
}

MultiVector flat_point__contraction__motor(FlatPoint self, Motor other) {
    return flat_point__anti_wedge__multi_vector(self, motor__anti_dual(other));
}

MultiVector flat_point__contraction__multi_vector(FlatPoint self, MultiVector other) {
    return flat_point__anti_wedge__multi_vector(self, multi_vector__anti_dual(other));
}

RoundPoint flat_point__contraction__round_point(FlatPoint self, RoundPoint other) {
    return flat_point__anti_wedge__sphere(self, round_point__anti_dual(other));
}

RoundPoint flector__contraction__circle(Flector self, Circle other) {
    return flector__anti_wedge__dipole(self, circle__anti_dual(other));
}

MultiVector flector__contraction__dipole(Flector self, Dipole other) {
    return flector__anti_wedge__circle(self, dipole__anti_dual(other));
}

MultiVector flector__contraction__flat_point(Flector self, FlatPoint other) {
    return flector__anti_wedge__circle(self, flat_point__anti_dual(other));
}

MultiVector flector__contraction__flector(Flector self, Flector other) {
    return flector__anti_wedge__multi_vector(self, flector__anti_dual(other));
}

RoundPoint flector__contraction__line(Flector self, Line other) {
    return flector__anti_wedge__dipole(self, line__anti_dual(other));
}

MultiVector flector__contraction__motor(Flector self, Motor other) {
    return flector__anti_wedge__multi_vector(self, motor__anti_dual(other));
}

MultiVector flector__contraction__multi_vector(Flector self, MultiVector other) {
    return flector__anti_wedge__multi_vector(self, multi_vector__anti_dual(other));
}

Scalar flector__contraction__plane(Flector self, Plane other) {
    return flector__anti_wedge__round_point(self, plane__anti_dual(other));
}

MultiVector flector__contraction__round_point(Flector self, RoundPoint other) {
    return flector__anti_wedge__sphere(self, round_point__anti_dual(other));
}

Scalar flector__contraction__sphere(Flector self, Sphere other) {
    return flector__anti_wedge__round_point(self, sphere__anti_dual(other));
}

Scalar line__contraction__circle(Line self, Circle other) {
    return line__anti_wedge__dipole(self, circle__anti_dual(other));
}

RoundPoint line__contraction__dipole(Line self, Dipole other) {
    return line__anti_wedge__circle(self, dipole__anti_dual(other));
}

RoundPoint line__contraction__flat_point(Line self, FlatPoint other) {
    return line__anti_wedge__circle(self, flat_point__anti_dual(other));
}

MultiVector line__contraction__flector(Line self, Flector other) {
    return line__anti_wedge__multi_vector(self, flector__anti_dual(other));
}

Scalar line__contraction__line(Line self, Line other) {
    return line__anti_wedge__dipole(self, line__anti_dual(other));
}

MultiVector line__contraction__motor(Line self, Motor other) {
    return line__anti_wedge__multi_vector(self, motor__anti_dual(other));
}

MultiVector line__contraction__multi_vector(Line self, MultiVector other) {
    return line__anti_wedge__multi_vector(self, multi_vector__anti_dual(other));
}

Dipole line__contraction__round_point(Line self, RoundPoint other) {
    return line__anti_wedge__sphere(self, round_point__anti_dual(other));
}

MultiVector motor__contraction__circle(Motor self, Circle other) {
    return motor__anti_wedge__dipole(self, circle__anti_dual(other));
}

MultiVector motor__contraction__dipole(Motor self, Dipole other) {
    return motor__anti_wedge__circle(self, dipole__anti_dual(other));
}

MultiVector motor__contraction__flat_point(Motor self, FlatPoint other) {
    return motor__anti_wedge__circle(self, flat_point__anti_dual(other));
}

MultiVector motor__contraction__flector(Motor self, Flector other) {
    return motor__anti_wedge__multi_vector(self, flector__anti_dual(other));
}

MultiVector motor__contraction__line(Motor self, Line other) {
    return motor__anti_wedge__dipole(self, line__anti_dual(other));
}

MultiVector motor__contraction__motor(Motor self, Motor other) {
    return motor__anti_wedge__multi_vector(self, motor__anti_dual(other));
}

MultiVector motor__contraction__multi_vector(Motor self, MultiVector other) {
    return motor__anti_wedge__multi_vector(self, multi_vector__anti_dual(other));
}

RoundPoint motor__contraction__plane(Motor self, Plane other) {
    return motor__anti_wedge__round_point(self, plane__anti_dual(other));
}

MultiVector motor__contraction__round_point(Motor self, RoundPoint other) {
    return motor__anti_wedge__sphere(self, round_point__anti_dual(other));
}

RoundPoint motor__contraction__sphere(Motor self, Sphere other) {
    return motor__anti_wedge__round_point(self, sphere__anti_dual(other));
}

MultiVector multi_vector__contraction__circle(MultiVector self, Circle other) {
    return multi_vector__anti_wedge__dipole(self, circle__anti_dual(other));
}

MultiVector multi_vector__contraction__dipole(MultiVector self, Dipole other) {
    return multi_vector__anti_wedge__circle(self, dipole__anti_dual(other));
}

MultiVector multi_vector__contraction__flat_point(MultiVector self, FlatPoint other) {
    return multi_vector__anti_wedge__circle(self, flat_point__anti_dual(other));
}

MultiVector multi_vector__contraction__flector(MultiVector self, Flector other) {
    return multi_vector__anti_wedge__multi_vector(self, flector__anti_dual(other));
}

MultiVector multi_vector__contraction__line(MultiVector self, Line other) {
    return multi_vector__anti_wedge__dipole(self, line__anti_dual(other));
}

MultiVector multi_vector__contraction__motor(MultiVector self, Motor other) {
    return multi_vector__anti_wedge__multi_vector(self, motor__anti_dual(other));
}

MultiVector multi_vector__contraction__multi_vector(MultiVector self, MultiVector other) {
    return multi_vector__anti_wedge__multi_vector(self, multi_vector__anti_dual(other));
}

MultiVector multi_vector__contraction__plane(MultiVector self, Plane other) {
    return multi_vector__anti_wedge__round_point(self, plane__anti_dual(other));
}

MultiVector multi_vector__contraction__round_point(MultiVector self, RoundPoint other) {
    return multi_vector__anti_wedge__sphere(self, round_point__anti_dual(other));
}

MultiVector multi_vector__contraction__sphere(MultiVector self, Sphere other) {
    return multi_vector__anti_wedge__round_point(self, sphere__anti_dual(other));
}

RoundPoint plane__contraction__circle(Plane self, Circle other) {
    return plane__anti_wedge__dipole(self, circle__anti_dual(other));
}

Dipole plane__contraction__dipole(Plane self, Dipole other) {
    return plane__anti_wedge__circle(self, dipole__anti_dual(other));
}

Dipole plane__contraction__flat_point(Plane self, FlatPoint other) {
    return plane__anti_wedge__circle(self, flat_point__anti_dual(other));
}

MultiVector plane__contraction__flector(Plane self, Flector other) {
    return plane__anti_wedge__multi_vector(self, flector__anti_dual(other));
}

RoundPoint plane__contraction__line(Plane self, Line other) {
    return plane__anti_wedge__dipole(self, line__anti_dual(other));
}

MultiVector plane__contraction__motor(Plane self, Motor other) {
    return plane__anti_wedge__multi_vector(self, motor__anti_dual(other));
}

MultiVector plane__contraction__multi_vector(Plane self, MultiVector other) {
    return plane__anti_wedge__multi_vector(self, multi_vector__anti_dual(other));
}

Scalar plane__contraction__plane(Plane self, Plane other) {
    return plane__anti_wedge__round_point(self, plane__anti_dual(other));
}

Circle plane__contraction__round_point(Plane self, RoundPoint other) {
    return plane__anti_wedge__sphere(self, round_point__anti_dual(other));
}

Scalar plane__contraction__sphere(Plane self, Sphere other) {
    return plane__anti_wedge__round_point(self, sphere__anti_dual(other));
}

MultiVector round_point__contraction__flector(RoundPoint self, Flector other) {
    return round_point__anti_wedge__multi_vector(self, flector__anti_dual(other));
}

MultiVector round_point__contraction__motor(RoundPoint self, Motor other) {
    return round_point__anti_wedge__multi_vector(self, motor__anti_dual(other));
}

MultiVector round_point__contraction__multi_vector(RoundPoint self, MultiVector other) {
    return round_point__anti_wedge__multi_vector(self, multi_vector__anti_dual(other));
}

Scalar round_point__contraction__round_point(RoundPoint self, RoundPoint other) {
    return round_point__anti_wedge__sphere(self, round_point__anti_dual(other));
}

RoundPoint sphere__contraction__circle(Sphere self, Circle other) {
    return sphere__anti_wedge__dipole(self, circle__anti_dual(other));
}

Dipole sphere__contraction__dipole(Sphere self, Dipole other) {
    return sphere__anti_wedge__circle(self, dipole__anti_dual(other));
}

Dipole sphere__contraction__flat_point(Sphere self, FlatPoint other) {
    return sphere__anti_wedge__circle(self, flat_point__anti_dual(other));
}

MultiVector sphere__contraction__flector(Sphere self, Flector other) {
    return sphere__anti_wedge__multi_vector(self, flector__anti_dual(other));
}

RoundPoint sphere__contraction__line(Sphere self, Line other) {
    return sphere__anti_wedge__dipole(self, line__anti_dual(other));
}

MultiVector sphere__contraction__motor(Sphere self, Motor other) {
    return sphere__anti_wedge__multi_vector(self, motor__anti_dual(other));
}

MultiVector sphere__contraction__multi_vector(Sphere self, MultiVector other) {
    return sphere__anti_wedge__multi_vector(self, multi_vector__anti_dual(other));
}

Scalar sphere__contraction__plane(Sphere self, Plane other) {
    return sphere__anti_wedge__round_point(self, plane__anti_dual(other));
}

Circle sphere__contraction__round_point(Sphere self, RoundPoint other) {
    return sphere__anti_wedge__sphere(self, round_point__anti_dual(other));
}

Scalar sphere__contraction__sphere(Sphere self, Sphere other) {
    return sphere__anti_wedge__round_point(self, sphere__anti_dual(other));
}

AntiScalar circle__expansion__circle(Circle self, Circle other) {
    return circle__wedge__dipole(self, circle__anti_dual(other));
}

MultiVector circle__expansion__flector(Circle self, Flector other) {
    return circle__wedge__multi_vector(self, flector__anti_dual(other));
}

AntiScalar circle__expansion__line(Circle self, Line other) {
    return circle__wedge__dipole(self, line__anti_dual(other));
}

MultiVector circle__expansion__motor(Circle self, Motor other) {
    return circle__wedge__multi_vector(self, motor__anti_dual(other));
}

MultiVector circle__expansion__multi_vector(Circle self, MultiVector other) {
    return circle__wedge__multi_vector(self, multi_vector__anti_dual(other));
}

Sphere circle__expansion__plane(Circle self, Plane other) {
    return circle__wedge__round_point(self, plane__anti_dual(other));
}

Sphere circle__expansion__sphere(Circle self, Sphere other) {
    return circle__wedge__round_point(self, sphere__anti_dual(other));
}

Sphere dipole__expansion__circle(Dipole self, Circle other) {
    return dipole__wedge__dipole(self, circle__anti_dual(other));
}

AntiScalar dipole__expansion__dipole(Dipole self, Dipole other) {
    return dipole__wedge__circle(self, dipole__anti_dual(other));
}

AntiScalar dipole__expansion__flat_point(Dipole self, FlatPoint other) {
    return dipole__wedge__circle(self, flat_point__anti_dual(other));
}

MultiVector dipole__expansion__flector(Dipole self, Flector other) {
    return dipole__wedge__multi_vector(self, flector__anti_dual(other));
}

Sphere dipole__expansion__line(Dipole self, Line other) {
    return dipole__wedge__dipole(self, line__anti_dual(other));
}

MultiVector dipole__expansion__motor(Dipole self, Motor other) {
    return dipole__wedge__multi_vector(self, motor__anti_dual(other));
}

MultiVector dipole__expansion__multi_vector(Dipole self, MultiVector other) {
    return dipole__wedge__multi_vector(self, multi_vector__anti_dual(other));
}

Circle dipole__expansion__plane(Dipole self, Plane other) {
    return dipole__wedge__round_point(self, plane__anti_dual(other));
}

Circle dipole__expansion__sphere(Dipole self, Sphere other) {
    return dipole__wedge__round_point(self, sphere__anti_dual(other));
}

Plane flat_point__expansion__circle(FlatPoint self, Circle other) {
    return flat_point__wedge__dipole(self, circle__anti_dual(other));
}

AntiScalar flat_point__expansion__dipole(FlatPoint self, Dipole other) {
    return flat_point__wedge__circle(self, dipole__anti_dual(other));
}

AntiScalar flat_point__expansion__flat_point(FlatPoint self, FlatPoint other) {
    return flat_point__wedge__circle(self, flat_point__anti_dual(other));
}

MultiVector flat_point__expansion__flector(FlatPoint self, Flector other) {
    return flat_point__wedge__multi_vector(self, flector__anti_dual(other));
}

Plane flat_point__expansion__line(FlatPoint self, Line other) {
    return flat_point__wedge__dipole(self, line__anti_dual(other));
}

MultiVector flat_point__expansion__motor(FlatPoint self, Motor other) {
    return flat_point__wedge__multi_vector(self, motor__anti_dual(other));
}

MultiVector flat_point__expansion__multi_vector(FlatPoint self, MultiVector other) {
    return flat_point__wedge__multi_vector(self, multi_vector__anti_dual(other));
}

Line flat_point__expansion__plane(FlatPoint self, Plane other) {
    return flat_point__wedge__round_point(self, plane__anti_dual(other));
}

Line flat_point__expansion__sphere(FlatPoint self, Sphere other) {
    return flat_point__wedge__round_point(self, sphere__anti_dual(other));
}

Plane flector__expansion__circle(Flector self, Circle other) {
    return flector__wedge__dipole(self, circle__anti_dual(other));
}

AntiScalar flector__expansion__dipole(Flector self, Dipole other) {
    return flector__wedge__circle(self, dipole__anti_dual(other));
}

AntiScalar flector__expansion__flat_point(Flector self, FlatPoint other) {
    return flector__wedge__circle(self, flat_point__anti_dual(other));
}

MultiVector flector__expansion__flector(Flector self, Flector other) {
    return flector__wedge__multi_vector(self, flector__anti_dual(other));
}

Plane flector__expansion__line(Flector self, Line other) {
    return flector__wedge__dipole(self, line__anti_dual(other));
}

MultiVector flector__expansion__motor(Flector self, Motor other) {
    return flector__wedge__multi_vector(self, motor__anti_dual(other));
}

MultiVector flector__expansion__multi_vector(Flector self, MultiVector other) {
    return flector__wedge__multi_vector(self, multi_vector__anti_dual(other));
}

Motor flector__expansion__plane(Flector self, Plane other) {
    return flector__wedge__round_point(self, plane__anti_dual(other));
}

Motor flector__expansion__sphere(Flector self, Sphere other) {
    return flector__wedge__round_point(self, sphere__anti_dual(other));
}

AntiScalar line__expansion__circle(Line self, Circle other) {
    return line__wedge__dipole(self, circle__anti_dual(other));
}

MultiVector line__expansion__flector(Line self, Flector other) {
    return line__wedge__multi_vector(self, flector__anti_dual(other));
}

AntiScalar line__expansion__line(Line self, Line other) {
    return line__wedge__dipole(self, line__anti_dual(other));
}

MultiVector line__expansion__motor(Line self, Motor other) {
    return line__wedge__multi_vector(self, motor__anti_dual(other));
}

MultiVector line__expansion__multi_vector(Line self, MultiVector other) {
    return line__wedge__multi_vector(self, multi_vector__anti_dual(other));
}

Plane line__expansion__plane(Line self, Plane other) {
    return line__wedge__round_point(self, plane__anti_dual(other));
}

Plane line__expansion__sphere(Line self, Sphere other) {
    return line__wedge__round_point(self, sphere__anti_dual(other));
}

AntiScalar motor__expansion__circle(Motor self, Circle other) {
    return motor__wedge__dipole(self, circle__anti_dual(other));
}

MultiVector motor__expansion__flector(Motor self, Flector other) {
    return motor__wedge__multi_vector(self, flector__anti_dual(other));
}

AntiScalar motor__expansion__line(Motor self, Line other) {
    return motor__wedge__dipole(self, line__anti_dual(other));
}

MultiVector motor__expansion__motor(Motor self, Motor other) {
    return motor__wedge__multi_vector(self, motor__anti_dual(other));
}

MultiVector motor__expansion__multi_vector(Motor self, MultiVector other) {
    return motor__wedge__multi_vector(self, multi_vector__anti_dual(other));
}

Plane motor__expansion__plane(Motor self, Plane other) {
    return motor__wedge__round_point(self, plane__anti_dual(other));
}

Plane motor__expansion__sphere(Motor self, Sphere other) {
    return motor__wedge__round_point(self, sphere__anti_dual(other));
}

MultiVector multi_vector__expansion__circle(MultiVector self, Circle other) {
    return multi_vector__wedge__dipole(self, circle__anti_dual(other));
}

MultiVector multi_vector__expansion__dipole(MultiVector self, Dipole other) {
    return multi_vector__wedge__circle(self, dipole__anti_dual(other));
}

MultiVector multi_vector__expansion__flat_point(MultiVector self, FlatPoint other) {
    return multi_vector__wedge__circle(self, flat_point__anti_dual(other));
}

MultiVector multi_vector__expansion__flector(MultiVector self, Flector other) {
    return multi_vector__wedge__multi_vector(self, flector__anti_dual(other));
}

MultiVector multi_vector__expansion__line(MultiVector self, Line other) {
    return multi_vector__wedge__dipole(self, line__anti_dual(other));
}

MultiVector multi_vector__expansion__motor(MultiVector self, Motor other) {
    return multi_vector__wedge__multi_vector(self, motor__anti_dual(other));
}

MultiVector multi_vector__expansion__multi_vector(MultiVector self, MultiVector other) {
    return multi_vector__wedge__multi_vector(self, multi_vector__anti_dual(other));
}

MultiVector multi_vector__expansion__plane(MultiVector self, Plane other) {
    return multi_vector__wedge__round_point(self, plane__anti_dual(other));
}

MultiVector multi_vector__expansion__round_point(MultiVector self, RoundPoint other) {
    return multi_vector__wedge__sphere(self, round_point__anti_dual(other));
}

MultiVector multi_vector__expansion__sphere(MultiVector self, Sphere other) {
    return multi_vector__wedge__round_point(self, sphere__anti_dual(other));
}

MultiVector plane__expansion__flector(Plane self, Flector other) {
    return plane__wedge__multi_vector(self, flector__anti_dual(other));
}

MultiVector plane__expansion__motor(Plane self, Motor other) {
    return plane__wedge__multi_vector(self, motor__anti_dual(other));
}

MultiVector plane__expansion__multi_vector(Plane self, MultiVector other) {
    return plane__wedge__multi_vector(self, multi_vector__anti_dual(other));
}

AntiScalar plane__expansion__plane(Plane self, Plane other) {
    return plane__wedge__round_point(self, plane__anti_dual(other));
}

AntiScalar plane__expansion__sphere(Plane self, Sphere other) {
    return plane__wedge__round_point(self, sphere__anti_dual(other));
}

Circle round_point__expansion__circle(RoundPoint self, Circle other) {
    return round_point__wedge__dipole(self, circle__anti_dual(other));
}

Sphere round_point__expansion__dipole(RoundPoint self, Dipole other) {
    return round_point__wedge__circle(self, dipole__anti_dual(other));
}

Sphere round_point__expansion__flat_point(RoundPoint self, FlatPoint other) {
    return round_point__wedge__circle(self, flat_point__anti_dual(other));
}

MultiVector round_point__expansion__flector(RoundPoint self, Flector other) {
    return round_point__wedge__multi_vector(self, flector__anti_dual(other));
}

Circle round_point__expansion__line(RoundPoint self, Line other) {
    return round_point__wedge__dipole(self, line__anti_dual(other));
}

MultiVector round_point__expansion__motor(RoundPoint self, Motor other) {
    return round_point__wedge__multi_vector(self, motor__anti_dual(other));
}

MultiVector round_point__expansion__multi_vector(RoundPoint self, MultiVector other) {
    return round_point__wedge__multi_vector(self, multi_vector__anti_dual(other));
}

Dipole round_point__expansion__plane(RoundPoint self, Plane other) {
    return round_point__wedge__round_point(self, plane__anti_dual(other));
}

AntiScalar round_point__expansion__round_point(RoundPoint self, RoundPoint other) {
    return round_point__wedge__sphere(self, round_point__anti_dual(other));
}

Dipole round_point__expansion__sphere(RoundPoint self, Sphere other) {
    return round_point__wedge__round_point(self, sphere__anti_dual(other));
}

MultiVector sphere__expansion__flector(Sphere self, Flector other) {
    return sphere__wedge__multi_vector(self, flector__anti_dual(other));
}

MultiVector sphere__expansion__motor(Sphere self, Motor other) {
    return sphere__wedge__multi_vector(self, motor__anti_dual(other));
}

MultiVector sphere__expansion__multi_vector(Sphere self, MultiVector other) {
    return sphere__wedge__multi_vector(self, multi_vector__anti_dual(other));
}

AntiScalar sphere__expansion__plane(Sphere self, Plane other) {
    return sphere__wedge__round_point(self, plane__anti_dual(other));
}

AntiScalar sphere__expansion__sphere(Sphere self, Sphere other) {
    return sphere__wedge__round_point(self, sphere__anti_dual(other));
}

Circle circle__anti_project_orthogonally_onto__circle(Circle self, Circle other) {
    return circle__wedge__scalar(other, circle__anti_wedge__dipole(self, circle__anti_dual(other)));
}

Circle circle__anti_project_orthogonally_onto__dipole(Circle self, Dipole other) {
    return dipole__wedge__round_point(other, circle__anti_wedge__circle(self, dipole__anti_dual(other)));
}

Line circle__anti_project_orthogonally_onto__flat_point(Circle self, FlatPoint other) {
    return flat_point__wedge__round_point(other, circle__anti_wedge__circle(self, flat_point__anti_dual(other)));
}

MultiVector circle__anti_project_orthogonally_onto__flector(Circle self, Flector other) {
    return flector__wedge__multi_vector(other, circle__anti_wedge__multi_vector(self, flector__anti_dual(other)));
}

Line circle__anti_project_orthogonally_onto__line(Circle self, Line other) {
    return line__wedge__scalar(other, circle__anti_wedge__dipole(self, line__anti_dual(other)));
}

MultiVector circle__anti_project_orthogonally_onto__motor(Circle self, Motor other) {
    return motor__wedge__multi_vector(other, circle__anti_wedge__multi_vector(self, motor__anti_dual(other)));
}

MultiVector circle__anti_project_orthogonally_onto__multi_vector(Circle self, MultiVector other) {
    return multi_vector__wedge__multi_vector(other, circle__anti_wedge__multi_vector(self, multi_vector__anti_dual(other)));
}

Circle circle__anti_project_orthogonally_onto__round_point(Circle self, RoundPoint other) {
    return round_point__wedge__dipole(other, circle__anti_wedge__sphere(self, round_point__anti_dual(other)));
}

Dipole dipole__anti_project_orthogonally_onto__dipole(Dipole self, Dipole other) {
    return dipole__wedge__scalar(other, dipole__anti_wedge__circle(self, dipole__anti_dual(other)));
}

FlatPoint dipole__anti_project_orthogonally_onto__flat_point(Dipole self, FlatPoint other) {
    return flat_point__wedge__scalar(other, dipole__anti_wedge__circle(self, flat_point__anti_dual(other)));
}

MultiVector dipole__anti_project_orthogonally_onto__flector(Dipole self, Flector other) {
    return flector__wedge__multi_vector(other, dipole__anti_wedge__multi_vector(self, flector__anti_dual(other)));
}

MultiVector dipole__anti_project_orthogonally_onto__motor(Dipole self, Motor other) {
    return motor__wedge__multi_vector(other, dipole__anti_wedge__multi_vector(self, motor__anti_dual(other)));
}

MultiVector dipole__anti_project_orthogonally_onto__multi_vector(Dipole self, MultiVector other) {
    return multi_vector__wedge__multi_vector(other, dipole__anti_wedge__multi_vector(self, multi_vector__anti_dual(other)));
}

Dipole dipole__anti_project_orthogonally_onto__round_point(Dipole self, RoundPoint other) {
    return round_point__wedge__round_point(other, dipole__anti_wedge__sphere(self, round_point__anti_dual(other)));
}

Dipole flat_point__anti_project_orthogonally_onto__dipole(FlatPoint self, Dipole other) {
    return dipole__wedge__scalar(other, flat_point__anti_wedge__circle(self, dipole__anti_dual(other)));
}

FlatPoint flat_point__anti_project_orthogonally_onto__flat_point(FlatPoint self, FlatPoint other) {
    return flat_point__wedge__scalar(other, flat_point__anti_wedge__circle(self, flat_point__anti_dual(other)));
}

MultiVector flat_point__anti_project_orthogonally_onto__flector(FlatPoint self, Flector other) {
    return flector__wedge__multi_vector(other, flat_point__anti_wedge__multi_vector(self, flector__anti_dual(other)));
}

MultiVector flat_point__anti_project_orthogonally_onto__motor(FlatPoint self, Motor other) {
    return motor__wedge__multi_vector(other, flat_point__anti_wedge__multi_vector(self, motor__anti_dual(other)));
}

MultiVector flat_point__anti_project_orthogonally_onto__multi_vector(FlatPoint self, MultiVector other) {
    return multi_vector__wedge__multi_vector(other, flat_point__anti_wedge__multi_vector(self, multi_vector__anti_dual(other)));
}

Dipole flat_point__anti_project_orthogonally_onto__round_point(FlatPoint self, RoundPoint other) {
    return round_point__wedge__round_point(other, flat_point__anti_wedge__sphere(self, round_point__anti_dual(other)));
}

Sphere flector__anti_project_orthogonally_onto__circle(Flector self, Circle other) {
    return circle__wedge__round_point(other, flector__anti_wedge__dipole(self, circle__anti_dual(other)));
}

MultiVector flector__anti_project_orthogonally_onto__dipole(Flector self, Dipole other) {
    return dipole__wedge__multi_vector(other, flector__anti_wedge__circle(self, dipole__anti_dual(other)));
}

MultiVector flector__anti_project_orthogonally_onto__flat_point(Flector self, FlatPoint other) {
    return flat_point__wedge__multi_vector(other, flector__anti_wedge__circle(self, flat_point__anti_dual(other)));
}

MultiVector flector__anti_project_orthogonally_onto__flector(Flector self, Flector other) {
    return flector__wedge__multi_vector(other, flector__anti_wedge__multi_vector(self, flector__anti_dual(other)));
}

Plane flector__anti_project_orthogonally_onto__line(Flector self, Line other) {
    return line__wedge__round_point(other, flector__anti_wedge__dipole(self, line__anti_dual(other)));
}

MultiVector flector__anti_project_orthogonally_onto__motor(Flector self, Motor other) {
    return motor__wedge__multi_vector(other, flector__anti_wedge__multi_vector(self, motor__anti_dual(other)));
}

MultiVector flector__anti_project_orthogonally_onto__multi_vector(Flector self, MultiVector other) {
    return multi_vector__wedge__multi_vector(other, flector__anti_wedge__multi_vector(self, multi_vector__anti_dual(other)));
}

Plane flector__anti_project_orthogonally_onto__plane(Flector self, Plane other) {
    return plane__wedge__scalar(other, flector__anti_wedge__round_point(self, plane__anti_dual(other)));
}

MultiVector flector__anti_project_orthogonally_onto__round_point(Flector self, RoundPoint other) {
    return round_point__wedge__multi_vector(other, flector__anti_wedge__sphere(self, round_point__anti_dual(other)));
}

Sphere flector__anti_project_orthogonally_onto__sphere(Flector self, Sphere other) {
    return sphere__wedge__scalar(other, flector__anti_wedge__round_point(self, sphere__anti_dual(other)));
}

Circle line__anti_project_orthogonally_onto__circle(Line self, Circle other) {
    return circle__wedge__scalar(other, line__anti_wedge__dipole(self, circle__anti_dual(other)));
}

Circle line__anti_project_orthogonally_onto__dipole(Line self, Dipole other) {
    return dipole__wedge__round_point(other, line__anti_wedge__circle(self, dipole__anti_dual(other)));
}

Line line__anti_project_orthogonally_onto__flat_point(Line self, FlatPoint other) {
    return flat_point__wedge__round_point(other, line__anti_wedge__circle(self, flat_point__anti_dual(other)));
}

MultiVector line__anti_project_orthogonally_onto__flector(Line self, Flector other) {
    return flector__wedge__multi_vector(other, line__anti_wedge__multi_vector(self, flector__anti_dual(other)));
}

Line line__anti_project_orthogonally_onto__line(Line self, Line other) {
    return line__wedge__scalar(other, line__anti_wedge__dipole(self, line__anti_dual(other)));
}

MultiVector line__anti_project_orthogonally_onto__motor(Line self, Motor other) {
    return motor__wedge__multi_vector(other, line__anti_wedge__multi_vector(self, motor__anti_dual(other)));
}

MultiVector line__anti_project_orthogonally_onto__multi_vector(Line self, MultiVector other) {
    return multi_vector__wedge__multi_vector(other, line__anti_wedge__multi_vector(self, multi_vector__anti_dual(other)));
}

Circle line__anti_project_orthogonally_onto__round_point(Line self, RoundPoint other) {
    return round_point__wedge__dipole(other, line__anti_wedge__sphere(self, round_point__anti_dual(other)));
}

MultiVector motor__anti_project_orthogonally_onto__circle(Motor self, Circle other) {
    return circle__wedge__multi_vector(other, motor__anti_wedge__dipole(self, circle__anti_dual(other)));
}

MultiVector motor__anti_project_orthogonally_onto__dipole(Motor self, Dipole other) {
    return dipole__wedge__multi_vector(other, motor__anti_wedge__circle(self, dipole__anti_dual(other)));
}

MultiVector motor__anti_project_orthogonally_onto__flat_point(Motor self, FlatPoint other) {
    return flat_point__wedge__multi_vector(other, motor__anti_wedge__circle(self, flat_point__anti_dual(other)));
}

MultiVector motor__anti_project_orthogonally_onto__flector(Motor self, Flector other) {
    return flector__wedge__multi_vector(other, motor__anti_wedge__multi_vector(self, flector__anti_dual(other)));
}

MultiVector motor__anti_project_orthogonally_onto__line(Motor self, Line other) {
    return line__wedge__multi_vector(other, motor__anti_wedge__dipole(self, line__anti_dual(other)));
}

MultiVector motor__anti_project_orthogonally_onto__motor(Motor self, Motor other) {
    return motor__wedge__multi_vector(other, motor__anti_wedge__multi_vector(self, motor__anti_dual(other)));
}

MultiVector motor__anti_project_orthogonally_onto__multi_vector(Motor self, MultiVector other) {
    return multi_vector__wedge__multi_vector(other, motor__anti_wedge__multi_vector(self, multi_vector__anti_dual(other)));
}

AntiScalar motor__anti_project_orthogonally_onto__plane(Motor self, Plane other) {
    return plane__wedge__round_point(other, motor__anti_wedge__round_point(self, plane__anti_dual(other)));
}

MultiVector motor__anti_project_orthogonally_onto__round_point(Motor self, RoundPoint other) {
    return round_point__wedge__multi_vector(other, motor__anti_wedge__sphere(self, round_point__anti_dual(other)));
}

AntiScalar motor__anti_project_orthogonally_onto__sphere(Motor self, Sphere other) {
    return sphere__wedge__round_point(other, motor__anti_wedge__round_point(self, sphere__anti_dual(other)));
}

MultiVector multi_vector__anti_project_orthogonally_onto__circle(MultiVector self, Circle other) {
    return circle__wedge__multi_vector(other, multi_vector__anti_wedge__dipole(self, circle__anti_dual(other)));
}

MultiVector multi_vector__anti_project_orthogonally_onto__dipole(MultiVector self, Dipole other) {
    return dipole__wedge__multi_vector(other, multi_vector__anti_wedge__circle(self, dipole__anti_dual(other)));
}

MultiVector multi_vector__anti_project_orthogonally_onto__flat_point(MultiVector self, FlatPoint other) {
    return flat_point__wedge__multi_vector(other, multi_vector__anti_wedge__circle(self, flat_point__anti_dual(other)));
}

MultiVector multi_vector__anti_project_orthogonally_onto__flector(MultiVector self, Flector other) {
    return flector__wedge__multi_vector(other, multi_vector__anti_wedge__multi_vector(self, flector__anti_dual(other)));
}

MultiVector multi_vector__anti_project_orthogonally_onto__line(MultiVector self, Line other) {
    return line__wedge__multi_vector(other, multi_vector__anti_wedge__dipole(self, line__anti_dual(other)));
}

MultiVector multi_vector__anti_project_orthogonally_onto__motor(MultiVector self, Motor other) {
    return motor__wedge__multi_vector(other, multi_vector__anti_wedge__multi_vector(self, motor__anti_dual(other)));
}

MultiVector multi_vector__anti_project_orthogonally_onto__multi_vector(MultiVector self, MultiVector other) {
    return multi_vector__wedge__multi_vector(other, multi_vector__anti_wedge__multi_vector(self, multi_vector__anti_dual(other)));
}

MultiVector multi_vector__anti_project_orthogonally_onto__plane(MultiVector self, Plane other) {
    return plane__wedge__multi_vector(other, multi_vector__anti_wedge__round_point(self, plane__anti_dual(other)));
}

MultiVector multi_vector__anti_project_orthogonally_onto__round_point(MultiVector self, RoundPoint other) {
    return round_point__wedge__multi_vector(other, multi_vector__anti_wedge__sphere(self, round_point__anti_dual(other)));
}

MultiVector multi_vector__anti_project_orthogonally_onto__sphere(MultiVector self, Sphere other) {
    return sphere__wedge__multi_vector(other, multi_vector__anti_wedge__round_point(self, sphere__anti_dual(other)));
}

Sphere plane__anti_project_orthogonally_onto__circle(Plane self, Circle other) {
    return circle__wedge__round_point(other, plane__anti_wedge__dipole(self, circle__anti_dual(other)));
}

Sphere plane__anti_project_orthogonally_onto__dipole(Plane self, Dipole other) {
    return dipole__wedge__dipole(other, plane__anti_wedge__circle(self, dipole__anti_dual(other)));
}

Plane plane__anti_project_orthogonally_onto__flat_point(Plane self, FlatPoint other) {
    return flat_point__wedge__dipole(other, plane__anti_wedge__circle(self, flat_point__anti_dual(other)));
}

MultiVector plane__anti_project_orthogonally_onto__flector(Plane self, Flector other) {
    return flector__wedge__multi_vector(other, plane__anti_wedge__multi_vector(self, flector__anti_dual(other)));
}

Plane plane__anti_project_orthogonally_onto__line(Plane self, Line other) {
    return line__wedge__round_point(other, plane__anti_wedge__dipole(self, line__anti_dual(other)));
}

MultiVector plane__anti_project_orthogonally_onto__motor(Plane self, Motor other) {
    return motor__wedge__multi_vector(other, plane__anti_wedge__multi_vector(self, motor__anti_dual(other)));
}

MultiVector plane__anti_project_orthogonally_onto__multi_vector(Plane self, MultiVector other) {
    return multi_vector__wedge__multi_vector(other, plane__anti_wedge__multi_vector(self, multi_vector__anti_dual(other)));
}

Plane plane__anti_project_orthogonally_onto__plane(Plane self, Plane other) {
    return plane__wedge__scalar(other, plane__anti_wedge__round_point(self, plane__anti_dual(other)));
}

Sphere plane__anti_project_orthogonally_onto__round_point(Plane self, RoundPoint other) {
    return round_point__wedge__circle(other, plane__anti_wedge__sphere(self, round_point__anti_dual(other)));
}

Sphere plane__anti_project_orthogonally_onto__sphere(Plane self, Sphere other) {
    return sphere__wedge__scalar(other, plane__anti_wedge__round_point(self, sphere__anti_dual(other)));
}

MultiVector round_point__anti_project_orthogonally_onto__flector(RoundPoint self, Flector other) {
    return flector__wedge__multi_vector(other, round_point__anti_wedge__multi_vector(self, flector__anti_dual(other)));
}

MultiVector round_point__anti_project_orthogonally_onto__motor(RoundPoint self, Motor other) {
    return motor__wedge__multi_vector(other, round_point__anti_wedge__multi_vector(self, motor__anti_dual(other)));
}

MultiVector round_point__anti_project_orthogonally_onto__multi_vector(RoundPoint self, MultiVector other) {
    return multi_vector__wedge__multi_vector(other, round_point__anti_wedge__multi_vector(self, multi_vector__anti_dual(other)));
}

RoundPoint round_point__anti_project_orthogonally_onto__round_point(RoundPoint self, RoundPoint other) {
    return round_point__wedge__scalar(other, round_point__anti_wedge__sphere(self, round_point__anti_dual(other)));
}

Sphere sphere__anti_project_orthogonally_onto__circle(Sphere self, Circle other) {
    return circle__wedge__round_point(other, sphere__anti_wedge__dipole(self, circle__anti_dual(other)));
}

Sphere sphere__anti_project_orthogonally_onto__dipole(Sphere self, Dipole other) {
    return dipole__wedge__dipole(other, sphere__anti_wedge__circle(self, dipole__anti_dual(other)));
}

Plane sphere__anti_project_orthogonally_onto__flat_point(Sphere self, FlatPoint other) {
    return flat_point__wedge__dipole(other, sphere__anti_wedge__circle(self, flat_point__anti_dual(other)));
}

MultiVector sphere__anti_project_orthogonally_onto__flector(Sphere self, Flector other) {
    return flector__wedge__multi_vector(other, sphere__anti_wedge__multi_vector(self, flector__anti_dual(other)));
}

Plane sphere__anti_project_orthogonally_onto__line(Sphere self, Line other) {
    return line__wedge__round_point(other, sphere__anti_wedge__dipole(self, line__anti_dual(other)));
}

MultiVector sphere__anti_project_orthogonally_onto__motor(Sphere self, Motor other) {
    return motor__wedge__multi_vector(other, sphere__anti_wedge__multi_vector(self, motor__anti_dual(other)));
}

MultiVector sphere__anti_project_orthogonally_onto__multi_vector(Sphere self, MultiVector other) {
    return multi_vector__wedge__multi_vector(other, sphere__anti_wedge__multi_vector(self, multi_vector__anti_dual(other)));
}

Plane sphere__anti_project_orthogonally_onto__plane(Sphere self, Plane other) {
    return plane__wedge__scalar(other, sphere__anti_wedge__round_point(self, plane__anti_dual(other)));
}

Sphere sphere__anti_project_orthogonally_onto__round_point(Sphere self, RoundPoint other) {
    return round_point__wedge__circle(other, sphere__anti_wedge__sphere(self, round_point__anti_dual(other)));
}

Sphere sphere__anti_project_orthogonally_onto__sphere(Sphere self, Sphere other) {
    return sphere__wedge__scalar(other, sphere__anti_wedge__round_point(self, sphere__anti_dual(other)));
}

Circle circle__anti_project_via_horizon_onto__circle(Circle self, Circle other) {
    return circle__wedge__scalar(other, circle__anti_wedge__dipole(self, circle__dual(other)));
}

Circle circle__anti_project_via_horizon_onto__dipole(Circle self, Dipole other) {
    return dipole__wedge__round_point(other, circle__anti_wedge__circle(self, dipole__dual(other)));
}

Line circle__anti_project_via_horizon_onto__flat_point(Circle self, FlatPoint other) {
    return flat_point__wedge__round_point(other, circle__anti_wedge__circle(self, flat_point__dual(other)));
}

MultiVector circle__anti_project_via_horizon_onto__flector(Circle self, Flector other) {
    return flector__wedge__multi_vector(other, circle__anti_wedge__multi_vector(self, flector__dual(other)));
}

Line circle__anti_project_via_horizon_onto__line(Circle self, Line other) {
    return line__wedge__scalar(other, circle__anti_wedge__dipole(self, line__dual(other)));
}

MultiVector circle__anti_project_via_horizon_onto__motor(Circle self, Motor other) {
    return motor__wedge__multi_vector(other, circle__anti_wedge__multi_vector(self, motor__dual(other)));
}

MultiVector circle__anti_project_via_horizon_onto__multi_vector(Circle self, MultiVector other) {
    return multi_vector__wedge__multi_vector(other, circle__anti_wedge__multi_vector(self, multi_vector__dual(other)));
}

Circle circle__anti_project_via_horizon_onto__round_point(Circle self, RoundPoint other) {
    return round_point__wedge__dipole(other, circle__anti_wedge__sphere(self, round_point__dual(other)));
}

Dipole dipole__anti_project_via_horizon_onto__dipole(Dipole self, Dipole other) {
    return dipole__wedge__scalar(other, dipole__anti_wedge__circle(self, dipole__dual(other)));
}

FlatPoint dipole__anti_project_via_horizon_onto__flat_point(Dipole self, FlatPoint other) {
    return flat_point__wedge__scalar(other, dipole__anti_wedge__circle(self, flat_point__dual(other)));
}

MultiVector dipole__anti_project_via_horizon_onto__flector(Dipole self, Flector other) {
    return flector__wedge__multi_vector(other, dipole__anti_wedge__multi_vector(self, flector__dual(other)));
}

MultiVector dipole__anti_project_via_horizon_onto__motor(Dipole self, Motor other) {
    return motor__wedge__multi_vector(other, dipole__anti_wedge__multi_vector(self, motor__dual(other)));
}

MultiVector dipole__anti_project_via_horizon_onto__multi_vector(Dipole self, MultiVector other) {
    return multi_vector__wedge__multi_vector(other, dipole__anti_wedge__multi_vector(self, multi_vector__dual(other)));
}

Dipole dipole__anti_project_via_horizon_onto__round_point(Dipole self, RoundPoint other) {
    return round_point__wedge__round_point(other, dipole__anti_wedge__sphere(self, round_point__dual(other)));
}

Dipole flat_point__anti_project_via_horizon_onto__dipole(FlatPoint self, Dipole other) {
    return dipole__wedge__scalar(other, flat_point__anti_wedge__circle(self, dipole__dual(other)));
}

FlatPoint flat_point__anti_project_via_horizon_onto__flat_point(FlatPoint self, FlatPoint other) {
    return flat_point__wedge__scalar(other, flat_point__anti_wedge__circle(self, flat_point__dual(other)));
}

MultiVector flat_point__anti_project_via_horizon_onto__flector(FlatPoint self, Flector other) {
    return flector__wedge__multi_vector(other, flat_point__anti_wedge__multi_vector(self, flector__dual(other)));
}

MultiVector flat_point__anti_project_via_horizon_onto__motor(FlatPoint self, Motor other) {
    return motor__wedge__multi_vector(other, flat_point__anti_wedge__multi_vector(self, motor__dual(other)));
}

MultiVector flat_point__anti_project_via_horizon_onto__multi_vector(FlatPoint self, MultiVector other) {
    return multi_vector__wedge__multi_vector(other, flat_point__anti_wedge__multi_vector(self, multi_vector__dual(other)));
}

Dipole flat_point__anti_project_via_horizon_onto__round_point(FlatPoint self, RoundPoint other) {
    return round_point__wedge__round_point(other, flat_point__anti_wedge__sphere(self, round_point__dual(other)));
}

Sphere flector__anti_project_via_horizon_onto__circle(Flector self, Circle other) {
    return circle__wedge__round_point(other, flector__anti_wedge__dipole(self, circle__dual(other)));
}

MultiVector flector__anti_project_via_horizon_onto__dipole(Flector self, Dipole other) {
    return dipole__wedge__multi_vector(other, flector__anti_wedge__circle(self, dipole__dual(other)));
}

MultiVector flector__anti_project_via_horizon_onto__flat_point(Flector self, FlatPoint other) {
    return flat_point__wedge__multi_vector(other, flector__anti_wedge__circle(self, flat_point__dual(other)));
}

MultiVector flector__anti_project_via_horizon_onto__flector(Flector self, Flector other) {
    return flector__wedge__multi_vector(other, flector__anti_wedge__multi_vector(self, flector__dual(other)));
}

Plane flector__anti_project_via_horizon_onto__line(Flector self, Line other) {
    return line__wedge__round_point(other, flector__anti_wedge__dipole(self, line__dual(other)));
}

MultiVector flector__anti_project_via_horizon_onto__motor(Flector self, Motor other) {
    return motor__wedge__multi_vector(other, flector__anti_wedge__multi_vector(self, motor__dual(other)));
}

MultiVector flector__anti_project_via_horizon_onto__multi_vector(Flector self, MultiVector other) {
    return multi_vector__wedge__multi_vector(other, flector__anti_wedge__multi_vector(self, multi_vector__dual(other)));
}

Plane flector__anti_project_via_horizon_onto__plane(Flector self, Plane other) {
    return plane__wedge__scalar(other, flector__anti_wedge__round_point(self, plane__dual(other)));
}

MultiVector flector__anti_project_via_horizon_onto__round_point(Flector self, RoundPoint other) {
    return round_point__wedge__multi_vector(other, flector__anti_wedge__sphere(self, round_point__dual(other)));
}

Sphere flector__anti_project_via_horizon_onto__sphere(Flector self, Sphere other) {
    return sphere__wedge__scalar(other, flector__anti_wedge__round_point(self, sphere__dual(other)));
}

Circle line__anti_project_via_horizon_onto__circle(Line self, Circle other) {
    return circle__wedge__scalar(other, line__anti_wedge__dipole(self, circle__dual(other)));
}

Circle line__anti_project_via_horizon_onto__dipole(Line self, Dipole other) {
    return dipole__wedge__round_point(other, line__anti_wedge__circle(self, dipole__dual(other)));
}

Line line__anti_project_via_horizon_onto__flat_point(Line self, FlatPoint other) {
    return flat_point__wedge__round_point(other, line__anti_wedge__circle(self, flat_point__dual(other)));
}

MultiVector line__anti_project_via_horizon_onto__flector(Line self, Flector other) {
    return flector__wedge__multi_vector(other, line__anti_wedge__multi_vector(self, flector__dual(other)));
}

Line line__anti_project_via_horizon_onto__line(Line self, Line other) {
    return line__wedge__scalar(other, line__anti_wedge__dipole(self, line__dual(other)));
}

MultiVector line__anti_project_via_horizon_onto__motor(Line self, Motor other) {
    return motor__wedge__multi_vector(other, line__anti_wedge__multi_vector(self, motor__dual(other)));
}

MultiVector line__anti_project_via_horizon_onto__multi_vector(Line self, MultiVector other) {
    return multi_vector__wedge__multi_vector(other, line__anti_wedge__multi_vector(self, multi_vector__dual(other)));
}

Circle line__anti_project_via_horizon_onto__round_point(Line self, RoundPoint other) {
    return round_point__wedge__dipole(other, line__anti_wedge__sphere(self, round_point__dual(other)));
}

MultiVector motor__anti_project_via_horizon_onto__circle(Motor self, Circle other) {
    return circle__wedge__multi_vector(other, motor__anti_wedge__dipole(self, circle__dual(other)));
}

MultiVector motor__anti_project_via_horizon_onto__dipole(Motor self, Dipole other) {
    return dipole__wedge__multi_vector(other, motor__anti_wedge__circle(self, dipole__dual(other)));
}

MultiVector motor__anti_project_via_horizon_onto__flat_point(Motor self, FlatPoint other) {
    return flat_point__wedge__multi_vector(other, motor__anti_wedge__circle(self, flat_point__dual(other)));
}

MultiVector motor__anti_project_via_horizon_onto__flector(Motor self, Flector other) {
    return flector__wedge__multi_vector(other, motor__anti_wedge__multi_vector(self, flector__dual(other)));
}

MultiVector motor__anti_project_via_horizon_onto__line(Motor self, Line other) {
    return line__wedge__multi_vector(other, motor__anti_wedge__dipole(self, line__dual(other)));
}

MultiVector motor__anti_project_via_horizon_onto__motor(Motor self, Motor other) {
    return motor__wedge__multi_vector(other, motor__anti_wedge__multi_vector(self, motor__dual(other)));
}

MultiVector motor__anti_project_via_horizon_onto__multi_vector(Motor self, MultiVector other) {
    return multi_vector__wedge__multi_vector(other, motor__anti_wedge__multi_vector(self, multi_vector__dual(other)));
}

AntiScalar motor__anti_project_via_horizon_onto__plane(Motor self, Plane other) {
    return plane__wedge__round_point(other, motor__anti_wedge__round_point(self, plane__dual(other)));
}

MultiVector motor__anti_project_via_horizon_onto__round_point(Motor self, RoundPoint other) {
    return round_point__wedge__multi_vector(other, motor__anti_wedge__sphere(self, round_point__dual(other)));
}

AntiScalar motor__anti_project_via_horizon_onto__sphere(Motor self, Sphere other) {
    return sphere__wedge__round_point(other, motor__anti_wedge__round_point(self, sphere__dual(other)));
}

MultiVector multi_vector__anti_project_via_horizon_onto__circle(MultiVector self, Circle other) {
    return circle__wedge__multi_vector(other, multi_vector__anti_wedge__dipole(self, circle__dual(other)));
}

MultiVector multi_vector__anti_project_via_horizon_onto__dipole(MultiVector self, Dipole other) {
    return dipole__wedge__multi_vector(other, multi_vector__anti_wedge__circle(self, dipole__dual(other)));
}

MultiVector multi_vector__anti_project_via_horizon_onto__flat_point(MultiVector self, FlatPoint other) {
    return flat_point__wedge__multi_vector(other, multi_vector__anti_wedge__circle(self, flat_point__dual(other)));
}

MultiVector multi_vector__anti_project_via_horizon_onto__flector(MultiVector self, Flector other) {
    return flector__wedge__multi_vector(other, multi_vector__anti_wedge__multi_vector(self, flector__dual(other)));
}

MultiVector multi_vector__anti_project_via_horizon_onto__line(MultiVector self, Line other) {
    return line__wedge__multi_vector(other, multi_vector__anti_wedge__dipole(self, line__dual(other)));
}

MultiVector multi_vector__anti_project_via_horizon_onto__motor(MultiVector self, Motor other) {
    return motor__wedge__multi_vector(other, multi_vector__anti_wedge__multi_vector(self, motor__dual(other)));
}

MultiVector multi_vector__anti_project_via_horizon_onto__multi_vector(MultiVector self, MultiVector other) {
    return multi_vector__wedge__multi_vector(other, multi_vector__anti_wedge__multi_vector(self, multi_vector__dual(other)));
}

MultiVector multi_vector__anti_project_via_horizon_onto__plane(MultiVector self, Plane other) {
    return plane__wedge__multi_vector(other, multi_vector__anti_wedge__round_point(self, plane__dual(other)));
}

MultiVector multi_vector__anti_project_via_horizon_onto__round_point(MultiVector self, RoundPoint other) {
    return round_point__wedge__multi_vector(other, multi_vector__anti_wedge__sphere(self, round_point__dual(other)));
}

MultiVector multi_vector__anti_project_via_horizon_onto__sphere(MultiVector self, Sphere other) {
    return sphere__wedge__multi_vector(other, multi_vector__anti_wedge__round_point(self, sphere__dual(other)));
}

Sphere plane__anti_project_via_horizon_onto__circle(Plane self, Circle other) {
    return circle__wedge__round_point(other, plane__anti_wedge__dipole(self, circle__dual(other)));
}

Sphere plane__anti_project_via_horizon_onto__dipole(Plane self, Dipole other) {
    return dipole__wedge__dipole(other, plane__anti_wedge__circle(self, dipole__dual(other)));
}

Plane plane__anti_project_via_horizon_onto__flat_point(Plane self, FlatPoint other) {
    return flat_point__wedge__dipole(other, plane__anti_wedge__circle(self, flat_point__dual(other)));
}

MultiVector plane__anti_project_via_horizon_onto__flector(Plane self, Flector other) {
    return flector__wedge__multi_vector(other, plane__anti_wedge__multi_vector(self, flector__dual(other)));
}

Plane plane__anti_project_via_horizon_onto__line(Plane self, Line other) {
    return line__wedge__round_point(other, plane__anti_wedge__dipole(self, line__dual(other)));
}

MultiVector plane__anti_project_via_horizon_onto__motor(Plane self, Motor other) {
    return motor__wedge__multi_vector(other, plane__anti_wedge__multi_vector(self, motor__dual(other)));
}

MultiVector plane__anti_project_via_horizon_onto__multi_vector(Plane self, MultiVector other) {
    return multi_vector__wedge__multi_vector(other, plane__anti_wedge__multi_vector(self, multi_vector__dual(other)));
}

Plane plane__anti_project_via_horizon_onto__plane(Plane self, Plane other) {
    return plane__wedge__scalar(other, plane__anti_wedge__round_point(self, plane__dual(other)));
}

Sphere plane__anti_project_via_horizon_onto__round_point(Plane self, RoundPoint other) {
    return round_point__wedge__circle(other, plane__anti_wedge__sphere(self, round_point__dual(other)));
}

Sphere plane__anti_project_via_horizon_onto__sphere(Plane self, Sphere other) {
    return sphere__wedge__scalar(other, plane__anti_wedge__round_point(self, sphere__dual(other)));
}

MultiVector round_point__anti_project_via_horizon_onto__flector(RoundPoint self, Flector other) {
    return flector__wedge__multi_vector(other, round_point__anti_wedge__multi_vector(self, flector__dual(other)));
}

MultiVector round_point__anti_project_via_horizon_onto__motor(RoundPoint self, Motor other) {
    return motor__wedge__multi_vector(other, round_point__anti_wedge__multi_vector(self, motor__dual(other)));
}

MultiVector round_point__anti_project_via_horizon_onto__multi_vector(RoundPoint self, MultiVector other) {
    return multi_vector__wedge__multi_vector(other, round_point__anti_wedge__multi_vector(self, multi_vector__dual(other)));
}

RoundPoint round_point__anti_project_via_horizon_onto__round_point(RoundPoint self, RoundPoint other) {
    return round_point__wedge__scalar(other, round_point__anti_wedge__sphere(self, round_point__dual(other)));
}

Sphere sphere__anti_project_via_horizon_onto__circle(Sphere self, Circle other) {
    return circle__wedge__round_point(other, sphere__anti_wedge__dipole(self, circle__dual(other)));
}

Sphere sphere__anti_project_via_horizon_onto__dipole(Sphere self, Dipole other) {
    return dipole__wedge__dipole(other, sphere__anti_wedge__circle(self, dipole__dual(other)));
}

Plane sphere__anti_project_via_horizon_onto__flat_point(Sphere self, FlatPoint other) {
    return flat_point__wedge__dipole(other, sphere__anti_wedge__circle(self, flat_point__dual(other)));
}

MultiVector sphere__anti_project_via_horizon_onto__flector(Sphere self, Flector other) {
    return flector__wedge__multi_vector(other, sphere__anti_wedge__multi_vector(self, flector__dual(other)));
}

Plane sphere__anti_project_via_horizon_onto__line(Sphere self, Line other) {
    return line__wedge__round_point(other, sphere__anti_wedge__dipole(self, line__dual(other)));
}

MultiVector sphere__anti_project_via_horizon_onto__motor(Sphere self, Motor other) {
    return motor__wedge__multi_vector(other, sphere__anti_wedge__multi_vector(self, motor__dual(other)));
}

MultiVector sphere__anti_project_via_horizon_onto__multi_vector(Sphere self, MultiVector other) {
    return multi_vector__wedge__multi_vector(other, sphere__anti_wedge__multi_vector(self, multi_vector__dual(other)));
}

Plane sphere__anti_project_via_horizon_onto__plane(Sphere self, Plane other) {
    return plane__wedge__scalar(other, sphere__anti_wedge__round_point(self, plane__dual(other)));
}

Sphere sphere__anti_project_via_horizon_onto__round_point(Sphere self, RoundPoint other) {
    return round_point__wedge__circle(other, sphere__anti_wedge__sphere(self, round_point__dual(other)));
}

Sphere sphere__anti_project_via_horizon_onto__sphere(Sphere self, Sphere other) {
    return sphere__wedge__scalar(other, sphere__anti_wedge__round_point(self, sphere__dual(other)));
}

Circle circle__project_orthogonally_onto__circle(Circle self, Circle other) {
    return circle__anti_wedge__anti_scalar(other, circle__wedge__dipole(self, circle__anti_dual(other)));
}

MultiVector circle__project_orthogonally_onto__flector(Circle self, Flector other) {
    return flector__anti_wedge__multi_vector(other, circle__wedge__multi_vector(self, flector__anti_dual(other)));
}

Line circle__project_orthogonally_onto__line(Circle self, Line other) {
    return line__anti_wedge__anti_scalar(other, circle__wedge__dipole(self, line__anti_dual(other)));
}

MultiVector circle__project_orthogonally_onto__motor(Circle self, Motor other) {
    return motor__anti_wedge__multi_vector(other, circle__wedge__multi_vector(self, motor__anti_dual(other)));
}

MultiVector circle__project_orthogonally_onto__multi_vector(Circle self, MultiVector other) {
    return multi_vector__anti_wedge__multi_vector(other, circle__wedge__multi_vector(self, multi_vector__anti_dual(other)));
}

Circle circle__project_orthogonally_onto__plane(Circle self, Plane other) {
    return plane__anti_wedge__sphere(other, circle__wedge__round_point(self, plane__anti_dual(other)));
}

Circle circle__project_orthogonally_onto__sphere(Circle self, Sphere other) {
    return sphere__anti_wedge__sphere(other, circle__wedge__round_point(self, sphere__anti_dual(other)));
}

Dipole dipole__project_orthogonally_onto__circle(Dipole self, Circle other) {
    return circle__anti_wedge__sphere(other, dipole__wedge__dipole(self, circle__anti_dual(other)));
}

Dipole dipole__project_orthogonally_onto__dipole(Dipole self, Dipole other) {
    return dipole__anti_wedge__anti_scalar(other, dipole__wedge__circle(self, dipole__anti_dual(other)));
}

FlatPoint dipole__project_orthogonally_onto__flat_point(Dipole self, FlatPoint other) {
    return flat_point__anti_wedge__anti_scalar(other, dipole__wedge__circle(self, flat_point__anti_dual(other)));
}

MultiVector dipole__project_orthogonally_onto__flector(Dipole self, Flector other) {
    return flector__anti_wedge__multi_vector(other, dipole__wedge__multi_vector(self, flector__anti_dual(other)));
}

Dipole dipole__project_orthogonally_onto__line(Dipole self, Line other) {
    return line__anti_wedge__sphere(other, dipole__wedge__dipole(self, line__anti_dual(other)));
}

MultiVector dipole__project_orthogonally_onto__motor(Dipole self, Motor other) {
    return motor__anti_wedge__multi_vector(other, dipole__wedge__multi_vector(self, motor__anti_dual(other)));
}

MultiVector dipole__project_orthogonally_onto__multi_vector(Dipole self, MultiVector other) {
    return multi_vector__anti_wedge__multi_vector(other, dipole__wedge__multi_vector(self, multi_vector__anti_dual(other)));
}

Dipole dipole__project_orthogonally_onto__plane(Dipole self, Plane other) {
    return plane__anti_wedge__circle(other, dipole__wedge__round_point(self, plane__anti_dual(other)));
}

Dipole dipole__project_orthogonally_onto__sphere(Dipole self, Sphere other) {
    return sphere__anti_wedge__circle(other, dipole__wedge__round_point(self, sphere__anti_dual(other)));
}

Dipole flat_point__project_orthogonally_onto__circle(FlatPoint self, Circle other) {
    return circle__anti_wedge__plane(other, flat_point__wedge__dipole(self, circle__anti_dual(other)));
}

Dipole flat_point__project_orthogonally_onto__dipole(FlatPoint self, Dipole other) {
    return dipole__anti_wedge__anti_scalar(other, flat_point__wedge__circle(self, dipole__anti_dual(other)));
}

FlatPoint flat_point__project_orthogonally_onto__flat_point(FlatPoint self, FlatPoint other) {
    return flat_point__anti_wedge__anti_scalar(other, flat_point__wedge__circle(self, flat_point__anti_dual(other)));
}

MultiVector flat_point__project_orthogonally_onto__flector(FlatPoint self, Flector other) {
    return flector__anti_wedge__multi_vector(other, flat_point__wedge__multi_vector(self, flector__anti_dual(other)));
}

FlatPoint flat_point__project_orthogonally_onto__line(FlatPoint self, Line other) {
    return line__anti_wedge__plane(other, flat_point__wedge__dipole(self, line__anti_dual(other)));
}

MultiVector flat_point__project_orthogonally_onto__motor(FlatPoint self, Motor other) {
    return motor__anti_wedge__multi_vector(other, flat_point__wedge__multi_vector(self, motor__anti_dual(other)));
}

MultiVector flat_point__project_orthogonally_onto__multi_vector(FlatPoint self, MultiVector other) {
    return multi_vector__anti_wedge__multi_vector(other, flat_point__wedge__multi_vector(self, multi_vector__anti_dual(other)));
}

FlatPoint flat_point__project_orthogonally_onto__plane(FlatPoint self, Plane other) {
    return plane__anti_wedge__line(other, flat_point__wedge__round_point(self, plane__anti_dual(other)));
}

Dipole flat_point__project_orthogonally_onto__sphere(FlatPoint self, Sphere other) {
    return sphere__anti_wedge__line(other, flat_point__wedge__round_point(self, sphere__anti_dual(other)));
}

Dipole flector__project_orthogonally_onto__circle(Flector self, Circle other) {
    return circle__anti_wedge__plane(other, flector__wedge__dipole(self, circle__anti_dual(other)));
}

Dipole flector__project_orthogonally_onto__dipole(Flector self, Dipole other) {
    return dipole__anti_wedge__anti_scalar(other, flector__wedge__circle(self, dipole__anti_dual(other)));
}

FlatPoint flector__project_orthogonally_onto__flat_point(Flector self, FlatPoint other) {
    return flat_point__anti_wedge__anti_scalar(other, flector__wedge__circle(self, flat_point__anti_dual(other)));
}

MultiVector flector__project_orthogonally_onto__flector(Flector self, Flector other) {
    return flector__anti_wedge__multi_vector(other, flector__wedge__multi_vector(self, flector__anti_dual(other)));
}

FlatPoint flector__project_orthogonally_onto__line(Flector self, Line other) {
    return line__anti_wedge__plane(other, flector__wedge__dipole(self, line__anti_dual(other)));
}

MultiVector flector__project_orthogonally_onto__motor(Flector self, Motor other) {
    return motor__anti_wedge__multi_vector(other, flector__wedge__multi_vector(self, motor__anti_dual(other)));
}

MultiVector flector__project_orthogonally_onto__multi_vector(Flector self, MultiVector other) {
    return multi_vector__anti_wedge__multi_vector(other, flector__wedge__multi_vector(self, multi_vector__anti_dual(other)));
}

Flector flector__project_orthogonally_onto__plane(Flector self, Plane other) {
    return plane__anti_wedge__motor(other, flector__wedge__round_point(self, plane__anti_dual(other)));
}

MultiVector flector__project_orthogonally_onto__sphere(Flector self, Sphere other) {
    return sphere__anti_wedge__motor(other, flector__wedge__round_point(self, sphere__anti_dual(other)));
}

Circle line__project_orthogonally_onto__circle(Line self, Circle other) {
    return circle__anti_wedge__anti_scalar(other, line__wedge__dipole(self, circle__anti_dual(other)));
}

MultiVector line__project_orthogonally_onto__flector(Line self, Flector other) {
    return flector__anti_wedge__multi_vector(other, line__wedge__multi_vector(self, flector__anti_dual(other)));
}

Line line__project_orthogonally_onto__line(Line self, Line other) {
    return line__anti_wedge__anti_scalar(other, line__wedge__dipole(self, line__anti_dual(other)));
}

MultiVector line__project_orthogonally_onto__motor(Line self, Motor other) {
    return motor__anti_wedge__multi_vector(other, line__wedge__multi_vector(self, motor__anti_dual(other)));
}

MultiVector line__project_orthogonally_onto__multi_vector(Line self, MultiVector other) {
    return multi_vector__anti_wedge__multi_vector(other, line__wedge__multi_vector(self, multi_vector__anti_dual(other)));
}

Line line__project_orthogonally_onto__plane(Line self, Plane other) {
    return plane__anti_wedge__plane(other, line__wedge__round_point(self, plane__anti_dual(other)));
}

Circle line__project_orthogonally_onto__sphere(Line self, Sphere other) {
    return sphere__anti_wedge__plane(other, line__wedge__round_point(self, sphere__anti_dual(other)));
}

Circle motor__project_orthogonally_onto__circle(Motor self, Circle other) {
    return circle__anti_wedge__anti_scalar(other, motor__wedge__dipole(self, circle__anti_dual(other)));
}

MultiVector motor__project_orthogonally_onto__flector(Motor self, Flector other) {
    return flector__anti_wedge__multi_vector(other, motor__wedge__multi_vector(self, flector__anti_dual(other)));
}

Line motor__project_orthogonally_onto__line(Motor self, Line other) {
    return line__anti_wedge__anti_scalar(other, motor__wedge__dipole(self, line__anti_dual(other)));
}

MultiVector motor__project_orthogonally_onto__motor(Motor self, Motor other) {
    return motor__anti_wedge__multi_vector(other, motor__wedge__multi_vector(self, motor__anti_dual(other)));
}

MultiVector motor__project_orthogonally_onto__multi_vector(Motor self, MultiVector other) {
    return multi_vector__anti_wedge__multi_vector(other, motor__wedge__multi_vector(self, multi_vector__anti_dual(other)));
}

Line motor__project_orthogonally_onto__plane(Motor self, Plane other) {
    return plane__anti_wedge__plane(other, motor__wedge__round_point(self, plane__anti_dual(other)));
}

Circle motor__project_orthogonally_onto__sphere(Motor self, Sphere other) {
    return sphere__anti_wedge__plane(other, motor__wedge__round_point(self, sphere__anti_dual(other)));
}

MultiVector multi_vector__project_orthogonally_onto__circle(MultiVector self, Circle other) {
    return circle__anti_wedge__multi_vector(other, multi_vector__wedge__dipole(self, circle__anti_dual(other)));
}

MultiVector multi_vector__project_orthogonally_onto__dipole(MultiVector self, Dipole other) {
    return dipole__anti_wedge__multi_vector(other, multi_vector__wedge__circle(self, dipole__anti_dual(other)));
}

MultiVector multi_vector__project_orthogonally_onto__flat_point(MultiVector self, FlatPoint other) {
    return flat_point__anti_wedge__multi_vector(other, multi_vector__wedge__circle(self, flat_point__anti_dual(other)));
}

MultiVector multi_vector__project_orthogonally_onto__flector(MultiVector self, Flector other) {
    return flector__anti_wedge__multi_vector(other, multi_vector__wedge__multi_vector(self, flector__anti_dual(other)));
}

MultiVector multi_vector__project_orthogonally_onto__line(MultiVector self, Line other) {
    return line__anti_wedge__multi_vector(other, multi_vector__wedge__dipole(self, line__anti_dual(other)));
}

MultiVector multi_vector__project_orthogonally_onto__motor(MultiVector self, Motor other) {
    return motor__anti_wedge__multi_vector(other, multi_vector__wedge__multi_vector(self, motor__anti_dual(other)));
}

MultiVector multi_vector__project_orthogonally_onto__multi_vector(MultiVector self, MultiVector other) {
    return multi_vector__anti_wedge__multi_vector(other, multi_vector__wedge__multi_vector(self, multi_vector__anti_dual(other)));
}

MultiVector multi_vector__project_orthogonally_onto__plane(MultiVector self, Plane other) {
    return plane__anti_wedge__multi_vector(other, multi_vector__wedge__round_point(self, plane__anti_dual(other)));
}

MultiVector multi_vector__project_orthogonally_onto__round_point(MultiVector self, RoundPoint other) {
    return round_point__anti_wedge__multi_vector(other, multi_vector__wedge__sphere(self, round_point__anti_dual(other)));
}

MultiVector multi_vector__project_orthogonally_onto__sphere(MultiVector self, Sphere other) {
    return sphere__anti_wedge__multi_vector(other, multi_vector__wedge__round_point(self, sphere__anti_dual(other)));
}

MultiVector plane__project_orthogonally_onto__flector(Plane self, Flector other) {
    return flector__anti_wedge__multi_vector(other, plane__wedge__multi_vector(self, flector__anti_dual(other)));
}

MultiVector plane__project_orthogonally_onto__motor(Plane self, Motor other) {
    return motor__anti_wedge__multi_vector(other, plane__wedge__multi_vector(self, motor__anti_dual(other)));
}

MultiVector plane__project_orthogonally_onto__multi_vector(Plane self, MultiVector other) {
    return multi_vector__anti_wedge__multi_vector(other, plane__wedge__multi_vector(self, multi_vector__anti_dual(other)));
}

Plane plane__project_orthogonally_onto__plane(Plane self, Plane other) {
    return plane__anti_wedge__anti_scalar(other, plane__wedge__round_point(self, plane__anti_dual(other)));
}

Sphere plane__project_orthogonally_onto__sphere(Plane self, Sphere other) {
    return sphere__anti_wedge__anti_scalar(other, plane__wedge__round_point(self, sphere__anti_dual(other)));
}

RoundPoint round_point__project_orthogonally_onto__circle(RoundPoint self, Circle other) {
    return circle__anti_wedge__circle(other, round_point__wedge__dipole(self, circle__anti_dual(other)));
}

RoundPoint round_point__project_orthogonally_onto__dipole(RoundPoint self, Dipole other) {
    return dipole__anti_wedge__sphere(other, round_point__wedge__circle(self, dipole__anti_dual(other)));
}

RoundPoint round_point__project_orthogonally_onto__flat_point(RoundPoint self, FlatPoint other) {
    return flat_point__anti_wedge__sphere(other, round_point__wedge__circle(self, flat_point__anti_dual(other)));
}

MultiVector round_point__project_orthogonally_onto__flector(RoundPoint self, Flector other) {
    return flector__anti_wedge__multi_vector(other, round_point__wedge__multi_vector(self, flector__anti_dual(other)));
}

RoundPoint round_point__project_orthogonally_onto__line(RoundPoint self, Line other) {
    return line__anti_wedge__circle(other, round_point__wedge__dipole(self, line__anti_dual(other)));
}

MultiVector round_point__project_orthogonally_onto__motor(RoundPoint self, Motor other) {
    return motor__anti_wedge__multi_vector(other, round_point__wedge__multi_vector(self, motor__anti_dual(other)));
}

MultiVector round_point__project_orthogonally_onto__multi_vector(RoundPoint self, MultiVector other) {
    return multi_vector__anti_wedge__multi_vector(other, round_point__wedge__multi_vector(self, multi_vector__anti_dual(other)));
}

RoundPoint round_point__project_orthogonally_onto__plane(RoundPoint self, Plane other) {
    return plane__anti_wedge__dipole(other, round_point__wedge__round_point(self, plane__anti_dual(other)));
}

RoundPoint round_point__project_orthogonally_onto__round_point(RoundPoint self, RoundPoint other) {
    return round_point__anti_wedge__anti_scalar(other, round_point__wedge__sphere(self, round_point__anti_dual(other)));
}

RoundPoint round_point__project_orthogonally_onto__sphere(RoundPoint self, Sphere other) {
    return sphere__anti_wedge__dipole(other, round_point__wedge__round_point(self, sphere__anti_dual(other)));
}

MultiVector sphere__project_orthogonally_onto__flector(Sphere self, Flector other) {
    return flector__anti_wedge__multi_vector(other, sphere__wedge__multi_vector(self, flector__anti_dual(other)));
}

MultiVector sphere__project_orthogonally_onto__motor(Sphere self, Motor other) {
    return motor__anti_wedge__multi_vector(other, sphere__wedge__multi_vector(self, motor__anti_dual(other)));
}

MultiVector sphere__project_orthogonally_onto__multi_vector(Sphere self, MultiVector other) {
    return multi_vector__anti_wedge__multi_vector(other, sphere__wedge__multi_vector(self, multi_vector__anti_dual(other)));
}

Plane sphere__project_orthogonally_onto__plane(Sphere self, Plane other) {
    return plane__anti_wedge__anti_scalar(other, sphere__wedge__round_point(self, plane__anti_dual(other)));
}

Sphere sphere__project_orthogonally_onto__sphere(Sphere self, Sphere other) {
    return sphere__anti_wedge__anti_scalar(other, sphere__wedge__round_point(self, sphere__anti_dual(other)));
}

Circle circle__project_via_origin_onto__circle(Circle self, Circle other) {
    return circle__anti_wedge__anti_scalar(other, circle__wedge__dipole(self, circle__dual(other)));
}

MultiVector circle__project_via_origin_onto__flector(Circle self, Flector other) {
    return flector__anti_wedge__multi_vector(other, circle__wedge__multi_vector(self, flector__dual(other)));
}

Line circle__project_via_origin_onto__line(Circle self, Line other) {
    return line__anti_wedge__anti_scalar(other, circle__wedge__dipole(self, line__dual(other)));
}

MultiVector circle__project_via_origin_onto__motor(Circle self, Motor other) {
    return motor__anti_wedge__multi_vector(other, circle__wedge__multi_vector(self, motor__dual(other)));
}

MultiVector circle__project_via_origin_onto__multi_vector(Circle self, MultiVector other) {
    return multi_vector__anti_wedge__multi_vector(other, circle__wedge__multi_vector(self, multi_vector__dual(other)));
}

Circle circle__project_via_origin_onto__plane(Circle self, Plane other) {
    return plane__anti_wedge__sphere(other, circle__wedge__round_point(self, plane__dual(other)));
}

Circle circle__project_via_origin_onto__sphere(Circle self, Sphere other) {
    return sphere__anti_wedge__sphere(other, circle__wedge__round_point(self, sphere__dual(other)));
}

Dipole dipole__project_via_origin_onto__circle(Dipole self, Circle other) {
    return circle__anti_wedge__sphere(other, dipole__wedge__dipole(self, circle__dual(other)));
}

Dipole dipole__project_via_origin_onto__dipole(Dipole self, Dipole other) {
    return dipole__anti_wedge__anti_scalar(other, dipole__wedge__circle(self, dipole__dual(other)));
}

FlatPoint dipole__project_via_origin_onto__flat_point(Dipole self, FlatPoint other) {
    return flat_point__anti_wedge__anti_scalar(other, dipole__wedge__circle(self, flat_point__dual(other)));
}

MultiVector dipole__project_via_origin_onto__flector(Dipole self, Flector other) {
    return flector__anti_wedge__multi_vector(other, dipole__wedge__multi_vector(self, flector__dual(other)));
}

Dipole dipole__project_via_origin_onto__line(Dipole self, Line other) {
    return line__anti_wedge__sphere(other, dipole__wedge__dipole(self, line__dual(other)));
}

MultiVector dipole__project_via_origin_onto__motor(Dipole self, Motor other) {
    return motor__anti_wedge__multi_vector(other, dipole__wedge__multi_vector(self, motor__dual(other)));
}

MultiVector dipole__project_via_origin_onto__multi_vector(Dipole self, MultiVector other) {
    return multi_vector__anti_wedge__multi_vector(other, dipole__wedge__multi_vector(self, multi_vector__dual(other)));
}

Dipole dipole__project_via_origin_onto__plane(Dipole self, Plane other) {
    return plane__anti_wedge__circle(other, dipole__wedge__round_point(self, plane__dual(other)));
}

Dipole dipole__project_via_origin_onto__sphere(Dipole self, Sphere other) {
    return sphere__anti_wedge__circle(other, dipole__wedge__round_point(self, sphere__dual(other)));
}

Dipole flat_point__project_via_origin_onto__circle(FlatPoint self, Circle other) {
    return circle__anti_wedge__plane(other, flat_point__wedge__dipole(self, circle__dual(other)));
}

Dipole flat_point__project_via_origin_onto__dipole(FlatPoint self, Dipole other) {
    return dipole__anti_wedge__anti_scalar(other, flat_point__wedge__circle(self, dipole__dual(other)));
}

FlatPoint flat_point__project_via_origin_onto__flat_point(FlatPoint self, FlatPoint other) {
    return flat_point__anti_wedge__anti_scalar(other, flat_point__wedge__circle(self, flat_point__dual(other)));
}

MultiVector flat_point__project_via_origin_onto__flector(FlatPoint self, Flector other) {
    return flector__anti_wedge__multi_vector(other, flat_point__wedge__multi_vector(self, flector__dual(other)));
}

FlatPoint flat_point__project_via_origin_onto__line(FlatPoint self, Line other) {
    return line__anti_wedge__plane(other, flat_point__wedge__dipole(self, line__dual(other)));
}

MultiVector flat_point__project_via_origin_onto__motor(FlatPoint self, Motor other) {
    return motor__anti_wedge__multi_vector(other, flat_point__wedge__multi_vector(self, motor__dual(other)));
}

MultiVector flat_point__project_via_origin_onto__multi_vector(FlatPoint self, MultiVector other) {
    return multi_vector__anti_wedge__multi_vector(other, flat_point__wedge__multi_vector(self, multi_vector__dual(other)));
}

FlatPoint flat_point__project_via_origin_onto__plane(FlatPoint self, Plane other) {
    return plane__anti_wedge__line(other, flat_point__wedge__round_point(self, plane__dual(other)));
}

Dipole flat_point__project_via_origin_onto__sphere(FlatPoint self, Sphere other) {
    return sphere__anti_wedge__line(other, flat_point__wedge__round_point(self, sphere__dual(other)));
}

Dipole flector__project_via_origin_onto__circle(Flector self, Circle other) {
    return circle__anti_wedge__plane(other, flector__wedge__dipole(self, circle__dual(other)));
}

Dipole flector__project_via_origin_onto__dipole(Flector self, Dipole other) {
    return dipole__anti_wedge__anti_scalar(other, flector__wedge__circle(self, dipole__dual(other)));
}

FlatPoint flector__project_via_origin_onto__flat_point(Flector self, FlatPoint other) {
    return flat_point__anti_wedge__anti_scalar(other, flector__wedge__circle(self, flat_point__dual(other)));
}

MultiVector flector__project_via_origin_onto__flector(Flector self, Flector other) {
    return flector__anti_wedge__multi_vector(other, flector__wedge__multi_vector(self, flector__dual(other)));
}

FlatPoint flector__project_via_origin_onto__line(Flector self, Line other) {
    return line__anti_wedge__plane(other, flector__wedge__dipole(self, line__dual(other)));
}

MultiVector flector__project_via_origin_onto__motor(Flector self, Motor other) {
    return motor__anti_wedge__multi_vector(other, flector__wedge__multi_vector(self, motor__dual(other)));
}

MultiVector flector__project_via_origin_onto__multi_vector(Flector self, MultiVector other) {
    return multi_vector__anti_wedge__multi_vector(other, flector__wedge__multi_vector(self, multi_vector__dual(other)));
}

Flector flector__project_via_origin_onto__plane(Flector self, Plane other) {
    return plane__anti_wedge__motor(other, flector__wedge__round_point(self, plane__dual(other)));
}

MultiVector flector__project_via_origin_onto__sphere(Flector self, Sphere other) {
    return sphere__anti_wedge__motor(other, flector__wedge__round_point(self, sphere__dual(other)));
}

Circle line__project_via_origin_onto__circle(Line self, Circle other) {
    return circle__anti_wedge__anti_scalar(other, line__wedge__dipole(self, circle__dual(other)));
}

MultiVector line__project_via_origin_onto__flector(Line self, Flector other) {
    return flector__anti_wedge__multi_vector(other, line__wedge__multi_vector(self, flector__dual(other)));
}

Line line__project_via_origin_onto__line(Line self, Line other) {
    return line__anti_wedge__anti_scalar(other, line__wedge__dipole(self, line__dual(other)));
}

MultiVector line__project_via_origin_onto__motor(Line self, Motor other) {
    return motor__anti_wedge__multi_vector(other, line__wedge__multi_vector(self, motor__dual(other)));
}

MultiVector line__project_via_origin_onto__multi_vector(Line self, MultiVector other) {
    return multi_vector__anti_wedge__multi_vector(other, line__wedge__multi_vector(self, multi_vector__dual(other)));
}

Line line__project_via_origin_onto__plane(Line self, Plane other) {
    return plane__anti_wedge__plane(other, line__wedge__round_point(self, plane__dual(other)));
}

Circle line__project_via_origin_onto__sphere(Line self, Sphere other) {
    return sphere__anti_wedge__plane(other, line__wedge__round_point(self, sphere__dual(other)));
}

Circle motor__project_via_origin_onto__circle(Motor self, Circle other) {
    return circle__anti_wedge__anti_scalar(other, motor__wedge__dipole(self, circle__dual(other)));
}

MultiVector motor__project_via_origin_onto__flector(Motor self, Flector other) {
    return flector__anti_wedge__multi_vector(other, motor__wedge__multi_vector(self, flector__dual(other)));
}

Line motor__project_via_origin_onto__line(Motor self, Line other) {
    return line__anti_wedge__anti_scalar(other, motor__wedge__dipole(self, line__dual(other)));
}

MultiVector motor__project_via_origin_onto__motor(Motor self, Motor other) {
    return motor__anti_wedge__multi_vector(other, motor__wedge__multi_vector(self, motor__dual(other)));
}

MultiVector motor__project_via_origin_onto__multi_vector(Motor self, MultiVector other) {
    return multi_vector__anti_wedge__multi_vector(other, motor__wedge__multi_vector(self, multi_vector__dual(other)));
}

Line motor__project_via_origin_onto__plane(Motor self, Plane other) {
    return plane__anti_wedge__plane(other, motor__wedge__round_point(self, plane__dual(other)));
}

Circle motor__project_via_origin_onto__sphere(Motor self, Sphere other) {
    return sphere__anti_wedge__plane(other, motor__wedge__round_point(self, sphere__dual(other)));
}

MultiVector multi_vector__project_via_origin_onto__circle(MultiVector self, Circle other) {
    return circle__anti_wedge__multi_vector(other, multi_vector__wedge__dipole(self, circle__dual(other)));
}

MultiVector multi_vector__project_via_origin_onto__dipole(MultiVector self, Dipole other) {
    return dipole__anti_wedge__multi_vector(other, multi_vector__wedge__circle(self, dipole__dual(other)));
}

MultiVector multi_vector__project_via_origin_onto__flat_point(MultiVector self, FlatPoint other) {
    return flat_point__anti_wedge__multi_vector(other, multi_vector__wedge__circle(self, flat_point__dual(other)));
}

MultiVector multi_vector__project_via_origin_onto__flector(MultiVector self, Flector other) {
    return flector__anti_wedge__multi_vector(other, multi_vector__wedge__multi_vector(self, flector__dual(other)));
}

MultiVector multi_vector__project_via_origin_onto__line(MultiVector self, Line other) {
    return line__anti_wedge__multi_vector(other, multi_vector__wedge__dipole(self, line__dual(other)));
}

MultiVector multi_vector__project_via_origin_onto__motor(MultiVector self, Motor other) {
    return motor__anti_wedge__multi_vector(other, multi_vector__wedge__multi_vector(self, motor__dual(other)));
}

MultiVector multi_vector__project_via_origin_onto__multi_vector(MultiVector self, MultiVector other) {
    return multi_vector__anti_wedge__multi_vector(other, multi_vector__wedge__multi_vector(self, multi_vector__dual(other)));
}

MultiVector multi_vector__project_via_origin_onto__plane(MultiVector self, Plane other) {
    return plane__anti_wedge__multi_vector(other, multi_vector__wedge__round_point(self, plane__dual(other)));
}

MultiVector multi_vector__project_via_origin_onto__round_point(MultiVector self, RoundPoint other) {
    return round_point__anti_wedge__multi_vector(other, multi_vector__wedge__sphere(self, round_point__dual(other)));
}

MultiVector multi_vector__project_via_origin_onto__sphere(MultiVector self, Sphere other) {
    return sphere__anti_wedge__multi_vector(other, multi_vector__wedge__round_point(self, sphere__dual(other)));
}

MultiVector plane__project_via_origin_onto__flector(Plane self, Flector other) {
    return flector__anti_wedge__multi_vector(other, plane__wedge__multi_vector(self, flector__dual(other)));
}

MultiVector plane__project_via_origin_onto__motor(Plane self, Motor other) {
    return motor__anti_wedge__multi_vector(other, plane__wedge__multi_vector(self, motor__dual(other)));
}

MultiVector plane__project_via_origin_onto__multi_vector(Plane self, MultiVector other) {
    return multi_vector__anti_wedge__multi_vector(other, plane__wedge__multi_vector(self, multi_vector__dual(other)));
}

Plane plane__project_via_origin_onto__plane(Plane self, Plane other) {
    return plane__anti_wedge__anti_scalar(other, plane__wedge__round_point(self, plane__dual(other)));
}

Sphere plane__project_via_origin_onto__sphere(Plane self, Sphere other) {
    return sphere__anti_wedge__anti_scalar(other, plane__wedge__round_point(self, sphere__dual(other)));
}

RoundPoint round_point__project_via_origin_onto__circle(RoundPoint self, Circle other) {
    return circle__anti_wedge__circle(other, round_point__wedge__dipole(self, circle__dual(other)));
}

RoundPoint round_point__project_via_origin_onto__dipole(RoundPoint self, Dipole other) {
    return dipole__anti_wedge__sphere(other, round_point__wedge__circle(self, dipole__dual(other)));
}

RoundPoint round_point__project_via_origin_onto__flat_point(RoundPoint self, FlatPoint other) {
    return flat_point__anti_wedge__sphere(other, round_point__wedge__circle(self, flat_point__dual(other)));
}

MultiVector round_point__project_via_origin_onto__flector(RoundPoint self, Flector other) {
    return flector__anti_wedge__multi_vector(other, round_point__wedge__multi_vector(self, flector__dual(other)));
}

RoundPoint round_point__project_via_origin_onto__line(RoundPoint self, Line other) {
    return line__anti_wedge__circle(other, round_point__wedge__dipole(self, line__dual(other)));
}

MultiVector round_point__project_via_origin_onto__motor(RoundPoint self, Motor other) {
    return motor__anti_wedge__multi_vector(other, round_point__wedge__multi_vector(self, motor__dual(other)));
}

MultiVector round_point__project_via_origin_onto__multi_vector(RoundPoint self, MultiVector other) {
    return multi_vector__anti_wedge__multi_vector(other, round_point__wedge__multi_vector(self, multi_vector__dual(other)));
}

RoundPoint round_point__project_via_origin_onto__plane(RoundPoint self, Plane other) {
    return plane__anti_wedge__dipole(other, round_point__wedge__round_point(self, plane__dual(other)));
}

RoundPoint round_point__project_via_origin_onto__round_point(RoundPoint self, RoundPoint other) {
    return round_point__anti_wedge__anti_scalar(other, round_point__wedge__sphere(self, round_point__dual(other)));
}

RoundPoint round_point__project_via_origin_onto__sphere(RoundPoint self, Sphere other) {
    return sphere__anti_wedge__dipole(other, round_point__wedge__round_point(self, sphere__dual(other)));
}

MultiVector sphere__project_via_origin_onto__flector(Sphere self, Flector other) {
    return flector__anti_wedge__multi_vector(other, sphere__wedge__multi_vector(self, flector__dual(other)));
}

MultiVector sphere__project_via_origin_onto__motor(Sphere self, Motor other) {
    return motor__anti_wedge__multi_vector(other, sphere__wedge__multi_vector(self, motor__dual(other)));
}

MultiVector sphere__project_via_origin_onto__multi_vector(Sphere self, MultiVector other) {
    return multi_vector__anti_wedge__multi_vector(other, sphere__wedge__multi_vector(self, multi_vector__dual(other)));
}

Plane sphere__project_via_origin_onto__plane(Sphere self, Plane other) {
    return plane__anti_wedge__anti_scalar(other, sphere__wedge__round_point(self, plane__dual(other)));
}

Sphere sphere__project_via_origin_onto__sphere(Sphere self, Sphere other) {
    return sphere__anti_wedge__anti_scalar(other, sphere__wedge__round_point(self, sphere__dual(other)));
}

Circle circle__anti_reject_orthogonally_from__dipole(Circle self, Dipole other) {
    return anti_scalar__anti_wedge__circle(circle__wedge__dipole(self, other), dipole__anti_dual(other));
}

Circle circle__anti_reject_orthogonally_from__flat_point(Circle self, FlatPoint other) {
    return anti_scalar__anti_wedge__circle(circle__wedge__flat_point(self, other), flat_point__anti_dual(other));
}

MultiVector circle__anti_reject_orthogonally_from__flector(Circle self, Flector other) {
    return anti_scalar__anti_wedge__multi_vector(circle__wedge__flector(self, other), flector__anti_dual(other));
}

MultiVector circle__anti_reject_orthogonally_from__multi_vector(Circle self, MultiVector other) {
    return multi_vector__anti_wedge__multi_vector(circle__wedge__multi_vector(self, other), multi_vector__anti_dual(other));
}

Circle circle__anti_reject_orthogonally_from__round_point(Circle self, RoundPoint other) {
    return sphere__anti_wedge__sphere(circle__wedge__round_point(self, other), round_point__anti_dual(other));
}

Dipole dipole__anti_reject_orthogonally_from__circle(Dipole self, Circle other) {
    return anti_scalar__anti_wedge__dipole(dipole__wedge__circle(self, other), circle__anti_dual(other));
}

Dipole dipole__anti_reject_orthogonally_from__dipole(Dipole self, Dipole other) {
    return sphere__anti_wedge__circle(dipole__wedge__dipole(self, other), dipole__anti_dual(other));
}

Dipole dipole__anti_reject_orthogonally_from__flat_point(Dipole self, FlatPoint other) {
    return plane__anti_wedge__circle(dipole__wedge__flat_point(self, other), flat_point__anti_dual(other));
}

MultiVector dipole__anti_reject_orthogonally_from__flector(Dipole self, Flector other) {
    return plane__anti_wedge__multi_vector(dipole__wedge__flector(self, other), flector__anti_dual(other));
}

Dipole dipole__anti_reject_orthogonally_from__line(Dipole self, Line other) {
    return anti_scalar__anti_wedge__dipole(dipole__wedge__line(self, other), line__anti_dual(other));
}

MultiVector dipole__anti_reject_orthogonally_from__motor(Dipole self, Motor other) {
    return anti_scalar__anti_wedge__multi_vector(dipole__wedge__motor(self, other), motor__anti_dual(other));
}

MultiVector dipole__anti_reject_orthogonally_from__multi_vector(Dipole self, MultiVector other) {
    return multi_vector__anti_wedge__multi_vector(dipole__wedge__multi_vector(self, other), multi_vector__anti_dual(other));
}

Dipole dipole__anti_reject_orthogonally_from__round_point(Dipole self, RoundPoint other) {
    return circle__anti_wedge__sphere(dipole__wedge__round_point(self, other), round_point__anti_dual(other));
}

Dipole flat_point__anti_reject_orthogonally_from__circle(FlatPoint self, Circle other) {
    return anti_scalar__anti_wedge__dipole(flat_point__wedge__circle(self, other), circle__anti_dual(other));
}

Dipole flat_point__anti_reject_orthogonally_from__dipole(FlatPoint self, Dipole other) {
    return plane__anti_wedge__circle(flat_point__wedge__dipole(self, other), dipole__anti_dual(other));
}

MultiVector flat_point__anti_reject_orthogonally_from__multi_vector(FlatPoint self, MultiVector other) {
    return multi_vector__anti_wedge__multi_vector(flat_point__wedge__multi_vector(self, other), multi_vector__anti_dual(other));
}

Dipole flat_point__anti_reject_orthogonally_from__round_point(FlatPoint self, RoundPoint other) {
    return line__anti_wedge__sphere(flat_point__wedge__round_point(self, other), round_point__anti_dual(other));
}

Dipole flector__anti_reject_orthogonally_from__circle(Flector self, Circle other) {
    return anti_scalar__anti_wedge__dipole(flector__wedge__circle(self, other), circle__anti_dual(other));
}

Dipole flector__anti_reject_orthogonally_from__dipole(Flector self, Dipole other) {
    return plane__anti_wedge__circle(flector__wedge__dipole(self, other), dipole__anti_dual(other));
}

MultiVector flector__anti_reject_orthogonally_from__multi_vector(Flector self, MultiVector other) {
    return multi_vector__anti_wedge__multi_vector(flector__wedge__multi_vector(self, other), multi_vector__anti_dual(other));
}

MultiVector flector__anti_reject_orthogonally_from__round_point(Flector self, RoundPoint other) {
    return motor__anti_wedge__sphere(flector__wedge__round_point(self, other), round_point__anti_dual(other));
}

Circle line__anti_reject_orthogonally_from__dipole(Line self, Dipole other) {
    return anti_scalar__anti_wedge__circle(line__wedge__dipole(self, other), dipole__anti_dual(other));
}

MultiVector line__anti_reject_orthogonally_from__multi_vector(Line self, MultiVector other) {
    return multi_vector__anti_wedge__multi_vector(line__wedge__multi_vector(self, other), multi_vector__anti_dual(other));
}

Circle line__anti_reject_orthogonally_from__round_point(Line self, RoundPoint other) {
    return plane__anti_wedge__sphere(line__wedge__round_point(self, other), round_point__anti_dual(other));
}

Circle motor__anti_reject_orthogonally_from__dipole(Motor self, Dipole other) {
    return anti_scalar__anti_wedge__circle(motor__wedge__dipole(self, other), dipole__anti_dual(other));
}

MultiVector motor__anti_reject_orthogonally_from__multi_vector(Motor self, MultiVector other) {
    return multi_vector__anti_wedge__multi_vector(motor__wedge__multi_vector(self, other), multi_vector__anti_dual(other));
}

Circle motor__anti_reject_orthogonally_from__round_point(Motor self, RoundPoint other) {
    return plane__anti_wedge__sphere(motor__wedge__round_point(self, other), round_point__anti_dual(other));
}

MultiVector multi_vector__anti_reject_orthogonally_from__circle(MultiVector self, Circle other) {
    return multi_vector__anti_wedge__dipole(multi_vector__wedge__circle(self, other), circle__anti_dual(other));
}

MultiVector multi_vector__anti_reject_orthogonally_from__dipole(MultiVector self, Dipole other) {
    return multi_vector__anti_wedge__circle(multi_vector__wedge__dipole(self, other), dipole__anti_dual(other));
}

MultiVector multi_vector__anti_reject_orthogonally_from__flat_point(MultiVector self, FlatPoint other) {
    return multi_vector__anti_wedge__circle(multi_vector__wedge__flat_point(self, other), flat_point__anti_dual(other));
}

MultiVector multi_vector__anti_reject_orthogonally_from__flector(MultiVector self, Flector other) {
    return multi_vector__anti_wedge__multi_vector(multi_vector__wedge__flector(self, other), flector__anti_dual(other));
}

MultiVector multi_vector__anti_reject_orthogonally_from__line(MultiVector self, Line other) {
    return multi_vector__anti_wedge__dipole(multi_vector__wedge__line(self, other), line__anti_dual(other));
}

MultiVector multi_vector__anti_reject_orthogonally_from__motor(MultiVector self, Motor other) {
    return multi_vector__anti_wedge__multi_vector(multi_vector__wedge__motor(self, other), motor__anti_dual(other));
}

MultiVector multi_vector__anti_reject_orthogonally_from__multi_vector(MultiVector self, MultiVector other) {
    return multi_vector__anti_wedge__multi_vector(multi_vector__wedge__multi_vector(self, other), multi_vector__anti_dual(other));
}

MultiVector multi_vector__anti_reject_orthogonally_from__plane(MultiVector self, Plane other) {
    return multi_vector__anti_wedge__round_point(multi_vector__wedge__plane(self, other), plane__anti_dual(other));
}

MultiVector multi_vector__anti_reject_orthogonally_from__round_point(MultiVector self, RoundPoint other) {
    return multi_vector__anti_wedge__sphere(multi_vector__wedge__round_point(self, other), round_point__anti_dual(other));
}

MultiVector multi_vector__anti_reject_orthogonally_from__sphere(MultiVector self, Sphere other) {
    return multi_vector__anti_wedge__round_point(multi_vector__wedge__sphere(self, other), sphere__anti_dual(other));
}

MultiVector plane__anti_reject_orthogonally_from__multi_vector(Plane self, MultiVector other) {
    return multi_vector__anti_wedge__multi_vector(plane__wedge__multi_vector(self, other), multi_vector__anti_dual(other));
}

Sphere plane__anti_reject_orthogonally_from__round_point(Plane self, RoundPoint other) {
    return anti_scalar__anti_wedge__sphere(plane__wedge__round_point(self, other), round_point__anti_dual(other));
}

RoundPoint round_point__anti_reject_orthogonally_from__circle(RoundPoint self, Circle other) {
    return sphere__anti_wedge__dipole(round_point__wedge__circle(self, other), circle__anti_dual(other));
}

RoundPoint round_point__anti_reject_orthogonally_from__dipole(RoundPoint self, Dipole other) {
    return circle__anti_wedge__circle(round_point__wedge__dipole(self, other), dipole__anti_dual(other));
}

RoundPoint round_point__anti_reject_orthogonally_from__flat_point(RoundPoint self, FlatPoint other) {
    return line__anti_wedge__circle(round_point__wedge__flat_point(self, other), flat_point__anti_dual(other));
}

MultiVector round_point__anti_reject_orthogonally_from__flector(RoundPoint self, Flector other) {
    return motor__anti_wedge__multi_vector(round_point__wedge__flector(self, other), flector__anti_dual(other));
}

RoundPoint round_point__anti_reject_orthogonally_from__line(RoundPoint self, Line other) {
    return plane__anti_wedge__dipole(round_point__wedge__line(self, other), line__anti_dual(other));
}

MultiVector round_point__anti_reject_orthogonally_from__motor(RoundPoint self, Motor other) {
    return plane__anti_wedge__multi_vector(round_point__wedge__motor(self, other), motor__anti_dual(other));
}

MultiVector round_point__anti_reject_orthogonally_from__multi_vector(RoundPoint self, MultiVector other) {
    return multi_vector__anti_wedge__multi_vector(round_point__wedge__multi_vector(self, other), multi_vector__anti_dual(other));
}

RoundPoint round_point__anti_reject_orthogonally_from__plane(RoundPoint self, Plane other) {
    return anti_scalar__anti_wedge__round_point(round_point__wedge__plane(self, other), plane__anti_dual(other));
}

RoundPoint round_point__anti_reject_orthogonally_from__round_point(RoundPoint self, RoundPoint other) {
    return dipole__anti_wedge__sphere(round_point__wedge__round_point(self, other), round_point__anti_dual(other));
}

RoundPoint round_point__anti_reject_orthogonally_from__sphere(RoundPoint self, Sphere other) {
    return anti_scalar__anti_wedge__round_point(round_point__wedge__sphere(self, other), sphere__anti_dual(other));
}

MultiVector sphere__anti_reject_orthogonally_from__multi_vector(Sphere self, MultiVector other) {
    return multi_vector__anti_wedge__multi_vector(sphere__wedge__multi_vector(self, other), multi_vector__anti_dual(other));
}

Sphere sphere__anti_reject_orthogonally_from__round_point(Sphere self, RoundPoint other) {
    return anti_scalar__anti_wedge__sphere(sphere__wedge__round_point(self, other), round_point__anti_dual(other));
}

Circle circle__anti_reject_via_horizon_from__dipole(Circle self, Dipole other) {
    return anti_scalar__anti_wedge__circle(circle__wedge__dipole(self, other), dipole__dual(other));
}

Circle circle__anti_reject_via_horizon_from__flat_point(Circle self, FlatPoint other) {
    return anti_scalar__anti_wedge__circle(circle__wedge__flat_point(self, other), flat_point__dual(other));
}

MultiVector circle__anti_reject_via_horizon_from__flector(Circle self, Flector other) {
    return anti_scalar__anti_wedge__multi_vector(circle__wedge__flector(self, other), flector__dual(other));
}

MultiVector circle__anti_reject_via_horizon_from__multi_vector(Circle self, MultiVector other) {
    return multi_vector__anti_wedge__multi_vector(circle__wedge__multi_vector(self, other), multi_vector__dual(other));
}

Circle circle__anti_reject_via_horizon_from__round_point(Circle self, RoundPoint other) {
    return sphere__anti_wedge__sphere(circle__wedge__round_point(self, other), round_point__dual(other));
}

Dipole dipole__anti_reject_via_horizon_from__circle(Dipole self, Circle other) {
    return anti_scalar__anti_wedge__dipole(dipole__wedge__circle(self, other), circle__dual(other));
}

Dipole dipole__anti_reject_via_horizon_from__dipole(Dipole self, Dipole other) {
    return sphere__anti_wedge__circle(dipole__wedge__dipole(self, other), dipole__dual(other));
}

Dipole dipole__anti_reject_via_horizon_from__flat_point(Dipole self, FlatPoint other) {
    return plane__anti_wedge__circle(dipole__wedge__flat_point(self, other), flat_point__dual(other));
}

MultiVector dipole__anti_reject_via_horizon_from__flector(Dipole self, Flector other) {
    return plane__anti_wedge__multi_vector(dipole__wedge__flector(self, other), flector__dual(other));
}

Dipole dipole__anti_reject_via_horizon_from__line(Dipole self, Line other) {
    return anti_scalar__anti_wedge__dipole(dipole__wedge__line(self, other), line__dual(other));
}

MultiVector dipole__anti_reject_via_horizon_from__motor(Dipole self, Motor other) {
    return anti_scalar__anti_wedge__multi_vector(dipole__wedge__motor(self, other), motor__dual(other));
}

MultiVector dipole__anti_reject_via_horizon_from__multi_vector(Dipole self, MultiVector other) {
    return multi_vector__anti_wedge__multi_vector(dipole__wedge__multi_vector(self, other), multi_vector__dual(other));
}

Dipole dipole__anti_reject_via_horizon_from__round_point(Dipole self, RoundPoint other) {
    return circle__anti_wedge__sphere(dipole__wedge__round_point(self, other), round_point__dual(other));
}

Dipole flat_point__anti_reject_via_horizon_from__circle(FlatPoint self, Circle other) {
    return anti_scalar__anti_wedge__dipole(flat_point__wedge__circle(self, other), circle__dual(other));
}

Dipole flat_point__anti_reject_via_horizon_from__dipole(FlatPoint self, Dipole other) {
    return plane__anti_wedge__circle(flat_point__wedge__dipole(self, other), dipole__dual(other));
}

MultiVector flat_point__anti_reject_via_horizon_from__multi_vector(FlatPoint self, MultiVector other) {
    return multi_vector__anti_wedge__multi_vector(flat_point__wedge__multi_vector(self, other), multi_vector__dual(other));
}

Dipole flat_point__anti_reject_via_horizon_from__round_point(FlatPoint self, RoundPoint other) {
    return line__anti_wedge__sphere(flat_point__wedge__round_point(self, other), round_point__dual(other));
}

Dipole flector__anti_reject_via_horizon_from__circle(Flector self, Circle other) {
    return anti_scalar__anti_wedge__dipole(flector__wedge__circle(self, other), circle__dual(other));
}

Dipole flector__anti_reject_via_horizon_from__dipole(Flector self, Dipole other) {
    return plane__anti_wedge__circle(flector__wedge__dipole(self, other), dipole__dual(other));
}

MultiVector flector__anti_reject_via_horizon_from__multi_vector(Flector self, MultiVector other) {
    return multi_vector__anti_wedge__multi_vector(flector__wedge__multi_vector(self, other), multi_vector__dual(other));
}

MultiVector flector__anti_reject_via_horizon_from__round_point(Flector self, RoundPoint other) {
    return motor__anti_wedge__sphere(flector__wedge__round_point(self, other), round_point__dual(other));
}

Circle line__anti_reject_via_horizon_from__dipole(Line self, Dipole other) {
    return anti_scalar__anti_wedge__circle(line__wedge__dipole(self, other), dipole__dual(other));
}

MultiVector line__anti_reject_via_horizon_from__multi_vector(Line self, MultiVector other) {
    return multi_vector__anti_wedge__multi_vector(line__wedge__multi_vector(self, other), multi_vector__dual(other));
}

Circle line__anti_reject_via_horizon_from__round_point(Line self, RoundPoint other) {
    return plane__anti_wedge__sphere(line__wedge__round_point(self, other), round_point__dual(other));
}

Circle motor__anti_reject_via_horizon_from__dipole(Motor self, Dipole other) {
    return anti_scalar__anti_wedge__circle(motor__wedge__dipole(self, other), dipole__dual(other));
}

MultiVector motor__anti_reject_via_horizon_from__multi_vector(Motor self, MultiVector other) {
    return multi_vector__anti_wedge__multi_vector(motor__wedge__multi_vector(self, other), multi_vector__dual(other));
}

Circle motor__anti_reject_via_horizon_from__round_point(Motor self, RoundPoint other) {
    return plane__anti_wedge__sphere(motor__wedge__round_point(self, other), round_point__dual(other));
}

MultiVector multi_vector__anti_reject_via_horizon_from__circle(MultiVector self, Circle other) {
    return multi_vector__anti_wedge__dipole(multi_vector__wedge__circle(self, other), circle__dual(other));
}

MultiVector multi_vector__anti_reject_via_horizon_from__dipole(MultiVector self, Dipole other) {
    return multi_vector__anti_wedge__circle(multi_vector__wedge__dipole(self, other), dipole__dual(other));
}

MultiVector multi_vector__anti_reject_via_horizon_from__flat_point(MultiVector self, FlatPoint other) {
    return multi_vector__anti_wedge__circle(multi_vector__wedge__flat_point(self, other), flat_point__dual(other));
}

MultiVector multi_vector__anti_reject_via_horizon_from__flector(MultiVector self, Flector other) {
    return multi_vector__anti_wedge__multi_vector(multi_vector__wedge__flector(self, other), flector__dual(other));
}

MultiVector multi_vector__anti_reject_via_horizon_from__line(MultiVector self, Line other) {
    return multi_vector__anti_wedge__dipole(multi_vector__wedge__line(self, other), line__dual(other));
}

MultiVector multi_vector__anti_reject_via_horizon_from__motor(MultiVector self, Motor other) {
    return multi_vector__anti_wedge__multi_vector(multi_vector__wedge__motor(self, other), motor__dual(other));
}

MultiVector multi_vector__anti_reject_via_horizon_from__multi_vector(MultiVector self, MultiVector other) {
    return multi_vector__anti_wedge__multi_vector(multi_vector__wedge__multi_vector(self, other), multi_vector__dual(other));
}

MultiVector multi_vector__anti_reject_via_horizon_from__plane(MultiVector self, Plane other) {
    return multi_vector__anti_wedge__round_point(multi_vector__wedge__plane(self, other), plane__dual(other));
}

MultiVector multi_vector__anti_reject_via_horizon_from__round_point(MultiVector self, RoundPoint other) {
    return multi_vector__anti_wedge__sphere(multi_vector__wedge__round_point(self, other), round_point__dual(other));
}

MultiVector multi_vector__anti_reject_via_horizon_from__sphere(MultiVector self, Sphere other) {
    return multi_vector__anti_wedge__round_point(multi_vector__wedge__sphere(self, other), sphere__dual(other));
}

MultiVector plane__anti_reject_via_horizon_from__multi_vector(Plane self, MultiVector other) {
    return multi_vector__anti_wedge__multi_vector(plane__wedge__multi_vector(self, other), multi_vector__dual(other));
}

Sphere plane__anti_reject_via_horizon_from__round_point(Plane self, RoundPoint other) {
    return anti_scalar__anti_wedge__sphere(plane__wedge__round_point(self, other), round_point__dual(other));
}

RoundPoint round_point__anti_reject_via_horizon_from__circle(RoundPoint self, Circle other) {
    return sphere__anti_wedge__dipole(round_point__wedge__circle(self, other), circle__dual(other));
}

RoundPoint round_point__anti_reject_via_horizon_from__dipole(RoundPoint self, Dipole other) {
    return circle__anti_wedge__circle(round_point__wedge__dipole(self, other), dipole__dual(other));
}

RoundPoint round_point__anti_reject_via_horizon_from__flat_point(RoundPoint self, FlatPoint other) {
    return line__anti_wedge__circle(round_point__wedge__flat_point(self, other), flat_point__dual(other));
}

MultiVector round_point__anti_reject_via_horizon_from__flector(RoundPoint self, Flector other) {
    return motor__anti_wedge__multi_vector(round_point__wedge__flector(self, other), flector__dual(other));
}

RoundPoint round_point__anti_reject_via_horizon_from__line(RoundPoint self, Line other) {
    return plane__anti_wedge__dipole(round_point__wedge__line(self, other), line__dual(other));
}

MultiVector round_point__anti_reject_via_horizon_from__motor(RoundPoint self, Motor other) {
    return plane__anti_wedge__multi_vector(round_point__wedge__motor(self, other), motor__dual(other));
}

MultiVector round_point__anti_reject_via_horizon_from__multi_vector(RoundPoint self, MultiVector other) {
    return multi_vector__anti_wedge__multi_vector(round_point__wedge__multi_vector(self, other), multi_vector__dual(other));
}

RoundPoint round_point__anti_reject_via_horizon_from__plane(RoundPoint self, Plane other) {
    return anti_scalar__anti_wedge__round_point(round_point__wedge__plane(self, other), plane__dual(other));
}

RoundPoint round_point__anti_reject_via_horizon_from__round_point(RoundPoint self, RoundPoint other) {
    return dipole__anti_wedge__sphere(round_point__wedge__round_point(self, other), round_point__dual(other));
}

RoundPoint round_point__anti_reject_via_horizon_from__sphere(RoundPoint self, Sphere other) {
    return anti_scalar__anti_wedge__round_point(round_point__wedge__sphere(self, other), sphere__dual(other));
}

MultiVector sphere__anti_reject_via_horizon_from__multi_vector(Sphere self, MultiVector other) {
    return multi_vector__anti_wedge__multi_vector(sphere__wedge__multi_vector(self, other), multi_vector__dual(other));
}

Sphere sphere__anti_reject_via_horizon_from__round_point(Sphere self, RoundPoint other) {
    return anti_scalar__anti_wedge__sphere(sphere__wedge__round_point(self, other), round_point__dual(other));
}

Circle circle__reject_orthogonally_from__circle(Circle self, Circle other) {
    return round_point__wedge__dipole(circle__anti_wedge__circle(self, other), circle__anti_dual(other));
}

Circle circle__reject_orthogonally_from__dipole(Circle self, Dipole other) {
    return scalar__wedge__circle(circle__anti_wedge__dipole(self, other), dipole__anti_dual(other));
}

Circle circle__reject_orthogonally_from__flat_point(Circle self, FlatPoint other) {
    return scalar__wedge__circle(circle__anti_wedge__flat_point(self, other), flat_point__anti_dual(other));
}

MultiVector circle__reject_orthogonally_from__flector(Circle self, Flector other) {
    return multi_vector__wedge__multi_vector(circle__anti_wedge__flector(self, other), flector__anti_dual(other));
}

Circle circle__reject_orthogonally_from__line(Circle self, Line other) {
    return round_point__wedge__dipole(circle__anti_wedge__line(self, other), line__anti_dual(other));
}

MultiVector circle__reject_orthogonally_from__motor(Circle self, Motor other) {
    return multi_vector__wedge__multi_vector(circle__anti_wedge__motor(self, other), motor__anti_dual(other));
}

MultiVector circle__reject_orthogonally_from__multi_vector(Circle self, MultiVector other) {
    return multi_vector__wedge__multi_vector(circle__anti_wedge__multi_vector(self, other), multi_vector__anti_dual(other));
}

Circle circle__reject_orthogonally_from__plane(Circle self, Plane other) {
    return dipole__wedge__round_point(circle__anti_wedge__plane(self, other), plane__anti_dual(other));
}

Circle circle__reject_orthogonally_from__sphere(Circle self, Sphere other) {
    return dipole__wedge__round_point(circle__anti_wedge__sphere(self, other), sphere__anti_dual(other));
}

Dipole dipole__reject_orthogonally_from__circle(Dipole self, Circle other) {
    return scalar__wedge__dipole(dipole__anti_wedge__circle(self, other), circle__anti_dual(other));
}

MultiVector dipole__reject_orthogonally_from__flector(Dipole self, Flector other) {
    return round_point__wedge__multi_vector(dipole__anti_wedge__flector(self, other), flector__anti_dual(other));
}

Dipole dipole__reject_orthogonally_from__line(Dipole self, Line other) {
    return scalar__wedge__dipole(dipole__anti_wedge__line(self, other), line__anti_dual(other));
}

MultiVector dipole__reject_orthogonally_from__motor(Dipole self, Motor other) {
    return multi_vector__wedge__multi_vector(dipole__anti_wedge__motor(self, other), motor__anti_dual(other));
}

MultiVector dipole__reject_orthogonally_from__multi_vector(Dipole self, MultiVector other) {
    return multi_vector__wedge__multi_vector(dipole__anti_wedge__multi_vector(self, other), multi_vector__anti_dual(other));
}

Dipole dipole__reject_orthogonally_from__plane(Dipole self, Plane other) {
    return round_point__wedge__round_point(dipole__anti_wedge__plane(self, other), plane__anti_dual(other));
}

Dipole dipole__reject_orthogonally_from__sphere(Dipole self, Sphere other) {
    return round_point__wedge__round_point(dipole__anti_wedge__sphere(self, other), sphere__anti_dual(other));
}

Dipole flat_point__reject_orthogonally_from__circle(FlatPoint self, Circle other) {
    return scalar__wedge__dipole(flat_point__anti_wedge__circle(self, other), circle__anti_dual(other));
}

MultiVector flat_point__reject_orthogonally_from__flector(FlatPoint self, Flector other) {
    return round_point__wedge__multi_vector(flat_point__anti_wedge__flector(self, other), flector__anti_dual(other));
}

MultiVector flat_point__reject_orthogonally_from__motor(FlatPoint self, Motor other) {
    return flat_point__wedge__multi_vector(flat_point__anti_wedge__motor(self, other), motor__anti_dual(other));
}

MultiVector flat_point__reject_orthogonally_from__multi_vector(FlatPoint self, MultiVector other) {
    return multi_vector__wedge__multi_vector(flat_point__anti_wedge__multi_vector(self, other), multi_vector__anti_dual(other));
}

Dipole flat_point__reject_orthogonally_from__plane(FlatPoint self, Plane other) {
    return round_point__wedge__round_point(flat_point__anti_wedge__plane(self, other), plane__anti_dual(other));
}

Dipole flat_point__reject_orthogonally_from__sphere(FlatPoint self, Sphere other) {
    return round_point__wedge__round_point(flat_point__anti_wedge__sphere(self, other), sphere__anti_dual(other));
}

MultiVector flector__reject_orthogonally_from__circle(Flector self, Circle other) {
    return multi_vector__wedge__dipole(flector__anti_wedge__circle(self, other), circle__anti_dual(other));
}

Sphere flector__reject_orthogonally_from__dipole(Flector self, Dipole other) {
    return round_point__wedge__circle(flector__anti_wedge__dipole(self, other), dipole__anti_dual(other));
}

Sphere flector__reject_orthogonally_from__flat_point(Flector self, FlatPoint other) {
    return round_point__wedge__circle(flector__anti_wedge__flat_point(self, other), flat_point__anti_dual(other));
}

MultiVector flector__reject_orthogonally_from__flector(Flector self, Flector other) {
    return multi_vector__wedge__multi_vector(flector__anti_wedge__flector(self, other), flector__anti_dual(other));
}

Plane flector__reject_orthogonally_from__line(Flector self, Line other) {
    return flat_point__wedge__dipole(flector__anti_wedge__line(self, other), line__anti_dual(other));
}

MultiVector flector__reject_orthogonally_from__motor(Flector self, Motor other) {
    return flector__wedge__multi_vector(flector__anti_wedge__motor(self, other), motor__anti_dual(other));
}

MultiVector flector__reject_orthogonally_from__multi_vector(Flector self, MultiVector other) {
    return multi_vector__wedge__multi_vector(flector__anti_wedge__multi_vector(self, other), multi_vector__anti_dual(other));
}

MultiVector flector__reject_orthogonally_from__plane(Flector self, Plane other) {
    return multi_vector__wedge__round_point(flector__anti_wedge__plane(self, other), plane__anti_dual(other));
}

Sphere flector__reject_orthogonally_from__round_point(Flector self, RoundPoint other) {
    return scalar__wedge__sphere(flector__anti_wedge__round_point(self, other), round_point__anti_dual(other));
}

MultiVector flector__reject_orthogonally_from__sphere(Flector self, Sphere other) {
    return multi_vector__wedge__round_point(flector__anti_wedge__sphere(self, other), sphere__anti_dual(other));
}

Circle line__reject_orthogonally_from__circle(Line self, Circle other) {
    return round_point__wedge__dipole(line__anti_wedge__circle(self, other), circle__anti_dual(other));
}

Circle line__reject_orthogonally_from__dipole(Line self, Dipole other) {
    return scalar__wedge__circle(line__anti_wedge__dipole(self, other), dipole__anti_dual(other));
}

MultiVector line__reject_orthogonally_from__flector(Line self, Flector other) {
    return flat_point__wedge__multi_vector(line__anti_wedge__flector(self, other), flector__anti_dual(other));
}

Circle line__reject_orthogonally_from__line(Line self, Line other) {
    return round_point__wedge__dipole(line__anti_wedge__line(self, other), line__anti_dual(other));
}

MultiVector line__reject_orthogonally_from__motor(Line self, Motor other) {
    return multi_vector__wedge__multi_vector(line__anti_wedge__motor(self, other), motor__anti_dual(other));
}

MultiVector line__reject_orthogonally_from__multi_vector(Line self, MultiVector other) {
    return multi_vector__wedge__multi_vector(line__anti_wedge__multi_vector(self, other), multi_vector__anti_dual(other));
}

Line line__reject_orthogonally_from__plane(Line self, Plane other) {
    return flat_point__wedge__round_point(line__anti_wedge__plane(self, other), plane__anti_dual(other));
}

Circle line__reject_orthogonally_from__sphere(Line self, Sphere other) {
    return dipole__wedge__round_point(line__anti_wedge__sphere(self, other), sphere__anti_dual(other));
}

MultiVector motor__reject_orthogonally_from__circle(Motor self, Circle other) {
    return multi_vector__wedge__dipole(motor__anti_wedge__circle(self, other), circle__anti_dual(other));
}

MultiVector motor__reject_orthogonally_from__dipole(Motor self, Dipole other) {
    return multi_vector__wedge__circle(motor__anti_wedge__dipole(self, other), dipole__anti_dual(other));
}

AntiScalar motor__reject_orthogonally_from__flat_point(Motor self, FlatPoint other) {
    return flat_point__wedge__circle(motor__anti_wedge__flat_point(self, other), flat_point__anti_dual(other));
}

MultiVector motor__reject_orthogonally_from__flector(Motor self, Flector other) {
    return flector__wedge__multi_vector(motor__anti_wedge__flector(self, other), flector__anti_dual(other));
}

MultiVector motor__reject_orthogonally_from__line(Motor self, Line other) {
    return multi_vector__wedge__dipole(motor__anti_wedge__line(self, other), line__anti_dual(other));
}

MultiVector motor__reject_orthogonally_from__motor(Motor self, Motor other) {
    return multi_vector__wedge__multi_vector(motor__anti_wedge__motor(self, other), motor__anti_dual(other));
}

MultiVector motor__reject_orthogonally_from__multi_vector(Motor self, MultiVector other) {
    return multi_vector__wedge__multi_vector(motor__anti_wedge__multi_vector(self, other), multi_vector__anti_dual(other));
}

Motor motor__reject_orthogonally_from__plane(Motor self, Plane other) {
    return flector__wedge__round_point(motor__anti_wedge__plane(self, other), plane__anti_dual(other));
}

AntiScalar motor__reject_orthogonally_from__round_point(Motor self, RoundPoint other) {
    return round_point__wedge__sphere(motor__anti_wedge__round_point(self, other), round_point__anti_dual(other));
}

MultiVector motor__reject_orthogonally_from__sphere(Motor self, Sphere other) {
    return multi_vector__wedge__round_point(motor__anti_wedge__sphere(self, other), sphere__anti_dual(other));
}

MultiVector multi_vector__reject_orthogonally_from__circle(MultiVector self, Circle other) {
    return multi_vector__wedge__dipole(multi_vector__anti_wedge__circle(self, other), circle__anti_dual(other));
}

MultiVector multi_vector__reject_orthogonally_from__dipole(MultiVector self, Dipole other) {
    return multi_vector__wedge__circle(multi_vector__anti_wedge__dipole(self, other), dipole__anti_dual(other));
}

MultiVector multi_vector__reject_orthogonally_from__flat_point(MultiVector self, FlatPoint other) {
    return multi_vector__wedge__circle(multi_vector__anti_wedge__flat_point(self, other), flat_point__anti_dual(other));
}

MultiVector multi_vector__reject_orthogonally_from__flector(MultiVector self, Flector other) {
    return multi_vector__wedge__multi_vector(multi_vector__anti_wedge__flector(self, other), flector__anti_dual(other));
}

MultiVector multi_vector__reject_orthogonally_from__line(MultiVector self, Line other) {
    return multi_vector__wedge__dipole(multi_vector__anti_wedge__line(self, other), line__anti_dual(other));
}

MultiVector multi_vector__reject_orthogonally_from__motor(MultiVector self, Motor other) {
    return multi_vector__wedge__multi_vector(multi_vector__anti_wedge__motor(self, other), motor__anti_dual(other));
}

MultiVector multi_vector__reject_orthogonally_from__multi_vector(MultiVector self, MultiVector other) {
    return multi_vector__wedge__multi_vector(multi_vector__anti_wedge__multi_vector(self, other), multi_vector__anti_dual(other));
}

MultiVector multi_vector__reject_orthogonally_from__plane(MultiVector self, Plane other) {
    return multi_vector__wedge__round_point(multi_vector__anti_wedge__plane(self, other), plane__anti_dual(other));
}

MultiVector multi_vector__reject_orthogonally_from__round_point(MultiVector self, RoundPoint other) {
    return multi_vector__wedge__sphere(multi_vector__anti_wedge__round_point(self, other), round_point__anti_dual(other));
}

MultiVector multi_vector__reject_orthogonally_from__sphere(MultiVector self, Sphere other) {
    return multi_vector__wedge__round_point(multi_vector__anti_wedge__sphere(self, other), sphere__anti_dual(other));
}

Sphere plane__reject_orthogonally_from__circle(Plane self, Circle other) {
    return dipole__wedge__dipole(plane__anti_wedge__circle(self, other), circle__anti_dual(other));
}

Sphere plane__reject_orthogonally_from__dipole(Plane self, Dipole other) {
    return round_point__wedge__circle(plane__anti_wedge__dipole(self, other), dipole__anti_dual(other));
}

Sphere plane__reject_orthogonally_from__flat_point(Plane self, FlatPoint other) {
    return round_point__wedge__circle(plane__anti_wedge__flat_point(self, other), flat_point__anti_dual(other));
}

MultiVector plane__reject_orthogonally_from__flector(Plane self, Flector other) {
    return multi_vector__wedge__multi_vector(plane__anti_wedge__flector(self, other), flector__anti_dual(other));
}

Plane plane__reject_orthogonally_from__line(Plane self, Line other) {
    return flat_point__wedge__dipole(plane__anti_wedge__line(self, other), line__anti_dual(other));
}

MultiVector plane__reject_orthogonally_from__motor(Plane self, Motor other) {
    return flector__wedge__multi_vector(plane__anti_wedge__motor(self, other), motor__anti_dual(other));
}

MultiVector plane__reject_orthogonally_from__multi_vector(Plane self, MultiVector other) {
    return multi_vector__wedge__multi_vector(plane__anti_wedge__multi_vector(self, other), multi_vector__anti_dual(other));
}

Plane plane__reject_orthogonally_from__plane(Plane self, Plane other) {
    return line__wedge__round_point(plane__anti_wedge__plane(self, other), plane__anti_dual(other));
}

Sphere plane__reject_orthogonally_from__round_point(Plane self, RoundPoint other) {
    return scalar__wedge__sphere(plane__anti_wedge__round_point(self, other), round_point__anti_dual(other));
}

Sphere plane__reject_orthogonally_from__sphere(Plane self, Sphere other) {
    return circle__wedge__round_point(plane__anti_wedge__sphere(self, other), sphere__anti_dual(other));
}

MultiVector round_point__reject_orthogonally_from__flector(RoundPoint self, Flector other) {
    return scalar__wedge__multi_vector(round_point__anti_wedge__flector(self, other), flector__anti_dual(other));
}

MultiVector round_point__reject_orthogonally_from__motor(RoundPoint self, Motor other) {
    return round_point__wedge__multi_vector(round_point__anti_wedge__motor(self, other), motor__anti_dual(other));
}

MultiVector round_point__reject_orthogonally_from__multi_vector(RoundPoint self, MultiVector other) {
    return multi_vector__wedge__multi_vector(round_point__anti_wedge__multi_vector(self, other), multi_vector__anti_dual(other));
}

RoundPoint round_point__reject_orthogonally_from__plane(RoundPoint self, Plane other) {
    return scalar__wedge__round_point(round_point__anti_wedge__plane(self, other), plane__anti_dual(other));
}

RoundPoint round_point__reject_orthogonally_from__sphere(RoundPoint self, Sphere other) {
    return scalar__wedge__round_point(round_point__anti_wedge__sphere(self, other), sphere__anti_dual(other));
}

Sphere sphere__reject_orthogonally_from__circle(Sphere self, Circle other) {
    return dipole__wedge__dipole(sphere__anti_wedge__circle(self, other), circle__anti_dual(other));
}

Sphere sphere__reject_orthogonally_from__dipole(Sphere self, Dipole other) {
    return round_point__wedge__circle(sphere__anti_wedge__dipole(self, other), dipole__anti_dual(other));
}

Sphere sphere__reject_orthogonally_from__flat_point(Sphere self, FlatPoint other) {
    return round_point__wedge__circle(sphere__anti_wedge__flat_point(self, other), flat_point__anti_dual(other));
}

MultiVector sphere__reject_orthogonally_from__flector(Sphere self, Flector other) {
    return multi_vector__wedge__multi_vector(sphere__anti_wedge__flector(self, other), flector__anti_dual(other));
}

Sphere sphere__reject_orthogonally_from__line(Sphere self, Line other) {
    return dipole__wedge__dipole(sphere__anti_wedge__line(self, other), line__anti_dual(other));
}

MultiVector sphere__reject_orthogonally_from__motor(Sphere self, Motor other) {
    return multi_vector__wedge__multi_vector(sphere__anti_wedge__motor(self, other), motor__anti_dual(other));
}

MultiVector sphere__reject_orthogonally_from__multi_vector(Sphere self, MultiVector other) {
    return multi_vector__wedge__multi_vector(sphere__anti_wedge__multi_vector(self, other), multi_vector__anti_dual(other));
}

Sphere sphere__reject_orthogonally_from__plane(Sphere self, Plane other) {
    return circle__wedge__round_point(sphere__anti_wedge__plane(self, other), plane__anti_dual(other));
}

Sphere sphere__reject_orthogonally_from__round_point(Sphere self, RoundPoint other) {
    return scalar__wedge__sphere(sphere__anti_wedge__round_point(self, other), round_point__anti_dual(other));
}

Sphere sphere__reject_orthogonally_from__sphere(Sphere self, Sphere other) {
    return circle__wedge__round_point(sphere__anti_wedge__sphere(self, other), sphere__anti_dual(other));
}

Circle circle__reject_via_origin_from__circle(Circle self, Circle other) {
    return round_point__wedge__dipole(circle__anti_wedge__circle(self, other), circle__dual(other));
}

Circle circle__reject_via_origin_from__dipole(Circle self, Dipole other) {
    return scalar__wedge__circle(circle__anti_wedge__dipole(self, other), dipole__dual(other));
}

Circle circle__reject_via_origin_from__flat_point(Circle self, FlatPoint other) {
    return scalar__wedge__circle(circle__anti_wedge__flat_point(self, other), flat_point__dual(other));
}

MultiVector circle__reject_via_origin_from__flector(Circle self, Flector other) {
    return multi_vector__wedge__multi_vector(circle__anti_wedge__flector(self, other), flector__dual(other));
}

Circle circle__reject_via_origin_from__line(Circle self, Line other) {
    return round_point__wedge__dipole(circle__anti_wedge__line(self, other), line__dual(other));
}

MultiVector circle__reject_via_origin_from__motor(Circle self, Motor other) {
    return multi_vector__wedge__multi_vector(circle__anti_wedge__motor(self, other), motor__dual(other));
}

MultiVector circle__reject_via_origin_from__multi_vector(Circle self, MultiVector other) {
    return multi_vector__wedge__multi_vector(circle__anti_wedge__multi_vector(self, other), multi_vector__dual(other));
}

Circle circle__reject_via_origin_from__plane(Circle self, Plane other) {
    return dipole__wedge__round_point(circle__anti_wedge__plane(self, other), plane__dual(other));
}

Circle circle__reject_via_origin_from__sphere(Circle self, Sphere other) {
    return dipole__wedge__round_point(circle__anti_wedge__sphere(self, other), sphere__dual(other));
}

Dipole dipole__reject_via_origin_from__circle(Dipole self, Circle other) {
    return scalar__wedge__dipole(dipole__anti_wedge__circle(self, other), circle__dual(other));
}

MultiVector dipole__reject_via_origin_from__flector(Dipole self, Flector other) {
    return round_point__wedge__multi_vector(dipole__anti_wedge__flector(self, other), flector__dual(other));
}

Dipole dipole__reject_via_origin_from__line(Dipole self, Line other) {
    return scalar__wedge__dipole(dipole__anti_wedge__line(self, other), line__dual(other));
}

MultiVector dipole__reject_via_origin_from__motor(Dipole self, Motor other) {
    return multi_vector__wedge__multi_vector(dipole__anti_wedge__motor(self, other), motor__dual(other));
}

MultiVector dipole__reject_via_origin_from__multi_vector(Dipole self, MultiVector other) {
    return multi_vector__wedge__multi_vector(dipole__anti_wedge__multi_vector(self, other), multi_vector__dual(other));
}

Dipole dipole__reject_via_origin_from__plane(Dipole self, Plane other) {
    return round_point__wedge__round_point(dipole__anti_wedge__plane(self, other), plane__dual(other));
}

Dipole dipole__reject_via_origin_from__sphere(Dipole self, Sphere other) {
    return round_point__wedge__round_point(dipole__anti_wedge__sphere(self, other), sphere__dual(other));
}

Dipole flat_point__reject_via_origin_from__circle(FlatPoint self, Circle other) {
    return scalar__wedge__dipole(flat_point__anti_wedge__circle(self, other), circle__dual(other));
}

MultiVector flat_point__reject_via_origin_from__flector(FlatPoint self, Flector other) {
    return round_point__wedge__multi_vector(flat_point__anti_wedge__flector(self, other), flector__dual(other));
}

MultiVector flat_point__reject_via_origin_from__motor(FlatPoint self, Motor other) {
    return flat_point__wedge__multi_vector(flat_point__anti_wedge__motor(self, other), motor__dual(other));
}

MultiVector flat_point__reject_via_origin_from__multi_vector(FlatPoint self, MultiVector other) {
    return multi_vector__wedge__multi_vector(flat_point__anti_wedge__multi_vector(self, other), multi_vector__dual(other));
}

Dipole flat_point__reject_via_origin_from__plane(FlatPoint self, Plane other) {
    return round_point__wedge__round_point(flat_point__anti_wedge__plane(self, other), plane__dual(other));
}

Dipole flat_point__reject_via_origin_from__sphere(FlatPoint self, Sphere other) {
    return round_point__wedge__round_point(flat_point__anti_wedge__sphere(self, other), sphere__dual(other));
}

MultiVector flector__reject_via_origin_from__circle(Flector self, Circle other) {
    return multi_vector__wedge__dipole(flector__anti_wedge__circle(self, other), circle__dual(other));
}

Sphere flector__reject_via_origin_from__dipole(Flector self, Dipole other) {
    return round_point__wedge__circle(flector__anti_wedge__dipole(self, other), dipole__dual(other));
}

Sphere flector__reject_via_origin_from__flat_point(Flector self, FlatPoint other) {
    return round_point__wedge__circle(flector__anti_wedge__flat_point(self, other), flat_point__dual(other));
}

MultiVector flector__reject_via_origin_from__flector(Flector self, Flector other) {
    return multi_vector__wedge__multi_vector(flector__anti_wedge__flector(self, other), flector__dual(other));
}

Plane flector__reject_via_origin_from__line(Flector self, Line other) {
    return flat_point__wedge__dipole(flector__anti_wedge__line(self, other), line__dual(other));
}

MultiVector flector__reject_via_origin_from__motor(Flector self, Motor other) {
    return flector__wedge__multi_vector(flector__anti_wedge__motor(self, other), motor__dual(other));
}

MultiVector flector__reject_via_origin_from__multi_vector(Flector self, MultiVector other) {
    return multi_vector__wedge__multi_vector(flector__anti_wedge__multi_vector(self, other), multi_vector__dual(other));
}

MultiVector flector__reject_via_origin_from__plane(Flector self, Plane other) {
    return multi_vector__wedge__round_point(flector__anti_wedge__plane(self, other), plane__dual(other));
}

Sphere flector__reject_via_origin_from__round_point(Flector self, RoundPoint other) {
    return scalar__wedge__sphere(flector__anti_wedge__round_point(self, other), round_point__dual(other));
}

MultiVector flector__reject_via_origin_from__sphere(Flector self, Sphere other) {
    return multi_vector__wedge__round_point(flector__anti_wedge__sphere(self, other), sphere__dual(other));
}

Circle line__reject_via_origin_from__circle(Line self, Circle other) {
    return round_point__wedge__dipole(line__anti_wedge__circle(self, other), circle__dual(other));
}

Circle line__reject_via_origin_from__dipole(Line self, Dipole other) {
    return scalar__wedge__circle(line__anti_wedge__dipole(self, other), dipole__dual(other));
}

MultiVector line__reject_via_origin_from__flector(Line self, Flector other) {
    return flat_point__wedge__multi_vector(line__anti_wedge__flector(self, other), flector__dual(other));
}

Circle line__reject_via_origin_from__line(Line self, Line other) {
    return round_point__wedge__dipole(line__anti_wedge__line(self, other), line__dual(other));
}

MultiVector line__reject_via_origin_from__motor(Line self, Motor other) {
    return multi_vector__wedge__multi_vector(line__anti_wedge__motor(self, other), motor__dual(other));
}

MultiVector line__reject_via_origin_from__multi_vector(Line self, MultiVector other) {
    return multi_vector__wedge__multi_vector(line__anti_wedge__multi_vector(self, other), multi_vector__dual(other));
}

Line line__reject_via_origin_from__plane(Line self, Plane other) {
    return flat_point__wedge__round_point(line__anti_wedge__plane(self, other), plane__dual(other));
}

Circle line__reject_via_origin_from__sphere(Line self, Sphere other) {
    return dipole__wedge__round_point(line__anti_wedge__sphere(self, other), sphere__dual(other));
}

MultiVector motor__reject_via_origin_from__circle(Motor self, Circle other) {
    return multi_vector__wedge__dipole(motor__anti_wedge__circle(self, other), circle__dual(other));
}

MultiVector motor__reject_via_origin_from__dipole(Motor self, Dipole other) {
    return multi_vector__wedge__circle(motor__anti_wedge__dipole(self, other), dipole__dual(other));
}

AntiScalar motor__reject_via_origin_from__flat_point(Motor self, FlatPoint other) {
    return flat_point__wedge__circle(motor__anti_wedge__flat_point(self, other), flat_point__dual(other));
}

MultiVector motor__reject_via_origin_from__flector(Motor self, Flector other) {
    return flector__wedge__multi_vector(motor__anti_wedge__flector(self, other), flector__dual(other));
}

MultiVector motor__reject_via_origin_from__line(Motor self, Line other) {
    return multi_vector__wedge__dipole(motor__anti_wedge__line(self, other), line__dual(other));
}

MultiVector motor__reject_via_origin_from__motor(Motor self, Motor other) {
    return multi_vector__wedge__multi_vector(motor__anti_wedge__motor(self, other), motor__dual(other));
}

MultiVector motor__reject_via_origin_from__multi_vector(Motor self, MultiVector other) {
    return multi_vector__wedge__multi_vector(motor__anti_wedge__multi_vector(self, other), multi_vector__dual(other));
}

Motor motor__reject_via_origin_from__plane(Motor self, Plane other) {
    return flector__wedge__round_point(motor__anti_wedge__plane(self, other), plane__dual(other));
}

AntiScalar motor__reject_via_origin_from__round_point(Motor self, RoundPoint other) {
    return round_point__wedge__sphere(motor__anti_wedge__round_point(self, other), round_point__dual(other));
}

MultiVector motor__reject_via_origin_from__sphere(Motor self, Sphere other) {
    return multi_vector__wedge__round_point(motor__anti_wedge__sphere(self, other), sphere__dual(other));
}

MultiVector multi_vector__reject_via_origin_from__circle(MultiVector self, Circle other) {
    return multi_vector__wedge__dipole(multi_vector__anti_wedge__circle(self, other), circle__dual(other));
}

MultiVector multi_vector__reject_via_origin_from__dipole(MultiVector self, Dipole other) {
    return multi_vector__wedge__circle(multi_vector__anti_wedge__dipole(self, other), dipole__dual(other));
}

MultiVector multi_vector__reject_via_origin_from__flat_point(MultiVector self, FlatPoint other) {
    return multi_vector__wedge__circle(multi_vector__anti_wedge__flat_point(self, other), flat_point__dual(other));
}

MultiVector multi_vector__reject_via_origin_from__flector(MultiVector self, Flector other) {
    return multi_vector__wedge__multi_vector(multi_vector__anti_wedge__flector(self, other), flector__dual(other));
}

MultiVector multi_vector__reject_via_origin_from__line(MultiVector self, Line other) {
    return multi_vector__wedge__dipole(multi_vector__anti_wedge__line(self, other), line__dual(other));
}

MultiVector multi_vector__reject_via_origin_from__motor(MultiVector self, Motor other) {
    return multi_vector__wedge__multi_vector(multi_vector__anti_wedge__motor(self, other), motor__dual(other));
}

MultiVector multi_vector__reject_via_origin_from__multi_vector(MultiVector self, MultiVector other) {
    return multi_vector__wedge__multi_vector(multi_vector__anti_wedge__multi_vector(self, other), multi_vector__dual(other));
}

MultiVector multi_vector__reject_via_origin_from__plane(MultiVector self, Plane other) {
    return multi_vector__wedge__round_point(multi_vector__anti_wedge__plane(self, other), plane__dual(other));
}

MultiVector multi_vector__reject_via_origin_from__round_point(MultiVector self, RoundPoint other) {
    return multi_vector__wedge__sphere(multi_vector__anti_wedge__round_point(self, other), round_point__dual(other));
}

MultiVector multi_vector__reject_via_origin_from__sphere(MultiVector self, Sphere other) {
    return multi_vector__wedge__round_point(multi_vector__anti_wedge__sphere(self, other), sphere__dual(other));
}

Sphere plane__reject_via_origin_from__circle(Plane self, Circle other) {
    return dipole__wedge__dipole(plane__anti_wedge__circle(self, other), circle__dual(other));
}

Sphere plane__reject_via_origin_from__dipole(Plane self, Dipole other) {
    return round_point__wedge__circle(plane__anti_wedge__dipole(self, other), dipole__dual(other));
}

Sphere plane__reject_via_origin_from__flat_point(Plane self, FlatPoint other) {
    return round_point__wedge__circle(plane__anti_wedge__flat_point(self, other), flat_point__dual(other));
}

MultiVector plane__reject_via_origin_from__flector(Plane self, Flector other) {
    return multi_vector__wedge__multi_vector(plane__anti_wedge__flector(self, other), flector__dual(other));
}

Plane plane__reject_via_origin_from__line(Plane self, Line other) {
    return flat_point__wedge__dipole(plane__anti_wedge__line(self, other), line__dual(other));
}

MultiVector plane__reject_via_origin_from__motor(Plane self, Motor other) {
    return flector__wedge__multi_vector(plane__anti_wedge__motor(self, other), motor__dual(other));
}

MultiVector plane__reject_via_origin_from__multi_vector(Plane self, MultiVector other) {
    return multi_vector__wedge__multi_vector(plane__anti_wedge__multi_vector(self, other), multi_vector__dual(other));
}

Plane plane__reject_via_origin_from__plane(Plane self, Plane other) {
    return line__wedge__round_point(plane__anti_wedge__plane(self, other), plane__dual(other));
}

Sphere plane__reject_via_origin_from__round_point(Plane self, RoundPoint other) {
    return scalar__wedge__sphere(plane__anti_wedge__round_point(self, other), round_point__dual(other));
}

Sphere plane__reject_via_origin_from__sphere(Plane self, Sphere other) {
    return circle__wedge__round_point(plane__anti_wedge__sphere(self, other), sphere__dual(other));
}

MultiVector round_point__reject_via_origin_from__flector(RoundPoint self, Flector other) {
    return scalar__wedge__multi_vector(round_point__anti_wedge__flector(self, other), flector__dual(other));
}

MultiVector round_point__reject_via_origin_from__motor(RoundPoint self, Motor other) {
    return round_point__wedge__multi_vector(round_point__anti_wedge__motor(self, other), motor__dual(other));
}

MultiVector round_point__reject_via_origin_from__multi_vector(RoundPoint self, MultiVector other) {
    return multi_vector__wedge__multi_vector(round_point__anti_wedge__multi_vector(self, other), multi_vector__dual(other));
}

RoundPoint round_point__reject_via_origin_from__plane(RoundPoint self, Plane other) {
    return scalar__wedge__round_point(round_point__anti_wedge__plane(self, other), plane__dual(other));
}

RoundPoint round_point__reject_via_origin_from__sphere(RoundPoint self, Sphere other) {
    return scalar__wedge__round_point(round_point__anti_wedge__sphere(self, other), sphere__dual(other));
}

Sphere sphere__reject_via_origin_from__circle(Sphere self, Circle other) {
    return dipole__wedge__dipole(sphere__anti_wedge__circle(self, other), circle__dual(other));
}

Sphere sphere__reject_via_origin_from__dipole(Sphere self, Dipole other) {
    return round_point__wedge__circle(sphere__anti_wedge__dipole(self, other), dipole__dual(other));
}

Sphere sphere__reject_via_origin_from__flat_point(Sphere self, FlatPoint other) {
    return round_point__wedge__circle(sphere__anti_wedge__flat_point(self, other), flat_point__dual(other));
}

MultiVector sphere__reject_via_origin_from__flector(Sphere self, Flector other) {
    return multi_vector__wedge__multi_vector(sphere__anti_wedge__flector(self, other), flector__dual(other));
}

Sphere sphere__reject_via_origin_from__line(Sphere self, Line other) {
    return dipole__wedge__dipole(sphere__anti_wedge__line(self, other), line__dual(other));
}

MultiVector sphere__reject_via_origin_from__motor(Sphere self, Motor other) {
    return multi_vector__wedge__multi_vector(sphere__anti_wedge__motor(self, other), motor__dual(other));
}

MultiVector sphere__reject_via_origin_from__multi_vector(Sphere self, MultiVector other) {
    return multi_vector__wedge__multi_vector(sphere__anti_wedge__multi_vector(self, other), multi_vector__dual(other));
}

Sphere sphere__reject_via_origin_from__plane(Sphere self, Plane other) {
    return circle__wedge__round_point(sphere__anti_wedge__plane(self, other), plane__dual(other));
}

Sphere sphere__reject_via_origin_from__round_point(Sphere self, RoundPoint other) {
    return scalar__wedge__sphere(sphere__anti_wedge__round_point(self, other), round_point__dual(other));
}

Sphere sphere__reject_via_origin_from__sphere(Sphere self, Sphere other) {
    return circle__wedge__round_point(sphere__anti_wedge__sphere(self, other), sphere__dual(other));
}

Sphere circle__anti_support(Circle self) {
    return circle__wedge__round_point(self, sphere__anti_wedge__dipole(round_point__complement(RoundPoint(vec3(0.0), vec2(1.0, 0.0))), circle__dual(self)));
}

Sphere dipole__anti_support(Dipole self) {
    return dipole__wedge__dipole(self, sphere__anti_wedge__circle(round_point__complement(RoundPoint(vec3(0.0), vec2(1.0, 0.0))), dipole__dual(self)));
}

Sphere dual_num__anti_support(DualNum self) {
    return dual_num__wedge__sphere(self, sphere__anti_wedge__dual_num(round_point__complement(RoundPoint(vec3(0.0), vec2(1.0, 0.0))), dual_num__dual(self)));
}

Plane flat_point__anti_support(FlatPoint self) {
    return flat_point__wedge__dipole(self, sphere__anti_wedge__circle(round_point__complement(RoundPoint(vec3(0.0), vec2(1.0, 0.0))), flat_point__dual(self)));
}

MultiVector flector__anti_support(Flector self) {
    return flector__wedge__multi_vector(self, sphere__anti_wedge__multi_vector(round_point__complement(RoundPoint(vec3(0.0), vec2(1.0, 0.0))), flector__dual(self)));
}

Plane line__anti_support(Line self) {
    return line__wedge__round_point(self, sphere__anti_wedge__dipole(round_point__complement(RoundPoint(vec3(0.0), vec2(1.0, 0.0))), line__dual(self)));
}

MultiVector motor__anti_support(Motor self) {
    return motor__wedge__multi_vector(self, sphere__anti_wedge__multi_vector(round_point__complement(RoundPoint(vec3(0.0), vec2(1.0, 0.0))), motor__dual(self)));
}

MultiVector multi_vector__anti_support(MultiVector self) {
    return multi_vector__wedge__multi_vector(self, sphere__anti_wedge__multi_vector(round_point__complement(RoundPoint(vec3(0.0), vec2(1.0, 0.0))), multi_vector__dual(self)));
}

Plane plane__anti_support(Plane self) {
    return plane__wedge__scalar(self, sphere__anti_wedge__round_point(round_point__complement(RoundPoint(vec3(0.0), vec2(1.0, 0.0))), plane__dual(self)));
}

Sphere round_point__anti_support(RoundPoint self) {
    return round_point__wedge__circle(self, sphere__anti_wedge__sphere(round_point__complement(RoundPoint(vec3(0.0), vec2(1.0, 0.0))), round_point__dual(self)));
}

Sphere scalar__anti_support(Scalar self) {
    return scalar__wedge__sphere(self, sphere__anti_wedge__anti_scalar(round_point__complement(RoundPoint(vec3(0.0), vec2(1.0, 0.0))), scalar__dual(self)));
}

Sphere sphere__anti_support(Sphere self) {
    return sphere__wedge__scalar(self, sphere__anti_wedge__round_point(round_point__complement(RoundPoint(vec3(0.0), vec2(1.0, 0.0))), sphere__dual(self)));
}

RoundPoint anti_scalar__support(AntiScalar self) {
    return anti_scalar__anti_wedge__round_point(self, round_point__wedge__scalar(RoundPoint(vec3(0.0), vec2(1.0, 0.0)), anti_scalar__anti_dual(self)));
}

RoundPoint circle__support(Circle self) {
    return circle__anti_wedge__circle(self, round_point__wedge__dipole(RoundPoint(vec3(0.0), vec2(1.0, 0.0)), circle__anti_dual(self)));
}

RoundPoint dipole__support(Dipole self) {
    return dipole__anti_wedge__sphere(self, round_point__wedge__circle(RoundPoint(vec3(0.0), vec2(1.0, 0.0)), dipole__anti_dual(self)));
}

RoundPoint dual_num__support(DualNum self) {
    return dual_num__anti_wedge__round_point(self, round_point__wedge__dual_num(RoundPoint(vec3(0.0), vec2(1.0, 0.0)), dual_num__anti_dual(self)));
}

RoundPoint flat_point__support(FlatPoint self) {
    return flat_point__anti_wedge__sphere(self, round_point__wedge__circle(RoundPoint(vec3(0.0), vec2(1.0, 0.0)), flat_point__anti_dual(self)));
}

MultiVector flector__support(Flector self) {
    return flector__anti_wedge__multi_vector(self, round_point__wedge__multi_vector(RoundPoint(vec3(0.0), vec2(1.0, 0.0)), flector__anti_dual(self)));
}

RoundPoint line__support(Line self) {
    return line__anti_wedge__circle(self, round_point__wedge__dipole(RoundPoint(vec3(0.0), vec2(1.0, 0.0)), line__anti_dual(self)));
}

MultiVector motor__support(Motor self) {
    return motor__anti_wedge__multi_vector(self, round_point__wedge__multi_vector(RoundPoint(vec3(0.0), vec2(1.0, 0.0)), motor__anti_dual(self)));
}

MultiVector multi_vector__support(MultiVector self) {
    return multi_vector__anti_wedge__multi_vector(self, round_point__wedge__multi_vector(RoundPoint(vec3(0.0), vec2(1.0, 0.0)), multi_vector__anti_dual(self)));
}

RoundPoint plane__support(Plane self) {
    return plane__anti_wedge__dipole(self, round_point__wedge__round_point(RoundPoint(vec3(0.0), vec2(1.0, 0.0)), plane__anti_dual(self)));
}

RoundPoint round_point__support(RoundPoint self) {
    return round_point__anti_wedge__anti_scalar(self, round_point__wedge__sphere(RoundPoint(vec3(0.0), vec2(1.0, 0.0)), round_point__anti_dual(self)));
}

RoundPoint sphere__support(Sphere self) {
    return sphere__anti_wedge__dipole(self, round_point__wedge__round_point(RoundPoint(vec3(0.0), vec2(1.0, 0.0)), sphere__anti_dual(self)));
}

