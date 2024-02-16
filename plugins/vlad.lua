local function keys(tbl)
	local res = {}
	for k, _ in pairs(tbl) do
		table.insert(res, k)
	end
	return res
end

local function tprint(tbl, indent)
	if not indent then
		indent = 0
	end
	local toprint = string.rep(" ", indent) .. "{\r\n"
	indent = indent + 2
	for k, v in pairs(tbl) do
		toprint = toprint .. string.rep(" ", indent)
		if type(k) == "number" then
			toprint = toprint .. "[" .. k .. "] = "
		elseif type(k) == "string" then
			toprint = toprint .. k .. "= "
		end
		if type(v) == "number" then
			toprint = toprint .. v .. ",\r\n"
		elseif type(v) == "string" then
			toprint = toprint .. '"' .. v .. '",\r\n'
		elseif type(v) == "table" then
			toprint = toprint .. tprint(v, indent + 2) .. ",\r\n"
		else
			toprint = toprint .. '"' .. tostring(v) .. '",\r\n'
		end
	end
	toprint = toprint .. string.rep(" ", indent - 2) .. "}"
	return toprint
end

local function get_systems(token)
	local a = http_get("https://kong-api.prd.worten.net/vlad2/latest/systems", {
		Authorization = "Bearer " .. token,
	})

	local res = {}
	for i, value in pairs(a.systems) do
		res[i] = { title = value.name, icon = nil, action = { type = "secret", secret_name = value.name } }
	end
	print(tprint(res))

	return res
end

local function get_components(token)
	local a = http_get("https://kong-api.prd.worten.net/vlad2/latest/componentsv2?page_size=1000", {
		Authorization = "Bearer " .. token,
	})

	local tmp = {}

	for i, value in pairs(a.components) do
		if i > 1 then
			tmp[i] = value
		end
	end

	local cenas = {}
	local idx = 1
	for _, value in pairs(tmp) do
		if type(value.system) == "string" and type(value.name) == "string" then
			cenas[idx] = {
				title = value.system .. "-" .. value.name .. "-" .. value.environment,
				icon = nil,
				action = { type = "secret", secret_name = "foo" },
			}
			idx = idx + 1
		end
	end

	return cenas
end

local function parse_token()
	local auth_path = "/home/jqcorreia/.config/vlad/auth.json"
	vlad_auth = open_json(auth_path)

	local account_key = ""
	for _, k in ipairs(keys(vlad_auth.AccessToken)) do
		account_key = k
	end

	local token = vlad_auth.AccessToken[account_key].secret
	return token
end

local token = parse_token()
return get_components(token)
