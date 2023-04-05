
use skyline::nn::ui2d::ResColor;
use once_cell::sync::Lazy;


pub static mut PLAYER_FRAME_GAUGE: Lazy<FrameGauge> = Lazy::new(|| FrameGauge::new(0));
pub static mut CPU_FRAME_GAUGE: Lazy<FrameGauge> = Lazy::new(|| FrameGauge::new(0));

#[derive(Clone)]
pub struct FrameGauge {
    pub frames_total: u32,
    pub frames: u32,
    pub color: ResColor,
}

impl FrameGauge {
    pub fn new(frames: u32) -> FrameGauge {
        FrameGauge {
            frames_total: 0,
            frames,
            color: ResColor { r: 255, g: 255, b: 255, a: 255},
        }
    }

    // Returns: has_completed
    // pub fn tick(&mut self) -> bool {
    //     if self.length <= 1 {
    //         return true;
    //     }
    //     self.length -= 1;
    //     false
    // }
}

pub fn update_frame_gauge(frames: u32, player: bool) { //mut gauge: &FrameGauge) {
    // (*gauge).frames = frames;
    // (*gauge).color = ResColor { r: frames as u8, g: 255, b: 255, a: 255 };
    unsafe{
        if player {
            let mut color_offset: f32 = 0.0;
            if frames > 0 {
                if PLAYER_FRAME_GAUGE.frames_total == 0 {
                    PLAYER_FRAME_GAUGE.frames_total = frames;
                }
                color_offset = (frames as f32 / PLAYER_FRAME_GAUGE.frames_total as f32) * 255.0;
            } else if PLAYER_FRAME_GAUGE.frames_total > 0 {
                PLAYER_FRAME_GAUGE.frames_total = 0;
            }
            // clear_notifications("TESTING");
            // let total = PLAYER_FRAME_GAUGE.frames_total;
            // color_notification(
            //     "TESTING".to_string(),
            //     format!("{frames}/{total}= {color_offset}"),
            //     60,
            //     ResColor { r: color_offset as u8, g: 0, b: 0, a: 255 },
            // );
            
            PLAYER_FRAME_GAUGE.frames = frames;
            PLAYER_FRAME_GAUGE.color = ResColor { r: 255, g: 255 - color_offset as u8, b: 255 - color_offset as u8, a: 255 };
        } else {
            let mut color_offset: f32 = 0.0;
            if frames > 0 {
                if CPU_FRAME_GAUGE.frames_total == 0 {
                    CPU_FRAME_GAUGE.frames_total = frames;
                }
                color_offset = (frames as f32 / CPU_FRAME_GAUGE.frames_total as f32) * 255.0;
            } else if CPU_FRAME_GAUGE.frames_total > 0 {
                CPU_FRAME_GAUGE.frames_total = 0;
            }
            CPU_FRAME_GAUGE.frames = frames;
            CPU_FRAME_GAUGE.color = ResColor { r: 255, g: 255 - color_offset as u8, b: 255 - color_offset as u8, a: 255 };
        }
    }
}