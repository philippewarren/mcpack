# McPack

## A modpack package manager for Minecraft

Easily share and update custom Minecraft modpacks.

## Client Usage

```shell
# Initialise a new modpack with the name `name` in the instance directory `dir` using the pack data from `uri`
# If `dir` is not specified, the current directory is used
# If `name` is not specified, the modpack is given a unique identifier
# `uri` can be:
#     - The URL of a Git repository containing the pack data
#     - A file path to a local pack data archive file
#     - A directory path to a local pack data directory
mcpack init <uri> [name] [dir]

# Show the status about the modpack `name`
# If `name` is not specified, the current directory is used if it is a modpack
mcpack status [name]

# Update the modpack `name` to the latest version
# If `name` is not specified, the current directory is used if it is a modpack
# If `version` is specified, use that version instead of the latest, if available
mcpack update [name] [version]

# Shows the differences between the modpack `name` current install and the latest version
# If `name` is not specified, the current directory is used if it is a modpack
# If `--from` is specified, the this is taken instead of the current version
# If `--to` is specified, the this is taken instead of the latest version
mcpack changelog [name] [--from f_version] [--to t_version]

# Change data for a modpack `name`
# If `name` is not specified, the current directory is used if it is a modpack
mcpack edit [name] [--name new_name] [--uri new_uri] [--dir new_dir]

# Shows the help for mcpack, or for the command `command` if specified
mcpack help [command]

# Upgrade `mcpack` to the latest version
mcpack --upgrade
```

## Developer Usage

```shell
mcpack dev init [name] [dir]

mcpack dev status [name]

mcpack dev export [name] [-M] [-m] [-p]
```

## Modpack Data Format

### `mcpack.toml` file

Represents the metadata about a modpack, like the author, name, version, and description.
Also contains a list of URI from which to download the mods.
Also includes a list of included files or folders, like data packs, textures, config files, CraftTweaker scripts, etc.

```toml
[pack]
name = "My Modpack"
author = "My Name"
version = "1.0.0"
description = "My Modpack Description"
minecraft_version = "1.18.2"
mod_loader = "forge"
forge_version.minimum = "24.5.254"
forge_version.recommended = "24.5.256"

[mods]
create = "https://[...]"
jei = "https://[...]"
appleskin = "https://[...]"

[files]
include = [
    "config/*",
    "scripts/*",
]
exclude = [
    "config/cofh/client.cfg",
]
```

### `data` directory

The `data` directory contains the modpack data.
This includes all files and folders listed in the `files.include` section of the `mcpack.toml` file, with the exception
of the files listed in the `files.exclude` section.

It is structured as is the root of a Minecraft instance folder, which means that a folder `data/config` will end up
as `.minecraft/config`.

## Licence and Copyright

Copyright (c) 2022 Philippe Warren (philippewarren)\
Liencesed under the Apache 2.0 License

See the full license in the [licence file](LICENCE).
