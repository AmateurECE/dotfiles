return {
  { "williamboman/mason-lspconfig.nvim", enabled = false },

  {
    "williamboman/mason.nvim",
    opts = {
      ensure_installed = {
        "stylua",
        "shellcheck",
        "shfmt",
      },
    },
  },

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

  {
    "neovim/nvim-lspconfig",
  },

  {
    "nvim-neo-tree/neo-tree.nvim",
    opts = {
      filesystem = {
        filtered_items = {
          visible = false, -- when true, they will just be displayed differently than normal items
          hide_dotfiles = false,
          hide_gitignored = true,
          hide_hidden = false, -- only works on Windows for hidden files/directories
          always_show = { -- remains visible even if other settings would normally hide it
            ".gitignore",
          },
          never_show = { -- remains hidden even if visible is toggled to true, this overrides always_show
            ".DS_Store",
            "thumbs.db",
            ".git",
          },
        },
      },
    },
  },

  { "akinsho/git-conflict.nvim", version = "*", config = true },

  { "folke/flash.nvim", enabled = false },
}
