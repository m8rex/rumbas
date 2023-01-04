# Installation

## Manual Installation (Advanced Usage)

It is possible to use rumbas without using docker, but this implies some manual installation of software.

### Prerequisites

#### Python 3
Python 3 should be installed and added to the `PATH`

Running `python --version` in the terminal / command prompt should yield a version bigger than 3.

#### The Numbas codebase
`rumbas` uses `Numbas` to compile the exams to html. 

Clone / Download the numbas code from [Github](https://github.com/Numbas/numbas) and place it in a folder on your machine.

Set the environment variable `NUMBAS_FOLDER` to the path of this new folder.

#### The Numbas extensions

You will need to download all extensions that you want to use and place them in the `extensions` folder of the `NUMBAS_FOLDER`.

You can find most extensions at [Github](https://github.com/numbas).

It is important that you name the folder of the extension exactly as they are named in `rumbas`.

You can always look at the [Dockerfile](https://github.com/m8rex/rumbas/blob/main/Dockerfile#L197) of `rumbas` to see how the naming should happen and where you can find the extensions.

#### The Numbas themes

The themes that are being used in the `rumbas` repo, should be copied to the `themes` folder in the `NUMBAS_FOLDER`.

The Dockerfile uses [this script](https://github.com/m8rex/rumbas/blob/main/entrypoint.sh) to make sure that the themes are correct.

#### The rumbas binary

In the future, you might find these binaries on the `rumbas` [Github Releases](https://github.com/m8rex/rumbas/releases).

For now, you will need to build the binary yourself.

##### Building the binary

- Install the rust
- Execute `cargo build --release` in the `rumbas` folder.

You can also use `cargo install --path .` in the rumbas folder to build and install the binary.
