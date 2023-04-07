use skyline::nn::ui2d::*;
use smash::ui2d::{SmashPane, SmashTextBox};

use crate::training::ui;

macro_rules! display_parent_fmt {
    ($x:ident) => {
        format!("TrModDisp{}", $x).as_str()
    };
}

macro_rules! display_header_fmt {
    ($x:ident) => {
        format!("TrModDisp{}Header", $x).as_str()
    };
}

macro_rules! display_txt_fmt {
    ($x:ident) => {
        format!("TrModDisp{}Txt", $x).as_str()
    };
}

pub unsafe fn draw(root_pane: &mut Pane) {
    let notification_idx = 0;

    let queue = &mut ui::notifications::QUEUE;
    let notification = queue.first();

    root_pane.find_pane_by_name_recursive(display_parent_fmt!(notification_idx))
        .unwrap().set_visible(notification.is_some());
    if notification.is_none() {
        return;
    }

    let notification = notification.unwrap();
    let color = notification.color;

    root_pane.find_pane_by_name_recursive(display_header_fmt!(notification_idx))
        .unwrap()
        .as_textbox().set_text_string(&notification.header);

    let text = root_pane.find_pane_by_name_recursive(display_txt_fmt!(notification_idx))
        .unwrap().as_textbox();
    text.set_text_string(&notification.message);
    text.set_default_material_colors();
    text.set_color(color.r, color.g, color.b, color.a);

    let notification = queue.first_mut().unwrap();
    let has_completed = notification.tick();
    if has_completed {
        queue.remove(0);
    }

    root_pane.find_pane_by_name_recursive("TrModHitstunDisp").unwrap().set_visible(true);//should hook to a menu thing

    let player_frames_text = root_pane.find_pane_by_name_recursive("PlayerHitstunFrames").unwrap().as_textbox();
    let player_hitstun_frames = ui::frame_gauge::PLAYER_FRAME_GAUGE.frames;
    player_frames_text.set_text_string(&format!("{player_hitstun_frames}"));
    let color = ui::frame_gauge::PLAYER_FRAME_GAUGE.color;
    player_frames_text.set_color(color.r, color.g, color.b, color.a);


    let cpu_frames_text = root_pane.find_pane_by_name_recursive("CPUHitstunFrames").unwrap().as_textbox();
    let cpu_hitstun_frames = ui::frame_gauge::CPU_FRAME_GAUGE.frames;
    cpu_frames_text.set_text_string(&format!("{cpu_hitstun_frames}"));
    let color = ui::frame_gauge::CPU_FRAME_GAUGE.color;
    cpu_frames_text.set_color(color.r, color.g, color.b, color.a);

    let cpu_hitstun_gauge = (0..ui::frame_gauge::CPU_FRAME_GAUGE.frames_total)
        .map(|frame| if frame > ui::frame_gauge::CPU_FRAME_GAUGE.frames { ' ' } else { '|' })
        .collect::<String>();

    let player_hitstun_gauge = (0..ui::frame_gauge::PLAYER_FRAME_GAUGE.frames_total)
        .map(|frame| if frame > ui::frame_gauge::PLAYER_FRAME_GAUGE.frames { ' ' } else { '|' })
        .collect::<String>();

    root_pane.find_pane_by_name_recursive("PlayerHitstunGauge").unwrap().as_textbox().set_text_string(&player_hitstun_gauge);
    root_pane.find_pane_by_name_recursive("CPUHitstunGauge").unwrap().as_textbox().set_text_string(&cpu_hitstun_gauge);

    let cpu_frame_timeline = ui::frame_gauge::CPU_FRAME_GAUGE.states.iter().map(|state| if *state != 0 { "A" } else { "U" })
        .collect::<String>();

    let player_frame_timeline = ui::frame_gauge::PLAYER_FRAME_GAUGE.states.iter().map(|state| if *state != 0 { "A" } else { "U" })
        .collect::<String>();

    root_pane.find_pane_by_name_recursive("CPUFrameTimeline").unwrap().as_textbox().set_text_string(&cpu_frame_timeline);
    root_pane.find_pane_by_name_recursive("PlayerFrameTimeline").unwrap().as_textbox().set_text_string(&player_frame_timeline);
}