Once every tick (90 seconds), every organ calculates damage. Each actor inside
it takes damage, and all food inside it gets digested partially, shrinking
according to a flat value.

For living things, being digested isn't determined by hp. Instead, a meter
called "digest saturation" builds slowly as they digest; when it hits max
(equaling the prey's max hp?) the prey is digested and becomes food to absorb.
The prey's health being lower speeds this up linearly, up to twice as fast at 0
hp.
