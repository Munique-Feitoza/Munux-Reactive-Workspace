# 🐚 Integração Git Inteligente

O Munux inclui uma integração Git de nível profissional que transforma seu prompt em um painel de desenvolvimento em tempo real. Ele fornece feedback imediato sobre o estado do seu repositório sem exigir comandos manuais de `git status`.

---

## 🚀 Como Funciona

O prompt detecta automaticamente se você está dentro de um repositório Git (ou qualquer subpasta) e anexa um **Segmento Git** após o seu nome de usuário.

**Formato:** `(repo:branch +staged ~modified ?untracked ↑ahead ↓behind)`

---

## 🧭 Pipeline de Detecção

```mermaid
flowchart LR
    A([cd / comando]):::in --> B{É um<br/>repo git?}:::q
    B -- não --> Plain[Prompt simples]:::out
    B -- sim --> C[Resolve nome do repo]:::core
    C --> D[Lê HEAD → branch]:::core
    D --> E[git status --porcelain]:::core
    E --> F[Conta +~?]:::core
    F --> G[Compara upstream<br/>↑ahead ↓behind]:::core
    G --> H([Renderiza segmento Git]):::out

    classDef in fill:#ffd166,stroke:#d4a017,color:#000
    classDef out fill:#a0e7e5,stroke:#17a2b8,color:#000
    classDef core fill:#b4a7f5,stroke:#6f42c1,color:#000
    classDef q fill:#fde68a,stroke:#d97706,color:#000
```

---

## 🚦 Máquina de Estados do Repo

```mermaid
stateDiagram-v2
    direction LR
    [*] --> Limpo
    Limpo --> Modificado : editou arquivo
    Modificado --> Staged : git add
    Staged --> Commitado : git commit
    Commitado --> Ahead : commits locais
    Ahead --> Sincronizado : git push
    Sincronizado --> Limpo
    Limpo --> Behind : remoto recebeu push
    Behind --> Limpo : git pull
    Modificado --> Untracked : novo arquivo
    Untracked --> Staged : git add

    classDef clean fill:#b8e994,stroke:#38a169,color:#000
    classDef warn fill:#fde68a,stroke:#d97706,color:#000
    classDef bad fill:#ff6b6b,stroke:#c0392b,color:#fff
    classDef info fill:#81d4fa,stroke:#0277bd,color:#000
    class Limpo clean
    class Sincronizado clean
    class Modificado warn
    class Staged warn
    class Untracked bad
    class Behind bad
    class Ahead info
    class Commitado info
```

---

## 📊 Referência de Indicadores

Cada símbolo e cor no segmento Git tem um significado específico projetado para leitura rápida.

| Símbolo | Nome | Cor | Significado |
|:---:|:---|:---|:---|
| `+` | **Staged** | Verde | Arquivos na área de preparação (`git add`) |
| `~` | **Modificado** | Amarelo | Arquivos alterados mas não adicionados (staged) |
| `?` | **Untracked** | Vermelho | Novos arquivos ainda não rastreados pelo Git |
| `↑` | **Ahead** | Ciano | Commits locais que ainda não estão no servidor |
| `↓` | **Behind** | Vermelho | Novos commits no servidor que você precisa puxar |

---

## 🎨 Exemplos Visuais

### 1. Tudo Limpo

`(Munux-Project:main)`
> Você está na branch `main` e tudo está sincronizado e salvo.

### 2. Desenvolvimento Ativo

`(Munux-Project:feature/ui ~5 ?2)`
> Você está em uma branch de funcionalidade com 5 arquivos modificados e 2 novos arquivos.

### 3. Pronto para o Push

`(Munux-Project:main ↑3)`
> Você fez 3 commits locais e está pronto para rodar `git push`.

### 4. Precisa de Pull

`(Munux-Project:main ↓1)`
> Alguém enviou mudanças para o servidor. Hora de dar `git pull`!

---

## 🌟 Dicas Pro

### Visibilidade

O prompt usa variantes **Light** de Azul e Magenta para o nome do repositório e da branch, garantindo leitura total independente da transparência ou cor de fundo do seu terminal.

### Atualizações em Tempo Real

O prompt é **reativo**. Ele atualiza sempre que você:

- Muda de diretório (`cd`)
- Executa um comando (`git add`, `git commit`, etc.)
- Modifica arquivos no background

---

## 🎮 Impacto na Gamificação

Usar comandos Git no Munux contribui para sua progressão:

- **XP Base**: Cada comando `git` bem-sucedido concede **25 XP**.
- **XP de Sincronia**: Manter o status atualizado concede bônus de **10 XP**.

---

## Próximos Passos

- Saiba mais sobre o [Sistema de Gamificação](gamification-system.md)
- Volte para o [Guia de Início Rápido](quick-start.md)
