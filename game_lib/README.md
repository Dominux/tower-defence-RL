# Tower defence game lib

Extremely simple tower defence game library to use as a backend/kernel to create tower defence games for ordinar users. 

Includes minimal business logic that needs to run any tower defence game. Should be used via convenient interfaces and facades. Or can be used just programmatically. For any idea its simplicity suits best

### Gameplay rules

This description helps to understand what the exact gameplay and entities set this library suggests.

The main cycle of the game is pretty simple:

1. In the start player possesses some money. 
2. The player can spend them onto bulding towers and their further enchancing.
3. Money can be retrivied from killing monsters.

Overall idea that comes to mind:

1. The game tends to become harder every round/wave: monsters become stronger and stronger in arephmetic progression.
2. Since amount of money is equil to the monster level, it's being inscreased too, equially.

And facts that comes to mind in the end:

1. Towers cannot be enchanced eternally.
2. So in some time interval the original set of available towers is being extended with new, advanced, and more expensive ones.
3. The player can remove towers

### Under the hood

Since the time management is complex, bots can make moves in a nanosecond and so on, __the game library itself does not provide any time limits__ and leaves it to its implementing clients. 
The game process is set like the player manages towers (building, enchancing, removing) until they are ready to stop. And then the wave of monsters trying to go through the way. The next player's move starts with the result of a previous monsters wave: whether they made some damage, defeated the target or did nothing, and the money the player received from killing them with the towers.

