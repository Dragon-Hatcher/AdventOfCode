function _tide_item_aoc_active_puz
    if path is $_tide_parent_dirs/../AdventOfCode
        set year (jq .active_puzzle.year meta.json 2>/dev/null)
        set day (jq .active_puzzle.day meta.json 2>/dev/null)

        if test "$year" -a "$day"
           _tide_print_item aoc_active_puz $tide_aoc_active_puz_icon' ' "$year, $day"
        end
    end
end
funcsave _tide_item_aoc_active_puz

set -U tide_aoc_active_puz_color yellow
set -U tide_aoc_active_puz_bg_color black

set -U tide_aoc_active_puz_icon 🎄
