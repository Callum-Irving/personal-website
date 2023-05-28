+++
title = "Getting Started with Yambar"
date = 2023-05-19
description = "A guide on using and configuring Yambar."
+++

[Yambar](https://codeberg.org/dnkl/yambar) is a status bar compatible with X11 and Wayland. I choose to use Yambar because of its configurability.

# Installing

I recommend installing Yambar through your distro's package manager if possible. You can go to the Yambar repo to find out if it's packaged for your distro. If not, there are instructions for building from source in the readme.

# Running

To start Yambar, all you need to do is run the command:

```
$ yambar
```

# Configuring

All of the configuration is handled in one YAML file, located at `XDG_CONFIG_HOME/yambar/config.yml`

The man pages contain all the info you will need to write a config.