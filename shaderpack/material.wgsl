// Blocks: vanilla's colour, its light pulled toward full light (0 = vanilla, 1 = full).
override brightness: f32 = 1.0;

fn material(input: MaterialInput) -> Surface {
    let light = mix(input.lightmap, vec3<f32>(1.0), brightness);
    return Surface(input.albedo * input.shaded_tint * vec4<f32>(light, 1.0));
}
