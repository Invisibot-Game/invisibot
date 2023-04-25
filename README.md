# Invisibot - bots that are invisible try to kill other bots that are also invisible.


## Decisions

### How to handle collisions?
  - Random?
  - No one moves
  - 'First in list'
  - Quickest response
  - Make it a mechanic?
    - Speed for each bot?

### How do we communicate with players?
 - Running the code directly, e.g. webassembly
 - Websockets
 - Grpc

#### How do we ensure fair tournaments (running on the server?)?
Suggestion: we use websockets but in tournament or online games, users upload a dockerfile that for example takes an environment variable and can be run by the server.

#### How do we ensure easy debugging locally?
Just use the websocket directly, no docker required!

### How do we trim down the games for storage/transfer to the frontend
 - Deltas? 
 - Are we fine to transfer the ENTIRE state? 
 - Streaming?

