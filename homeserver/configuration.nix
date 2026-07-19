# Edit this configuration file to define what should be installed on
# your system.  Help is available in the configuration.nix(5) man page
# and in the NixOS manual (accessible by running ‘nixos-help’).

{ config, pkgs, inputs, ... }:

{
  imports = [
    # Include the results of the hardware scan.
    # Can be generated using nixos-generate-config
    ./hardware-configuration.nix
    ../wireguard/wireguard.nix
    ../dns/duckdns.nix
    ../fileserver/fileserver.nix
  ];

  boot = {
    loader = {
        systemd-boot.enable = true;
        efi.canTouchEfiVariables = true;
        systemd-boot.configurationLimit = 10;
    };
    blacklistedKernelModules = [
        # Disable Wifi, we're always on LAN
        "wl"
    ];
    # Turn off screen after 60s in console
    kernelParams = [ "consoleblank=60" ];
  };

  # Don't need bluetooth
  hardware.bluetooth.enable = false;

  # Please stay alive at all costs
  systemd.sleep.settings.Sleep = {
    AllowHibernation = "no";
    AllowHybridSleep = "no";
    AllowSuspend = "no";
    AllowSuspendThenHibernate = "no";
  };

  networking = {
    hostName = "nixos";

    nameservers = [
        "1.1.1.1"
        "1.0.0.1"
        "8.8.8.8"
        "8.8.4.4"
    ];
    # Interface names can be found via `ip link show`
    interfaces.enp1s0f0 = {
      ipv6.addresses = [{
        # Local address, so no default gateway
        address = "2a00:cf8:e417::10";
        prefixLength = 64;
      }];
    };
  };

  # Ports for Caddy reverse proxy
  networking.firewall.allowedTCPPorts = [
    # Normal HTTP and HTTPS ports for caddy
    80 443
    # For when I directly access the homepage by IP
    8080
    # For NFS
    2049
    # For NFS version 3
    111 4000 4001 4002 20048
  ];
  networking.firewall.allowedUDPPorts = [
    # For NFS version 3
    111 2049 4000 4001 4002 20048
  ];
  services.caddy = {
    enable = true;
    # Will be upgraded to a reverse proxy once it is working
    virtualHosts."stefnotch-home.duckdns.org".extraConfig = ''
      tls {
        dns duckdns {file./etc/duckdns.key}
      }

      reverse_proxy http://localhost:8080 {
        header_down X-Forwarded-For {http.request.remote}
      }
    '';
    package = pkgs.caddy.withPlugins {
        plugins = [ "github.com/caddy-dns/duckdns@v0.5.0" ];
        hash = "sha256-BI72FyEpCKTyQ9lRlVcRsPLSyXlfwdOae57KhVTH/M8=";
    };
  };

  time.timeZone = "Europe/Vienna";
  i18n.defaultLocale = "en_US.UTF-8";
  i18n.extraLocaleSettings = {
    LC_ADDRESS = "en_AU.UTF-8";
    LC_IDENTIFICATION = "en_AU.UTF-8";
    LC_MEASUREMENT = "en_AU.UTF-8";
    LC_MONETARY = "en_AU.UTF-8";
    LC_NAME = "en_AU.UTF-8";
    LC_NUMERIC = "en_AU.UTF-8";
    LC_PAPER = "en_AU.UTF-8";
    LC_TELEPHONE = "en_AU.UTF-8";
    LC_TIME = "en_AU.UTF-8";
  };

  # Define a user account. Don't forget to set a password with ‘passwd’.
  users.users.stefnotch = {
    isNormalUser = true;
    description = "Stefnotch";
    extraGroups = [
      "networkmanager"
      "wheel"
    ];
    packages = with pkgs; [

    ];
    openssh.authorizedKeys.keyFiles = [
      ../ssh/authorized_keys
    ];
  };

  users.users.homeserver = {
      description = "User for accessing the homeserver (e.g. Samba)";
      uid = 1001;
      extraGroups = [ "users" ];
      isNormalUser = true;
  };

  users.groups.users.gid = 100;

  nix.settings.trusted-users = [ "@wheel" ];

  security.sudo.extraConfig = ''
    Defaults lecture = never # less noisy sudo
    Defaults pwfeedback # show dots when typing password
    Defaults timestamp_timeout=60 # only ask for password every hour
    '';

  # Allow unfree packages
  nixpkgs.config.allowUnfree = true;
  # Freaking insecure drivers
  nixpkgs.config.permittedInsecurePackages = [
    "broadcom-sta-6.30.223.271-59-6.18.37"
  ];

  nix.optimise.automatic = true;
  nix.gc = {
    automatic = true;
    dates = "weekly";
    options = "--delete-older-than 30d";
  };

  # Nix Flakes Support
  nix.settings.experimental-features = [ "nix-command" "flakes" ];

  # List packages installed in system profile. To search, run:
  # $ nix search wget
  environment.systemPackages = with pkgs; [
    git # Flakes need this
    jujutsu
    # Zed for remote development
    zed-editor
  ];

  # List services that you want to enable:

  # Automatically log me in
  services.getty.autologinUser = "stefnotch";
  services.logind.settings.Login = {
    HandleLidSwitch = "ignore";
  };

  # Enable the OpenSSH daemon.
  services.openssh = {
    enable = true;
    openFirewall = true;
    settings = {
      PasswordAuthentication = false;
      KbdInteractiveAuthentication = false;
      PermitRootLogin = "no";
      AllowUsers = [ "stefnotch" ];
    };
  };

  # To inspect: systemctl status homepage
  systemd.services.homepage = {
    description = "Best Homepage";
    wantedBy = ["multi-user.target"];
    after = ["network.target"];
    serviceConfig = {
      ProtectSystem = "strict";
      ExecStart = "${inputs.homepage.packages.${pkgs.stdenv.hostPlatform.system}.default}/bin/homepage";
      Environment = "PATH=${pkgs.wireguard-tools}/bin";
      Restart = "always";
      Type = "simple";
      User = "stefnotch";
      Group = "users";
      # Grant network modification even for underprivileged users
      AmbientCapabilities = [
        "CAP_NET_ADMIN"
      ];
      ReadWritePaths = [
        "/home/stefnotch/dotfiles/wireguard"
        "/home/stefnotch/dotfiles/wireguard/config.json"
      ];
    };
    environment = {
      IP = "::";
      PORT = "8080";
      WIREGUARD_PATH = "/home/stefnotch/dotfiles/wireguard";
      RUST_BACKTRACE = "1";
    };
  };


  # Auto updates let's gooo
  system.autoUpgrade.enable = true;

  # This value determines the NixOS release from which the default
  # settings for stateful data, like file locations and database versions
  # on your system were taken. It‘s perfectly fine and recommended to leave
  # this value at the release version of the first install of this system.
  # Before changing this value read the documentation for this option
  # (e.g. man configuration.nix or on https://nixos.org/nixos/options.html).
  system.stateVersion = "24.11"; # Did you read the comment?
}
