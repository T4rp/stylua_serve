local LOG_FORMAT = "[StyluaServePlugin] %s"

local Logger = {}

function Logger.warn(str)
    warn(string.format(LOG_FORMAT, str))
end

return Logger
