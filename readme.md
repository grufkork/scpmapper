# scpmapper
A simple tool that helps you navigate in SCP: Secret Lab without injecting/hacking the game itself. 

At the moment, it can identify what Light Containment Zone layout is active, starting from the D-Class spawn room. Use as following:

For each room you walk through, press:
- Numpad 4 or Left Arrow if you took a left turn
- Numpad 8 or Up Arrow if you continued forward
- Numpad 6 or Right Arrow if you took a right turn
- Numpad 5 or Down Arrow if you ended up in a room
If you mistype, press Numpad zero or backspace to remove a char.

Start pressing only once you have left the spawn room (spawn doesn't count as a corridor). For example, if you exit spawn, encounter a T-intersection in which you turn left, move straight through an X intersection, then move straight through a plain corridor you would press left-forward-forward.

As soon as a unique match is found the map will be opened with your default program for .PNGs. This might however defocus the game for a second, so make sure 173 hasn't just spotted your soft, malleable neck!

### Future Features
If I feel like working on it I'll see if I can make it  find the layout and player position on any of the levels, starting from anywhere. I haven't actually explored the plausibility of this as a lot of parts are similar. It might be very computationally expensive to check every rotation and position, but again, I currently have no idea about the complexity of it.

Also, make no remarks about the architecture/general programming, it's not clean, particularly extensible or efficient but it works :D. Rust is fast and the task it is doing is minimal. Though if you still feel very strongly about, do please refactor my code.
There's also lots of dead/commmented code and assets, might clean up sometime...