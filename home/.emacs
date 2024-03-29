;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
;; NAME:	    .emacs
;;
;; AUTHOR:	    Ethan D. Twardy
;;
;; DESCRIPTION:	    Emacs initialization file.
;;
;; CREATED:	    09/15/2017
;;
;; LAST EDITED:	    01/20/2023
;;;

;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
;; MISCELLANEOUS INITIALIZATION
;;;

(custom-set-variables
 ;; custom-set-variables was added by Custom.
 ;; If you edit it by hand, you could mess it up, so be careful.
 ;; Your init file should contain only one such instance.
 ;; If there is more than one, they won't work right.
 '(c-basic-offset 4)
 '(column-number-mode t)
 '(custom-enabled-themes '(manoj-dark))
 '(electric-pair-mode t)
 '(indent-tabs-mode nil)
 '(inhibit-default-init t)
 '(ispell-dictionary nil)
 '(package-selected-packages
   '(mustache-mode kotlin-mode ninja-mode plantuml-mode dts-mode nftables-mode csharp-mode kconfig-mode jenkinsfile-mode lua-mode bison-mode rjsx-mode cmake-mode nginx-mode yaml-mode meson-mode ess local-snippet bitbake markdown-mode dockerfile-mode insert-banner rust-mode))
 '(standard-indent 8)
 '(tab-stop-list '(4 8 12 16 20 24 28 32)))

;; Configure for MELPA
(require 'package)
(let* ((no-ssl (and (memq system-type '(windows-nt ms-dos))
		    (not (gnutls-available-p))))
       (url (concat (if no-ssl "http" "https") "://melpa.org/packages/")))
  (add-to-list 'package-archives (cons "melpa" url) t))
(package-initialize)
;; (package-refresh-contents)

;; Turn on error-catching.
(setq debug-on-error t)

;; Prevent verilog mode from automatically inserting a newline after every
;; semicolon.
(setq verilog-auto-newline nil)

(if (eq major-mode 'nxml-mode)
    (setq indent-tabs-mode t))

(require 'local-snippet)
(add-hook 'emacs-startup-hook (lambda () (load-local-snippets)))

(require 'insert-banner)

;; Re-enable up/downcase-region command
(put 'downcase-region 'disabled nil)
(put 'upcase-region 'disabled nil)

;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
;; Auto-mode-alist Modifications
;;;

(add-to-list 'auto-mode-alist '("\\.gradle" . java-mode))
(add-to-list 'auto-mode-alist '("\\.bash_aliases" . sh-mode))
(add-to-list 'auto-mode-alist '("\\.plist" . xml-mode))
(delete "[Mm]akefile\\'" auto-mode-alist)
(add-to-list 'auto-mode-alist '("[Mm]akefile\\'" . makefile-gmake-mode))
(add-to-list 'auto-mode-alist '("\\.html" . nxml-mode))
(add-to-list 'auto-mode-alist '("\\.Rd\\'" . doctex-mode))
(add-to-list 'auto-mode-alist '("\\.tcc\\'" . c++-mode))

(setq auto-mode-alist (delete '("\\.js\\'" . javascript-mode) auto-mode-alist))
(add-to-list 'auto-mode-alist '("\\.js\\'" . rjsx-mode))
(add-to-list 'auto-mode-alist '("\\.mjs\\'" . rjsx-mode))

(add-to-list 'auto-mode-alist '("\\.c\\'" . c-mode))
(setq auto-mode-alist (delete '("\\.[ch]\\'" . c-mode) auto-mode-alist))
(setq auto-mode-alist (delete '("\\.[ch]\\(pp\\|xx\\|\\+\\+\\)\\?\\'"
                                . c++-mode)
                              auto-mode-alist))
(add-to-list 'auto-mode-alist '("\\.[ch]\\(pp\\|xx\\|\\+\\+\\)\\?\\'"
                                . c++-mode))
(add-to-list 'auto-mode-alist '("\\(Containerfile\\|Dockerfile\\)\\'"
                                . dockerfile-mode))

(add-to-list 'auto-mode-alist
             '("\\.bb\\'" . bitbake-mode)
             '("\\.inc\\'" . bitbake-mode))

(add-to-list 'auto-mode-alist '("PKGBUILD\\'" . sh-mode))

(add-to-list 'auto-mode-alist '("\\.kts\\'" . kotlin-mode))

;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
;; KEY BINDINGS
;;;

;; Set undo to \C-z
(global-unset-key (kbd "C-/"))
;; (global-unset-key (kbd "C-z"))
(global-set-key (kbd "C-z") 'undo)

(global-unset-key (kbd "C-u"))
(if (eq major-mode 'latex-mode)
    (progn
      (global-set-key (kbd "C-u u") '(lambda()
                                       (interactive) (insert "{\\\"u}")))
      (global-set-key (kbd "C-u a") '(lambda()
                                       (interactive) (insert "{\\\"a}")))
      (global-set-key (kbd "C-u o") '(lambda()
                                       (interactive) (insert "{\\\"o}")))
      (global-set-key (kbd "C-u U") '(lambda()
                                       (interactive) (insert "{\\\"U}")))
      (global-set-key (kbd "C-u A") '(lambda()
                                       (interactive) (insert "{\\\"A}")))
      (global-set-key (kbd "C-u O") '(lambda()
                                       (interactive) (insert "{\\\"O}")))
      (global-set-key (kbd "C-u s") '(lambda()
                                       (interactive) (insert "{\\ss}")))))

;; For emmet-mode
(if (fboundp 'emmet-expand-line)
    (progn
      (global-set-key (kbd "C-u") 'emmet-expand-line)))

;; Rebind universal argument
(global-set-key (kbd "C-n") 'universal-argument)

;; forward-whitespace and backward-whitespace (see below) are shadowed by C-j
;; key binding in LaTeX mode and Asm mode. These hooks fix that.
(add-hook 'latex-mode-hook
	  (lambda()
	    (local-unset-key (kbd "C-j"))))
(add-hook 'asm-mode-hook
	  (lambda()
	    (local-unset-key (kbd "C-j"))))


(defun forward-whitespace (arg)
  "Move point to the end of the next sequence of whitespace chars.
Each such sequence may be a single newline, or a sequence of
consecutive space and/or tab characters.
With prefix argument ARG, do it ARG times if positive, or move
backwards ARG times if negative."
  (interactive "^p")
  (if (natnump arg)
      (re-search-forward "[ \t]+\\|\n" nil 'move arg)
    (while (< arg 0)
      (if (re-search-backward "[ \t]+\\|\n" nil 'move)
	  (or (eq (char-after (match-beginning 0)) ?\n)
	      (skip-chars-backward " \t")))
      (setq arg (1+ arg)))))

;; Bindings for forward-whitespace and backward whitespace, etc.
(global-unset-key (kbd "C-k"))
(global-set-key (kbd "C-k") 'forward-whitespace)
(global-unset-key (kbd "C-j"))
(global-set-key (kbd "C-j") ;; Lamba function that behaves like a mirror of
		'(lambda()  ;; forward-whitespace.
		   (interactive)
		   (cond
		    ((eq (char-before (point)) ?\n)
		     (skip-chars-backward " \n\t"))
		    ((or (eq (char-before (point)) ?\t)
			 (eq (char-before (point)) ?\s))
		     (skip-chars-backward " \t"))
		    (t (progn
			 (while (looking-back "[^[:space:]]" (- (point) 1))
			   (re-search-backward "[^[:space:]]"))
			 (skip-chars-backward " \t"))))))

;; Fix indentation for C mode:
(c-set-offset 'arglist-cont-nonempty '+)
(c-set-offset 'arglist-intro '+)
(c-set-offset 'brace-list-intro '+)
(c-set-offset 'substatement-open '0)

(defconst my-cc-style
  '("cc-mode"
    (c-offsets-alist . ((innamespace . [0])))))

(c-add-style "my-cc-mode" my-cc-style)
(if (eq major-mode 'c++-mode)
    (c-set-style "my-cc-mode"))

;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
(custom-set-faces
 ;; custom-set-faces was added by Custom.
 ;; If you edit it by hand, you could mess it up, so be careful.
 ;; Your init file should contain only one such instance.
 ;; If there is more than one, they won't work right.
 )
