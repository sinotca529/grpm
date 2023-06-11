# grpm : GitHub Release Based Package Manager
This tool manage projects which released by GitHub based on release tag.

**This project is not implemented yet**

## Settings
`~/.config/grpm/grpm.toml` is the file to write settings.

### Information of packages
```toml
[[packages]]
name = "nvim"
repo = "neovim/neovim"
bin  = "nvim.appimage"
sign = "nvim.appimage.sha256sum"

[[packages]]
name = "tmux"
repo = "tmux/tmux"
bin  = "tmux-*.tar.gz"
```

- `repo`
    - Name of repository.

- `bin`
    - Name pattern of binary file.
    - If the file is compressed, it will automatically be decompressed.
- `sign`
    - Name pattern of signature file.
    - The type of signature is detected from file name.
