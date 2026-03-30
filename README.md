# PromptMesh

P2P AI prompt relay built on Holochain. Share prompts across a decentralized mesh of peers, collect crowd-sourced improvements, and relay the aggregated result to a local or remote LLM — no central server required.

## How it works

1. **Post** a prompt to the DHT
2. **Peers** add suggestions and improvements (stored as linked entries)
3. **Relay** — your node aggregates peer suggestions and sends them to an LLM, then posts the refined result back to the mesh

## Structure

```
dnas/promptmesh/
  zomes/
    prompts_integrity/   # entry + link types, validation
    prompts/             # create_prompt, get_all_prompts, get_agent_prompts
    suggestions_integrity/
    suggestions/         # create_suggestion, get_suggestions_for_prompt
workdir/happ.yaml
ui/index.html
```

---

## Quick start — UI only (any OS)

You can use the UI and AI relay without Holochain running. It falls back to demo mode automatically.

1. Install [Ollama](https://ollama.com) and pull a model:
   ```bash
   ollama pull llama3
   ```
2. Open `ui/index.html` in your browser
3. The LLM bar defaults to **Ollama / llama3** — start relaying prompts immediately

---

## Full P2P setup — Linux / macOS

**1. Install Nix**
```bash
sh <(curl -L https://nixos.org/nix/install) --daemon
```

**2. Enter the Holochain dev shell** (pins correct versions automatically)
```bash
nix develop github:holochain/holochain
```

**3. Install Rust wasm target**
```bash
rustup target add wasm32-unknown-unknown
```

**4. Build**
```bash
cargo build --release --target wasm32-unknown-unknown
hc dna pack dnas/promptmesh/workdir
hc app pack workdir
```

**5. Run**
```bash
hc sandbox generate workdir/promptmesh.happ --run=8888
```

Then open `ui/index.html` — it connects to the conductor at `ws://localhost:8888`.

---

## Full P2P setup — Windows (via WSL2)

Holochain's toolchain requires Linux. WSL2 gives you a full Linux environment inside Windows.

**1. Install WSL2**

Open PowerShell as Administrator and run:
```powershell
wsl --install
```
Restart when prompted. This installs Ubuntu by default.

**2. Open your WSL2 terminal**

Search for "Ubuntu" in the Start menu, or run `wsl` in PowerShell.

**3. Install Nix inside WSL2**
```bash
sh <(curl -L https://nixos.org/nix/install) --daemon
```
Close and reopen the terminal after install.

**4. Clone the repo inside WSL2**
```bash
git clone https://github.com/ta10101/promptmesh
cd promptmesh
```

> Do not clone into `/mnt/c/...` — keep it inside the Linux filesystem for performance.

**5. Enter the Holochain dev shell**
```bash
nix develop github:holochain/holochain
```
This downloads the correct Holochain + hc CLI versions. First run takes a few minutes.

**6. Install Rust wasm target**
```bash
rustup target add wasm32-unknown-unknown
```

**7. Build**
```bash
cargo build --release --target wasm32-unknown-unknown
hc dna pack dnas/promptmesh/workdir
hc app pack workdir
```

**8. Run the conductor**
```bash
hc sandbox generate workdir/promptmesh.happ --run=8888
```

**9. Open the UI on Windows**

The conductor inside WSL2 is reachable from Windows browsers at `ws://localhost:8888` (WSL2 bridges the port automatically). Open `ui/index.html` in your Windows browser — it will connect.

**Ollama on Windows + Holochain in WSL2**

Install [Ollama for Windows](https://ollama.com) normally. The UI runs in your Windows browser and calls Ollama at `localhost:11434` and Holochain at `localhost:8888` — both work transparently.

---

## LLM options

| Provider | Setup | API Key needed |
|---|---|---|
| Ollama (default) | Install Ollama + `ollama pull llama3` | No |
| Anthropic | — | Yes (console.anthropic.com) |
| OpenAI | — | Yes (platform.openai.com) |

Switch providers in the LLM bar at the top of the UI.
