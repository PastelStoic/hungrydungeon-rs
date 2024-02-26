For the interface side of things, here's every action a user can take:

- Attack a target (generic "attack")
- Use an ability on a target (abilities have their own input requirements)
- Devour a target (counts as an ability?)
- Use an ability on themselves
- interact with something

Rather than trying to create interactive situations for every possible action
and ability, it's probably smarter to just have a single parser that works like
regular text adventure. "use devour on slime a", for example, would interpret
"use" to mean performing an action, "devour" as the ability, and "slime a" as
the target. This will require a consistent grammar and clear error messages, but
will be easier on the discord interface side.
