import pygame


class Player(pygame.sprite.Sprite):

    def __init__(self, x, y, width, height):
        super().__init__()

        # Set the size and position of the player
        self.image = pygame.Surface([width, height])
        self.image.fill((255, 0, 0))
        self.rect = self.image.get_rect()
        self.rect.x = x
        self.rect.y = y

        # Set the speed of the player
        self.speed = 5

    def update(self):
        # Move the player based on user input
        keys = pygame.key.get_pressed()
        if keys[pygame.K_LEFT]:
            self.rect.x -= self.speed
        elif keys[pygame.K_RIGHT]:
            self.rect.x += self.speed
        if keys[pygame.K_UP]:
            self.rect.y -= self.speed
        elif keys[pygame.K_DOWN]:
            self.rect.y += self.speed

        # Keep the player within the game screen
        screen_width, screen_height = pygame.display.get_surface().get_size()
        if self.rect.left < 0:
            self.rect.left = 0
        elif self.rect.right > screen_width:
            self.rect.right = screen_width
        if self.rect.top < 0:
            self.rect.top = 0
        elif self.rect.bottom > screen_height:
            self.rect.bottom = screen_height
