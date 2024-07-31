local aocUtils = require("aoc_utils")

local FILENAME = "inputs/d01"

local numberNameToNumber = {
    one = 1,
    two = 2,
    three = 3,
    four = 4,
    five = 5,
    six = 6,
    seven = 7,
    eight = 8,
    nine = 9
}

local MIN_NUMBER_NAME_LEN = 3
local MAX_NUMBER_NAME_LEN = 5

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

        for nameStart = 1, #line do
            local c = line:sub(nameStart, nameStart)
            local num = tonumber(c)
            if not num then
                for nameEnd = nameStart+MIN_NUMBER_NAME_LEN-1, nameStart+MAX_NUMBER_NAME_LEN do
                    local name = line:sub(nameStart, nameEnd)
                    local numFromTable = numberNameToNumber[name]
                    if numFromTable then
                        num = numberNameToNumber[name]
                        break
                    end
                end
            end

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