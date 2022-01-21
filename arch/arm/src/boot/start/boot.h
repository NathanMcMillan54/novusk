void boot_die() {
    while (1) {
        asm("wfe");
    }
}