# General

Unity classes implement controller and view layer

Domain is responsible for all game logic

Domain can be implemented by a Fake to mock any functionality to View and Controller

Domain use a FFI implementation to send and receive messages

Domain interact with game specific objects

## Domain

## Notes

There is no need to manage connections from Unity3d. It can always connect to rust through 
FFI and rust run things from local or implement the networking.

Scene changes can cause destruction of game objects causing desync with game that generate
previous CreateObj messages. If scene change is require a new logic need to be implemented,
like query the domain for a full list (the same as a new player).