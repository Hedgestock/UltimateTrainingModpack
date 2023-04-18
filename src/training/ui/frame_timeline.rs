use once_cell::sync::Lazy;

use skyline::nn::ui2d::*;
use smash::ui2d::{SmashPane, SmashTextBox};

use training_mod_consts::OnOff;
use training_mod_consts::MENU;

pub static mut FRAME_TIMELINE: Lazy<FrameTimeline> = Lazy::new(|| FrameTimeline::new());

#[derive(Clone)]
pub struct FrameTimeline {
    pub player_states: [i32; 61],
    pub cpu_states: [i32; 61],
}

impl FrameTimeline {
    pub fn new() -> FrameTimeline {
        FrameTimeline {
            player_states: [0; 61],
            cpu_states: [0; 61],
        }
    }
}

pub fn update_frame_timeline(player_state:i32, cpu_state: i32) {
    if player_state == 0 && cpu_state == 0 {
        return;
    }
    unsafe {
        FRAME_TIMELINE.player_states.rotate_right(1);
        FRAME_TIMELINE.player_states[0] = player_state;
        
        FRAME_TIMELINE.cpu_states.rotate_right(1);
        FRAME_TIMELINE.cpu_states[0] = cpu_state;
    }
}

pub unsafe fn draw(root_pane: &mut Pane) {
    let visible = MENU.frame_timeline == OnOff::On;
    root_pane.find_pane_by_name_recursive("TrModFrameTimeline").unwrap().set_visible(visible);
    if !visible { return; }


    // let cpu_frame_timeline = CPU_FRAME_GAUGE.states.iter().map(|state| if *state != 0 { "A" } else { "U" })
    //     .collect::<String>();

    // let player_frame_timeline = PLAYER_FRAME_GAUGE.states.iter().map(|state| if *state != 0 { "A" } else { "U" })
    //     .collect::<String>();

    let player_frame_timeline = FRAME_TIMELINE.player_states.iter().map(|state| format!("{state} "))
        .collect::<String>();

    let cpu_frame_timeline = FRAME_TIMELINE.cpu_states.iter().map(|state| format!("{state} "))
        .collect::<String>();


    root_pane.find_pane_by_name_recursive("PlayerFrameTimeline").unwrap().as_textbox().set_text_string(&player_frame_timeline);
    root_pane.find_pane_by_name_recursive("CPUFrameTimeline").unwrap().as_textbox().set_text_string(&cpu_frame_timeline);
}