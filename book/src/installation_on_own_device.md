## Recommended installation (with docker)

### Install docker

- All information about docker can be found on the [docker website](https://www.docker.com/get-started).

- Make sure that it is added to the `PATH` environment variable. This might be a checkbox during the installation.

- Validate the installation by typing `docker ps` in a terminal / command prompt.

### Simplifying docker usage

In principe, installing docker is enough to run `rumbas` but the commands will be cumbersome.

In this section we describe how to create `bat` or `sh` files to make the docker usage of rumbas transparant for the user.

Important: Make sure to explicitly use the latest version of rumbas (or at least a specific version, not `latest`)

### Explanation

In the Windows and Unix sections below, we will explain how you can run rumbas in your terminal / command prompt by just writing `rumbas` or `rumbas_shell`.

- `rumbas`: You will be able to write `rumbas` in the terminal, instead of needing to type the whole docker command with the volume mount.
- `rumbas_shell`: Always starting a container might be a bit to slow and overkill. With this script you can run `rumbas-shell` to get a docker container where you can repeatedly execute rumbas commands. Because of the current implementation of the docker container, it is best to call `/usr/app/entrypoint.sh <path>` instead of calling rumbas directly. Just calling `rumbas` will only work if you don't use custom themes.

#### Windows

We will create a folder `docker_scripts` on the `C` drive and add it to the `PATH` environment variable.
In this folder you can create a file for each of the following two scripts.

##### Creating the `docker_scripts` folder

- Create a folder 'docker_scripts' on the C drive
- Click on the window icon in the left bottom corner
- Search for 'Edit environment variables' and click on it
- Select 'Path' and click 'edit'
- Click on 'New'
- Typ 'C:\docker_scripts'
- Click on 'Ok'
- Open a new terminal so the new PATH variable is set

In this folder you need to create a file for each of the following two scripts.

##### rumbas.bat

Place the following text in the file `rumbas.bat` in the `docker_scripts` folder.

```bat
@echo off
set str=%*
set "str=%str:\=/%"
docker run --rm -v %cd%:/rumbas ghcr.io/m8rex/rumbas:0.7.1 %str%
```

##### rumbas_shell.bat

Place the following text in the file `rumbas_shell.bat` in the `docker_scripts` folder.

```bat
@echo off
docker run -it --rm -v %cd%:/rumbas --entrypoint=sh ghcr.io/m8rex/rumbas:0.7.1
```

#### Unix

We will create a folder `docker_scripts` in `/usr/local/bin`

```
sudo mkdir /usr/local/bin/docker_scripts
```

And add it to the path by adding to following line to the `~/.bashrc` file:

```
export PATH=$PATH:/usr/local/bin/docker_scripts
```

In this folder you can create a file for each of the following two scripts.

##### rumbas

Place the following text in the file `rumbas.bat` in the `docker_scripts` folder.
```sh
#!/bin/sh
docker run --rm -v $PWD:/rumbas ghcr.io/m8rex/rumbas:0.7.1 $@
```

Afterwards execute: `sudo chmod +x /usr/local/bin/docker_scripts/rumbas`

##### rumbas_shell

Place the following text in the file `rumbas_shell.bat` in the `docker_scripts` folder.
```sh
#!/bin/sh
docker run -it --rm -v $PWD:/rumbas --entrypoint=sh ghcr.io/m8rex/rumbas:0.7.1
```
Afterwards execute: `sudo chmod +x /usr/local/bin/docker_scripts/rumbas_shell`


### Explicit docker usage

It is possible to run rumbas in docker without using the `docker_scripts` setup.

- Run `docker run --rm -it -v <absolute_path_to_rumbas_repo>:/rumbas ghcr.io/m8rex/rumbas:0.7.1 rumbas <command>`
  - e.g.`docker run --rm -it -v C:\Users\jesse\Documents\rumbas-examples:/rumbas ghcr.io/m8rex/rumbas:0.7.1 compile exams/M0/algebra/begintest.yaml`

