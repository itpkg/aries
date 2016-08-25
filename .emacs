;(set-default-font "WenQuanYi Zen Hei 12")
(set-default-font "YaHei Consolas Hybrid 14")

;(custom-set-variables
;   '(initial-frame-alist (quote ((fullscreen . maximized)))))


;(setf inhibit-splash-screen t)
;(switch-to-buffer (get-buffer-create "emtpy"))
;(delete-other-windows)

(tool-bar-mode -1)
(menu-bar-mode -1)
(setq inhibit-startup-message t)

(setq-default cursor-type 'bar)
(blink-cursor-mode -1)
(column-number-mode t)
(show-paren-mode 1)

(defalias 'yes-or-no-p 'y-or-n-p)
(setq tab-width 4)

;backup/autosave
(defvar backup-dir (expand-file-name "~/.emacs.d/backup/"))
(defvar autosave-dir (expand-file-name "~/.emacs.d/autosave/"))
(setq backup-directory-alist (list (cons ".*" backup-dir)))
(setq auto-save-list-file-prefix autosave-dir)
(setq auto-save-file-name-transforms `((".*" ,autosave-dir t)))

;git clone https://github.com/mattfidler/tabbar-ruler.el.git ~/.emacs.d/tabbar-ruler.el
;(add-to-list 'load-path "~/.emacs.d/tabbar-ruler.el")
;(setq tabbar-ruler-global-tabbar t) ; If you want tabbar
;(setq tabbar-ruler-global-ruler t) ; if you want a global ruler
;(setq tabbar-ruler-popup-menu t) ; If you want a popup menu.
;(setq tabbar-ruler-popup-toolbar t) ; If you want a popup toolbar
;(setq tabbar-ruler-popup-scrollbar t) ; If you want to only show the
                                      ; scroll bar when your mouse is moving.
;(require 'tabbar-ruler)



;git clone git://jblevins.org/git/markdown-mode.git ~/.emacs.d/markdown-mode
(add-to-list 'load-path "~/.emacs.d/markdown-mode")
(autoload 'markdown-mode "markdown-mode"
   "Major mode for editing Markdown files" t)
(add-to-list 'auto-mode-alist '("\\.md\\'" . markdown-mode))


;git clone https://github.com/sellout/emacs-color-theme-solarized.git ~/.emacs.d/emacs-color-theme-solarized
(add-to-list 'custom-theme-load-path "~/.emacs.d/emacs-color-theme-solarized")
(load-theme 'solarized t)

;git clone https://github.com/rust-lang/rust-mode.git ~/.emacs.d/rust-mode
(add-to-list 'load-path "~/.emacs.d/rust-mode/")
(autoload 'rust-mode "rust-mode" nil t)
(add-to-list 'auto-mode-alist '("\\.rs\\'" . rust-mode))

;git clone https://github.com/dominikh/go-mode.el.git ~/.emacs.d/go-mode
;(add-to-list 'load-path "~/.emacs.d/go-mode/")
;(require 'go-mode-autoloads)


;git clone https://github.com/dryman/toml-mode.el.git ~/.emacs.d/toml-mode
(add-to-list 'load-path "~/.emacs.d/toml-mode/")
(require 'toml-mode)

