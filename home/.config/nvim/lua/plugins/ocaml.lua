return {
  {
    "neovim/nvim-lspconfig",
    ---@class PluginLspOpts
    opts = {
      servers = {
        ocamllsp = { },
      },
    },
  },

  {
    "stevearc/conform.nvim",
    opts = {
      formatters = {
        ocamlformat = {},
      },
    }
  }
}
