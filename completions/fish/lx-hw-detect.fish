# Fish completion for lx-hw-detect

# Clear existing completions
complete -c lx-hw-detect -e

# Global options
complete -c lx-hw-detect -s v -l verbose -d "Enable verbose logging"
complete -c lx-hw-detect -s q -l quiet -d "Suppress all output except errors"
complete -c lx-hw-detect -s p -l privacy -d "Privacy level" -xa "basic enhanced strict"
complete -c lx-hw-detect -s c -l config -d "Configuration file path" -r
complete -c lx-hw-detect -s h -l help -d "Print help information"
complete -c lx-hw-detect -s V -l version -d "Print version information"

# Commands
complete -c lx-hw-detect -n "__fish_use_subcommand" -xa "detect" -d "Detect hardware and generate compatibility report"
complete -c lx-hw-detect -n "__fish_use_subcommand" -xa "check" -d "Check which detection tools are available"
complete -c lx-hw-detect -n "__fish_use_subcommand" -xa "validate" -d "Validate hardware report files"
complete -c lx-hw-detect -n "__fish_use_subcommand" -xa "analyze" -d "Analyze kernel support and provide upgrade recommendations"
complete -c lx-hw-detect -n "__fish_use_subcommand" -xa "config" -d "Generate configuration templates"
complete -c lx-hw-detect -n "__fish_use_subcommand" -xa "recommend" -d "Generate system configuration recommendations"
complete -c lx-hw-detect -n "__fish_use_subcommand" -xa "submit" -d "Submit hardware report directly to GitHub"
complete -c lx-hw-detect -n "__fish_use_subcommand" -xa "help" -d "Print help information"

# detect subcommand
complete -c lx-hw-detect -n "__fish_seen_subcommand_from detect" -l output -d "Output file path" -r
complete -c lx-hw-detect -n "__fish_seen_subcommand_from detect" -s f -l format -d "Output format" -xa "yaml json markdown"
complete -c lx-hw-detect -n "__fish_seen_subcommand_from detect" -l no-anonymization -d "Disable privacy anonymization"
complete -c lx-hw-detect -n "__fish_seen_subcommand_from detect" -l include-kernel -d "Include kernel compatibility analysis"
complete -c lx-hw-detect -n "__fish_seen_subcommand_from detect" -l include-modules -d "Include kernel module information"

# recommend subcommand
complete -c lx-hw-detect -n "__fish_seen_subcommand_from recommend" -s d -l distribution -d "Target distribution" -xa "Ubuntu Debian Fedora Arch NixOS openSUSE Gentoo"
complete -c lx-hw-detect -n "__fish_seen_subcommand_from recommend" -s r -l report -d "Hardware report file" -r
complete -c lx-hw-detect -n "__fish_seen_subcommand_from recommend" -s f -l format -d "Output format" -xa "yaml json markdown"
complete -c lx-hw-detect -n "__fish_seen_subcommand_from recommend" -s o -l output -d "Output file path" -r
complete -c lx-hw-detect -n "__fish_seen_subcommand_from recommend" -l script -d "Generate installation script"
complete -c lx-hw-detect -n "__fish_seen_subcommand_from recommend" -l dkms -d "Include DKMS module recommendations"
complete -c lx-hw-detect -n "__fish_seen_subcommand_from recommend" -l performance -d "Include performance optimizations"

# validate subcommand
complete -c lx-hw-detect -n "__fish_seen_subcommand_from validate" -l schema -d "Schema file to validate against" -r
complete -c lx-hw-detect -n "__fish_seen_subcommand_from validate" -l strict -d "Enable strict validation mode"
complete -c lx-hw-detect -n "__fish_seen_subcommand_from validate" -l fix-errors -d "Attempt to fix validation errors"

# analyze subcommand
complete -c lx-hw-detect -n "__fish_seen_subcommand_from analyze" -l kernel-version -d "Target kernel version"
complete -c lx-hw-detect -n "__fish_seen_subcommand_from analyze" -l distribution -d "Target distribution" -xa "Ubuntu Debian Fedora Arch NixOS openSUSE Gentoo"
complete -c lx-hw-detect -n "__fish_seen_subcommand_from analyze" -l detailed -d "Enable detailed analysis"
complete -c lx-hw-detect -n "__fish_seen_subcommand_from analyze" -l compare -d "Compare with previous reports"

# config subcommand
complete -c lx-hw-detect -n "__fish_seen_subcommand_from config" -l generate -d "Generate default configuration"
complete -c lx-hw-detect -n "__fish_seen_subcommand_from config" -l template -d "Configuration template"
complete -c lx-hw-detect -n "__fish_seen_subcommand_from config" -l distribution -d "Target distribution" -xa "Ubuntu Debian Fedora Arch NixOS openSUSE Gentoo"
complete -c lx-hw-detect -n "__fish_seen_subcommand_from config" -l validate -d "Validate configuration file"

# submit subcommand  
complete -c lx-hw-detect -n "__fish_seen_subcommand_from submit" -l repository -d "GitHub repository"
complete -c lx-hw-detect -n "__fish_seen_subcommand_from submit" -l branch -d "Target branch"
complete -c lx-hw-detect -n "__fish_seen_subcommand_from submit" -l title -d "Pull request title"
complete -c lx-hw-detect -n "__fish_seen_subcommand_from submit" -l description -d "Pull request description"
complete -c lx-hw-detect -n "__fish_seen_subcommand_from submit" -l labels -d "Labels to add"

# check subcommand
complete -c lx-hw-detect -n "__fish_seen_subcommand_from check" -l tools -d "Check specific tools only"
complete -c lx-hw-detect -n "__fish_seen_subcommand_from check" -l system -d "Check system requirements"