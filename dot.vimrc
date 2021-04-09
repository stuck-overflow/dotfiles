if empty(glob('~/.vim/autoload/plug.vim'))
  silent !curl -fLo ~/.vim/autoload/plug.vim --create-dirs
    \ https://raw.githubusercontent.com/junegunn/vim-plug/master/plug.vim
  autocmd VimEnter * PlugInstall --sync | source $MYVIMRC
endif
call plug#begin()
Plug 'preservim/NERDTree'
Plug 'prabirshrestha/vim-lsp'
Plug 'mattn/vim-lsp-settings'
Plug 'prabirshrestha/asyncomplete.vim'
Plug 'prabirshrestha/asyncomplete-lsp.vim'
Plug 'junegunn/fzf', { 'do': { -> fzf#install() } }
Plug 'junegunn/fzf.vim'
Plug 'kitagry/asyncomplete-tabnine.vim', { 'do': './install.sh' }
Plug 'sainnhe/edge'
call plug#end()

inoremap <expr> <Tab>   pumvisible() ? "\<C-n>" : "\<Tab>"
inoremap <expr> <S-Tab> pumvisible() ? "\<C-p>" : "\<S-Tab>"
inoremap <expr> <cr>    pumvisible() ? asyncomplete#close_popup() : "\<cr>"

imap <c-space> <Plug>(asyncomplete_force_refresh)

set nu

let g:lsp_diagnostics_float_cursor = 1
let &t_8f = "\<Esc>[38;2;%lu;%lu;%lum"
let &t_8b = "\<Esc>[48;2;%lu;%lu;%lum"

set background=dark
set t_Co=256
call asyncomplete#register_source(asyncomplete#sources#tabnine#get_source_options({
    \ 'name': 'tabnine',
    \ 'allowlist': ['*'],
    \ 'completor': function('asyncomplete#sources#tabnine#completor'),
    \ 'config': {
    \   'line_limit': 1000,
    \   'max_num_result': 20,
    \  },
    \ }))
" Important!!
if has('termguicolors')
          set termguicolors
endif

" The configuration options should be placed before `colorscheme edge`.
let g:edge_style = 'default'
let g:edge_enable_italic = 1
let g:edge_disable_italic_comment = 1

colorscheme edge






