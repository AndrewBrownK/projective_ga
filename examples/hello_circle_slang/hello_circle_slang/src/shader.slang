import cga3d;
// TODO alter namespacing to just be cga3d and that's it
using data;
using traits;

// Quick refresher on shader stages:
// 1) Objects start in homogenous space, perhaps also known as "view space"
// 2) In the vertex shader, you project this space so the view frustum = the normalized device coordinates (NDC) box.
//    This is called "clip space".
// 3) After the vertex shader, the clipping stage will remove/ignore objects outside the frustum (which is the NDC box).
//    If an object is partially inside and partially outside the clipped space, the GPU automatically creates new
//    vertices at the intersection points.
// 4) The clipped geometry is given to the fragment shader for coloring.


struct VertexOutput {
    float4 position : SV_Position;
    float4 percent_position : TEXCOORD0;
};

[shader("vertex")]
VertexOutput vs_main(uint in_vertex_index : SV_VertexID) {
    // Full screen triangle
    float expand = 3.0;
    float x = float(int(in_vertex_index) - 1);
    float y = float(int(in_vertex_index & 1u) * 2 - 1);
    VertexOutput out;
    out.position = float4(expand * x, expand * y, 0.0, 1.0);
    out.percent_position = float4(0.5 * (1.0 + (expand * x)), 0.5 * (1.0 - (expand * y)), 0.0, 1.0);
    return out;
}

[[vk::binding(0)]]
ConstantBuffer<float2> ScreenSize : register(b0);

[shader("fragment")]
float4 fs_main(
    in VertexOutput vertex,
): SV_Target0 {

    //return float4(0, 1, 0, 1);

    // Start by recovering a nice coordinate space
    // (0, 0) is in the center
    // lesser of x or y will range from -1 to 1
    // Unlike NDC, this will be square because it is corrected for aspect ratio
    float new_x = vertex.percent_position.x;
    float new_y = vertex.percent_position.y;
    new_x = (2.0 * new_x) - 1.0;
    new_y = 1.0 - (2.0 * new_y);
    float max_aspect = max(ScreenSize.x, ScreenSize.y);
    float x_correction = max_aspect / ScreenSize.y;
    float y_correction = max_aspect / ScreenSize.x;
    new_x = new_x * x_correction;
    new_y = new_y * y_correction;

    // Choose some background colors that will help us confirm that we got the coordinate space we wanted
    float4 color = float4(new_x, new_y, 0.0, 1.0);
    float4 background_color = select(
        (color.x < -1.0) || (color.y < -0.5),
        float4(color.x, color.y, 0.5, 1.0),
        color,
    );
    //return background_color;



    // Now let's do some CGA

    // Start by constructing a Circle.
    // We'll create our circle by joining 3 RoundPoints.

    // Create 3 round points
    RoundPoint left = RoundPoint(-1.0, 0.0, 0.0, 1.0, 0.5);
    RoundPoint top = RoundPoint(0.0, 1.0, 0.0, 1.0, 0.5);
    RoundPoint right = RoundPoint(1.0, 0.0, 0.0, 1.0, 0.5);

    // Join the first pair
    Dipole left_to_top_dipole = left /wedge/ top;
    // return background_color + float4(left_to_top_dipole.e41);

    // Join the remaining point
    Circle circle = left_to_top_dipole /wedge/ right;

    // Circle { e423: 0.0, e431: 0.0, e412: -2.0, e415: 0.0, e425: 0.0, e435: 0.0, e321: 0.0, e235: 0.0, e315: 0.0, e125: -1.0 }

    //return background_color + float4(circle.e412); // -2, causes black screen
    //return background_color + float4(circle.e125); // -1, causes color changes, mostly black
    //return background_color - float4(circle.e412); // +2, causes white screen
    //return background_color - float4(circle.e125); // +1, Causes color changes, mostly white


    // Now that we have our circle, we can determine what we want to do with this fragment

    // Let's create a line representing this fragment
    float fp_e5 = (new_x * new_x + new_y * new_y + 1.0) / 2.0;
    RoundPoint fragment_point = RoundPoint(new_x, new_y, 1.0, 1.0, fp_e5);
    FlatPoint direction = FlatPoint(0.0, 0.0, 1.0, 0.0);

    // Join a round point and point at infinity to create a line
    Line fragment_as_line = direction.wedge(fragment_point);

    // Now perform a meet to get a RoundPoint centered on the line and contained by the circle
    RoundPoint meet_fragment_and_circle = circle.anti_wedge(fragment_as_line);

    // TODO it seems like radius_squared is positive infinity, which naga will refuse to compile literals,
    // TODO but perhaps the GPU rolls with it at runtime
    // TODO the only way for it to be infinity is (assuming divide by zero) for e4 on the RoundPoint to be zero
    // TODO hello_circle_rust never finds an e4 = 0.0 on that
    // TODO so how can we get e4 = 0.0 from circle.anti_wedge(fragment_as_line)?
    // TODO this Circle and other Line -> e4 = -(this.e431 * other.e425) - (this.e412 * other.e435) - (this.e423 * other.e415)
    // TODO in rust, e4 is always = 2.0 here
    // TODO so let's try an experiment....
    //return background_color + float4(1.0 / meet_fragment_and_circle.e4);
    //return background_color - float4(1.0 / meet_fragment_and_circle.e4);
    // TODO damn it... it is really acting like e4 is 2.0.... well that's good in a way.
    // TODO but how is radius_squared turning into infinity then?

    // Now get the squared radius of the meet

    // TODO confirmed... the associated method returns infinity, but manually inlining returns correct value.
    float radius_squared = meet_fragment_and_circle.unitized_radius_norm_squared(); // this one is infinity
    //float radius_squared = unitized_radius_norm_squared_1(meet_fragment_and_circle); // this one is not infinity
    //float radius_squared = meet_fragment_and_circle.unitized_radius_norm_squared_2(); // This one causes NVVM failure

/*
    if (abs(radius_squared) > pow(2.0, 126)) {
        return float4(1.0) - background_color;
    } else {
        return background_color;
    }
*/

    // And lastly, make pixels more bright the closer they are to the circle.
    float4 destructive_circle_color = float4(1.0) - float4(abs(radius_squared) * 100.0);
    return background_color + destructive_circle_color;
}
/*
float unitized_radius_norm_squared_1(RoundPoint dis) {
    RoundPoint sub_type = RoundPoint(
        0.0, 0.0, 0.0, dis.e4, 0.0
    );
    DualNum other = DualNum(
        1.0, 0.0
    );
    return (pow(dis.e1, 2) * pow(other.e5, -2) * pow(sub_type.e4, -2))
        + (pow(dis.e2, 2) * pow(other.e5, -2) * pow(sub_type.e4, -2))
        + (pow(dis.e3, 2) * pow(other.e5, -2) * pow(sub_type.e4, -2))
        - 2.0 * (dis.e4 * dis.e5 * pow(other.e5, -2) * pow(sub_type.e4, -2));
}

public extension RoundPoint {
    public float unitized_radius_norm_squared_2() {
        RoundPoint sub_type = RoundPoint(
            float4(float3(0.0), this.e4),
            0.0
        );
        DualNum other = DualNum(
            float2(1.0, 0.0)
        );
        return (pow(this.e1, 2) * pow(other.e5, -2) * pow(sub_type.e4, -2)) + (pow(this.e2, 2) * pow(other.e5, -2) * pow(sub_type.e4, -2)) + (pow(this.e3, 2) * pow(other.e5, -2) * pow(sub_type.e4, -2)) - 2.0 * (this.e4 * this.e5 * pow(other.e5, -2) * pow(sub_type.e4, -2));
    }
}
*/