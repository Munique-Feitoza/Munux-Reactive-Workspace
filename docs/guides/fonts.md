# 🎨 Font and Emoji Configuration - Munux Reactive Workspace

> **Note:** This guide is about configuring fonts for the best Munux experience. For Portuguese speakers, see the original configuration notes at the end of this document.

## Why Font Configuration Matters

Munux uses Unicode characters and emojis throughout the interface:
- 🐧 Tux penguin (6 evolutionary forms)
- 🏆 Achievement badges
- 🔥 Streak indicators
- 📊 Progress bars
- ⬢ Level symbols
- And many more...

Without proper font configuration, you might see:
- ▯ Empty boxes
- � Question marks
- Broken borders
- Missing emojis

## Recommended Fonts

### Best Option: Nerd Fonts

Nerd Fonts are patched fonts that include thousands of glyphs and icons.

**Popular Nerd Fonts:**
- **JetBrains Mono Nerd Font** (recommended for coding)
- **Fira Code Nerd Font**
- **Hack Nerd Font**
- **Source Code Pro Nerd Font**
- **Ubuntu Mono Nerd Font**

### Installation

#### Arch Linux / Manjaro
```bash
# Install all Nerd Fonts
yay -S nerd-fonts-complete

# Or specific font:
yay -S ttf-jetbrains-mono-nerd
yay -S ttf-fira-code-nerd
yay -S ttf-hack-nerd
```

#### Ubuntu / Debian
```bash
# Option 1: From repository
sudo apt install fonts-nerd-font

# Option 2: Manual installation
# Download from https://www.nerdfonts.com/font-downloads
# Example for JetBrains Mono:
mkdir -p ~/.local/share/fonts
cd ~/.local/share/fonts
wget https://github.com/ryanoasis/nerd-fonts/releases/download/v3.1.1/JetBrainsMono.zip
unzip JetBrainsMono.zip
rm JetBrainsMono.zip
fc-cache -fv
```

#### Fedora
```bash
# Install Nerd Fonts from Copr
sudo dnf copr enable peterwu/iosevka
sudo dnf install iosevka-term-fonts

# Or manual installation (same as Ubuntu)
```

#### openSUSE
```bash
# Manual installation
mkdir -p ~/.local/share/fonts
cd ~/.local/share/fonts
wget https://github.com/ryanoasis/nerd-fonts/releases/download/v3.1.1/JetBrainsMono.zip
unzip JetBrainsMono.zip
rm JetBrainsMono.zip
fc-cache -fv
```

### Manual Installation (Any Distro)

1. **Download** your preferred Nerd Font from [nerdfonts.com](https://www.nerdfonts.com/)

2. **Extract** the ZIP file

3. **Install** the fonts:
```bash
# System-wide (requires sudo)
sudo cp *.ttf /usr/share/fonts/truetype/
sudo fc-cache -fv

# User-only (recommended)
mkdir -p ~/.local/share/fonts
cp *.ttf ~/.local/share/fonts/
fc-cache -fv
```

4. **Verify** installation:
```bash
fc-list | grep "JetBrains Mono"
```

## Terminal Configuration

After installing fonts, configure your terminal emulator:

### GNOME Terminal
1. Open Terminal → Preferences
2. Select your profile
3. Fonts tab
4. Uncheck "Use system font"
5. Select your Nerd Font (e.g., "JetBrainsMono Nerd Font Mono")
6. Font size: 11-13 recommended

### Konsole (KDE)
1. Settings → Edit Current Profile
2. Appearance tab
3. Select Font → Choose Nerd Font
4. Apply

### Alacritty
Edit `~/.config/alacritty/alacritty.yml`:
```yaml
font:
  normal:
    family: JetBrainsMono Nerd Font
    style: Regular
  size: 11.0
```

### Kitty
Edit `~/.config/kitty/kitty.conf`:
```conf
font_family      JetBrainsMono Nerd Font
font_size 11.0
```

### Terminator
1. Right-click → Preferences
2. Profiles → General
3. Uncheck "Use system font"
4. Select your Nerd Font

### XFCE Terminal
1. Edit → Preferences
2. Appearance tab
3. Font: Select Nerd Font

### Tilix
1. Preferences → Profile → Default
2. Font: Select Nerd Font

## Emoji Support

### Install Emoji Fonts

#### Ubuntu / Debian
```bash
sudo apt install fonts-noto-color-emoji
```

#### Arch / Manjaro
```bash
sudo pacman -S noto-fonts-emoji
```

#### Fedora
```bash
sudo dnf install google-noto-emoji-fonts
```

### Emoji Rendering

Create/edit `~/.config/fontconfig/fonts.conf`:
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

Then refresh font cache:
```bash
fc-cache -fv
```

## Testing Your Configuration

After configuration, test in a new terminal:

### Test 1: Basic Emojis
```bash
echo "🐧 🏆 🔥 📊 ⬢ ◆"
```
Should display: 🐧 🏆 🔥 📊 ⬢ ◆

### Test 2: Nerd Font Icons
```bash
echo "        "
```
Should show various programming icons.

### Test 3: Tux ASCII Art
Run Munux and check if Tux displays correctly on the welcome screen.

### Test 4: Box Drawing Characters
```bash
echo "┌─────┐"
echo "│ Box │"
echo "└─────┘"
```
Should display a proper box, not broken lines.

## Troubleshooting

### Problem: Emojis show as boxes

**Solution:**
1. Install emoji font (see Emoji Support section)
2. Update fontconfig (see Emoji Rendering section)
3. Restart terminal

### Problem: Nerd Font icons don't show

**Solution:**
1. Verify font is installed: `fc-list | grep Nerd`
2. Make sure terminal is using Nerd Font (check terminal settings)
3. Try different Nerd Font variant (Regular, Mono, etc.)

### Problem: Mixed character sizes

**Solution:**
- Use the "Mono" variant of Nerd Font
- Example: "JetBrainsMono Nerd Font Mono" instead of "JetBrainsMono Nerd Font"

### Problem: Tux appears distorted

**Solution:**
1. Increase terminal font size (11-13 recommended)
2. Make sure terminal size is at least 80x24
3. Use monospace Nerd Font

## Recommended Configuration

For best Munux experience:

```
Font: JetBrainsMono Nerd Font Mono
Size: 11-12 pt
Emoji: Noto Color Emoji
Terminal Size: At least 100x30
Color Depth: 256 colors (xterm-256color)
```

## Additional Resources

- **Nerd Fonts Website:** https://www.nerdfonts.com/
- **Nerd Fonts GitHub:** https://github.com/ryanoasis/nerd-fonts
- **Font Configuration Guide:** https://wiki.archlinux.org/title/Fonts
- **Emoji on Linux:** https://wiki.archlinux.org/title/Fonts#Emoji_and_symbols

---

## Original Portuguese Notes (PT-BR)

<details>
<summary>Clique para ver a documentação original em português</summary>

### Configuração de Fontes para Munux

O Munux usa caracteres Unicode e emojis em toda a interface. Para melhor experiência:

**Fontes Recomendadas:**
- JetBrains Mono Nerd Font
- Fira Code Nerd Font
- Hack Nerd Font

**Instalação no Manjaro:**
```bash
yay -S nerd-fonts-complete
# ou específico:
yay -S ttf-jetbrains-mono-nerd
```

**Configuração no Konsole:**
1. Configurações → Editar Perfil Atual
2. Aparência → Selecionar Fonte
3. Escolher: JetBrainsMono Nerd Font Mono
4. Tamanho: 11-12

**Emojis:**
```bash
sudo pacman -S noto-fonts-emoji
```

**Testar:**
```bash
echo "🐧 Munux é demais! 🚀"
```

</details>

---

**Last Updated:** January 3, 2026  
**Version:** v0.1.0 BETA

For more configuration help, see [Troubleshooting](troubleshooting.md).
