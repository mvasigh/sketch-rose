use nannou::math::prelude::*;
use nannou::math::Deg;
use nannou::prelude::*;

const SIZE: f32 = 350.0;

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
    let r = SIZE * (k * model.n).sin();
    let x = k.sin() * r;
    let y = k.cos() * r;

    pt2(x, y)
}

fn get_color(pt: Point2<f32>) -> Srgb {
    let dist = distance(pt, pt2(0.0, 0.0));
    let val = map_range(dist, 0.0, 450.0, 0.08, 0.8);
    srgb(val, val, val)
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
    let d = 39.0;

    Model { _window, n, d }
}

fn update(_app: &App, _model: &mut Model, _update: Update) {}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    if app.elapsed_frames() == 1 {
        draw.background().color(gray(0.08));
    }
    draw.rect()
        .w_h(800.0, 800.0)
        .color(srgba(0.08, 0.08, 0.08, 0.2));

    let outer = (0..=360).map(|i| {
        let pt = get_point(i, LineKind::Outer, &model);
        let color = get_color(pt);

        (pt, color)
    });

    draw.polyline().weight(4.0).points_colored(outer);

    let inner = (0..=360).map(|i| {
        let pt = get_point(i, LineKind::Inner, &model);
        let color = get_color(pt);

        (pt, color)
    });

    draw.polyline().weight(1.5).points_colored(inner);

    draw.to_frame(app, &frame).unwrap();
}
