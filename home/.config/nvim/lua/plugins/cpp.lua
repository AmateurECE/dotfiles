local lspconfig = require("lspconfig")

return {
  {
    "neovim/nvim-lspconfig",
    ---@class PluginLspOpts
    opts = {
      servers = {
        clangd = {
          root_dir = lspconfig.util.root_pattern(
            "compile_commands.json",
            ".git",
            ".clangd",
            ".clang-tidy",
            ".clang-format",
            "compile_flags.txt",
            "configure.ac"
          ),
        },
      },
    },
  },
}
