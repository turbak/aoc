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
local function getAllSymbolIndexes(s, rowIdx)
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
    local allSymbolIndexes = {}

    for i = 1, #lines do
        local line = lines[i]

        local numsAndIdxesFromLine = getAllNumbersAndIndexes(line, i)
        table.move(numsAndIdxesFromLine, 1, #numsAndIdxesFromLine, #allNumbersAndIndexes + 1, allNumbersAndIndexes)

        local symbolIdxesFromLine = getAllSymbolIndexes(line, i)
        table.move(symbolIdxesFromLine, 1, #symbolIdxesFromLine, #allSymbolIndexes + 1, allSymbolIndexes)
    end

    local sumOfAllAdjacentNums = 0
    for _, numberAndIdxes in ipairs(allNumbersAndIndexes) do
        for _, separateNumIdx in ipairs(numberAndIdxes.indexes) do
            for _, symbolIdxes in ipairs(allSymbolIndexes) do
                if isAdjacent(separateNumIdx, symbolIdxes) then
                    sumOfAllAdjacentNums = sumOfAllAdjacentNums + numberAndIdxes.num
                    goto outerLoop
                end
            end
        end
        ::outerLoop::
    end

    print(sumOfAllAdjacentNums)
end

solution()