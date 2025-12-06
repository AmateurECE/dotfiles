return {
  {
    "ashinkarov/nvim-agda",
    -- Only load this plugin when a *.agda file is opened.
    ft = "agda",
    -- The plugin does not call its own setup function, apparently?
    -- Setup function does not set up the key bindings listed in the README :(
    config = function()
        vim.keymap.set("n", "<leader>l", ":AgdaLoad<CR>")
        vim.keymap.set("n", "<leader>q", ":AgdaCloseMsg<CR>")
        vim.keymap.set("n", "<leader>,", ":AgdaTypeContext<CR>")
        vim.keymap.set("n", "<leader>.", ":AgdaTypeContextInfer<CR>")
        vim.keymap.set("n", "<leader>u,", ":AgdaTypeContextNorm<CR>")
        vim.keymap.set("n", "<leader>d", ":AgdaInfer<CR>")
        vim.keymap.set("n", "<leader>r", ":AgdaRefine<CR>")
        vim.keymap.set("n", "<leader>c", ":AgdaMakeCase<CR>")
        vim.keymap.set("n", "<leader>n", ":AgdaCompute<CR>")
        vim.keymap.set("n", "<leader>a", ":AgdaAuto<CR>")
        vim.keymap.set("n", "<leader>s", ":AgdaSolve<CR>")
        vim.keymap.set("n", "<leader>h", ":AgdaHelperFun<CR>")
        vim.keymap.set("n", "<leader>o", ":AgdaModuleContents<CR>")
        vim.keymap.set("n", "<leader>w", ":AgdaWhyInscope<CR>")
        vim.keymap.set("n", "<leader>e", ":MkPrompt<CR>")
        vim.keymap.set("n", "<leader>?", ":PrintGoals<CR>")
        vim.keymap.set("n", "<leader>f", ":GoalNext<CR>")
        vim.keymap.set("n", "<leader>b", ":GoalPrev<CR>")
    end,
  },
}
