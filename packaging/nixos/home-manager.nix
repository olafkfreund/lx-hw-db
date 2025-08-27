{ config, lib, pkgs, ... }:

with lib;

let
  cfg = config.programs.lx-hw-db;
  
  configFile = pkgs.writeText "lx-hw-db-config.toml" ''
    [detection]
    privacy_level = "${cfg.settings.privacyLevel}"
    enable_kernel_analysis = ${boolToString cfg.settings.enableKernelAnalysis}
    
    [output]
    default_format = "${cfg.settings.defaultFormat}"
    output_directory = "${cfg.settings.outputDirectory}"
    
    [privacy]
    salt_rotation_hours = ${toString cfg.settings.privacy.saltRotationHours}
    anonymization_level = "${cfg.settings.privacy.anonymizationLevel}"
    
    ${cfg.settings.extraConfig}
  '';
  
in {
  options.programs.lx-hw-db = {
    enable = mkEnableOption "lx-hw-db hardware detection tool";

    package = mkPackageOption pkgs "lx-hw-db" {};

    settings = {
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

      outputDirectory = mkOption {
        type = types.str;
        default = "\${XDG_DATA_HOME:-$HOME/.local/share}/lx-hw-db";
        description = "Directory to store hardware reports";
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
    };

    enableBashIntegration = mkOption {
      type = types.bool;
      default = true;
      description = "Whether to enable Bash integration";
    };

    enableZshIntegration = mkOption {
      type = types.bool;
      default = true;
      description = "Whether to enable Zsh integration";
    };

    enableFishIntegration = mkOption {
      type = types.bool;
      default = true;
      description = "Whether to enable Fish integration";
    };

    enablePeriodicDetection = mkOption {
      type = types.bool;
      default = false;
      description = "Enable periodic hardware detection via systemd user timer";
    };

    periodicDetectionCalendar = mkOption {
      type = types.str;
      default = "weekly";
      description = "Calendar specification for periodic hardware detection";
    };
  };

  config = mkIf cfg.enable {
    # Install the package
    home.packages = [ cfg.package ];

    # Create configuration directory and file
    xdg.configFile."lx-hw-db/config.toml".source = configFile;

    # Create data directory
    xdg.dataFile."lx-hw-db/.keep".text = "";

    # Shell completions
    programs.bash.interactiveShellInit = mkIf cfg.enableBashIntegration ''
      # lx-hw-db completions
      if command -v lx-hw-detect &> /dev/null; then
        source <(lx-hw-detect completion bash 2>/dev/null || true)
      fi
    '';

    programs.zsh.initExtra = mkIf cfg.enableZshIntegration ''
      # lx-hw-db completions
      if command -v lx-hw-detect &> /dev/null; then
        source <(lx-hw-detect completion zsh 2>/dev/null || true)
      fi
    '';

    programs.fish.interactiveShellInit = mkIf cfg.enableFishIntegration ''
      # lx-hw-db completions
      if command -v lx-hw-detect &> /dev/null
        lx-hw-detect completion fish 2>/dev/null | source
      end
    '';

    # Periodic hardware detection
    systemd.user.timers.lx-hw-db-detect = mkIf cfg.enablePeriodicDetection {
      Unit = {
        Description = "Periodic hardware detection for personal lx-hw-db";
      };
      Timer = {
        OnCalendar = cfg.periodicDetectionCalendar;
        Persistent = true;
        RandomizedDelaySec = "30m";
      };
      Install = {
        WantedBy = [ "timers.target" ];
      };
    };

    systemd.user.services.lx-hw-db-detect = mkIf cfg.enablePeriodicDetection {
      Unit = {
        Description = "Hardware detection for personal lx-hw-db database";
      };
      Service = {
        Type = "oneshot";
        ExecStart = "${cfg.package}/bin/lx-hw-detect detect --config %h/.config/lx-hw-db/config.toml --output %h/.local/share/lx-hw-db/hardware-report-$(date +%Y%m%d).yaml";
      };
    };

    # Desktop integration
    xdg.desktopEntries.lx-hw-db = {
      name = "Hardware Database Tool";
      comment = "Detect hardware and generate compatibility reports";
      exec = "${cfg.package}/bin/lx-hw-detect detect --gui";
      icon = "computer";
      terminal = true;
      categories = [ "System" "HardwareSettings" ];
      keywords = [ "hardware" "compatibility" "drivers" "detection" ];
    };

    # MIME type associations for hardware report files
    xdg.mimeApps = {
      associations.added = {
        "application/x-lx-hw-db-report" = [ "lx-hw-db.desktop" ];
      };
      defaultApplications = {
        "application/x-lx-hw-db-report" = [ "lx-hw-db.desktop" ];
      };
    };

    # Register custom MIME type
    xdg.dataFile."mime/packages/lx-hw-db.xml".text = ''
      <?xml version="1.0" encoding="UTF-8"?>
      <mime-info xmlns="http://www.freedesktop.org/standards/shared-mime-info">
        <mime-type type="application/x-lx-hw-db-report">
          <comment>Linux Hardware Database Report</comment>
          <glob pattern="*.lx-hw-report"/>
          <glob pattern="*.lxhw"/>
          <magic priority="50">
            <match type="string" offset="0" value="# Linux Hardware Database Report"/>
          </magic>
        </mime-type>
      </mime-info>
    '';
  };
}