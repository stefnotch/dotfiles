# Home Network

Router in the basement
- 192.168.1.1
- 2a00:cf8:e417::1

Router upstairs
- 192.168.1.2
- Mac: B0:BE:76:21:FF:A7

NixOS homeserver
- 192.168.1.10
- 2a00:cf8:e417::10
- Mac: 10:dd:b1:b6:b6:89

LG TV
- 192.168.1.12
- Mac: 00:a1:59:91:46:14

Doorbell
- 192.168.1.16 
- Mac: 00:5a:21:3b:97:3a

## IPv6

I wanted to use a RFC4193 network, but Firefox filters those out.
So we're squatting on the Liwest IPv6 space. They aren't using it anyways.

## Domains

`stefnotch.duckdns.org` points at the router's public IP.
`stefnotch-home.duckdns.org` points at the homeserver's private IP.
