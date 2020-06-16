# General

Unity classes implement controller and view layer

Domain is responsible for all game logic

Domain can be implemented by a Fake to mock any functionality to View and Controller

Domain use a FFI implementation to send and receive messages

Domain interact with game specific objects

Server interact with low level packages

Server impl deal with bytes

# Namespaces

Package
kind: u16
bytes[]


## Domain

Responsible for game logic. Can be faked or connect to the logic by a FFI library.

## Rust

Game logic implementation

## Notes

There is no need to manage connections from Unity3d. It can always connect to rust through 
FFI and rust run things from local or implement the networking.