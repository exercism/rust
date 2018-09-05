#[macro_use]
extern crate structopt;
use structopt::StructOpt;

extern crate fetch_configlet;
use fetch_configlet::*;

extern crate failure;
use failure::Error;

#[derive(Debug, StructOpt)]
#[structopt(name = "fetch-configlet")]
struct Opt {
    /// print operating system
    #[structopt(short = "o", long = "os")]
    print_os: bool,

    /// print target architecture
    #[structopt(short = "a", long = "arch")]
    print_arch: bool,

    /// print latest version
    #[structopt(short = "l", long = "latest")]
    print_latest: bool,

    /// print tarball url
    #[structopt(short = "t", long = "tarball")]
    print_tarball: bool,

    /// print path to binary directory
    #[structopt(short = "b", long = "binpath")]
    print_binpath: bool,

    /// do not actually download configlet
    #[structopt(short = "n", long = "no-download")]
    no_download: bool,
}

fn main() -> Result<(), Error> {
    let opt = Opt::from_args();
    if opt.print_os {
        println!("os:      {}", target_os());
    }
    if opt.print_arch {
        println!("arch:    {}", target_arch());
    }
    if opt.print_latest {
        println!("latest:  {}", get_latest()?);
    }
    if opt.print_tarball {
        println!("tarball: {}", pkg_url()?);
    }
    if opt.print_binpath {
        println!("binpath: {:?}", binpath()?);
    }
    if !opt.no_download {
        download()?;
    }
    Ok(())
}
