# FRI Challenger ♟️

A  chess engine written in **Rust**. This project is the culmination of a diploma thesis for the **Faculty of Computer and Information Science (FRI)** at the **University of Ljubljana**.


| Version | Current ELO Rating (Est.) |
| :--- | :--- |
| `0.6.0` | ~UNKNOWN |
| `0.5.0` | ~UNKNOWN |
| `0.4.0` | ~UNKNOWN |
| `0.3.0` | ~UNKNOWN |
| `0.2.0` | ~UNKNOWN |
| `0.1.0` | ~UNKNOWN |

## 🧠 Core Engine Features & Algorithms

The engine is built on standard and modern chess programming principles:

#### Move Generator
- Semi-legal move generator
- Move Ordering (PV, Killers, History heuristics, SEE)
#### Search 
- Iterative Deepening
- Principal Variation Table (Upgraded Alpha Beta search)
- Quiescence search
- NMP + NMR
- LMR (Late move pruning) (Added 1 More depth to the search)
#### Evaluation
- Material
- PSQT (Piece Square Table)
- Imbalance
- Pawns
- King
- Mobility
- Threats
- Passed Pawn  #(Fixed in this release)
- Space
- Tempo

## 🛠️ Getting Started (Development Setup)

This project uses **Docker** to ensure a consistent, reproducible build environment.

### Prerequisites

To contribute to or run the engine in a development environment, you will need:

1.  **Visual Studio Code** (VS Code).
2.  The **Dev Containers** extension for VS Code.

### 🐳 Run with VS Code Dev Containers (Recommended)

This method automatically sets up the complete Rust toolchain and environment inside a container.

1.  Clone the repository to your local machine.
2.  Open the cloned folder in **Visual Studio Code**.
3.  When prompted by the **Dev Containers** extension, click **"Reopen in Container"**.
4.  VS Code will build the Docker image and start the development container, providing you with a standardized, pre-configured environment.

### 💻 Manual Docker Execution

If you prefer to build and run the executable manually:

1.  **Build the Docker image:**
    ```bash
    docker build -t fri-challenger:0.6.0 .
    ```
2.  **Run the engine container (and start the UCI interface):**
    ```bash
    docker run -it fri-challenger:0.6.0 /bin/bash -c "./target/release/fri-challenger"
    ```

***

## 🌐 Lichess Integration


* **Lichess Profile:** *[Link will be added here upon deployment]*
