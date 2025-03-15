-- Function to set up LuaRocks paths
local function add_luarocks_paths()
  local luarocks_path = vim.fn.system("luarocks path --lr-path"):gsub("\n", "")
  local luarocks_cpath = vim.fn.system("luarocks path --lr-cpath"):gsub("\n", "")

  package.path = package.path .. ";" .. luarocks_path
  package.cpath = package.cpath .. ";" .. luarocks_cpath
end

add_luarocks_paths()

-- bootstrap lazy.nvim, LazyVim and your plugins
require("config.lazy")
