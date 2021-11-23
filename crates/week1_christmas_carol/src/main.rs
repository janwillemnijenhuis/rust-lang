fn main() {
    let a = ["Twelve drummers drumming",
        "Eleven pipers piping",
        "Ten lords a-leaping",
        "Nine ladies dancing",
        "Eight maids a-milking",
        "Seven swans a-swimming",
        "Six geese a-laying",
        "Five golden rings",
        "Four calling birds",
        "Three french hens",
        "Two turtle doves, and",
        "A partridge in a pear tree"];

    let n = ["first",
        "second",
        "third",
        "fourth",
        "fifth",
        "sixth",
        "seventh",
        "eighth",
        "ninth",
        "tenth",
        "eleventh",
        "twelfth"];

    let mut m = 0;

    let first: &str = "On the ";
    let last: &str = " day of Christmas, my true love sent to me";

    while m != 12 {
        let together = format!("{}{}{}", first, n[m], last);

        println!("{}", together);

        for el in &a[11 - m .. 12] {
            println!("{}", el);
        }

        println!("");

        m += 1;
    }
}
