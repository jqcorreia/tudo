local function string_split(inputstr, sep)
	if sep == nil then
		sep = "%s"
	end
	local t = {}
	for str in string.gmatch(inputstr, "(.-)(" .. sep .. ")") do
		table.insert(t, str)
	end
	return t
end

local function get_dir_contents(dir)
	local i, t, popen = 0, {}, io.popen

	local pfile = popen("ls -1 '" .. dir .. "'")
	if pfile == nil then
		return {}
	end
	for filename in pfile:lines() do
		i = i + 1
		t[i] = filename
	end
	pfile:close()
	return t
end

local res = {}
for i, file in ipairs(get_dir_contents(os.getenv("HOME") .. "/.password-store")) do
	local title = string_split(file, ".gpg")[1]
	print(title)

	res[i] = { title = title, icon = nil, action = { type = "secret", secret_name = title } }

	print(file)
end

return res
