A list of the different monsterbrains and their behaviors.

# Glossary

## Range

Can be either ranged, melee, or self. Self skills don't have a target.

## Conditions

The requirements for this action to be considered. Unless otherwise indicated,
all actions require the actor to be active (not incapacitated) and at least one
valid target to exist (in cases where the action requires a target).

## Execution

What happens when the action is used.

## Influence

Factors that make the ability more or less likely to be used.

# Generic

Some actions are generic enough that they're functionally identical, aside from
the text. These are listed below.

Unless otherwise specified, all AI are capable of Single-target attack,
regenerate, and struggle. Any AI not listed have only those options.

- Single-target attack (melee)
  - Valid targets: all enemies in reach
  - Execution: target's hp is reduced based on attacker's attack stat.
- Vore (melee)
  - Conditions: actor has at least one organ with room for prey
  - Valid targets: all enemies in reach which the actor thinks can be eaten
  - Execution: target's parent is changed to one of the attacker's organs.
- Struggle
  - Conditions: actor is inside of an enemy's organ
  - Valid targets: the organ (actor's direct parent)
  - Execution: the organ's hp is reduced based on attacker's attack stat.
- Regenerate (self)
  - Conditions: no living enemies are the room
  - Execution: actor regains some hp.
- Belly rub other (melee)
  - Influence: more likely when fewer enemies in reach, more likely when ally is
    struggling to keep prey down
  - Valid targets: ally in reach with live prey
  - Execution: actor rubs ally's belly/other organ, which regenerates some HP
- Belly rub (self)
  - Influence: more likely when struggling to keep prey down
  - Execution: actor rubs belly/other organ, which regenerates some HP

# Slimegirl

- Vore
- Belly rub other
- Heal queen (melee)
  - Conditions: allied slime queen exists, is in reach, and has < 0 HP
  - Influence: more likely the lower slime queen HP is
  - Execution: slimegirl despawns, any prey inside her is moved inside the slime
    queen, slime queen recovers HP based on slimegirl max + current hp

# Slime Queen

- Vore
- Belly rub
