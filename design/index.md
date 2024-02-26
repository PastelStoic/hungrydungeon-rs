In this document, find the overall plans for the development of the project.

Game concepts undocumented:

- Perks
- Status effects
- Vore
- Organs
- Actors

** Freedom of movement.

As a text-based game, there is no movement, and actors have no position. To
serve as a substitute, "openness" exists, determining how much freedom of
movement characters have. For example, if a character has their movement
restricted in a wide-open area, they won't be able to hit anyone with melee
attacks or devour them, though other characters can feed themselves to them.
While inside an organ, however, everyone is stuck within reach of each other, so
restricted movement means nothing. The openness of an area is relvant to both
movement and reach; I might have speed play a part in things, allowing fast
characters to dodge away from slow ones in melee.

** The initialize-modify-finalize loop

Whenever anything happens in the game, it happens in three steps. The initialize
step broadcasts that an action is going to be taken, and contains the source,
the targets (if any exist), the kind of action, and anything else relevant. The
modify step allows anything listening in to modify the the effect of the action,
such as reducing or increasing its damage. The finalize step applies the effect
of the action, and triggers things such as thorns or heal-on-hit. This step may
have events of its own, such as triggering poison or bleed, and these have their
own loop.

** State

All actors have a state, which determines what actions they're able to take.
Status effects like sleep, restrained, or devoured all change the state an actor
is in. Examples:

- Restrained movement: being held in place.

There's a "logging" system that gathers everything that happens in a single bevy
cycle and writes it to discord all at once. This prevents overloading of the
discord api.
