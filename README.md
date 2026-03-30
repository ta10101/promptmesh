# PromptMesh

P2P AI prompt relay built on Holochain. Share prompts across a decentralized mesh of peers, collect crowd-sourced improvements, and relay the aggregated result to an AI model — no central server required.

## How it works

1. **Post** a prompt to the DHT
2. **Peers** add suggestions and improvements (stored as linked entries)
3. **Relay** — your node aggregates peer suggestions and sends them to an AI API, then posts the refined result back to the mesh

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

## Requirements

- [Holochain](https://developer.holochain.org/) 0.3+
- Rust with `wasm32-unknown-unknown` target

## Build

```bash
rustup target add wasm32-unknown-unknown
hc app pack workdir
```

## Run

```bash
hc sandbox generate workdir/promptmesh.happ --run=8888
```

Then open `ui/index.html` in your browser.
