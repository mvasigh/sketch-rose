use nannou::math::prelude::*;
use nannou::math::Deg;
use nannou::prelude::*;

enum LineKind {
    Inner,
    Outer,
}

fn distance(pt1: Point2<f32>, pt2: Point2<f32>) -> f32 {
    f32::sqrt((pt2.x - pt1.x).pow(2) + (pt2.y - pt1.y).pow(2))
}

fn get_point(angle: i32, variant: LineKind, model: &Model) -> Point2<f32> {
    let factor = match variant {
        LineKind::Inner => model.d,
        LineKind::Outer => 1.0,
    };
    let k = Deg(angle as f32 * factor);
    let r = 300.0 * (k * model.n).sin();
    let x = k.sin() * r;
    let y = k.cos() * r;

    pt2(x, y)
}

fn get_color(pt: Point2<f32>) -> Hsl {
    let dist = distance(pt, pt2(0.0, 0.0));
    hsl(0.0, 0.0, map_range(dist, 0.0, 300.0, 0.6, 0.1))
}

struct Model {
    _window: WindowId,
    n: f32,
    d: f32,
}

fn main() {
    nannou::app(model).update(update).run();
}

fn model(app: &App) -> Model {
    let _window = app.new_window().size(800, 800).view(view).build().unwrap();
    let n = 2.0;
    let d = 35.0;

    Model { _window, n, d }
}

fn update(_app: &App, _model: &mut Model, _update: Update) {}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    draw.background().color(gray(0.08));

    let inner = (0..=360).map(|i| {
        let pt = get_point(i, LineKind::Inner, &model);
        let color = get_color(pt);

        (pt, color)
    });

    draw.polyline().weight(2.0).points_colored(inner);

    let outer = (0..=360).map(|i| {
        let pt = get_point(i, LineKind::Outer, &model);
        let color = get_color(pt);

        (pt, color)
    });

    draw.polyline().weight(3.0).points_colored(outer);

    draw.to_frame(app, &frame).unwrap();
}
