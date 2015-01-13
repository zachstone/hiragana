use std::rand;
use std::io;

fn main() {
  let hira_hash = [
    ('あ', "a"),
    ('い', "i"),
    ('う', "u"),
    ('え', "e"),
    ('お', "o"),

    ('か', "ka"),
    ('き', "ki"),
    ('く', "ku"),
    ('け', "ke"),
    ('こ', "ko"),

    ('さ', "sa"),
    ('し', "shi"),
    ('す', "su"),
    ('せ', "se"),
    ('そ', "so"),

    ('た', "ta"),
    ('ち', "chi"),
    ('つ', "tsu"),
    ('て', "te"),
    ('と', "to"),

    ('な', "na"),
    ('に', "ni"),
    ('ぬ', "nu"),
    ('ね', "ne"),
    ('の', "no"),

    ('は', "ha"),
    ('ひ', "hi"),
    ('ふ', "fu"),
    ('へ', "he"),
    ('ほ', "ho"),

    ('た', "ta"),
    ('ち', "chi"),
    ('つ', "tsu"),
    ('て', "te"),
    ('と', "to"),

    ('ま', "ma"),
    ('み', "mi"),
    ('む', "mu"),
    ('め', "me"),
    ('も', "mo"),

    ('や', "ya"),
    ('ゆ', "yu"),
    ('よ', "yo"),

    ('ら', "ra"),
    ('り', "ri"),
    ('る', "ru"),
    ('れ', "re"),
    ('ろ', "ro"),

    ('わ', "wa"),
    ('を', "o"),
    
    ('ん', "nn"),
  ];

  loop {
    //let r = rand::random::<usize>() % hira_hash.len();
    let r = (rand::random::<usize>() % 5) + 10;

    print!("{0} : ", hira_hash[r].0);
    let i = io::stdin().read_line().ok().expect("What was that?");

    if hira_hash[r].1 == i.trim() {
      println!("correct!");
    } else {
      println!("incorrect. {} = {}", hira_hash[r].0, hira_hash[r].1);
    }
  }
}
