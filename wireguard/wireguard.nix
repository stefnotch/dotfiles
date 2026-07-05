{ config, pkgs, ... }:

{
  # Let Wireguard through the firewall
  networking.firewall.allowedUDPPorts = [ 51820 ];

  networking.wg-quick.interfaces = {
    wg0 = {
      # Private, non-conflicting IP address for Wireguard
      address = [
        "10.90.90.1/24"
      ];
      dns = [
        "1.1.1.1"
        "1.0.0.1"
      ];
      listenPort = 51820;
      privateKeyFile = "/etc/wireguard-wg0.key";
      generatePrivateKeyFile = true;
      peers = [
        { # Stefan gets an IP in my network
          publicKey = "ZD52qjNPWLKeery33vp3LcWHfZEN+9PSYb3T9gtZ1Qw=";
          allowedIPs = [
            "10.90.90.10/32"
          ];
        }
      ];
    };
  };
}
