# ddi
## A safer dd
---
# Introduction

If you ever used `dd`, the GNU coreutil that lets you copy data from one file to another, then you may have encountered a typical use case: burning an iso file to a USB flashdrive.

This is how a typical `dd` command intended for this purpose would look like:

    $ dd if=image.iso of=/dev/sdc

This command would copy all contents of `image.iso` into `/dev/sdc` which we'll assume it's a USB flashdrive in this example. However, it's VERY easy to mess up this command and send the output to a different device:

    $ dd if=image.iso of=/dev/sda

Now all your data is gone and you're in tears. Thankfully, this tool exists, although it won't save you from past mistakes.

`ddi` (or dd-interactive) is a wrapper for `dd`. It takes all the same arguments, and all it really does is call `dd` in the background. But, if a device file is detected, you'll get a warning message detailing some information about what you're about to do as seen here:

![example](img/example.gif)

Any kind of response other than "y" will abort the command. If you use `dd` frequently, you might appreciate not having to worry so much about nuking your entire installation by one typo.

If the command doesn't have the `of` flag, or the `of` flag doesn't direct to a block device, then the warning message is ommited and `dd` will be called without interrumption. This means that `ddi` can be used as a drop-in replacement for `dd` with the added benefit of warning you if you're about to commit a potentially destructive action.

# Installation

`ddi` can be installed form the [AUR](https://aur.archlinux.org/packages/ddi-bin/).

    $ paru -S ddi-bin
    
Optionally, there's pre-compiled linux binaries for download on the [release](https://github.com/tralph3/ddi/releases) section.
