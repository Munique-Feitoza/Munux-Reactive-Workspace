# 📦 Guia de Gerenciadores de Pacotes

O Munux reconhece nativamente e gamifica o uso de gerenciadores de pacotes. Usá-los corretamente recompensa você com muito XP.

![Multi-Distro](https://img.shields.io/badge/Suporte-Multi--Distro-brightgreen) ![Recompensas XP](https://img.shields.io/badge/XP-50%20por%20comando-blue)

> [!IMPORTANT]
> **Suporte Multi-Distro:** O Munux detecta qual distribuição você está rodando e ajusta as sugestões de acordo.

---

## Arch Linux / Manjaro

### 🏔️ Pacman & AUR

| Ação | Comando | Recompensa Munux |
|:-------|:--------|:------------:|
| **Instalar** | `sudo pacman -S firefox` | `50 XP` |
| **Remover** | `sudo pacman -Rns firefox` | `50 XP` |
| **Atualizar** | `sudo pacman -Syu` | `75 XP` |
| **Pesquisar** | `pacman -Ss termo` | `20 XP` |

#### Auxiliares AUR (Yay / Paru)

```bash
# Instalar do AUR
yay -S visual-studio-code-bin

# Atualizar pacotes do AUR
yay -Syu
```

> [!TIP]
> O Munux também suporta **Yay** e **Paru**. O primeiro uso desbloqueia a conquista **"Explorador AUR"**!

---

## Debian / Ubuntu

### 📦 APT & Snap

| Ação | Comando | Recompensa Munux |
|:-------|:--------|:------------:|
| **Instalar** | `sudo apt install git` | `50 XP` |
| **Remover** | `sudo apt purge git` | `50 XP` |
| **Atualizar Índice** | `sudo apt update` | `25 XP` |
| **Upgrade Geral** | `sudo apt update && sudo apt upgrade` | `75 XP` |

---

## Fedora / RHEL

### 🎩 DNF

| Ação | Comando | Recompensa Munux |
|:-------|:--------|:------------:|
| **Instalar** | `sudo dnf install htop` | `50 XP` |
| **Upgrade** | `sudo dnf upgrade` | `75 XP` |
| **Histórico** | `dnf history` | `30 XP` |

---

## Universal: Flatpak

### 📦 Gerenciador de Pacotes Multi-plataforma

| Ação | Comando | Recompensa Munux |
|:-------|:--------|:------------:|
| **Instalar** | `flatpak install flathub org.gimp.GIMP` | `50 XP` |
| **Executar** | `flatpak run org.gimp.GIMP` | `10 XP` |
| **Atualizar** | `flatpak update` | `30 XP` |

---

## 🛡️ Melhores Práticas e Segurança

### ✅ FAÇA

- **Leia as listas de pacotes antes de confirmar** (+10 XP)
- **Atualize regularmente** (+25 XP)
- **Limpe o cache de pacotes** (+20 XP)

### ❌ NÃO FAÇA

- **Upgrades parciais** (Quebra o streak)
- **Forçar instalação em conflitos** (-50 XP)

---

## 🎯 Conquistas Relacionadas

| Conquista | Gatilho | Recompensa |
|:------------|:--------|:------:|
| 🏔️ **Arch User** | Primeiro comando `pacman` | 50 XP |
| 📦 **Debian Disciple** | Primeiro comando `apt` | 50 XP |
| 🎩 **Fedora Faithful** | Primeiro comando `dnf` | 50 XP |
| 🌍 **Distro Hopper** | Use 3+ gerenciadores diferentes | 100 XP |

---

## Próximos Passos

- 🎮 [Sistema de Gamificação](gamification-system.md)
- ⚡ [Início Rápido](quick-start.md)
- 🔧 [Solução de Problemas](troubleshooting.md)

**Boa caça aos pacotes!** 📦🚀
