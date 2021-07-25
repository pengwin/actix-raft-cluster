![rust workflow](https://github.com/pengwin/actix-raft-cluster/actions/workflows/rust.yml/badge.svg)

# Actix raft cluster

Cluster of nodes with actors

Actors powered by [actix framework](https://github.com/actix/actix)

Network layer powered by [actix-web](https://github.com/actix/actix-web)

The end goal is to implement cluster of virtual actors similar to [Orleans](https://dotnet.github.io/orleans/)

## Features plan:

- [x] Actors can be reached remotely
- [x] Actors activated by first call
- [ ] Actors passivation after inactivity period
- [x] Cluster nodes can attach to leader
- [ ] Cluster nodes ping each other and detach
- [ ] Cluster nodes detach unreachable nodes
- [ ] Use Raft protocol
- [x] Nodes use simple http server for network communication
- [ ] Nodes use tcp server for network communication
- [ ] Builder like API to register actor types

# Requirements

Tested on:

- Docker v20.10.7
- Docker-compose v1.29.2

# Build

```
docker-compose build
```

# Run

```
docker-compose run
```