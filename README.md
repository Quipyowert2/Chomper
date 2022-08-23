This is a Pacman game with a twist written in Rust, using the SDL2 library.

Instead of eating ghosts, eat other pacmen to increase your score. Score is based on current size and starting size which is currently hardcoded at forty for the player.

Use the arrow keys for movement and F7 to toggle FPS display. Currently the FPS is rather low even at the start of the game, so I should probably optimize the code.

Known bugs:
    1. enemy pacmen get stuck on edges of screen.