# Edit this configuration file to define what should be installed on
# your system.  Help is available in the configuration.nix(5) man page
# and in the NixOS manual (accessible by running ‘nixos-help’).

{ config, pkgs, ... }:

{
  imports = [
    # Include the results of the hardware scan.
    # Can be generated using nixos-generate-config
    ./hardware-configuration.nix
    ../wireguard/wireguard.nix
    ../dns/duckdns.nix
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
    networkmanager.enable = true;
    hostName = "nixos";

    # Use the device mac instead of it changing all the time
    networkmanager.ethernet.macAddress = "permanent";

    nameservers = [
        "1.1.1.1"
        "1.0.0.1"
        "8.8.8.8"
        "8.8.4.4"
    ];
    # Interface names can be found via `ip link show`
    interfaces.enp1s0f0 = {

    };
  };

  # Set your time zone.
  time.timeZone = "Europe/Vienna";

  # Select internationalisation properties.
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

  security.sudo.extraConfig = ''
    Defaults lecture = never # less noisy sudo
    Defaults pwfeedback # show dots when typing password
    Defaults timestamp_timeout=30 # only ask for password every 30 minutes
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
