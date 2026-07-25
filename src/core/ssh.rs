// Author: Munique Alves Pacheco Feitoza
// License: GPLv3

use anyhow::{Context, Result};
use ssh2::{CheckResult, KnownHostFileKind, Session};
use std::io::Read;
use std::net::TcpStream;
use std::path::{Path, PathBuf};

/// Gerencia uma sessão SSH ativa
pub struct SshSession {
    session: Session,
    _tcp: TcpStream, // Mantém a conexão TCP viva
    pub host: String,
    pub user: String,
    pub remote_cwd: String,
}

/// Faz shell-quoting POSIX de um valor que será interpolado num comando remoto.
///
/// Envolve em aspas simples e escapa aspas simples internas (`'` -> `'\''`),
/// neutralizando injeção via `remote_cwd` (vem do `pwd` do servidor) e via
/// caminhos informados pelo usuário. O comando do próprio usuário NÃO é citado:
/// ele é o shell remoto e precisa preservar pipes/redirecionamentos.
fn shell_quote(value: &str) -> String {
    format!("'{}'", value.replace('\'', "'\\''"))
}

/// Porta SSH usada pela conexão. Entra também na chave do `known_hosts`.
const SSH_PORT: u16 = 22;

/// Falhas de SSH que a interface precisa exibir para o usuário.
///
/// São erros **tipados** (e não strings) porque o `core` não deve escolher o
/// idioma da mensagem: ele relata o fato e a camada `app` escreve as palavras
/// via `.ftl`. Quem recebe um `anyhow::Error` desta camada recupera a variante
/// com `downcast_ref::<SshError>()`; ver `App::describe_ssh_error`.
///
/// O `Display` abaixo é só fallback para log — a UI nunca o usa.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SshError {
    /// Não foi possível abrir o socket TCP até o host.
    Connect { host: String, port: u16 },
    /// O handshake do protocolo SSH falhou.
    Handshake,
    /// A chave do host mudou desde a última conexão — possível ataque.
    HostKeyMismatch { host: String, known_hosts: String },
    /// Não foi possível decidir se o host é confiável.
    HostKeyUnverifiable { host: String },
    /// Nenhum método de autenticação disponível funcionou.
    AuthFailed,
    /// O `cd` remoto não encontrou o diretório.
    RemoteDirNotFound { path: String },
}

impl SshError {
    /// Chave Fluent da mensagem correspondente.
    /// Fonte única do mapeamento erro → texto.
    pub fn message_key(&self) -> &'static str {
        match self {
            SshError::Connect { .. } => "sys-ssh-err-connect",
            SshError::Handshake => "sys-ssh-err-handshake",
            SshError::HostKeyMismatch { .. } => "sys-ssh-hostkey-mismatch",
            SshError::HostKeyUnverifiable { .. } => "sys-ssh-hostkey-unverifiable",
            SshError::AuthFailed => "sys-ssh-err-auth",
            SshError::RemoteDirNotFound { .. } => "sys-ssh-err-nodir",
        }
    }
}

impl std::fmt::Display for SshError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SshError::Connect { host, port } => write!(f, "failed to connect to {host}:{port}"),
            SshError::Handshake => write!(f, "SSH handshake failed"),
            SshError::HostKeyMismatch { host, known_hosts } => write!(
                f,
                "host key for '{host}' changed — possible man-in-the-middle. \
                 If the change is legitimate, remove the old entry from {known_hosts}"
            ),
            SshError::HostKeyUnverifiable { host } => {
                write!(f, "could not verify the host key for '{host}'")
            }
            SshError::AuthFailed => write!(f, "SSH authentication failed"),
            SshError::RemoteDirNotFound { path } => {
                write!(f, "remote directory not found: {path}")
            }
        }
    }
}

impl std::error::Error for SshError {}



/// Caminho do `known_hosts` do usuário (`~/.ssh/known_hosts`).
fn known_hosts_path() -> Option<PathBuf> {
    std::env::var_os("HOME").map(|home| Path::new(&home).join(".ssh").join("known_hosts"))
}

/// Verifica a chave pública do servidor contra o `~/.ssh/known_hosts` **antes**
/// de qualquer tentativa de autenticação.
///
/// Sem esta checagem o handshake era aceito às cegas: um atacante no caminho
/// podia se passar pelo servidor e colher o que fosse enviado na sessão. A
/// política é a mesma do OpenSSH em modo padrão:
///
/// - **Match** — segue.
/// - **Mismatch** — aborta. A chave mudou; ou o servidor foi reinstalado, ou
///   alguém está no meio. Quem decide é a pessoa, editando o `known_hosts`.
/// - **NotFound** — primeira conexão (TOFU): registra a chave e segue.
/// - **Failure** — aborta; não dá para afirmar que o host é confiável.
fn verify_host_key(session: &Session, host: &str) -> Result<()> {
    let (key, key_type) = session
        .host_key()
        .ok_or_else(|| SshError::HostKeyUnverifiable { host: host.to_string() })?;
    // Copiado: o empréstimo de `session` acaba aqui e `known_hosts` precisa dele.
    let key = key.to_vec();

    let mut known = session
        .known_hosts()
        .map_err(|_| SshError::HostKeyUnverifiable { host: host.to_string() })?;

    let path = known_hosts_path();
    if let Some(path) = &path {
        // Arquivo ausente é normal na primeira conexão da vida; erro de leitura
        // (permissão, arquivo corrompido) NÃO pode virar "host desconhecido".
        if path.exists() {
            known
                .read_file(path, KnownHostFileKind::OpenSSH)
                .with_context(|| format!("falha ao ler {}", path.display()))?;
        }
    }

    match known.check_port(host, SSH_PORT, &key) {
        CheckResult::Match => Ok(()),
        CheckResult::Mismatch => Err(SshError::HostKeyMismatch {
            host: host.to_string(),
            known_hosts: path
                .as_ref()
                .map(|p| p.display().to_string())
                .unwrap_or_else(|| "~/.ssh/known_hosts".into()),
        }
        .into()),
        CheckResult::NotFound => {
            // Primeira conexão: confia e registra (TOFU).
            known
                .add(host, &key, "munux", key_type.into())
                .context("falha ao registrar a chave do host")?;
            if let Some(path) = &path {
                if let Some(dir) = path.parent() {
                    let _ = std::fs::create_dir_all(dir);
                }
                known
                    .write_file(path, KnownHostFileKind::OpenSSH)
                    .with_context(|| format!("falha ao gravar {}", path.display()))?;
            }
            Ok(())
        }
        CheckResult::Failure => {
            Err(SshError::HostKeyUnverifiable { host: host.to_string() }.into())
        }
    }
}

impl SshSession {
    /// Conecta a um host SSH
    pub fn connect(user: &str, host: &str) -> Result<Self> {
        let tcp = TcpStream::connect(format!("{}:{}", host, SSH_PORT)).map_err(|_| {
            SshError::Connect { host: host.to_string(), port: SSH_PORT }
        })?;

        let mut session = Session::new().map_err(|_| SshError::Handshake)?;

        session.set_tcp_stream(tcp.try_clone()?);
        session.handshake().map_err(|_| SshError::Handshake)?;

        // Autenticidade do servidor ANTES de autenticar: nenhuma credencial (nem
        // a identidade do agente) é oferecida a um host que não confere.
        verify_host_key(&session, host)?;

        // Tenta autenticação na ordem: Agente -> Chave Pública -> Senha (não interativa aqui)
        // Nota: Para senha interativa precisaríamos de UI, por enquanto vamos focar em chaves
        // ou agente, que é o padrão para servidores como RunCloud.
        
        let mut authenticated = false;

        // 1. Tenta usar o ssh-agent
        if !authenticated {
            let mut agent = session.agent().map_err(|_| SshError::AuthFailed)?;
            if agent.connect().is_ok() {
                agent.list_identities().ok();
                for identity in agent.identities().unwrap_or(vec![]) {
                    if agent.userauth(user, &identity).is_ok() {
                        authenticated = true;
                        break;
                    }
                }
            }
        }

        // 2. Tenta chaves padrão (~/.ssh/id_rsa, etc)
        if !authenticated {
             // Tenta descobrir chaves automaticamente
             // Isso requer que o usuário tenha chaves em locais padrão sem passphrase ou configuradas
             if session.userauth_agent(user).is_ok() {
                 authenticated = true;
             }
        }
        
        // 3. Tenta chave específica se não conseguiu (fallback comum)
        if !authenticated {
            let home = std::env::var("HOME").unwrap_or_else(|_| "/".to_string());
            let id_rsa = Path::new(&home).join(".ssh/id_rsa");
            
            if id_rsa.exists()
                && session.userauth_pubkey_file(user, None, &id_rsa, None).is_ok()
            {
                authenticated = true;
            }
        }

        if !authenticated {
            // Se falhar tudo, retorna erro pedindo configuração de chaves
            // Em uma evolução futura, podemos pedir senha via UI
            return Err(SshError::AuthFailed.into());
        }

        // Obtém diretório inicial
        let mut channel = session.channel_session()?;
        channel.exec("pwd")?;
        let mut s = String::new();
        channel.read_to_string(&mut s)?;
        let remote_cwd = s.trim().to_string();

        Ok(Self {
            session,
            _tcp: tcp,
            host: host.to_string(),
            user: user.to_string(),
            remote_cwd,
        })
    }

    /// Executa um comando no servidor remoto
    pub fn execute(&mut self, command: &str) -> Result<(String, String, i32)> {
        let mut channel = self.session.channel_session()
            .context("Falha ao abrir canal SSH")?;
        
        // Executa no diretório atual remoto. `remote_cwd` é citado (vem do
        // servidor); `command` permanece cru por ser o shell do usuário.
        let cmd = format!("cd {} && {}", shell_quote(&self.remote_cwd), command);
        
        channel.exec(&cmd)?;
        
        let mut stdout = String::new();
        let mut stderr = String::new();
        
        channel.read_to_string(&mut stdout)?;
        channel.stderr().read_to_string(&mut stderr)?;
        
        channel.wait_close()?;
        let exit_code = channel.exit_status()?;
        
        Ok((stdout, stderr, exit_code))
    }

    /// Muda o diretório remoto
    pub fn change_dir(&mut self, path: &str) -> Result<()> {
        // Normaliza o path
        let new_path = if path.starts_with('/') {
            path.to_string()
        } else if path == ".." {
             // Simplificação: deixa o servidor resolver via cd
             format!("{}/..", self.remote_cwd)
        } else {
             format!("{}/{}", self.remote_cwd, path)
        };

        // Verifica se o diretório existe tentando dar cd (path citado contra injeção).
        let (stdout, _, code) = self.execute(&format!("cd {} && pwd", shell_quote(&new_path)))?;
        
        if code == 0 {
            self.remote_cwd = stdout.trim().to_string();
            Ok(())
        } else {
            Err(SshError::RemoteDirNotFound { path: path.to_string() }.into())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::i18n::{I18n, Language};

    /// Toda variante precisa de uma chave que resolva nos dois idiomas — senão
    /// uma falha de SSH apareceria para a pessoa como `[MISSING: ...]`.
    #[test]
    fn every_error_variant_has_a_translation() {
        let variants = [
            SshError::Connect { host: "example.com".into(), port: 22 },
            SshError::Handshake,
            SshError::HostKeyMismatch {
                host: "example.com".into(),
                known_hosts: "/home/x/.ssh/known_hosts".into(),
            },
            SshError::HostKeyUnverifiable { host: "example.com".into() },
            SshError::AuthFailed,
            SshError::RemoteDirNotFound { path: "/srv/app".into() },
        ];

        for lang in [Language::PtBr, Language::EnUs] {
            let i18n = I18n::new(lang);
            for variant in &variants {
                let text = i18n.tc(variant.message_key());
                assert!(
                    !text.starts_with("[MISSING"),
                    "{:?} sem tradução em {:?} (chave '{}')",
                    variant,
                    lang,
                    variant.message_key()
                );
            }
        }
    }

    /// O `shell_quote` precisa neutralizar aspas simples — é o que impede
    /// injeção via `remote_cwd` (que vem do servidor) e via caminhos digitados.
    ///
    /// O teste passa a string citada por um `sh` de verdade e confere que ela
    /// volta **idêntica**: é a única prova real de que o quoting funciona.
    /// Inspecionar o texto citado à mão engana — `''\''; rm -rf / #'` *parece*
    /// perigoso e é exatamente a forma correta.
    #[test]
    fn shell_quote_survives_a_real_shell() {
        let hostile = [
            "/srv/app",
            "it's",
            "'; rm -rf / #",
            "$(whoami)",
            "`id`",
            "a\nb",
            "caminho com espaço",
            r#"aspas "duplas" e 'simples'"#,
        ];

        for original in hostile {
            let quoted = shell_quote(original);
            let out = std::process::Command::new("sh")
                .arg("-c")
                .arg(format!("printf %s {}", quoted))
                .output()
                .expect("sh indisponível");

            assert!(out.status.success(), "sh falhou com {:?}", quoted);
            assert_eq!(
                String::from_utf8_lossy(&out.stdout),
                original,
                "o shell não devolveu o valor original de {:?} (citado: {})",
                original,
                quoted
            );
            assert!(out.stderr.is_empty(), "sh reclamou de {:?}", quoted);
        }
    }
}
