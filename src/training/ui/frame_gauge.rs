use once_cell::sync::Lazy;

use skyline::nn::ui2d::ResColor;
use skyline::nn::ui2d::*;
use smash::ui2d::{SmashPane, SmashTextBox};

use training_mod_consts::OnOff;
use training_mod_consts::MENU;

pub static mut PLAYER_FRAME_GAUGE: Lazy<FrameGauge> = Lazy::new(|| FrameGauge::new(0));
pub static mut CPU_FRAME_GAUGE: Lazy<FrameGauge> = Lazy::new(|| FrameGauge::new(0));

#[derive(Clone)]
pub struct FrameGauge {
    pub frames_total: u32,
    pub frames: u32,
    pub color: ResColor,
    pub states: [i32; 61],
}

impl FrameGauge {
    pub fn new(frames: u32) -> FrameGauge {
        FrameGauge {
            frames_total: 0,
            frames,
            color: ResColor { r: 255, g: 255, b: 255, a: 255},
            states: [0; 61],
        }
    }
}

pub fn update_frame_gauge(frames: u32, state: i32, gauge: &mut FrameGauge) {
    let mut color_offset: f32 = 0.0;
    if frames > 0 {
        if gauge.frames_total == 0 {
            gauge.frames_total = frames;
        }
        color_offset = (frames as f32 / gauge.frames_total as f32) * 255.0;
    } else if gauge.frames_total > 0 {
        gauge.frames_total = 0;
    }

    gauge.states.rotate_right(1);
    gauge.states[0] = state;

    gauge.frames = frames;
    gauge.color = ResColor { r: 255, g: 255 - color_offset as u8, b: 255 - color_offset as u8, a: 255 };
}

pub unsafe fn draw(root_pane: &mut Pane) {
    handle_hitstun_vis(root_pane);
    handle_frame_timeline(root_pane);
}

unsafe fn handle_hitstun_vis(root_pane: &mut Pane) {
    let visible = MENU.frame_advantage == OnOff::On;
    root_pane.find_pane_by_name_recursive("TrModHitstunDisp").unwrap().set_visible(visible);
    if !visible { return; }

    let player_frames_text = root_pane.find_pane_by_name_recursive("PlayerHitstunFrames").unwrap().as_textbox();
    let player_hitstun_frames = PLAYER_FRAME_GAUGE.frames;
    player_frames_text.set_text_string(&format!("{player_hitstun_frames}"));
    let color = PLAYER_FRAME_GAUGE.color;
    player_frames_text.set_color(color.r, color.g, color.b, color.a);


    let cpu_frames_text = root_pane.find_pane_by_name_recursive("CPUHitstunFrames").unwrap().as_textbox();
    let cpu_hitstun_frames = CPU_FRAME_GAUGE.frames;
    cpu_frames_text.set_text_string(&format!("{cpu_hitstun_frames}"));
    let color = CPU_FRAME_GAUGE.color;
    cpu_frames_text.set_color(color.r, color.g, color.b, color.a);

    let cpu_hitstun_gauge = (0..CPU_FRAME_GAUGE.frames_total)
        .map(|frame| if frame > CPU_FRAME_GAUGE.frames { ' ' } else { '|' })
        .collect::<String>();

    let player_hitstun_gauge = (0..PLAYER_FRAME_GAUGE.frames_total)
        .map(|frame| if frame > PLAYER_FRAME_GAUGE.frames { ' ' } else { '|' })
        .collect::<String>();

    root_pane.find_pane_by_name_recursive("PlayerHitstunGauge").unwrap().as_textbox().set_text_string(&player_hitstun_gauge);
    root_pane.find_pane_by_name_recursive("CPUHitstunGauge").unwrap().as_textbox().set_text_string(&cpu_hitstun_gauge);
}

unsafe fn handle_frame_timeline(root_pane: &mut Pane) {
    let visible = MENU.frame_timeline == OnOff::On;
    root_pane.find_pane_by_name_recursive("TrModFrameTimeline").unwrap().set_visible(visible);
    if !visible { return; }


    let cpu_frame_timeline = CPU_FRAME_GAUGE.states.iter().map(|state| if *state != 0 { "A" } else { "U" })
        .collect::<String>();

    let player_frame_timeline = PLAYER_FRAME_GAUGE.states.iter().map(|state| if *state != 0 { "A" } else { "U" })
        .collect::<String>();

    root_pane.find_pane_by_name_recursive("CPUFrameTimeline").unwrap().as_textbox().set_text_string(&cpu_frame_timeline);
    root_pane.find_pane_by_name_recursive("PlayerFrameTimeline").unwrap().as_textbox().set_text_string(&player_frame_timeline);
}