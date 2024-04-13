function rd.
    set directory (basename (pwd))
    cd ..
    dt rd $directory; or cd $directory
end

function rd
    dt rd $argv
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