(let ((project-root "."))  ; Replace with your project root
    (setq eglot-workspace-configuration
          `((:rust-analyzer . ((workspace . ((root . ,project-root))))))))
