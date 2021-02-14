# Novusk
New Kernel - A small kernel written in Rust

Read ``Documentation/`` for setting up or developing

### x86 TODO list
- [x] Make proper support for a screen size of 200 (default)
- [ ] Make proper support for a screen size of 6400
- [ ] Make shutdown work
- [ ] Add real kernel clock

### arm TODO list
- [x] Make arm a supported architecture
- [x] Make aarch64(arm64) a supported architecture
- [ ] Create a text mode

arm/aarch64 doesn't have a text mode like how x86/x86_64 has VGA Buffer, right now Novusk arm/aarch64 only prints in 
qemu.