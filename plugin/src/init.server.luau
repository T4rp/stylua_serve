local CollectionService = game:GetService("CollectionService")
local HttpService = game:GetService("HttpService")
local ScriptEditorService = game:GetService("ScriptEditorService")
local ServerStorage = game:GetService("ServerStorage")
local StudioService = game:GetService("StudioService")
local defaultSettings = require(script.defaultSettings)
local Logger = require(script.Logger)

local PORT = 55096
local FORMAT_URL = "http://localhost:%s/format"

local toolBar = plugin:CreateToolbar("Stylua Format")

local formatButton = toolBar:CreateButton("StyluaServeFormat", "Format this file", "rbxassetid://18448244219", "Format")
formatButton.ClickableWhenViewportHidden = true

local settingsButton = toolBar:CreateButton("Settings", "Open settings", "rbxassetid://18448226997", "Settings")
settingsButton.ClickableWhenViewportHidden = true

local function getSettings()
	local settingsScript = CollectionService:GetTagged("StyluaServeSettings")[1]

	if settingsScript then
		local fn, err = loadstring(settingsScript.Source)

		if err then
			Logger.warn(`Could not load StyluaServeSettings: {err}`)
			return nil
		end

		local isOk, resultOrErr = pcall(fn)

		if not isOk then
			Logger.warn(`Failed to load StyluaServeSettings: {resultOrErr}`)
			return nil
		end

		return resultOrErr
	else
		return defaultSettings
	end
end

local function formatSource(source: string)
	local styluaServeSettings = getSettings()
	local styluaOptions = styluaServeSettings.styluaOptions

	local url = string.format(FORMAT_URL, PORT)

	local body = {
		options = styluaOptions,
		source = source,
	}

	local bodyJson = HttpService:JSONEncode(body)

	local isOk, errOrResult = pcall(HttpService.RequestAsync, HttpService, {
		Url = url,
		Method = "POST",
		Body = bodyJson,
		Headers = {
			["Content-Type"] = "application/json",
		},
	})

	if not isOk then
		return nil, errOrResult
	end

	if errOrResult.StatusCode ~= 200 then
		return nil, errOrResult.Body
	end

	return errOrResult.Body
end

formatButton.Click:Connect(function()
	local activeScript = StudioService.ActiveScript

	if not activeScript then
		return
	end

	local document = ScriptEditorService:FindScriptDocument(activeScript)

	if not document then
		return
	end

	if document:IsCommandBar() then
		return
	end

	local lineCount = document:GetLineCount()
	local lastLine = document:GetLine(lineCount)
	local lastCharacter = #lastLine + 1

	local source = document:GetText()
	local formatted, err = formatSource(source)

	if err then
		Logger.warn(`Failed to format: {err}`)
		return
	end

	document:EditTextAsync(formatted, 1, 1, lineCount, lastCharacter)
end)

settingsButton.Click:Connect(function()
	local settingsScript = CollectionService:GetTagged("StyluaServeSettings")[1]

	if not settingsScript then
		settingsScript = script.defaultSettings:Clone()
		settingsScript.Name = "StyluaServeSettings"
		settingsScript:AddTag("StyluaServeSettings")
		settingsScript.Parent = ServerStorage
	end

	local isOk, err = pcall(ScriptEditorService.OpenScriptDocumentAsync, ScriptEditorService, settingsScript)

	if not isOk then
		Logger.warn(`Failed to open settings script: {err}`)
	end
end)
