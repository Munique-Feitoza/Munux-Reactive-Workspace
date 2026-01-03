#!/bin/bash
# Author: Munique Alves Pacheco Feitoza
# License: GPLv3
#
# Script de instalação e setup do Munux Reactive Workspace

set -e

echo "🌌 Munux Reactive Workspace - Setup"
echo "===================================="
echo ""

# Verifica se o Rust está instalado
if ! command -v cargo &> /dev/null; then
    echo "❌ Rust não encontrado!"
    echo ""
    echo "Por favor, instale o Rust primeiro:"
    echo "curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"
    echo ""
    exit 1
fi

echo "✅ Rust encontrado: $(rustc --version)"
echo ""

# Compila o projeto
echo "🔨 Compilando o projeto..."
cargo build --release

if [ $? -eq 0 ]; then
    echo ""
    echo "✅ Compilação bem-sucedida!"
    echo ""
    echo "Para executar o Munux:"
    echo "  cargo run"
    echo ""
    echo "Ou use o binário otimizado:"
    echo "  ./target/release/munux-reactive-workspace"
    echo ""
    echo "Aproveite! 🚀"
else
    echo ""
    echo "❌ Erro na compilação. Verifique as mensagens acima."
    exit 1
fi
