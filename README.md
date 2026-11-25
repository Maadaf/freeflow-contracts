# freeflow-contracts

Fonte única dos contratos FreeFlow entre os serviços edge (simulador, gateways, vehicle-tracker, edge-uploader).

**Versão:** `0.1.0` · **Schema:** `1.0` · **Sem codegen nesta v1** — JSON Schema + fixtures + modelos espelhados Python/Rust + CI de sync.

## Conteúdo

| Caminho | Função |
|---------|--------|
| `schemas/v1/` | JSON Schema Draft 2020-12 (verdade externa) |
| `schemas/v1/subjects.md` | Convenção NATS / JetStream |
| `fixtures/v1/` | Exemplos válidos partilhados |
| `packaging/python/` | Pacote Pydantic `freeflow-contracts` |
| `packaging/rust/` | Crate Serde `freeflow-contracts` |
| `tools/check_sync.py` | Valida fixtures ↔ schemas ↔ modelos Python |

## Contratos v1

**Eventos** (`schema_version: "1.0"`):

- Envelope comum + `radar.vehicle_detected` | `lidar.vehicle_profiled` | `camera.plate_read` | `antenna.tag_read`
- `passage_id` pode ser `null` (gateways) ou UUID (simulador)
- Evidência: URIs `sim://`, `s3://`, `file://`; `size_bytes` obrigatório

**Passagens:**

- `passage.finalized` com `status` `complete` | `expired`
- Subjects: `freeflow.passages.completed` / `freeflow.passages.expired`

**Subjects de eventos (com gantry):**

```text
freeflow.events.<gantry_id>.<sensor_type>.<event_name>
```

## Uso rápido

### Python

```powershell
cd packaging/python
pip install -e ".[dev]"
pytest
```

```python
from freeflow_contracts import EventEnvelope, event_subject

event = EventEnvelope.model_validate_json(raw)
subject = event_subject(event.gantry_id, "camera")
```

Dependência Git (quando o consumer migrar):

```toml
# pyproject / requirements
# freeflow-contracts @ git+https://github.com/Maadaf/freeflow-contracts.git@v0.1.0#subdirectory=packaging/python
```

### Rust

```powershell
cd packaging/rust
cargo test
```

```toml
# Cargo.toml do consumer (workspace na raiz do repo de contratos)
freeflow-contracts = { git = "https://github.com/Maadaf/freeflow-contracts.git", tag = "v0.1.0" }
```

Para desenvolvimento local a partir de um sibling repo:

```toml
freeflow-contracts = { path = "../freeflow-contracts/packaging/rust" }
```

## CI de sync

```powershell
pip install -e "packaging/python.[dev]"
python tools/check_sync.py
cargo test --manifest-path packaging/rust/Cargo.toml
```

Alterar um contrato = PR neste repo → tag SemVer → bump do pin nos consumers. **Não** alterar semanticamente os schemas nos serviços.

## Fora de escopo (ainda)

Migração dos consumers (simulador, VT, gateways, uploader), WIM, UI de pórticos, codegen automático.

## Decisões aprovadas (2026-11-25)

1. Repo novo `freeflow-contracts` (não no simulador).
2. JSON Schema + fixtures + modelos espelhados + CI; sem codegen na v1.
3. Subjects com gantry; URIs `sim://` | `s3://` | `file://`.
