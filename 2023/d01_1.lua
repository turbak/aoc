---@param filename string
---@return string[]?
local function readInput(filename)
    local file = io.open(filename, "r")
    if not file then return nil end

    local lines = {}
    for line in file:lines() do
        table.insert(lines, line)
    end
    file:close()
    return lines
end

local FILENAME = "inputs/d01"

local function solution()
    local lines = readInput(FILENAME)
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