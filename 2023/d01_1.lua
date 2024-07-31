local aocUtils = require("aoc_utils")
require "string"

local FILENAME = "inputs/d01"

local function solution()
    local lines = aocUtils.readInput(FILENAME)
    if not lines then return end

    local sum = 0
    for i = 1, #lines do
        local line = lines[i]

        ---@type integer?
        local firstDigit = nil
        ---@type integer?
        local lastDigit = nil

        for j = 1, #line do
            local c = line:sub(j, j)
            local num = tonumber(c)
            if num then
                if not firstDigit then
                    firstDigit = num
                end
                lastDigit = num
            end
        end
    
        sum =  sum + firstDigit * 10 + lastDigit
    end
    
    print(sum)
end

solution()