local aocUtils = require("aoc_utils")

local FILENAME = "inputs/d02"

local maxCubesByColor = {
    red = 12,
    green = 13,
    blue = 14
}

local function solution()
    local lines = aocUtils.readInput(FILENAME)
    if lines == nil then
        print("lines is nil")
        return
    end

    ---Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green

    local legalGamesIDsSum = 0
    for i = 1, #lines do
        local line = lines[i] .. ";"
        
        for gameNumber, gameValue in string.gmatch(line, "Game (%d+): (.+)") do
            for gameIteration in string.gmatch(gameValue, "(.-);") do
                for count, color in string.gmatch(gameIteration, "(%d+) (%a+)") do
                    if tonumber(count) > maxCubesByColor[color] then
                        print(string.format("game %d is illegal", tonumber(gameNumber)))
                        goto loop
                    end
                end
            end
            legalGamesIDsSum = legalGamesIDsSum + tonumber(gameNumber)
            ::loop::
        end
    end

    print(string.format("sum of legal games IDs = %d", legalGamesIDsSum))
end

solution()