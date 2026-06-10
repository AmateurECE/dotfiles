return {
  {
    "neovim/nvim-lspconfig",
    ---@class PluginLspOpts
    opts = {
      servers = {
        hls = {
          cmd = { "stack", "exec", "haskell-language-server-wrapper", "--", "--lsp" }
        },
      },
    },
  },
}
