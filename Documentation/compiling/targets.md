# Novusk Build Targets

To make a new target for an existing cross compiler, go to a ``targets`` directory in an architecture specific 
directory and run this command:

```commandline
rustc -Z unstable-options --print target-spec-json --target cross-compiler-name > targets/target-name.json
```

If you're making a target for aarch64 put it in ``boards/board-name.txt``, at the top of the file add the board name 
and anything else that your board will need.
