# Interface Control Document (ICD) - `votive-svc-storage`

<center>

<img src="https://github.com/eldencat/terraform/raw/main/src/shared/votive-banner.png" style="height:250px" />

</center>

:exclamation: This document is intended for internal use.
## :telescope: Overview

This document details interfaces unique to this microservice.

## :books: Related Documents

Document | Description
--- | ---
:construction: Requirements & User Stories - `votive-svc-storage` :construction: | Requirements and user stories
[Concept of Operations (CONOPS) - `votive-svc-storage`](./conops.md) | Module overview
[Software Design Document (SDD) - `votive-svc-storage`](./sdd.md) | Module software design
:construction: [High-Level Interface Control Document (ICD)](https://raw.githubusercontent.com/eldencat/se-votive/develop/docs/icd.md) :construction: | Interfaces shared by all microservices.

## :wave: Public API
### :file_folder: Files

File locations for REST/GraphQL/gRPC files.

Filename | Description
--- | ---
`<root>/schema/example.graphql`

### :guardsman: Authorization

See the High-Level ICD.

### :mailbox_with_mail: Endpoints

| Endpoint | Type | Arguments | Description |
| ---- | --- | ---- | ---- |

## :revolving_hearts: Private API

### :file_folder: Files
File locations for gRPC/MQTT/ZeroMQ/etc. files

Filename | Description
--- | ---
`<root>/schema/example.proto`

### :guardsman: Authorization

See the High-Level ICD.

### :mailbox_with_mail: Server Methods

| Service | Description | Authorized Clients
| ---- | ---- | ---
| `exampleService` | Description of service | `svc-example`: Needs something