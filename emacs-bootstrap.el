;; Setup for package manager
(require 'package)
(add-to-list 'package-archives
             '("melpa" . "https://melpa.org/packages/") t)
(package-initialize)
(package-refresh-contents)

;; Install packages I care about
(package-install 'rust-mode)
(package-install 'dockerfile-mode)
