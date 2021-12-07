#[derive(Copy, Clone, PartialOrd, PartialEq)]
pub enum VgaModes {
    Text80x25,
    Graphics320x200,
    Graphics320x240,
    Graphics640x480,
    None,
}

impl VgaModes {
    pub fn address(self) -> Option<usize> {
        if self == VgaModes::Text80x25 {
            return Some(0xb8000);
        } else if self == VgaModes::Graphics320x200 || self == VgaModes::Graphics320x240 || self == VgaModes::Graphics640x480 {
            return Some(0xa0000)
        } else { return None }
    }
}

impl Default for VgaModes {
    fn default() -> Self {
        return VgaModes::None;
    }
}

pub fn convert_usize_to_vgamode(vgamode: usize) -> VgaModes {
    return match vgamode {
        0 => VgaModes::Text80x25,
        1 => VgaModes::Graphics320x240,
        2 => VgaModes::Graphics320x240,
        3 => VgaModes::Graphics640x480,

        _ => VgaModes::None,
    };
}
