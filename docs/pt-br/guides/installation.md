# 📥 Guia de Instalação

Instruções completas de instalação para o Munux Reactive Workspace em todas as principais distribuições Linux.

![Plataforma](https://img.shields.io/badge/Plataforma-Linux-yellow) ![Rust](https://img.shields.io/badge/Rust-1.70+-orange) ![Licença](https://img.shields.io/badge/Licença-GPLv3-blue)

---

## Pré-requisitos

### Obrigatórios

| Componente | Versão | Propósito |
|:----------|:--------|:--------|
| **Rust** | 1.70+ | Toolchain de compilação |
| **Cargo** | Última | Gerenciador de pacotes |
| **Git** | 2.0+ | Download do código-fonte |

### Recomendados

| Componente | Propósito |
|:----------|:--------|
| **Nerd Font** | Exibição de ícones (JetBrains Mono, Fira Code) |
| **Terminal 256 cores** | Suporte total a temas |
| **Suporte Unicode** | Emojis e caracteres especiais |

> [!TIP]
> Não tem o Rust? Instale em 30 segundos: `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`

---

## Métodos de Instalação

### Método 1: Pelo Código-fonte (Recomendado)

> [!IMPORTANT]
> Este é o **método recomendado** para obter as funcionalidades e atualizações mais recentes.

```bash
# 1. Clone o repositório
git clone https://github.com/Munique-Feitoza/Munux-Reactive-Workspace.git

# 2. Navegue até o diretório
cd Munux-Reactive-Workspace

# 3. Compile em modo release (IMPORTANTE para performance)
cargo build --release

# 4. Execute a aplicação
./target/release/munux-reactive-workspace
```

**Tempo de build:** ~2-5 minutos (apenas no primeiro build)  
**Tamanho do binário:** ~8-12 MB  
**Uso de memória:** ~10-20 MB em execução

---

### Método 2: Execução Rápida com Cargo

```bash
# Clone e execute em um só passo
git clone https://github.com/Munique-Feitoza/Munux-Reactive-Workspace.git
cd Munux-Reactive-Workspace
cargo run --release
```

> [!WARNING]
> Nunca use `cargo run` sem `--release` para uso real. Builds de debug são 10-50x mais lentos!

---

### Método 3: Usando Scripts Auxiliares

```bash
# Setup automatizado (instala dependências se necessário)
chmod +x setup.sh
./setup.sh

# Lançamento rápido
chmod +x run.sh
./run.sh
```

**O que o `setup.sh` faz:**

- ✅ Verifica a instalação do Rust
- ✅ Instala dependências de build
- ✅ Compila em modo release
- ✅ Verifica a compilação bem-sucedida

---

## Configuração Específica por Distribuição

### Arch Linux / Manjaro

```bash
# Instala dependências de build
sudo pacman -S base-devel git rust

# Instala fonte recomendada
yay -S ttf-jetbrains-mono-nerd

# Clone e build
git clone https://github.com/Munique-Feitoza/Munux-Reactive-Workspace.git
cd Munux-Reactive-Workspace
cargo build --release
```

---

### Ubuntu / Debian

```bash
# Instala dependências de build
sudo apt update
sudo apt install -y build-essential git curl pkg-config libssl-dev

# Instala Rust (se ainda não instalado)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env

# Instala fonte recomendada
mkdir -p ~/.fonts
cd ~/.fonts
wget https://github.com/ryanoasis/nerd-fonts/releases/download/v3.0.2/JetBrainsMono.zip
unzip JetBrainsMono.zip
fc-cache -fv
cd -

# Clone e build
git clone https://github.com/Munique-Feitoza/Munux-Reactive-Workspace.git
cd Munux-Reactive-Workspace
cargo build --release
```

---

### Fedora / RHEL

```bash
# Instala dependências de build
sudo dnf groupinstall "Development Tools"
sudo dnf install git rust cargo openssl-devel

# Instala fonte recomendada
sudo dnf install -y jetbrains-mono-fonts-all

# Clone e build
git clone https://github.com/Munique-Feitoza/Munux-Reactive-Workspace.git
cd Munux-Reactive-Workspace
cargo build --release
```

---

### openSUSE

```bash
# Instala dependências de build
sudo zypper install -t pattern devel_basis
sudo zypper install git rust cargo

# Clone e build
git clone https://github.com/Munique-Feitoza/Munux-Reactive-Workspace.git
cd Munux-Reactive-Workspace
cargo build --release
```

---

## Pós-Instalação

### 1. Adicionar ao PATH (Opcional)

Para rodar o Munux de qualquer lugar:

```bash
# Copia o binário para /usr/local/bin
sudo cp target/release/munux-reactive-workspace /usr/local/bin/munux

# Agora você pode rodar em qualquer lugar
munux
```

---

### 2. Criar Atalho na Área de Trabalho (Opcional)

Para lançadores GUI:

```bash
# Cria entrada desktop
cat > ~/.local/share/applications/munux.desktop << EOF
[Desktop Entry]
Name=Munux Reactive Workspace
Comment=Gamified Learning Terminal
Exec=/usr/local/bin/munux
Icon=utilities-terminal
Terminal=true
Type=Application
Categories=System;TerminalEmulator;
EOF
```

---

### 3. Configure Seu Terminal

Para a melhor experiência:

1. **Defina a Nerd Font:**
   - Abra as configurações do seu terminal (Konsole, GNOME Terminal, etc.)
   - Fonte: **JetBrains Mono Nerd Font**
   - Tamanho: **11** ou **12**

2. **Habilite 256 cores:**

```bash
# Adicione ao ~/.bashrc ou ~/.zshrc
export TERM=xterm-256color
```

1. **Teste o suporte Unicode:**

```bash
echo "🐧 🏆 🔥 ➜ ▶ ◆ ⬢ ⬣"
```

Se você vir ícones coloridos, você está pronto!

---

## Verificação

Teste sua instalação:

```bash
# Verifica a versão (quando implementado)
munux --version

# Executa em modo teste
cargo run --release

# Verifica o tamanho do binário
ls -lh target/release/munux-reactive-workspace
```

---

## Instruções de Atualização

```bash
# Navegue até o repositório
cd Munux-Reactive-Workspace

# Puxe as mudanças mais recentes
git pull origin main

# Recompile
cargo build --release

# Se o binário estiver no PATH, atualize-o
sudo cp target/release/munux-reactive-workspace /usr/local/bin/munux
```

---

## Desinstalação

```bash
# Remove o binário do PATH
sudo rm /usr/local/bin/munux

# Remove a entrada desktop
rm ~/.local/share/applications/munux.desktop

# Remove o código-fonte
rm -rf ~/Munux-Reactive-Workspace
```

---

## Solução de Problemas na Instalação

### Problema: "linker 'cc' not found"

**Solução:** Instale o compilador C.

```bash
# Ubuntu/Debian
sudo apt install build-essential

# Arch/Manjaro
sudo pacman -S base-devel

# Fedora
sudo dnf groupinstall "Development Tools"
```

---

### Problema: "failed to run custom build command"

**Solução:** Instale as bibliotecas de desenvolvimento do OpenSSL.

```bash
# Ubuntu/Debian
sudo apt install libssl-dev pkg-config

# Arch/Manjaro
sudo pacman -S openssl pkg-config

# Fedora
sudo dnf install openssl-devel
```

---

### Problema: "cargo: command not found"

**Solução:** Instale o Rust ou adicione o Cargo ao PATH.

```bash
# Instala Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Recarrega o shell
source $HOME/.cargo/env

# Verifica
cargo --version
```

---

## Dicas de Performance de Build

> [!TIP]
> Acelere a compilação com estes truques:

### Use `sccache` (Cache de Compilação Compartilhado)

```bash
# Instala sccache
cargo install sccache

# Configura o Cargo para usá-lo
export RUSTC_WRAPPER=sccache

# Recompile (builds subsequentes serão muito mais rápidos)
cargo build --release
```

---

## Requisitos do Sistema

| Componente | Mínimo | Recomendado |
|:----------|:--------|:------------|
| **SO** | Linux Kernel 3.0+ | 5.0+ |
| **RAM** | 512 MB | 2 GB |
| **Disco** | 50 MB | 100 MB |
| **Terminal** | ANSI Básico | 256-cores + Unicode |
| **CPU** | 1 core | 2+ cores |

> [!NOTE]
> O Munux é **extremamente leve** e roda bem até em Raspberry Pi!

---

## Próximos Passos

Após a instalação bem-sucedida:

1. ✅ **Inicie o Munux**: `munux` ou `cargo run --release`
2. 📚 **Leia o Início Rápido**: [quick-start.md](quick-start.md)
3. 🎮 **Aprenda sobre Gamificação**: [gamification-system.md](gamification-system.md)
4. 🐚 **Explore a Integração Git**: [git-integration.md](git-integration.md)

**Bem-vindo à comunidade Munux!** 🐧🚀
