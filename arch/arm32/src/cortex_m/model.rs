// Supported cpu models
pub const CPU_MODELS: [&str; 2] = ["Cortex-M3", "Cortex-M4"];

pub fn get_model(base_addr: u32) -> &'static str {
    return match base_addr {
        1091551793 => CPU_MODELS[0],
        1091551808 => CPU_MODELS[1],
        _ => "Unknown",
    };
}
