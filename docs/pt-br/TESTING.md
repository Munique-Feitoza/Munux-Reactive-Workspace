# 🧪 Guia de Testes

Documentação abrangente de testes para o Munux Reactive Workspace.

![Testes](https://img.shields.io/badge/Testes-Passando-brightgreen) ![Cobertura](https://img.shields.io/badge/Cobertura-85%25-green) ![CI](https://img.shields.io/badge/CI-GitHub_Actions-blue)

> [!NOTE]
> O Munux segue princípios de **desenvolvimento orientado a testes (TDD)**. Todos os módulos principais possuem testes unitários.

---

## Início Rápido

```bash
# Rodar todos os testes
cargo test

# Rodar testes mostrando as saídas
cargo test -- --nocapture

# Rodar um teste específico
cargo test test_xp_calculation

# Rodar testes em modo release (mais rápido)
cargo test --release
```

---

## Organização dos Testes

- `core/`: Testes de parser, shell, filesystem e monitoramento.
- `game/`: Testes de estado, lógica, conquistas e missões.
- `ui/`: Testes de temas e reatividade.

Total: ~108 testes unitários.

---

## Exemplos de Testes

### Testes de Parser

Verificam se os comandos são classificados corretamente e se o cálculo de XP está correto.

### Testes de Lógica de Jogo

Garantem que a progressão de nível, multiplicadores de streak e cálculo de tiers funcionam conforme esperado.

### Testes de Sistema de Arquivos

Testam a listagem de diretórios, leitura de arquivos e mudança de diretório, garantindo segurança e tratamento de erros.

---

## Testes de Integração

Localizados em `tests/integration_test.rs`, verificam o fluxo completo de execução de comandos e atualização de estado global.

---

## Guia de Teste Manual

### ✅ Instalação e Inicialização

- Compila sem erros em Arch, Ubuntu e Fedora.
- Abre sem crashar.
- Terminal é restaurado corretamente ao sair (`Ctrl+C`).

### ✅ Gamificação

- XP aumenta após o comando.
- Notificação de Level Up aparece.
- Conquistas desbloqueiam corretamente.
- Streak quebra em caso de erro de comando.

---

## Cobertura de Código

Utilizamos o `tarpaulin` para medição. Cobertura atual: **85%**.

---

## Próximos Passos

- 🏗️ [Visão Geral da Arquitetura](architecture/overview.md)
- 🔧 [Referência da API](api/core-modules.md)
- 🤝 [Contribuição](../contributing/code-of-conduct.md)

**Bons testes!** 🧪✨
