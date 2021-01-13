use colored::*;

const ART: &str = r###"
            }wJ@g@Wg@gg _           
        ,g1@@@W%%gd6%W@@@gg         
       gg@@Q;_________ "%@@gm       
      g@@@@@@@@@@@@@@@@g `@@Ww      
     w@@jFc}@@@L```"@@@@O#Qj@g=     
     g@@O  }@@@@@@@@@@@_   1@Wg     
     g@@C  }@@@L``'1@@@W_ &@@@_     
      l@@@@@@@@@@@  @@@@@@@@@Q      
      `F@@Q)$"????   *TDD@@@U       
        ]@@Qj@_      _@jF@@         
          1%1@@@@@@@@@@HW           
               *`1C'O "             

"###;

fn exc(exc: &str) -> Result<std::process::Output, std::io::Error> {
    let exc: Vec<&str> = exc.split_whitespace().collect();
    let mut cmd = std::process::Command::new(exc[0]);
    cmd.args(&exc[1..exc.len()])
        .output()
}

fn get_ver(cmd: &str) -> String {
    let get_ver = match exc(cmd) {
        Ok(ver) => ver.stdout,
        Err(_) => "not present".as_bytes().to_vec(),
    };
    let mut get_ver = std::str::from_utf8(&get_ver)
        .unwrap()
        .lines();
    match get_ver.next() {
        Some(v) => v.to_string(),
        None => "not present.".to_string(),
    }
}

fn cargo_installs() -> String {
    let cargo_installs = match exc("cargo install --list") {
        Ok(installs) => installs.stdout,
        Err(_) => "not present".as_bytes().to_vec(),
    };
    let cargo_installs = std::str::from_utf8(&cargo_installs)
        .unwrap()
        .lines();
    String::from(format!("{}", cargo_installs.count() / 2))
}

fn print(info: Vec<String>) {
    let mut lines = ART.lines();
    let mut i = 0;
    let empty = String::from("");
    loop {
        match lines.next() {
            Some(line) => {
                println!("{}{}", 
                         line.red(), 
                         match i <= info.len() - 1 {
                             true => {
                                 i = i + 1;
                                 &info[i - 1]
                             },
                             false => &empty,
                        });
            },
            None => break,
        }
    }
}

fn main() {
    let rust_ver = get_ver("rustc -vV");
    let sys_info = sys_info::linux_os_release()
        .unwrap();
    let mem = sys_info::mem_info()
        .unwrap();
    let mem_used = (mem.total - mem.avail) / 1024;
    let mem_total = mem.total / 1024;
    let mut info: Vec<String> = vec![];

    info.push("".to_string());
    info.push(format!("{}{}{}", whoami::username().bold(), "@".bright_purple().bold(), whoami::hostname().bold()));
    info.push(format!("{}", "━━━━━━━━━━━━━━━".blue().bold()));
    info.push(format!("{}    {}", "rust version".purple(), rust_ver));
    info.push(format!("{}   {}", "cargo version".purple(), get_ver("cargo -vV")));
    info.push(format!("{}  {}", "rustup version".purple(), get_ver("rustup -vV")));
    let rust_ver: Vec<&str> = rust_ver.split(" ").collect();
    let rust_ver: &str = rust_ver[1];
    let chan = match semver::Version::parse(rust_ver) {
        Ok(ver) => {
            match ver.pre.first() {
                None => "stable",
                Some(semver::Identifier::AlphaNumeric(s)) => &s,
                Some(_) => "unknown",
            }.to_string()
        },
        Err(_) => "not present".to_string(),
    };
    info.push(format!("{}    {}", "rust channel".purple(), chan));
    info.push(format!("{}  {}", "cargo installs".purple(), cargo_installs()));
    info.push(format!("{}              {}", "os".purple(), match sys_info.pretty_name {
                                                                Some(os) => os,
                                                                None => "".to_string(),
    }));
    info.push(format!("{}         {}", "release".purple(), sys_info::os_release().unwrap()));
    info.push(format!("{}          {}MiB/{}MiB ({:.1}%)", "memory".purple(), mem_used, mem_total, (mem_used as f32 / mem_total as f32) * 100.0));
    info.push("".to_string());
    info.push(format!("{}{}{}{}{}{}{}{}", "███".black(), "███".red(), "███".green(), "███".yellow(), "███".blue(), "███".magenta(), "███".cyan(), "███".white()));

    print(info);
}
