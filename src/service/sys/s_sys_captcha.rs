use crate::model::sys::args::acaptch::*;
use crate::service::prelude::*;
use captcha_rust::Captcha;
use image::{DynamicImage, ImageBuffer, Rgba, RgbaImage};
use rand::Rng;
use std::io::Cursor;
use base64::{Engine as _, engine::general_purpose::STANDARD as BASE64};

const BACKGROUND_WIDTH: u32 = 320;
const BACKGROUND_HEIGHT: u32 = 160;
const SLIDER_SIZE: u32 = 40;

pub async fn get_captcha(VQuery(arg): VQuery<ClientInfo>) -> impl IntoResponse {
    let res: CaptchaImage = gen_captcha(arg).await;
    ApiResponse::ok(res)
}

async fn gen_captcha(arg: ClientInfo) -> CaptchaImage {
    let captcha = Captcha::new(5, 130, 40);

    let cacheinfo=CaptchaCacheInfo {
        client_id: arg.client_id,
        cache_text: captcha.text.clone(),
    };
    let uuid = GID().await;
    let cache = CacheManager::instance().await;
    let _ = cache
        .set_value_ex(&format!("capcha:{}", uuid), &cacheinfo, 300)
        .await;
    info!("获取验证码:{}", captcha.text);
    CaptchaImage {
        captcha_on_off: true,
        uuid,
        img: captcha.base_img,
    }
}

pub async fn gen_wx_captcha() -> String {
    let captcha = Captcha::new(5, 130, 40);
    let captchtxt = captcha.text;
    let wxcaptcha = format!("wx_{}", captchtxt);
    let cache = CacheManager::instance().await;
    let _ = cache
        .set_string_ex(&wxcaptcha, captchtxt.as_str(), 300)
        .await;
    captchtxt
}

pub async fn get_slider_captcha(VQuery(arg): VQuery<ClientInfo>) -> impl IntoResponse {
    match gen_slider_captcha(arg.client_id).await {
        Ok(captcha) => ApiResponse::ok(captcha),
        Err(e) => ApiResponse::internal_server_error(e.to_string()),
    }
}

async fn gen_slider_captcha(client_id: String) -> std::result::Result<SliderCaptchaImage, String> {
    let (gap_x, gap_y) = generate_random_gap();

    let background = generate_background_image()?;
    let slider = extract_slider_from_background(&background, gap_x, gap_y)?;

    let background_base64 = image_to_base64(&DynamicImage::ImageRgba8(background.clone()), "PNG")?;
    let slider_base64 = image_to_base64(&DynamicImage::ImageRgba8(slider.clone()), "PNG")?;

    let cache_info = SliderCaptchaCacheInfo {
        client_id: client_id.clone(),
        gap_x,
        gap_y,
    };
    let uuid = GID().await;
    let cache = CacheManager::instance().await;
    cache
        .set_value_ex(&format!("slider_captcha:{}", uuid), &cache_info, 300)
        .await
        .map_err(|e| format!("Failed to cache slider captcha: {}", e))?;

    info!("生成滑动验证码: uuid={}, gap=({}, {})", uuid, gap_x, gap_y);

    Ok(SliderCaptchaImage {
        captcha_on_off: true,
        uuid,
        background_image: background_base64,
        slider_image: slider_base64,
        gap_x,
        gap_y,
    })
}

fn generate_random_gap() -> (u32, u32) {
    let mut rng = rand::rng();

    let min_x = SLIDER_SIZE;
    let max_x = BACKGROUND_WIDTH - SLIDER_SIZE - 10;
    let min_y = SLIDER_SIZE;
    let max_y = BACKGROUND_HEIGHT - SLIDER_SIZE - 10;

    let gap_x = if min_x < max_x {
        rng.random_range(min_x..max_x)
    } else {
        20
    };

    let gap_y = if min_y < max_y {
        rng.random_range(min_y..max_y)
    } else {
        20
    };

    (gap_x, gap_y)
}

fn generate_background_image() -> std::result::Result<RgbaImage, String> {
    let mut img = RgbaImage::new(BACKGROUND_WIDTH, BACKGROUND_HEIGHT);
    let mut rng = rand::rng();

    for y in 0..BACKGROUND_HEIGHT {
        for x in 0..BACKGROUND_WIDTH {
            let r = rng.random_range(200..255);
            let g = rng.random_range(200..255);
            let b = rng.random_range(200..255);
            img.put_pixel(x, y, Rgba([r, g, b, 255]));
        }
    }

    for _ in 0..50 {
        let x1 = rng.random_range(0..BACKGROUND_WIDTH);
        let y1 = rng.random_range(0..BACKGROUND_HEIGHT);
        let x2 = rng.random_range(0..BACKGROUND_WIDTH);
        let y2 = rng.random_range(0..BACKGROUND_HEIGHT);
        let r = rng.random_range(150..200);
        let g = rng.random_range(150..200);
        let b = rng.random_range(150..200);

        draw_line(&mut img, x1, y1, x2, y2, Rgba([r, g, b, 200]));
    }

    for _ in 0..30 {
        let cx = rng.random_range(0..BACKGROUND_WIDTH);
        let cy = rng.random_range(0..BACKGROUND_HEIGHT);
        let radius = rng.random_range(5..20);
        let r = rng.random_range(150..200);
        let g = rng.random_range(150..200);
        let b = rng.random_range(150..200);

        draw_circle(&mut img, cx, cy, radius, Rgba([r, g, b, 180]));
    }

    Ok(img)
}

fn draw_line(img: &mut RgbaImage, x1: u32, y1: u32, x2: u32, y2: u32, color: Rgba<u8>) {
    let dx = (x2 as i32 - x1 as i32).abs();
    let dy = (y2 as i32 - y1 as i32).abs();
    let sx = if x1 < x2 { 1 } else { -1 };
    let sy = if y1 < y2 { 1 } else { -1 };
    let mut err = dx - dy;

    let mut x = x1 as i32;
    let mut y = y1 as i32;

    loop {
        if x >= 0 && x < BACKGROUND_WIDTH as i32 && y >= 0 && y < BACKGROUND_HEIGHT as i32 {
            img.put_pixel(x as u32, y as u32, color);
        }

        if x == x2 as i32 && y == y2 as i32 {
            break;
        }

        let e2 = 2 * err;
        if e2 > -dy {
            err -= dy;
            x += sx;
        }
        if e2 < dx {
            err += dx;
            y += sy;
        }
    }
}

fn draw_circle(img: &mut RgbaImage, cx: u32, cy: u32, radius: u32, color: Rgba<u8>) {
    let r_sq = (radius * radius) as i32;

    for y in 0..=radius {
        for x in 0..=radius {
            let dist_sq = (x * x + y * y) as i32;
            if dist_sq <= r_sq {
                let px = if cx > x { cx - x } else { cx };
                let py = if cy > y { cy - y } else { cy };
                let nx = cx + x;
                let ny = cy + y;

                if px < BACKGROUND_WIDTH && py < BACKGROUND_HEIGHT {
                    img.put_pixel(px, py, color);
                }
                if nx < BACKGROUND_WIDTH && py < BACKGROUND_HEIGHT {
                    img.put_pixel(nx, py, color);
                }
                if px < BACKGROUND_WIDTH && ny < BACKGROUND_HEIGHT {
                    img.put_pixel(px, ny, color);
                }
                if nx < BACKGROUND_WIDTH && ny < BACKGROUND_HEIGHT {
                    img.put_pixel(nx, ny, color);
                }
            }
        }
    }
}

fn extract_slider_from_background(background: &RgbaImage, gap_x: u32, gap_y: u32) -> std::result::Result<RgbaImage, String> {
    let mut slider = ImageBuffer::new(SLIDER_SIZE, SLIDER_SIZE);

    for y in 0..SLIDER_SIZE {
        for x in 0..SLIDER_SIZE {
            let bg_x = gap_x + x;
            let bg_y = gap_y + y;

            if bg_x < BACKGROUND_WIDTH && bg_y < BACKGROUND_HEIGHT {
                let pixel = background.get_pixel(bg_x, bg_y);
                slider.put_pixel(x, y, *pixel);
            } else {
                slider.put_pixel(x, y, Rgba([255, 255, 255, 255]));
            }
        }
    }

    for y in 0..SLIDER_SIZE {
        for x in 0..SLIDER_SIZE {
            if x == 0 || x == SLIDER_SIZE - 1 || y == 0 || y == SLIDER_SIZE - 1 {
                let pixel = slider.get_pixel_mut(x, y);
                *pixel = Rgba([80, 80, 80, 255]);
            }
        }
    }

    for y in 2..SLIDER_SIZE - 2 {
        for x in 2..SLIDER_SIZE - 2 {
            let pixel = slider.get_pixel_mut(x, y);
            if pixel[3] > 0 {
                pixel[3] = (pixel[3] as f32 * 0.9) as u8;
            }
        }
    }

    Ok(slider)
}

fn image_to_base64(img: &DynamicImage, format: &str) -> std::result::Result<String, String> {
    let mut buffer = Cursor::new(Vec::new());

    let encoded_format = match format {
        "PNG" => image::ImageFormat::Png,
        "JPEG" | "JPG" => image::ImageFormat::Jpeg,
        _ => image::ImageFormat::Png,
    };

    img.write_to(&mut buffer, encoded_format)
        .map_err(|e| format!("Failed to encode image: {}", e))?;

    Ok(format!("data:image/{};base64,{}",
        format.to_lowercase(),
        BASE64.encode(buffer.into_inner())))
}
