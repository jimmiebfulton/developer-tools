function rd.
    set directory (basename (pwd))
    cd ..
    d rd $directory; or cd $directory
end

function cy
    z (echo (yazi --cwd-file /dev/stdout))
end

function rd
    d rd $argv
end

function r.
    rustrover .
end

function i.
    idea .
end

function pc.
    pycharm .
end

function mcd
    mkdir -p $argv
    cd $argv
end

function reload
    source ~/.config/fish/config.fish
end

function pbc
    pbcopy < $argv
end

function fish_mode_prompt
    switch $fish_bind_mode
        case default
            set_color --bold red
            echo N
        case insert
            set_color --bold green
            echo I
        case replace_one
            set_color --bold green
            echo R
        case visual
            set_color --bold brmagenta
            echo V
        case '*'
            set_color --bold red
            echo '?'
    end
    set_color normal
end

function fish_user_key_bindings
    # Execute this once per mode that emacs bindings should be used in
    fish_default_key_bindings -M insert

    # Then execute the vi-bindings so they take precedence when there's a conflict.
    # Without --no-erase fish_vi_key_bindings will default to
    # resetting all bindings.
    # The argument specifies the initial mode (insert, "default" or visual).
    fish_vi_key_bindings --no-erase insert

    # Emulates vim's cursor shape behavior
    # Set the normal and visual mode cursors to a block
    set fish_cursor_default block

    # Set the insert mode cursor to a line
    set fish_cursor_insert line

    # Set the replace mode cursors to an underscore
    set fish_cursor_replace_one underscore
    set fish_cursor_replace underscore

    # Set the external cursor to a line. The external cursor appears when a command is started.
    # The cursor shape takes the value of fish_cursor_default when fish_cursor_external is not specified.
    set fish_cursor_external line
    # The following variable can be used to configure cursor shape in
    # visual mode, but due to fish_cursor_default, is redundant here
    set fish_cursor_visual block

end
