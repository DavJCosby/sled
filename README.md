# SLC - Spatial Light Controller
SLC-core (pronounced silk) handles the core logic for my custom strip light controller.

## Duties
SLC-core is in charge of LED strips and their location in the room, individual pixel colors, and the observer's position and orientation in the room.

## Disclaimer
SLC-core is *not* in charge of interacting with any hardware components or pins or displaying colors in any way. CLC is simply a module that handles the relationship between the observer and the leds in spatial terms.

SLC-core is also not in charge of setting the color of its own pixel objects; rather, it exposes methods for other crates to interact with. See RoomController for more information.
