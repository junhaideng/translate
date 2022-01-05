use crate::api::Response;
use crossterm::style::Stylize;

pub fn youdao(res: &Response) {
    print!("{}", &res.query);
    if &res.basic.phonetic.len() != &0 {
        print!(" {} ", format!("[{}]", &res.basic.phonetic).magenta());
    }
    print!("{}", " ~ fanyi.youdao.com\n\n");

    for explain in &res.basic.explains {
        println!("- {}", &explain.clone().green());
    }
    println!();

    for (i, web) in res.web.iter().enumerate() {
        println!("{}. {}\n   {}", i + 1, web.key, &web.value.join(",").blue());
    }
    println!();
}

// pub fn youdao(res: &Response) {
//     let mut buf = String::with_capacity(1024);
//     buf.push_str(&res.query);
//     print!("{}", &res.query);
//     if &res.basic.phonetic.len() != &0 {
//         buf.push_str("  ");
//         buf.push('[');
//         buf.push_str(&res.basic.phonetic);
//         buf.push(']');
//     }
//     buf.push_str(" ~ fanyi.youdao.com\n");
//     buf.push('\n');

//     for explain in &res.basic.explains {
//         buf.push_str("- ".red().on_blue().bold());
//         buf.push_str(&explain);
//         buf.push('\n');
//     }

//     for (i, web) in res.web.iter().enumerate() {
//         buf.push('\n');
//         buf.push_str(&(i + 1).to_string());
//         buf.push('.');
//         buf.push_str("  ");
//         buf.push_str(&web.key);
//         buf.push('\n');
//         buf.push_str("    ");
//         buf.push_str(&web.value.join(","));
//     }
//     println!("{}", buf);
// }
