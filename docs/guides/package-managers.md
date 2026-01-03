# Package Managers Guide

## Overview

Munux provides comprehensive support for package managers across all major Linux distributions. This guide covers command syntax, best practices, and distribution-specific features.

## Supported Package Managers

### Arch Linux / Manjaro

#### Pacman (Official)

**Installation**
```bash
pacman -S <package>           # Install package
pacman -S <pkg1> <pkg2>       # Install multiple packages
sudo pacman -S firefox        # Install Firefox
```

**Removal**
```bash
pacman -R <package>           # Remove package
pacman -Rs <package>          # Remove package + unused dependencies
pacman -Rns <package>         # Remove package + dependencies + config files
```

**System Update**
```bash
pacman -Syu                   # Update system (Sync + refresh + upgrade)
pacman -Syyu                  # Force refresh all package databases
```

**Search & Info**
```bash
pacman -Ss <keyword>          # Search for packages
pacman -Qi <package>          # Show installed package info
pacman -Ql <package>          # List files installed by package
pacman -Qdt                   # List orphaned packages
```

**Cache Management**
```bash
pacman -Sc                    # Clean package cache
pacman -Scc                   # Clean all cache (aggressive)
```

#### Yay (AUR Helper)

**Installation**
```bash
yay -S <package>              # Install from official repos or AUR
yay -S spotify                # Install Spotify from AUR
yay -S visual-studio-code-bin # Install VS Code from AUR
```

**System Update**
```bash
yay -Syu                      # Update system + AUR packages
yay                           # Shortcut for update
```

**AUR-Specific Operations**
```bash
yay -Ps                       # Print system statistics
yay -Yc                       # Clean unneeded dependencies
yay -G <package>              # Download PKGBUILD from AUR
```

#### Paru (Modern AUR Helper)

```bash
paru -S <package>             # Install package
paru                          # Update all packages
paru -Sua                     # Update only AUR packages
paru -c                       # Clean cache
```

#### Pamac (GUI + CLI)

```bash
pamac install <package>       # Install package
pamac remove <package>        # Remove package
pamac update                  # Update system
pamac search <keyword>        # Search packages
```

### Ubuntu / Debian / Mint

#### APT (Advanced Package Tool)

**Installation**
```bash
apt install <package>         # Install package
sudo apt install firefox      # Install Firefox
apt install -y <package>      # Install without confirmation
```

**Removal**
```bash
apt remove <package>          # Remove package (keep config)
apt purge <package>           # Remove package + config files
apt autoremove                # Remove unused dependencies
```

**System Update**
```bash
apt update                    # Update package lists
apt upgrade                   # Upgrade installed packages
apt full-upgrade              # Upgrade + handle dependencies
apt dist-upgrade              # Distribution upgrade
```

**Search & Info**
```bash
apt search <keyword>          # Search for packages
apt show <package>            # Show package details
apt list --installed          # List installed packages
apt list --upgradable         # List upgradable packages
```

**Cache Management**
```bash
apt clean                     # Clean package cache
apt autoclean                 # Clean obsolete cache files
```

#### DPKG (Low-level)

```bash
dpkg -i package.deb           # Install .deb file
dpkg -r <package>             # Remove package
dpkg -l                       # List installed packages
dpkg -L <package>             # List files from package
dpkg -s <package>             # Show package status
```

#### Snap (Universal)

```bash
snap install <package>        # Install snap package
snap install --classic code   # Install with classic confinement
snap remove <package>         # Remove snap
snap refresh                  # Update all snaps
snap list                     # List installed snaps
snap find <keyword>           # Search snaps
```

#### APT-GET (Legacy, but still used)

```bash
apt-get install <package>     # Install package
apt-get remove <package>      # Remove package
apt-get update                # Update package lists
apt-get upgrade               # Upgrade packages
apt-get dist-upgrade          # Distribution upgrade
```

### Fedora / RHEL / CentOS

#### DNF (Dandified YUM)

**Installation**
```bash
dnf install <package>         # Install package
sudo dnf install firefox      # Install Firefox
dnf install -y <package>      # Install without confirmation
```

**Removal**
```bash
dnf remove <package>          # Remove package
dnf autoremove                # Remove unused dependencies
```

**System Update**
```bash
dnf update                    # Update all packages
dnf upgrade                   # Same as update
dnf check-update              # Check for available updates
```

**Search & Info**
```bash
dnf search <keyword>          # Search packages
dnf info <package>            # Show package info
dnf list installed            # List installed packages
dnf provides <file>           # Find package providing file
```

**Repository Management**
```bash
dnf repolist                  # List enabled repositories
dnf config-manager --add-repo <url>  # Add repository
dnf clean all                 # Clean cache
```

#### YUM (Legacy)

```bash
yum install <package>         # Install package
yum remove <package>          # Remove package
yum update                    # Update packages
yum search <keyword>          # Search packages
```

#### RPM (Low-level)

```bash
rpm -i package.rpm            # Install RPM file
rpm -e <package>              # Remove package
rpm -qa                       # List all installed packages
rpm -ql <package>             # List files from package
rpm -qi <package>             # Show package info
```

### openSUSE

#### Zypper

**Installation**
```bash
zypper install <package>      # Install package
zypper in <package>           # Short form
sudo zypper in firefox        # Install Firefox
```

**Removal**
```bash
zypper remove <package>       # Remove package
zypper rm <package>           # Short form
```

**System Update**
```bash
zypper refresh                # Refresh repositories (zypper ref)
zypper update                 # Update packages (zypper up)
zypper dist-upgrade           # Distribution upgrade (zypper dup)
```

**Search & Info**
```bash
zypper search <keyword>       # Search packages (zypper se)
zypper info <package>         # Show package info (zypper if)
zypper packages               # List all packages
```

**Repository Management**
```bash
zypper repos                  # List repositories
zypper addrepo <url> <alias>  # Add repository
zypper removerepo <alias>     # Remove repository
```

### Universal Package Managers

#### Flatpak

```bash
flatpak install <package>     # Install Flatpak
flatpak install flathub org.gimp.GIMP  # Install GIMP
flatpak uninstall <package>   # Uninstall
flatpak update                # Update all Flatpaks
flatpak list                  # List installed
flatpak search <keyword>      # Search Flatpaks
```

**Add Flathub Repository**
```bash
flatpak remote-add --if-not-exists flathub https://flathub.org/repo/flathub.flatpakrepo
```

#### AppImage

```bash
chmod +x app.AppImage         # Make executable
./app.AppImage                # Run AppImage
```

## Munux Integration

### XP Rewards

Package manager commands provide significant XP rewards:

```yaml
Base XP: 50 points per package manager command
Streak Bonus: Up to +50% with 25+ streak
Total Possible: 75 XP per successful package operation
```

### Achievements

Special achievements unlock when using package managers:

- **First Pacman** (🏔️): "Arch User - BTW, I use Arch!" - 50 XP
- **First APT** (📦): "Debian Disciple" - 50 XP
- **First DNF**: "Fedora Faithful" - 50 XP
- **First Zypper**: "openSUSE Enthusiast" - 50 XP

### In-App Help

Access distribution-specific guides directly in Munux:

```bash
help arch     # Manjaro/Arch guide
help debian   # Ubuntu/Debian guide
help fedora   # Fedora/RHEL guide
help opensuse # openSUSE guide
```

## Best Practices

### Security

1. **Always use sudo** for system-wide installations
2. **Verify package sources** before installing from third-party repos
3. **Read package descriptions** before installing
4. **Keep system updated** regularly

### Performance

1. **Clean cache regularly** to free disk space
2. **Remove orphaned packages** to keep system lean
3. **Use minimal installations** when possible
4. **Update during off-hours** for large updates

### Safety

1. **Backup before major updates**
2. **Test new software in virtual machines first**
3. **Read update changelogs** for breaking changes
4. **Avoid mixing package managers** for the same software

## Common Tasks

### Installing Development Tools

**Arch/Manjaro**
```bash
sudo pacman -S base-devel git cmake
yay -S visual-studio-code-bin
```

**Ubuntu/Debian**
```bash
sudo apt install build-essential git cmake
sudo snap install --classic code
```

**Fedora**
```bash
sudo dnf groupinstall "Development Tools"
sudo dnf install git cmake code
```

### Installing Media Codecs

**Arch/Manjaro**
```bash
sudo pacman -S ffmpeg gstreamer-plugins-{base,good,bad,ugly}
```

**Ubuntu/Debian**
```bash
sudo apt install ubuntu-restricted-extras
sudo apt install ffmpeg
```

**Fedora**
```bash
sudo dnf install ffmpeg
sudo dnf install gstreamer1-plugins-{base,good,bad,ugly}
```

### System Cleanup

**Arch/Manjaro**
```bash
sudo pacman -Sc      # Clean cache
sudo pacman -Rns $(pacman -Qdtq)  # Remove orphans
```

**Ubuntu/Debian**
```bash
sudo apt autoremove  # Remove unused packages
sudo apt clean       # Clean cache
```

**Fedora**
```bash
sudo dnf autoremove  # Remove unused dependencies
sudo dnf clean all   # Clean cache
```

## Troubleshooting

### Package Conflicts

**Arch**
```bash
pacman -Syu --overwrite '*'  # Force overwrite conflicting files
```

**Ubuntu/Debian**
```bash
sudo apt --fix-broken install  # Fix broken dependencies
sudo dpkg --configure -a       # Configure pending packages
```

**Fedora**
```bash
sudo dnf distro-sync  # Synchronize packages
```

### Locked Database

**Arch**
```bash
sudo rm /var/lib/pacman/db.lck
```

**Ubuntu/Debian**
```bash
sudo rm /var/lib/apt/lists/lock
sudo rm /var/cache/apt/archives/lock
```

### Failed Updates

1. Check internet connection
2. Refresh repository lists
3. Clear package cache
4. Check disk space
5. Review error messages carefully

## Additional Resources

- **Arch Wiki**: https://wiki.archlinux.org/
- **Ubuntu Documentation**: https://help.ubuntu.com/
- **Fedora Docs**: https://docs.fedoraproject.org/
- **openSUSE Wiki**: https://en.opensuse.org/Portal:Wiki

---

**Next:** [Troubleshooting Guide](troubleshooting.md) for common issues and solutions.
