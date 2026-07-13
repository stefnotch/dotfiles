# Homepage

Mostly vibecoded

```bash
dx serve
```

For the Wireguard auth, I think I need to bind a HTTP server to the wg0 interface explicitly.
(10.90.90.1/24)
(And then try out https://docs.rs/axum-client-ip/latest/axum_client_ip/ )

And for the peers, here's the magic sauce https://serverfault.com/a/1110966
