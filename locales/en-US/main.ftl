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
achievement_easter_egg_nuke-desc = Tried to delete everything...
achievement-easter_egg_train-name = Choocoo!
achievement-easter_egg_train-desc = Saw the steam locomotive
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
