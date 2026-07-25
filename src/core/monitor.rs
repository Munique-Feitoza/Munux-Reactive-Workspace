// Author: Munique Alves Pacheco Feitoza
// License: GPLv3

use sysinfo::System;

/// Quantos processos o painel de recursos exibe. Fonte única: o `take(5)` do
/// render e o corte do monitor precisam concordar.
pub const TOP_PROCESSES: usize = 5;

/// Percentual de memória usada em `0.0..=100.0`, protegido contra divisão por
/// zero (`total == 0` => 0). Fonte única do cálculo, antes repetido em 3 lugares.
pub fn mem_percent(used: u64, total: u64) -> f32 {
    if total == 0 {
        return 0.0;
    }
    (used as f64 / total as f64 * 100.0) as f32
}

/// Monitor de recursos do sistema
pub struct SystemMonitor {
    system: System,
}

impl SystemMonitor {
    /// Cria uma nova instância do monitor
    pub fn new() -> Self {
        Self {
            system: System::new_all(),
        }
    }
    
    /// Atualiza as informações do sistema
    pub fn refresh(&mut self) {
        self.system.refresh_all();
    }

    /// Retorna o uso médio de CPU (0-100) a partir do estado já atualizado.
    ///
    /// **Não** dispara `refresh_cpu` por conta própria: quem chama é responsável
    /// por atualizar antes (ver [`Self::get_system_summary`]). Refrescar aqui
    /// duplicava a varredura de CPU a cada tick e ainda encurtava o intervalo de
    /// delta que o `sysinfo` usa para calcular a porcentagem.
    pub fn cpu_usage(&self) -> f32 {
        let cpus = self.system.cpus();
        if cpus.is_empty() {
            return 0.0;
        }

        let total: f32 = cpus.iter().map(|cpu| cpu.cpu_usage()).sum();
        total / cpus.len() as f32
    }
    
    /// Retorna memória usada e total (em bytes)
    pub fn get_memory_info(&self) -> (u64, u64) {
        let used = self.system.used_memory();
        let total = self.system.total_memory();
        (used, total)
    }
    
    /// Retorna o número de processos em execução
    pub fn get_process_count(&self) -> usize {
        self.system.processes().len()
    }
    
    /// Retorna os [`TOP_PROCESSES`] processos que mais usam CPU, a partir do
    /// estado já atualizado.
    ///
    /// Usa seleção parcial (`select_nth_unstable_by`, O(n)) em vez de ordenar a
    /// lista inteira (O(n log n)) só para ficar com os primeiros. A comparação é
    /// `total_cmp`: `partial_cmp().unwrap()` daria panic caso o `sysinfo`
    /// devolvesse `NaN`, e isso rodaria dentro do caminho de render.
    pub fn top_processes(&self) -> Vec<ProcessInfo> {
        let mut processes: Vec<_> = self
            .system
            .processes()
            .iter()
            .map(|(pid, process)| ProcessInfo {
                pid: pid.to_string(),
                name: process.name().to_string(),
                cpu_usage: process.cpu_usage(),
                memory: process.memory(),
            })
            .collect();

        if processes.len() > TOP_PROCESSES {
            processes.select_nth_unstable_by(TOP_PROCESSES, |a, b| {
                b.cpu_usage.total_cmp(&a.cpu_usage)
            });
            processes.truncate(TOP_PROCESSES);
        }
        // Só os poucos sobreviventes precisam ficar em ordem.
        processes.sort_unstable_by(|a, b| b.cpu_usage.total_cmp(&a.cpu_usage));

        processes
    }

    /// Retorna informações formatadas do sistema.
    ///
    /// Um único `refresh_all()` alimenta todos os campos; antes, `get_cpu_usage`
    /// e `get_top_processes` refresvavam de novo por conta própria, varrendo CPU
    /// e a tabela de processos duas vezes a cada tick.
    pub fn get_system_summary(&mut self) -> SystemSummary {
        self.refresh();

        let (mem_used, mem_total) = self.get_memory_info();

        SystemSummary {
            cpu_usage: self.cpu_usage(),
            memory_used: mem_used,
            memory_total: mem_total,
            memory_percent: mem_percent(mem_used, mem_total),
            process_count: self.get_process_count(),
            top_processes: self.top_processes(),
        }
    }
}

impl Default for SystemMonitor {
    fn default() -> Self {
        Self::new()
    }
}

/// Informações de um processo
#[derive(Debug, Clone)]
pub struct ProcessInfo {
    pub pid: String,
    pub name: String,
    pub cpu_usage: f32,
    pub memory: u64,
}

/// Resumo do sistema
#[derive(Debug, Clone, Default)]
pub struct SystemSummary {
    pub cpu_usage: f32,
    pub memory_used: u64,
    pub memory_total: u64,
    pub memory_percent: f32,
    pub process_count: usize,
    pub top_processes: Vec<ProcessInfo>,
}
