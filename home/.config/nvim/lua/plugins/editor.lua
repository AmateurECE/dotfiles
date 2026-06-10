return {
  {
    "nvim-telescope/telescope.nvim",
    opts = {
      pickers = {
        find_files = {
          find_command = { "rg", "--ignore", "--files", "--hidden", "--glob", "!**/.git/*", "--glob", "!**/target/*" },
        },
      },
    },
  },

  { "akinsho/git-conflict.nvim", version = "*", config = true },
}
