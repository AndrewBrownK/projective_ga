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


    // return background_color + float4(meet_fragment_and_circle.e4);

    // Now get the squared radius of the meet
    float radius_squared = meet_fragment_and_circle.unitized_radius_norm_squared();

    return background_color + float4(radius_squared);

    // And lastly, make pixels more bright the closer they are to the circle.
    float4 destructive_circle_color = float4(1.0) - float4(abs(radius_squared) * 100.0);
    return background_color + destructive_circle_color;
}

/*
TODO there is an NVVM crash that happens when we compile to SPIRV, but not when we compile to WGSL
 It doesn't happen if we return before meet_fragment_and_circle.unitized_radius_norm_squared(),
 and it does happen if we return after meet_fragment_and_circle.unitized_radius_norm_squared().
 I gave the before and after spirv-dis to SuperGrok, as well as a bit of code surrounding
 the meet_fragment_and_circle.unitized_radius_norm_squared() invocation, and the definition
 of RoundPoint.unitized_radius_norm_squared();
 SuperGrok thinks the issue could be a driver bug, since spirv-val validates it just fine.
 It thinks we might be able to prevent the issue if we get rid of the pow(x, -2) invocation.
 There are three things I can do towards this end
 1) make pow(x, 2) output as x*x instead
 2) make pow(x, -y) output as division if y is 1 or 2
 3) Perform destructured simplification, which unwraps types when the whole type is not used, and will
    cause some of the expressions to simplify like pow(1, -2) = 1.
 Although I'd rather have the slang example working perfectly before moving on to more simplification,
 it is undoubtedly true that this is simplification that I am interested in doing eventually, and if
 doing so is necessary in order to fix the slang example, then oh well. I guess I'll do it sooner than I expected.
*/
/*
public extension RoundPoint {
    public float unitized_radius_norm_squared() {
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