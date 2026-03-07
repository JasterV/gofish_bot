# ⚠️ This repo has been migrated ⚠️

Mainly for philosofic reasons I've decided to migrate all my projects to a free software alternative forge ([Codeberg](https://codeberg.org/)).

You can find the most recent version of this repository [here](https://codeberg.org/JasterV/gofish_bot).

I encourage any free software activist to follow the same steps, as it doesn't make sense to make use of a propietary software to host our projects if we advocate for free software.

# gofish_bot

> Bot build with Teloxide to play gofish with my friends

@peter-fisher-bot

## Game description

```
Each participant will receive a private message with their status (cards in hand & score) after each turn.

Ask someone for a card, if they have it they will give you all the cards with that number, otherwise you will automatically draw a card.

If the card you've drawn is the same you asked for you can continue asking, otherwise the turn ends.

The game ends if someone has 0 cards in hand!
```

## Build

```cargo build [--release]```

## Run

```cargo run```



