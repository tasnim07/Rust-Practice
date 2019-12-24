pub fn build_proverb_n(list: &[&str]) -> String {
    let mut song: Vec<String> = Vec::new();
    let length = list.len();
    for i in 0..length {
      if i == length - 1 {
        let r_str = format!(
          "And all for the want of a {}.",
          list[0]
        );
        song.push(r_str)
      }
      else {
        let r_str = format!(
          "For want of a {} the {} was lost.",
          list[i], list[i+1]
        );
        song.push(r_str)
      }
    }
    song.join("\n")
}


pub fn build_proverb(list: &[&str]) -> String {
    let mut ret = vec!();
    for (i, _) in list.iter().enumerate() {
	if i == list.len() - 1 {
	    ret.push(format!("And all for the want of a {}.", list[0]));
	} else {
	    ret.push(format!("For want of a {0} the {1} was lost.", list[i], list[i+1]));
	}
    }
    ret.join("\n")
}
