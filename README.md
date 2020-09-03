# Legio-Parser
This is a Rust library for parsing data using combinatory logic.

## No unsafe code!
The use of unsafe code is forbidden within the library in order to provide purely safe interfaces.

## Features
* `std`
    * This feature provides interfaces that use the standard library. E.g.: `CollectingMatch`.
	* Opting-out this feature will make the library use just the `libcore` while limiting functionality.
    * This feature is turned on by default.
