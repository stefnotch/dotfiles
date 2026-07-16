{ config, pkgs, ... }:

{
  # Let Wireguard through the firewall
  # Run it on a port that is less likely to be blocked
  networking.firewall.allowedUDPPorts = [ 3478 ];

  networking.nat = {
    enable = true;
    enableIPv6 = true;
    externalInterface = "enp1s0f0";
    internalInterfaces = [ "wg0" ];
  };

  networking.wg-quick.interfaces.wg0 = 
  let
    connection = builtins.fromJSON (builtins.readFile ./config.json);
    prefix = connection.prefix;
    peers = connection.peers;
  in
  {
    # Private, non-conflicting IP address for Wireguard
    address = [
      "${prefix.IPv4}.1/24"
      "${prefix.IPv6}::1/64"
    ];
    dns = [
      "1.1.1.1"
      "1.0.0.1"
    ];
    listenPort = 3478;
    privateKeyFile = "/etc/wireguard-wg0.key";
    generatePrivateKeyFile = true;
    # This allows the wireguard server to route your traffic
    postUp = ''
      ${pkgs.iptables}/bin/iptables -A FORWARD -i wg0 -j ACCEPT
      ${pkgs.iptables}/bin/iptables -t nat -A POSTROUTING -s ${prefix.IPv4}.0/24 -o enp1s0f0 -j MASQUERADE
    '';

    # Undo the above
    preDown = ''
      ${pkgs.iptables}/bin/iptables -D FORWARD -i wg0 -j ACCEPT
      ${pkgs.iptables}/bin/iptables -t nat -D POSTROUTING -s ${prefix.IPv4}.0/24 -o enp1s0f0 -j MASQUERADE
    '';

    peers = map (peer: {
          publicKey = peer.publicKey;
          # Only allow the exact IP, hence /32
          allowedIPs = [
            "${prefix.IPv4}.${toString peer.id}/32"
            "${prefix.IPv6}::${toString peer.id}/128"
          ];
          persistentKeepalive = 25;
        }) peers;
  };
}
