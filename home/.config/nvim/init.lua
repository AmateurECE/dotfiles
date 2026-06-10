-- Set <space> as the leader key
-- See `:h mapleader`
-- NOTE: Must happen before plugins are loaded (otherwise wrong leader will be used)
vim.g.mapleader = ' '

-- OPTIONS
--
-- See `:h vim.o`
-- NOTE: You can change these options as you wish!
-- For more options, you can see `:h option-list`
-- To see documentation for an option, you can use `:h 'optionname'`, for example `:h 'number'`
-- (Note the single quotes)

vim.o.number = true -- Show line numbers in a column.

-- Show line numbers relative to where the cursor is.
-- Affects the 'number' option above, see `:h number_relativenumber`.
vim.o.relativenumber = true

vim.o.tabstop = 2
vim.o.softtabstop = 0
vim.o.shiftwidth = 0
vim.o.expandtab = true

-- Sync clipboard between OS and Neovim. Schedule the setting after `UIEnter`
-- because it can increase startup-time. Remove this option if you want your OS
-- clipboard to remain independent. See `:h 'clipboard'`
vim.api.nvim_create_autocmd('UIEnter', {
  callback = function()
    vim.o.clipboard
    = 'unnamedplus'
  end,
})

-- Source - https://stackoverflow.com/a/78976154 Posted by Blomiir, modified by
-- community. See post 'Timeline' for change history Retrieved 2026-06-09,
-- License - CC BY-SA 4.0
vim.api.nvim_create_autocmd("BufWritePre", {
  callback = function()
    local mode = vim.api.nvim_get_mode().mode
    local filetype = vim.bo.filetype
    if vim.bo.modified == true and mode == 'n' and filetype ~= "oil" then
      vim.cmd('lua vim.lsp.buf.format()')
    else
    end
  end
})

-- Case-insensitive searching UNLESS \C or one or more capital letters in the
-- search term
vim.o.ignorecase = true
vim.o.smartcase = true

vim.o.cursorline = true -- Highlight the line where the cursor is on.
vim.o.scrolloff = 10    -- Keep this many screen lines above/below the cursor.
vim.o.list = true       -- Show <tab> and trailing spaces.

-- If performing an operation that would fail due to unsaved changes in the
-- buffer (like `:q`), instead raise a dialog asking if you wish to save the
-- current file(s). See `:h 'confirm'`
vim.o.confirm = true

-- KEYMAPS
--
-- See `:h vim.keymap.set()`, `:h mapping`, `:h keycodes`

-- Use <Esc> to exit terminal mode
vim.keymap.set('t', '<Esc>', '<C-\\><C-n>')

-- Map <A-j>, <A-k>, <A-h>, <A-l> to navigate between windows in any modes
vim.keymap.set({ 't', 'i' }, '<A-h>', '<C-\\><C-n><C-w>h')
vim.keymap.set({ 't', 'i' }, '<A-j>', '<C-\\><C-n><C-w>j')
vim.keymap.set({ 't', 'i' }, '<A-k>', '<C-\\><C-n><C-w>k')
vim.keymap.set({ 't', 'i' }, '<A-l>', '<C-\\><C-n><C-w>l')
vim.keymap.set({ 'n' }, '<A-h>', '<C-w>h')
vim.keymap.set({ 'n' }, '<A-j>', '<C-w>j')
vim.keymap.set({ 'n' }, '<A-k>', '<C-w>k')
vim.keymap.set({ 'n' }, '<A-l>', '<C-w>l')
vim.keymap.set({ 'n', 'i' }, '<C-s>', '<Esc>:w<CR>')
vim.keymap.set({ 'i' }, "<S-BS>", "<Esc>lcl")

-- AUTOCOMMANDS (EVENT HANDLERS)
--
-- See `:h lua-guide-autocommands`, `:h autocmd`, `:h nvim_create_autocmd()`

-- Highlight when yanking (copying) text.
-- Try it with `yap` in normal mode. See `:h vim.hl.on_yank()`
vim.api.nvim_create_autocmd('TextYankPost', {
  desc = 'Highlight when yanking (copying) text',
  callback = function()
    vim.hl.on_yank()
  end,
})

-- USER COMMANDS: DEFINE CUSTOM COMMANDS
--
-- See `:h nvim_create_user_command()` and `:h user-commands`

-- Create a command `:GitBlameLine` that print the git blame for the current line
vim.api.nvim_create_user_command('GitBlameLine', function()
  -- Get the current line number. See `:h line()`
  local line_number = vim.fn.line('.')
  local filename = vim.api.nvim_buf_get_name(0)
  print(vim.system({
    'git', 'blame', '-L', line_number .. ',+1', filename }):wait().stdout)
end, { desc = 'Print the git blame for the current line' })

-- PLUGINS
--
-- See `:h :packadd`, `:h vim.pack`

-- Add the "nohlsearch" package to automatically disable search highlighting
-- after 'updatetime' and when going to insert mode.
vim.cmd('packadd! nohlsearch')

-- Install third-party plugins via "vim.pack.add()".
vim.pack.add({
  -- Quickstart configs for LSP
  'https://github.com/neovim/nvim-lspconfig',
  -- Fuzzy picker
  'https://github.com/ibhagwan/fzf-lua',
  -- Autocompletion
  'https://github.com/nvim-mini/mini.completion',
  -- Enhanced quickfix/loclist
  'https://github.com/stevearc/quicker.nvim',
  -- Git integration
  'https://github.com/lewis6991/gitsigns.nvim',
  -- Theme
  'https://github.com/ellisonleao/gruvbox.nvim',
  -- Utility library
  'https://github.com/nvim-lua/plenary.nvim',

  -- UI
  'https://github.com/nvim-tree/nvim-web-devicons',
  'https://github.com/akinsho/bufferline.nvim',
  'https://github.com/nvim-lualine/lualine.nvim',

  -- Neo-tree
  'https://github.com/nvim-lua/plenary.nvim',
  'https://github.com/MunifTanjim/nui.nvim',
  {
    src = 'https://github.com/nvim-neo-tree/neo-tree.nvim',
    version = vim.version.range('3'),
  },

  -- Comments
  'https://github.com/folke/todo-comments.nvim',

  -- Noice
  'https://github.com/rcarriga/nvim-notify',
  -- nui.nvim is a dependency
  'https://github.com/folke/noice.nvim',
})

--
-- Plugin Setup
--

require('fzf-lua').setup { fzf_colors = true }
require('mini.completion').setup {}
require('quicker').setup {}
require('gitsigns').setup {}
require('gruvbox').setup {}

require('nvim-web-devicons').setup {}
require('bufferline').setup {}
require('lualine').setup {}

require('neo-tree').setup {
  filesystem = {
    filtered_items = {
      -- when true, they will just be displayed differently than normal items
      visible = false,
      hide_dotfiles = false,
      hide_gitignored = true,
      -- only works on Windows for hidden files/directories
      hide_hidden = false,
      -- remains visible even if other settings would normally hide it
      always_show = {
        '.gitignore',
      },
      -- remains hidden even if visible is toggled to true, this overrides
      -- always_show
      never_show = {
        '.DS_Store',
        'thumbs.db',
        '.git',
      },
    },
  },
}

require('todo-comments').setup {
  format_on_save = {
    timeout_ms = 500,
    lsp_format = "fallback",
  }
}

require('notify').setup {}
require('noice').setup({
  lsp = {
    -- override markdown rendering so that **cmp** and other plugins use
    -- **Treesitter**
    override = {
      ['vim.lsp.util.convert_input_to_markdown_lines'] = true,
      ['vim.lsp.util.stylize_markdown'] = true,
      ['cmp.entry.get_documentation'] = true, -- requires hrsh7th/nvim-cmp
    },
  },
  -- you can enable a preset for easier configuration
  presets = {
    bottom_search = true,         -- use a classic bottom cmdline for search
    command_palette = true,       -- position the cmdline and popupmenu together
    long_message_to_split = true, -- long messages will be sent to a split
    inc_rename = false,           -- enables an input dialog for inc-rename.nvim
    lsp_doc_border = false,       -- add border to hover docs and signature help
  },
})

vim.cmd.colorscheme('gruvbox')

vim.keymap.set({ 'n' }, '<leader>e',
  ':Neotree filesystem reveal toggle left<CR>')

vim.lsp.config('clangd',
  {
    root_markers = {
      "compile_commands.json",
      ".git",
      ".clangd",
      ".clang-tidy",
      ".clang-format",
      "compile_flags.txt",
      "configure.ac"
    },
  })

vim.lsp.config('rust_analyzer',
  {
    cmd = { "sh", "-c", "rustup run $(rustup show active-toolchain | awk '{print $1}') rust-analyzer" },
    settings = {
      ["rust-analyzer"] = {
        check = {
          allTargets = false,
        },
        cargo = {
          unsetTest = false,
          sysroot = "discover",
        },
      },
    },
  }
)

vim.lsp.enable('lua_ls')
vim.lsp.enable('clangd')
vim.lsp.enable('pylsp')

vim.g.autoformat = true

vim.o.formatexpr = "v:lua.require'conform'.formatexpr()"
