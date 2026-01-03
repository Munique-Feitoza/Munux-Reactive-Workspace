// Author: Munique Alves Pacheco Feitoza
// License: GPLv3

/// Easter eggs do terminal
pub struct EasterEggs;

impl EasterEggs {
    /// Verifica se o comando é um easter egg e retorna a resposta
    pub fn check(command: &str) -> Option<String> {
        let cmd = command.trim().to_lowercase();
        
        // sl (trem quando erra ls)
        if cmd == "sl" {
            return Some(Self::train_animation());
        }
        
        // cowsay
        if cmd.contains("cowsay") {
            let message = cmd.replace("cowsay", "").trim().to_string();
            return Some(Self::cowsay(if message.is_empty() { 
                "Moo! Use: cowsay <mensagem>" 
            } else { 
                &message 
            }));
        }
        
        // fortune
        if cmd == "fortune" {
            return Some(Self::fortune());
        }
        
        // matrix
        if cmd == "matrix" || cmd == "hack" {
            return Some(Self::matrix_message());
        }
        
        // sudo su
        if cmd == "sudo su" {
            return Some(Self::sudo_su());
        }
        
        // sudo rm -rf /
        if cmd.contains("sudo rm -rf /") || cmd.contains("rm -rf /") {
            return Some(Self::nuke_warning());
        }
        
        // whoami
        if cmd == "whoami" {
            return Some(Self::whoami());
        }
        
        // hack the planet
        if cmd.contains("hack the planet") {
            return Some(Self::hack_the_planet());
        }
        
        // konami code
        if cmd == "↑↑↓↓←→←→ba" || cmd == "konami" {
            return Some(Self::konami_code());
        }
        
        None
    }
    
    fn train_animation() -> String {
        r#"
      ====        ________                ___________
  _D _|  |_______/        \__I_I_____===__|_________|
   |(_)---  |   H\________/ |   |        =|___ ___|      _________________
   /     |  |   H  |  |     |   |         ||_| |_||     _|                \_____
  |      |  |   H  |__--------------------| [___] |   =|                        |
  | ________|___H__/__|_____/[][]~\_______|       |   -|                        |
  |/ |   |-----------I_____I [][] []  D   |=======|____|________________________|_
__/ =| o |=-~~\  /~~\  /~~\  /~~\ ____Y___________|__|__________________________|_
 |/-=|___|=    ||    ||    ||    |_____/~\___/          |_D__D__D_|  |_D__D__D_|
  \_/      \_O=====O=====O=====O/      \_/               \_/   \_/    \_/   \_/

Ops! Você quis dizer 'ls'? 🚂
"#.to_string()
    }
    
    fn cowsay(message: &str) -> String {
        let border = "-".repeat(message.len() + 2);
        format!(r#"
 {}
< {} >
 {}
        \   ^__^
         \  (oo)\_______
            (__)\       )\/\
                ||----w |
                ||     ||
"#, border, message, border)
    }
    
    fn fortune() -> String {
        let fortunes = [
            "Talk is cheap. Show me the code. - Linus Torvalds",
            "In a world without walls and fences, who needs windows and gates?",
            "Linux is not in the 'public domain'. Linux is a cancer that attaches itself in an intellectual property sense to everything it touches. - Steve Ballmer",
            "I'm doing a (free) operating system (just a hobby, won't be big and professional like gnu) - Linus Torvalds, 1991",
            "Software is like sex: it's better when it's free. - Linus Torvalds",
            "Microsoft isn't evil, they just make really crappy operating systems. - Linus Torvalds",
            "Intelligence is the ability to avoid doing work, yet getting the work done. - Linus Torvalds",
            "I'd like to interject for a moment... - Richard Stallman (probably)",
            "Real programmers don't use tabs. Real programmers don't use spaces. Real programmers use cats walking on their keyboard.",
            "There are only two hard things in Computer Science: cache invalidation and naming things. - Phil Karlton",
        ];
        
        use std::time::{SystemTime, UNIX_EPOCH};
        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
        let index = (now % fortunes.len() as u64) as usize;
        
        format!("\n💭 {}\n", fortunes[index])
    }
    
    fn matrix_message() -> String {
        r#"
Wake up, Neo...
The Matrix has you...
Follow the white rabbit.

🐰 Knock, knock, Neo.

[ACCESSING MAINFRAME...]
[BYPASSING SECURITY...]
[DOWNLOADING DATA...]
█████████████████████ 100%

Welcome to the real world.
"#.to_string()
    }
    
    fn sudo_su() -> String {
        r#"
╔═══════════════════════════════════════╗
║                                       ║
║   Com grandes poderes vêm grandes     ║
║   responsabilidades.                  ║
║                                       ║
║   Você agora tem poder ROOT.          ║
║   Use com sabedoria.                  ║
║                                       ║
║              - Uncle Ben              ║
║                                       ║
╚═══════════════════════════════════════╝
"#.to_string()
    }
    
    fn nuke_warning() -> String {
        r#"
⚠️  ☢️  ⚠️  ALERTA NUCLEAR  ⚠️  ☢️  ⚠️

Você tentou deletar o UNIVERSO INTEIRO!

Por favor, não faça isso. Existem pessoas
(e gatos) que dependem deste sistema.

Este comando foi bloqueado para sua segurança
e a segurança do mundo digital.

Se você REALMENTE quer destruir tudo:
  1. Isso é uma má ideia
  2. Sério, não faça isso
  3. Vai destruir TUDO
  4. Não diga que não avisei

COMANDO BLOQUEADO! ❌
"#.to_string()
    }
    
    fn whoami() -> String {
        format!(r#"
Você é...

Um hacker? 👨‍💻
Um aprendiz? 🎓
Um curioso? 🤔
Um rebelde? 😎

Você é quem você escolhe ser.

No Munux, você é {} - E está no caminho
para se tornar uma LENDA! 🚀
"#, "um Terminal Master")
    }
    
    fn hack_the_planet() -> String {
        r#"
🌍 HACK THE PLANET! 🌍

"They're trashing our rights! Trashing!
Trashing! Trashing!"

Access granted to Gibson mainframe...
Downloading all files...
███████████████████████ 100%

ZERO COOL IS HERE!

Congratulations, you've been 1337 since 1995! 🎮
"#.to_string()
    }
    
    fn konami_code() -> String {
        r#"
🎮 KONAMI CODE ATIVADO! 🎮

┏━━━━━━━━━━━━━━━━━━━━━━━━━┓
┃   30 VIDAS DESBLOQUEADAS  ┃
┃   XP BOOST x2 ATIVADO     ┃
┃   GOD MODE: ON            ┃
┗━━━━━━━━━━━━━━━━━━━━━━━━━┛

Achievement desbloqueado:
"Old School Gamer" 🕹️

+500 XP
"#.to_string()
    }
}
