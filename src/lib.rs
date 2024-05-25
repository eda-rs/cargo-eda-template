use clap::{Command,Arg,ArgAction};
use log::info;

pub enum InAppError {
	#[error("data store disconnected")]
	Error0,
	#[error("yet another error `{0}`")]
	Error1(String),
	#[error("error chained from std::io::Error")]
	Error2(#[from] std::io::Error),

}

pub fn function1() {
	info!("enter function1")
}


pub fn entry() {
    let logfile = FileAppender::builder()
        .encoder(Box::new(PatternEncoder::new("{l} - {m}\n")))
        .build("output.log").unwrap();

    let config = Config::builder()
        .appender(Appender::builder().build("logfile", Box::new(logfile)))
        .build(Root::builder()
                   .appender("logfile")
                   .build(LevelFilter::Info)).unwrap();

    log4rs::init_config(config).unwrap();

	let m = Command::new("demo_cmd")
	    .author("Me, me@mail.com")
	    .version("1.0.2")
	    .about("Explains in brief what the program does")
	    .arg(Arg::new("arg0")
	        .short('h')
	        .long("argument0"))
	    .arg(Arg::new("arg1")
	        .long("argument1")
	        .global(true)
	        .action(ArgAction::Help))
	    .after_help("Longer explanation to appear after the options when \
	                 displaying the help information from --help or -h")
	    .get_matches();   


	    function1() 
}