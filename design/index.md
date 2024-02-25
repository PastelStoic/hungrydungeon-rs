In this document, find the overall plans for the development of the project.

Game concepts undocumented:

- Perks
- Status effects
- Vore
- Organs
- Actors

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
is in.
