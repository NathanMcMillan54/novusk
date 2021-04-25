## Requirements for OS-dev with Novusk

Compiling:
- Cargo/Rust v1.51+
- GCC v9.3+
- Novusk v1.2+

---

Required functions:

``main_test`` - Returns ``i32``

This is for your operating system's tests. If the tests pass, return ``0``, else return ``1``.

``is_os`` - Returns ``bool``

If you're writing an OS with Novusk this function needs to return ``true``, if you're writing an extension to the kernel
for yourself it need to return ``false``

``os_name`` - Returns ``&'static str``

This needs to return the name of your OS, if your writing a kernel extension, return ``none``.

``is_initramfs`` - Returns ``bool``

If your OS deals with memory management or creates a temporary file system this function needs to return ``true``.

``initramfs_main`` - No return type required

If ``is_initramfs`` return ``true``, you should add a memory management functions and tasks to the ``initramfs_main`` 
function. If ``is_initramfs``returns ``false``, ``initramfs_main`` is still required but since it doesn't need a return 
type it can just return normally.

``kernfs_name`` - Returns ``&'static str``

This returns the name of the filesystem the kernel will use, if there isn't an initramfs function, the kernel will 
create a temporary filesystem using the filesystem you provide 

``kernfs_init`` - No return type required

This function should initialized the kernel filesystem, you can write your own or include a library that will make the
filesystem.

---

After you've included all required functions and added the things they should be doing (you can still just return a 
value supported for the type) you should look at the OS example and start writing your own.
