#![no_std]

pub fn str_to_addr(addr_str: &str) -> u32 {
    let mut addr = 0x0;
    
    for (i, c) in addr_str.chars().enumerate() {

        // This is a horrible way of doing it
        if c == '_' {
            continue;
        }

        if i == 2 {
            if c == '0' {
                addr += 0x0000_0000;
            } else if c == '1' {
                addr += 0x1000_0000;
            } else if c == '2' {
                addr += 0x2000_0000;
            } else if c == '3' {
                addr += 0x3000_0000;
            } else if c == '4' {
                addr += 0x4000_0000;
            } else if c == '5' {
                addr += 0x5000_0000;
            } else if c == '6' {
                addr += 0x6000_0000;
            } else if c == '7' {
                addr += 0x7000_0000;
            } else if c == '8' {
                addr += 0x8000_0000;
            } else if c == '9' {
                addr += 0x9000_0000;
            }

            if c == 'A' {
                addr += 0xA000_0000;
            } else if c == 'B' {
                addr += 0xB000_0000;
            } else if c == 'C' {
                addr += 0xC000_0000;
            } else if c == 'D' {
                addr += 0xD000_0000;
            } else if c == 'E' {
                addr += 0xE000_0000;
            } else if c == 'F' {
                addr += 0xF000_0000;
            }
        }

        if i == 3 {
            if c == '0' {
                addr += 0x0000_0000;
            } else if c == '1' {
                addr += 0x0100_0000;
            } else if c == '2' {
                addr += 0x0200_0000;
            } else if c == '3' {
                addr += 0x0300_0000;
            } else if c == '4' {
                addr += 0x0400_0000;
            } else if c == '5' {
                addr += 0x0500_0000;
            } else if c == '6' {
                addr += 0x0600_0000;
            } else if c == '7' {
                addr += 0x0700_0000;
            } else if c == '8' {
                addr += 0x0800_0000;
            } else if c == '9' {
                addr += 0x0900_0000;
            }

            if c == 'A' {
                addr += 0x0A00_0000;
            } else if c == 'B' {
                addr += 0x0B00_0000;
            } else if c == 'C' {
                addr += 0x0C00_0000;
            } else if c == 'D' {
                addr += 0x0D00_0000;
            } else if c == 'E' {
                addr += 0x0E00_0000;
            } else if c == 'F' {
                addr += 0x0F00_0000;
            }
        }

        if i == 4 {
            if c == '0' {
                addr += 0x0000_0000;
            } else if c == '1' {
                addr += 0x0010_0000;
            } else if c == '2' {
                addr += 0x0020_0000;
            } else if c == '3' {
                addr += 0x0030_0000;
            } else if c == '4' {
                addr += 0x0040_0000;
            } else if c == '5' {
                addr += 0x0050_0000;
            } else if c == '6' {
                addr += 0x0060_0000;
            } else if c == '7' {
                addr += 0x0070_0000;
            } else if c == '8' {
                addr += 0x0080_0000;
            } else if c == '9' {
                addr += 0x0090_0000;
            }

            if c == 'A' {
                addr += 0x00A0_0000;
            } else if c == 'B' {
                addr += 0x00B0_0000;
            } else if c == 'C' {
                addr += 0x00C0_0000;
            } else if c == 'D' {
                addr += 0x00D0_0000;
            } else if c == 'E' {
                addr += 0x00E0_0000;
            } else if c == 'F' {
                addr += 0x00F0_0000;
            }
        }

        if i == 5 {
            if c == '0' {
                addr += 0x0000_0000;
            } else if c == '1' {
                addr += 0x0001_0000;
            } else if c == '2' {
                addr += 0x0002_0000;
            } else if c == '3' {
                addr += 0x0003_0000;
            } else if c == '4' {
                addr += 0x0004_0000;
            } else if c == '5' {
                addr += 0x0005_0000;
            } else if c == '6' {
                addr += 0x0006_0000;
            } else if c == '7' {
                addr += 0x0007_0000;
            } else if c == '8' {
                addr += 0x0008_0000;
            } else if c == '9' {
                addr += 0x0009_0000;
            }

            if c == 'A' {
                addr += 0x000A_0000;
            } else if c == 'B' {
                addr += 0x000B_0000;
            } else if c == 'C' {
                addr += 0x000C_0000;
            } else if c == 'D' {
                addr += 0x000D_0000;
            } else if c == 'E' {
                addr += 0x000E_0000;
            } else if c == 'F' {
                addr += 0x000F_0000;
            }
        }

        if i == 6 {
            if c == '0' {
                addr += 0x0000_0000;
            } else if c == '1' {
                addr += 0x0000_1000;
            } else if c == '2' {
                addr += 0x0000_2000;
            } else if c == '3' {
                addr += 0x0000_3000;
            } else if c == '4' {
                addr += 0x0000_4000;
            } else if c == '5' {
                addr += 0x0000_5000;
            } else if c == '6' {
                addr += 0x0000_6000;
            } else if c == '7' {
                addr += 0x0000_7000;
            } else if c == '8' {
                addr += 0x0000_8000;
            } else if c == '9' {
                addr += 0x0000_9000;
            }

            if c == 'A' {
                addr += 0x0000_A000;
            } else if c == 'B' {
                addr += 0x0000_B000;
            } else if c == 'C' {
                addr += 0x0000_C000;
            } else if c == 'D' {
                addr += 0x0000_D000;
            } else if c == 'E' {
                addr += 0x0000_E000;
            } else if c == 'F' {
                addr += 0x0000_F000;
            }
        }

        if i == 7 {
            if c == '0' {
                addr += 0x0000_0000;
            } else if c == '1' {
                addr += 0x0000_0100;
            } else if c == '2' {
                addr += 0x0000_0200;
            } else if c == '3' {
                addr += 0x0000_0300;
            } else if c == '4' {
                addr += 0x0000_0400;
            } else if c == '5' {
                addr += 0x0000_0500;
            } else if c == '6' {
                addr += 0x0000_0600;
            } else if c == '7' {
                addr += 0x0000_0700;
            } else if c == '8' {
                addr += 0x0000_0800;
            } else if c == '9' {
                addr += 0x0000_0900;
            }

            if c == 'A' {
                addr += 0x0000_0A00;
            } else if c == 'B' {
                addr += 0x0000_0B00;
            } else if c == 'C' {
                addr += 0x0000_0C00;
            } else if c == 'D' {
                addr += 0x0000_0D00;
            } else if c == 'E' {
                addr += 0x0000_0E00;
            } else if c == 'F' {
                addr += 0x0000_0F00;
            }
        }

        if i == 8 {
            if c == '0' {
                addr += 0x0000_0000;
            } else if c == '1' {
                addr += 0x0000_0010;
            } else if c == '2' {
                addr += 0x0000_0020;
            } else if c == '3' {
                addr += 0x0000_0030;
            } else if c == '4' {
                addr += 0x0000_0040;
            } else if c == '5' {
                addr += 0x0000_0050;
            } else if c == '6' {
                addr += 0x0000_0060;
            } else if c == '7' {
                addr += 0x0000_0070;
            } else if c == '8' {
                addr += 0x0000_0080;
            } else if c == '9' {
                addr += 0x0000_0090;
            }

            if c == 'A' {
                addr += 0x0000_00A0;
            } else if c == 'B' {
                addr += 0x0000_00B0;
            } else if c == 'C' {
                addr += 0x0000_00C0;
            } else if c == 'D' {
                addr += 0x0000_00D0;
            } else if c == 'E' {
                addr += 0x0000_00E0;
            } else if c == 'F' {
                addr += 0x0000_00F0;
            }
        }

        if i == 9 {
            if c == '0' {
                addr += 0x0000_0000;
            } else if c == '1' {
                addr += 0x0000_0001;
            } else if c == '2' {
                addr += 0x0000_0002;
            } else if c == '3' {
                addr += 0x0000_0003;
            } else if c == '4' {
                addr += 0x0000_0004;
            } else if c == '5' {
                addr += 0x0000_0005;
            } else if c == '6' {
                addr += 0x0000_0006;
            } else if c == '7' {
                addr += 0x0000_0007;
            } else if c == '8' {
                addr += 0x0000_0008;
            } else if c == '9' {
                addr += 0x0000_0009;
            }

            if c == 'A' {
                addr += 0x0000_000A;
            } else if c == 'B' {
                addr += 0x0000_000B;
            } else if c == 'C' {
                addr += 0x0000_000C;
            } else if c == 'D' {
                addr += 0x0000_000D;
            } else if c == 'E' {
                addr += 0x0000_000E;
            } else if c == 'F' {
                addr += 0x0000_000F;
            }
        }
    }

    return addr;
}
