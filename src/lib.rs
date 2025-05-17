// gotta go fast!
turbo::go!({
    clear(0x0000ffff);
    let t = tick() as f32;
    let canvas_bounds = bounds::canvas();
    let water_bounds_fg = canvas_bounds.height(32).anchor_bottom(&canvas_bounds);
    let water_bounds_bg = water_bounds_fg.above_self();
    let trees_bounds = water_bounds_bg.height(16).above_self();
    let mountains_bounds = trees_bounds.height(32).translate_y(-24);
    let clouds_bounds = canvas_bounds.height(64);
    sprite!(
        "clouds",
        xy = clouds_bounds.xy(),
        wh = clouds_bounds.wh(),
        repeat = true,
        tx = t / 16.,
    );
    sprite!(
        "mountains",
        xy = mountains_bounds.xy(),
        wh = mountains_bounds.wh(),
        repeat = true,
        tx = t / 8.,
    );
    sprite!(
        "trees",
        xy = trees_bounds.xy(),
        wh = trees_bounds.wh(),
        repeat = true,
        tx = t / 4.,
    );
    sprite!(
        "water",
        xy = water_bounds_bg.xy(),
        wh = water_bounds_bg.wh(),
        repeat = true,
        tx = t / 2.,
    );
    sprite!(
        "water",
        xy = water_bounds_fg.xy(),
        wh = water_bounds_fg.wh(),
        repeat = true,
        tx = t,
    );
    let logo_bounds = Bounds::with_size(128, 128)
        .anchor_center(&canvas_bounds)
        .translate_y(oscillate(t / 40., 1., -4., 4.));
    sprite!("sanic_logo", xy = logo_bounds.xy());

    let blink_time = 32.;
    if t % blink_time < (blink_time / 2.) {
        let press_start_bounds = canvas_bounds
            .height(8)
            .anchor_bottom(&canvas_bounds)
            .translate_y(-8);
        text_box!(
            "PRESS START",
            xy = press_start_bounds.xy(),
            wh = press_start_bounds.wh(),
            align = "center",
        );
    }
});

fn oscillate(t: f32, freq: f32, low: f32, high: f32) -> f32 {
    let mid = (low + high) / 2.0;
    let amp = (high - low) / 2.0;
    mid + amp * (2.0 * std::f32::consts::PI * freq * t).sin()
}
