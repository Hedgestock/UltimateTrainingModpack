use once_cell::sync::Lazy;
use crate::training::ui;

use skyline::nn::ui2d::ResColor;
use skyline::nn::ui2d::*;
use smash::ui2d::{SmashPane, SmashTextBox};

use training_mod_consts::OnOff;
use training_mod_consts::MENU;

pub static mut PLAYER_HITSTUN_GAUGE: Lazy<HitstunGauge> = Lazy::new(|| HitstunGauge::new(0.0));
pub static mut CPU_HITSTUN_GAUGE: Lazy<HitstunGauge> = Lazy::new(|| HitstunGauge::new(0.0));

#[derive(Clone)]
pub struct HitstunGauge {
    pub frames_total: u32,
    pub frames: f32,
    pub color: ResColor,
}

impl HitstunGauge {
    pub fn new(frames: f32) -> HitstunGauge {
        HitstunGauge {
            frames_total: 0,
            frames,
            color: ResColor { r: 255, g: 255, b: 255, a: 255},
        }
    }
}

pub fn update_hitstun_gauge(frames: f32, gauge: &mut HitstunGauge) {
    let mut color_offset: f32 = 0.0;
    if frames > 0.0 {
        if gauge.frames_total == 0 {
            gauge.frames_total = frames as u32;
        }
        color_offset = (std::cmp::min(frames as u32, 30) as f32 / std::cmp::min(gauge.frames_total, 30) as f32) * 255.0;
    } else if gauge.frames_total > 0 {
        gauge.frames_total = 0;
    }
    ui::notifications::clear_notifications("TESTING");
    ui::notifications::notification(
        "TESTING".to_string(),
        format!("{color_offset}"),
        60
    );

    gauge.frames = frames;
    gauge.color = ResColor { r: 255, g: 255 - color_offset as u8, b: 255 - color_offset as u8, a: 255 };
}

pub unsafe fn draw(root_pane: &mut Pane) {
    let visible = MENU.hitstun_vis == OnOff::On;
    root_pane.find_pane_by_name_recursive("TrModHitstunDisp").unwrap().set_visible(visible);
    if !visible { return; }

    let player_frames_text = root_pane.find_pane_by_name_recursive("PlayerHitstunFrames").unwrap().as_textbox();
    let player_hitstun_frames = PLAYER_HITSTUN_GAUGE.frames as u32;
    player_frames_text.set_text_string(&format!("{player_hitstun_frames}"));
    let color = PLAYER_HITSTUN_GAUGE.color;
    player_frames_text.set_color(color.r, color.g, color.b, color.a);


    let cpu_frames_text = root_pane.find_pane_by_name_recursive("CPUHitstunFrames").unwrap().as_textbox();
    let cpu_hitstun_frames = CPU_HITSTUN_GAUGE.frames as u32;
    cpu_frames_text.set_text_string(&format!("{cpu_hitstun_frames}"));
    let color = CPU_HITSTUN_GAUGE.color;
    cpu_frames_text.set_color(color.r, color.g, color.b, color.a);

    let cpu_hitstun_gauge = (1..CPU_HITSTUN_GAUGE.frames_total)
        .map(|frame| if frame > CPU_HITSTUN_GAUGE.frames as u32 { ' ' } else { '|' })
        .collect::<String>();

    let player_hitstun_gauge = (1..PLAYER_HITSTUN_GAUGE.frames_total)
        .map(|frame| if frame > PLAYER_HITSTUN_GAUGE.frames as u32 { ' ' } else { '|' })
        .collect::<String>();

    root_pane.find_pane_by_name_recursive("PlayerHitstunGauge").unwrap().as_textbox().set_text_string(&player_hitstun_gauge);
    root_pane.find_pane_by_name_recursive("CPUHitstunGauge").unwrap().as_textbox().set_text_string(&cpu_hitstun_gauge);
}