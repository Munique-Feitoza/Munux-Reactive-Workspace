# 📋 Registro de Mudanças (Changelog)

Todas as mudanças notáveis no Munux Reactive Workspace serão documentadas neste arquivo.

O formato é baseado no [Keep a Changelog](https://keepachangelog.com/en/1.0.0/), e este projeto adere ao [Versionamento Semântico](https://semver.org/spec/v2.0.0.html).

![Versão](https://img.shields.io/badge/Última-v0.1.1-blue) ![Status](https://img.shields.io/badge/Status-Beta-yellow)

---

## [Não lançado]

### ✨ Adicionado
- **🌐 Suporte a Sessão SSH** ([src/core/ssh.rs](../../src/core/ssh.rs)) — shell remoto persistente via crate `ssh2`.
  - Cadeia de auth: `ssh-agent` → `userauth_agent` → `~/.ssh/id_rsa` (sem prompt de senha ainda).
  - Tracking de `cwd` remoto com `change_dir()` dedicado.
  - Painel Terminal com borda ciano + prompt remoto `user@host cwd$` quando a sessão está ativa.
  - Injeção automática de `--color=always` em `ls`/`grep` para preservar ANSI via `ansi-to-tui`.
  - `exit`/`logout` encerra a sessão e volta ao shell local.
- **📚 Docs atualizadas** — arquitetura e API agora incluem **diagramas UML coloridos** (classes, estados, sequência, fluxo) em EN e PT-BR.

---

## [0.1.1] - 2026-02-16

### ✨ Adicionado

- 🌍 **Documentação Bilíngue Completa**: Todo o ecossistema de docs agora está disponível em Inglês (EN) e Português (PT-BR).
- 🐚 **Integração Git Inteligente**: Novo prompt com counters de arquivos (staged, modified, untracked) e status de sincronia (ahead/behind).

---

## [0.1.0] - 2026-01-03 (BETA)

> [!IMPORTANT]
> **Lançamento Inicial Beta** - Primeira versão pública do Munux!

### ✨ Funcionalidades Iniciais

- 🐧 **Terminal Funcional**: Execução via shell `sh -c`.
- 🎮 **Gamificação**: Sistema completo de XP, níveis e conquistas.
- 📊 **Interface Reativa**: Painéis divididos (60/40) que mudam conforme o comando.
- 🎨 **Temas Progressivos**: 6 temas desbloqueáveis (Beginner → Legend).
- 🔥 **Sistema de Streak**: Multiplicadores de XP por uso consistente.

---

## Roadmap

### v0.2.0 (Q1 2026)

- [ ] Estado persistente (Salvar XP em disco)
- [ ] Histórico de comandos persistente
- [ ] Editor de temas customizados

---

## Contribuição

Veja o [Código de Conduta](contributing/code-of-conduct.md) para diretrizes sobre como contribuir.
