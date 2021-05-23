use nannou::math::prelude::*;
use nannou::math::Deg;
use nannou::prelude::*;

const SIZE: f32 = 350.0;
const MAX_GREY: f32 = 0.08;
const MIN_GREY: f32 = 0.9;
const IMG_OUTPUT: bool = true;

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
    let val = map_range(dist, 0.0, 450.0, MAX_GREY + 0.3, MIN_GREY);
    srgb(val, val, val)
}

struct Model {
    _window: WindowId,
    n: f32,
    d: f32,
    outer: Vec<(Point2, Srgb)>,
    inner: Vec<(Point2, Srgb)>,
}

fn main() {
    nannou::app(model).update(update).run();
}

fn model(app: &App) -> Model {
    let _window = app.new_window().size(800, 800).view(view).build().unwrap();
    let n = 4.0;
    let d = 71.0;
    let outer = Vec::new();
    let inner = Vec::new();

    Model {
        _window,
        n,
        d,
        outer,
        inner,
    }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    model.d += 0.125;

    let start_index = app.elapsed_frames() % 360;
    let outer = (start_index..start_index + 20)
        .map(|i| {
            let pt = get_point(i as i32, LineKind::Outer, &model);
            let color = get_color(pt);

            (pt, color)
        })
        .collect();
    let inner = (start_index..start_index + 180)
        .map(|i| {
            let pt = get_point(i as i32, LineKind::Inner, &model);
            let color = get_color(pt);

            (pt, color)
        })
        .collect();

    model.outer = outer;
    model.inner = inner;
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    let window = app
        .window(model._window)
        .expect("Could not get the main window");

    if app.elapsed_frames() == 1 {
        draw.background().color(gray(MAX_GREY));
    }
    draw.rect()
        .w_h(800.0, 800.0)
        .color(srgba(MAX_GREY, MAX_GREY, MAX_GREY, 0.05));

    // draw.polyline()
    //     .weight(4.0)
    //     .points_colored(model.outer.to_owned());

    draw.polyline()
        .weight(0.5)
        .points_colored(model.inner.to_owned());

    if IMG_OUTPUT {   
        let filename = format!("./out/img{:04}.png", app.elapsed_frames());
        window.capture_frame(filename);
    }
    draw.to_frame(app, &frame).unwrap();
}
