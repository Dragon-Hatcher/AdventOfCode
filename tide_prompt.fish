function _tide_item_aoc_active_puz
    if path is $_tide_parent_dirs/../AdventOfCode
        set year (jq .active_puzzle.year meta.json 2>/dev/null)
        set day (jq .active_puzzle.day meta.json 2>/dev/null)

        set day_pad (printf "%02d\n" $day)
        set stars (jq ".puzzle_infos.\"$year-$day_pad\" | [.part1_solution, .part2_solution] | map(select(. != null)) | length" meta.json)

        set star_str ""
        for i in (seq $stars)
            set star_str "$star_str*"
        end

        if test "$year" -a "$day"
            set o (echo -e "$year day $day \e[38;2;255;255;102m$star_str\e[0m")
           _tide_print_item aoc_active_puz $tide_aoc_active_puz_icon' ' $o
        end
    end
end
funcsave _tide_item_aoc_active_puz

set -U tide_aoc_active_puz_color yellow
set -U tide_aoc_active_puz_bg_color black

set -U tide_aoc_active_puz_icon 🎄
