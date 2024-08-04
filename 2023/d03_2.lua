local aocUtils = require("aoc_utils")

local FILENAME = "inputs/d03"

---@alias numberAndIndex {num: number, indexes: rowAndCol[]}
---@alias rowAndCol {rowIdx: number, colIdx: number}

---@param s string
---@param rowIdx number
---@return numberAndIndex[]
local function getAllNumbersAndIndexes(s, rowIdx)
    ---@type numberAndIndex[]
    local res = {}
    ---@type integer?
    local findStart = 1
    while true do
        local startIdx, endIdx = s:find("%d+", findStart)
        if startIdx == nil then
            break
        end

        local numStr = s:sub(startIdx, endIdx)
        ---@type rowAndCol[]
        local indexesArr = {}
        for i = startIdx, endIdx, 1 do
            indexesArr[#indexesArr+1] = {rowIdx = rowIdx, colIdx = i}
        end

        local numParsed = tonumber(numStr)
        if numParsed == nil then
            print("invalid number", numStr)
            return {}
        end

        res[#res+1] = {num = numParsed, indexes = indexesArr}

        findStart = endIdx+1
    end

    return res
end

---@param s string
---@param rowIdx number
---@return rowAndCol[]
local function getAllGearIndexes(s, rowIdx)
    ---@type rowAndCol[]
    local res = {}
    ---@type integer?
    local findStart = 1
    while true do
        local startIdx, endIdx = s:find("[^%d.]", findStart)
        if startIdx == nil then
            break
        end

        res[#res+1] = {rowIdx = rowIdx, colIdx = startIdx}

        findStart = endIdx+1
    end

    return res
end

---@param a rowAndCol
---@param b rowAndCol
---@return boolean
local function isAdjacent(a, b)
    local colDiff = math.abs(a.colIdx - b.colIdx)
    local rowDiff = math.abs(a.rowIdx - b.rowIdx)

    return colDiff < 2 and rowDiff < 2
end

local function solution()
    local lines = aocUtils.readInput(FILENAME)
    if lines == nil then
        print("lines is nil")
        return
    end

    ---@type numberAndIndex[]
    local allNumbersAndIndexes = {}
    ---@type rowAndCol[]
    local allGearIndexes = {}

    for i = 1, #lines do
        local line = lines[i]

        local numsAndIdxesFromLine = getAllNumbersAndIndexes(line, i)
        table.move(numsAndIdxesFromLine, 1, #numsAndIdxesFromLine, #allNumbersAndIndexes + 1, allNumbersAndIndexes)

        local symbolIdxesFromLine = getAllGearIndexes(line, i)
        table.move(symbolIdxesFromLine, 1, #symbolIdxesFromLine, #allGearIndexes + 1, allGearIndexes)
    end

    local sumOfAllGearRatios = 0
    for _, gearIdx in ipairs(allGearIndexes) do
        ---@type number[]
        local adjacentNumbers = {}
        for _, numberAndIdxes in ipairs(allNumbersAndIndexes) do
            for _, separateNumIdx in ipairs(numberAndIdxes.indexes) do
                if isAdjacent(separateNumIdx, gearIdx) then
                    adjacentNumbers[#adjacentNumbers+1] = numberAndIdxes.num
                    break
                end
            end

            if #adjacentNumbers > 2 then
                break
            end
        end

        if #adjacentNumbers == 2 then
            local gearRatio = 1
            for _, num in ipairs(adjacentNumbers) do
                gearRatio = gearRatio * num
            end

            sumOfAllGearRatios = sumOfAllGearRatios + gearRatio
        end
    end

    print(sumOfAllGearRatios)
end

solution()