scriptencoding utf-8

" indent-guides
"let g:indent_guides_enable_on_vim_startup=1 " enable indent-guides let g:indent_guides_guide_size =1
let g:indent_guides_start_level=1
let g:indent_guides_auto_colors=0 " enable auto colors
let g:indent_guides_exclude_filetypes = ['help', 'nerdtree']
autocmd VimEnter,Colorscheme * :hi IndentGuidesOdd   ctermbg=237
autocmd VimEnter,Colorscheme * :hi IndentGuidesEven  ctermbg=237

" indentLine
let g:indentLine_enabled = 0 " disable by default
let g:indentLine_char = '⎸'

" fugitive.vim
set statusline+=%{FugitiveStatusline()}

" syntasic
set statusline+=%#warningmsg#
set statusline+=%{SyntasticStatuslineFlag()}
set statusline+=%*
let g:syntastic_always_populate_loc_list = 1
let g:syntastic_auto_loc_list = 1
let g:syntastic_check_on_open = 1
let g:syntastic_check_on_wq = 0
let g:syntastic_mode_map = {
      \ 'mode': 'passive',
      \ 'active_filetypes': ['c','cpp','python3']
      \}
let g:syntastic_cpp_compiler = 'clang++'
let g:syntastic_cpp_compiler_options = ' -std=c++11 -stdlib=libc++'

" submode
call submode#enter_with('bufmove', 'n', '', 's>', '<C-w>>')
call submode#enter_with('bufmove', 'n', '', 's<', '<C-w><')
call submode#enter_with('bufmove', 'n', '', 's+', '<C-w>+')
call submode#enter_with('bufmove', 'n', '', 's-', '<C-w>-')
call submode#map('bufmove', 'n', '', '>', '<C-w>>')
call submode#map('bufmove', 'n', '', '<', '<C-w><')
call submode#map('bufmove', 'n', '', '+', '<C-w>+')
call submode#map('bufmove', 'n', '', '-', '<C-w>-')

" lightline
" let g:lightline = {
"      \ 'colorscheme': 'onedark',
"      \ }

"vim-clang
let g:clang_c_options = '-std=gnu11'
let g:clang_cpp_options = '-std=c++11 -stdlib=libc++' " c++11 用
" disable auto completion for vim-clang
let g:clang_auto = 0

" deoplate
" let g:deoplete#enable_at_startup = 1

"------------------------------------------
"           -- colorschemes --
"------------------------------------------
