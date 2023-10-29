return {
  {
    "neovim/nvim-lspconfig",
    ---@class PluginLspOpts
    opts = {
      servers = {
        rust_analyzer = {
          cmd = { "sh", "-c", "rustup run $(rustup show active-toolchain | awk '{print $1}') rust-analyzer" },
          settings = {
            ['rust-analyzer'] = {
              check = {
                allTargets = false,
              },
            },
          },
        },
      },
    },
  },
}
