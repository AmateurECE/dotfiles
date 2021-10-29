;; Setup for package manager
(require 'package)
(add-to-list 'package-archives
             '("melpa" . "https://melpa.org/packages/") t)
(package-initialize)
(package-refresh-contents)

;; Install packages I care about
(package-install 'rust-mode)
(package-install 'dockerfile-mode)
(package-install 'markdown-mode)
(package-install 'bitbake)
(package-install 'ess)
(package-install 'meson-mode)
