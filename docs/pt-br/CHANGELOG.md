# 📋 Registro de Mudanças (Changelog)

Todas as mudanças notáveis no Munux Reactive Workspace serão documentadas neste arquivo.

O formato é baseado no [Keep a Changelog](https://keepachangelog.com/en/1.0.0/), e este projeto adere ao [Versionamento Semântico](https://semver.org/spec/v2.0.0.html).

![Versão](https://img.shields.io/badge/Última-v0.2.0-blue) ![Status](https://img.shields.io/badge/Status-Beta-yellow)

---

## [Não lançado]

### Recursos planejados
- [ ] Temas customizáveis (crie o seu)
- [ ] Sistema de plugins para comandos próprios
- [ ] Modo competitivo / placares
- [ ] Multiplayer (comparar progresso)
- [ ] Sincronização na nuvem

---

## [0.2.0] - 2026-05-12

### ✨ Adicionado
- **💾 Persistência de progresso** ([src/core/persistence.rs](../../src/core/persistence.rs)) — XP, nível, conquistas, streak, histórico de comandos e aliases são salvos em `$XDG_DATA_HOME/munux/state.json` após cada comando e ao sair. Escrita atômica (arquivo temporário + rename) e fallback `.bak` quando o save está corrompido. O streak diário é atualizado na sessão seguinte via `GameState::update_daily_streak`.
- **⌨️ Autocomplete com Tab** ([src/core/completion.rs](../../src/core/completion.rs)) — completa a primeira palavra a partir dos comandos internos + executáveis do `$PATH`, e os argumentos a partir dos caminhos do diretório atual. Estende até o maior prefixo comum e lista os candidatos quando há ambiguidade.
- **🎨 Realce de sintaxe no preview de arquivos** ([src/ui/highlight.rs](../../src/ui/highlight.rs)) — realçador leve por linha (sem dependências novas) para Rust, Python, JavaScript/TypeScript, Bash, JSON e TOML.
- **🔗 Sistema de aliases** — `alias nome='cmd'`, `alias` (lista), `unalias nome`. Os aliases são expandidos na hora de executar (com recursão limitada para evitar ciclos) e persistidos junto com o resto do progresso.
- **🎓 Modo tutorial interativo** ([src/game/tutorial.rs](../../src/game/tutorial.rs)) — `tutorial` inicia um passo a passo de 5 etapas para iniciantes (`help` → `pwd` → `ls` → `stats` → `cat <arquivo>`) com bônus de +100 XP ao concluir. `tutorial sair` encerra.
- **⏱️ Modo benchmark** ([src/game/benchmark.rs](../../src/game/benchmark.rs)) — `benchmark` roda um teste de velocidade de digitação (WPM + precisão) e recompensa com XP proporcional ao desempenho.
- **🥚 Mais easter eggs e conquistas secretas** — novos eggs (sanduíche xkcd, `42`, `xyzzy`, "the cake is a lie", reflexo do Vim, Star Wars) com conquistas secretas e a meta-conquista **Caçador de Easter Eggs** (encontrar 5+). Desbloquear um easter egg agora mostra o popup de conquista.
- **🌐 Suporte a Sessão SSH** ([src/core/ssh.rs](../../src/core/ssh.rs)) — shell remoto persistente via crate `ssh2`.
  - Cadeia de auth: `ssh-agent` → `userauth_agent` → `~/.ssh/id_rsa` (sem prompt de senha ainda).
  - Tracking de `cwd` remoto com `change_dir()` dedicado.
  - Painel Terminal com borda ciano + prompt remoto `user@host cwd$` quando a sessão está ativa.
  - Injeção automática de `--color=always` em `ls`/`grep` para preservar ANSI via `ansi-to-tui`.
  - `exit`/`logout` encerra a sessão e volta ao shell local.
- **📚 Docs atualizadas** — arquitetura e API incluem diagramas UML coloridos (classes, estados, sequência, fluxo) em EN e PT-BR.

### ♻️ Alterado / Refatorado
- Reconhecimento de easter eggs consolidado em um único `EasterEggs::classify` → enum `Egg`, fonte única de verdade tanto para a arte exibida quanto para a conquista concedida.
- Extraído `App::award_achievement` para eliminar a repetição entre os pontos que desbloqueiam conquistas.
- `FileEntry::get_icon` reescrito orientado por tabela; gravação do progresso centralizada em um único ponto (após cada comando + ao sair).
- Limpeza geral: zerei os avisos do `clippy` (condicionais aninhadas colapsadas, `parts.first()`, `rsplit().next()` no lugar de `last()` em iterador bidirecional, `Language::to_langid` recebendo `self` por valor).

### 🐛 Corrigido
- Typo na chave de localização `achievement_easter_egg_nuke-desc` (usava `_` em vez de `-`), que deixava a descrição da conquista faltando.

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
