scriptencoding utf-8

" update plugins ->
" :PlugInstall
" :UpdateRemotePlugins

call plug#begin()
Plug 'SirVer/ultisnips'
Plug 'honza/vim-snippets'
Plug 'kana/vim-submode'        " resize split windows

" differences
Plug 'brooth/Far.vim'

" git
Plug 'tpope/vim-fugitive'

" indent guides
Plug 'Yggdroot/indentLine'

" colorlise status line
" Plug 'itchyny/lightline.vim'
" Plug 'Cassin01/neoline.vim'

" syntastic
Plug 'vim-syntastic/syntastic'

" pathogen for syntastic
Plug 'tpope/vim-pathogen'

" haskell
Plug 'eagletmt/neco-ghc'                " completion
Plug 'neovimhaskell/haskell-vim'        " syntax
Plug 'dag/vim2hs'                       " syntax, indent

" python3
Plug 'davidhalter/jedi-vim'             " completion

" c++
Plug 'octol/vim-cpp-enhanced-highlight' " syntax highlight
Plug 'justmao945/vim-clang'             " completion

" Ruby
Plug 'vim-ruby/vim-ruby'

" Rust
Plug 'rust-lang/rust.vim'              " completion

" toml
Plug 'cespare/vim-toml'

"markdown
"Plug 'godlygeek/tabular'
"Plug 'plasticboy/vim-markdown'

" :Unite colorscheme -auto-preview
Plug 'Shougo/unite.vim'
Plug 'ujihisa/unite-colorscheme'

" colorschemes
Plug 'altercation/vim-colors-solarized'   " solarized
Plug 'croaker/mustang-vim'                " mustang
Plug 'jeffreyiacono/vim-colors-wombat'    " wombat
Plug 'nanotech/jellybeans.vim'            " jellybeans
Plug 'vim-scripts/Lucius'                 " lucius
Plug 'vim-scripts/Zenburn'                " zenburn
Plug 'mrkn/mrkn256.vim'                   " mrkn256
Plug 'jpo/vim-railscasts-theme'           " railscasts
Plug 'therubymug/vim-pyte'                " pyte
Plug 'tomasr/molokai'                     " molokai
Plug 'chriskempson/vim-tomorrow-theme'    " tomorrow night
Plug 'vim-scripts/twilight'               " twilight
Plug 'w0ng/vim-hybrid'                    " hybrid
Plug 'freeo/vim-kalisi'                   " kalisi
Plug 'morhetz/gruvbox'                    " gruvbox
Plug 'toupeira/vim-desertink'             " desertink
Plug 'sjl/badwolf'                        " badwolf
Plug 'itchyny/landscape.vim'              " landscape
Plug 'joshdick/onedark.vim'               " onedark in atom
Plug 'gosukiwi/vim-atom-dark'             " atom-dark
Plug 'liuchengxu/space-vim-dark'          " space-vim-dark -> recommended 'hi Comment cterm=italic'
Plug 'kristijanhusak/vim-hybrid-material' "hybrid_material
Plug 'drewtempelmeyer/palenight.vim'      " palenight

" others
Plug 'scrooloose/nerdtree'
Plug 'Shougo/deoplete.nvim'               " auto completion
call plug#end()
