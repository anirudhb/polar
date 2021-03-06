use clap::Parser;
use minijinja::Environment;
use std::path::PathBuf;

mod html;

#[derive(Parser)]
#[clap(
    version = "0.1",
    author = "anirudhb <anirudhb@users.noreply.github.com>"
)]
struct Opts {
    #[clap(short, long, default_value = "out")]
    outdir: PathBuf,
    filename: PathBuf,
}

struct Polar<'a> {
    env: Environment<'a>,
    outdir: PathBuf,
}

/* Convert markdown to html */
fn htmlify(markdown: &str) -> String {
    use pulldown_cmark::{Options, Parser};
    let mut options = Options::empty();
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_TASKLISTS);
    options.insert(Options::ENABLE_STRIKETHROUGH);
    let parser = Parser::new_ext(markdown, options);
    let mut output = Vec::new();
    let mut writer = html::HtmlWriter::new(parser, &mut output);
    // safe to unwrap since output is a Vec
    writer.run().unwrap();
    String::from_utf8(output).unwrap()
}

fn main() -> eyre::Result<()> {
    let opts = Opts::parse();
    let mut polar = Polar {
        env: Environment::new(),
        outdir: opts.outdir,
    };

    println!("Filename = {:?}", opts.filename);
    let contents = std::fs::read_to_string(&opts.filename)?;
    let out_html = htmlify(&contents);
    println!("HTMLified = {}", out_html);
    polar.env.add_template("main", &out_html)?;
    let out = polar.env.get_template("main")?.render(())?;
    println!("templated = {}", out);

    Ok(())
}
