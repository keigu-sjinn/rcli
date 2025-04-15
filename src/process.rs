use crate::subcmd::CsvArgs;

pub fn process_csv(opts: CsvArgs) {
    // TODO: HANDLER CSV FILE
    for input in opts.input {
        println!("{}", input);
    }
}
