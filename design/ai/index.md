All ai have a series of possible actions they can take, each of which has a way
of producing a ranking, either 0 (action should not be taken) or a positive
non-zero integer representing how strongly the ai wants to take the action.

There's lots of different event types, and each of them have a different set of
parameters. Should I make it into an enum? Would that get too big, or too
complex? Would it be too awkward, to have all the different actions and events
passed around fitting in a single function? I'll worry about that later.

- Basic attack: Requires the attack value of the attacking entity, as well as
  their Entity for possible retaliation.
- Vore: requires the pred, the prey, and the organ the pred is going into.
- Feed: requires the feeder, the feedee, the food, and the organ.

AI flow:

1. Every game timer trigger, AI are polled to see if they're in a state to act.
   If so, they send an AiShouldRunEvent.
2. The event is received, and the AI chooses what to do. This creates an
   AiRunActionEvent, depending on what choice they make.
3. This event is received, and the action is checked to see if it's still valid.
   If not, an AiShouldRunEvent is sent again, restarting the loop. If so, the
   action runs.

It now seems likely that every possible action will require its own system, its
own event, and its own process cycle. The code is likely to grow large, over
time, but hopefully the added clarity will make up for it.
