#!/bin/bash
# Script para executar o Munux com tratamento de erros

echo "🌌 Iniciando Munux Reactive Workspace..."
echo ""

cd "/home/muniquefeitoza/Área de trabalho/Munique/Munux-Reactive-Workspace"

# Verifica se já está compilado
if [ ! -f "target/debug/munux-reactive-workspace" ]; then
    echo "📦 Compilando pela primeira vez..."
    cargo build
fi

echo ""
echo "🚀 Iniciando o Munux..."
echo "   Pressione Ctrl+C para sair"
echo ""
sleep 1

# Executa
cargo run

# Garante que o terminal seja restaurado
reset
echo ""
echo "✅ Munux fechado. Terminal restaurado."
