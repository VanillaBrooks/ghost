mod cli;
mod prelude;

use prelude::*;

fn main() -> Result<()> {
    let args = cli::Args::parse();

    match args.subcommand {
        cli::Command::Build(b) => build(args.name, b)?,
        cli::Command::Run(r) => run(args.name, r)?,
        cli::Command::Exec(e) => exec(args.name, e)?,
        cli::Command::Stop(s) => stop(args.name, s)?,
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

fn runtime(sh: &Shell) -> Result<&'static str> {
    let quiet_cmd = |name| {
        cmd!(sh, "{name} --help")
            .quiet()
            .ignore_stdout()
            .ignore_stderr()
    };

    // check if podman exists
    if quiet_cmd("podman").run().is_ok() {
        Ok("podman")
    } else if quiet_cmd("docker").run().is_ok() {
        Ok("docker")
    } else {
        anyhow::bail!("neither `podman` or `docker` found in local environment")
    }
}

fn stop(name: String, args: cli::Stop) -> Result<()> {
    let sh = shell()?;
    let rt = runtime(&sh)?;

    cmd!(sh, "{rt} stop {name} --time 1").run().ok();
    cmd!(sh, "{rt} rm {name}").run()?;

    Ok(())
}

fn build(name: String, args: cli::Build) -> Result<()> {
    let sh = shell()?;
    let rt = runtime(&sh)?;

    cmd!(sh, "{rt} build . -t {name}").run()?;

    Ok(())
}

fn run(name: String, args: cli::Run) -> Result<()> {
    let sh = shell()?;
    let rt = runtime(&sh)?;

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

fn exec(name: String, args: cli::Exec) -> Result<()> {
    let sh = shell()?;
    let rt = runtime(&sh)?;
    let terminal = terminal(args.bash);

    let mut child = std::process::Command::new(rt)
        .args(&["exec", "-it", &name, &terminal])
        .stdin(std::process::Stdio::inherit())
        .stdout(std::process::Stdio::inherit())
        .stderr(std::process::Stdio::inherit())
        .spawn()
        .with_context(|| format!("failed to execute `{rt} exec -it {name} {terminal}`"))?;

    child.wait().with_context(|| "exec process exited with error")?;

    Ok(())
}
