# 🎨 Guia de Configuração de Fontes e Emojis

O Munux utiliza caracteres Unicode, emojis e ícones de Nerd Fonts para criar uma experiência de terminal bonita. Este guia mostra como configurar seu sistema para a exibição ideal.

![Fontes](https://img.shields.io/badge/Nerd_Fonts-Obrigatório-blue) ![Unicode](https://img.shields.io/badge/Unicode-UTF--8-green)

---

## Por que Nerd Fonts?

> [!IMPORTANT]
> **Nerd Fonts** são fontes modificadas que incluem milhares de ícones de pacotes populares como Font Awesome, Material Design Icons e outros.

Sem Nerd Fonts, você verá:

- ❌ `□` ou `?` em vez de ícones
- ❌ Texto desalinhado
- ❌ Elementos de interface quebrados

Com Nerd Fonts, você verá:

- ✅ Ícones bonitos: 🐧 🏆 🔥 ➜ ▶ ◆
- ✅ Alinhamento perfeito
- ✅ Aparência profissional

---

## Teste Rápido

Antes de configurar, teste sua configuração atual:

```bash
echo "🐧 Tux | 🏆 Conquista | 🔥 Streak | ➜ Prompt | ▶ Nível 2 | ◆ Nível 3"
```

**O que você deve ver:**

```
🐧 Tux | 🏆 Conquista | 🔥 Streak | ➜ Prompt | ▶ Nível 2 | ◆ Nível 3
```

Se você vir quadrados (`□`) ou pontos de interrogação (`?`), continue lendo!

---

## Instalação por Distribuição

### Arch Linux / Manjaro

> [!TIP]
> Use o AUR para a instalação mais fácil.

```bash
# Instalar via AUR (recomendado)
yay -S ttf-jetbrains-mono-nerd
yay -S ttf-firacode-nerd
yay -S ttf-hack-nerd

# Ou instalar manualmente
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
# Instalar dependências
sudo apt update
sudo apt install -y wget unzip fontconfig

# Baixar e instalar JetBrains Mono Nerd Font
mkdir -p ~/.local/share/fonts
cd ~/.local/share/fonts
wget https://github.com/ryanoasis/nerd-fonts/releases/download/v3.0.2/JetBrainsMono.zip
unzip JetBrainsMono.zip
rm JetBrainsMono.zip

# Atualizar cache de fontes
fc-cache -fv

# Verificar instalação
fc-list | grep "JetBrains"
```

---

### Fedora / RHEL

```bash
# Instalar via DNF (repositórios oficiais)
sudo dnf install -y jetbrains-mono-fonts-all

# Ou instalar variante Nerd Font manualmente
sudo dnf install -y wget unzip
mkdir -p ~/.local/share/fonts
cd ~/.local/share/fonts
wget https://github.com/ryanoasis/nerd-fonts/releases/download/v3.0.2/JetBrainsMono.zip
unzip JetBrainsMono.zip
rm JetBrainsMono.zip
fc-cache -fv
```

---

## Fontes Nerd Recomendadas

| Fonte | Ideal Para | Link de Download |
|:-----|:---------|:--------------|
| **JetBrains Mono Nerd Font** | Programação (recomendada) | [Download](https://github.com/ryanoasis/nerd-fonts/releases/latest) |
| **Fira Code Nerd Font** | Suporte a ligaduras | [Download](https://github.com/ryanoasis/nerd-fonts/releases/latest) |
| **Hack Nerd Font** | Clareza em tamanhos pequenos | [Download](https://github.com/ryanoasis/nerd-fonts/releases/latest) |

---

## Configuração do Terminal

Após instalar a fonte, configure o emulador de terminal:

### Konsole (KDE)

1. Configurações → Editar Perfil Atual
2. Aba Aparência
3. Fonte: **JetBrains Mono Nerd Font**
4. Tamanho: **11** ou **12**

### GNOME Terminal

1. Preferências → Seu Perfil
2. Aba Texto
3. ✅ Fonte personalizada
4. Selecionar: **JetBrains Mono Nerd Font 11**

---

## Suporte a Emojis

O Munux usa emojis extensivamente: 🐧 🏆 🔥 📊 📁 🎯

### Instalar Fontes de Emoji

```bash
# Ubuntu/Debian
sudo apt install fonts-noto-color-emoji

# Arch/Manjaro
sudo pacman -S noto-fonts-emoji

# Fedora
sudo dnf install google-noto-emoji-fonts
```

---

## Passos de Verificação

### 1. Testar Ícones Nerd Font

```bash
echo -e "\ue0b0 \ue0b1 \ue0b2 \ue0b3"  # Símbolos Powerline
echo -e "\uf113 \uf269 \uf489 \uf17c"  # Ícones de arquivo
```

### 2. Testar Emojis

```bash
echo "🐧 🚀 💻 🎮 🏆 🔥 📊 📁 ✅ ❌"
```

### 3. Testar Símbolos do Munux

```bash
echo "➜ ► ▶ ◆ ⬢ ⬣"
```

---

## Solução de Problemas

### Problema: "Fonte instalada mas ícones ainda aparecem como quadrados"

**Solução:** Limpe o cache de fontes e reinicie o terminal.

```bash
fc-cache -fv
```

### Problema: "Emojis são preto e branco em vez de coloridos"

Isso é **normal** em muitos terminais. Para suporte total a cores, use Kitty ou Alacritty.

---

## Próximos Passos

Após configurar as fontes:

1. 🚀 [Inicie o Munux](quick-start.md)
2. 📚 [Aprenda o básico](quick-start.md)
3. 🎮 [Entenda a gamificação](gamification-system.md)

**Aproveite sua bela experiência no terminal!** 🎨✨
