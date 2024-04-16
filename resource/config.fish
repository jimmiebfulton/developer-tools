fish_add_path /opt/homebrew/bin
fish_add_path ~/bin
fish_add_path ~/.cargo/bin

source ~/.config/fish/local.fish

# Commands to run in interactive sessions can go here
if status is-interactive
    d init fish | source

    if test -f ~/.cargo/bin/archetect
        archetect completions fish | source
    end

    if test -f ~/.cargo/bin/starship
        starship init fish | source
    end

    if test -f ~/.cargo/bin/zoxide
        zoxide init fish | source
    end

    set -gx EDITOR nvim
end