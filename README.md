# thrussh-hs

This is an expermient to see if I can create an SSH Library for Haskell using the a pure rust
 implementation of SSH via thrusssh.

## findings
  * It is possible to have rust and haskell communicate but they both have to
    talk using the C FFI of the respective lamnguage.
  * This approach feels really dumb, but it feels like the best shot that haskell
    has to interop with systems programming going forward. (Lest it be relegated
    to C forever)
  * I believe I have proven that the haskell library can both create an async runtime
    and initiate a session, but I've hit a roadblock with actually using the session
    for useful communication.
  * Haskell does NOT make loading libraries easy, and there isn't good documentation
    in case something goes wrong.
    * Want to load a rust library? Better not have debug symbols ¯\_(ツ)_/¯
  * Rust hates the FFI which isn't that surprising considering it basically throws
    data ownership out of the window.
  * Haskell also hates the FFI, but due to the fact that side effects abound.

## takeaways
  * I still think this is possible, just a little bit more than I could chew in one
    sitting.
  * I am not good at systems programming still, I need to lean into rust to learn more.
  * It's a shame because I'd rather stay in the Ivory tower but I have real work to do.

# How to build

```bash
sudo make clean
make build
sudo make install
stack clean
stack build
```

# Project organization

This project is really two projects in one directory. Since Haskell is particular, and
this is supposed to be a haskell library, the root is a stack based project. There is 
a special addition to this repository. Thats the `thrusshlib` directory that houses
a cargo project that builds the rust library. There is a makefile in the root directory
as well that serves to build the rust library and load it for linking.
