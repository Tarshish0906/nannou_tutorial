// 基本的なアイテムをインポート
use nannou::prelude::*;

// アプリケーションの状態を定義
struct Model {}

// アプリケーションの開始
fn main() {
    // 600*400 のウィンドウを用意
    nannou::app(model).simple_window(view).size(600, 400).run();
}

// Modelのインスタンスを生成
fn model(_app: &App) -> Model {
    Model {}
}

// 画面の描画処理
fn view(app: &App, _model: &Model, frame: Frame) {
    // キャンバスを取得
    let draw = app.draw();

    let sine = app.time.sin();
    let slowersine1 = (app.time / 2.0).sin();
    let slowersine2 = (app.time / 3.0).sin();

    let boundary = app.window_rect();

    let x = map_range(sine, -1.0, 1.0, boundary.left(), boundary.right());
    let y1 = map_range(slowersine1, -1.0, 1.0, boundary.bottom(), boundary.top());
    let y2 = map_range(slowersine2, -1.0, 1.0, boundary.bottom(), boundary.top());

    // 背景色を設定
    draw.background().color(NAVY);

    // 半径10の円を原点に表示
    draw.ellipse().x_y(x, y1).color(STEELBLUE);
    draw.ellipse().x_y(x, y2).color(BLUE);

    // フレームに書き出し
    draw.to_frame(app, &frame).unwrap();
}
