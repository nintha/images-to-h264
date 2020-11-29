use images_to_h264::image_x264;
use clap::Clap;

#[derive(Clap, Debug)]
#[clap(version = "0.1", author = "Ninthakeey <ninthakeey@hotmail.com>")]
struct Opts {
    #[clap(about="image file template name, '%d' is variable value, eg: image%d.png")]
    input: String,
    #[clap(long, default_value = "out.h264", about="output filename")]
    output: String,
    #[clap(long, default_value = "30")]
    fps: u32,
    #[clap(long, short, default_value = "640")]
    width: u32,
    #[clap(long, short, default_value = "360")]
    height: u32,
    #[clap(long, default_value = "0", about="input %d start num")]
    start_num: u32,
}

fn main() -> anyhow::Result<()> {
    let opts: Opts = Opts::parse();
    println!("{:?}", &opts);
    image_x264::gen_h264(
        opts.fps,
        opts.width,
        opts.height,
        &opts.input,
        opts.start_num,
        &opts.output,
    )?;
    Ok(())
}