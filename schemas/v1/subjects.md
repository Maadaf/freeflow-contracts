# Convenção NATS / JetStream — v1

## Streams

| Stream | Subjects | Produtores típicos | Consumidores típicos |
|--------|----------|--------------------|----------------------|
| `FREEFLOW_EVENTS` | `freeflow.events.>` | simulador, gateways | vehicle-tracker |
| `FREEFLOW_PASSAGES` | `freeflow.passages.*` | vehicle-tracker | edge-uploader |

## Subjects de eventos (com gantry no path)

Formato obrigatório v1:

```text
freeflow.events.<gantry_id>.<sensor_type>.<event_name>
```

Exemplos:

- `freeflow.events.a2-km25-s.radar.vehicle_detected`
- `freeflow.events.a2-km25-s.lidar.vehicle_profiled`
- `freeflow.events.a2-km25-s.camera.plate_read`
- `freeflow.events.a2-km25-s.antenna.tag_read`

Notas:

- `<gantry_id>` deve coincidir com o campo `gantry_id` do envelope.
- `<sensor_type>` ∈ `radar` | `lidar` | `camera` | `antenna`.
- O vehicle-tracker pode subscrever `freeflow.events.>`.
- Subjects “planos” sem gantry (`freeflow.events.camera.plate_read`) **não** fazem parte da convenção v1.

## Subjects de passagens

| Subject | Quando |
|---------|--------|
| `freeflow.passages.completed` | `status=complete` |
| `freeflow.passages.expired` | `status=expired` |

Prefixo configurável nos serviços (`VT_RESULT_SUBJECT_PREFIX`, default `freeflow.passages`).

## Headers JetStream

| Header | Valor |
|--------|-------|
| `Nats-Msg-Id` (eventos) | `event_id` (UUID) |
| `Nats-Msg-Id` (passagens) | `passage:{passage_id}:{gantry_id}` |

## URIs de evidência permitidas (v1)

- `sim://…` — evidência sintética do laboratório
- `s3://…` — object storage (MinIO / S3)
- `file://…` — ficheiro local no edge (gateways)
