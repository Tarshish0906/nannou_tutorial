use nannou::prelude::*;

struct Model {
    speed: f32,
}

fn main() {
    nannou::app(model).event(event).simple_window(view).run();
}

fn model(_app: &App) -> Model {
    Model { speed: 5.0 }
}

fn event(_app: &App, model: &mut Model, event: Event) {
    match event {
        Event::WindowEvent {
            id: _id,
            simple: window_event,
        } => {
            if let Some(KeyPressed(key)) = window_event {
                match key {
                    Key::Up => model.speed = model.speed + 1.0,
                    Key::Down => model.speed = model.speed - 1.0,
                    _ => {}
                }
            }
        }
        _ => {}
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let t = app.time;
    let draw = app.draw();

    draw.background().color(BLACK);

    let x = model.speed * t as f32;
    let point = pt2(x.cos(), x.sin()) * 100.0;

    draw.ellipse()
        .color(STEELBLUE)
        .w(200.0)
        .h(200.0)
        .x_y(point.x, point.y);
    draw.to_frame(app, &frame).unwrap();
}
