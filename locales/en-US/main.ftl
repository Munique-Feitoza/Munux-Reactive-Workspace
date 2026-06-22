# Munux TUI - English (US)
# Industry-standard Fluent localization

# UI Labels
ui-welcome-title = Welcome to Munux TUI
ui-stats-title = Statistics
ui-quests-title = Active Quests
ui-danger-zone-title = Danger Zone
ui-help-title = Help
ui-terminal-prompt = type command...
ui-total-commands = Total Commands
ui-successful-commands = Successful
ui-failed-commands = Failed
ui-success-rate = Success Rate
ui-current-streak = Current Streak
ui-integrity = Integrity
ui-achievements = Achievements
ui-last-unlocked = Last unlocked
ui-active-quests = Active Quests
ui-level = Level
ui-rank = Rank
ui-no-achievements = No achievements yet. Run commands to unlock!
ui-all-quests-done = All quests completed!
ui-new-quests-level = New quests will be unlocked when you level up.
ui-navigation = Navigation
ui-err-read-dir = Error reading directory
ui-empty-file = Empty file
ui-err-read-file = Error reading file (too large or no permission)
ui-processes = Processes
ui-memory = Memory
ui-examples = Examples
ui-tip = Tip
ui-warning = WARNING
ui-terminal-active = Active Terminal
ui-stats = Stats
ui-streak = Current Streak
ui-streak-commands = commands
ui-recent-achievements = Recent Achievements
ui-achievement-unlocked = ACHIEVEMENT UNLOCKED!
ui-achievement-reward = Reward: { $xp } XP
ui-terminal-title = Reactive Educational Terminal
ui-reactive-desc = The right panel changes as
ui-reactive-desc-2 = you type commands!
ui-start-commands = Commands to start

ui-esc-to-back = ESC to go back
ui-scroll-hint = Use Arrows to Scroll
ui-preview-title = Preview: { $filename }
ui-resource-title = Resource Monitor
ui-cpu-usage = CPU Usage
ui-level-up-title = Level Up!

# Game States
game-rank-novice = Novice
game-rank-apprentice = Apprentice
game-rank-hacker = Hacker
game-rank-elite = Elite
game-rank-legend = Legend

# Quests
quest-explorer-title = Explorer
quest-explorer-desc = List files in the current directory
quest-location-title = Location
quest-location-desc = See which directory you are in
quest-architect-title = Architect
quest-architect-desc = Create a folder named 'munux'
quest-reader-title = Reader
quest-reader-desc = Read the README file
quest-cleaner-title = Cleaner
quest-cleaner-desc = Remove a temporary file
quest-focus-title = Focus
quest-focus-desc = Run 10 commands

# Quest Progress
quest-progress-run = Run '{ $command }' { $current }/{ $count } times
quest-progress-created = ✓ { $item } '{ $name }' created
quest-progress-create = Create { $item } named '{ $name }'
quest-progress-navigated = ✓ Navigated to { $path }
quest-progress-navigate = Go to directory { $path }
quest-progress-read = ✓ Content of '{ $name }' read
quest-progress-read-action = Read file '{ $name }' using cat
quest-progress-deleted = ✓ File '{ $name }' deleted
quest-progress-delete-action = Delete file '{ $name }'
quest-progress-reach-level = Reach level { $level }
quest-progress-any-command = Execute any command: { $current }/{ $count }
quest-progress-grep = Use grep { $current }/{ $count } times
quest-progress-git-done = ✓ Git used
quest-progress-git-todo = Use any git command
quest-progress-ssh-done = ✓ SSH used
quest-progress-ssh-todo = Use the ssh command
quest-progress-pkg-done = ✓ Package manager used
quest-progress-pkg-todo = Use the package manager
quest-progress-pipe-done = ✓ Pipe (|) used
quest-progress-pipe-todo = Use a pipe (|) to link commands
quest-progress-symlink-done = ✓ Symlink created
quest-progress-symlink-todo = Create a symlink with ln -s
quest-progress-editor-done = ✓ { $editor } used
quest-progress-editor-todo = Use the { $editor } editor
quest-progress-systemctl-done = ✓ systemctl used
quest-progress-systemctl-todo = Use the systemctl command
quest-progress-script-done = ✓ Script executed
quest-progress-script-todo = Write and execute a .sh script

sys-destructive-detected = Potentially destructive command detected!
sys-leveled-up = CONGRATULATIONS! You reached level { $level }!
sys-integrity-warning = INTEGRITY CRITICALLY LOW!
sys-danger-detected = DESTRUCTIVE COMMAND DETECTED
sys-consequences = POSSIBLE CONSEQUENCES
sys-access-denied-title = ⚠️ Access Denied
sys-access-denied-body = Your current level does not allow this command. Level up to unlock the system!
sys-access-denied = ✗ Access Denied: Safe Mode Active! This command is restricted at low levels.
sys-level-up-msg = Congratulations! You reached level { $level }!
sys-welcome-body = A reactive and gamified terminal to learn Linux commands.

game-msg-level-1 = Welcome to the terminal!
game-msg-level-2 = You are learning!
game-msg-level-5 = Safe mode DISABLED!
game-msg-level-6 = Dangerous commands unlocked.
game-msg-level-10 = You are a hacker now!
game-msg-level-11 = The system is your playground.
game-msg-level-20 = Entering Cyberpunk...
game-msg-level-21 = You dominate the terminal.
game-msg-level-30 = Elite mode ACTIVATED!
game-msg-level-31 = Few make it here...
game-msg-level-50 = YOU ARE A LEGEND!
game-msg-level-default = All your base are belong to us.

game-art-terminal-tag = [TERMINAL MODE]
game-art-hacker-tag = [HACKER MODE]
game-art-cyberpunk-tag = [CYBERPUNK]
game-art-elite-tag = [ELITE HACKER]
game-art-legend-tag = [LEGEND]

# Achievements
achievement-first_command-name = First Contact
achievement-first_command-desc = Run your first command
achievement-first_ls-name = Eye of the Tiger
achievement-first_ls-desc = Use 'ls' for the first time
achievement-first_cd-name = Traveler
achievement-first_cd-desc = Move between directories
achievement-first_file-name = Creator
achievement-first_file-desc = Create a file with 'touch'
achievement-first_dir-name = Builder
achievement-first_dir-desc = Create a directory with 'mkdir'
achievement-first_cat-name = Reader
achievement-first_cat-desc = View file content with 'cat'
achievement-first_rm-name = Eraser
achievement-first_rm-desc = Delete a file with 'rm'
achievement-first_sudo-name = Root Access
achievement-first_sudo-desc = Use 'sudo' for the first time
achievement-first_pacman-name = Packager
achievement-first_pacman-desc = Use the package manager
achievement-first_git-name = Version Controller
achievement-first_git-desc = Use any 'git' command
achievement-first_systemctl-name = System Admin
achievement-first_systemctl-desc = Manage services with 'systemctl'
achievement-first_ssh-name = Remote Access
achievement-first_ssh-desc = Use 'ssh' to connect
achievement-commands_10-name = Dedicated
achievement-commands_10-desc = Run 10 commands
achievement-commands_50-name = Professional
achievement-commands_50-desc = Run 50 commands
achievement-commands_100-name = Master
achievement-commands_100-desc = Run 100 commands
achievement-commands_500-name = Legend
achievement-commands_500-desc = Run 500 commands
achievement-pipe_master-name = Pipe Master
achievement-pipe_master-desc = Connect commands with '|'
achievement-streak_5-name = On Fire
achievement-streak_5-desc = Run 5 commands without errors
achievement-streak_10-name = Untouchable
achievement-streak_10-desc = Run 10 commands without errors
achievement-streak_25-name = God Mode
achievement-streak_25-desc = Run 25 commands without errors
achievement-level_5-name = Apprentice
achievement-level_5-desc = Reach level 5
achievement-level_10-name = Hacker
achievement-level_10-desc = Reach level 10
achievement-level_20-name = Pro Hacker
achievement-level_20-desc = Reach level 20
achievement-level_30-name = Cyber Elite
achievement-level_30-desc = Reach level 30
achievement-level_50-name = Transcendent
achievement-level_50-desc = Reach level 50
achievement-easter_egg_nuke-name = System Breaker
achievement-easter_egg_nuke-desc = Tried to delete everything...
achievement-easter_egg_train-name = Choocoo!
achievement-easter_egg_train-desc = Saw the steam locomotive
achievement-easter_egg_cow-name = Talking Cow
achievement-easter_egg_cow-desc = Made the cow talk with 'cowsay'
achievement-easter_egg_matrix-name = The One
achievement-easter_egg_matrix-desc = Followed the white rabbit with 'matrix'
achievement-easter_egg_konami-name = Old School Gamer
achievement-easter_egg_konami-desc = Entered the Konami Code
achievement-easter_egg_sandwich-name = Make It Yourself
achievement-easter_egg_sandwich-desc = "sudo make me a sandwich" (xkcd #149)
achievement-easter_egg_42-name = Deep Thought
achievement-easter_egg_42-desc = Found the answer to everything
achievement-easter_egg_xyzzy-name = Adventurer
achievement-easter_egg_xyzzy-desc = Said the magic word 'xyzzy'
achievement-easter_egg_cake-name = The Cake Is a Lie
achievement-easter_egg_cake-desc = Found the Portal reference
achievement-easter_egg_vim-name = :wq
achievement-easter_egg_vim-desc = The reflex of someone who got stuck in Vim
achievement-easter_egg_starwars-name = May the Force Be with You
achievement-easter_egg_starwars-desc = Summoned Star Wars in the terminal
achievement-easter_egg_hunter-name = Easter Egg Hunter
achievement-easter_egg_hunter-desc = Found 5 secret easter eggs
sys-xp-gain = ✓ +{ $amount } XP | { $current }/{ $total } to level { $next }
sys-xp-usage = ✗ Usage: xp <amount>
ui-err-is-dir-hint = 💡 Use 'ls { $name }' to list contents.

# Command Help
help-ls-desc = List files and directories in the current folder.
help-ls-hint = 💡 'ls' stands for 'list'. Use 'ls -R' to list subfolders recursively!
help-cd-desc = Navigate between directories (Change Directory).
help-cd-hint = 💡 Use the TAB key to autocomplete folder names!
help-grep-desc = Search for text within files or outputs.
help-grep-hint = 💡 Global Regular Expression Print. Powerful tool for filtering logs!
help-cat-desc = Show the content of a file on the screen.
help-cat-hint = 💡 Be careful with binary files! Use 'less' for large files.
help-sudo-desc = Run commands with superuser (Root) privileges.
help-sudo-hint = ⚠️ With great power comes great responsibility. Use with care!

    Use normal keys to type.
    Press Enter to execute.
    Press Ctrl+C to exit.

# Command Hints (Welcome Screen)
hint-ls = List directory contents
hint-pwd = Print working directory
hint-mkdir = Create new directory
hint-cat = Show file content
hint-rm = Remove file
hint-cp = Copy file
hint-mv = Move or rename file
hint-ssh = Remote login
hint-grep = Search text
hint-systemctl = Manage services

# Danger Zone and confirmation
ui-attention-max = MAXIMUM ALERT
ui-command-detected = Command detected
ui-risk = Risk
ui-data-loss = Irreversible data loss
ui-unstable-system = System may become unstable
ui-irreversible-damage = Damage that cannot be undone
ui-available-actions = Available actions
ui-cancel-rec = Cancel (recommended)
ui-execute-anyway = Execute anyway
ui-backup-tip = Back up before running destructive commands.
sys-danger-confirm = ⚠️ Dangerous command. Type 'yes' and Enter to confirm, or ESC to cancel.
sys-danger-cancelled = ✓ Dangerous command cancelled.

# Rank progression
ui-next-rank = 🎯 Next rank: { $rank } (level { $level })
ui-max-rank = 🏆 Max rank reached!

# Alias
sys-alias-none = No aliases defined. Use: alias name='command'
sys-alias-list-title = 📎 Defined aliases:
sys-alias-removed = ✓ Alias '{ $name }' removed.
sys-alias-missing = ✗ Alias '{ $name }' does not exist.
sys-alias-usage = Usage: alias name='command'
sys-alias-no-spaces = ✗ Alias name cannot contain spaces.
sys-alias-created = ✓ Alias created: { $name } = { $value }

# Tutorial
sys-tutorial-ended = 🎓 Tutorial closed. Come back anytime: 'tutorial'.
sys-tutorial-none = No tutorial in progress.
sys-tutorial-started = 🎓 Tutorial started! Follow the instructions on the panel.
sys-tutorial-mode-title = 🎓 Tutorial Mode
sys-tutorial-step-done-title = ✅ Step complete!
sys-tutorial-complete-title = 🎉 Tutorial complete!
sys-tutorial-complete-body = Congratulations! You mastered the Munux basics.{ "" }
    { "" }
    +{ $xp } bonus XP!{ "" }
    { "" }
    Now explore freely — use 'help' whenever you need.

# Benchmark
sys-bench-none = No benchmark in progress.
sys-bench-cancelled = ⏱️ Benchmark cancelled.
sys-bench-result-title = ⏱️ Benchmark Result
sys-bench-popup-title = ⏱️ Typing Benchmark
sys-bench-result = ⏱️ { $seconds }s  •  { $wpm } WPM  •  { $accuracy }% accuracy  •  +{ $xp } XP
sys-bench-start = ⏱️ TYPING BENCHMARK{ "" }
    { "" }
    Type the phrase below and press Enter:{ "" }
    { "" }
      { $phrase }{ "" }
    { "" }
    ('benchmark exit' cancels)
sys-bench-popup-body = Type this phrase exactly and press Enter:{ "" }
    { "" }
    { $phrase }{ "" }
    { "" }
    The timer has already started! ('benchmark exit' to cancel)

# SSH
sys-error = ✗ Error: { $msg }
sys-ssh-disconnected = 🔌 Disconnected from remote server.
sys-ssh-cd-ok = ✓ Remote directory changed to: { $dir }
sys-ssh-exec-error = ✗ Remote execution error: { $msg }
sys-ssh-connecting = 🔄 Connecting to { $user }@{ $host }...
sys-ssh-connected = ✓ Connected to { $host } at { $dir }
sys-ssh-conn-title = Connection Established
sys-ssh-conn-body = Successfully connected to { $user }@{ $host }{ "" }
    { "" }
    Directory: { $dir }
sys-ssh-fail = ✗ Connection failed: { $msg }
sys-ssh-fail-title = Connection Error
sys-ssh-fail-body = Could not connect to { $target }:{ "" }
    { $msg }

# Special commands / help
sys-showing-stats = ✓ Showing statistics
sys-showing-quests = ✓ Showing active quests
sys-tip-title = 💡 Tip of the Day
sys-tip-body = Use the 'help' command to list all available commands.{ "" }
    { "" }
    Try 'stats' to see your progress!
sys-tip-showing = Showing tip...
sys-help-cmd = 📚 Command help: { $topic }
sys-help-showing-title = 📚 Showing: { $title } (Press ESC to go back)
sys-help-showing = 📚 Showing help (Press ESC to go back)
help-system-title = Munux Help System
help-system-body =
    📚 MUNUX HELP SYSTEM

    Use: help <distro>

    Supported distributions:
      help arch     - Manjaro, Arch Linux (pacman, yay, paru)
      help debian   - Ubuntu, Debian, Mint (apt, dpkg, snap)
      help fedora   - Fedora, RHEL, CentOS (dnf, rpm)
      help opensuse - openSUSE (zypper)
      help linux    - Universal Linux commands

    Munux special commands:
      stats         → Stats and progress
      quests        → Active quests
      achievements  → Unlocked achievements
      tutorial      → Interactive tutorial for beginners
      benchmark     → Typing speed test
      alias n='cmd' → Create a command shortcut (unalias n removes)

    Press ESC to return to normal mode.

# Command execution (shell, cd, ls)
sys-cd-ok = ✓ Directory changed to: { $dir }
sys-cd-notfound = Directory not found: { $path }
sys-ls-listed = 📂 Files listed in the right panel →
sys-cmd-ok = ✓ Command executed successfully
sys-cmd-error = ✗ Command execution error
sys-cmd-exec-error = ✗ Error running command: { $msg }
sys-quest-complete = { "" }
    📋 QUEST COMPLETE!{ "" }
    { $title }{ "" }
    +{ $xp } XP

# Level up / achievements
sys-levelup-title = 🎉 LEVEL UP!
sys-levelup-body = Level { $old } → { $new }{ "" }
    { "" }
    { $rank }{ "" }
    { "" }
    { $msg }
sys-achievement-title = 🏆 Achievement Unlocked!
sys-achievement-announce = 🏆 ACHIEVEMENT UNLOCKED!{ "" }
    { "" }
    { $name }{ "" }
    { $desc }{ "" }
    { "" }
    +{ $xp } XP

# Educational hints (text only; shell-output matchers stay in code)
hint-err-rm-isdir = { "" }
    { "" }
    💡 TIP: 'rm' removes FILES.{ "" }
       To remove directories use:{ "" }
       - 'rmdir name'     (empty directory){ "" }
       - 'rm -r name'     (directory with contents){ "" }
       - 'rm -rf name'    (force removal - CAREFUL!)
hint-err-rmdir-notempty = { "" }
    { "" }
    💡 TIP: 'rmdir' only removes EMPTY directories.{ "" }
       To remove with contents use: 'rm -r name'
hint-err-cat-isdir = { "" }
    { "" }
    💡 TIP: 'cat' shows the contents of FILES.{ "" }
       To list directories use: 'ls name'
hint-err-cd-notdir = { "" }
    { "" }
    💡 TIP: 'cd' navigates into DIRECTORIES.{ "" }
       To open files use: 'cat name' or 'nano name'
hint-err-mkdir-dots = { "" }
    { "" }
    💡 TIP: 'mkdir' creates DIRECTORIES (folders).{ "" }
       To create files use:{ "" }
       - 'touch file.txt'             (empty file){ "" }
       - 'echo "text" > file.txt'     (file with content)
hint-err-permission = { "" }
    { "" }
    💡 TIP: You don't have permission.{ "" }
       Try 'sudo' before the command (careful!)
hint-err-notfound = { "" }
    { "" }
    💡 TIP: Command does not exist or is not installed.{ "" }
       - Check that you typed it correctly{ "" }
       - Use 'which command' to verify it exists

# Danger zone warnings (shown in the DangerZone panel)
danger-rm-root = RECURSIVE DELETE ON ROOT DIRECTORY!
danger-rm-rf = Recursive, forced file deletion
danger-rm = File deletion - irreversible operation
danger-sudo = Execution with superuser privileges
danger-dd = Low-level copy - may overwrite data
danger-fs = Partition/filesystem modification
danger-perm = File permission/ownership modification
danger-power = System shutdown/restart
danger-generic = Potentially destructive command detected

# Misc
ui-top-processes = Top processes (CPU)
ui-browse-hint = ↑↓ navigate • Enter open
ui-back-to-normal = Back to normal mode
sys-file-not-found = ❌ File '{ $name }' not found
sys-files-found = 💡 Files found:
