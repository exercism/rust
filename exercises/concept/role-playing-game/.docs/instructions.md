# Instructions

You're working on implementing a role-playing game. The player's character is represented by the following:

```rust
pub struct Player {
    health: u32,
    mana: Option<u32>,
    level: u32,
}
```

Players in this game must reach level 10 before they unlock a mana pool so that they can start casting spells. Before that point, the Player's mana is `None`.

You're working on two pieces of functionality in this game, the revive mechanic and the spell casting mechanic.

The `revive` method should check to ensure that the Player is indeed dead (their health has reached 0), and if they are, the method should return a new Player instance with 100 health.
If the Player's level is 10 or above, they should also be revived with 100 mana.
If the Player's level is below 10, their mana should be `None`. The `revive` method should preserve the Player's level.

```rust
let dead_player = Player { health: 0, mana: None, level: 2 };
dead_player.revive()
// Returns Player { health: 100, mana: None, level: 2 }
```

If the `revive` method is called on a Player whose health is 1 or above, then the method should return `None`.

```rust
let alive_player = Player { health: 1, mana: Some(15), level: 11 };
alive_player.revive()
// Returns None
```

The `cast_spell` method takes a mutable reference to the Player as well as a `mana_cost` parameter indicating how much mana the spell costs. It returns the amount of damage that the cast spell performs, which will always be two times the mana cost of the spell if the spell is successfully cast.

- If the player does not have access to a mana pool, attempting to cast the spell must decrease their health by the mana cost of the spell. The damage returned must be 0.

  ```rust
  let not_a_wizard_yet = Player { health: 79, mana: None, level: 9 };
  assert_eq!(not_a_wizard_yet.cast_spell(5), 0)
  assert_eq!(not_a_wizard_yet.health, 74);
  assert_eq!(not_a_wizard_yet.mana, None);
  ```

- If the player has a mana pool but insufficient mana, the method should not affect the pool, but instead return 0

  ```rust
  let low_mana_wizard = Player { health: 93, mana: Some(3), level: 12 };
  assert_eq!(low_mana_wizard.cast_spell(10), 0);
  assert_eq!(low_mana_wizard.health, 93);
  assert_eq!(low_mana_wizard.mana, Some(3));
  ```

- Otherwise, the `mana_cost` should be deducted from the Player's mana pool and the appropriate amount of damage should be returned.

  ```rust
  let wizard = Player { health: 123, mana: Some(30), level: 18 };
  assert_eq!(wizard.cast_spell(10), 20);
  assert_eq!(wizard.health, 123);
  assert_eq!(wizard.mana, Some(20));
  ```

Have fun!
