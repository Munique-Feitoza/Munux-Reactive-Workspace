# 📋 Registro de Mudanças (Changelog)

Todas as mudanças notáveis no Munux Reactive Workspace serão documentadas neste arquivo.

O formato é baseado no [Keep a Changelog](https://keepachangelog.com/en/1.0.0/), e este projeto adere ao [Versionamento Semântico](https://semver.org/spec/v2.0.0.html).

![Versão](https://img.shields.io/badge/Última-v0.3.1-blue) ![Status](https://img.shields.io/badge/Status-Beta-yellow)

---

## [Não lançado]

### Recursos planejados
- [ ] Temas customizáveis (crie o seu)
- [ ] Sistema de plugins para comandos próprios
- [ ] Modo competitivo / placares
- [ ] Multiplayer (comparar progresso)
- [ ] Sincronização na nuvem

---

## [0.3.1] - 2026-07-25

Versão de **saneamento**: performance, complexidade e duplicação, sem recursos novos. Precedeu deliberadamente o roadmap de features.

### 🔒 Segurança
- **Verificação de chave de host no SSH** ([src/core/ssh.rs](../../src/core/ssh.rs)) — a conexão passou a conferir a chave do servidor contra o `~/.ssh/known_hosts` **antes** de oferecer qualquer credencial. Política igual à do OpenSSH: chave conhecida segue, chave **alterada aborta** (possível man-in-the-middle), chave nova é registrada (TOFU). Antes o handshake era aceito às cegas.
- A falha de verificação é um erro **tipado** (`SshError`), para que a mensagem — a mais crítica do fluxo SSH — chegue traduzida ao usuário.

### 🐛 Corrigido
- **`add_xp` subia no máximo um nível por chamada** ([src/game/state.rs](../../src/game/state.rs)) — `add_xp(10_000)` no nível 1 deixava o estado em `level=2, xp=9900, limiar=120`, e a pessoa destravava um nível por comando até drenar a sobra. Concessões grandes são reais: o easter egg `rm -rf /` dá 666 XP.
- **Sucesso deduzido de string traduzível** ([src/app.rs](../../src/app.rs)) — conquistas e streaks dependiam de `last_output.starts_with("✗")`. Agora usam `output.success`.
- **Scroll sem teto** ([src/main.rs](../../src/main.rs)) — segurar PageDown levava a um painel vazio sem pista de retorno. `App::scroll_by` limita ao conteúdo e unifica os quatro pontos de scroll.
- **`lsof` e `last` tratados como listagem** — a comparação era por `starts_with`; agora é por token exato, vindo de `core::commands`.
- **`partial_cmp().unwrap()` no monitor** — trocado por `total_cmp`; era um panic possível dentro do caminho de render.
- **`$item` não traduzido nas quests** — o texto em português dizia "file '...' criado(a)". Agora há chaves separadas para arquivo e pasta.
- **Saída de comando sem limite** — `cat` de um arquivo grande carregava tudo na RAM e o ANSI era reprocessado a cada frame. Limite de 2.000 linhas com aviso.

### ⚡ Performance
- **Autocomplete O(k²) → O(k)** ([src/core/completion.rs](../../src/core/completion.rs)) — a deduplicação varria linearmente o acumulado. Com 7.683 executáveis no `$PATH`, um Tab de prefixo vazio custava **29,4 ms** (release); agora custa **8,5 ms**, e o que resta é `readdir` puro.
- **Fim do `read_dir` por frame** ([src/app.rs](../../src/app.rs)) — a árvore de arquivos era relida (com um `stat` por entrada e um sort) a cada tecla digitada. Agora há cache, refeito ao trocar de diretório, após cada comando e no tick de 1 s enquanto a árvore está visível.
- **Fim da releitura de arquivo por tecla** — digitar `cat arquivo.txt` relia o arquivo inteiro (até 1 MB) a cada caractere. Preview memoizado por caminho.
- **Catálogo de comandos O(n) → O(1)** ([src/core/commands.rs](../../src/core/commands.rs)) — `classify_command` roda 2× por tecla e 1× por frame e varria 133 entradas; agora consulta um índice `HashMap`.
- **`classify_command` duplicado por tecla** — `analyze_input` classificava e `command_to_panel_mode` reclassificava. Agora o tipo é calculado uma vez e repassado.
- **Monitor de sistema** ([src/core/monitor.rs](../../src/core/monitor.rs)) — `refresh_all` + `refresh_cpu` + `refresh_processes` varriam CPU e processos duas vezes por tick; agora é um refresh só. O top-5 usa seleção parcial O(n) no lugar de um sort O(n log n) completo.
- **`has_achievement` O(n) → O(1)** — até 15 consultas por comando varriam o vetor inteiro; índice `HashSet` derivado, invalidado a cada desbloqueio.
- **`find_matching_files` sem `stat` por entrada** — usa o `d_type` do `readdir`.

### ♻️ Alterado / Refatorado
- **`execute_command` deixou de ser God Object** — 382 linhas e complexidade ciclomática **57** viraram um orquestrador de três tempos (preparar / despachar / liquidar) com handlers dedicados. Nenhum deles passa de CC 11.
- **Tipos de quest viraram dados** ([src/game/quests.rs](../../src/game/quests.rs)) — as 17 variantes de `QuestObjective` exigiam editar três `match` (CC 46 + 31 + 18 = **95**) para acrescentar uma quest. Agora um objetivo é um `Trigger` + chaves de texto, e uma quest nova é uma entrada em `generate_quests_for_level`. `update_progress` caiu para CC 9.
- **Faixas de nível: 6 tabelas → 1** — `Stage` ([src/ui/theme.rs](../../src/ui/theme.rs)) é a fonte única dos cortes visuais e refina `Tier`. A tabela divergente de `level_commands` cortava em 10 enquanto todas as outras cortavam em 9: quem estava no nível 10 já era Aprendiz e via o tema Hacker, mas ainda recebia as dicas de iniciante. Um teste garante que estágio e patente não voltem a divergir.
- **PRNG único** ([src/game/rng.rs](../../src/game/rng.rs)) — `clock_index` (segundos) e `pseudo_index` (nanos) eram o mesmo conceito em duas implementações.
- **Aviso da zona de perigo virou tabela** — uma cadeia de 8 `else if` dentro de `command_to_panel_mode`.
- **Listas paralelas ao catálogo removidas** — `mkdir|touch|rm |mv |cp `, `["ls","ll","la"]` e as palavras de comando perigoso agora derivam de `core::commands`.
- **`core` deixou de escrever texto para o usuário** — `read_file_preview` devolve o fato (`truncated_at`) e a UI decide as palavras.

### 🌍 Internacionalização
- Strings que ainda estavam fixas em português no código foram para os locales: preview de arquivo, ajuda de comando, rodapé de popup, dica da zona de perigo, aviso de arquivo grande, mensagens de host key e **o tutorial interativo inteiro**.
- Novo teste estrutural: os dois locales precisam declarar **exatamente** o mesmo conjunto de chaves — uma chave adicionada só em um idioma cairia calada no fallback.

- **Guias de distro e easter eggs traduzidos** — os cinco guias do `help <distro>` (~180 linhas) e os 14 easter eggs saíram de blocos `r#"..."#` fixos em português e foram para `locales/<lang>/{guides,eggs}/*.txt`, carregados por `I18n::content`. Blocos longos e formatados não cabem no Fluent (valor multilinha exige indentação de continuação e quebraria os `{}` dos exemplos de código), então viraram arquivos de texto embutidos no binário pelo mesmo `include_dir!`. Citações de obras em inglês (Matrix, Portal, Hackers) são idênticas nos dois idiomas de propósito — e há um teste que garante isso.
- **Erros de SSH tipados** — `SshError` cobre conexão, handshake, autenticação, host key e `cd` remoto. O `core` relata o fato, a camada `app` escolhe as palavras; antes essas mensagens chegavam em português fixo a quem usa o app em inglês.
- **Balão do `cowsay` alinhado com acentos** — a borda era dimensionada em bytes (`len()`), então `cowsay coração` saía torto. Agora conta caracteres.

### 📚 Documentação
- **CHANGELOG PT e EN alinhados.** O inglês não tinha entrada **0.1.1** e listava "Internationalization" na 0.2.0 — mas os seis commits de i18n são todos de 2026-02-16, a data da 0.1.1. Corrigido nos dois idiomas, e a 0.1.1 passou a registrar o que de fato aconteceu ali (a implantação do Project Fluent). A entrada 0.0.1 (alpha interno), que só existia no inglês, foi espelhada no português.
- Badges de versão de `docs/{en,pt-br}/README.md` atualizados (estavam em 0.2.0).
- README: o roadmap de features foi renumerado para **0.4.0** — a 0.3.1 é release de saneamento.

### 💾 Formato de save
- **v1 → v2.** O formato de `QuestObjective` mudou. Saves v1 são migrados automaticamente: XP, nível, conquistas, streaks, histórico e aliases são **preservados integralmente**; apenas as quests em andamento são descartadas e regeneradas para o nível atual. Coberto por teste contra disco.

### 📊 Números
| Métrica | Antes | Depois |
|---|---|---|
| Complexidade ciclomática média | 4,2 | **3,5** |
| Funções com CC > 20 | 7 | **3** |
| Pior função (`execute_command`) | CC 57 | **< 8** |
| Quests (`update_progress`) | CC 46 | **CC 9** |
| Tab com prefixo vazio (release) | 29,4 ms | **8,5 ms** |
| Testes | 42 | **69** |

---

## [0.3.0] - 2026-06-22

### ✨ Adicionado
- **⌨️ Navegação interativa de arquivos** — com a árvore de arquivos visível e o input vazio, as setas selecionam um item e o Enter abre (arquivo → preview, diretório → cd), com seleção destacada e a dica `↑↓ navegar • Enter abrir`.
- **📊 Painel Top-5 processos** ([src/ui/reactive.rs](../../src/ui/reactive.rs)) — o monitor de recursos agora lista os 5 processos por uso de CPU (pid, nome, cpu%, memória).
- **🏅 Progressão de patente** — o painel de stats mostra a próxima patente e o nível que a desbloqueia; `game::tier::Tier` é a fonte única de patente, tema, símbolo do prompt e borda.
- **🛟 Guard RAII do terminal** ([src/tui.rs](../../src/tui.rs)) — `TerminalGuard` possui o terminal e o restaura em qualquer saída (retorno normal, `?`, panic).

### 🔒 Segurança
- **Bypass do modo seguro corrigido** ([src/core/parser.rs](../../src/core/parser.rs)) — o modo seguro agora valida **todos** os segmentos do comando (`;`, `&&`, `||`, `|`) e bloqueia substituição de comando (`$(...)`/crase). Antes só o primeiro token era checado, então `echo ok; rm -rf /tmp/x` passava batido.
- **Shell-quoting no SSH** ([src/core/ssh.rs](../../src/core/ssh.rs)) — `remote_cwd` (vindo do servidor) e caminhos remotos são citados em POSIX, evitando injeção de shell.
- **Confirmação na zona de perigo** — comandos destrutivos exigem um `sim`/`yes` explícito em vez de executar no Enter.
- **Cheat `xp` restrito** — o cheat de XP só é compilado em builds de debug.

### ♻️ Alterado / Refatorado
- **Fontes únicas de verdade**: `core/commands` (catálogo de comandos → classificação, autocomplete, cor), `core/filetype` (extensão → linguagem/ícone/cor) e `game/tier` (patentes). Um único `SystemMonitor` persistente. Novos helpers `git()`, `panel_block`, `try_t`, `parse_cd_arg`, `git_segment_spans`, `local_prompt_prefix`.
- **Internacionalização completa** — todas as strings visíveis ao usuário movidas para `locales/{pt-BR,en-US}` (~70 chaves novas); um teste garante que todas resolvem nos dois idiomas.
- `command_to_panel_mode` saiu do `core` (remove a dependência core→app); `execute_command` enxugado (handlers de comandos especiais e de sessão SSH extraídos).
- **Removido todo o código morto** — zero `#[allow(dead_code)]` restante.

### 🐛 Corrigido
- Arquivos `.ts` realçados como JavaScript (o ramo TypeScript era inalcançável).
- Divisão por zero no % de memória, panics nos gauges (agora limitados a 100), `benchmark parar` não cancelando, títulos errados nos guias de distro, e sugestões "você quis dizer?" buscando no diretório errado após `cd`.

### 🧪 Testes
- 42 testes no total — `GameState`, `Quest::update_progress`, `AchievementChecker`, resolução de chaves i18n e uma regressão de segurança do modo seguro.

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

- 🌍 **Internacionalização (Project Fluent)** ([src/i18n.rs](../../src/i18n.rs)) — todas as strings visíveis ao usuário foram para `locales/{pt-BR,en-US}/main.ftl`, com o idioma detectado a partir do locale do sistema e inglês como fallback. Painel reativo, temas, stats, quests, conquistas, terminal, HUD e o loop de eventos foram localizados.
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

## [0.0.1] - 2025-12-15 (Alpha - Interno)

### ✨ Adicionado

- Prova de conceito inicial
- Emulação básica de terminal
- Sistema de XP simples
- Protótipo de interface com Ratatui

### ♻️ Alterado

- Migração de Python para Rust por performance
- Interface redesenhada para a arquitetura de painéis divididos

### 🗑️ Removido

- Código do protótipo em Python

---

## Roadmap

### v0.2.0 (Q1 2026)

- [ ] Estado persistente (Salvar XP em disco)
- [ ] Histórico de comandos persistente
- [ ] Editor de temas customizados

---

## Contribuição

Veja o [Código de Conduta](contributing/code-of-conduct.md) para diretrizes sobre como contribuir.
