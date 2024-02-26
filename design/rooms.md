Rooms are the containers of all action in the game. Moving between rooms is the
only real form of movement the game has.

At startup, the database is queried to get all rooms, with their discord channel
id and biome type. This spawns in room objects according to their biome.

A room is "attached" to other rooms, in a way that a player can move between
them.

Every so often, a room will spawn an encounter, rolled randomly based on the
kind of biome it is.

Every actor needs to have triggers run whenever they enter a new room, to assign
them a letter id.
