{ config, pkgs, ... }:

{
  services.samba = {
    enable = true;
    openFirewall = true;
    # See https://www.samba.org/samba/docs/current/man-html/smb.conf.5.html
    settings = {
      global = {
        "workgroup" = "WORKGROUP";
        "server string" = "homeserver";
        "netbios name" = "homeserver";
        "security" = "user";
        "use sendfile" = "yes";
        "deadtime" = "120"; # timeout after 2 hours
      };
      "shared" = {
        "valid users" = "homeserver";
        "path" = "/srv/shared";
        "browseable" = "yes";
        "read only" = "no";
        "create mask" = "0664";
        "directory mask" = "0775";
      };
    };
  };

  # Ensure the shared folder exists
  systemd.tmpfiles.rules = [
    "d /srv/shared 0775 homeserver users - -"
  ];
}
