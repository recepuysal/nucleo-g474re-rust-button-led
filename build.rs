//! Linker scripts: cortex-m-rt (link.x, memory.x from embassy-stm32 `memory-x`) and defmt (defmt.x).

fn main()
{
    println!("cargo:rustc-link-arg-bins=--nmagic");
    println!("cargo:rustc-link-arg-bins=-Tlink.x");
    println!("cargo:rustc-link-arg-bins=-Tdefmt.x");
}
