# ghost

Helper scripts to build, run, and execute containers


## Building / Installing


You will need a recent version of the rust compiler and `cargo`. To install:

```bash
cargo install --git https://github.com/VanillaBrooks/ghost.git --force
```


## Usage


### Build an Image

To build a `Dockerfile` in your current working directory:

```
ghost foo build
```

then check the image exists with `podman`:

```
podman images
```

output:


```
REPOSITORY                                    TAG          IMAGE ID      CREATED        SIZE
localhost/foo                                 latest       a589c40e9506  3 days ago     388 MB
```

### Run a Container

```
ghost foo run -v path/to/local/dir:/path/to/mount/dir -p 8080:8080
```

This runs a container named `foo` (assumes there is an image previously built with name `foo`),
mounts a directory into the container, and opens a port into the container.


```
podman ps
```

```
CONTAINER ID  IMAGE                 COMMAND     CREATED         STATUS         PORTS       NAMES
2a2d7071bd32  localhost/foo:latest  bash        27 minutes ago  Up 27 minutes              foo
```


### Executing a docker

```
ghost foo exec
```

This will open a shell (defaults to `fish`) inside the container.


## CLI Reference

```
ghost --help

Usage: ghost [OPTIONS] <NAME> <COMMAND>

Commands:
  build
  run
  exec
  stop
  help   Print this message or the help of the given subcommand(s)

Arguments:
  <NAME>


Options:
  -r, --runtime <RUNTIME>
          [default: auto]
          [possible values: podman, docker, auto]

  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version
```

```
ghost build --help
Usage: ghost <NAME> build

Options:
  -h, --help  Print help
```

```
ghost run --help
Usage: ghost <NAME> run [OPTIONS]

Options:
  -v, --volume <VOLUMES>  volumes to mount to the container
  -p, --port <PORTS>      ports to mount to the container
  -h, --help              Print help
```

```
ghost exec --help
Usage: ghost <NAME> exec [OPTIONS]

Options:
  -b, --bash  use bash insertion instead of fish
  -h, --help  Print help
```
