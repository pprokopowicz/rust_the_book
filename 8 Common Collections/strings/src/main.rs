fn main() {
    {
        let mut s = String::new();
        let data = "initial contents";

        let s1 = data.to_string();

        // the method also works on a literal directly:
        let s2 = "initial contents".to_string();

        let s3 = String::from("initial contents");
    }

    {
        let mut s = String::from("foo");
        s.push_str("bar");

        let mut s1 = String::from("foo");
        let s2 = "bar";
        s1.push_str(s2);
        println!("s2 is {}", s2);

        let mut s3 = String::from("lo");
        s3.push('l');
    }

    {
        let s1 = String::from("Hello, ");
        let s2 = String::from("world!");
        let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used

        let s4 = String::from("tic");
        let s5 = String::from("tac");
        let s6 = String::from("toe");
    
        // let s_sum = s4 + "-" + &s5 + "-" + &s6;
        let s_sum2 = format!("{}-{}-{}", s4, s5, s6);
    }

    {
        for c in "नमस्ते".chars() {
            println!("{}", c);
        }

        for b in "नमस्ते".bytes() {
            println!("{}", b);
        }
    }
}
