fn main() {
    let verses: [&str; 12] = [
        "A partridge in a pear tree.",
        "Two turtle doves,",
        "Three French hens,",
        "Four calling birds,",
        "Five golden rings,",
        "Six geese a-laying,",
        "Seven swans a-swimming,",
        "Eight maids a-milking,",
        "Nine ladies dancing,",
        "Ten lords a-leaping,",
        "Eleven pipers piping,",
        "Twelve drummers drumming,"
    ];
    let mut verse_counter: usize = 0;
    
    let mut verses_reversed: [&str; 12] = [""; 12];
    let mut reverse_counter: usize = verses.len() -1;

    while verse_counter < verses.len() {
        
        println!("\nOn the first day of Christmas,
My true love sent to me");
        println!("{}", verses[verse_counter]);
        
        for v in verses_reversed {
            if v != "" {
                if v == "A partridge in a pear tree." {
                    println!("And a partridge in a pear tree.");
                } 
                else {
                    println!("{v}");
                }
            }
        }
        
        verses_reversed[reverse_counter] = verses[verse_counter];
        verse_counter += 1;
        
        reverse_counter = if reverse_counter > 0 {
            reverse_counter - 1
        } else {
            reverse_counter
        };
    }
    
}
