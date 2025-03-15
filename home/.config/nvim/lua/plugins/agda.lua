return {
  {
    "ashinkarov/nvim-agda",
    -- Only load this plugin when a *.agda file is opened.
    ft = "agda",
    -- The plugin does not call its own setup function, apparently?
    config = function()
      require("agda").setup()
    end,
    -- Setup function does not set up the key bindings listed in the README :(
    keys = {
      { "<leader>l", "<cmd>AgdaLoad<CR>" },
      { "<leader>q", "<cmd>AgdaCloseMsg<CR>" },
      { "<leader>,", "<cmd>AgdaTypeContext<CR>" },
      { "<leader>.", "<cmd>AgdaTypeContextInfer<CR>" },
      { "<leader>u,", "<cmd>AgdaTypeContextNorm<CR>" },
      { "<leader>d", "<cmd>AgdaInfer<CR>" },
      { "<leader>r", "<cmd>AgdaRefine<CR>" },
      { "<leader>c", "<cmd>AgdaMakeCase<CR>" },
      { "<leader>n", "<cmd>AgdaCompute<CR>" },
      { "<leader>a", "<cmd>AgdaAuto<CR>" },
      { "<leader>s", "<cmd>AgdaSolve<CR>" },
      { "<leader>h", "<cmd>AgdaHelperFun<CR>" },
      { "<leader>o", "<cmd>AgdaModuleContents<CR>" },
      { "<leader>w", "<cmd>AgdaWhyInscope<CR>" },
      { "<leader>e", "<cmd>MkPrompt<CR>" },
      { "<leader>?", "<cmd>PrintGoals<CR>" },
      { "<leader>f", "<cmd>GoalNext<CR>" },
      { "<leader>b", "<cmd>GoalPrev<CR>" },
    },
  },
}
