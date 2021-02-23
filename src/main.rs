//基本的なアイテムをインポート
use nannou::prelude::*;

//アプリケーションの状態を定義
struct Model {}

//アプリケーションの開始
fn main() {
    // 600*400 のウィンドウを用意
    nannou::app(model).simple_window(view).size(600, 400).run();
}

//Modelのインスタンスを生成
fn model(_app: &App) -> Model {
    Model {}
}

//画面の描画処理
fn view(app: &App, _model: &Model, frame: Frame) {
    //キャンバスを取得
    let draw = app.draw();

    //背景色を設定
    draw.background().color(NAVY);

    // 1辺100の正方形を原点に表示
    draw.rect().x_y(0.0, 0.0).w_h(100.0, 100.0).color(BLUE);

    //フレームに書き出し
    draw.to_frame(app, &frame).unwrap();
}
