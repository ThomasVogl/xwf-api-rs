# Quick Start Guide with xwf-api-rs

## Prerequisites
- Skills 
  - some practical experience with [rust-lang](https://www.rust-lang.org/learn)
  - experienced user of [X-Ways Forensics](https://www.x-ways.net/forensics/index-d.html)
  - good understanding of the basic concepts of the [X-Ways X-Tension API](https://www.x-ways.net/forensics/x-tensions/api.html)
    - ideally you already went through all the pain of developing X-Tensions with low-level C
      
      (in this case I'm sure you will love this library 😉 )
- Installation of [Visual Studio Build Tools 2022](https://aka.ms/vs/17/release/vs_BuildTools.exe)
    - select Workload  _C++ Desktop Development_
    - also select Component _C++/CLI Support for v143-Buildtools (latest)_
- Installation of latest [rustup](https://www.rust-lang.org/tools/install)
  - Installation of rust toolchain _stable-x86_64-pc-windows-msvc_
- X-Ways Forensics **20.0 or higher** required
  - older XWF versions are not supported by this project, 
    as I do not intend to account for differing API feature levels in earlier versions.
- A set-up XWF-project with some evidences and files as a playground
  - a huge database of forensic image datasets from _NIST_ can be found [here](https://cfreds.nist.gov/)

## Compile and test Example Project
- clone the github source code, e.g. within terminal
  ```
  cd <your favourite path>
  git clone https://github.com/ThomasVogl/xwf-api-rs
  ```
  
- goto the cloned project path compile everything with cargo 
  ```
  cd xwf-api-rs
  cargo build --release
  ```
  
- now the compiled example X-Tensions (dll-File) can be found in `xwf-api-rs\target\release`
  - Load one of the compiled dlls into X-Ways via Main Menu (_<Shift+F8>_) to execute them.

- You can use one of the example projects as a starting point/boilerplate for your own X-Tensions:
  - You will just need the files `cargo.toml` and `src\lib.rs` from an example, nothing more.
    - copy these two files from xwf-api-rs git workspace or directly download them from github.
    - _xwf-api-rs_ dependency should be automatically pulled by cargo from github, you should not need the whole project source of _xwf-api-rs_
  - adapt **cargo.toml** to your needs
    - change project name and version
    - change API level feature for xwf-api-rs (feature _api\_\<major\>\_\<minor\>_) to the expected minimum version you want to support.
      
      For example, the feature _api\_20\_3_ will enable  X-Tension API features that were introduced by X-Ways in version 20.3.
      Of course you would need then at least this version for executing the X-Tension. (this is checked at runtime in XT_Init automatically)
  - load the new project in you favourite IDE. I personally would suggest [RustRover](https://www.jetbrains.com/de-de/rust/) for maximum convenience)
  - adapt **src/lib.rs** to your needs
    - change the name of the struct that implements XTension-trait
    - change the last line in the code:
      ```
      export_all_functions!(<name of static variable (can be anything)>, <struct-Name that impls XTension trait>);`
      ```
      This macro statement does all the magic of defining and exporting functions for the resulting DLL-Library
  - Have fun with rust and X-Ways!