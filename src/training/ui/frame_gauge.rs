
use skyline::nn::ui2d::ResColor;
use once_cell::sync::Lazy;


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
        let mut test: [i32; 61] = [0; 61];
        test[30] = 1;
        FrameGauge {
            frames_total: 0,
            frames,
            color: ResColor { r: 255, g: 255, b: 255, a: 255},
            states: test,
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