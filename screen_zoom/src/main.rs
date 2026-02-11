use std::sync::Arc;

use winit::{
    dpi::{ LogicalPosition, LogicalSize },
    event::{ Event, MouseScrollDelta, WindowEvent },
    event_loop::EventLoop,
    keyboard::Key,
    window::{ WindowAttributes, WindowLevel },
    raw_window_handle::{ HasWindowHandle, RawWindowHandle },
};

use pixels::{ Pixels, SurfaceTexture };
use captrs::Capturer;

use global_hotkey::{ hotkey::{ Code, HotKey, Modifiers }, GlobalHotKeyEvent, GlobalHotKeyManager };

// -------- Windows global mouse --------
use windows::Win32::{
    Foundation::{ POINT, HWND },
    UI::WindowsAndMessaging::GetCursorPos,
    Graphics::Gdi::{ RedrawWindow, RDW_UPDATENOW },
};

fn global_mouse_pos() -> (f64, f64) {
    unsafe {
        let mut pt = POINT { x: 0, y: 0 };
        GetCursorPos(&mut pt);
        (pt.x as f64, pt.y as f64)
    }
}

fn force_update(window: &winit::window::Window) {
    if let Ok(handle) = window.window_handle() {
        if let winit::raw_window_handle::RawWindowHandle::Win32(h) = handle.as_raw() {
            let hwnd = windows::Win32::Foundation::HWND(h.hwnd.get() as _);
            unsafe {
                // ต้องใส่ Some ล้อมรอบ hwnd
                RedrawWindow(Some(hwnd), None, None, RDW_UPDATENOW);
            }
        }
    }
}
// -------- App State --------
struct ZoomApp {
    zoom: f32,
    active: bool,

    mouse_pos: (f64, f64),
    window_pos: (f64, f64),

    lens_w: u32,
    lens_h: u32,

    capturer: Capturer,
    screen_w: u32,
    screen_h: u32,

    last_screen: Option<Vec<u8>>,
}

// -------- Draw zoom frame --------
fn update_frame(pixels: &mut Pixels, app: &ZoomApp) {
    let frame = pixels.frame_mut();
    frame.fill(0); // ล้างเฟรมเก่า

    // ดึงภาพล่าสุดจาก Cache มาวาด (ถ้าไม่มีก็จบงานไปก่อน)
    let Some(screen) = app.last_screen.as_ref() else {
        return;
    };

    let src_w = ((app.lens_w as f32) / app.zoom) as u32;
    let src_h = ((app.lens_h as f32) / app.zoom) as u32;
    let start_x = (app.mouse_pos.0 as u32).saturating_sub(src_w / 2);
    let start_y = (app.mouse_pos.1 as u32).saturating_sub(src_h / 2);

    for y in 0..app.lens_h {
        for x in 0..app.lens_w {
            let sx = start_x + (((x as f32) / app.zoom) as u32);
            let sy = start_y + (((y as f32) / app.zoom) as u32);

            if sx < app.screen_w && sy < app.screen_h {
                let dst = ((y * app.lens_w + x) * 4) as usize;
                let src = ((sy * app.screen_w + sx) * 4) as usize;

                // เขียนข้อมูลลง Frame ตรงๆ (RGBA)
                frame[dst..dst + 4].copy_from_slice(&screen[src..src + 4]);
            }
        }
    }
    draw_border(frame, app.lens_w, app.lens_h, 2, 255, 255, 255, 255);
}

// -------- Border --------
fn draw_border(frame: &mut [u8], w: u32, h: u32, t: u32, r: u8, g: u8, b: u8, a: u8) {
    let w = w as usize;
    let h = h as usize;
    let t = t as usize;

    for y in 0..h {
        for x in 0..w {
            let edge = x < t || x >= w - t || y < t || y >= h - t;
            if edge {
                let i = (y * w + x) * 4;
                frame[i] = r;
                frame[i + 1] = g;
                frame[i + 2] = b;
                frame[i + 3] = a;
            }
        }
    }
}

fn main() {
    // -------- Event loop --------
    let event_loop = EventLoop::new().unwrap();

    // -------- Window --------
    let window = Arc::new(
        event_loop
            .create_window(
                WindowAttributes::default()
                    .with_title("Zoom Overlay")
                    .with_inner_size(LogicalSize::new(400.0, 400.0))
                    .with_decorations(false)
                    .with_transparent(true)
                    .with_window_level(WindowLevel::AlwaysOnTop)
            )
            .unwrap()
    );

    // -------- Pixels --------
    let size = window.inner_size();
    let surface = SurfaceTexture::new(size.width, size.height, &window);
    let mut pixels = Pixels::new(size.width, size.height, surface).unwrap();

    // -------- Capture --------
    let capturer = Capturer::new(0).unwrap();
    let (screen_w, screen_h) = capturer.geometry();

    // -------- App --------
    let mut app = ZoomApp {
        zoom: 2.0,
        active: false,
        mouse_pos: (0.0, 0.0),
        window_pos: (200.0, 200.0),
        lens_w: size.width,
        lens_h: size.height,
        capturer,
        screen_w,
        screen_h,
        last_screen: None,
    };

    // -------- Global hotkey (Ctrl + Alt + Z) --------
    let manager = GlobalHotKeyManager::new().unwrap();
    let hotkey = HotKey::new(Some(Modifiers::CONTROL | Modifiers::ALT), Code::KeyZ);
    manager.register(hotkey).unwrap();
    let hotkey_rx = GlobalHotKeyEvent::receiver();

    let window = window.clone();
    window.set_visible(false);

    // -------- Run --------
    event_loop.run(move |event, elwt| {
        // toggle
        let mut toggled = false;
        while hotkey_rx.try_recv().is_ok() {
            toggled = true;
        }
        if toggled {
            app.active = !app.active;
            window.set_visible(app.active);
        }

        match event {
            Event::WindowEvent { event, .. } =>
                match event {
                    WindowEvent::CloseRequested => elwt.exit(),

                    WindowEvent::MouseWheel { delta, .. } => {
                        let d = match delta {
                            MouseScrollDelta::LineDelta(_, y) => y,
                            MouseScrollDelta::PixelDelta(p) => (p.y as f32) / 20.0,
                        };
                        app.zoom = (app.zoom + d * 0.1).clamp(1.0, 10.0);
                    }

                    WindowEvent::KeyboardInput { event, .. } if event.state.is_pressed() => {
                        match event.logical_key {
                            Key::Character(ref c) if c == "+" || c == "=" => {
                                app.zoom += 0.5;
                            }
                            Key::Character(ref c) if c == "-" => {
                                app.zoom -= 0.5;
                            }
                            _ => {}
                        }
                        app.zoom = app.zoom.clamp(1.0, 10.0);
                    }

                    WindowEvent::RedrawRequested => {
                        if app.active {
                            update_frame(&mut pixels, &mut app);
                            let _ = pixels.render();
                        }
                    }

                    _ => {}
                }

            // smooth follow mouse (lerp)
            Event::AboutToWait => {
                if app.active {
                    let mouse = global_mouse_pos();

                    // 1. ตรวจสอบระยะห่างจาก "จุดที่ถ่ายรูปไว้ล่าสุด"
                    // ไม่ใช่ระยะห่างจากตำแหน่งเลนส์ปัจจุบัน
                    let dx = mouse.0 - app.mouse_pos.0;
                    let dy = mouse.1 - app.mouse_pos.1;
                    let dist = (dx * dx + dy * dy).sqrt();

                    // ถ่ายใหม่เมื่อเมาส์ขยับเกิน 5px เพื่อลดภาระเครื่อง
                    if app.last_screen.is_none() || dist > 5.0 {
                        // WARP หนีกล้อง (แทนการ set_visible)
                        window.set_outer_position(LogicalPosition::new(-3000.0, -3000.0));

                        // รอจังหวะ Windows Sync สั้นๆ
                        std::thread::sleep(std::time::Duration::from_millis(2));

                        if app.capturer.capture_store_frame().is_ok() {
                            if let Some(screen) = app.capturer.get_stored_frame() {
                                let mut buf = vec![0u8; screen.len() * 4];
                                for (i, p) in screen.iter().enumerate() {
                                    let idx = i * 4;
                                    buf[idx] = p.r;
                                    buf[idx + 1] = p.g;
                                    buf[idx + 2] = p.b;
                                    buf[idx + 3] = 255;
                                }
                                app.last_screen = Some(buf);
                                app.mouse_pos = mouse; // บันทึกจุดที่ถ่ายไว้
                            }
                        }
                        // วาร์ปกลับมาตำแหน่งล่าสุด
                        window.set_outer_position(
                            LogicalPosition::new(app.window_pos.0, app.window_pos.1)
                        );
                        force_update(&window);
                    }

                    // 2. LERP การเคลื่อนที่ (ทำให้นุ่มนวล)
                    let sf = window.scale_factor();
                    let target_x = mouse.0 / sf - (app.lens_w as f64) / (2.0 * sf);
                    let target_y = mouse.1 / sf - (app.lens_h as f64) / (2.0 * sf);

                    app.window_pos.0 += (target_x - app.window_pos.0) * 0.15; // 0.15 = นุ่มขึ้น
                    app.window_pos.1 += (target_y - app.window_pos.1) * 0.15;

                    window.set_outer_position(
                        LogicalPosition::new(app.window_pos.0, app.window_pos.1)
                    );
                    window.request_redraw();
                }
            }
            _ => {}
        }
    });
}
