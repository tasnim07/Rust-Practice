pub fn verse(i: u32) -> String {
    let mut rhyme_string = String::new();

    if i > 1 {
	let r_str = format!(
	    "{} bottles of beer on the wall, {} bottles of beer.\n", 
	    i, i
	);
	rhyme_string.push_str(&r_str)
    }
    else if i == 1 {
	let r_str = "1 bottle of beer on the wall, 1 bottle of beer.\n";
	rhyme_string.push_str(&r_str)
    }
    else {
	rhyme_string.push_str(
	    &"No more bottles of beer on the wall, no more bottles of beer.\nGo to the store and buy some more, 99 bottles of beer on the wall.\n"
	);
	return rhyme_string
    }
    
    if i - 1 > 1 {
	let r_str = format!(
	    "Take one down and pass it around, {} bottles of beer  on the wall.\n",
	    i - 1
	);
	rhyme_string.push_str(&r_str)
    }
    else if i - 1 == 1 {
	let r_str = "Take one down and pass it around, 1 bottle of beer on the wall.\n";
	rhyme_string.push_str(&r_str)
    }
    else if i - 1 == 0 {
	let r_str = "Take it down and pass it around, no more bottles of beer on the wall.\n";
	rhyme_string.push_str(&r_str)
    }
    
    rhyme_string
}


pub fn sing(start: u32, end: u32) -> String {
    (end..=start).rev().map(verse).collect::<Vec<String>>().join("\n")
}
