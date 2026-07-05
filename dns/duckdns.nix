{ config, pkgs, ... }:

{
  services.duckdns = {
    enable = true;
    domains = [ "stefnotch" ];
    tokenFile = "/etc/duckdns.key";
  };
}
