use nannou::prelude::*;

struct Model {
    color: Rgb8,
}

fn main() {
    nannou::app(model).simple_window(view).size(600, 400).run();
}
fn model(app: &App) -> Model {
    //イベントハンドラなどを設定
    app.new_window()
        .size(600, 400)
        .mouse_pressed(mouse_pressed)
        .view(view)
        .build()
        .unwrap();

    //色の初期値を指定してインスタンス生成
    Model {
        color: gen_random_color(),
    }
}

//クリック時のイベントハンドラ
fn mouse_pressed(_app: &App, model: &mut Model, _button: MouseButton) {
    model.color = gen_random_color()
}

// ランダムな色を返す自作関数
fn gen_random_color() -> Rgb8 {
    let r = random::<u8>();
    let g = random::<u8>();
    let b = random::<u8>();
    let random_color = rgb8(r, g, b);
    random_color
}
//アプリケーションが起動している間、ループ
fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    draw.background().color(WHITE);

    draw.rect()
        //設定した色で塗りつぶす
        .color(model.color)
        .x_y(0.0, 0.0)
        .w_h(100.0, 100.0);

    draw.to_frame(app, &frame).unwrap();
}
