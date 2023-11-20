# ork

Ork aims to be a full-fledged pay-as-you-go Minecraft hosting suite built on top of kubernetes.

## ork-bridge-service

`ork-bridge-service` is a single source of truth for managing a "player's location" across multiple proxies and servers
within a minecraft network. it decides ***where a player should go after various events** alongside exposing rich APIs
to help consumers navigate players through servers.

***where a player should go after various events**: e.g. when player joins network or when player's current server goes
down.