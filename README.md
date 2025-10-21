# Provenance AI

A full-stack decentralized service for the generation, registration, and auditing of AI-generated Intellectual Property (IP), built 100% on-chain.

**Project for LegalHack 2025**
*Sponsor Tracks Addressed: Internet Computer (ICP), Story Protocol, Constellation Network.*

---

## The Problem

The proliferation of generative AI has created an intellectual property "black hole." Critical questions remain unanswered: Who owns an AI-generated asset? How can a creator prove its origin? How can licenses be programmatically enforced? How can we audit the provenance of a model or its output?

This lack of auditable provenance and clear title of ownership breaks existing legal frameworks and stifles the creator economy.

## The Solution

**Provenance AI** provides a 100% on-chain, trustless solution to this problem.

It is a "provably honest" AI generation tool that, at the moment of creation, executes an atomic workflow to establish an immutable IP lineage.

When a user generates a new asset (e.g., an image, text, or code), Provenance AI performs three simultaneous actions:

1.  **Registers (Minting):** The output is registered as a new immutable `IP Asset` on **Story Protocol**, programmatically attaching licensing terms.
2.  **Audits (Logging):** An immutable "Proof of Generation" is logged on the **Constellation Hypergraph**, creating a verifiable audit trail (model used, content hash, timestamp).
3.  **Orchestrates (Executing):** The entire workflow is managed by a tamper-proof backend canister running entirely on the **Internet Computer (ICP)**, which also hosts the web frontend.

The result is a new class of digital asset: an AI-generated IP with an unquestionable on-chain title of ownership and a verifiable creation history.

## Architecture Diagram

The project's core is the orchestration of three distinct blockchains from a single ICP backend.

+--------------------------+
|     User on the          |
| [Frontend Canister (ICP)]|
+--------------------------+
             |
             v
+--------------------------+
|  [Brain Canister (ICP)]  |  <-- (Backend Logic in Rust)
|                          |
|  1. Pay/Verify ckBTC     |
|  2. Generate AI content  |
+--------------------------+
             |
 +-----------+-----------+
 |                       |
 v                       v
+-------------------+   +--------------------+
| (EVM Call via     |   | (HTTP Call via     |
| Chain-Key ECDSA)  |   | ICP Outcall)       |
+-------------------+   +--------------------+
 |                       |
 v                       v
+-------------------+   +--------------------+
|  Story Protocol   |   | Constellation      |
| (IP Registration, |   | (Audit Log)        |
|  Royalty Payment) |   |                    |
+-------------------+   +--------------------+

### Architecture Components

* **Internet Computer (ICP):** Serves as the computation, logic, and orchestration layer.
    * **Frontend Canister:** Hosts the React dApp, serving the web directly to the user.
    * **Brain Canister (Backend):** A Rust canister that manages business logic. It uses **Chain-Key ECDSA** to generate a canister-owned EVM address and sign transactions for Story Protocol. It uses **HTTP Outcalls** to send audit data to the Constellation Metagraph. It manages incoming **ckBTC** payments.
* **Story Protocol (IP Layer):** Serves as the legal ownership layer.
    * **`IPAssetRegistry`:** Used to register the AI output as a `Root IP Asset` (IPA).
    * **`Licensing Module`:** Used to attach a programmatic license template to the new IPA.
    * **`Royalty Module`:** Used to receive royalty payments (sent from the ICP canister) and distribute them to the parent "AI Model" IP.
* **Constellation Network (Audit Layer):** Serves as the data validation and audit layer.
    * **Metagraph (L0):** A custom-deployed Metagraph exposing an HTTP endpoint. It accepts "Proof of Generation" data (hash, `ipId`, timestamp) and immutably validates and logs it onto the DAG.

## Key Features

* **Decentralized IP Generation:** 100% on-chain frontend and backend on ICP.
* **Atomic IP Registration:** Generated assets are instantly registered on Story Protocol at the moment of creation.
* **Programmatic Licensing:** Each new IP is created with a Story Protocol License (PIL) attached.
* **Immutable Audit Log:** Every asset is linked to an audit record on the Constellation Hypergraph.
* **Native Bitcoin Payments:** The service utilizes `ckBTC` for payments, demonstrating ICP's Bitcoin integration.
* **Automated AI Royalties:** A percentage of the `ckBTC` fee is automatically paid to the parent "AI Model" IP using Story's Royalty Module.
* **Dispute Resolution:** The UI allows users to flag infringing IP, invoking Story's Dispute Module.

## Technology Used

* **Blockchain & Infrastructure:** Internet Computer (ICP), Story Protocol, Constellation Network (Hypergraph)
* **Backend:** Rust, ICP Canisters, Chain-Key ECDSA, HTTP Outcalls
* **Frontend:** React, TypeScript, DFX
* **Tokens & Contracts:** ckBTC, Story Protocol Contract Standards

## Live Demo & Links

* **Deployed dApp (ICP Mainnet):** `https://<FRONTEND_CANISTER_ID>.icp0.io`
* **Backend Brain Canister ID:** `<BRAIN_CANISTER_ID>`
* **GitHub Repository:** `[LINK TO YOUR GITHUB REPO]`
* **Demo Video (Pitch):** `[LINK TO YOUR 2-MINUTE VIDEO]`
* **Code Walkthrough Video (ICP):** `[LINK TO YOUR CODE WALKTHROUGH]`

---

## Local Development

### Prerequisites

* Node.js (v16+)
* Rust and Cargo
* `dfx` (The ICP SDK)

Follow the instructions at [sdk.dfinity.org](https://sdk.dfinity.org/docs/index.html) to install `dfx`.

### Setup

1.  **Clone the repository:**
    ```bash
    git clone [LINK TO YOUR GITHUB REPO]
    cd provenance-ai
    ```

2.  **Install dependencies:**
    ```bash
    npm install
    ```

3.  **Configure environment variables:**
    Create a `.env` file in the project root.
    ```properties
    # Story Protocol Testnet RPC URL
    STORY_RPC_URL=https://...
    # Your deployed Constellation Metagraph Endpoint
    CONSTELLATION_METAGRAPH_URL=https://...
    # ckBTC Ledger Canister ID (Testnet)
    CKBTC_LEDGER_ID=...
    ```

### Running Locally

1.  **Start the local ICP replica:**
    ```bash
    dfx start --background --clean
    ```

2.  **Deploy the canisters locally:**
    ```bash
    dfx deploy
    ```

3.  **Get the frontend URL:**
    ```bash
    echo "Frontend canister running at: [http://127.0.0.1:4943?canisterId=$(dfx](http://127.0.0.1:4943?canisterId=$(dfx) canister id frontend_canister)"
    ```

4.  **Start the frontend dev server (optional, for hot-reloading):**
    ```bash
    npm start
    ```

## License

This project is open source and licensed under the **MIT License**.
