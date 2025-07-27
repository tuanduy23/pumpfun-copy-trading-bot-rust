# 🚀 PumpSwap Copy Trading Bot in Rust

An **optimized, high-performance copy trading bot** for the [Pump.fun](https://pump.fun) Solana-based token exchange, built with ❤️ in **Rust**. This bot listens to traders’ activities in real time and executes trades automatically based on your configurations.

---

## 📦 Features

- ⚡ Real-time copy trading via WebSocket
- 🧠 Automated decision-making logic
- 🔐 Secure key handling
- 🪙 Trade with Jito tips to maximize speed
- 🛠️ Fully customizable trader list and logic
- 📝 Easy configuration via `.env` file

---

## 🖥️ Preview

![Output Screenshot](output.png)

---

## 🔧 Setup Instructions

### 1. Clone the Repository

#### Run following command in terminal or cmd

```bash
git clone https://github.com/tuanduy23/pumpswap-copy-trading-bot-rust.git
cd pumpswap-copy-trading-bot-rust
```

### 2. Configure .env

#### Edit and load enviroment varibales

```bash
PRIVATE_KEY=
RPC_HTTP=https://solana-rpc.publicnode.com
RPC_WSS=wss://solana-rpc.publicnode.com
YELLOWSTONE_GRPC_HTTP=https://solana-yellowstone-grpc.publicnode.com:443
SLIPPAGE=10
JITO_BLOCK_ENGINE_URL=https://ny.mainnet.block-engine.jito.wtf
JITO_TIP_VALUE=0.0001
JITO_PRIORITY_FEE=0.0005
```

### 3. Copy wallets list

#### Add wallet addresses in targetlist.txt file

```bash
Add wallet 1
Add wallet 2
```

### 4. Run project and enjoy journey

Run following command in terminal or cmd

```bash
cargo run
```

🤝 Contribution
PRs and forks are welcome!
If you have improvements or trading logic tweaks, feel free to contribute.

📜 License
This project is under the MIT License — see the LICENSE file for details.

🙋‍♂️ Author
Made with passion by @tuanduy23
📧 duy231150@gmail.com
