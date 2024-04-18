fish_add_path /opt/homebrew/bin
fish_add_path ~/bin
fish_add_path ~/.cargo/bin

# Commands to run in interactive sessions can go here
if status is-interactive
    fish_vi_key_bindings

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

    set -gx CDPATH . ~/projects ~/projects/rust ~/projects/archetypes ~/tmp
end

source ~/.config/fish/local.fish

