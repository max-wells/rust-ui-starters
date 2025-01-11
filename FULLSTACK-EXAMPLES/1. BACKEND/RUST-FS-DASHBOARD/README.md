

# 1. Run the project

## 1.1 Backend

Open a first terminal and run:

```bash
cd backend
cargo run
```


💁 Troubleshoot
```bash
cargo clean
cargo build
cargo sqlx prepare  # Working
```


## 1.2 Frontend

Open a second terminal and run:

```bash
cargo leptos watch
```























## What is Pingap

A reverse proxy like nginx, built on [pingora](https://github.com/cloudflare/pingora), simple and efficient.

Sentry and opentelemetry are optional, they are supported in the full-featured version.



```mermaid
flowchart LR
  internet("Internet") -- request --> pingap["Pingap"]
  pingap -- proxy:pingap.io/api/* --> apiUpstream["10.1.1.1,10.1.1.2"]
  pingap -- proxy:cdn.pingap.io --> cdnUpstream["10.1.2.1,10.1.2.2"]
  pingap -- proxy:pingap.io --> upstream["10.1.3.1,10.1.3.2"]
```





## Proxy step

```mermaid
graph TD;
  server["HTTP Server"];
  locationA["Location A"];
  locationB["Location B"];
  locationPluginListA["Proxy Plugin List A"];
  locationPluginListB["Proxy Plugin List B"];
  upstreamA1["Upstream A1"];
  upstreamA2["Upstream A2"];
  upstreamB1["Upstream B1"];
  upstreamB2["Upstream B2"];
  locationResponsePluginListA["Response Plugin List A"];
  locationResponsePluginListB["Response Plugin List B"];

  start("New Request") --> server

  server -- "host:HostA, Path:/api/*" --> locationA

  server -- "Path:/rest/*"--> locationB

  locationA -- "Exec Proxy Plugins" --> locationPluginListA

  locationB -- "Exec Proxy Plugins" --> locationPluginListB

  locationPluginListA -- "proxy pass: 10.0.0.1:8001" --> upstreamA1

  locationPluginListA -- "proxy pass: 10.0.0.2:8001" --> upstreamA2

  locationPluginListA -- "done" --> response

  locationPluginListB -- "proxy pass: 10.0.0.1:8002" --> upstreamB1

  locationPluginListB -- "proxy pass: 10.0.0.2:8002" --> upstreamB2

  locationPluginListB -- "done" --> response

  upstreamA1 -- "Exec Response Plugins" --> locationResponsePluginListA
  upstreamA2 -- "Exec Response Plugins" --> locationResponsePluginListA

  upstreamB1 -- "Exec Response Plugins" --> locationResponsePluginListB
  upstreamB2 -- "Exec Response Plugins" --> locationResponsePluginListB

  locationResponsePluginListA --> response
  locationResponsePluginListB --> response

  response["HTTP Response"] --> stop("Logging");
```

