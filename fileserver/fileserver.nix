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

        # For the Apple devices at home ;w;
        "vfs objects" = "catia fruit streams_xattr";
        "fruit:metadata" = "stream";
        "fruit:resource" = "stream";
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

  services.nfs.server = {
    enable = true;
    # Insecure just means that all ports can be used.
    exports = ''
    /srv/shared    *(insecure,rw,sync,no_subtree_check,all_squash,anonuid=1001,anongid=100)
    '';
    # The doorbell relies on NFS version 3
    lockdPort = 4001;
    mountdPort = 4002;
    statdPort = 4000;
  };
  services.nfs.settings = {
      nfsd = {
        udp = "y";
        # vers2 = false;
        # vers3 = false;
        # vers4 = true;
      };
  };

  # Ensure the shared folder exists
  systemd.tmpfiles.rules = [
    "d /srv/shared 0775 homeserver users - -"
  ];
}
