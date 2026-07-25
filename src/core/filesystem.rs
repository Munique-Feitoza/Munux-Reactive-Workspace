// Author: Munique Alves Pacheco Feitoza
// License: GPLv3

use anyhow::Result;
use std::fs;
use std::path::{Path, PathBuf};

/// Tamanho máximo lido inteiro para preview (1 MB).
const MAX_PREVIEW_SIZE: u64 = 1024 * 1024;

/// Quanto se lê do começo de um arquivo grande demais.
const PREVIEW_HEAD_BYTES: usize = 1024;

/// Conteúdo lido para preview, com o aviso de corte separado do texto.
///
/// A mensagem "arquivo grande demais" era concatenada aqui, em português fixo,
/// dentro de um módulo `core` que não deve conhecer idioma. Agora o `core`
/// informa **o fato** e a UI decide como dizê-lo.
#[derive(Debug, Clone, Default)]
pub struct FilePreview {
    /// O texto lido (completo ou só o começo).
    pub content: String,
    /// Tamanho total em bytes quando o arquivo foi cortado; `None` se inteiro.
    pub truncated_at: Option<u64>,
}

/// Gerenciador de sistema de arquivos
pub struct FileSystemManager;

impl FileSystemManager {
    /// Lê o conteúdo de um arquivo para preview.
    ///
    /// Acima de [`MAX_PREVIEW_SIZE`] devolve só o começo, sinalizado em
    /// [`FilePreview::truncated_at`] — a mensagem para o usuário é montada e
    /// traduzida na camada de UI, que é quem conhece o `i18n`.
    pub fn read_file_preview(path: &Path) -> Result<FilePreview> {
        let metadata = fs::metadata(path)?;

        if metadata.len() > MAX_PREVIEW_SIZE {
            // Lê só o cabeçalho em vez do arquivo inteiro: antes o `fs::read`
            // carregava o arquivo completo na memória só para fatiar 1 KB dele.
            let mut head = vec![0u8; PREVIEW_HEAD_BYTES];
            let mut file = fs::File::open(path)?;
            let read = std::io::Read::read(&mut file, &mut head)?;
            head.truncate(read);

            return Ok(FilePreview {
                content: String::from_utf8_lossy(&head).into_owned(),
                truncated_at: Some(metadata.len()),
            });
        }

        Ok(FilePreview { content: fs::read_to_string(path)?, truncated_at: None })
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
            return "🔗";
        }
        if self.is_dir {
            return "📁";
        }
        crate::core::filetype::classify(&self.name).icon
    }
}
