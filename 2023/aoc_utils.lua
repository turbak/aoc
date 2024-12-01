---@class aocutils
local aocUtils = {}

---@param filename string
---@return string[]?
function aocUtils.readInput(filename)
    local file = assert(io.open(filename, "r"), "failed to open file")
    if not file then return nil end

    local lines = {}
    for line in file:lines() do
        table.insert(lines, line)
    end
    file:close()
    return lines
end

return aocUtils