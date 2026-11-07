// Trindle Folders: the file manager, started as `trindle-folders`.

#![allow(clippy::chunks_exact_to_as_chunks, clippy::manual_is_multiple_of)]

mod common;
mod files;
mod host;

use trindle_toolkit as toolkit;

#[used]
#[link_section = ".init_array"]
static TOOLKIT_HOST: extern "C" fn() = host::set;

fn not_traceable() -> bool {
    unsafe { libc::prctl(libc::PR_SET_DUMPABLE, 0, 0, 0, 0) == 0 }
}

fn main() -> std::process::ExitCode {
    common::memory::few_heaps();
    let mut args: Vec<String> = std::env::args().collect();
    let called = args.first().map(|a| a.rsplit('/').next().unwrap_or(a).to_string()).unwrap_or_default();
    let part = if called == "trindle" && args.len() > 1 { format!("trindle-{}", args.remove(1)) } else { called };
    let code = match part.as_str() {
        "trindle-unarchive" => files::unarchive::main(args),
        "trindle-draw-svg" => files::walled::svg_main(args),
        files::askpass::NAME => {
            if !not_traceable() {
                eprintln!("{part}: this system would not mark it as closed to other programs");
            }
            files::askpass::main(args)
        }
        _ => files::main(args),
    };
    std::process::ExitCode::from(code as u8)
}
