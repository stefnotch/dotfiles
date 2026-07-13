{ config, pkgs, ... }:

{
  # TODO: Figure out where on disk the photos should land before enabling this config
  services.immich = {
    enable = true;
    host = "0.0.0.0";
    port = 2283;
    openFirewall = true;
    environment.IMMICH_LOG_LEVEL = "warn";
    machine-learning.enable = false;
  };
  services.redis.servers.immich.logLevel = "warning";
}
