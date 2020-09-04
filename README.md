# Legio-Parser
This is a Rust library for parsing data using combinatory logic.

## No unsafe code!
The use of unsafe code is forbidden within the library in order to provide purely safe interfaces.

## Features
* `std`
    * This feature provides interfaces that use the standard library. E.g.: `CollectingMatch`.
	* Opting-out this feature will make the library use just the `libcore` while limiting functionality.
    * This feature is turned on by default.
* `no_track_caller`
    * Disables the `#[track_caller]` attributes within the library.
    * This is required for compilation below version 1.46 of Rust.

## How to include into project?
* Variant 1 - Use latest version
    
    Write this under the `[dependancies]` section:
    ```
    legio-parser = { git = "https://github.com/Dark-Legion/Legio-Parser.git", branch = "release" }
    ```
* Variant 2 - Use latest version
    ```
    [dependencies.legio-parser]
    git = "https://github.com/Dark-Legion/Legio-Parser.git"
    branch = "release"
    ```
* Variant 3 - Use specific version
    ```
    [dependencies.legio-parser]
    git = "https://github.com/Dark-Legion/Legio-Parser.git"
    tag = "v0.1"
    ```
