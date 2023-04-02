import pygame
from game_objects import Player

pygame.init()

pygame.display.set_caption("First Game")
win = pygame.display.set_mode((800, 800))

player = Player(x=50, y=50, width=4, height=6)
run = True

while run:
    pygame.time.delay(100)

    for event in pygame.event.get():
        if event.type == pygame.QUIT:
            run = False

    keys = pygame.key.get_pressed()

    player.update()

    pygame.draw.rect(win, (255, 0, 0), player.rect)
    pygame.display.update()

pygame.quit()
