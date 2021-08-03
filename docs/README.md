# Keylink Documentation

## Redirect Signing Request

Example flow in the context of [Kepler](https://github.com/spruceid/kepler/):
```
                                               ┌─┐
                                               ║"│
                                               └┬┘
                                               ┌┼┐
     ┌──────┐  ┌───────────────┐                │                        ┌───────┐  ┌─────────────────┐
     │Kepler│  │Kepler Frontend│               ┌┴┐                       │Keylink│  │Identity Provider│
     └──┬───┘  └───────┬───────┘              Alice                      └───┬───┘  └────────┬────────┘
        │              │Wants to upload a file  │                            │               │
        │              │<───────────────────────│                            │               │
        │              │                        │                            │               │
        │              │Redirect to Keylink for │                            │               │
        │              │the ZCAP to be signed   │                            │               │
        │              │ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ >│                            │               │
        │              │                        │                            │               │
        │              │                        │                            │               │
        │              │                        │ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ >│               │
        │              │                        │                            │               │
        │              │                 ╔══════╧════════════════════════════╧═══════════════╧══════╗
        │              │                 ║OIDC Authentication                                      ░║
        │              │                 ╚══════╤════════════════════════════╤═══════════════╤══════╝
        │              │                        │                            │               │
        │              │                        │  Select which key to       │               │
        │              │                        │  sign the VC (ZCAP) with   │               │
        │              │                        │───────────────────────────>│               │
        │              │                        │                            │               │
        │              │                        │Redirect with the signed VC │               │
        │              │                        │<─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─│               │
        │              │                        │                            │               │
        │              │                        │                            │               │
        │              │<─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─│                            │               │
        │              │                        │                            │               │
        │    Authenticated file upload          │                            │               │
        │ <─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─│                            │               │
     ┌──┴───┐  ┌───────┴───────┐              Alice                      ┌───┴───┐  ┌────────┴────────┐
     │Kepler│  │Kepler Frontend│               ┌─┐                       │Keylink│  │Identity Provider│
     └──────┘  └───────────────┘               ║"│                       └───────┘  └─────────────────┘
                                               └┬┘
                                               ┌┼┐
                                                │
                                               ┌┴┐
```

[comment]: # (Diagram PlantUML source code is in ./redirect_diagram.utxt)
