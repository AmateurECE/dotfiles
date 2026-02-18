return {
  {
    "neovim/nvim-lspconfig",
    ---@class PluginLspOpts
    opts = {
      servers = {
        coq_lsp = {}
      }
    },
  },

  { "whonore/Coqtail" },
}
