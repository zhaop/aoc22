use std::{io::{stdin, Read}, collections::HashMap};

fn parent_path(path: &str) -> String {
    let mut parts = path.split("/").collect::<Vec<_>>();
    parts.pop();
    return parts.join("/");
}

pub fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut fs: HashMap<String, u32> = HashMap::new();
    let mut pwd = String::new();
    input.split("$ ").skip(1).for_each(|interaction| {
        let mut lines = interaction.strip_suffix("\n").unwrap().split("\n");
        let cmd = lines.next().unwrap();
        if cmd == "cd /" {
            pwd = String::new();
        } else if cmd == "cd .." {
            pwd = parent_path(&pwd);
        } else if cmd.starts_with("cd ") {
            pwd = format!("{}/{}", pwd, cmd.split(" ").last().unwrap());
        } else if cmd == "ls" {
            let size =
                lines
                .map(|ln| ln.split(" ").next().unwrap())
                .filter(|size| *size != "dir")
                .map(|size| size.parse::<u32>().unwrap())
                .sum();
            fs.insert(pwd.clone(), size);
        }
    });

    let mut total_sizes: HashMap<String, u32> = HashMap::new();
    fs.iter().for_each(|(path, _)| {
        let total_size = fs.iter().filter(|(path2, _)| (*path2).starts_with(path)).map(|(_, size2)| size2).sum::<u32>();
        total_sizes.insert(path.clone(), total_size);
    });
    let answer: u32 = total_sizes.iter().filter(|(_, size)| **size < 100000).map(|(_, size)| size).sum();
    println!("{}", answer);

    let free_space = 70000000 - total_sizes.get("").unwrap();
    let to_delete = 30000000 - free_space;
    if to_delete > 0 {
        let answer: u32 = total_sizes.iter().filter(|(_, size)| **size > to_delete).map(|(_, size)| *size).min().unwrap();
        println!("{}", answer);
    }
}
