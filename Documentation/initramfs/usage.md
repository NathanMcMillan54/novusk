# Usage

### What is an initramfs?

An initramfs is a Ram FileSystem, is a temporary file system that stores memory in a file so RAM doesn't have to.

### Why should it be used?

If your OS or application is using too much memory, you can store the memory that isn't being used much in the Ram 
FileSystem so more memory could be used. This can make a process run slower because it has to read a file instead of 
reading memory, but atleast you can use more memory.

---

Everything stored in the Ram FileSystem should be deleted on shutdown so nothing important or anything that should be 
saved to storage should be in it. If you're on Linux, you probably have a directory in your root called ``/tmp``, this 
is like a Ram FileSystem everything is temporary (that's why it's called "tmp").
