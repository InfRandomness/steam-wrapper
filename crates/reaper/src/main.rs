use std::collections::BTreeMap;
use std::env;
use std::process::Command;

fn main() {
    let env = env::vars_os().collect::<BTreeMap<_, _>>();

    let env = env
        .into_iter()
        .map(|(key, value)| format!("{key:?} -> {value:?}"))
        .collect::<Vec<_>>();

    let env = env.join("\n");

    println!("{env}");

    let args = env::args_os()
        .map(|arg| format!("{arg:?}"))
        .collect::<Vec<_>>();

    let args = args.join(", ");

    println!("{args}");

    let args = env::args_os().collect::<Vec<_>>();
    let skip = args.iter().position(|arg| arg == "--").unwrap_or(0) + 1;
    let rest = args.iter().skip(skip).collect::<Vec<_>>();
    let program = rest.first().unwrap();
    let args = rest.iter().skip(1);

    println!(" ! program = {program:?}");

    let mut child = Command::new(program).args(args).spawn().expect("run game");

    child.wait().expect("wait game");
}
