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

  networking.wg-quick.interfaces.wg0 = {
    # Private, non-conflicting IP address for Wireguard
    address = [
      "10.90.90.1/24"
      fddf:2882:0550:9aa9::1/64
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
      ${pkgs.iptables}/bin/iptables -t nat -A POSTROUTING -s 10.90.90.0/24 -o enp1s0f0 -j MASQUERADE
      ${pkgs.iptables}/bin/ip6tables -A FORWARD -i wg0 -j ACCEPT
      ${pkgs.iptables}/bin/ip6tables -t nat -A POSTROUTING -s fddf:2882:0550:9aa9::1/64 -o enp1s0f0 -j MASQUERADE
    '';

    # Undo the above
    preDown = ''
      ${pkgs.iptables}/bin/iptables -D FORWARD -i wg0 -j ACCEPT
      ${pkgs.iptables}/bin/iptables -t nat -D POSTROUTING -s 10.90.90.0/24 -o enp1s0f0 -j MASQUERADE
      ${pkgs.iptables}/bin/ip6tables -D FORWARD -i wg0 -j ACCEPT
      ${pkgs.iptables}/bin/ip6tables -t nat -D POSTROUTING -s fddf:2882:0550:9aa9::1/64 -o enp1s0f0 -j MASQUERADE
    '';

    peers =
      let
        peerData = builtins.fromJSON (builtins.readFile ./peers.json);
      in
        map (peer: {
          publicKey = peer.publicKey;
          # Only allow the exact IP, hence /32
          allowedIPs = [
            "10.90.90.${toString peer.id}/32"
            "fddf:2882:0550:9aa9::${toString peer.id}/128"
          ];
          persistentKeepalive = 25;
        }) peerData;
  };
}
