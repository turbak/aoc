local aocUtils = require("aoc_utils")

local FILENAME = "inputs/d02"

local function getInitialSetOfCubes()
    return {
        red = 0,
        green = 0,
        blue = 0
    }
end

local function solution()
    local lines = aocUtils.readInput(FILENAME)
    if lines == nil then
        print("lines is nil")
        return
    end

    ---Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green

    local totalPowerOfCubes = 0
    for i = 1, #lines do
        local line = lines[i] .. ";"
        
        for gameValue in string.gmatch(line, "Game %d+: (.+)") do

            local setOfCubes = getInitialSetOfCubes()
            for gameIteration in string.gmatch(gameValue, "(.-);") do
                for count, color in string.gmatch(gameIteration, "(%d+) (%a+)") do
                    if tonumber(count) > setOfCubes[color] then
                        setOfCubes[color] = tonumber(count)
                    end
                end
            end

            local powerOfCubes = 1
            for _, count in pairs(setOfCubes) do
                powerOfCubes = powerOfCubes * count
            end

            totalPowerOfCubes = totalPowerOfCubes + powerOfCubes
        end
    end

    print(string.format("total power of cubes is %d", totalPowerOfCubes))
end

solution()