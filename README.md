# FRI Challenger ♟️

A high-performance chess engine written in **Rust**. This project is the culmination of a diploma thesis for the **Faculty of Computer and Information Science (FRI)** at the **University of Ljubljana**.

*The name "FRI Challenger" pays homage to the faculty where it was created.*

***

## 👤 Author

* **Nikola Simjanovski**

***

## 🚀 Project Status

| Metric | Detail |
| :--- | :--- |
| **Current ELO Rating (Est.)** | ~2000 |
| **Current Version** | `0.6.0` |
| **Technology** | Rust |

### 🎯 Roadmap to v1.0.0 (First Release)

The engine is currently in an advanced alpha state. The first official stable release will be version `1.0.0` and is planned once the following key search optimizations are implemented:

* **Futility Search**
* **Transposition Table**

***

## 🧠 Core Engine Features & Algorithms

The engine is built on standard and modern chess programming principles:

* **Alpha-Beta Pruning:** The primary search optimization for the minimax algorithm.
* **Iterative Deepening:** A search strategy that increases search depth incrementally, allowing for time management and better move ordering.
* **Quiescence Search:** A specialized search extension used to evaluate positions only after tactical exchanges have settled (to mitigate the **horizon effect**).

***

## 🛠️ Getting Started (Development Setup)

This project uses **Docker** to ensure a consistent, reproducible build environment.

### Prerequisites

To contribute to or run the engine in a development environment, you will need:

1.  **Docker Desktop** (or equivalent Docker installation).
2.  **Visual Studio Code** (VS Code).
3.  The **Dev Containers** extension for VS Code.

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

We plan to create an official **FRI Challenger** bot on **Lichess** to participate in rated games and provide public access for testing.

* **Lichess Profile:** *[Link will be added here upon deployment]*