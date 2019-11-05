#![doc(html_playground_url = "https://play.rust-lang.org/")]

//! Work in Progress - not usable yet!
//! 
//! # cli-bp
//! `cli-bp` provides the fairly simple boilerplate template using [`clap::App`] and log/simplelog to create a starting point for command line applications
//! 
//! This gives you a quick and dirty start to a command line applications with built in / starting support for command line / config parsing as 
//! well as logging.  You have full access to clap and log/simplelog for changing the defaults through their apis.
//! 
//! # Simple Example
//! 
//! ```
//! use cli_bp::*;
//! use std::fmt;
//! 
//! fn main() {
//!     let clibp = CliBp::new("myapp")
//!                             .version("1.0")
//!                             .author("Me <me@gmail.com>")
//!                             .about("Does awesome things")
//!                             .get_matches();
//!     clibp.init_logging(); 
//! }
//! 
use std::fs::OpenOptions;
use std::fs;
use std::path::Path;
use simplelog::*;
use log::*;


pub struct CliBp<'a,'b> {
    pub app: clap::App<'a,'b>,
    pub matches: clap::ArgMatches<'a>,
}

impl<'a, 'b> CliBp<'a, 'b> {
    pub fn new<S: Into<String>>(n: S) -> Self {
        let app = clap::App::new(n)
                    .version(clap::crate_version!())
                    .arg(clap::Arg::with_name("config_dir")
                        .short("c")
                        .long("config_dir")
                        .value_name("DIR")
                        .help("Sets a custom config directory")
                    );
        let matches = clap::ArgMatches::new();
        CliBp {
            app,
            matches,
        }
    }
    pub fn get_matches(mut self) -> Self {
        let name = self.app.get_name().to_string();
        self.matches = self.app.get_matches();
        self.app = clap::App::new(name)
                        .version(clap::crate_version!())
                        .arg(clap::Arg::with_name("config_dir")
                            .short("c")
                            .long("config_dir")
                            .value_name("DIR")
                            .help("Sets a custom config directory")
                    );
        self
    }

    pub fn init_logging(&self) {
        let appname = self.app.get_name();
        let default_config = format!("{}/.{}", env!("HOME"), appname);
        let mut created_dir = false;
        let config_dir = self.matches.value_of("config_dir")
                .unwrap_or(&default_config);
        if !(Path::new(&config_dir).exists()) { 
            fs::create_dir_all(&config_dir).unwrap();
            created_dir = true;
        }
        CombinedLogger::init(vec![
            TermLogger::new(LevelFilter::Debug, Config::default(), TerminalMode::Mixed).unwrap(),
            WriteLogger::new(
                LevelFilter::Info,
                Config::default(),
                OpenOptions::new()
                    .create(true)
                    .append(true)
                    .open(format!("{}/{}.log", config_dir, appname))
                    .unwrap(),
            ),
        ])
        .unwrap();
        if let Some(version) = self.matches.value_of_lossy("version") {
            debug!("Logging started for v{} of {}", version, self.app.get_name());
        } else {
            debug!("Logging started for {}", self.app.get_name());
        }
        if created_dir {
            info!("Created new config directory: {}", config_dir);
        }
    }

    /// Sets the [`clap::App::author()`]
    pub fn author<S: Into<&'b str>>(mut self, author: S) -> Self {
        self.app.p.meta.author = Some(author.into());
        self
    }

    /// Sets the [`clap::App::bin_name()`]
    pub fn bin_name<S: Into<String>>(mut self, name: S) -> Self {
        self.app.p.meta.bin_name = Some(name.into());
        self
    }

    /// Sets the [`clap::App::about()`]
    pub fn about<S: Into<&'b str>>(mut self, about: S) -> Self {
        self.app.p.meta.about = Some(about.into());
        self
    }

    /// Sets the [`clap::App::long_about()`]
    pub fn long_about<S: Into<&'b str>>(mut self, about: S) -> Self {
        self.app.p.meta.long_about = Some(about.into());
        self
    }

    /// Sets the [`clap::App::name()`]
    pub fn name<S: Into<String>>(mut self, name: S) -> Self {
        self.app.p.meta.name = name.into();
        self
    }
    
    /// Sets the [`clap::App::after_help()`]
    pub fn after_help<S: Into<&'b str>>(mut self, help: S) -> Self {
        self.app.p.meta.more_help = Some(help.into());
        self
    }
    
    /// Sets the [`clap::App::before_help()`]
    pub fn before_help<S: Into<&'b str>>(mut self, help: S) -> Self {
        self.app.p.meta.pre_help = Some(help.into());
        self
    }
    
    /// Sets the [`clap::App::version()`]
    pub fn version<S: Into<&'b str>>(mut self, ver: S) -> Self {
        self.app.p.meta.version = Some(ver.into());
        self
    }
    
    /// Sets the [`clap::App::long_version()`]
    pub fn long_version<S: Into<&'b str>>(mut self, ver: S) -> Self {
        self.app.p.meta.long_version = Some(ver.into());
        self
    }
    
    /// Sets the [`clap::App::usage()`]
    pub fn usage<S: Into<&'b str>>(mut self, usage: S) -> Self {
        self.app.p.meta.usage_str = Some(usage.into());
        self
    }
    
    /// Sets the [`clap::App::help()`]
    pub fn help<S: Into<&'b str>>(mut self, help: S) -> Self {
        self.app.p.meta.help_str = Some(help.into());
        self
    }

    // wrong reference - See [`clap::App::help_short()`](../clap/struct.Arg.html#method.short)
    /// Sets the [`clap::App::help_short()`]    
    pub fn help_short<S: AsRef<str> + 'b>(mut self, s: S) -> Self {
        self.app.p.help_short(s.as_ref());
        self
    }
    
    /// Sets the [`clap::App::version_short()`]
    pub fn version_short<S: AsRef<str>>(mut self, s: S) -> Self {
        self.app.p.version_short(s.as_ref());
        self
    }

    /// Sets the [`clap::App::help_message()`]
    pub fn help_message<S: Into<&'a str>>(mut self, s: S) -> Self {
        self.app.p.help_message = Some(s.into());
        self
    }

    /// Sets the [`clap::App::version_message()`]
    pub fn version_message<S: Into<&'a str>>(mut self, s: S) -> Self {
        self.app.p.version_message = Some(s.into());
        self
    }

}