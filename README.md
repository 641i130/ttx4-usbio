# ttx4-usbio
TTX4 USB IO card tools and hopefully emulation soon...

## Love Live Button Functions

The data sent from the io card is a 64 byte buffer (possibly for each pin on the board, but this is just a theory).

The pins do not correlate with the location in the array sent from the io card!

Below is a table with the physical pin of the button and what value is added to the buffer when it is pressed. (Buttons can be pressed to add up to 255 base 10, in various locations). All addition is in base 10.
Another note, buttons can affect multiple buffer locations (probably for turning on lights on the arcade machine or increasing the coin counter for example.
Buffer values 16-31 seem to be random values, though the odd numbers in that range don't change as sparatically as the even ones.
| IO Pin | Button          | Add -> | Buffer Location |
|--------|-----------------|--------|-----------------|
| 13     | Door L SW       | -1     | buffer[9]       |
| 14     | Door R SW       | -2     | buffer[9]       |
| 25     | Rythm Button 1  | 1      | buffer[6]       |
| 26     | Rythm Button 2  | 2      | buffer[6]       |
| 27     | Rythm Button 3  | 4      | buffer[6]       |
| 28     | Rythm Button 4  | 8      | buffer[6]       |
| 29     | Rythm Button 5  | 16     | buffer[6]       |
| 30     | Rythm Button 6  | 32     | buffer[6]       |
| 31     | Rythm Button 7  | 64     | buffer[6]       |
| 32     | Enter SW        | 128    | buffer[6]       |
| 32     | Enter SW        | 255    | buffer[38]      |
| 32     | Enter SW        | 255    | buffer[39]      |
| 33     | Rythm Button 8  | 1      | buffer[4]       |
| 34     | Rythm Button 9  | 2      | buffer[4]       |
| 35     | Cansel Button   | x      | x               |
| 36     | Decision Button | 8      | buffer[4]       |
| 37     | Select SW       | 16     | buffer[4]       |
| 38     | Test SW         | 32     | buffer[4]       |
| 39     | Service SW      | 64     | buffer[4]       |
| 40     | Coin SW         | 128    | buffer[4]       |
| 40     | Coin SW*        | 1      | buffer[12]      |
| 40     | Coin SW**       | 64     | buffer[13]      |

\* The coin switch adds one up to 255 (does not subtract when released) `buffer[12]`. When it hits 255 it goes back to 0 and continues to add 1.
** this only is added to if you hold the coin switch down for more than a second...


### Video example:
![](https://raw.githubusercontent.com/641i130/ttx4-usbio/main/assets/monitor.mp4)
