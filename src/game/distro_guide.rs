// Author: Munique Alves Pacheco Feitoza
// License: GPLv3

/// Guia de comandos por distribuição Linux
pub struct DistroGuide;

impl DistroGuide {
    /// Retorna um guia completo de comandos para a distro
    pub fn get_guide(distro: &str) -> String {
        match distro.to_lowercase().as_str() {
            "manjaro" | "arch" => Self::arch_guide(),
            "ubuntu" | "debian" | "mint" => Self::debian_guide(),
            "fedora" | "rhel" | "centos" => Self::fedora_guide(),
            "opensuse" => Self::opensuse_guide(),
            "all" | "geral" => Self::general_guide(),
            _ => Self::general_guide(),
        }
    }
    
    fn arch_guide() -> String {
        r#"
╔═══════════════════════════════════════════════════════════╗
║          GUIA MANJARO/ARCH LINUX - PACMAN                 ║
╚═══════════════════════════════════════════════════════════╝

📦 GERENCIAMENTO DE PACOTES (PACMAN)
────────────────────────────────────────────────────────────
  sudo pacman -S <pacote>       # Instala pacote
  sudo pacman -R <pacote>       # Remove pacote
  sudo pacman -Rs <pacote>      # Remove + dependências
  sudo pacman -Syu              # Atualiza sistema completo
  sudo pacman -Ss <nome>        # Busca pacote
  sudo pacman -Qi <pacote>      # Info do pacote instalado
  sudo pacman -Sc               # Limpa cache

📦 AUR HELPER (YAY/PARU)
────────────────────────────────────────────────────────────
  yay <pacote>                  # Busca e instala (AUR)
  yay -S <pacote>               # Instala do AUR
  yay -Syu                      # Atualiza tudo (oficial + AUR)
  yay -Ps                       # Estatísticas do sistema
  paru <pacote>                 # Alternativa ao yay

🔧 PAMAC (GUI/CLI)
────────────────────────────────────────────────────────────
  pamac install <pacote>        # Instala
  pamac remove <pacote>         # Remove
  pamac update                  # Atualiza
  pamac search <nome>           # Busca

💡 DICAS MANJARO
────────────────────────────────────────────────────────────
  • Sempre rode 'sudo pacman -Syu' antes de instalar algo
  • AUR não é oficial - use com cuidado
  • Base-devel é necessário para compilar do AUR
  • Kernel Manager: sudo mhwd-kernel -li
"#.to_string()
    }
    
    fn debian_guide() -> String {
        r#"
╔═══════════════════════════════════════════════════════════╗
║        GUIA UBUNTU/DEBIAN - APT                           ║
╚═══════════════════════════════════════════════════════════╝

📦 GERENCIAMENTO DE PACOTES (APT)
────────────────────────────────────────────────────────────
  sudo apt update               # Atualiza lista de pacotes
  sudo apt upgrade              # Atualiza pacotes instalados
  sudo apt full-upgrade         # Atualiza + remove obsoletos
  sudo apt install <pacote>     # Instala pacote
  sudo apt remove <pacote>      # Remove pacote
  sudo apt purge <pacote>       # Remove + configs
  sudo apt autoremove           # Remove dependências órfãs
  sudo apt search <nome>        # Busca pacote
  sudo apt show <pacote>        # Mostra info do pacote
  sudo apt clean                # Limpa cache

📦 REPOSITÓRIOS (PPA)
────────────────────────────────────────────────────────────
  sudo add-apt-repository ppa:user/ppa  # Adiciona PPA
  sudo add-apt-repository --remove ppa:user/ppa  # Remove PPA

📦 DPKG (Pacotes .deb)
────────────────────────────────────────────────────────────
  sudo dpkg -i pacote.deb       # Instala .deb
  sudo dpkg -r <pacote>         # Remove pacote
  dpkg -l                       # Lista instalados
  sudo apt --fix-broken install # Corrige dependências

📦 SNAP
────────────────────────────────────────────────────────────
  sudo snap install <pacote>    # Instala snap
  sudo snap remove <pacote>     # Remove snap
  snap find <nome>              # Busca snap
  snap refresh                  # Atualiza todos

💡 DICAS DEBIAN/UBUNTU
────────────────────────────────────────────────────────────
  • Sempre 'sudo apt update' antes de 'apt install'
  • use 'apt' em vez de 'apt-get' (mais moderno)
  • PPAs podem causar conflitos - cuidado!
  • LTS = Long Term Support (5 anos de atualizações)
"#.to_string()
    }
    
    fn fedora_guide() -> String {
        r#"
╔═══════════════════════════════════════════════════════════╗
║         GUIA FEDORA/RHEL - DNF/YUM                        ║
╚═══════════════════════════════════════════════════════════╝

📦 GERENCIAMENTO DE PACOTES (DNF)
────────────────────────────────────────────────────────────
  sudo dnf install <pacote>     # Instala pacote
  sudo dnf remove <pacote>      # Remove pacote
  sudo dnf upgrade              # Atualiza tudo
  sudo dnf search <nome>        # Busca pacote
  sudo dnf info <pacote>        # Info do pacote
  sudo dnf autoremove           # Remove órfãos
  sudo dnf clean all            # Limpa cache

📦 REPOSITÓRIOS
────────────────────────────────────────────────────────────
  sudo dnf config-manager --add-repo <url>  # Adiciona repo
  sudo dnf repolist             # Lista repos

📦 RPM (Pacotes .rpm)
────────────────────────────────────────────────────────────
  sudo rpm -i pacote.rpm        # Instala .rpm
  sudo rpm -e <pacote>          # Remove
  rpm -qa                       # Lista instalados
  rpm -qi <pacote>              # Info do pacote

💡 DICAS FEDORA
────────────────────────────────────────────────────────────
  • DNF é sucessor do YUM (mais rápido)
  • RPM Fusion para codecs/drivers proprietários
  • Versão nova a cada 6 meses
  • COPR = PPAs do Fedora
"#.to_string()
    }
    
    fn opensuse_guide() -> String {
        r#"
╔═══════════════════════════════════════════════════════════╗
║            GUIA OPENSUSE - ZYPPER                         ║
╚═══════════════════════════════════════════════════════════╝

📦 GERENCIAMENTO DE PACOTES (ZYPPER)
────────────────────────────────────────────────────────────
  sudo zypper install <pacote>  # Instala (in/i)
  sudo zypper remove <pacote>   # Remove (rm)
  sudo zypper update            # Atualiza (up)
  sudo zypper search <nome>     # Busca (se)
  sudo zypper info <pacote>     # Info (if)
  sudo zypper refresh           # Atualiza repos (ref)
  sudo zypper dist-upgrade      # Upgrade completo (dup)

📦 REPOSITÓRIOS
────────────────────────────────────────────────────────────
  sudo zypper addrepo <url> <nome>  # Adiciona repo
  sudo zypper repos             # Lista repos

💡 DICAS OPENSUSE
────────────────────────────────────────────────────────────
  • YaST = ferramenta de configuração central
  • Tumbleweed = rolling release
  • Leap = release estável
  • Btrfs + snapshots nativos
"#.to_string()
    }
    
    fn general_guide() -> String {
        r#"
╔═══════════════════════════════════════════════════════════╗
║        COMANDOS LINUX UNIVERSAIS                          ║
╚═══════════════════════════════════════════════════════════╝

📁 NAVEGAÇÃO E ARQUIVOS
────────────────────────────────────────────────────────────
  ls                 # Lista arquivos
  ls -la             # Lista com detalhes + ocultos
  cd <pasta>         # Muda diretório
  pwd                # Mostra local atual
  mkdir <nome>       # Cria pasta
  touch <arquivo>    # Cria arquivo
  rm <arquivo>       # Remove arquivo
  rm -r <pasta>      # Remove pasta
  cp origem destino  # Copia
  mv origem destino  # Move/renomeia

📝 VISUALIZAÇÃO DE ARQUIVOS
────────────────────────────────────────────────────────────
  cat <arquivo>      # Mostra conteúdo
  less <arquivo>     # Visualiza (navegável)
  head <arquivo>     # Primeiras linhas
  tail <arquivo>     # Últimas linhas
  nano <arquivo>     # Editor simples
  vim <arquivo>      # Editor avançado

🌐 REDE
────────────────────────────────────────────────────────────
  ping <host>        # Testa conexão
  curl <url>         # Baixa/acessa URL
  wget <url>         # Baixa arquivo
  ssh user@host      # Acesso remoto
  ip addr            # Mostra IPs
  netstat -tunlp     # Portas abertas

⚙️ SISTEMA
────────────────────────────────────────────────────────────
  top / htop         # Monitor de processos
  ps aux             # Lista processos
  kill <PID>         # Mata processo
  free -h            # Memória
  df -h              # Espaço em disco
  uname -a           # Info do sistema
  systemctl status <serviço>  # Status de serviço

📦 COMPRESSÃO
────────────────────────────────────────────────────────────
  tar -czf arquivo.tar.gz pasta/  # Compacta
  tar -xzf arquivo.tar.gz         # Extrai
  zip -r arquivo.zip pasta/       # ZIP
  unzip arquivo.zip               # Descompacta

💡 Use 'help <distro>' para guias específicos:
   help manjaro | help ubuntu | help fedora | help opensuse
"#.to_string()
    }
}
