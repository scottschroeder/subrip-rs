use std::{env, io, io::Read, path::Path};

fn read_file<P: AsRef<Path>>(tpath: P) -> Result<String, io::Error> {
    let tpath = tpath.as_ref();
    let mut f = std::fs::File::open(tpath)?;
    let mut v = Vec::new();
    f.read_to_end(&mut v)?;

    Ok(match String::from_utf8(v) {
        Ok(s) => s,
        Err(e) => {
            let v = e.into_bytes();
            // SRT files are WINDOWS_1252 by default, but there is no requirement, so who knows
            let (text, encoding, replacements) = encoding_rs::WINDOWS_1252.decode(v.as_slice());
            if replacements {
                eprintln!(
                    "could not decode {:?} accurately with {}",
                    tpath,
                    encoding.name()
                );
            }
            text.to_string()
        }
    })
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let get_arg = |n: usize| {
        args.get(n)
            .ok_or_else(|| format!("could not get arg #{}", n))
    };

    let filepath = get_arg(1).expect("need filepath to .srt");
    let contents = read_file(filepath).expect("could not read file");

    let mut subs = match subrip::parse(&contents) {
        Ok(subs) => subs,
        Err(subrip::Error::ParseError(subs, bad_offset)) => {
            let junk = std::str::from_utf8(&contents.as_bytes()[bad_offset..]).unwrap();
            eprintln!("found unexpected text and end of file: {:?}", junk);
            subs
        }
        Err(e) => {
            panic!("failure to parse file: {:?}", e)
        }
    };
    subrip::utils::sort_subtitles(&mut subs);
    for out_of_order in subrip::utils::out_of_order_subs(&subs) {
        eprintln!("found subtitle in the wrong order: {:?}", out_of_order);
    }

    for sub in &subs {
        println!("{:?}", sub);
    }
}
