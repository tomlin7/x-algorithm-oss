# X Algorithm OSS

This repository contains the open-source port of the **X Recommendation Algorithm** backend.

![Y Platform Screenshot](./screenshot.png)

### X-Algorithm Backend (OSS Port)
A fully buildable and runnable Rust port of the original X algorithm `home-mixer` service.
- This port is built on Rust, Tonic (gRPC), Axum, Tokio.
- **Simulation Mode**.
  - Generates 10,000 in-memory candidate tweets.
  - Simulates engagement scoring (Likes, Replies, Retweets) using randomized probability models.
  - Runs the full ranking pipeline (Candidate Source -> Filters -> Scoring -> Mixing) locally.

### Y
A modern, "everything app" frontend designed to interface with the X-Algorithm.
- Connect-RPC (gRPC-Web) directly to the Rust backend.
- Features:
  - Live stream of posts ranked by the algorithm.
  - Client-side feed updates for instant interactivity.
  - Like, Repost, Bookmark actions with optimistic updates.
  - Fully functional Sidebar and sub-pages (Explore, Profile, etc.).

## Architecture

```mermaid
graph TD
    User[User (Browser)] -->|HTTP/gRPC-Web| Frontend[Y Platform (Next.js)]
    Frontend -->|gRPC GetScoredPosts| Backend[X-Home-Mixer (Rust)]
    
    subgraph "X-Algorithm Backend"
        Backend --> Source[ThunderSource (Mock Data)]
        Source -->|10k Candidates| Filter[Visibility Filters]
        Filter --> Scorer[PhoenixScorer (Mock AI)]
        Scorer -->|Ranked Tweets| Mixer[Home Mixer]
    end
    
    Mixer -->|ScoredPostsResponse| Frontend
```

## Getting Started

### Prerequisites
- **Rust**: Latest stable.
- **Bun**: Latest version (`npm install -g bun`).

### Running the Backend (Simulation Mode)
The backend listens on port `50051`.

```bash
cargo run -p xai-home-mixer -- --grpc-port 50051 --metrics-port 9090 --reload-interval-minutes 60 --chunk-size 100
```

### Running Y
The frontend serves on `http://localhost:3000`.

```bash
bun i; bun dev
```

## Notice
This project is for educational and demonstration purposes.
