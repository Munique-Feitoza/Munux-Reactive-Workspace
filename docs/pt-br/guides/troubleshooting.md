# 🔧 Guia de Solução de Problemas (Troubleshooting)

Encontrou algo errado? Confira as soluções abaixo. Se o problema persistir, abra uma [Issue no GitHub](https://github.com/Munique-Feitoza/Munux-Reactive-Workspace/issues).

![Status](https://img.shields.io/badge/Status-Beta-yellow) ![Ajuda](https://img.shields.io/badge/Comunidade-Ativa-green)

---

## 🎨 Problemas de Exibição e Fontes

### Problema: "Vejo quadrados `□` ou pontos de interrogação `?` em vez de ícones"

Isso acontece quando seu terminal não suporta **Nerd Fonts**.

> [!TIP]
> **Solução:** Instale uma Nerd Font e configure seu terminal para usá-la.

Siga o guia de [Configuração de Fontes](fonts.md) para instruções detalhadas passo a passo.

---

## 🦀 Problemas de Compilação

### Problema: `linker 'cc' not found`

Faltam as ferramentas básicas de build em C exigidas pelas dependências do Rust.

**Solução:** Instale o `build-essential` ou `base-devel`.

```bash
# Ubuntu/Debian
sudo apt update && sudo apt install build-essential

# Arch/Manjaro
sudo pacman -S base-devel
```

---

### Problema: `error: failed to run custom build command for 'some-crate'`

Geralmente significa que uma dependência do sistema (como OpenSSL) está faltando.

**Solução:** Instale as bibliotecas de desenvolvimento.

```bash
# Ubuntu/Debian
sudo apt install -y pkg-config libssl-dev

# Arch/Manjaro
sudo pacman -S pkgconf openssl
```

---

## ⚡ Problemas de Execução (Runtime)

### Problema: "O terminal parece lento ou travado"

Builds de debug no Rust são lentos devido a checagens pesadas de runtime.

> [!IMPORTANT]
> **Solução:** Sempre execute em **Modo Release** para uso real.

```bash
# ❌ LENTO (Modo Debug)
cargo run

# ✅ RÁPIDO (Modo Release - 10x a 50x mais rápido)
cargo run --release
```

---

### Problema: "Munux crasha ao iniciar"

1. **Terminal não suportado:** Verifique se sua variável `$TERM` está como `xterm-256color`.
2. **Dependências de biblioteca:** Certifique-se de ter a `glibc` ou `libc6` instalada.

---

## 🎮 Problemas de Gamificação

### Problema: "XP não atualiza / Conquistas não desbloqueiam"

1. **Verifique os status:** Digite `stats` dentro do Munux.
2. **Teste manual:** Digite `xp 100` para testar se o sistema está recebendo pontos.

---

## 🆘 Obtendo Ajuda

### Opção 1: GitHub Issues

Para bugs ou pedidos de funcionalidades: [GitHub Issues](https://github.com/Munique-Feitoza/Munux-Reactive-Workspace/issues)

### Opção 2: Discussões da Comunidade

Para perguntas e ajuda geral: [GitHub Discussions](https://github.com/Munique-Feitoza/Munux-Reactive-Workspace/discussions)

### Opção 3: Ajuda no App

Digite `help` dentro do Munux para ver os comandos suportados e guias específicos.

**Estamos aqui para ajudar!** 💪🐧
