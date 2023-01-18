use nannou::prelude::*;
fn main() {
    nannou::app(model)
        // .update(update)
        .simple_window(view)
        .run();
}

struct Model {}

fn model(_app: &App) -> Model {
    Model {}
}

fn view(app: &App, _model: &Model, frame: Frame) {
    let draw = app.draw();

    // Generate sine wave data based on the time of the app
    let sine = (app.time * 5.0).sin();
    let slowersine = (app.time / 2.0).sin();
    // Get boundary of the window (to constrain the movements of our circle)
    let boundary = app.window_rect();

    // Map the sine wave functions to ranges between the boundaries of the window
    let x = map_range(sine, -1.0, 1.0, boundary.left(), boundary.right());
    let y = map_range(slowersine, -1.0, 1.0, boundary.bottom(), boundary.top());

    draw.background().color(DARKGRAY);
    draw.ellipse()
        .color(nannou::color::Alpha {
            color: DARKBLUE,
            alpha: 0.6,
        })
        .x_y(x, y);
    draw.to_frame(app, &frame).unwrap();
}

//fn update(_app: &App, _model: &mut Model, _update: Update) {}
