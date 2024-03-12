Step 1: have two slimes spawn, just taking turns hitting each other. Will test
actor spawning, health, and the event system.

Step 2: have one of the slimes try to eat the other. Will test AI
decision-making and vore.

I've succeeded partially, but I have a problem: right now, the AI actions are
decided in one go, then executed in another. What happens if, as the result of
one action, another is no longer valid?

Have each action check if it's still valid before it executes. If not, trigger
the eventwriter again to send the ai action back into the processing queue.

Idea: since attack actions are pretty standardized, what if I had a generic
"actionfailedevent", and depending on whether the actor was an AI or a player I
could either rerun the ai decision tree or print an error message.
Theoretically, it's wasted memory usage if an error message is created that
never gets seen, but it's probably worth it for simplicity.

Player input is the new struggle, so here's the flow:

1. Input is received by system, creating a PlayerCommandInput event
2. event is parsed to figure out what the player wants to do
3. Same system as 2 figures out whether this is even possible (targets exist,
   are in reach, etc)
4. If so, new event is created to actually execute the action, same as an AI
   would have

The input parser needs support for names that have multiple words in them.

Rather than using events, I should replace most actions with oneshot systems.
Rather than a specific ai struct for each ai and player, I can have a
Behavior(SystemId) component that holds the system id that runs the ai.

Problem: The systemId is created once, at runtime. How do I make sure all
entities of a given type spawn with that speicific entity ID?

One option is to just re-register the system for each spawned entity. Slow?
Maybe, but also maybe not - it's what's easy, so how about I just do that and
then see if it's an issue.
