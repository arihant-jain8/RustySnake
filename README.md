# 🐍 RustySnake

RustySnake is a classic **Snake Game** built in Rust using the [Piston](https://www.piston.rs/) game engine.  
It’s a fun project to learn Rust, game loops, and rendering while recreating one of the most iconic arcade games.

---

## 🎮 Features
- Smooth snake movement with arrow keys **and** WASD controls
- Food spawning with random placement
- Collision detection with walls and snake’s own tail
- Game over screen overlay
- Automatic restart after delay or by pressing any key
- Configurable board size


## 🚀 Getting Started

## Prerequisites
- Rust (latest stable version). Install via [rustup](https://rustup.rs/).
- Cargo (comes with Rust).


## Clone the repo
```bash
git clone https://github.com/arihant-jain8/RustySnake.git
cd RustySnake
```

## Run the game
```bash
cargo run
```

## ⌨ Controls
- Arrow Keys or WASD → Move the snake
- Any key after Game Over → Restart the game
- Esc → Exit the game

## ⚙️ Configuration
You can tweak game parameters in game.rs:
- MOVING_PERIOD → Snake speed (lower = faster)
- RESTART_TIME → Delay before auto-restart
- Board size → Set in main.rs with (width, height)

## 📸 Screenshots
<img width="310" height="322" alt="image" src="https://github.com/user-attachments/assets/f70ef45b-f44c-4a1f-8ed3-d69955a71b08" />
<img width="312" height="322" alt="image" src="https://github.com/user-attachments/assets/ed369613-1aa5-4631-aa92-258420541287" />
<img width="307" height="321" alt="image" src="https://github.com/user-attachments/assets/647a5e39-8680-48da-9445-76a5692e8175" />
<img width="313" height="322" alt="image" src="https://github.com/user-attachments/assets/18db1866-6283-4dee-9f7e-5cdc4ba8b34e" />


## 🛠️ Built with
- [Rust](https://www.rust-lang.org/)
- [Piston Window](https://crates.io/crates/piston_window)
- [Rand](https://crates.io/crates/rand)

## 🤝 Contributing
Contributions, issues, and feature requests are welcome!Feel free to fork this repo and submit a pull request.


---


## 🙏 Credits
This game was created by following a tutorial by [Rustlang Project: Snake Game - Tensor Programming](https://www.youtube.com/watch?v=DnT_7M7L7vo&t=569s). Many thanks for the clear guidance and inspiration!
