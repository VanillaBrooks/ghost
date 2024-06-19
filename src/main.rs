mod cli;
mod prelude;

use prelude::*;

fn main() -> Result<()> {
    let args = cli::Args::parse();

    match args.subcommand {
        cli::Command::Build(b) => build(args.name, args.runtime, b)?,
        cli::Command::Run(r) => run(args.name, args.runtime, r)?,
        cli::Command::Exec(e) => exec(args.name, args.runtime, e)?,
        cli::Command::Stop(s) => stop(args.name, args.runtime, s)?,
    };

    Ok(())
}

fn shell() -> Result<Shell> {
    xshell::Shell::new().with_context(|| "failed to create shell, this should never happen")
}

fn terminal(is_bash: bool) -> &'static str {
    if is_bash {
        "bash"
    } else {
        "fish"
    }
}

fn determine_runtime(sh: &Shell, runtime: cli::ContainerRuntime) -> Result<&'static str> {
    let quiet_cmd = |name| {
        cmd!(sh, "{name} --help")
            .quiet()
            .ignore_stdout()
            .ignore_stderr()
    };

    // check if podman is available in $PATH
    let is_podman_available = quiet_cmd("podman").run().is_ok();

    // check if docker is available in $PATH
    let is_docker_available = quiet_cmd("docker").run().is_ok();

    match runtime {
        cli::ContainerRuntime::Podman => {
            if is_podman_available {
                Ok("podman")
            } else {
                anyhow::bail!(
                    "`podman` selected as container runtime, but does not exist in $PATH"
                );
            }
        }
        cli::ContainerRuntime::Docker => {
            if is_docker_available {
                Ok("docker")
            } else {
                anyhow::bail!(
                    "`docker` selected as container runtime, but does not exist in $PATH"
                );
            }
        }
        cli::ContainerRuntime::Auto => {
            if is_podman_available {
                Ok("podman")
            } else if is_docker_available {
                Ok("docker")
            } else {
                anyhow::bail!("neither `podman` or `docker` found in local environment")
            }
        }
    }
}

fn stop(name: String, runtime: cli::ContainerRuntime, args: cli::Stop) -> Result<()> {
    let sh = shell()?;
    let rt = determine_runtime(&sh, runtime)?;

    cmd!(sh, "{rt} stop {name} --time 1").run().ok();
    cmd!(sh, "{rt} rm {name}").run()?;

    Ok(())
}

fn build(name: String, runtime: cli::ContainerRuntime, args: cli::Build) -> Result<()> {
    let sh = shell()?;
    let rt = determine_runtime(&sh, runtime)?;

    cmd!(sh, "{rt} build . -t {name}").run()?;

    Ok(())
}

fn run(name: String, runtime: cli::ContainerRuntime, args: cli::Run) -> Result<()> {
    let sh = shell()?;
    let rt = determine_runtime(&sh, runtime)?;

    let entrypoint = terminal(true);

    let mut cmd = cmd!(sh, "{rt} run -itd --name {name}");

    for volume in args.volumes {
        let cli::Mount { host, container } = volume;
        cmd = cmd.args(&["-v".into(), format!("{host}:{container}")]);
    }

    for port in args.ports {
        let cli::Mount { host, container } = port;
        cmd = cmd.args(&["-p".into(), format!("{host}:{container}")]);
    }

    cmd.args(&[&name, entrypoint]).run()?;

    Ok(())
}

fn exec(name: String, runtime: cli::ContainerRuntime, args: cli::Exec) -> Result<()> {
    let sh = shell()?;
    let rt = determine_runtime(&sh, runtime)?;
    let terminal = terminal(args.bash);

    let mut child = std::process::Command::new(rt)
        .args(&["exec", "-it", &name, &terminal])
        .stdin(std::process::Stdio::inherit())
        .stdout(std::process::Stdio::inherit())
        .stderr(std::process::Stdio::inherit())
        .spawn()
        .with_context(|| format!("failed to execute `{rt} exec -it {name} {terminal}`"))?;

    child
        .wait()
        .with_context(|| "exec process exited with error")?;

    Ok(())
}
