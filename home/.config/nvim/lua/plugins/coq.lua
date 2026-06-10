vim.g.coqtail_treat_stderr_as_warning = 1

return {
  {
    "neovim/nvim-lspconfig",
    ---@class PluginLspOpts
    opts = {
      servers = {
        coq_lsp = {},
      },
    },
  },

  {
    "whonore/Coqtail",
    init = function()
      -- Use a compact window layout on smaller screens. See
      -- autoload/coqtail/panels.vim in the coqtail plugin for
      -- more information.
      if vim.o.columns < 140 then
        vim.g.coqtail_panel_layout = {
          goal = {
            { "info", "left" },
            { "main", "below" },
          },
          info = {
            { "goal", "right" },
            { "main", "below" },
          },
        }
      end
    end,
  },
}
