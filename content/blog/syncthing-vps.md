+++
title = "Running Syncthing on a VPS"
date = 2023-05-30
description = "A guide on setting up a syncthing node on a VPS on the command line."
[taxonomies]
tags = ["self-hosting"]
+++

I've been going down the self-hosting rabbit hole recently and decided to try out [Syncthing](https://syncthing.net/) as a way of syncing notes across my devices. I decided to run a node on a VPS so that I would have one node running 24/7 that my devices can sync to. This guide will help you set up Syncthing on a VPS and secure it using [ufw](https://launchpad.net/ufw) and [fail2ban](https://www.fail2ban.org/wiki/index.php/Main_Page). This guide will only use the command line (no web GUI).

## Installing

The installation process varies depending on which OS you are running on the server but most common Linux distros should have a version of Syncthing packaged in the default repositories. We are also going to install `fail2ban` for security.

For Ubuntu or Debian server:

```sh
$ sudo apt install syncthing fail2ban
```

For Arch:

```sh
$ sudo pacman -S syncthing ufw fail2ban
```

For Fedora:

```sh
$ sudo dnf install syncthing ufw fail2ban
```

## Security

### Disclaimer

I am not a security expert. Do your own research before following these instructions.

### Enable `fail2ban`

All you have to do to enable fail2ban is run

```sh
$ systemctl enable fail2ban
$ systemctl start fail2ban
```

### Opening ports on VPS

The ports we are going to need open for Syncthing are 22000 and 21027/udp. Your VPS provider should have instructions on opening ports.

### Setting up `ufw`

`ufw` (uncomplicated firewall) is an easy-to-use firewall tool that we will use to drop all incoming traffic on ports other than the Syncthing ports. We can allow traffic on Syncthing ports by running

```sh
$ sudo ufw allow syncthing
```

If you are using SSH to access your VPS it is also important that your open the SSH port so you don't get locked out. To do this, run `sudo ufw allow ssh`.

## Configuration

To generate a Syncthing config on the server, run

```sh
$ syncthing generate
```

 This should output a device ID, make sure to save this for later. I will refer to this as `SERVER_ID` from now on.

### Connecting a device

These next instructions will show you how to connect a device, such as a laptop or desktop computer, to the server. First, make sure Syncthing is running on both the device and the server. Start Syncthing on the server by running the following two commands:

```sh
$ systemctl enable syncthing@USER.service
$ systemctl start syncthing@USER.service
```

where `USER` is replaced by your username.

First, we will need the device ID of the device. Run `syncthing -device-id` to get it. I will refer to this as `LOCAL_ID` from now on. On your local device, run

```sh
$ syncthing cli config devices add --device-id LOCAL_ID
```

and on your local device run

```sh
$ syncthing cli config devices add --device-id SERVER_ID
```

To configure the server to auto-accept folders from the local device, run the following command on the server:

```sh
$ syncthing cli config devices LOCAL_ID auto-accept-folders set true
```

Now you can add a folder from your local device. On the local device, run

```sh
$ syncthing cli config folders list
```

and copy the folder ID of the folder you want to add. Then run

```sh
$ syncthing cli config folders FOLDER_ID devices add --device-id SERVER_ID
```

to add the folder.

## Credits

- Thanks to [jonny-exe](https://gist.github.com/Jonny-exe) for creating [this gist](https://gist.github.com/Jonny-exe/9bad76c3adc6e916434005755ea70389).
- [This superuser post](https://superuser.com/questions/1397683/how-can-i-configure-syncthing-from-command-line-to-share-a-folder-with-another-c/1731999#1731999?s=0744928a5f9d4717b7445d039785ba53) was also great.
