use nannou::image;
use nannou::noise::{Fbm, MultiFractal, NoiseFn};
use nannou::prelude::*;

struct Model {
    texture: wgpu::Texture,
    noise: Fbm,
}

fn main() {
    nannou::app(model).run();
}

fn model(app: &App) -> Model {
    //イベントハンドラなどを設定
    app.new_window().size(600, 1200).view(view).build().unwrap();

    let window = app.main_window();
    let win = window.rect();
    let texture = wgpu::TextureBuilder::new()
        .size([win.w() as u32, win.h() as u32])
        .format(wgpu::TextureFormat::Rgba8Unorm)
        .usage(wgpu::TextureUsage::COPY_DST | wgpu::TextureUsage::SAMPLED)
        .build(window.swap_chain_device());

    Model {
        texture,
        noise: Fbm::new().set_octaves(5).set_persistence(0.5 as f64),
    }
}

//アプリケーションが起動している間、ループ
fn view(app: &App, model: &Model, frame: Frame) {
    frame.clear(BLACK);

    let win = app.window_rect();
    let noise = &model.noise;

    let noise_x_range = win.w() / 50.0;
    let noise_y_range = win.h() / 50.0;

    let image = image::ImageBuffer::from_fn(win.w() as u32, win.h() as u32, |x, y| {
        let noise_x = map_range(x, 0, win.w() as u32, 0.0, noise_x_range) as f64;
        let noise_y = map_range(y, 0, win.h() as u32, 0.0, noise_y_range) as f64;
        let noise_value = map_range(
            noise.get([noise_x, noise_y, app.time as f64]),
            1.0,
            -1.0,
            0.0,
            std::u8::MAX as f64,
        );
        let n = noise_value as u8;
        if x % 10 == 5 && y % 10 == 5 {
            return nannou::image::Rgba([n, n, 0, std::u8::MAX]);
        }
        nannou::image::Rgba([0, 0, 0, std::u8::MAX])
    });

    let flat_samples = image.as_flat_samples();
    model.texture.upload_data(
        app.main_window().swap_chain_device(),
        &mut *frame.command_encoder(),
        &flat_samples.as_slice(),
    );

    let draw = app.draw();
    draw.texture(&model.texture);

    draw.to_frame(app, &frame).unwrap();
}
