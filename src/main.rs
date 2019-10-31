use std::ffi::OsStr;
use tectonic::BibtexEngine;
use tectonic::engines::NoopIoEventBackend;
use tectonic::io::stack::IoStack;
use tectonic::status::NoopStatusBackend;
use tectonic::io::memory::MemoryIo;

fn create_io_environment() -> MemoryIo {
    let mut ret = MemoryIo::new(true);
    ret.create_entry("min.bst".as_ref(), include_bytes!("min.bst").to_vec());
    ret.create_entry("min.bib".as_ref(), include_bytes!("min.bib").to_vec());
    ret.create_entry("min.aux".as_ref(), include_bytes!("min.aux").to_vec());
    ret
}

fn run_bibtex() {
    let mut io = create_io_environment();
    let mut io_stack = IoStack::new(vec![&mut io]);
    let mut engine = BibtexEngine::new();

    let _ = engine.process(
        &mut io_stack,
        &mut NoopIoEventBackend::new(),
        &mut NoopStatusBackend::new(),
        "min.aux",
    );

    let stdout_data = String::from_utf8(io.files.borrow()[io.stdout_key()].clone()).unwrap();
    println!("{}", stdout_data);

    let bbl_data = String::from_utf8(io.files.borrow()[OsStr::new("min.bbl")].clone()).unwrap();
    println!("{}", bbl_data);
}

fn main() {
    run_bibtex();
    run_bibtex();
}
