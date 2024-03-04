Step 1: have two slimes spawn, just taking turns hitting each other. Will test
actor spawning, health, and the event system.

Step 2: have one of the slimes try to eat the other. Will test AI
decision-making and vore.

I've succeeded partially, but I have a problem: right now, the AI actions are
decided in one go, then executed in another. What happens if, as the result of
one action, another is no longer valid?

Have each action check if it's still valid before it executes. If not, trigger
the eventwriter again to send the ai action back into the processing queue.
