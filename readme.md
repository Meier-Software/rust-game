# The Game Design

# Game Loop
```sh
A - Join hub.
B - Join another zone.
C - Complete mini game or other such event.
D - Collect items to customize character.
E - GOTO B
```


# Architecture
## Server
I have broken the servers architecture up into a handful of services.

### Acceptor
This service accepts network connections and spawns off a server side client component

### Client
This is a server side client connection. 
```sh
A - loops till authed
B - when authed
C - spawn Player component
```

### Player
This is a game process that represents a clients player.
This can be around even without a client around. 
For example for anti-combat-logging.


## Protocol
- [ ] Add in placeholder text support with {i} to replace it with the currently held item for example
 - [ ] Add more placeholders for things like head chestplate and boots. {h} {c} {b}
 - [ ] Add in a placeholder meant to be filled client side ~{}


## Client
This is the component I will be working on the least.

### Rendering
There is a draw loop happening in the engine.

### Networking
There is a network loop sending and recieving events to the server.

### Game Logic
Deal with basic stuff here like gravity calculation of player movement prediction.

As for Ticks Per Second: I'm thinking about 10? or maybe 5.