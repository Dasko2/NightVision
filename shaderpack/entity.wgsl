// Mobs, players and block entities: vanilla's own entity colour (from the API reference), same light.
override brightness: f32 = 1.0;

fn entity(input: EntityInput) -> Surface {
    let base = input.albedo.rgb * input.tint.rgb * input.shade;
    let gamma = vec3<f32>(1.0 / 2.2);
    let flashed = pow(mix(pow(base, gamma), pow(input.overlay.rgb, gamma), input.overlay.a), vec3<f32>(2.2));
    let light = mix(input.lightmap, vec3<f32>(1.0), brightness);
    return Surface(vec4<f32>(flashed * light, input.albedo.a * input.tint.a));
}
