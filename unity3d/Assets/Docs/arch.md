# General

Unity classes implement controller and view layer

Domain is responsible for all game logic

Domain can be implemented by a Fake to mock any functionality to View and Controller

Domain use a Server implementation to send and receive messages

Server can have a LocalServer (FFI) or a RemoteServer (TCP/UDP)

Domain interact with game specific objects

Server interact with low level packages

Server impl deal with bytes