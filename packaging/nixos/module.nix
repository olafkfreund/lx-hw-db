{ config, lib, pkgs, ... }:

with lib;

let
  cfg = config.services.lx-hw-db;
  
  configFile = pkgs.writeText "lx-hw-db-config.toml" ''
    [detection]
    privacy_level = "${cfg.privacyLevel}"
    enable_kernel_analysis = ${boolToString cfg.enableKernelAnalysis}
    
    [web]
    enabled = ${boolToString cfg.web.enable}
    host = "${cfg.web.host}"
    port = ${toString cfg.web.port}
    
    [output]
    default_format = "${cfg.defaultFormat}"
    
    [privacy]
    salt_rotation_hours = ${toString cfg.privacy.saltRotationHours}
    anonymization_level = "${cfg.privacy.anonymizationLevel}"
    
    ${cfg.extraConfig}
  '';
  
in {
  options.services.lx-hw-db = {
    enable = mkEnableOption "lx-hw-db hardware detection service";

    package = mkPackageOption pkgs "lx-hw-db" {};

    privacyLevel = mkOption {
      type = types.enum [ "basic" "enhanced" "strict" ];
      default = "basic";
      description = "Privacy level for hardware data collection";
    };

    enableKernelAnalysis = mkOption {
      type = types.bool;
      default = true;
      description = "Enable kernel compatibility analysis";
    };

    defaultFormat = mkOption {
      type = types.enum [ "yaml" "json" "markdown" ];
      default = "yaml";
      description = "Default output format for hardware reports";
    };

    web = {
      enable = mkOption {
        type = types.bool;
        default = false;
        description = "Enable web interface server";
      };

      host = mkOption {
        type = types.str;
        default = "127.0.0.1";
        description = "Web interface bind address";
      };

      port = mkOption {
        type = types.port;
        default = 8080;
        description = "Web interface port";
      };
    };

    privacy = {
      saltRotationHours = mkOption {
        type = types.ints.positive;
        default = 24;
        description = "Hours between cryptographic salt rotation";
      };

      anonymizationLevel = mkOption {
        type = types.enum [ "basic" "enhanced" "strict" ];
        default = "basic";
        description = "Level of data anonymization";
      };
    };

    extraConfig = mkOption {
      type = types.lines;
      default = "";
      description = "Additional configuration in TOML format";
    };

    user = mkOption {
      type = types.str;
      default = "lx-hw-db";
      description = "User to run lx-hw-db service as";
    };

    group = mkOption {
      type = types.str;
      default = "lx-hw-db";
      description = "Group to run lx-hw-db service as";
    };
  };

  config = mkIf cfg.enable {
    # Create user and group
    users.users.${cfg.user} = {
      description = "lx-hw-db service user";
      group = cfg.group;
      home = "/var/lib/lx-hw-db";
      createHome = true;
      isSystemUser = true;
    };

    users.groups.${cfg.group} = {};

    # Install package
    environment.systemPackages = [ cfg.package ];

    # Create systemd service
    systemd.services.lx-hw-db = mkIf cfg.web.enable {
      description = "Linux Hardware Database Web Interface";
      wantedBy = [ "multi-user.target" ];
      after = [ "network.target" ];

      serviceConfig = {
        Type = "simple";
        User = cfg.user;
        Group = cfg.group;
        WorkingDirectory = "/var/lib/lx-hw-db";
        ExecStart = "${cfg.package}/bin/lx-hw-detect --config ${configFile} server --host ${cfg.web.host} --port ${toString cfg.web.port}";
        Restart = "on-failure";
        RestartSec = 5;

        # Security hardening
        NoNewPrivileges = true;
        ProtectSystem = "strict";
        ProtectHome = true;
        PrivateTmp = true;
        PrivateDevices = true;
        ProtectKernelTunables = true;
        ProtectControlGroups = true;
        RestrictSUIDSGID = true;
        RestrictRealtime = true;
        RestrictNamespaces = true;
        LockPersonality = true;
        MemoryDenyWriteExecute = true;
        SystemCallFilter = [ "@system-service" ];
        SystemCallErrorNumber = "EPERM";

        # Resource limits
        LimitNOFILE = 1024;
        MemoryMax = "512M";
        TasksMax = 100;
      };
    };

    # Create configuration directory
    environment.etc."lx-hw-db/config.toml".source = configFile;

    # Ensure required system tools are available
    environment.systemPackages = with pkgs; [
      lshw
      dmidecode
      pciutils
      usbutils
      util-linux
    ];

    # Set up periodic hardware detection (optional)
    systemd.timers.lx-hw-db-detect = mkIf cfg.web.enable {
      description = "Periodic hardware detection for lx-hw-db";
      wantedBy = [ "timers.target" ];
      timerConfig = {
        OnCalendar = "daily";
        Persistent = true;
        RandomizedDelaySec = "1h";
      };
    };

    systemd.services.lx-hw-db-detect = mkIf cfg.web.enable {
      description = "Hardware detection for lx-hw-db database";
      serviceConfig = {
        Type = "oneshot";
        User = cfg.user;
        Group = cfg.group;
        ExecStart = "${cfg.package}/bin/lx-hw-detect detect --config ${configFile} --output /var/lib/lx-hw-db/hardware-report.yaml";
      };
    };
  };
}