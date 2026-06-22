// Author: Munique Alves Pacheco Feitoza
// License: GPLv3

use sysinfo::System;

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
    
    /// Retorna o uso de CPU (0-100)
    pub fn get_cpu_usage(&mut self) -> f32 {
        self.system.refresh_cpu();
        
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
    
    /// Retorna os top 5 processos por uso de CPU
    pub fn get_top_processes(&mut self) -> Vec<ProcessInfo> {
        self.system.refresh_processes();
        
        let mut processes: Vec<_> = self.system.processes()
            .iter()
            .map(|(pid, process)| ProcessInfo {
                pid: pid.to_string(),
                name: process.name().to_string(),
                cpu_usage: process.cpu_usage(),
                memory: process.memory(),
            })
            .collect();
        
        processes.sort_by(|a, b| b.cpu_usage.partial_cmp(&a.cpu_usage).unwrap());
        processes.truncate(5);
        
        processes
    }
    
    /// Retorna informações formatadas do sistema
    pub fn get_system_summary(&mut self) -> SystemSummary {
        self.refresh();

        let (mem_used, mem_total) = self.get_memory_info();

        SystemSummary {
            cpu_usage: self.get_cpu_usage(),
            memory_used: mem_used,
            memory_total: mem_total,
            memory_percent: mem_percent(mem_used, mem_total),
            process_count: self.get_process_count(),
            top_processes: self.get_top_processes(),
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
