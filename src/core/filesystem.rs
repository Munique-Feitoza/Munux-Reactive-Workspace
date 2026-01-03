// Author: Munique Alves Pacheco Feitoza
// License: GPLv3

use anyhow::Result;
use std::fs;
use std::path::{Path, PathBuf};

/// Gerenciador de sistema de arquivos
pub struct FileSystemManager;

impl FileSystemManager {
    /// Lê o conteúdo de um arquivo (limitado a 1MB)
    pub fn read_file_preview(path: &Path) -> Result<String> {
        const MAX_SIZE: u64 = 1024 * 1024; // 1MB
        
        let metadata = fs::metadata(path)?;
        
        if metadata.len() > MAX_SIZE {
            return Ok(format!(
                "[Arquivo muito grande: {} bytes]\nApenas os primeiros bytes serão mostrados.\n\n{}",
                metadata.len(),
                String::from_utf8_lossy(&fs::read(path)?[..1024])
            ));
        }
        
        Ok(fs::read_to_string(path)?)
    }
    
    /// Lista arquivos e diretórios
    pub fn list_directory(path: &Path) -> Result<Vec<FileEntry>> {
        let mut entries = Vec::new();
        
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            let metadata = entry.metadata()?;
            let path = entry.path();
            let name = entry.file_name().to_string_lossy().to_string();
            
            entries.push(FileEntry {
                name,
                path,
                is_dir: metadata.is_dir(),
                is_symlink: metadata.is_symlink(),
                size: metadata.len(),
            });
        }
        
        // Ordena: diretórios primeiro, depois arquivos (alfabeticamente)
        entries.sort_by(|a, b| {
            if a.is_dir && !b.is_dir {
                std::cmp::Ordering::Less
            } else if !a.is_dir && b.is_dir {
                std::cmp::Ordering::Greater
            } else {
                a.name.to_lowercase().cmp(&b.name.to_lowercase())
            }
        });
        
        Ok(entries)
    }
    
    /// Verifica se um caminho existe e é seguro
    pub fn is_safe_path(path: &Path) -> bool {
        path.exists() && !path.to_string_lossy().contains("..")
    }
    
    /// Formata o tamanho do arquivo de forma legível
    pub fn format_size(bytes: u64) -> String {
        const UNITS: &[&str] = &["B", "KB", "MB", "GB", "TB"];
        let mut size = bytes as f64;
        let mut unit_index = 0;
        
        while size >= 1024.0 && unit_index < UNITS.len() - 1 {
            size /= 1024.0;
            unit_index += 1;
        }
        
        format!("{:.2} {}", size, UNITS[unit_index])
    }
}

/// Entrada de arquivo/diretório
#[derive(Debug, Clone)]
pub struct FileEntry {
    pub name: String,
    pub path: PathBuf,
    pub is_dir: bool,
    pub is_symlink: bool,
    pub size: u64,
}

impl FileEntry {
    /// Retorna o ícone baseado no tipo
    pub fn get_icon(&self) -> &str {
        if self.is_symlink {
            "🔗"
        } else if self.is_dir {
            "📁"
        } else if self.name.ends_with(".rs") {
            "🦀"
        } else if self.name.ends_with(".py") {
            "🐍"
        } else if self.name.ends_with(".js") || self.name.ends_with(".ts") {
            "📜"
        } else if self.name.ends_with(".sh") {
            "📜"
        } else if self.name.ends_with(".toml") || self.name.ends_with(".json") {
            "⚙️"
        } else if self.name.ends_with(".md") {
            "📝"
        } else if self.name.starts_with('.') {
            "👁️"
        } else {
            "📄"
        }
    }
}
