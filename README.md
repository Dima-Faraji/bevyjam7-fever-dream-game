
# Fever Dream (Bevy Jam #7 prototype)

A small playable Bevy game prototype made for **Bevy Jam #7** (theme: *“Extremely Incohesive Fever Dream”*).

The goal is simple: **collect all “memories” before time runs out**.  
Movement feels different depending on your “mood” (Normal / Heavy / Sideways).

---

## Current status (what works right now)

✅ Main menu UI  
- Type player name
- Pick difficulty (Easy / Normal / Hard)
- Start game / Quit

✅ Gameplay loop  
- Countdown → Playing → Game Over
- Player movement (WASD / arrows)
- Mood switching (1 / 2 / 3) changes movement physics
- Maze walls + collision
- Collect “memories” to increase score
- Timer + HUD (mood / score / remaining time)
- Game Over overlay with:
  - Replay
  - Back to menu
  - Quit
  - Difficulty selection for next run

✅ Builds and runs with `cargo run` (Windows tested)

---

## Controls

### Menu
- Type name with keyboard
- `Enter` = Start
- `Esc` = Quit

### In game
- Move: `WASD` or arrow keys
- Mood:
  - `1` = Normal
  - `2` = Heavy
  - `3` = Sideways

### Game Over
- `Enter` or `R` = Play again
- `M` = Menu
- `Esc` or `Q` = Quit

---

## How to run (native)

```bash
cargo run
```

Expected result:
- A window opens titled **Fever Dream**
- You land in the menu
- You can start a run and play through to game over without panics

---

## Difficulty rules (current)

These are intentionally simple for now:
- **Easy**: longer time limit, fewer memories
- **Normal**: medium time limit, normal memories
- **Hard**: shorter time limit, more memories

Expected result:
- Changing difficulty changes how long you have and how many memories spawn

---

## Grade 5 “Tasks” + expected results (what I claim I delivered)

This is written as a checklist because that’s how we can evaluate it quickly.

### Task 1 — Playable game loop
- [x] Menu → Countdown → Playing → Game Over → Replay/Menu/Quit

**Expected result:** A new player can understand what to do within 30 seconds and finish a run.

### Task 2 — Basic UI and player input
- [x] Name input in menu
- [x] Difficulty selection in menu
- [x] HUD showing mood / score / time
- [x] Keyboard controls listed clearly

**Expected result:** The UI responds immediately and does not require mouse-only interaction.

### Task 3 — Game mechanics tied to the theme
- [x] “Mood” system that changes movement behavior (Normal / Heavy / Sideways)

**Expected result:** Switching mood clearly changes the feel of movement and can help or hurt the player.

### Task 4 — Code quality baseline
- [x] Clean builds: `cargo fmt`, `cargo check`, `cargo run`
- [x] Avoid ECS query conflicts (ParamSet / Without where needed)

**Expected result:** No runtime ECS panics and no borrow conflicts during gameplay.

---

## Known limitations / what I would improve next

- Graphics are currently mostly shapes / simple sprites (gameplay first).
- No sound/music yet (would help feel).
- Maze is static; no procedural generation.
- No enemies or hazards (could add “fever dream” events).
- No scoring leaderboard (local or online).

---

## Project structure (high level)

- `loading.rs` — asset loading + optional loading UI
- `menu.rs` — menu UI, name input, difficulty selection, start/quit
- `player.rs` — gameplay systems (movement, collision, HUD, timer, game over)
- `actions/game_control.rs` — keyboard input helpers

---

## Credits / references

Engine:
- Bevy: https://bevyengine.org/
- Bevy “Learn” (official): https://bevyengine.org/learn/
- Bevy Cheat Book (community, very practical): https://bevy-cheatbook.github.io/

Jam:
- Bevy Jam #7: https://itch.io/jam/bevy-jam-7

Asset loading helper:
- bevy_asset_loader: https://github.com/NiklasEi/bevy_asset_loader

Web build tool (if used later):
- Trunk: https://trunkrs.dev/

---

## License

Code: MIT License
Assets: see `credits/` 
