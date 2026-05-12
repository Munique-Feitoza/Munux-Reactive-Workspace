// Author: Munique Alves Pacheco Feitoza
// License: GPLv3

use anyhow::{Context, Result};
use std::io::Read;
use std::net::TcpStream;
use std::path::Path;
use ssh2::Session;

/// Gerencia uma sessão SSH ativa
pub struct SshSession {
    session: Session,
    _tcp: TcpStream, // Mantém a conexão TCP viva
    pub host: String,
    pub user: String,
    pub remote_cwd: String,
}

impl SshSession {
    /// Conecta a um host SSH
    pub fn connect(user: &str, host: &str) -> Result<Self> {
        let tcp = TcpStream::connect(format!("{}:22", host))
            .context(format!("Falha ao conectar em {}:22", host))?;
        
        let mut session = Session::new()
            .context("Falha ao criar sessão SSH")?;
        
        session.set_tcp_stream(tcp.try_clone()?);
        session.handshake().context("SSH handshake falhou")?;

        // Tenta autenticação na ordem: Agente -> Chave Pública -> Senha (não interativa aqui)
        // Nota: Para senha interativa precisaríamos de UI, por enquanto vamos focar em chaves
        // ou agente, que é o padrão para servidores como RunCloud.
        
        let mut authenticated = false;

        // 1. Tenta usar o ssh-agent
        if !authenticated {
            let mut agent = session.agent().context("Falha ao iniciar ssh-agent")?;
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
            anyhow::bail!("Falha na autenticação SSH. Por favor, configure suas chaves SSH ou use ssh-agent.");
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
        
        // Executa no diretório atual remoto
        let cmd = format!("cd {} && {}", self.remote_cwd, command);
        
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

        // Verifica se o diretório existe tentando dar cd
        let (stdout, _, code) = self.execute(&format!("cd {} && pwd", new_path))?;
        
        if code == 0 {
            self.remote_cwd = stdout.trim().to_string();
            Ok(())
        } else {
            anyhow::bail!("Diretório remoto não encontrado: {}", path)
        }
    }
}
