# ttx4-usbio
TTX4 USB IO card tools and hopefully emulation soon...

## Love Live Button Functions

The data sent from the io card is a 64 byte buffer (possibly for each pin on the board, but this is just a theory).

The pins do not correlate with the location in the array sent from the io card!

Below is a table with the physical pin of the button and what value is added to the buffer when it is pressed. (Buttons can be pressed to add up to 255 base 10, in various locations). All addition is in base 10. 

IO Pin| Button | Add -> | Buffer Location |
---|---|---
13 | Door L SW | x | x
14 | Door R SW | x | x
25 | Rythm Button 1 | x | x
26 | Rythm Button 2 | x | x
27 | Rythm Button 3 | x | x
28 | Rythm Button 4 | x | x
29 | Rythm Button 5 | x | x
30 | Rythm Button 6 | x | x
31 | Rythm Button 7 | x | x
32 | Enter SW | x | x
33 | Rythm Button 8 | x | x
34 | Rythm Button 9 | x | x
35 | Cansel Button | x | x
36 | Decision Button | x | x
37 | Select SW | x | x
38 | Test SW | 32 | buffer[1]
39 | Service SW | x | x
40 | Coin SW | x | x

