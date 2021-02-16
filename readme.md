# scpmapper
A simple tool that helps you navigate in SCP: Secret Lab without injecting/hacking the game itself. 

Get the latest version from the [Releases](https://github.com/grufkork/scpmapper/releases) or download and run it yourself.

## Usage
Being with selecting your current zone with either the arrow keys or the numpad. Then go to any room in the level (D-Class spawn and the entrance/heavy gate counts). Then start walking anywhere and input where you go: 

For each room you walk through, press:
- Numpad 4 or Left Arrow if you take a left turn
- Numpad 8 or Up Arrow if you continue forward
- Numpad 6 or Right Arrow if you take a right turn
- Numpad 5 or Down Arrow if you end up in a room

If you mistype, press Numpad zero or backspace to remove a char.

Start pressing only once you have left the room (it doesn't count as a corridor). For example: If you exit, encounter a T-intersection in which you turn left, move straight through an X intersection, then move straight through a plain corridor you would press left-forward-forward.

As soon as a unique match is found the map for the level is displayed. The red square is where you started, and green is where you are now. Below your facing is shown. 

There are some things to consider though: 

- Try not to pick a too short path. Entering a room might be what's needed to get a unique match, but in most cases there will be multiple matches and you'll have to start over. Just go through corridors until it matches.
- Certain paths and levels might take a couple of steps to find a unique path. Try not to go in loops, as multiple paths feed into them.
- Some paths are never unique. In particular, symmetrical maps have paths that can be rotatated to fit in many places. Especially bad is the HCZ map "Window", which has a pattern with very few unique features. Not all paths can be picked out uniquely. 
- The maximum length of a path is 10 steps, to stop loops. This is an arbitrary number though, but any longer than that and you have already found all rooms. Longer paths -> longer startup times as well.

As far as I have tested, LCZ requires the shortest paths. HCZ is hit-or-miss, sometimes the unique paths are very short but sometimes you might never find a match. In EZ you will find eventually, but large parts can be similar.

### Etc
You pretty much need a separate monitor for this to work, or a transparent terminal that you can overlay the game.

Currently only supported on Windows, but I think Linux should work out-of-the-box. Anyways, if you got SCP:SL running on Linux, I'm sure you can figure this out. 

Make no remarks about the architecture/general programming, it's not clean, particularly extensible (I don't think NW will make new maps anytime soon though) or efficient but it works. Rust is fast and the task it is doing is minimal. Though if you still feel very strongly about it, do please refactor my code :D

### Credits
Credit to downloadpizza for the layouts as text files, here:
https://github.com/downloadpizza/scp-sl-layouts

Which in turn are (at least partially) based on https://steamcommunity.com/sharedfiles/filedetails/?id=1943822063