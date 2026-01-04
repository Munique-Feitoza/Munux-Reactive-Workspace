# 🎨 Fonts and Emoji Configuration Guide

Munux uses Unicode characters, emojis, and Nerd Font icons to create a beautiful terminal experience. This guide shows you how to configure your system for optimal display.

![Fonts](https://img.shields.io/badge/Nerd_Fonts-Required-blue) ![Unicode](https://img.shields.io/badge/Unicode-UTF--8-green)

---

## Why Nerd Fonts?

> [!IMPORTANT]
> **Nerd Fonts** are patched fonts that include thousands of icons from popular icon packs like Font Awesome, Material Design Icons, and more.

Without Nerd Fonts, you'll see:
- ❌ `□` or `?` instead of icons
- ❌ Misaligned text
- ❌ Broken UI elements

With Nerd Fonts, you'll see:
- ✅ Beautiful icons: 🐧 🏆 🔥 ➜ ▶ ◆
- ✅ Perfect alignment
- ✅ Professional appearance

---

## Quick Test

Before configuring, test your current setup:

```bash
echo "🐧 Tux | 🏆 Achievement | 🔥 Streak | ➜ Prompt | ▶ Level 2 | ◆ Level 3"
```

**What you should see:**
```
🐧 Tux | 🏆 Achievement | 🔥 Streak | ➜ Prompt | ▶ Level 2 | ◆ Level 3
```

If you see boxes (`□`) or question marks (`?`), continue reading!

---

## Installation by Distribution

### Arch Linux / Manjaro

> [!TIP]
> Use AUR for the easiest installation.

```bash
# Install via AUR (recommended)
yay -S ttf-jetbrains-mono-nerd
yay -S ttf-firacode-nerd
yay -S ttf-hack-nerd

# Or install manually
sudo pacman -S --needed wget unzip
mkdir -p ~/.local/share/fonts
cd ~/.local/share/fonts
wget https://github.com/ryanoasis/nerd-fonts/releases/download/v3.0.2/JetBrainsMono.zip
unzip JetBrainsMono.zip
rm JetBrainsMono.zip
fc-cache -fv
```

---

### Ubuntu / Debian

```bash
# Install dependencies
sudo apt update
sudo apt install -y wget unzip fontconfig

# Download and install JetBrains Mono Nerd Font
mkdir -p ~/.local/share/fonts
cd ~/.local/share/fonts
wget https://github.com/ryanoasis/nerd-fonts/releases/download/v3.0.2/JetBrainsMono.zip
unzip JetBrainsMono.zip
rm JetBrainsMono.zip

# Refresh font cache
fc-cache -fv

# Verify installation
fc-list | grep "JetBrains"
```

---

### Fedora / RHEL

```bash
# Install via DNF (official repositories)
sudo dnf install -y jetbrains-mono-fonts-all

# Or install Nerd Font variant manually
sudo dnf install -y wget unzip
mkdir -p ~/.local/share/fonts
cd ~/.local/share/fonts
wget https://github.com/ryanoasis/nerd-fonts/releases/download/v3.0.2/JetBrainsMono.zip
unzip JetBrainsMono.zip
rm JetBrainsMono.zip
fc-cache -fv
```

---

### openSUSE

```bash
# Install manually
sudo zypper install -y wget unzip
mkdir -p ~/.local/share/fonts
cd ~/.local/share/fonts
wget https://github.com/ryanoasis/nerd-fonts/releases/download/v3.0.2/JetBrainsMono.zip
unzip JetBrainsMono.zip
rm JetBrainsMono.zip
fc-cache -fv
```

---

## Recommended Nerd Fonts

| Font | Best For | Download Link |
|:-----|:---------|:--------------|
| **JetBrains Mono Nerd Font** | Programming (recommended) | [Download](https://github.com/ryanoasis/nerd-fonts/releases/latest) |
| **Fira Code Nerd Font** | Ligatures support | [Download](https://github.com/ryanoasis/nerd-fonts/releases/latest) |
| **Hack Nerd Font** | Clarity at small sizes | [Download](https://github.com/ryanoasis/nerd-fonts/releases/latest) |
| **Source Code Pro Nerd Font** | Adobe quality | [Download](https://github.com/ryanoasis/nerd-fonts/releases/latest) |
| **Cascadia Code Nerd Font** | Microsoft's developer font | [Download](https://github.com/ryanoasis/nerd-fonts/releases/latest) |

> [!NOTE]
> All Nerd Fonts are **free and open source** (SIL Open Font License).

---

## Terminal Configuration

After installing the font, configure your terminal emulator:

### Konsole (KDE)

```bash
# 1. Open Konsole
# 2. Settings → Edit Current Profile
# 3. Appearance tab
# 4. Font: JetBrains Mono Nerd Font
# 5. Size: 11 or 12
# 6. Apply
```

---

### GNOME Terminal

```bash
# 1. Open GNOME Terminal
# 2. Preferences → Your Profile
# 3. Text tab
# 4. ✅ Custom font
# 5. Select: JetBrains Mono Nerd Font 11
# 6. Close
```

---

### Alacritty

Edit `~/.config/alacritty/alacritty.yml`:

```yaml
font:
  normal:
    family: "JetBrainsMono Nerd Font"
    style: Regular
  bold:
    family: "JetBrainsMono Nerd Font"
    style: Bold
  italic:
    family: "JetBrainsMono Nerd Font"
    style: Italic
  size: 11.0
```

---

### Kitty

Edit `~/.config/kitty/kitty.conf`:

```conf
font_family      JetBrainsMono Nerd Font
bold_font        auto
italic_font      auto
bold_italic_font auto
font_size 11.0
```

---

### Terminator

```bash
# 1. Right-click → Preferences
# 2. Profiles → Default
# 3. ✅ Use system fixed width font (uncheck)
# 4. Font: JetBrains Mono Nerd Font 11
# 5. Close
```

---

### Tilix

```bash
# 1. Preferences → Default Profile
# 2. Text appearance
# 3. Custom font: JetBrains Mono Nerd Font 11
# 4. Close
```

---

## Emoji Support

Munux uses emojis extensively: 🐧 🏆 🔥 📊 📁 🎯

### Ensure UTF-8 Locale

```bash
# Check current locale
locale

# Expected output includes:
# LANG=en_US.UTF-8
# LC_ALL=en_US.UTF-8
```

If not set correctly:

```bash
# Add to ~/.bashrc or ~/.zshrc
export LANG=en_US.UTF-8
export LC_ALL=en_US.UTF-8

# Reload shell
source ~/.bashrc
```

---

### Install Emoji Fonts

```bash
# Ubuntu/Debian
sudo apt install fonts-noto-color-emoji

# Arch/Manjaro
sudo pacman -S noto-fonts-emoji

# Fedora
sudo dnf install google-noto-emoji-fonts

# openSUSE
sudo zypper install noto-coloremoji-fonts
```

---

## Font Configuration Priority

Create `~/.config/fontconfig/fonts.conf` to prioritize Nerd Font:

```xml
<?xml version="1.0"?>
<!DOCTYPE fontconfig SYSTEM "fonts.dtd">
<fontconfig>
  <alias>
    <family>monospace</family>
    <prefer>
      <family>JetBrainsMono Nerd Font</family>
      <family>Noto Color Emoji</family>
    </prefer>
  </alias>
</fontconfig>
```

Then refresh:

```bash
fc-cache -fv
```

---

## Verification Steps

### 1. Test Nerd Font Icons

```bash
echo -e "\ue0b0 \ue0b1 \ue0b2 \ue0b3"  # Powerline symbols
echo -e "\uf113 \uf269 \uf489 \uf17c"  # File icons
```

**Expected:** You should see various triangle and file icons.

---

### 2. Test Emojis

```bash
echo "🐧 🚀 💻 🎮 🏆 🔥 📊 📁 ✅ ❌"
```

**Expected:** Colorful emojis (may be monochrome depending on terminal).

---

### 3. Test Munux Symbols

```bash
echo "➜ ► ▶ ◆ ⬢ ⬣"
```

**Expected:** Various arrow and shape symbols used for level indicators.

---

### 4. Launch Munux

```bash
cd Munux-Reactive-Workspace
cargo run --release
```

Look for:
- 🐧 Tux penguin in welcome screen
- ➜ Prompt symbol
- 🏆 Achievement icons
- 🔥 Streak fire

---

## Troubleshooting

### Issue: "Font installed but icons still show as boxes"

**Solution 1:** Clear font cache and rebuild.

```bash
fc-cache -fv
# Restart terminal
```

**Solution 2:** Verify font is actually installed.

```bash
fc-list | grep -i "nerd"
```

You should see entries like:
```
JetBrainsMono Nerd Font:style=Regular
```

---

### Issue: "Emojis are black and white instead of color"

This is **normal** for many terminal emulators. Color emoji support is limited.

**Workaround:** Use a terminal with color emoji support:
- Kitty ✅
- Alacritty (with recent versions) ✅
- GNOME Terminal (limited) ⚠️
- Konsole (limited) ⚠️

---

### Issue: "Icons are misaligned or overlapping"

**Solution:** Adjust font size.

Try sizes: **10**, **11**, **12**, or **13** until alignment is perfect.

---

### Issue: "Font looks blurry"

**Solution:** Enable font hinting.

```bash
# Edit ~/.config/fontconfig/fonts.conf
<match target="font">
  <edit name="antialias" mode="assign">
    <bool>true</bool>
  </edit>
  <edit name="hinting" mode="assign">
    <bool>true</bool>
  </edit>
  <edit name="hintstyle" mode="assign">
    <const>hintfull</const>
  </edit>
</match>
```

---

## Font Size Recommendations

| Terminal Size | Font Size | Use Case |
|:--------------|:----------|:---------|
| 1920x1080 | 11-12 | Standard desktop |
| 2560x1440 | 13-14 | QHD monitor |
| 3840x2160 | 16-18 | 4K display |
| 1366x768 | 10-11 | Laptop |

> [!TIP]
> Adjust based on your eyesight and viewing distance!

---

## Alternative Font Options

If Nerd Fonts don't work for you:

### Option 1: Powerline Fonts (Limited icons)

```bash
# Ubuntu/Debian
sudo apt install fonts-powerline

# Arch/Manjaro
sudo pacman -S powerline-fonts
```

> [!WARNING]
> Powerline fonts have fewer icons than Nerd Fonts. Some Munux UI elements may not display correctly.

---

### Option 2: Fallback Mode (Future feature)

A future version of Munux will include a `--no-icons` mode for ASCII-only display.

---

## Best Practices

1. ✅ **Use a Nerd Font** (JetBrains Mono recommended)
2. ✅ **Install emoji fonts** (Noto Color Emoji)
3. ✅ **Set UTF-8 locale** (`LANG=en_US.UTF-8`)
4. ✅ **Use size 11-12** for most displays
5. ✅ **Enable font antialiasing** for clarity
6. ✅ **Test before using Munux** (`echo "🐧 ➜ 🏆"`)

---

## Next Steps

After configuring fonts:

1. 🚀 [Launch Munux](quick-start.md)
2. 📚 [Learn the basics](quick-start.md)
3. 🎮 [Understand gamification](gamification-system.md)

**Enjoy your beautiful terminal experience!** 🎨✨
