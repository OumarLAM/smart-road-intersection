# 🚗 Smart Road Project: The Future of Intersection Management 🚦

## Overview

Welcome to the **Smart Road Project**! This project tackles the challenge of creating a smart traffic control strategy for autonomous vehicles (AVs) at intersections without using traditional traffic lights. By leveraging advanced algorithms and animation, our goal is to minimize traffic congestion and avoid collisions.

## 🎯 Objectives

The main objectives of this project are to:
1. Develop a new traffic strategy algorithm for AVs at a cross intersection.
2. Simulate the movement of AVs without collisions.
3. Implement and animate the physics of AVs.
4. Display relevant statistics at the end of the simulation.

## 🛣 Intersection

We'll focus on a cross intersection where each lane has distinct routes:
- **r**: turning right
- **s**: going straight ahead
- **l**: turning left

```plaintext
               |   |   |   |   |   |   |
               |   |   |   |   |   |   |
               |   |   |   |   |   |   |
               |r  | s | l |   |   |   |
_______________| ← | ↓ | → |   |   |   |________________
                           |            ↑ r
_______________            |            ________________
                           |            ← s
_______________            |            ________________
                           |            ↓ l
___________________________|____________________________
           l ↑             |
_______________            |            ________________
           s →             |
_______________            |            ________________
           r ↓             |
_______________            |            ________________
               |   |   |   | ← | ↑ | → |
               |   |   |   | l | s | r |
               |   |   |   |   |   |   |
               |   |   |   |   |   |   |
               |   |   |   |   |   |   |
               |   |   |   |   |   |   |
```

## 🚀 Run the Project

### 📥 Clone the Repository:

```bash
git clone https://github.com/OumarLAM/smart-road-intersection.git
cd smart-road-intersection
```

### ⚙ Install Dependencies and Run the Project:

```bash
cargo build
cargo run
```

## 🎮 Simulation Controls

- ⬆ **Arrow Up**: Generate vehicles from south to north.
- ⬇ **Arrow Down**: Generate vehicles from north to south.
- ➡ **Arrow Right**: Generate vehicles from west to east.
- ⬅ **Arrow Left**: Generate vehicles from east to west.
- 🔄 **R**: Continually generate random vehicles.
- ⏹ **Esc**: End simulation and display statistics.

## 📊 Statistics

At the end of the simulation, display:

- 🚗 Max number of vehicles that passed the intersection.
- 🚀 Max velocity of vehicles.
- 🐢 Min velocity of vehicles.
- 🕒 Max time taken to pass the intersection.
- ⏱ Min time taken to pass the intersection.
- 🔄 Number of close calls (violations of the safe distance).

## Authors ✨

- [Oumar LAM](https://github.com/OumarLAM) 