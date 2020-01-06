# importing wasmtime allows us to import .wasm modules as if they were python modules
import wasmtime
import wasm_conway

import random
import math
import time
import pygame as pg

# constants
WINSIZE = [1600, 900]
CELL_SIZE = 10
U32_MAX = (1 << 32) - 1
W = int(WINSIZE[0] / CELL_SIZE)
H = int(WINSIZE[1] / CELL_SIZE)

def main():
    random.seed()

    # Initialize the game with random seed
    game = wasm_conway.game_of_life_new(random.randint(0, U32_MAX), W, H)

    clock = pg.time.Clock()
    pg.init()
    screen = pg.display.set_mode(WINSIZE)
    pg.display.set_caption("PyGame WASM Conway")

    white = 255, 255, 255
    black = 20, 20, 40

    try:
        while True:
            screen.fill(black)
            for x in range(W):
                for y in range(H):
                    if wasm_conway.game_of_life_get(game, x, y) == 0:
                        pg.draw.rect(screen, white, (x * CELL_SIZE + 1, y * CELL_SIZE + 1, CELL_SIZE - 1, CELL_SIZE - 1))

            pg.display.update()
            wasm_conway.game_of_life_tick(game)
            clock.tick(100)
    except Exception as e:
        print(e)
    finally:
        wasm_conway.game_of_life_free(game)

if __name__ == "__main__":
    main()
